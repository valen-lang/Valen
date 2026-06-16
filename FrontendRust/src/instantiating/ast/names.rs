use crate::interner::StrI;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::postparsing::names::IRuneS;
use crate::instantiating::ast::types::{CoordI, ICitizenIT, MutabilityI, VariabilityI};
use crate::instantiating::ast::templata::{ITemplataI, CoordTemplataI};
use crate::typing::types::types::RegionT;
use crate::instantiating::ast::ast::LocationInFunctionEnvironmentI;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::typing::types::types::CoordT;
use std::marker::PhantomData;


// mig: struct IdI
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IdI<'s, 'i> {
    pub package_coord: &'s PackageCoordinate<'s>,
    pub init_steps: &'i [INameI<'s, 'i>],
    pub local_name: INameI<'s, 'i>,
}
// mig: impl IdI

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for IdI` above.)

// mig: fn package_id
// (was cfg-gated)
impl<'s, 'i> IdI<'s, 'i> {
    pub fn package_id(&self) -> IdI<'s, 'i> {
        panic!("Unimplemented: package_id");
        // IdI(packageCoord, Vector(), PackageTopLevelNameI())
    }

// mig: fn init_id
// (was cfg-gated)
    pub fn init_id(&self) -> IdI<'s, 'i> {
        panic!("Unimplemented: init_id");
        // if (initSteps.isEmpty) IdI(packageCoord, Vector(), PackageTopLevelNameI())
        // else IdI(packageCoord, initSteps.init, initSteps.last)
    }
}

// mig: fn init_non_package_id
// (was cfg-gated)
impl<'s, 'i> IdI<'s, 'i> {
    pub fn init_non_package_id(&self) -> Option<IdI<'s, 'i>> {
        if self.init_steps.is_empty() {
            None
        } else {
            let (last, init) = self.init_steps.split_last().unwrap();
            Some(IdI { package_coord: self.package_coord, init_steps: init, local_name: *last })
        }
    }
}

// mig: fn steps
// (was cfg-gated)
impl<'s, 'i> IdI<'s, 'i> {
    pub fn steps(&self) -> &'i[INameI<'s, 'i>] {
        panic!("Unimplemented: steps");
        // localName match {
        //   case PackageTopLevelNameI() => initSteps
        //   case _ => initSteps :+ localName
        // }
    }
}

// mig: fn add_step
// (was cfg-gated)
pub fn add_step<'s, 'i>(old: &IdI<'s, 'i>, new_last: INameI<'s, 'i>) -> IdI<'s, 'i> {
    IdI { package_coord: old.package_coord, init_steps: old.init_steps, local_name: new_last }
}

// mig: enum INameI
/// Polyvalue (see @TFITCX) — derive Eq/Hash; never hand-roll `ptr::eq` on the outer `&self` (see @PVECFPZ).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum INameI<'s, 'i> {
    RegionName(&'i RegionNameI<'s>),
    DenizenDefaultRegionName(&'i DenizenDefaultRegionNameI),
    ExportTemplate(&'i ExportTemplateNameI<'s>),
    Export(&'i ExportNameI<'s>),
    ExternTemplate(&'i ExternTemplateNameI<'s>),
    Extern(&'i ExternNameI<'s>),
    ImplTemplate(&'i ImplTemplateNameI<'s>),
    Impl(&'i ImplNameI<'s, 'i>),
    ImplBoundTemplate(&'i ImplBoundTemplateNameI<'s>),
    ImplBound(&'i ImplBoundNameI<'s, 'i>),
    Let(&'i LetNameI<'s>),
    ExportAs(&'i ExportAsNameI<'s>),
    RawArray(&'i RawArrayNameI<'s, 'i>),
    ReachablePrototype(&'i ReachablePrototypeNameI),
    StaticSizedArrayTemplate(&'i StaticSizedArrayTemplateNameI),
    StaticSizedArray(&'i StaticSizedArrayNameI<'s, 'i>),
    RuntimeSizedArrayTemplate(&'i RuntimeSizedArrayTemplateNameI),
    RuntimeSizedArray(&'i RuntimeSizedArrayNameI<'s, 'i>),
    OverrideDispatcherTemplate(&'i OverrideDispatcherTemplateNameI<'s, 'i>),
    OverrideDispatcher(&'i OverrideDispatcherNameI<'s, 'i>),
    OverrideDispatcherCase(&'i OverrideDispatcherCaseNameI<'s, 'i>),
    CaseFunctionFromImpl(&'i CaseFunctionFromImplNameI<'s, 'i>),
    CaseFunctionFromImplTemplate(&'i CaseFunctionFromImplTemplateNameI<'s>),
    TypingPassBlockResultVar(&'i TypingPassBlockResultVarNameI<'i>),
    TypingPassFunctionResultVar(&'i TypingPassFunctionResultVarNameI),
    TypingPassTemporaryVar(&'i TypingPassTemporaryVarNameI<'i>),
    TypingPassPatternMember(&'i TypingPassPatternMemberNameI<'i>),
    TypingIgnoredParam(&'i TypingIgnoredParamNameI),
    TypingPassPatternDestructuree(&'i TypingPassPatternDestructureeNameI<'i>),
    UnnamedLocal(&'i UnnamedLocalNameI<'s>),
    ClosureParam(&'i ClosureParamNameI<'s>),
    ConstructingMember(&'i ConstructingMemberNameI<'s>),
    WhileCondResult(&'i WhileCondResultNameI<'s>),
    Iterable(&'i IterableNameI<'s>),
    Iterator(&'i IteratorNameI<'s>),
    IterationOption(&'i IterationOptionNameI<'s>),
    MagicParam(&'i MagicParamNameI<'s>),
    CodeVar(&'i CodeVarNameI<'s>),
    AnonymousSubstructMember(&'i AnonymousSubstructMemberNameI),
    Primitive(&'i PrimitiveNameI<'s>),
    PackageTopLevel(&'i PackageTopLevelNameI),
    Project(&'i ProjectNameI<'s>),
    Package(&'i PackageNameI<'s>),
    Rune(&'i RuneNameI<'s>),
    BuildingFunctionNameWithClosureds(&'i BuildingFunctionNameWithClosuredsI<'s, 'i>),
    ExternFunction(&'i ExternFunctionNameI<'s, 'i>),
    FunctionNameIX(&'i FunctionNameIX<'s, 'i>),
    ForwarderFunction(&'i ForwarderFunctionNameI<'s, 'i>),
    FunctionBoundTemplate(&'i FunctionBoundTemplateNameI<'s>),
    FunctionBound(&'i FunctionBoundNameI<'s, 'i>),
    ReachableFunctionTemplate(&'i ReachableFunctionTemplateNameI<'s>),
    ReachableFunction(&'i ReachableFunctionNameI<'s, 'i>),
    FunctionTemplate(&'i FunctionTemplateNameI<'s>),
    LambdaCallFunctionTemplate(&'i LambdaCallFunctionTemplateNameI<'s, 'i>),
    LambdaCallFunction(&'i LambdaCallFunctionNameI<'s, 'i>),
    ForwarderFunctionTemplate(&'i ForwarderFunctionTemplateNameI<'s, 'i>),
    ConstructorTemplate(&'i ConstructorTemplateNameI<'s>),
    Self_(&'i SelfNameI),
    Arbitrary(&'i ArbitraryNameI),
    StructName(&'i StructNameI<'s, 'i>),
    InterfaceName(&'i InterfaceNameI<'s, 'i>),
    LambdaCitizenTemplate(&'i LambdaCitizenTemplateNameI<'s>),
    LambdaCitizen(&'i LambdaCitizenNameI<'s>),
    StructTemplate(&'i StructTemplateNameI<'s>),
    InterfaceTemplate(&'i InterfaceTemplateNameI<'s>),
    AnonymousSubstructImplTemplate(&'i AnonymousSubstructImplTemplateNameI<'s, 'i>),
    AnonymousSubstructImpl(&'i AnonymousSubstructImplNameI<'s, 'i>),
    AnonymousSubstructTemplate(&'i AnonymousSubstructTemplateNameI<'s, 'i>),
    AnonymousSubstructConstructorTemplate(&'i AnonymousSubstructConstructorTemplateNameI<'s, 'i>),
    AnonymousSubstructConstructor(&'i AnonymousSubstructConstructorNameI<'s, 'i>),
    AnonymousSubstruct(&'i AnonymousSubstructNameI<'s, 'i>),
    ResolvingEnv(&'i ResolvingEnvNameI),
    CallEnv(&'i CallEnvNameI),
}

/// Interning transient (see @TFITCX) — mirror of INameI holding payloads by value
/// (used as HashMap lookup key in the interner). Per typing-pass parity (INameValT).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum INameValI<'s, 'i> {
    RegionName(RegionNameI<'s>),
    DenizenDefaultRegionName(DenizenDefaultRegionNameI),
    ExportTemplate(ExportTemplateNameI<'s>),
    Export(ExportNameI<'s>),
    ExternTemplate(ExternTemplateNameI<'s>),
    Extern(ExternNameI<'s>),
    ImplTemplate(ImplTemplateNameI<'s>),
    Impl(ImplNameI<'s, 'i>),
    ImplBoundTemplate(ImplBoundTemplateNameI<'s>),
    ImplBound(ImplBoundNameI<'s, 'i>),
    Let(LetNameI<'s>),
    ExportAs(ExportAsNameI<'s>),
    RawArray(RawArrayNameI<'s, 'i>),
    ReachablePrototype(ReachablePrototypeNameI),
    StaticSizedArrayTemplate(StaticSizedArrayTemplateNameI),
    StaticSizedArray(StaticSizedArrayNameI<'s, 'i>),
    RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateNameI),
    RuntimeSizedArray(RuntimeSizedArrayNameI<'s, 'i>),
    OverrideDispatcherTemplate(OverrideDispatcherTemplateNameI<'s, 'i>),
    OverrideDispatcher(OverrideDispatcherNameI<'s, 'i>),
    OverrideDispatcherCase(OverrideDispatcherCaseNameI<'s, 'i>),
    CaseFunctionFromImpl(CaseFunctionFromImplNameI<'s, 'i>),
    CaseFunctionFromImplTemplate(CaseFunctionFromImplTemplateNameI<'s>),
    TypingPassBlockResultVar(TypingPassBlockResultVarNameI<'i>),
    TypingPassFunctionResultVar(TypingPassFunctionResultVarNameI),
    TypingPassTemporaryVar(TypingPassTemporaryVarNameI<'i>),
    TypingPassPatternMember(TypingPassPatternMemberNameI<'i>),
    TypingIgnoredParam(TypingIgnoredParamNameI),
    TypingPassPatternDestructuree(TypingPassPatternDestructureeNameI<'i>),
    UnnamedLocal(UnnamedLocalNameI<'s>),
    ClosureParam(ClosureParamNameI<'s>),
    ConstructingMember(ConstructingMemberNameI<'s>),
    WhileCondResult(WhileCondResultNameI<'s>),
    Iterable(IterableNameI<'s>),
    Iterator(IteratorNameI<'s>),
    IterationOption(IterationOptionNameI<'s>),
    MagicParam(MagicParamNameI<'s>),
    CodeVar(CodeVarNameI<'s>),
    AnonymousSubstructMember(AnonymousSubstructMemberNameI),
    Primitive(PrimitiveNameI<'s>),
    PackageTopLevel(PackageTopLevelNameI),
    Project(ProjectNameI<'s>),
    Package(PackageNameI<'s>),
    Rune(RuneNameI<'s>),
    BuildingFunctionNameWithClosureds(BuildingFunctionNameWithClosuredsI<'s, 'i>),
    ExternFunction(ExternFunctionNameI<'s, 'i>),
    FunctionNameIX(FunctionNameIX<'s, 'i>),
    ForwarderFunction(ForwarderFunctionNameI<'s, 'i>),
    FunctionBoundTemplate(FunctionBoundTemplateNameI<'s>),
    FunctionBound(FunctionBoundNameI<'s, 'i>),
    ReachableFunctionTemplate(ReachableFunctionTemplateNameI<'s>),
    ReachableFunction(ReachableFunctionNameI<'s, 'i>),
    FunctionTemplate(FunctionTemplateNameI<'s>),
    LambdaCallFunctionTemplate(LambdaCallFunctionTemplateNameI<'s, 'i>),
    LambdaCallFunction(LambdaCallFunctionNameI<'s, 'i>),
    ForwarderFunctionTemplate(ForwarderFunctionTemplateNameI<'s, 'i>),
    ConstructorTemplate(ConstructorTemplateNameI<'s>),
    Self_(SelfNameI),
    Arbitrary(ArbitraryNameI),
    StructName(StructNameI<'s, 'i>),
    InterfaceName(InterfaceNameI<'s, 'i>),
    LambdaCitizenTemplate(LambdaCitizenTemplateNameI<'s>),
    LambdaCitizen(LambdaCitizenNameI<'s>),
    StructTemplate(StructTemplateNameI<'s>),
    InterfaceTemplate(InterfaceTemplateNameI<'s>),
    AnonymousSubstructImplTemplate(AnonymousSubstructImplTemplateNameI<'s, 'i>),
    AnonymousSubstructImpl(AnonymousSubstructImplNameI<'s, 'i>),
    AnonymousSubstructTemplate(AnonymousSubstructTemplateNameI<'s, 'i>),
    AnonymousSubstructConstructorTemplate(AnonymousSubstructConstructorTemplateNameI<'s, 'i>),
    AnonymousSubstructConstructor(AnonymousSubstructConstructorNameI<'s, 'i>),
    AnonymousSubstruct(AnonymousSubstructNameI<'s, 'i>),
    ResolvingEnv(ResolvingEnvNameI),
    CallEnv(CallEnvNameI),
}

// mig: impl INameI

// mig: enum ITemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ITemplateNameI<'s, 'i> {
    ExportTemplate(&'i ExportTemplateNameI<'s>),
    ImplTemplate(&'i ImplTemplateNameI<'s>),
    ImplBoundTemplate(&'i ImplBoundTemplateNameI<'s>),
    StaticSizedArrayTemplate(&'i StaticSizedArrayTemplateNameI),
    RuntimeSizedArrayTemplate(&'i RuntimeSizedArrayTemplateNameI),
    OverrideDispatcherTemplate(&'i OverrideDispatcherTemplateNameI<'s, 'i>),
    OverrideDispatcherCase(&'i OverrideDispatcherCaseNameI<'s, 'i>),
    ExternTemplate(&'i ExternTemplateNameI<'s>),
    ExternFunction(&'i ExternFunctionNameI<'s, 'i>),
    FunctionBoundTemplate(&'i FunctionBoundTemplateNameI<'s>),
    FunctionTemplate(&'i FunctionTemplateNameI<'s>),
    LambdaCallFunctionTemplate(&'i LambdaCallFunctionTemplateNameI<'s, 'i>),
    ForwarderFunctionTemplate(&'i ForwarderFunctionTemplateNameI<'s, 'i>),
    ConstructorTemplate(&'i ConstructorTemplateNameI<'s>),
    LambdaCitizenTemplate(&'i LambdaCitizenTemplateNameI<'s>),
    StructTemplate(&'i StructTemplateNameI<'s>),
    InterfaceTemplate(&'i InterfaceTemplateNameI<'s>),
    AnonymousSubstructImplTemplate(&'i AnonymousSubstructImplTemplateNameI<'s, 'i>),
    AnonymousSubstructTemplate(&'i AnonymousSubstructTemplateNameI<'s, 'i>),
    AnonymousSubstructConstructorTemplate(&'i AnonymousSubstructConstructorTemplateNameI<'s, 'i>),
}
// mig: impl ITemplateNameI

// Rust-only narrowing INameI -> ITemplateNameI (mirrors T-side TryFrom<INameT>). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for ITemplateNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::ExportTemplate(x) => Ok(ITemplateNameI::ExportTemplate(x)),
            INameI::ImplTemplate(x) => Ok(ITemplateNameI::ImplTemplate(x)),
            INameI::ImplBoundTemplate(x) => Ok(ITemplateNameI::ImplBoundTemplate(x)),
            INameI::StaticSizedArrayTemplate(x) => Ok(ITemplateNameI::StaticSizedArrayTemplate(x)),
            INameI::RuntimeSizedArrayTemplate(x) => Ok(ITemplateNameI::RuntimeSizedArrayTemplate(x)),
            INameI::OverrideDispatcherTemplate(x) => Ok(ITemplateNameI::OverrideDispatcherTemplate(x)),
            INameI::OverrideDispatcherCase(x) => Ok(ITemplateNameI::OverrideDispatcherCase(x)),
            INameI::ExternTemplate(x) => Ok(ITemplateNameI::ExternTemplate(x)),
            INameI::ExternFunction(x) => Ok(ITemplateNameI::ExternFunction(x)),
            INameI::FunctionBoundTemplate(x) => Ok(ITemplateNameI::FunctionBoundTemplate(x)),
            INameI::FunctionTemplate(x) => Ok(ITemplateNameI::FunctionTemplate(x)),
            INameI::LambdaCallFunctionTemplate(x) => Ok(ITemplateNameI::LambdaCallFunctionTemplate(x)),
            INameI::ForwarderFunctionTemplate(x) => Ok(ITemplateNameI::ForwarderFunctionTemplate(x)),
            INameI::ConstructorTemplate(x) => Ok(ITemplateNameI::ConstructorTemplate(x)),
            INameI::LambdaCitizenTemplate(x) => Ok(ITemplateNameI::LambdaCitizenTemplate(x)),
            INameI::StructTemplate(x) => Ok(ITemplateNameI::StructTemplate(x)),
            INameI::InterfaceTemplate(x) => Ok(ITemplateNameI::InterfaceTemplate(x)),
            INameI::AnonymousSubstructImplTemplate(x) => Ok(ITemplateNameI::AnonymousSubstructImplTemplate(x)),
            INameI::AnonymousSubstructTemplate(x) => Ok(ITemplateNameI::AnonymousSubstructTemplate(x)),
            INameI::AnonymousSubstructConstructorTemplate(x) => Ok(ITemplateNameI::AnonymousSubstructConstructorTemplate(x)),
            _ => Err(()),
        }
    }
}
// mig: enum IFunctionTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IFunctionTemplateNameI<'s, 'i> {
    OverrideDispatcherTemplate(&'i OverrideDispatcherTemplateNameI<'s, 'i>),
    ExternFunction(&'i ExternFunctionNameI<'s, 'i>),
    FunctionBoundTemplate(&'i FunctionBoundTemplateNameI<'s>),
    FunctionTemplate(&'i FunctionTemplateNameI<'s>),
    LambdaCallFunctionTemplate(&'i LambdaCallFunctionTemplateNameI<'s, 'i>),
    ForwarderFunctionTemplate(&'i ForwarderFunctionTemplateNameI<'s, 'i>),
    ConstructorTemplate(&'i ConstructorTemplateNameI<'s>),
    AnonymousSubstructConstructorTemplate(&'i AnonymousSubstructConstructorTemplateNameI<'s, 'i>),
}
// mig: impl IFunctionTemplateNameI

// Rust-only widening IFunctionTemplateNameI -> INameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> From<IFunctionTemplateNameI<'s, 'i>> for INameI<'s, 'i> {
    fn from(t: IFunctionTemplateNameI<'s, 'i>) -> Self {
        match t {
            IFunctionTemplateNameI::OverrideDispatcherTemplate(x) => INameI::OverrideDispatcherTemplate(x),
            IFunctionTemplateNameI::ExternFunction(x) => INameI::ExternFunction(x),
            IFunctionTemplateNameI::FunctionBoundTemplate(x) => INameI::FunctionBoundTemplate(x),
            IFunctionTemplateNameI::FunctionTemplate(x) => INameI::FunctionTemplate(x),
            IFunctionTemplateNameI::LambdaCallFunctionTemplate(x) => INameI::LambdaCallFunctionTemplate(x),
            IFunctionTemplateNameI::ForwarderFunctionTemplate(x) => INameI::ForwarderFunctionTemplate(x),
            IFunctionTemplateNameI::ConstructorTemplate(x) => INameI::ConstructorTemplate(x),
            IFunctionTemplateNameI::AnonymousSubstructConstructorTemplate(x) => INameI::AnonymousSubstructConstructorTemplate(x),
        }
    }
}

// Rust-only narrowing INameI -> IFunctionTemplateNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for IFunctionTemplateNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::OverrideDispatcherTemplate(x) => Ok(IFunctionTemplateNameI::OverrideDispatcherTemplate(x)),
            INameI::ExternFunction(x) => Ok(IFunctionTemplateNameI::ExternFunction(x)),
            INameI::FunctionBoundTemplate(x) => Ok(IFunctionTemplateNameI::FunctionBoundTemplate(x)),
            INameI::FunctionTemplate(x) => Ok(IFunctionTemplateNameI::FunctionTemplate(x)),
            INameI::LambdaCallFunctionTemplate(x) => Ok(IFunctionTemplateNameI::LambdaCallFunctionTemplate(x)),
            INameI::ForwarderFunctionTemplate(x) => Ok(IFunctionTemplateNameI::ForwarderFunctionTemplate(x)),
            INameI::ConstructorTemplate(x) => Ok(IFunctionTemplateNameI::ConstructorTemplate(x)),
            INameI::AnonymousSubstructConstructorTemplate(x) => Ok(IFunctionTemplateNameI::AnonymousSubstructConstructorTemplate(x)),
            _ => Err(()),
        }
    }
}
// mig: enum IInstantiationNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum IInstantiationNameI<'s, 'i> {
    Export(&'i ExportNameI<'s>),
    Impl(&'i ImplNameI<'s, 'i>),
    ImplBound(&'i ImplBoundNameI<'s, 'i>),
    StaticSizedArray(&'i StaticSizedArrayNameI<'s, 'i>),
    RuntimeSizedArray(&'i RuntimeSizedArrayNameI<'s, 'i>),
    OverrideDispatcher(&'i OverrideDispatcherNameI<'s, 'i>),
    OverrideDispatcherCase(&'i OverrideDispatcherCaseNameI<'s, 'i>),
    Extern(&'i ExternNameI<'s>),
    ExternFunction(&'i ExternFunctionNameI<'s, 'i>),
    Function(&'i FunctionNameIX<'s, 'i>),
    ForwarderFunction(&'i ForwarderFunctionNameI<'s, 'i>),
    FunctionBound(&'i FunctionBoundNameI<'s, 'i>),
    LambdaCallFunction(&'i LambdaCallFunctionNameI<'s, 'i>),
    Struct(&'i StructNameI<'s, 'i>),
    Interface(&'i InterfaceNameI<'s, 'i>),
    LambdaCitizen(&'i LambdaCitizenNameI<'s>),
    AnonymousSubstructImpl(&'i AnonymousSubstructImplNameI<'s, 'i>),
    AnonymousSubstructConstructor(&'i AnonymousSubstructConstructorNameI<'s, 'i>),
    AnonymousSubstruct(&'i AnonymousSubstructNameI<'s, 'i>),
}
// mig: impl IInstantiationNameI

// mig: fn template_args
impl<'s, 'i> IInstantiationNameI<'s, 'i> where 's: 'i {
    pub fn template_args(&self, interner: &InstantiatingInterner<'s, 'i>) -> &'i [ITemplataI<'s, 'i>] {
        match self {
            IInstantiationNameI::Export(_) => &[],
            IInstantiationNameI::Impl(x) => x.template_args,
            IInstantiationNameI::ImplBound(x) => x.template_args,
            IInstantiationNameI::StaticSizedArray(_) => {
                panic!("Unimplemented: template_args on StaticSizedArrayNameI (computed: needs interner to allocate slice)");
                // Vector(IntegerTemplataI(size), MutabilityTemplataI(arr.mutability), VariabilityTemplataI(variability), arr.elementType)
            }
            IInstantiationNameI::RuntimeSizedArray(_) => {
                panic!("Unimplemented: template_args on RuntimeSizedArrayNameI (computed: needs interner to allocate slice)");
                // Vector(MutabilityTemplataI(arr.mutability), arr.elementType)
            }
            IInstantiationNameI::OverrideDispatcher(x) => x.template_args,
            IInstantiationNameI::OverrideDispatcherCase(x) => x.independent_impl_template_args,
            IInstantiationNameI::Extern(_) => &[],
            IInstantiationNameI::ExternFunction(_) => {
                panic!("Unimplemented: template_args on ExternFunctionNameI (Scala: templateArgs field)");
                // ExternFunctionNameI.templateArgs
            }
            IInstantiationNameI::Function(x) => x.template_args,
            IInstantiationNameI::ForwarderFunction(f) => f.inner.template_args(),
            IInstantiationNameI::FunctionBound(x) => x.template_args,
            IInstantiationNameI::LambdaCallFunction(x) => x.template_args,
            IInstantiationNameI::Struct(x) => x.template_args,
            IInstantiationNameI::Interface(x) => x.template_args,
            IInstantiationNameI::LambdaCitizen(_) => &[],
            IInstantiationNameI::AnonymousSubstructImpl(x) => x.template_args,
            IInstantiationNameI::AnonymousSubstructConstructor(x) => x.template_args,
            IInstantiationNameI::AnonymousSubstruct(x) => x.template_args,
        }
    }
}

// mig: fn template
impl<'s, 'i> IInstantiationNameI<'s, 'i> where 's: 'i {
    pub fn template(&self) -> ITemplateNameI<'s, 'i> {
        match self {
            IInstantiationNameI::Export(x) => ITemplateNameI::ExportTemplate(&x.template),
            IInstantiationNameI::Impl(x) => match x.template {
                IImplTemplateNameI::ImplTemplate(t) => ITemplateNameI::ImplTemplate(t),
                IImplTemplateNameI::ImplBoundTemplate(t) => ITemplateNameI::ImplBoundTemplate(t),
                IImplTemplateNameI::AnonymousSubstructImplTemplate(t) => ITemplateNameI::AnonymousSubstructImplTemplate(t),
            },
            IInstantiationNameI::ImplBound(x) => ITemplateNameI::ImplBoundTemplate(&x.template),
            IInstantiationNameI::StaticSizedArray(x) => ITemplateNameI::StaticSizedArrayTemplate(&x.template),
            IInstantiationNameI::RuntimeSizedArray(x) => ITemplateNameI::RuntimeSizedArrayTemplate(&x.template),
            IInstantiationNameI::OverrideDispatcher(x) => ITemplateNameI::OverrideDispatcherTemplate(&x.template),
            IInstantiationNameI::OverrideDispatcherCase(x) => ITemplateNameI::OverrideDispatcherCase(x),
            IInstantiationNameI::Extern(x) => ITemplateNameI::ExternTemplate(&x.template),
            IInstantiationNameI::ExternFunction(x) => ITemplateNameI::ExternFunction(x),
            IInstantiationNameI::Function(x) => ITemplateNameI::FunctionTemplate(&x.template),
            IInstantiationNameI::ForwarderFunction(x) => ITemplateNameI::ForwarderFunctionTemplate(&x.template),
            IInstantiationNameI::FunctionBound(x) => ITemplateNameI::FunctionBoundTemplate(&x.template),
            IInstantiationNameI::LambdaCallFunction(x) => ITemplateNameI::LambdaCallFunctionTemplate(&x.template),
            IInstantiationNameI::Struct(x) => match x.template {
                IStructTemplateNameI::StructTemplate(t) => ITemplateNameI::StructTemplate(t),
                IStructTemplateNameI::LambdaCitizenTemplate(t) => ITemplateNameI::LambdaCitizenTemplate(t),
                IStructTemplateNameI::AnonymousSubstructTemplate(t) => ITemplateNameI::AnonymousSubstructTemplate(t),
            },
            IInstantiationNameI::Interface(x) => match x.template {
                IInterfaceTemplateNameI::InterfaceTemplate(t) => ITemplateNameI::InterfaceTemplate(t),
            },
            IInstantiationNameI::LambdaCitizen(x) => ITemplateNameI::LambdaCitizenTemplate(&x.template),
            IInstantiationNameI::AnonymousSubstructImpl(x) => ITemplateNameI::AnonymousSubstructImplTemplate(&x.template),
            IInstantiationNameI::AnonymousSubstructConstructor(x) => ITemplateNameI::AnonymousSubstructConstructorTemplate(&x.template),
            IInstantiationNameI::AnonymousSubstruct(x) => ITemplateNameI::AnonymousSubstructTemplate(&x.template),
        }
    }
}
// Rust-only narrowing from the wide INameI to the IInstantiationNameI subset
// (mirrors the T-side `TryFrom<INameT> for IInstantiationNameT`). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for IInstantiationNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::Export(x) => Ok(IInstantiationNameI::Export(x)),
            INameI::Impl(x) => Ok(IInstantiationNameI::Impl(x)),
            INameI::ImplBound(x) => Ok(IInstantiationNameI::ImplBound(x)),
            INameI::StaticSizedArray(x) => Ok(IInstantiationNameI::StaticSizedArray(x)),
            INameI::RuntimeSizedArray(x) => Ok(IInstantiationNameI::RuntimeSizedArray(x)),
            INameI::OverrideDispatcher(x) => Ok(IInstantiationNameI::OverrideDispatcher(x)),
            INameI::OverrideDispatcherCase(x) => Ok(IInstantiationNameI::OverrideDispatcherCase(x)),
            INameI::Extern(x) => Ok(IInstantiationNameI::Extern(x)),
            INameI::ExternFunction(x) => Ok(IInstantiationNameI::ExternFunction(x)),
            INameI::FunctionNameIX(x) => Ok(IInstantiationNameI::Function(x)),
            INameI::ForwarderFunction(x) => Ok(IInstantiationNameI::ForwarderFunction(x)),
            INameI::FunctionBound(x) => Ok(IInstantiationNameI::FunctionBound(x)),
            INameI::LambdaCallFunction(x) => Ok(IInstantiationNameI::LambdaCallFunction(x)),
            INameI::StructName(x) => Ok(IInstantiationNameI::Struct(x)),
            INameI::InterfaceName(x) => Ok(IInstantiationNameI::Interface(x)),
            INameI::LambdaCitizen(x) => Ok(IInstantiationNameI::LambdaCitizen(x)),
            INameI::AnonymousSubstructImpl(x) => Ok(IInstantiationNameI::AnonymousSubstructImpl(x)),
            INameI::AnonymousSubstructConstructor(x) => Ok(IInstantiationNameI::AnonymousSubstructConstructor(x)),
            INameI::AnonymousSubstruct(x) => Ok(IInstantiationNameI::AnonymousSubstruct(x)),
            _ => Err(()),
        }
    }
}
// mig: enum IFunctionNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IFunctionNameI<'s, 'i> {
    OverrideDispatcher(&'i OverrideDispatcherNameI<'s, 'i>),
    ExternFunction(&'i ExternFunctionNameI<'s, 'i>),
    Function(&'i FunctionNameIX<'s, 'i>),
    ForwarderFunction(&'i ForwarderFunctionNameI<'s, 'i>),
    FunctionBound(&'i FunctionBoundNameI<'s, 'i>),
    LambdaCallFunction(&'i LambdaCallFunctionNameI<'s, 'i>),
    AnonymousSubstructConstructor(&'i AnonymousSubstructConstructorNameI<'s, 'i>),
}
// mig: impl IFunctionNameI

// mig: fn template_args
impl<'s, 'i> IFunctionNameI<'s, 'i> where 's: 'i {
    pub fn template_args(&self) -> &'i [ITemplataI<'s, 'i>] {
        match self {
            IFunctionNameI::OverrideDispatcher(x) => x.template_args,
            IFunctionNameI::ExternFunction(_) => {
                panic!("Unimplemented: template_args on ExternFunctionNameI (Scala: templateArgs field)");
                // ExternFunctionNameI.templateArgs
            }
            IFunctionNameI::Function(x) => x.template_args,
            IFunctionNameI::ForwarderFunction(f) => f.inner.template_args(),
            IFunctionNameI::FunctionBound(x) => x.template_args,
            IFunctionNameI::LambdaCallFunction(x) => x.template_args,
            IFunctionNameI::AnonymousSubstructConstructor(x) => x.template_args,
        }
    }
// mig: fn template
    pub fn template(&self) -> IFunctionTemplateNameI<'s, 'i> {
        match self {
            IFunctionNameI::OverrideDispatcher(x) => IFunctionTemplateNameI::OverrideDispatcherTemplate(&x.template),
            IFunctionNameI::ExternFunction(_) => {
                panic!("Unimplemented: template on ExternFunctionNameI");
                // this
            }
            IFunctionNameI::Function(x) => IFunctionTemplateNameI::FunctionTemplate(&x.template),
            IFunctionNameI::ForwarderFunction(x) => IFunctionTemplateNameI::ForwarderFunctionTemplate(&x.template),
            IFunctionNameI::FunctionBound(x) => IFunctionTemplateNameI::FunctionBoundTemplate(&x.template),
            IFunctionNameI::LambdaCallFunction(x) => IFunctionTemplateNameI::LambdaCallFunctionTemplate(&x.template),
            IFunctionNameI::AnonymousSubstructConstructor(x) => IFunctionTemplateNameI::AnonymousSubstructConstructorTemplate(&x.template),
        }
    }
// mig: fn parameters
    pub fn parameters(&self) -> &'i [CoordI<'s, 'i>] {
        match self {
            IFunctionNameI::OverrideDispatcher(f) => f.parameters,
            IFunctionNameI::ExternFunction(f) => f.parameters,
            IFunctionNameI::Function(f) => f.parameters,
            IFunctionNameI::ForwarderFunction(f) => f.inner.parameters(),
            IFunctionNameI::FunctionBound(f) => f.parameters,
            IFunctionNameI::LambdaCallFunction(f) => f.parameters,
            IFunctionNameI::AnonymousSubstructConstructor(f) => f.parameters,
        }
    }
}
// Rust-only narrowing INameI -> IFunctionNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for IFunctionNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::OverrideDispatcher(x) => Ok(IFunctionNameI::OverrideDispatcher(x)),
            INameI::ExternFunction(x) => Ok(IFunctionNameI::ExternFunction(x)),
            INameI::FunctionNameIX(x) => Ok(IFunctionNameI::Function(x)),
            INameI::ForwarderFunction(x) => Ok(IFunctionNameI::ForwarderFunction(x)),
            INameI::FunctionBound(x) => Ok(IFunctionNameI::FunctionBound(x)),
            INameI::LambdaCallFunction(x) => Ok(IFunctionNameI::LambdaCallFunction(x)),
            INameI::AnonymousSubstructConstructor(x) => Ok(IFunctionNameI::AnonymousSubstructConstructor(x)),
            _ => Err(()),
        }
    }
}
// Rust-only widening IFunctionNameI -> INameI (mirrors Scala `IFunctionNameI extends INameI`
// subtyping; reverse of the TryFrom above, same as the template-name From widenings below). No Scala counterpart.
impl<'s, 'i> From<IFunctionNameI<'s, 'i>> for INameI<'s, 'i> where 's: 'i {
    fn from(name: IFunctionNameI<'s, 'i>) -> Self {
        match name {
            IFunctionNameI::OverrideDispatcher(x) => INameI::OverrideDispatcher(x),
            IFunctionNameI::ExternFunction(x) => INameI::ExternFunction(x),
            IFunctionNameI::Function(x) => INameI::FunctionNameIX(x),
            IFunctionNameI::ForwarderFunction(x) => INameI::ForwarderFunction(x),
            IFunctionNameI::FunctionBound(x) => INameI::FunctionBound(x),
            IFunctionNameI::LambdaCallFunction(x) => INameI::LambdaCallFunction(x),
            IFunctionNameI::AnonymousSubstructConstructor(x) => INameI::AnonymousSubstructConstructor(x),
        }
    }
}
// mig: enum ISuperKindTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ISuperKindTemplateNameI<'s, 'i> {
    InterfaceTemplate(&'i InterfaceTemplateNameI<'s>),
}
// mig: impl ISuperKindTemplateNameI

// mig: enum ISubKindTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ISubKindTemplateNameI<'s, 'i> {
    StaticSizedArrayTemplate(&'i StaticSizedArrayTemplateNameI),
    RuntimeSizedArrayTemplate(&'i RuntimeSizedArrayTemplateNameI),
    LambdaCitizenTemplate(&'i LambdaCitizenTemplateNameI<'s>),
    StructTemplate(&'i StructTemplateNameI<'s>),
    InterfaceTemplate(&'i InterfaceTemplateNameI<'s>),
    AnonymousSubstructTemplate(&'i AnonymousSubstructTemplateNameI<'s, 'i>),
}
// mig: impl ISubKindTemplateNameI

// mig: enum ICitizenTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ICitizenTemplateNameI<'s, 'i> {
    StaticSizedArrayTemplate(&'i StaticSizedArrayTemplateNameI),
    RuntimeSizedArrayTemplate(&'i RuntimeSizedArrayTemplateNameI),
    LambdaCitizenTemplate(&'i LambdaCitizenTemplateNameI<'s>),
    StructTemplate(&'i StructTemplateNameI<'s>),
    InterfaceTemplate(&'i InterfaceTemplateNameI<'s>),
    AnonymousSubstructTemplate(&'i AnonymousSubstructTemplateNameI<'s, 'i>),
}
// mig: impl ICitizenTemplateNameI

// mig: enum IStructTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IStructTemplateNameI<'s, 'i> {
    LambdaCitizenTemplate(&'i LambdaCitizenTemplateNameI<'s>),
    StructTemplate(&'i StructTemplateNameI<'s>),
    AnonymousSubstructTemplate(&'i AnonymousSubstructTemplateNameI<'s, 'i>),
}
// Widening conversions mirroring the Scala `extends` hierarchy of the template-name traits
// (IStructTemplateNameI <: ICitizenTemplateNameI <: ISubKindTemplateNameI). Rust-only (no Scala
// counterpart — Scala uses subtyping); the T-side encodes the same widenings as `From` impls.
impl<'s, 'i> From<IStructTemplateNameI<'s, 'i>> for ICitizenTemplateNameI<'s, 'i> {
    fn from(t: IStructTemplateNameI<'s, 'i>) -> Self {
        match t {
            IStructTemplateNameI::LambdaCitizenTemplate(x) => ICitizenTemplateNameI::LambdaCitizenTemplate(x),
            IStructTemplateNameI::StructTemplate(x) => ICitizenTemplateNameI::StructTemplate(x),
            IStructTemplateNameI::AnonymousSubstructTemplate(x) => ICitizenTemplateNameI::AnonymousSubstructTemplate(x),
        }
    }
}
impl<'s, 'i> From<ICitizenTemplateNameI<'s, 'i>> for ISubKindTemplateNameI<'s, 'i> {
    fn from(t: ICitizenTemplateNameI<'s, 'i>) -> Self {
        match t {
            ICitizenTemplateNameI::StaticSizedArrayTemplate(x) => ISubKindTemplateNameI::StaticSizedArrayTemplate(x),
            ICitizenTemplateNameI::RuntimeSizedArrayTemplate(x) => ISubKindTemplateNameI::RuntimeSizedArrayTemplate(x),
            ICitizenTemplateNameI::LambdaCitizenTemplate(x) => ISubKindTemplateNameI::LambdaCitizenTemplate(x),
            ICitizenTemplateNameI::StructTemplate(x) => ISubKindTemplateNameI::StructTemplate(x),
            ICitizenTemplateNameI::InterfaceTemplate(x) => ISubKindTemplateNameI::InterfaceTemplate(x),
            ICitizenTemplateNameI::AnonymousSubstructTemplate(x) => ISubKindTemplateNameI::AnonymousSubstructTemplate(x),
        }
    }
}
// Rust-only narrowing INameI -> ICitizenTemplateNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for ICitizenTemplateNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::StaticSizedArrayTemplate(x) => Ok(ICitizenTemplateNameI::StaticSizedArrayTemplate(x)),
            INameI::RuntimeSizedArrayTemplate(x) => Ok(ICitizenTemplateNameI::RuntimeSizedArrayTemplate(x)),
            INameI::LambdaCitizenTemplate(x) => Ok(ICitizenTemplateNameI::LambdaCitizenTemplate(x)),
            INameI::StructTemplate(x) => Ok(ICitizenTemplateNameI::StructTemplate(x)),
            INameI::InterfaceTemplate(x) => Ok(ICitizenTemplateNameI::InterfaceTemplate(x)),
            INameI::AnonymousSubstructTemplate(x) => Ok(ICitizenTemplateNameI::AnonymousSubstructTemplate(x)),
            _ => Err(()),
        }
    }
}

// Rust-only widening IInterfaceTemplateNameI -> ICitizenTemplateNameI
// (IInterfaceTemplateNameI <: ICitizenTemplateNameI). Rust-only (no Scala counterpart — Scala uses subtyping).
impl<'s, 'i> From<IInterfaceTemplateNameI<'s, 'i>> for ICitizenTemplateNameI<'s, 'i> {
    fn from(t: IInterfaceTemplateNameI<'s, 'i>) -> Self {
        match t {
            IInterfaceTemplateNameI::InterfaceTemplate(x) => ICitizenTemplateNameI::InterfaceTemplate(x),
        }
    }
}

// Rust-only narrowing ICitizenTemplateNameI -> IStructTemplateNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<ICitizenTemplateNameI<'s, 'i>> for IStructTemplateNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: ICitizenTemplateNameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            ICitizenTemplateNameI::StructTemplate(x) => Ok(IStructTemplateNameI::StructTemplate(x)),
            ICitizenTemplateNameI::AnonymousSubstructTemplate(x) => Ok(IStructTemplateNameI::AnonymousSubstructTemplate(x)),
            ICitizenTemplateNameI::LambdaCitizenTemplate(x) => Ok(IStructTemplateNameI::LambdaCitizenTemplate(x)),
            _ => Err(()),
        }
    }
}

// Rust-only narrowing ICitizenTemplateNameI -> IInterfaceTemplateNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<ICitizenTemplateNameI<'s, 'i>> for IInterfaceTemplateNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: ICitizenTemplateNameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            ICitizenTemplateNameI::InterfaceTemplate(x) => Ok(IInterfaceTemplateNameI::InterfaceTemplate(x)),
            _ => Err(()),
        }
    }
}

// Rust-only narrowing INameI -> IStructTemplateNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for IStructTemplateNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::StructTemplate(x) => Ok(IStructTemplateNameI::StructTemplate(x)),
            INameI::AnonymousSubstructTemplate(x) => Ok(IStructTemplateNameI::AnonymousSubstructTemplate(x)),
            INameI::LambdaCitizenTemplate(x) => Ok(IStructTemplateNameI::LambdaCitizenTemplate(x)),
            _ => Err(()),
        }
    }
}
// mig: impl IStructTemplateNameI

// mig: enum IInterfaceTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IInterfaceTemplateNameI<'s, 'i> {
    InterfaceTemplate(&'i InterfaceTemplateNameI<'s>),
}
// mig: impl IInterfaceTemplateNameI

// mig: enum ISuperKindNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ISuperKindNameI<'s, 'i> {
    Interface(&'i InterfaceNameI<'s, 'i>),
}
// mig: impl ISuperKindNameI

// mig: fn template_args
impl<'s, 'i> ISuperKindNameI<'s, 'i> where 's: 'i {
    pub fn template_args(&self) -> &'i [ITemplataI<'s, 'i>] {
        match self {
            ISuperKindNameI::Interface(x) => x.template_args,
        }
    }
// mig: fn template
    pub fn template(&self) -> ISuperKindTemplateNameI<'s, 'i> {
        match self {
            ISuperKindNameI::Interface(x) => match x.template {
                IInterfaceTemplateNameI::InterfaceTemplate(t) => ISuperKindTemplateNameI::InterfaceTemplate(t),
            },
        }
    }
}
// Rust-only narrowing INameI -> ISuperKindNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for ISuperKindNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::InterfaceName(x) => Ok(ISuperKindNameI::Interface(x)),
            _ => Err(()),
        }
    }
}
// mig: enum ISubKindNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ISubKindNameI<'s, 'i> {
    StaticSizedArray(&'i StaticSizedArrayNameI<'s, 'i>),
    RuntimeSizedArray(&'i RuntimeSizedArrayNameI<'s, 'i>),
    Struct(&'i StructNameI<'s, 'i>),
    Interface(&'i InterfaceNameI<'s, 'i>),
    LambdaCitizen(&'i LambdaCitizenNameI<'s>),
    AnonymousSubstruct(&'i AnonymousSubstructNameI<'s, 'i>),
}
// mig: impl ISubKindNameI

// mig: fn template_args
impl<'s, 'i> ISubKindNameI<'s, 'i> where 's: 'i {
    pub fn template_args(&self) -> &'i [ITemplataI<'s, 'i>] {
        match self {
            ISubKindNameI::StaticSizedArray(_) => {
                panic!("Unimplemented: template_args on StaticSizedArrayNameI (computed: needs interner to allocate slice)");
                // Vector(IntegerTemplataI(size), MutabilityTemplataI(arr.mutability), VariabilityTemplataI(variability), arr.elementType)
            }
            ISubKindNameI::RuntimeSizedArray(_) => {
                panic!("Unimplemented: template_args on RuntimeSizedArrayNameI (computed: needs interner to allocate slice)");
                // Vector(MutabilityTemplataI(arr.mutability), arr.elementType)
            }
            ISubKindNameI::Struct(x) => x.template_args,
            ISubKindNameI::Interface(x) => x.template_args,
            ISubKindNameI::LambdaCitizen(_) => &[],
            ISubKindNameI::AnonymousSubstruct(x) => x.template_args,
        }
    }
}
// mig: fn template
impl<'s, 'i> ISubKindNameI<'s, 'i> where 's: 'i {
    pub fn template(&self) -> ISubKindTemplateNameI<'s, 'i> {
        match self {
            ISubKindNameI::StaticSizedArray(x) => ISubKindTemplateNameI::StaticSizedArrayTemplate(&x.template),
            ISubKindNameI::RuntimeSizedArray(x) => ISubKindTemplateNameI::RuntimeSizedArrayTemplate(&x.template),
            ISubKindNameI::Struct(x) => ISubKindTemplateNameI::from(ICitizenTemplateNameI::from(x.template)),
            ISubKindNameI::Interface(x) => match x.template {
                IInterfaceTemplateNameI::InterfaceTemplate(t) => ISubKindTemplateNameI::InterfaceTemplate(t),
            },
            ISubKindNameI::LambdaCitizen(x) => ISubKindTemplateNameI::LambdaCitizenTemplate(&x.template),
            ISubKindNameI::AnonymousSubstruct(x) => ISubKindTemplateNameI::AnonymousSubstructTemplate(&x.template),
        }
    }
}
// Rust-only narrowing INameI -> ISubKindNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for ISubKindNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::StaticSizedArray(x) => Ok(ISubKindNameI::StaticSizedArray(x)),
            INameI::RuntimeSizedArray(x) => Ok(ISubKindNameI::RuntimeSizedArray(x)),
            INameI::StructName(x) => Ok(ISubKindNameI::Struct(x)),
            INameI::InterfaceName(x) => Ok(ISubKindNameI::Interface(x)),
            INameI::LambdaCitizen(x) => Ok(ISubKindNameI::LambdaCitizen(x)),
            INameI::AnonymousSubstruct(x) => Ok(ISubKindNameI::AnonymousSubstruct(x)),
            _ => Err(()),
        }
    }
}
// mig: enum ICitizenNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ICitizenNameI<'s, 'i> {
    StaticSizedArray(&'i StaticSizedArrayNameI<'s, 'i>),
    RuntimeSizedArray(&'i RuntimeSizedArrayNameI<'s, 'i>),
    Struct(&'i StructNameI<'s, 'i>),
    Interface(&'i InterfaceNameI<'s, 'i>),
    LambdaCitizen(&'i LambdaCitizenNameI<'s>),
    AnonymousSubstruct(&'i AnonymousSubstructNameI<'s, 'i>),
}
// mig: impl ICitizenNameI

// mig: fn template_args
impl<'s, 'i> ICitizenNameI<'s, 'i> where 's: 'i {
    pub fn template_args(&self) -> &'i [ITemplataI<'s, 'i>] {
        match self {
            ICitizenNameI::StaticSizedArray(_) => {
                panic!("Unimplemented: template_args on StaticSizedArrayNameI (computed: needs interner to allocate slice)");
                // Vector(IntegerTemplataI(size), MutabilityTemplataI(arr.mutability), VariabilityTemplataI(variability), arr.elementType)
            }
            ICitizenNameI::RuntimeSizedArray(_) => {
                panic!("Unimplemented: template_args on RuntimeSizedArrayNameI (computed: needs interner to allocate slice)");
                // Vector(MutabilityTemplataI(arr.mutability), arr.elementType)
            }
            ICitizenNameI::Struct(x) => x.template_args,
            ICitizenNameI::Interface(x) => x.template_args,
            ICitizenNameI::LambdaCitizen(_) => &[],
            ICitizenNameI::AnonymousSubstruct(x) => x.template_args,
        }
    }
}
// mig: fn template
impl<'s, 'i> ICitizenNameI<'s, 'i> where 's: 'i {
    pub fn template(&self) -> ICitizenTemplateNameI<'s, 'i> {
        match self {
            ICitizenNameI::StaticSizedArray(x) => ICitizenTemplateNameI::StaticSizedArrayTemplate(&x.template),
            ICitizenNameI::RuntimeSizedArray(x) => ICitizenTemplateNameI::RuntimeSizedArrayTemplate(&x.template),
            ICitizenNameI::Struct(x) => ICitizenTemplateNameI::from(x.template),
            ICitizenNameI::Interface(x) => match x.template {
                IInterfaceTemplateNameI::InterfaceTemplate(t) => ICitizenTemplateNameI::InterfaceTemplate(t),
            },
            ICitizenNameI::LambdaCitizen(x) => ICitizenTemplateNameI::LambdaCitizenTemplate(&x.template),
            ICitizenNameI::AnonymousSubstruct(x) => ICitizenTemplateNameI::AnonymousSubstructTemplate(&x.template),
        }
    }
}
// Rust-only narrowing INameI -> ICitizenNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for ICitizenNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::StaticSizedArray(x) => Ok(ICitizenNameI::StaticSizedArray(x)),
            INameI::RuntimeSizedArray(x) => Ok(ICitizenNameI::RuntimeSizedArray(x)),
            INameI::StructName(x) => Ok(ICitizenNameI::Struct(x)),
            INameI::InterfaceName(x) => Ok(ICitizenNameI::Interface(x)),
            INameI::LambdaCitizen(x) => Ok(ICitizenNameI::LambdaCitizen(x)),
            INameI::AnonymousSubstruct(x) => Ok(ICitizenNameI::AnonymousSubstruct(x)),
            _ => Err(()),
        }
    }
}
// Rust-only widening ICitizenNameI -> INameI (mirrors Scala subtyping; reverse of the TryFrom above,
// same family as the IFunctionNameI widening — feeds translateCitizenId's IdI.local_name). No Scala counterpart.
impl<'s, 'i> From<ICitizenNameI<'s, 'i>> for INameI<'s, 'i> where 's: 'i {
    fn from(name: ICitizenNameI<'s, 'i>) -> Self {
        match name {
            ICitizenNameI::StaticSizedArray(x) => INameI::StaticSizedArray(x),
            ICitizenNameI::RuntimeSizedArray(x) => INameI::RuntimeSizedArray(x),
            ICitizenNameI::Struct(x) => INameI::StructName(x),
            ICitizenNameI::Interface(x) => INameI::InterfaceName(x),
            ICitizenNameI::LambdaCitizen(x) => INameI::LambdaCitizen(x),
            ICitizenNameI::AnonymousSubstruct(x) => INameI::AnonymousSubstruct(x),
        }
    }
}
// mig: enum IStructNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IStructNameI<'s, 'i> {
    Struct(&'i StructNameI<'s, 'i>),
    LambdaCitizen(&'i LambdaCitizenNameI<'s>),
    AnonymousSubstruct(&'i AnonymousSubstructNameI<'s, 'i>),
}
// mig: impl IStructNameI

// mig: fn template_args
impl<'s, 'i> IStructNameI<'s, 'i> where 's: 'i {
    pub fn template_args(&self) -> &'i [ITemplataI<'s, 'i>] {
        match self {
            IStructNameI::Struct(x) => x.template_args,
            IStructNameI::LambdaCitizen(_) => &[],
            IStructNameI::AnonymousSubstruct(x) => x.template_args,
        }
    }
}
// mig: fn template
impl<'s, 'i> IStructNameI<'s, 'i> where 's: 'i {
    pub fn template(&self) -> IStructTemplateNameI<'s, 'i> {
        match self {
            IStructNameI::Struct(x) => x.template,
            IStructNameI::LambdaCitizen(x) => IStructTemplateNameI::LambdaCitizenTemplate(&x.template),
            IStructNameI::AnonymousSubstruct(x) => IStructTemplateNameI::AnonymousSubstructTemplate(&x.template),
        }
    }
}
// Rust-only narrowing INameI -> IStructNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for IStructNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::StructName(x) => Ok(IStructNameI::Struct(x)),
            INameI::LambdaCitizen(x) => Ok(IStructNameI::LambdaCitizen(x)),
            INameI::AnonymousSubstruct(x) => Ok(IStructNameI::AnonymousSubstruct(x)),
            _ => Err(()),
        }
    }
}
// Rust-only widening IStructNameI -> INameI (mirrors Scala subtyping; reverse of the TryFrom above,
// same family as the IFunctionNameI widening — feeds translateStructId's IdI.local_name). No Scala counterpart.
impl<'s, 'i> From<IStructNameI<'s, 'i>> for INameI<'s, 'i> where 's: 'i {
    fn from(name: IStructNameI<'s, 'i>) -> Self {
        match name {
            IStructNameI::Struct(x) => INameI::StructName(x),
            IStructNameI::LambdaCitizen(x) => INameI::LambdaCitizen(x),
            IStructNameI::AnonymousSubstruct(x) => INameI::AnonymousSubstruct(x),
        }
    }
}
// Rust-only widening IStructNameI -> ICitizenNameI (mirrors Scala `IStructNameI extends ICitizenNameI`
// subtyping; same shape as the INameI widening just above). No Scala counterpart.
impl<'s, 'i> From<IStructNameI<'s, 'i>> for ICitizenNameI<'s, 'i> where 's: 'i {
    fn from(name: IStructNameI<'s, 'i>) -> Self {
        match name {
            IStructNameI::Struct(x) => ICitizenNameI::Struct(x),
            IStructNameI::LambdaCitizen(x) => ICitizenNameI::LambdaCitizen(x),
            IStructNameI::AnonymousSubstruct(x) => ICitizenNameI::AnonymousSubstruct(x),
        }
    }
}

// Rust-only widening IInterfaceNameI -> ICitizenNameI (mirrors Scala `IInterfaceNameI extends
// ICitizenNameI` subtyping; same family as the IStructNameI widening just above). No Scala counterpart.
impl<'s, 'i> From<IInterfaceNameI<'s, 'i>> for ICitizenNameI<'s, 'i> where 's: 'i {
    fn from(name: IInterfaceNameI<'s, 'i>) -> Self {
        match name {
            IInterfaceNameI::Interface(x) => ICitizenNameI::Interface(x),
        }
    }
}

