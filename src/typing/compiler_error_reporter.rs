use crate::postparsing::names::{IFunctionDeclarationNameS, IImpreciseNameS, INameS, IRuneS};
use crate::postparsing::rules::rules::IRulexSR;
use crate::solver::solver::FailedSolve;
use crate::typing::ast::ast::{KindExportT, PrototypeT, SignatureT};
use crate::typing::borrow_checker::borrow_error::BorrowErrorKind;
use crate::typing::citizen::impl_compiler::IsntParent;
use crate::typing::infer::compiler_solver::ITypingPassSolverError;
use crate::typing::infer_compiler::{IDefiningError, IResolvingError};
use crate::typing::names::names::{IVarNameT, IdT};
use crate::typing::overload_resolver::FindFunctionFailure;
use crate::typing::rune_typing::rune_type_solver::RuneTypeSolveError;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::{InterfaceTT, KindT, StructTT};
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::RangeS;
use std::slice::from_ref;

/// Why a resolved Rust item's signature has no Vale form, carried by
/// `ICompileErrorT::CouldNotPostparseFunction` and rendered by the humanizer. Produced by the Rust
/// interop oracle (`src/typing/rust_interop/`), but defined here in core so the error type — and
/// `get_or_create_postparsed_function`, which returns it — can name it in every build.
///
/// **Structure only — no rendering here.** A case asserts the variant; the wording a person reads is
/// built where diagnostics are built. The reason travels because it *is* the point of declining: a
/// bare miss makes the eventual failure read *"couldn't find function `foo`"* for a function that
/// plainly exists, and carrying the reason is what avoids that lie.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CouldNotPostparseReason {
  /// An integer width `IntT` cannot hold — it carries only `bits`, and only 32 and 64 are mapped.
  IntWidth,
  /// `IntT` has no signedness, so an unsigned type would silently become its signed counterpart.
  UnsignedInteger,
  /// `FloatT` is a unit struct with no width field, so `f32` and `f64` would intern identically.
  Float,
  /// Vale has no unsized concept, so `str` / `[T]` / `dyn Trait` cannot be value types.
  Unsized,
  /// A type reached only through this signature and never imported (@RTMEIZ).
  UnimportedType,
  /// A projection such as `<I as Iterator>::Item`. Normalizing it *requires* reading the
  /// `I: Iterator` predicate to find the impl, and no predicates are read at all — so it is
  /// un-normalizable rather than merely unread.
  UnnormalizableAlias,
  /// A `ty::Param` inherited from a parent impl. Vale's declaration has no slot for it until the
  /// container is declared too.
  InheritedParameter,
  /// Two or more parameters share one lifetime (e.g. `fn f<'a>(x: &'a mut A, y: &'a B)`). Faithfully
  /// mirroring it would tie those parameters into a single Vale group, which needs lifetime decoding
  /// not yet built — so the import is declined rather than guessing the parameters are disjoint (what
  /// per-parameter groups assume).
  SharedParameterLifetime,
  /// A rustc type kind with no Vale representation yet — the catch-all.
  Unrepresentable,
}

