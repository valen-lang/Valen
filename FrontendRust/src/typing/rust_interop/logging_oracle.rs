// A `RustOracle` decorator that records what it was asked and what it answered.
//
// This exists because "the Vale program compiled" is weak evidence for interop. A program
// like `add_two_numbers(3, 4)` would compile just as happily if some Vale function of that
// name were in scope and the oracle were never consulted at all — the test would pass
// vacuously. The log turns consultation into an observable fact rather than an inference.
//
// It also makes one thing testable that was structurally untestable before. The @EarlyBinder
// rule says `fn_sig` must instantiate at the call's concrete args *before* lowering; the
// fixture oracle ignored its `args` entirely, so instantiate-then-lower and
// lower-then-instantiate were indistinguishable. The log records the args each `fn_sig` was
// called with, so the ordering becomes observable.
//
// Deliberately a decorator rather than a flag on the real oracle: the logging has nothing to
// do with rustc, works against any implementation, and keeps `TyCtxtOracle` free of test
// affordances.

use std::cell::RefCell;

use crate::interner::StrI;
use crate::typing::rust_interop::oracle::{
    RustItemId, RustOracle, ValeSig, ValeSigType,
};
use crate::typing::rust_interop::reserved::is_rust_backed_kind;
use crate::typing::types::types::*;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::PackageCoordinate;

/// One position of a lowered signature, reduced to the facts a test keys on.
///
/// Owned and free of arena lifetimes, because the log outlives the compilation that produced
/// it — the arenas die when `after_expansion` returns, and only owned data escapes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SigPosition {
    /// A settled kind. `rust_backed` says whether it is a Rust citizen, which is the coarse
    /// fact worth recording: it means a Rust type reached a signature position at all.
    Kind { rust_backed: bool },
    /// A reference to the item's own generic parameter at this index.
    Generic(u32),
    /// An imported citizen applied to arguments. Recorded with its name and its argument
    /// positions, because "a citizen reached this position" is not the interesting fact once
    /// generic types exist — *which* citizen at *which* arguments is.
    Citizen { name: String, args: Vec<SigPosition> },
}

/// A lowered signature, reduced to the facts a test keys on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SigShape {
    /// The item's own generic parameter names, in declaration order. Empty for a non-generic
    /// function — the degenerate case, not a separate one.
    pub generic_params: Vec<String>,
    pub params: Vec<SigPosition>,
    pub ret: SigPosition,
}

/// Which question was asked, and the part of the answer worth asserting on.
///
/// Structured rather than parsed back out of the rendered line. The rendering exists to be
/// read by a person; keying assertions on it couples them to `Debug` output, which is how two
/// assertions broke in one day — once when a return position started printing as
/// `Kind(Struct(..))` rather than `Struct(..)`, and once when `{:?}` on an `Option<String>`
/// escaped every quote inside it. Neither change altered any behaviour.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OracleQuery {
    ItemPackage { item: RustItemId, answered: bool },
    /// `None` means the signature was **declined** — it mentions something with no Vale form,
    /// so the declaration built from it would have a hole in it.
    FnSig { item: RustItemId, answer: Option<SigShape> },
    // The enumerating queries carry the handles alongside the names, so a test can correlate:
    // "the item offered as `first` is the one whose signature was declined" needs the id to
    // join on, and a name-only record would leave that assertion coupled to ordering.
    ImportableTypes { items: Vec<(String, RustItemId)> },
    ImportableFunctions { items: Vec<(String, RustItemId)> },
    Methods { owner: RustItemId, found: Vec<(String, RustItemId)> },
    /// A type's own generic parameter names. Empty means non-generic, which is the degenerate
    /// case rather than an absence.
    TypeGenericParams { item: RustItemId, names: Vec<String> },
}

impl OracleQuery {
    /// The handle offered under `name` by an enumerating query, if this is one and it offered it.
    pub fn offered(&self, name: &str) -> Option<RustItemId> {
        let items = match self {
            OracleQuery::ImportableTypes { items } => items,
            OracleQuery::ImportableFunctions { items } => items,
            OracleQuery::Methods { found, .. } => found,
            _ => return None,
        };
        items.iter().find(|(n, _)| n == name).map(|(_, id)| *id)
    }
}

/// One oracle query and its answer.
///
/// Two forms of the same event, deliberately: `query` is what tests assert on, `rendered` is
/// what a person reads when one fails. Keeping both means a diagnostic can stay chatty without
/// any assertion depending on how it is worded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OracleCall {
    pub query: OracleQuery,
    pub rendered: String,
}

pub struct LoggingOracle<'a, 's, 't> {
    inner: &'a dyn RustOracle<'s, 't>,
    /// Which rustc invocation these entries came from, e.g. the crate being compiled.
    ///
    /// One invocation today, so this is constant and the tag looks like ceremony. It stops
    /// being constant as soon as there is more than one compile — cargo spawns one rustc per
    /// crate, each a fresh process, and the same callback then fires in each with *different
    /// correct answers*. An untagged log silently interleaves them. Added now because adding
    /// it later means touching every assertion (learned from the toylang/Sky prototype, which
    /// did exactly that).
    compile_tag: String,
    calls: RefCell<Vec<OracleCall>>,
}