// Rust-only widening IStructTemplateNameI -> INameI (mirrors Scala `IStructTemplateNameI extends
// ITemplateNameI extends INameI`; needed so humanize_name can recurse on a struct name's `template`
// without inline matching at the call site). No Scala counterpart.
impl<'s, 'i> From<IStructTemplateNameI<'s, 'i>> for INameI<'s, 'i> where 's: 'i {
    fn from(name: IStructTemplateNameI<'s, 'i>) -> Self {
        match name {
            IStructTemplateNameI::StructTemplate(x) => INameI::StructTemplate(x),
            IStructTemplateNameI::AnonymousSubstructTemplate(x) => INameI::AnonymousSubstructTemplate(x),
            IStructTemplateNameI::LambdaCitizenTemplate(x) => INameI::LambdaCitizenTemplate(x),
        }
    }
}

// Rust-only widening IInterfaceTemplateNameI -> INameI. No Scala counterpart.
impl<'s, 'i> From<IInterfaceTemplateNameI<'s, 'i>> for INameI<'s, 'i> where 's: 'i {
    fn from(name: IInterfaceTemplateNameI<'s, 'i>) -> Self {
        match name {
            IInterfaceTemplateNameI::InterfaceTemplate(x) => INameI::InterfaceTemplate(x),
        }
    }
}