#[derive(Debug)]
pub enum ICompileErrorT<'s, 't> {
  CouldntNarrowDownCandidates {
    range: &'t [RangeS<'s>],
    candidates: &'t [PrototypeT<'s, 't>],
  },
  CouldntSolveRuneTypesT {
    range: &'t [RangeS<'s>],
    error: RuneTypeSolveError<'s>,
  },
  NotEnoughGenericArgs {
    range: &'t [RangeS<'s>],
  },
  ImplSubCitizenNotFound {
    range: &'t [RangeS<'s>],
    name: IImpreciseNameS<'s>,
  },
  ImplSuperInterfaceNotFound {
    range: &'t [RangeS<'s>],
    name: IImpreciseNameS<'s>,
  },
  ImmStructCantHaveVaryingMember {
    range: &'t [RangeS<'s>],
    struct_name: INameS<'s>,
    member_name: &'s str,
  },
  ImmStructCantHaveMutableMember {
    range: &'t [RangeS<'s>],
    struct_name: INameS<'s>,
    member_name: &'s str,
  },
  CantReconcileBranchesResults {
    range: &'t [RangeS<'s>],
    then_result: KindT<'s, 't>,
    else_result: KindT<'s, 't>,
  },
  IndexedArrayWithNonInteger {
    range: &'t [RangeS<'s>],
    types: KindT<'s, 't>,
  },
  WrongNumberOfDestructuresError {
    range: &'t [RangeS<'s>],
    actual_num: i32,
    expected_num: i32,
  },
  CantDowncastUnrelatedTypes {
    range: &'t [RangeS<'s>],
    source_kind: KindT<'s, 't>,
    target_kind: KindT<'s, 't>,
    candidates: &'t [IResolvingError<'s, 't>],
  },
  CantDowncastToInterface {
    range: &'t [RangeS<'s>],
    target_kind: InterfaceTT<'s, 't>,
  },
  CantUseRuneValueAsExpression {
    range: &'t [RangeS<'s>],
    rune: IRuneS<'s>,
  },
  CouldntFindTypeT {
    range: &'t [RangeS<'s>],
    name: IImpreciseNameS<'s>,
  },
  TooManyTypesWithNameT {
    range: &'t [RangeS<'s>],
    name: IImpreciseNameS<'s>,
  },
  /// An `import rust.crate.X.Y` statement that resolves to no importable item — the crate is not
  /// loaded, a module segment is missing, the target is a module rather than a fn/struct, or the
  /// import omits the crate (a bare `import rust.Widget`). `path` is the dotted path as written.
  UnresolvableRustImport {
    range: &'t [RangeS<'s>],
    path: String,
  },
  /// A **resolved** Rust item that is nonetheless called with a signature Vale cannot represent — an
  /// unsigned-int or float type, a return naming an unimported type, and the like. Distinct from
  /// `UnresolvableRustImport` (an import statement that resolves to nothing): here the item exists, but
  /// its signature has no Vale form. `path` is the function's human name; `reason` is why it declined.
  CouldNotPostparseFunction {
    range: &'t [RangeS<'s>],
    path: String,
    reason: CouldNotPostparseReason,
  },
  ArrayElementsHaveDifferentTypes {
    range: &'t [RangeS<'s>],
    types: &'t [KindT<'s, 't>],
  },
  UnexpectedArrayElementType {
    range: &'t [RangeS<'s>],
    expected_type: KindT<'s, 't>,
    actual_type: KindT<'s, 't>,
  },
  InitializedWrongNumberOfElements {
    range: &'t [RangeS<'s>],
    expected_num_elements: i32,
    num_elements_initialized: i32,
  },
  CannotSubscriptT {
    range: &'t [RangeS<'s>],
    tyype: KindT<'s, 't>,
  },
  NonReadonlyReferenceFoundInPureFunctionParameter {
    range: &'t [RangeS<'s>],
    param_name: IVarNameT<'s, 't>,
  },
  CouldntFindIdentifierToLoadT {
    range: &'t [RangeS<'s>],
    name: IImpreciseNameS<'s>,
  },
  CouldntFindMemberT {
    range: &'t [RangeS<'s>],
    member_name: &'s str,
  },
  BodyResultDoesntMatch {
    range: &'t [RangeS<'s>],
    function_name: IFunctionDeclarationNameS<'s>,
    expected_return_type: KindT<'s, 't>,
    result_type: KindT<'s, 't>,
  },
  CouldntConvertForReturnT {
    range: &'t [RangeS<'s>],
    expected_type: KindT<'s, 't>,
    actual_type: KindT<'s, 't>,
  },
  CouldntConvertForMutateT {
    range: &'t [RangeS<'s>],
    expected_type: KindT<'s, 't>,
    actual_type: KindT<'s, 't>,
  },
  // The two types are unrelated, e.g. converting an `int` to a `bool`. Neither can be an
  // upcast, because one of them isn't a citizen at all.
  CouldntConvertT {
    range: &'t [RangeS<'s>],
    source_type: KindT<'s, 't>,
    target_type: KindT<'s, 't>,
  },
  // Both are citizens, but no impl makes the source a subtype of the target, e.g. a `Dog`
  // where a `Cat` is wanted. Carries what the impl search rejected.
  CouldntUpcastT {
    range: &'t [RangeS<'s>],
    source_type: KindT<'s, 't>,
    target_type: KindT<'s, 't>,
    isnt_parent: IsntParent<'s, 't>,
  },
  CantMoveOutOfMemberT {
    range: &'t [RangeS<'s>],
    name: IVarNameT<'s, 't>,
  },
  CouldntFindFunctionToCallT {
    range: &'t [RangeS<'s>],
    fff: FindFunctionFailure<'s, 't>,
  },
  CouldntEvaluateFunction {
    range: &'t [RangeS<'s>],
    eff: IDefiningError<'s, 't>,
  },
  CouldntEvaluatImpl {
    range: &'t [RangeS<'s>],
    eff: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>,
  },
  CouldntEvaluateStruct {
    range: &'t [RangeS<'s>],
    eff: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>,
  },
  CouldntEvaluateInterface {
    range: &'t [RangeS<'s>],
    eff: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>,
  },
  CouldntFindOverrideT {
    range: &'t [RangeS<'s>],
    fff: FindFunctionFailure<'s, 't>,
  },
  ExportedFunctionDependedOnNonExportedKind {
    range: &'t [RangeS<'s>],
    paackage: PackageCoordinate<'s>,
    signature: &'t SignatureT<'s, 't>,
    non_exported_kind: KindT<'s, 't>,
  },
  ExternFunctionDependedOnNonExportedKind {
    range: &'t [RangeS<'s>],
    paackage: PackageCoordinate<'s>,
    signature: &'t SignatureT<'s, 't>,
    non_exported_kind: KindT<'s, 't>,
  },
  ExportedKindDependedOnNonExportedKind {
    range: &'t [RangeS<'s>],
    paackage: PackageCoordinate<'s>,
    exported_kind: KindT<'s, 't>,
    non_exported_kind: KindT<'s, 't>,
  },
  TypeExportedMultipleTimes {
    range: &'t [RangeS<'s>],
    paackage: PackageCoordinate<'s>,
    exports: &'t [KindExportT<'s, 't>],
  },
  CantUseUnstackifiedLocal {
    range: &'t [RangeS<'s>],
    local_id: IVarNameT<'s, 't>,
  },
  CantUnstackifyOutsideLocalFromInsideWhile {
    range: &'t [RangeS<'s>],
    local_id: IVarNameT<'s, 't>,
  },
  CantRestackifyOutsideLocalFromInsideWhile {
    range: &'t [RangeS<'s>],
    local_id: IVarNameT<'s, 't>,
  },
  FunctionAlreadyExists {
    old_function_range: RangeS<'s>,
    new_function_range: RangeS<'s>,
    signature: IdT<'s, 't>,
  },
  CantUseReadonlyReferenceAsReadwrite {
    range: &'t [RangeS<'s>],
  },
  LambdaReturnDoesntMatchInterfaceConstructor {
    range: &'t [RangeS<'s>],
  },
  ConditionIsntBoolean {
    range: &'t [RangeS<'s>],
    actual_type: KindT<'s, 't>,
  },
  HigherTypingInferError {
    range: &'t [RangeS<'s>],
    err: RuneTypeSolveError<'s>,
  },
  AbstractMethodOutsideOpenInterface {
    range: &'t [RangeS<'s>],
  },
  TypingPassSolverError {
    range: &'t [RangeS<'s>],
    failed_solve:
      FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>,
  },
  TypingPassResolvingError {
    range: &'t [RangeS<'s>],
    inner: IResolvingError<'s, 't>,
  },
  TypingPassDefiningError {
    range: &'t [RangeS<'s>],
    inner: IDefiningError<'s, 't>,
  },
  CantImplNonInterface {
    range: &'t [RangeS<'s>],
    templata: ITemplataT<'s, 't>,
  },
  NonCitizenCantImpl {
    range: &'t [RangeS<'s>],
    templata: ITemplataT<'s, 't>,
  },
  RangedInternalErrorT {
    range: &'t [RangeS<'s>],
    message: &'s str,
  },
  BorrowCheckError {
    range: &'t [RangeS<'s>],
    kind: BorrowErrorKind<'s, 't>,
  },
  WeakableImplingMismatch {
    range: &'t [RangeS<'s>],
    struct_weakable: bool,
    interface_weakable: bool,
  },
  TookWeakRefOfNonWeakableError {
    range: &'t [RangeS<'s>],
  },
  NoImplicitCloneDefinedT {
    range: &'t [RangeS<'s>],
    source_type: KindT<'s, 't>,
    target_type: KindT<'s, 't>,
  },
  ImplicitCloneRejectedT {
    range: &'t [RangeS<'s>],
    source_type: KindT<'s, 't>,
    target_type: KindT<'s, 't>,
    fff: FindFunctionFailure<'s, 't>,
  },
}

