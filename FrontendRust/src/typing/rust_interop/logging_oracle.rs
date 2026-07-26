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

use crate::typing::names::names::IdT;
use crate::typing::rust_interop::oracle::{
    RustFieldInfo, RustItemId, RustKind, RustOracle, ValeSig,
};
use crate::typing::types::types::*;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::PackageCoordinate;

/// One oracle query and its answer, rendered as text.
///
/// Text rather than a structured enum on purpose: the log crosses a process boundary (the
/// driver prints it, a test reads its stdout), so a stable human-readable line is the useful
/// form. It is evidence, not an API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OracleCall(pub String);

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

    /// Whether any recorded call contains `needle`. The coarse assertion a test wants:
    /// "was this question asked, and did it get this answer".
    pub fn logged(&self, needle: &str) -> bool {
        self.calls.borrow().iter().any(|c| c.0.contains(needle))
    }

    fn record(&self, line: String) {
        self.calls
            .borrow_mut()
            .push(OracleCall(format!("[compile={}] {}", self.compile_tag, line)));
    }
}

impl<'a, 's, 't> RustOracle<'s, 't> for LoggingOracle<'a, 's, 't> {
    fn resolve_path(&self, id: &IdT<'s, 't>) -> Option<RustItemId> {
        let answer = self.inner.resolve_path(id);
        self.record(format!("resolve_path -> {answer:?}"));
        answer
    }

    fn kind(&self, item: RustItemId) -> Option<RustKind> {
        let answer = self.inner.kind(item);
        self.record(format!("kind({item:?}) -> {answer:?}"));
        answer
    }

    fn resolve_method(&self, receiver: &IdT<'s, 't>, method_name: &str) -> Option<RustItemId> {
        let answer = self.inner.resolve_method(receiver, method_name);
        self.record(format!("resolve_method({method_name:?}) -> {answer:?}"));
        answer
    }

    fn resolve_function(&self, function_name: &str) -> Option<RustItemId> {
        let answer = self.inner.resolve_function(function_name);
        self.record(format!("resolve_function({function_name:?}) -> {answer:?}"));
        answer
    }

    fn item_package(&self, item: RustItemId) -> Option<&'s PackageCoordinate<'s>> {
        let answer = self.inner.item_package(item);
        let rendered = answer.map(|c| {
            format!("{}.{:?}", c.module.0, c.packages.iter().map(|p| p.0).collect::<Vec<_>>())
        });
        self.record(format!("item_package({item:?}) -> {rendered:?}"));
        answer
    }

    fn fn_sig(
        &self,
        item: RustItemId,
        args: &[KindT<'s, 't>],
        interner: &TypingInterner<'s, 't>,
    ) -> Option<ValeSig<'s, 't>> {
        let answer = self.inner.fn_sig(item, args, interner);
        // `args` is logged because it is the @EarlyBinder evidence: it shows what the
        // signature was instantiated *at*, which is the thing a fixture could never exercise.
        let rendered = answer.map(|sig| format!("params {:?} ret {:?}", sig.params, sig.ret));
        self.record(format!("fn_sig({item:?}, args {args:?}) -> {rendered:?}"));
        answer
    }

    fn field(&self, owner: &IdT<'s, 't>, field_name: &str) -> Option<RustFieldInfo<'s, 't>> {
        let answer = self.inner.field(owner, field_name);
        self.record(format!("field({field_name:?}) -> {answer:?}"));
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
        self.record(format!("importable_types -> {answer:?}"));
        answer
    }

    fn importable_functions(&self) -> Vec<(String, RustItemId)> {
        let answer = self.inner.importable_functions();
        self.record(format!("importable_functions -> {answer:?}"));
        answer
    }

    fn methods(&self, item: RustItemId) -> Vec<(String, RustItemId)> {
        let answer = self.inner.methods(item);
        self.record(format!("methods({item:?}) -> {answer:?}"));
        answer
    }
}