// Rust-only widening ICitizenTemplateNameI -> INameI. No Scala counterpart.
impl<'s, 'i> From<ICitizenTemplateNameI<'s, 'i>> for INameI<'s, 'i> where 's: 'i {
    fn from(name: ICitizenTemplateNameI<'s, 'i>) -> Self {
        match name {
            ICitizenTemplateNameI::StaticSizedArrayTemplate(x) => INameI::StaticSizedArrayTemplate(x),
            ICitizenTemplateNameI::RuntimeSizedArrayTemplate(x) => INameI::RuntimeSizedArrayTemplate(x),
            ICitizenTemplateNameI::LambdaCitizenTemplate(x) => INameI::LambdaCitizenTemplate(x),
            ICitizenTemplateNameI::StructTemplate(x) => INameI::StructTemplate(x),
            ICitizenTemplateNameI::InterfaceTemplate(x) => INameI::InterfaceTemplate(x),
            ICitizenTemplateNameI::AnonymousSubstructTemplate(x) => INameI::AnonymousSubstructTemplate(x),
        }
    }
}

// mig: enum IInterfaceNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IInterfaceNameI<'s, 'i> {
    Interface(&'i InterfaceNameI<'s, 'i>),
}
// mig: impl IInterfaceNameI

// mig: fn template_args
impl<'s, 'i> IInterfaceNameI<'s, 'i> where 's: 'i {
    pub fn template_args(&self) -> &'i [ITemplataI<'s, 'i>] {
        match self {
            IInterfaceNameI::Interface(x) => x.template_args,
        }
    }