impl<'s, 't> ICompileErrorT<'s, 't> {
  pub fn range(&self) -> &[RangeS<'s>] {
    match self {
      Self::CouldntNarrowDownCandidates { range, .. } => *range,
      Self::CouldntSolveRuneTypesT { range, .. } => *range,
      Self::NotEnoughGenericArgs { range, .. } => *range,
      Self::ImplSubCitizenNotFound { range, .. } => *range,
      Self::ImplSuperInterfaceNotFound { range, .. } => *range,
      Self::ImmStructCantHaveVaryingMember { range, .. } => *range,
      Self::ImmStructCantHaveMutableMember { range, .. } => *range,
      Self::CantReconcileBranchesResults { range, .. } => *range,
      Self::IndexedArrayWithNonInteger { range, .. } => *range,
      Self::WrongNumberOfDestructuresError { range, .. } => *range,
      Self::CantDowncastUnrelatedTypes { range, .. } => *range,
      Self::CantDowncastToInterface { range, .. } => *range,
      Self::CantUseRuneValueAsExpression { range, .. } => *range,
      Self::CouldntFindTypeT { range, .. } => *range,
      Self::TooManyTypesWithNameT { range, .. } => *range,
      Self::UnresolvableRustImport { range, .. } => *range,
      Self::CouldNotPostparseFunction { range, .. } => *range,
      Self::ArrayElementsHaveDifferentTypes { range, .. } => *range,
      Self::UnexpectedArrayElementType { range, .. } => *range,
      Self::InitializedWrongNumberOfElements { range, .. } => *range,
      Self::CannotSubscriptT { range, .. } => *range,
      Self::NonReadonlyReferenceFoundInPureFunctionParameter { range, .. } => *range,
      Self::CouldntFindIdentifierToLoadT { range, .. } => *range,
      Self::CouldntFindMemberT { range, .. } => *range,
      Self::BodyResultDoesntMatch { range, .. } => *range,
      Self::CouldntConvertForReturnT { range, .. } => *range,
      Self::CouldntConvertForMutateT { range, .. } => *range,
      Self::CouldntConvertT { range, .. } => *range,
      Self::CouldntUpcastT { range, .. } => *range,
      Self::CantMoveOutOfMemberT { range, .. } => *range,
      Self::CouldntFindFunctionToCallT { range, .. } => *range,
      Self::CouldntEvaluateFunction { range, .. } => *range,
      Self::CouldntEvaluatImpl { range, .. } => *range,
      Self::CouldntEvaluateStruct { range, .. } => *range,
      Self::CouldntEvaluateInterface { range, .. } => *range,
      Self::CouldntFindOverrideT { range, .. } => *range,
      Self::ExportedFunctionDependedOnNonExportedKind { range, .. } => *range,
      Self::ExternFunctionDependedOnNonExportedKind { range, .. } => *range,
      Self::ExportedKindDependedOnNonExportedKind { range, .. } => *range,
      Self::TypeExportedMultipleTimes { range, .. } => *range,
      Self::CantUseUnstackifiedLocal { range, .. } => *range,
      Self::CantUnstackifyOutsideLocalFromInsideWhile { range, .. } => *range,
      Self::CantRestackifyOutsideLocalFromInsideWhile { range, .. } => *range,
      Self::FunctionAlreadyExists { new_function_range, .. } => from_ref(new_function_range),
      Self::CantUseReadonlyReferenceAsReadwrite { range, .. } => *range,
      Self::LambdaReturnDoesntMatchInterfaceConstructor { range, .. } => *range,
      Self::ConditionIsntBoolean { range, .. } => *range,
      Self::HigherTypingInferError { range, .. } => *range,
      Self::AbstractMethodOutsideOpenInterface { range, .. } => *range,
      Self::TypingPassSolverError { range, .. } => *range,
      Self::TypingPassResolvingError { range, .. } => *range,
      Self::TypingPassDefiningError { range, .. } => *range,
      Self::CantImplNonInterface { range, .. } => *range,
      Self::NonCitizenCantImpl { range, .. } => *range,
      Self::RangedInternalErrorT { range, .. } => *range,
      Self::BorrowCheckError { range, .. } => *range,
      Self::WeakableImplingMismatch { range, .. } => *range,
      Self::TookWeakRefOfNonWeakableError { range, .. } => *range,
      Self::NoImplicitCloneDefinedT { range, .. } => *range,
      Self::ImplicitCloneRejectedT { range, .. } => *range,
    }
  }
}