impl<'a, 's, 't> LoggingOracle<'a, 's, 't> {
    pub fn new(inner: &'a dyn RustOracle<'s, 't>, compile_tag: &str) -> Self {
        LoggingOracle {
            inner,
            compile_tag: compile_tag.to_string(),
            calls: RefCell::new(Vec::new()),
        }
    }

    /// Every call recorded so far, oldest first.
    pub fn calls(&self) -> Vec<OracleCall> {
        self.calls.borrow().clone()
    }

    fn record(&self, query: OracleQuery, line: String) {
        self.calls.borrow_mut().push(OracleCall {
            query,
            rendered: format!("[compile={}] {}", self.compile_tag, line),
        });
    }
}

/// Reduce a lowered signature to the owned facts a test keys on.
fn shape_of(sig: &ValeSig) -> SigShape {
    fn position(t: &ValeSigType) -> SigPosition {
        match t {
            ValeSigType::Kind(k) => SigPosition::Kind { rust_backed: is_rust_backed_kind(*k) },
            ValeSigType::Generic(i) => SigPosition::Generic(*i),
            ValeSigType::Citizen { name, args } => SigPosition::Citizen {
                name: name.0.to_string(),
                args: args.iter().map(position).collect(),
            },
        }
    }
    SigShape {
        generic_params: sig.generic_params.iter().map(|n| n.0.to_string()).collect(),
        params: sig.params.iter().map(position).collect(),
        ret: position(&sig.ret),
    }
}

impl<'a, 's, 't> RustOracle<'s, 't> for LoggingOracle<'a, 's, 't> {
    fn item_package(&self, item: RustItemId) -> Option<&'s PackageCoordinate<'s>> {
        let answer = self.inner.item_package(item);
        let rendered = answer.map(|c| {
            format!("{}.{:?}", c.module.0, c.packages.iter().map(|p| p.0).collect::<Vec<_>>())
        });
        self.record(
            OracleQuery::ItemPackage { item, answered: answer.is_some() },
            format!("item_package({item:?}) -> {rendered:?}"),
        );
        answer
    }

    fn fn_sig(
        &self,
        item: RustItemId,
        interner: &TypingInterner<'s, 't>,
    ) -> Option<ValeSig<'s, 't>> {
        let answer = self.inner.fn_sig(item, interner);
        // The generic-parameter list is logged alongside the positions because together they are
        // the @EarlyBinder evidence under structural reading: they show that the signature came
        // back with its parameters intact rather than collapsed to one instantiation. A `None`
        // here is equally worth seeing — it means the signature mentions something with no Vale
        // form, so the declaration is about to be dropped.
        //
        // Rendered by hand rather than by `{:?}`-ing an `Option<String>`, which would escape every
        // quote inside it and leave assertions matching against `\"A\"`. The log exists to be read
        // — by a person and by a test — so the readable form is the correct one.
        let line = match answer {
            Some(sig) => format!(
                "fn_sig({item:?}) -> generics {:?} params {:?} ret {:?}",
                sig.generic_params, sig.params, sig.ret
            ),
            None => format!("fn_sig({item:?}) -> None"),
        };
        self.record(
            OracleQuery::FnSig { item, answer: answer.as_ref().map(shape_of) },
            line,
        );
        answer
    }

    // Every trait method must be forwarded explicitly, including ones with defaults.
    //
    // `importable_types` and `methods` have default impls returning empty, and for a while
    // this decorator inherited them — so it answered "no types" no matter what the real
    // oracle knew, and the importer's loop body simply never ran. Nothing failed; the import
    // was a silent no-op. A decorator that inherits a default is a decorator that lies, so
    // any method added to `RustOracle` has to be added here too.
    fn importable_types(&self) -> Vec<(String, RustItemId)> {
        let answer = self.inner.importable_types();
        self.record(
            OracleQuery::ImportableTypes { items: answer.clone() },
            format!("importable_types -> {answer:?}"),
        );
        answer
    }

    fn importable_functions(&self) -> Vec<(String, RustItemId)> {
        let answer = self.inner.importable_functions();
        self.record(
            OracleQuery::ImportableFunctions { items: answer.clone() },
            format!("importable_functions -> {answer:?}"),
        );
        answer
    }

    fn methods(&self, item: RustItemId) -> Vec<(String, RustItemId)> {
        let answer = self.inner.methods(item);
        self.record(
            OracleQuery::Methods { owner: item, found: answer.clone() },
            format!("methods({item:?}) -> {answer:?}"),
        );
        answer
    }

    fn type_generic_params(
        &self,
        item: RustItemId,
        interner: &TypingInterner<'s, 't>,
    ) -> &'t [StrI<'s>] {
        let answer = self.inner.type_generic_params(item, interner);
        let names: Vec<String> = answer.iter().map(|n| n.0.to_string()).collect();
        self.record(
            OracleQuery::TypeGenericParams { item, names: names.clone() },
            format!("type_generic_params({item:?}) -> {names:?}"),
        );
        answer
    }
}