// mig: fn template
    pub fn template(&self) -> &'i InterfaceTemplateNameI<'s> {
        match self {
            IInterfaceNameI::Interface(x) => match x.template {
                IInterfaceTemplateNameI::InterfaceTemplate(t) => t,
            },
        }
    }
}
// Rust-only narrowing INameI -> IInterfaceNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for IInterfaceNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::InterfaceName(x) => Ok(IInterfaceNameI::Interface(x)),
            _ => Err(()),
        }
    }
}
// Rust-only widening IInterfaceNameI -> INameI (mirrors Scala subtyping; reverse of the TryFrom above,
// same family as the IFunctionNameI widening — feeds translateInterfaceId's IdI.local_name). No Scala counterpart.
impl<'s, 'i> From<IInterfaceNameI<'s, 'i>> for INameI<'s, 'i> where 's: 'i {
    fn from(name: IInterfaceNameI<'s, 'i>) -> Self {
        match name {
            IInterfaceNameI::Interface(x) => INameI::InterfaceName(x),
        }
    }
}
// mig: enum IImplTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IImplTemplateNameI<'s, 'i> {
    ImplTemplate(&'i ImplTemplateNameI<'s>),
    ImplBoundTemplate(&'i ImplBoundTemplateNameI<'s>),
    AnonymousSubstructImplTemplate(&'i AnonymousSubstructImplTemplateNameI<'s, 'i>),
}
// mig: impl IImplTemplateNameI

