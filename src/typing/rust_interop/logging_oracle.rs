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
use crate::postparsing::ast::ImportS;
use crate::typing::env::environment::ResolvedName;
use crate::typing::compiler_error_reporter::CouldNotPostparseReason;
use crate::typing::rust_interop::oracle::{RustItemId, RustOracle, ValeSig, ValeSigType};
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
  /// A reference (`&T` / `&mut T`) wrapping another position — a Rust `&self` receiver or a
  /// borrowed parameter.
  Borrow(Box<SigPosition>),
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
  ItemPackage {
    item: RustItemId,
    answered: bool,
  },
  /// `None` means the signature was **declined** — it mentions something with no Vale form,
  /// so the declaration built from it would have a hole in it.
  FnSig {
    item: RustItemId,
    answer: Option<SigShape>,
  },
  // `Methods` carries the handles alongside the names so a test can correlate: "the method offered as
  // `first` is the one whose signature was declined" needs the id to join on, and a name-only record
  // would leave that assertion coupled to ordering.
  Methods {
    owner: RustItemId,
    found: Vec<(String, RustItemId)>,
  },
  /// A type's own generic parameter names. Empty means non-generic, which is the degenerate
  /// case rather than an absence.
  TypeGenericParams {
    item: RustItemId,
    names: Vec<String>,
  },
  /// One `import rust.crate.X.Y` resolved to a top-level type or free function. `None` means the
  /// import resolved to nothing. This is the offer point for top-level items — a type or free
  /// function reaches a program by being imported, so this is where a test sees it offered; methods
  /// are offered by `Methods` instead.
  ResolveImport {
    offered: Option<(String, RustItemId)>,
  },
}

impl OracleQuery {
  /// The handle offered under `name`, if this query offered it. A top-level type or free function is
  /// offered by resolving an import; a method by enumerating its type's `methods`.
  pub fn offered(&self, name: &str) -> Option<RustItemId> {
    match self {
      OracleQuery::Methods { found: items, .. } => {
        items.iter().find(|(n, _)| n == name).map(|(_, id)| *id)
      }
      OracleQuery::ResolveImport { offered: Some((n, id)) } if n == name => Some(*id),
      _ => None,
    }
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
    LoggingOracle { inner, compile_tag: compile_tag.to_string(), calls: RefCell::new(Vec::new()) }
  }

  /// Every call recorded so far, oldest first.
  pub fn calls(&self) -> Vec<OracleCall> {
    self.calls.borrow().clone()
  }

  fn record(&self, query: OracleQuery, line: String) {
    self
      .calls
      .borrow_mut()
      .push(OracleCall { query, rendered: format!("[compile={}] {}", self.compile_tag, line) });
  }
}

/// Reduce a lowered signature to the owned facts a test keys on.
fn shape_of(sig: &ValeSig) -> SigShape {
  fn position(t: &ValeSigType) -> SigPosition {
    match t {
      ValeSigType::Kind(k) => SigPosition::Kind { rust_backed: is_rust_backed_kind(*k) },
      ValeSigType::Generic(i) => SigPosition::Generic(*i),
      ValeSigType::Citizen { name, package: _, args } => {
        SigPosition::Citizen { name: name.0.to_string(), args: args.iter().map(position).collect() }
      }
      ValeSigType::Borrow { inner, .. } => SigPosition::Borrow(Box::new(position(inner))),
    }
  }
  SigShape {
    generic_params: sig.generic_params.iter().map(|n| n.0.to_string()).collect(),
    params: sig.params.iter().map(position).collect(),
    ret: position(&sig.ret),
  }
}

impl<'a, 's, 't> RustOracle<'s, 't> for LoggingOracle<'a, 's, 't> {
  // A pure delegation, not recorded: `resolve` is a name-to-item table lookup, not a question put
  // to rustc, so it does not belong in the oracle log the tests assert `fn_sig` counts against. It
  // must still be forwarded explicitly — a decorator that inherits the default `None` would silently
  // fail every lazy synthesis (see the note below on `methods`).
  fn resolve(&self, name: &ResolvedName<'s>) -> Option<RustItemId> {
    self.inner.resolve(name)
  }

  fn resolve_import(&self, import: &ImportS<'s>) -> Option<ResolvedName<'s>> {
    let answer = self.inner.resolve_import(import);
    // Record what the import offered, joined to its handle, so a test can assert an item reached
    // the program by import and then correlate that handle with later `fn_sig` queries.
    let offered = answer
      .and_then(|name| self.inner.resolve(&name).map(|id| (name.importee_name.0.to_string(), id)));
    let rendered = match &offered {
      Some((n, id)) => format!("resolve_import -> {n} ({id:?})"),
      None => "resolve_import -> None".to_string(),
    };
    self.record(OracleQuery::ResolveImport { offered }, rendered);
    answer
  }

  fn item_package(&self, item: RustItemId) -> Option<&'s PackageCoordinate<'s>> {
    let answer = self.inner.item_package(item);
    let rendered = answer
      .map(|c| format!("{}.{:?}", c.module.0, c.packages.iter().map(|p| p.0).collect::<Vec<_>>()));
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
  ) -> Result<ValeSig<'s, 't>, CouldNotPostparseReason> {
    let answer = self.inner.fn_sig(item, interner);
    // The generic-parameter list is logged alongside the positions because together they are
    // the @EarlyBinder evidence under structural reading: they show that the signature came
    // back with its parameters intact rather than collapsed to one instantiation. An `Err`
    // here is equally worth seeing — it is a decline, naming why the signature has no Vale form.
    //
    // Rendered by hand rather than by `{:?}`-ing an `Option<String>`, which would escape every
    // quote inside it and leave assertions matching against `\"A\"`. The log exists to be read
    // — by a person and by a test — so the readable form is the correct one.
    let line = match answer {
      Ok(sig) => format!(
        "fn_sig({item:?}) -> generics {:?} params {:?} ret {:?}",
        sig.generic_params, sig.params, sig.ret
      ),
      Err(reason) => format!("fn_sig({item:?}) -> declined ({reason:?})"),
    };
    self.record(OracleQuery::FnSig { item, answer: answer.as_ref().ok().map(shape_of) }, line);
    answer
  }

  // Every trait method must be forwarded explicitly, including ones with defaults.
  //
  // `methods` and `resolve` have default impls returning empty/`None`, and a decorator that inherits
  // a default is a decorator that lies — it would answer "no methods" no matter what the real oracle
  // knew, silently making the import a no-op. So any method added to `RustOracle` has to be forwarded
  // here too.
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