// mig: enum IImplNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IImplNameI<'s, 'i> {
    Impl(&'i ImplNameI<'s, 'i>),
    ImplBound(&'i ImplBoundNameI<'s, 'i>),
    AnonymousSubstructImpl(&'i AnonymousSubstructImplNameI<'s, 'i>),
}
// mig: impl IImplNameI

// mig: fn template_args
impl<'s, 'i> IImplNameI<'s, 'i> where 's: 'i {
    pub fn template_args(&self) -> &'i [ITemplataI<'s, 'i>] {
        match self {
            IImplNameI::Impl(x) => x.template_args,
            IImplNameI::ImplBound(x) => x.template_args,
            IImplNameI::AnonymousSubstructImpl(x) => x.template_args,
        }
    }
}
// mig: fn template
impl<'s, 'i> IImplNameI<'s, 'i> where 's: 'i {
    pub fn template(&self) -> IImplTemplateNameI<'s, 'i> {
        match self {
            IImplNameI::Impl(x) => x.template,
            IImplNameI::ImplBound(x) => IImplTemplateNameI::ImplBoundTemplate(&x.template),
            IImplNameI::AnonymousSubstructImpl(x) => IImplTemplateNameI::AnonymousSubstructImplTemplate(&x.template),
        }
    }
}
// Rust-only narrowing INameI -> IImplNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for IImplNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::Impl(x) => Ok(IImplNameI::Impl(x)),
            INameI::ImplBound(x) => Ok(IImplNameI::ImplBound(x)),
            INameI::AnonymousSubstructImpl(x) => Ok(IImplNameI::AnonymousSubstructImpl(x)),
            _ => Err(()),
        }
    }
}
// Rust-only widening IImplNameI -> INameI (mirrors Scala subtyping; reverse of the TryFrom above,
// same family as the IFunctionNameI widening — feeds translateImplId's IdI.local_name). No Scala counterpart.
impl<'s, 'i> From<IImplNameI<'s, 'i>> for INameI<'s, 'i> where 's: 'i {
    fn from(name: IImplNameI<'s, 'i>) -> Self {
        match name {
            IImplNameI::Impl(x) => INameI::Impl(x),
            IImplNameI::ImplBound(x) => INameI::ImplBound(x),
            IImplNameI::AnonymousSubstructImpl(x) => INameI::AnonymousSubstructImpl(x),
        }
    }
}
// mig: enum IRegionNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IRegionNameI<'s, 'i> {
    _Phantom(PhantomData<(&'s (), &'i ())>),
}
// mig: impl IRegionNameI

// mig: struct RegionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct RegionNameI<'s> {
    pub rune: IRuneS<'s>,
}

// mig: struct DenizenDefaultRegionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct DenizenDefaultRegionNameI;

// mig: struct ExportTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExportTemplateNameI<'s> {
    pub code_loc: CodeLocationS<'s>,
}

// mig: struct ExportNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExportNameI<'s> {
    pub template: ExportTemplateNameI<'s>,
}

// mig: struct ExternTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExternTemplateNameI<'s> {
    pub code_loc: CodeLocationS<'s>,
}

// mig: struct ExternNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExternNameI<'s> {
    pub template: ExternTemplateNameI<'s>,
}

// mig: struct ImplTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ImplTemplateNameI<'s> {
    pub code_location: CodeLocationS<'s>,
}

// mig: struct ImplNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ImplNameI<'s, 'i> {
    pub template: IImplTemplateNameI<'s, 'i>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
    pub sub_citizen: ICitizenIT<'s, 'i>,
}

// mig: struct ImplBoundTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ImplBoundTemplateNameI<'s> {
    pub code_location: CodeLocationS<'s>,
}

// mig: struct ImplBoundNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ImplBoundNameI<'s, 'i> {
    pub template: ImplBoundTemplateNameI<'s>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
}


// mig: struct LetNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct LetNameI<'s> {
    pub code_location: CodeLocationS<'s>,
}

// mig: struct ExportAsNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExportAsNameI<'s> {
    pub code_location: CodeLocationS<'s>,
}

// mig: struct RawArrayNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct RawArrayNameI<'s, 'i> {
    pub mutability: MutabilityI,
    pub element_type: CoordTemplataI<'s, 'i>,
    pub self_region: RegionT,
}


// mig: struct ReachablePrototypeNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ReachablePrototypeNameI {
    pub num: i32,
}

// mig: struct StaticSizedArrayTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct StaticSizedArrayTemplateNameI;

// mig: struct StaticSizedArrayNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct StaticSizedArrayNameI<'s, 'i> {
    pub template: StaticSizedArrayTemplateNameI,
    pub size: i64,
    pub variability: VariabilityI,
    pub arr: RawArrayNameI<'s, 'i>,
}


// mig: fn template_args
// (was cfg-gated)
impl<'s, 'i> StaticSizedArrayNameI<'s, 'i> {
    pub fn template_args(&self) -> &'i[ITemplataI<'s, 'i>] { panic!("Unimplemented: template_args"); }
}

// mig: struct RuntimeSizedArrayTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct RuntimeSizedArrayTemplateNameI;

// mig: struct RuntimeSizedArrayNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct RuntimeSizedArrayNameI<'s, 'i> {
    pub template: RuntimeSizedArrayTemplateNameI,
    pub arr: RawArrayNameI<'s, 'i>,
}

// mig: fn template_args
// (was cfg-gated)
impl<'s, 'i> RuntimeSizedArrayNameI<'s, 'i> {
    pub fn template_args(&self) -> &'i[ITemplataI<'s, 'i>] { panic!("Unimplemented: template_args"); }
}

// mig: struct OverrideDispatcherTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct OverrideDispatcherTemplateNameI<'s, 'i> {
    pub impl_id: IdI<'s, 'i>,
}

// mig: struct OverrideDispatcherNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct OverrideDispatcherNameI<'s, 'i> {
    pub template: OverrideDispatcherTemplateNameI<'s, 'i>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
    pub parameters: &'i[CoordI<'s, 'i>],
}


// mig: struct OverrideDispatcherCaseNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct OverrideDispatcherCaseNameI<'s, 'i> {
    pub independent_impl_template_args: &'i[ITemplataI<'s, 'i>],
}


// mig: struct CaseFunctionFromImplNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct CaseFunctionFromImplNameI<'s, 'i> {
    pub template: CaseFunctionFromImplTemplateNameI<'s>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
    pub parameters: &'i[CoordI<'s, 'i>],
}


// mig: struct CaseFunctionFromImplTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct CaseFunctionFromImplTemplateNameI<'s> {
    pub human_name: StrI<'s>,
    pub rune_in_impl: IRuneS<'s>,
    pub rune_in_citizen: IRuneS<'s>,
}


// mig: enum IVarNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IVarNameI<'s, 'i> {
    TypingPassBlockResultVar(&'i TypingPassBlockResultVarNameI<'i>),
    TypingPassFunctionResultVar(&'i TypingPassFunctionResultVarNameI),
    TypingPassTemporaryVar(&'i TypingPassTemporaryVarNameI<'i>),
    TypingPassPatternMember(&'i TypingPassPatternMemberNameI<'i>),
    TypingIgnoredParam(&'i TypingIgnoredParamNameI),
    TypingPassPatternDestructuree(&'i TypingPassPatternDestructureeNameI<'i>),
    UnnamedLocal(&'i UnnamedLocalNameI<'s>),
    ClosureParam(&'i ClosureParamNameI<'s>),
    ConstructingMember(&'i ConstructingMemberNameI<'s>),
    WhileCondResult(&'i WhileCondResultNameI<'s>),
    Iterable(&'i IterableNameI<'s>),
    Iterator(&'i IteratorNameI<'s>),
    IterationOption(&'i IterationOptionNameI<'s>),
    MagicParam(&'i MagicParamNameI<'s>),
    CodeVar(&'i CodeVarNameI<'s>),
    AnonymousSubstructMember(&'i AnonymousSubstructMemberNameI),
    Self_(&'i SelfNameI),
}
// mig: impl IVarNameI

// Rust-only narrowing INameI -> IVarNameI (mirrors T-side). No Scala counterpart.
impl<'s, 'i> TryFrom<INameI<'s, 'i>> for IVarNameI<'s, 'i> where 's: 'i {
    type Error = ();
    fn try_from(name: INameI<'s, 'i>) -> Result<Self, ()> {
        match name {
            INameI::TypingPassBlockResultVar(x) => Ok(IVarNameI::TypingPassBlockResultVar(x)),
            INameI::TypingPassFunctionResultVar(x) => Ok(IVarNameI::TypingPassFunctionResultVar(x)),
            INameI::TypingPassTemporaryVar(x) => Ok(IVarNameI::TypingPassTemporaryVar(x)),
            INameI::TypingPassPatternMember(x) => Ok(IVarNameI::TypingPassPatternMember(x)),
            INameI::TypingIgnoredParam(x) => Ok(IVarNameI::TypingIgnoredParam(x)),
            INameI::TypingPassPatternDestructuree(x) => Ok(IVarNameI::TypingPassPatternDestructuree(x)),
            INameI::UnnamedLocal(x) => Ok(IVarNameI::UnnamedLocal(x)),
            INameI::ClosureParam(x) => Ok(IVarNameI::ClosureParam(x)),
            INameI::ConstructingMember(x) => Ok(IVarNameI::ConstructingMember(x)),
            INameI::WhileCondResult(x) => Ok(IVarNameI::WhileCondResult(x)),
            INameI::Iterable(x) => Ok(IVarNameI::Iterable(x)),
            INameI::Iterator(x) => Ok(IVarNameI::Iterator(x)),
            INameI::IterationOption(x) => Ok(IVarNameI::IterationOption(x)),
            INameI::MagicParam(x) => Ok(IVarNameI::MagicParam(x)),
            INameI::CodeVar(x) => Ok(IVarNameI::CodeVar(x)),
            INameI::AnonymousSubstructMember(x) => Ok(IVarNameI::AnonymousSubstructMember(x)),
            INameI::Self_(x) => Ok(IVarNameI::Self_(x)),
            _ => Err(()),
        }
    }
}
// Rust-only widening IVarNameI -> INameI (mirrors Scala `IVarNameI extends INameI` subtyping;
// reverse of the TryFrom above, same shape as the From<IFunctionNameI> widening). No Scala counterpart.
impl<'s, 'i> From<IVarNameI<'s, 'i>> for INameI<'s, 'i> where 's: 'i {
    fn from(name: IVarNameI<'s, 'i>) -> Self {
        match name {
            IVarNameI::TypingPassBlockResultVar(x) => INameI::TypingPassBlockResultVar(x),
            IVarNameI::TypingPassFunctionResultVar(x) => INameI::TypingPassFunctionResultVar(x),
            IVarNameI::TypingPassTemporaryVar(x) => INameI::TypingPassTemporaryVar(x),
            IVarNameI::TypingPassPatternMember(x) => INameI::TypingPassPatternMember(x),
            IVarNameI::TypingIgnoredParam(x) => INameI::TypingIgnoredParam(x),
            IVarNameI::TypingPassPatternDestructuree(x) => INameI::TypingPassPatternDestructuree(x),
            IVarNameI::UnnamedLocal(x) => INameI::UnnamedLocal(x),
            IVarNameI::ClosureParam(x) => INameI::ClosureParam(x),
            IVarNameI::ConstructingMember(x) => INameI::ConstructingMember(x),
            IVarNameI::WhileCondResult(x) => INameI::WhileCondResult(x),
            IVarNameI::Iterable(x) => INameI::Iterable(x),
            IVarNameI::Iterator(x) => INameI::Iterator(x),
            IVarNameI::IterationOption(x) => INameI::IterationOption(x),
            IVarNameI::MagicParam(x) => INameI::MagicParam(x),
            IVarNameI::CodeVar(x) => INameI::CodeVar(x),
            IVarNameI::AnonymousSubstructMember(x) => INameI::AnonymousSubstructMember(x),
            IVarNameI::Self_(x) => INameI::Self_(x),
        }
    }
}

// mig: struct TypingPassBlockResultVarNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingPassBlockResultVarNameI<'i> {
    pub life: LocationInFunctionEnvironmentI<'i>,
}

// mig: struct TypingPassFunctionResultVarNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingPassFunctionResultVarNameI;

// mig: struct TypingPassTemporaryVarNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingPassTemporaryVarNameI<'i> {
    pub life: LocationInFunctionEnvironmentI<'i>,
}

// mig: struct TypingPassPatternMemberNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingPassPatternMemberNameI<'i> {
    pub life: LocationInFunctionEnvironmentI<'i>,
}

// mig: struct TypingIgnoredParamNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingIgnoredParamNameI {
    pub num: i32,
}

// mig: struct TypingPassPatternDestructureeNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingPassPatternDestructureeNameI<'i> {
    pub life: LocationInFunctionEnvironmentI<'i>,
}

// mig: struct UnnamedLocalNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct UnnamedLocalNameI<'s> {
    pub code_location: CodeLocationS<'s>,
}

// mig: struct ClosureParamNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ClosureParamNameI<'s> {
    pub code_location: CodeLocationS<'s>,
}

// mig: struct ConstructingMemberNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ConstructingMemberNameI<'s> {
    pub name: StrI<'s>,
}

// mig: struct WhileCondResultNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct WhileCondResultNameI<'s> {
    pub range: RangeS<'s>,
}

// mig: struct IterableNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct IterableNameI<'s> {
    pub range: RangeS<'s>,
}

// mig: struct IteratorNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct IteratorNameI<'s> {
    pub range: RangeS<'s>,
}

// mig: struct IterationOptionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct IterationOptionNameI<'s> {
    pub range: RangeS<'s>,
}

// mig: struct MagicParamNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct MagicParamNameI<'s> {
    pub code_location_2: CodeLocationS<'s>,
}

// mig: struct CodeVarNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct CodeVarNameI<'s> {
    pub name: StrI<'s>,
}

// mig: struct AnonymousSubstructMemberNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructMemberNameI {
    pub index: i32,
}

// mig: struct PrimitiveNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct PrimitiveNameI<'s> {
    pub human_name: StrI<'s>,
}

// mig: struct PackageTopLevelNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct PackageTopLevelNameI;

// mig: struct ProjectNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ProjectNameI<'s> {
    pub name: StrI<'s>,
}

// mig: struct PackageNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct PackageNameI<'s> {
    pub name: StrI<'s>,
}

// mig: struct RuneNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct RuneNameI<'s> {
    pub rune: IRuneS<'s>,
}

// mig: struct BuildingFunctionNameWithClosuredsI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct BuildingFunctionNameWithClosuredsI<'s, 'i> {
    pub template_name: IFunctionTemplateNameI<'s, 'i>,
}


// mig: struct ExternFunctionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExternFunctionNameI<'s, 'i> {
    pub human_name: StrI<'s>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
    pub parameters: &'i[CoordI<'s, 'i>],
}

// mig: struct FunctionNameIX
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct FunctionNameIX<'s, 'i> {
    pub template: FunctionTemplateNameI<'s>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
    pub parameters: &'i[CoordI<'s, 'i>],
}


// mig: struct ForwarderFunctionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ForwarderFunctionNameI<'s, 'i> {
    pub template: ForwarderFunctionTemplateNameI<'s, 'i>,
    pub inner: IFunctionNameI<'s, 'i>,
}


// mig: struct FunctionBoundTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct FunctionBoundTemplateNameI<'s> {
    pub human_name: StrI<'s>,
}


// mig: struct FunctionBoundNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct FunctionBoundNameI<'s, 'i> {
    pub template: FunctionBoundTemplateNameI<'s>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
    pub parameters: &'i[CoordI<'s, 'i>],
}


// mig: struct ReachableFunctionTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ReachableFunctionTemplateNameI<'s> {
    pub human_name: StrI<'s>,
}


// mig: struct ReachableFunctionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ReachableFunctionNameI<'s, 'i> {
    pub template: ReachableFunctionTemplateNameI<'s>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
    pub parameters: &'i[CoordI<'s, 'i>],
}


// mig: struct FunctionTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct FunctionTemplateNameI<'s> {
    pub human_name: StrI<'s>,
    pub code_location: CodeLocationS<'s>,
}


// Per @LAGTNGZ, paramTypes stays baked in (specialization happened earlier).
// mig: struct LambdaCallFunctionTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct LambdaCallFunctionTemplateNameI<'s, 'i> {
    pub code_location: CodeLocationS<'s>,
    pub param_types: &'i[CoordI<'s, 'i>],
}


// mig: struct LambdaCallFunctionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct LambdaCallFunctionNameI<'s, 'i> {
    pub template: LambdaCallFunctionTemplateNameI<'s, 'i>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
    pub parameters: &'i[CoordI<'s, 'i>],
}


// mig: struct ForwarderFunctionTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ForwarderFunctionTemplateNameI<'s, 'i> {
    pub inner: IFunctionTemplateNameI<'s, 'i>,
    pub index: i32,
}


// mig: struct ConstructorTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ConstructorTemplateNameI<'s> {
    pub code_location: CodeLocationS<'s>,
}


// mig: struct SelfNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct SelfNameI;

// mig: struct ArbitraryNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ArbitraryNameI;

// mig: enum CitizenNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum CitizenNameI<'s, 'i> {
    _Phantom(PhantomData<(&'s (), &'i ())>),
}
// mig: impl CitizenNameI

// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<Wide> for Narrow` or inline match.)

// mig: struct StructNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct StructNameI<'s, 'i> {
    pub template: IStructTemplateNameI<'s, 'i>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
}


// mig: struct InterfaceNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct InterfaceNameI<'s, 'i> {
    pub template: IInterfaceTemplateNameI<'s, 'i>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
}


// Per @LAGTNGZ, closure struct isn't parameterized; one struct corresponds to many LambdaCallFunctionNameIs.
// mig: struct LambdaCitizenTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct LambdaCitizenTemplateNameI<'s> {
    pub code_location: CodeLocationS<'s>,
}


// mig: struct LambdaCitizenNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct LambdaCitizenNameI<'s> {
    pub template: LambdaCitizenTemplateNameI<'s>,
}


// mig: enum CitizenTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum CitizenTemplateNameI<'s, 'i> {
    _Phantom(PhantomData<(&'s (), &'i ())>),
}
// mig: impl CitizenTemplateNameI

// mig: struct StructTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct StructTemplateNameI<'s> {
    pub human_name: StrI<'s>,
}

// mig: struct InterfaceTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct InterfaceTemplateNameI<'s> {
    pub human_namee: StrI<'s>,
}

// mig: fn human_name
// (was cfg-gated)
impl<'s> InterfaceTemplateNameI<'s> {
    pub fn human_name(&self) -> StrI<'s> { panic!("Unimplemented: human_name"); }
}

// mig: struct AnonymousSubstructImplTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructImplTemplateNameI<'s, 'i> {
    pub interface: IInterfaceTemplateNameI<'s, 'i>,
}

// mig: struct AnonymousSubstructImplNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructImplNameI<'s, 'i> {
    pub template: AnonymousSubstructImplTemplateNameI<'s, 'i>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
    pub sub_citizen: ICitizenIT<'s, 'i>,
}


// mig: struct AnonymousSubstructTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructTemplateNameI<'s, 'i> {
    pub interface: IInterfaceTemplateNameI<'s, 'i>,
}

// mig: struct AnonymousSubstructConstructorTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructConstructorTemplateNameI<'s, 'i> {
    pub substruct: ICitizenTemplateNameI<'s, 'i>,
}


// mig: struct AnonymousSubstructConstructorNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructConstructorNameI<'s, 'i> {
    pub template: AnonymousSubstructConstructorTemplateNameI<'s, 'i>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
    pub parameters: &'i[CoordI<'s, 'i>],
}


// mig: struct AnonymousSubstructNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructNameI<'s, 'i> {
    pub template: AnonymousSubstructTemplateNameI<'s, 'i>,
    pub template_args: &'i[ITemplataI<'s, 'i>],
}


// mig: struct ResolvingEnvNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ResolvingEnvNameI;


// mig: struct CallEnvNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct CallEnvNameI;

