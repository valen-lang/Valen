/*
Guardian: disable: SPDMX
*/

use std::hash::{Hash, Hasher};
use crate::interner::StrI;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::postparsing::names::IRuneS;
use crate::typing::types::types::{CoordT, IRegionT, RegionT, ICitizenTT};
use crate::typing::templata::templata::{ITemplataT, expect_mutability, expect_variability, expect_integer, expect_coord_templata};
use crate::typing::ast::ast::LocationInFunctionEnvironmentT;
use crate::typing::typing_interner::{MustIntern, TypingInterner};
use crate::Keywords;
use INameValT::*;
use std::marker::PhantomData;
use std::ptr::eq;
use std::ptr::hash;

/*
package dev.vale.typing.names

import dev.vale.postparsing._
import dev.vale.typing.ast.LocationInFunctionEnvironmentT
import dev.vale.typing.expression.CallCompiler
import dev.vale._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.templata.ITemplataT._
import dev.vale.typing.types._

// Scout's/Astronomer's name parts correspond to where they are in the source code,
// but Compiler's correspond more to what packages and stamped functions / structs
// they're in. See TNAD.
*/

// Monomorphic per `docs/reasoning/idt-typed-view-alternatives.md`. Scala's
// `IdT[+T <: INameT]` phantom outer parameter is erased in Rust — callers
// pattern-match on `local_name` at the point they need narrowing.
/// Interned (see @TFITCX)
#[derive(Copy, Clone, Debug)]
pub struct IdT<'s, 't>
where 's: 't,
{
    pub package_coord: &'s PackageCoordinate<'s>,
    pub init_steps: &'t [INameT<'s, 't>],
    pub local_name: INameT<'s, 't>,
    pub _must_intern: MustIntern,
}
/*
case class IdT[+T <: INameT](
  packageCoord: PackageCoordinate,
  initSteps: Vector[INameT],
  localName: T
)  {
*/
impl<'s, 't> IdT<'s, 't> {
    pub fn new() -> Self {
        panic!("Unimplemented IdT new");
    }
    /*
      this match {
        case _ =>
      }

      // Placeholders should only be the last name, getPlaceholdersInKind assumes it
      initSteps.foreach({
        case KindPlaceholderNameT(_) => vfail()
        case KindPlaceholderTemplateNameT(_, _) => vfail()
        case _ =>
      })
      // Placeholders are under the template name.
      // There's really no other way; we make the placeholders before knowing the function's
      // instantated name.
      localName match {
        case KindPlaceholderNameT(_) => {
          initSteps.last match {
            case _ : ITemplateNameT =>
            case OverrideDispatcherNameT(_, _, _) => {
              initSteps.init.last match {
                case _ : ITemplateNameT =>
                case other => vfail(other)
              }
            }
            case other => vfail(other)
          }
        }
        case _ =>
      }

      // PackageTopLevelName2 is just here because names have to have a last step.
      vassert(initSteps.collectFirst({ case PackageTopLevelNameT() => }).isEmpty)

      vcurious(initSteps.distinct == initSteps)

    */
    /*
      override def equals(obj: Any): Boolean = {
        obj match {
          case IdT(thatPackageCoord, thatInitSteps, thatLast) => {
            packageCoord == thatPackageCoord && initSteps == thatInitSteps && localName == thatLast
          }
          case _ => false
        }
      }
    */
    fn package_id() {
        panic!("Unimplemented IdT package ID");
    }
    /*
      def packageId(interner: Interner): IdT[PackageTopLevelNameT] = {
        IdT(packageCoord, Vector(), interner.intern(PackageTopLevelNameT()))
      }
    */
    // Rust adaptation (SPDMX-B): interner threaded because Scala constructs IdT freely but Rust must intern.
    pub fn init_id(&self, interner: &TypingInterner<'s, 't>) -> IdT<'s, 't> {
        if self.init_steps.is_empty() {
            let top_level = interner.alloc(PackageTopLevelNameT { _phantom: PhantomData });
            *interner.intern_id(IdValT {
                package_coord: self.package_coord,
                init_steps: &[],
                local_name: INameT::PackageTopLevel(top_level),
            })
        } else {
            let last = *self.init_steps.last().unwrap();
            let prefix = &self.init_steps[..self.init_steps.len() - 1];
            *interner.intern_id(IdValT {
                package_coord: self.package_coord,
                init_steps: prefix,
                local_name: last,
            })
        }
    }
    /*
      def initId(interner: Interner): IdT[INameT] = {
        if (initSteps.isEmpty) {
          IdT(packageCoord, Vector(), interner.intern(PackageTopLevelNameT()))
        } else {
          IdT(packageCoord, initSteps.init, initSteps.last)
        }
      }
    */
    pub fn init_non_package_id(&self, interner: &TypingInterner<'s, 't>) -> Option<IdT<'s, 't>> {
        if self.init_steps.is_empty() {
            None
        } else {
            let (last, init) = self.init_steps.split_last().unwrap();
            Some(*interner.intern_id(IdValT {
                package_coord: self.package_coord,
                init_steps: init,
                local_name: *last,
            }))
        }
    }
    /*
      def initNonPackageId(): Option[IdT[INameT]] = {
        if (initSteps.isEmpty) {
          None
        } else {
          Some(IdT(packageCoord, initSteps.init, initSteps.last))
        }
      }
    */
    pub fn steps(&self) -> Vec<INameT<'s, 't>> {
        match self.local_name {
            INameT::PackageTopLevel(_) => self.init_steps.to_vec(),
            _ => {
                let mut v = self.init_steps.to_vec();
                v.push(self.local_name);
                v
            }
        }
    }
    /*
      def steps: Vector[INameT] = {
        localName match {
          case PackageTopLevelNameT() => initSteps
          case _ => initSteps :+ localName
        }
      }
    */
    pub fn add_step(&self, interner: &TypingInterner<'s, 't>, new_last: INameT<'s, 't>) -> &'t IdT<'s, 't> {
        let steps = self.steps();
        interner.intern_id(IdValT {
            package_coord: self.package_coord,
            init_steps: &steps,
            local_name: new_last,
        })
    }
    /*
      def addStep[Y <: INameT](newLast: Y): IdT[Y] = {
        IdT[Y](packageCoord, steps, newLast)
      }
    }
    */
}

// (no scala counterpart — custom Hash/PartialEq/Eq: pointer-eq on package_coord
// and init_steps slice (canonicalized by the typing interner per IDEPFL),
// structural compare on local_name (inline-owned INameT).)
impl<'s, 't> Hash for IdT<'s, 't>
where 's: 't,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash(self.package_coord, state);
        hash(self.init_steps.as_ptr(), state);
        self.init_steps.len().hash(state);
        self.local_name.hash(state);
    }
}
// Per @IEOIBZ, identity-equality on the canonical slice pointer. Soundness
// requires `init_steps` to come from the canonical arena allocation in
// `intern_id` — guaranteed by sealing per @SICZ.
impl<'s, 't> PartialEq for IdT<'s, 't>
where 's: 't,
{
    fn eq(&self, other: &Self) -> bool {
        eq(self.package_coord, other.package_coord)
            && eq(self.init_steps.as_ptr(), other.init_steps.as_ptr())
            && self.init_steps.len() == other.init_steps.len()
            && self.local_name == other.local_name
    }
}
impl<'s, 't> Eq for IdT<'s, 't> where 's: 't, {}
// Widen/narrow conversion methods removed with the move to monomorphic IdT.
// Callers that need a specific leaf-name pattern-match on `local_name` directly,
// like Scala does. See `docs/reasoning/idt-typed-view-alternatives.md`.

/// Polyvalue (see @TFITCX) — derive Eq/Hash; never hand-roll `ptr::eq` on the outer `&self` (see @PVECFPZ).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum INameT<'s, 't> {
    ExportTemplate(&'t ExportTemplateNameT<'s, 't>),
    Export(&'t ExportNameT<'s, 't>),
    ImplTemplate(&'t ImplTemplateNameT<'s, 't>),
    Impl(&'t ImplNameT<'s, 't>),
    ImplBoundTemplate(&'t ImplBoundTemplateNameT<'s, 't>),
    ImplBound(&'t ImplBoundNameT<'s, 't>),
    Let(&'t LetNameT<'s, 't>),
    ExportAs(&'t ExportAsNameT<'s, 't>),
    RawArray(&'t RawArrayNameT<'s, 't>),
    ReachablePrototype(&'t ReachablePrototypeNameT<'s, 't>),
    StaticSizedArrayTemplate(&'t StaticSizedArrayTemplateNameT<'s, 't>),
    StaticSizedArray(&'t StaticSizedArrayNameT<'s, 't>),
    RuntimeSizedArrayTemplate(&'t RuntimeSizedArrayTemplateNameT<'s, 't>),
    RuntimeSizedArray(&'t RuntimeSizedArrayNameT<'s, 't>),
    KindPlaceholderTemplate(&'t KindPlaceholderTemplateNameT<'s, 't>),
    KindPlaceholder(&'t KindPlaceholderNameT<'s, 't>),
    NonKindNonRegionPlaceholder(&'t NonKindNonRegionPlaceholderNameT<'s, 't>),
    OverrideDispatcherTemplate(&'t OverrideDispatcherTemplateNameT<'s, 't>),
    OverrideDispatcher(&'t OverrideDispatcherNameT<'s, 't>),
    OverrideDispatcherCase(&'t OverrideDispatcherCaseNameT<'s, 't>),
    TypingPassBlockResultVar(&'t TypingPassBlockResultVarNameT<'s, 't>),
    TypingPassFunctionResultVar(&'t TypingPassFunctionResultVarNameT<'s, 't>),
    TypingPassTemporaryVar(&'t TypingPassTemporaryVarNameT<'s, 't>),
    TypingPassPatternMember(&'t TypingPassPatternMemberNameT<'s, 't>),
    TypingIgnoredParam(&'t TypingIgnoredParamNameT<'s, 't>),
    TypingPassPatternDestructuree(&'t TypingPassPatternDestructureeNameT<'s, 't>),
    UnnamedLocal(&'t UnnamedLocalNameT<'s, 't>),
    ClosureParam(&'t ClosureParamNameT<'s, 't>),
    ConstructingMember(&'t ConstructingMemberNameT<'s, 't>),
    WhileCondResult(&'t WhileCondResultNameT<'s, 't>),
    Iterable(&'t IterableNameT<'s, 't>),
    Iterator(&'t IteratorNameT<'s, 't>),
    IterationOption(&'t IterationOptionNameT<'s, 't>),
    MagicParam(&'t MagicParamNameT<'s, 't>),
    CodeVar(&'t CodeVarNameT<'s, 't>),
    AnonymousSubstructMember(&'t AnonymousSubstructMemberNameT<'s, 't>),
    Primitive(&'t PrimitiveNameT<'s, 't>),
    PackageTopLevel(&'t PackageTopLevelNameT<'s, 't>),
    Project(&'t ProjectNameT<'s, 't>),
    Package(&'t PackageNameT<'s, 't>),
    Rune(&'t RuneNameT<'s, 't>),
    BuildingFunctionNameWithClosureds(&'t BuildingFunctionNameWithClosuredsT<'s, 't>),
    ExternTemplate(&'t ExternTemplateNameT<'s, 't>),
    Extern(&'t ExternNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),
    Function(&'t FunctionNameT<'s, 't>),
    ForwarderFunction(&'t ForwarderFunctionNameT<'s, 't>),
    FunctionBoundTemplate(&'t FunctionBoundTemplateNameT<'s, 't>),
    FunctionBound(&'t FunctionBoundNameT<'s, 't>),
    PredictedFunctionTemplate(&'t PredictedFunctionTemplateNameT<'s, 't>),
    PredictedFunction(&'t PredictedFunctionNameT<'s, 't>),
    FunctionTemplate(&'t FunctionTemplateNameT<'s, 't>),
    LambdaCallFunctionTemplate(&'t LambdaCallFunctionTemplateNameT<'s, 't>),
    LambdaCallFunction(&'t LambdaCallFunctionNameT<'s, 't>),
    ForwarderFunctionTemplate(&'t ForwarderFunctionTemplateNameT<'s, 't>),
    ConstructorTemplate(&'t ConstructorTemplateNameT<'s, 't>),
    Self_(&'t SelfNameT<'s, 't>),
    Arbitrary(&'t ArbitraryNameT<'s, 't>),
    Struct(&'t StructNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s, 't>),
    LambdaCitizen(&'t LambdaCitizenNameT<'s, 't>),
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
    AnonymousSubstructImplTemplate(&'t AnonymousSubstructImplTemplateNameT<'s, 't>),
    AnonymousSubstructImpl(&'t AnonymousSubstructImplNameT<'s, 't>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
    AnonymousSubstructConstructorTemplate(&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>),
    AnonymousSubstructConstructor(&'t AnonymousSubstructConstructorNameT<'s, 't>),
    AnonymousSubstruct(&'t AnonymousSubstructNameT<'s, 't>),
    ResolvingEnv(&'t ResolvingEnvNameT<'s, 't>),
    CallEnv(&'t CallEnvNameT<'s, 't>),
}
/*
sealed trait INameT extends IInterning
*/
// (Rust adaptation: Scala expression `idT.localName.parameters` works
// because Scala types `localName` as `IFunctionNameT` via `IdT[IFunctionNameT]`'s
// type parameter. Rust's `IdT.local_name: INameT` loses that narrowing, so we
// expose `parameters()` on the broad enum and panic for non-function variants.
// Same shape as `PrototypeT::param_types` at ast.rs:1020.)
impl<'s, 't> INameT<'s, 't> where 's: 't {
    pub fn parameters(&self) -> &'t [CoordT<'s, 't>] {
        match self {
            INameT::OverrideDispatcher(f) => f.parameters,
            INameT::ExternFunction(f) => f.parameters,
            INameT::Function(f) => f.parameters,
            INameT::ForwarderFunction(f) => f.inner.parameters(),
            INameT::FunctionBound(f) => f.parameters,
            INameT::PredictedFunction(f) => f.parameters,
            INameT::LambdaCallFunction(f) => f.parameters,
            INameT::AnonymousSubstructConstructor(f) => f.parameters,
            other => panic!("INameT::parameters called on non-function name: {:?}", other),
        }
    }
    /* */
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ITemplateNameT<'s, 't> {
    ExportTemplate(&'t ExportTemplateNameT<'s, 't>),
    ImplTemplate(&'t ImplTemplateNameT<'s, 't>),
    ImplBoundTemplate(&'t ImplBoundTemplateNameT<'s, 't>),
    StaticSizedArrayTemplate(&'t StaticSizedArrayTemplateNameT<'s, 't>),
    RuntimeSizedArrayTemplate(&'t RuntimeSizedArrayTemplateNameT<'s, 't>),
    KindPlaceholderTemplate(&'t KindPlaceholderTemplateNameT<'s, 't>),
    OverrideDispatcherTemplate(&'t OverrideDispatcherTemplateNameT<'s, 't>),
    OverrideDispatcherCase(&'t OverrideDispatcherCaseNameT<'s, 't>),
    ExternTemplate(&'t ExternTemplateNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),
    FunctionBoundTemplate(&'t FunctionBoundTemplateNameT<'s, 't>),
    PredictedFunctionTemplate(&'t PredictedFunctionTemplateNameT<'s, 't>),
    FunctionTemplate(&'t FunctionTemplateNameT<'s, 't>),
    LambdaCallFunctionTemplate(&'t LambdaCallFunctionTemplateNameT<'s, 't>),
    ForwarderFunctionTemplate(&'t ForwarderFunctionTemplateNameT<'s, 't>),
    ConstructorTemplate(&'t ConstructorTemplateNameT<'s, 't>),
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s, 't>),
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
    AnonymousSubstructImplTemplate(&'t AnonymousSubstructImplTemplateNameT<'s, 't>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
    AnonymousSubstructConstructorTemplate(&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>),
}
/*
sealed trait ITemplateNameT extends INameT
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IFunctionTemplateNameT<'s, 't> {
    OverrideDispatcherTemplate(&'t OverrideDispatcherTemplateNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),
    FunctionBoundTemplate(&'t FunctionBoundTemplateNameT<'s, 't>),
    PredictedFunctionTemplate(&'t PredictedFunctionTemplateNameT<'s, 't>),
    FunctionTemplate(&'t FunctionTemplateNameT<'s, 't>),
    LambdaCallFunctionTemplate(&'t LambdaCallFunctionTemplateNameT<'s, 't>),
    ForwarderFunctionTemplate(&'t ForwarderFunctionTemplateNameT<'s, 't>),
    ConstructorTemplate(&'t ConstructorTemplateNameT<'s, 't>),
    AnonymousSubstructConstructorTemplate(&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>),
}
/*
sealed trait IFunctionTemplateNameT extends ITemplateNameT {
  def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT
}
*/
// Scala trait method: def makeFunctionName(...): IFunctionNameT
// Each variant overrides — see names.scala lines 265, 345, 376, 400, 415, 424, 441, 487, 666
impl<'s, 't> IFunctionTemplateNameT<'s, 't> where 's: 't {
  pub fn make_function_name(
    &self,
    interner: &TypingInterner<'s, 't>,
    _keywords: &Keywords<'_>,
    template_args: &[ITemplataT<'s, 't>],
    params: &[CoordT<'s, 't>],
  ) -> INameT<'s, 't> {
    match self {
      IFunctionTemplateNameT::FunctionTemplate(tmpl) => {
        interner.intern_name(INameValT::Function(FunctionNameValT {
          template: tmpl,
          template_args,
          parameters: params,
        }))
      }
      IFunctionTemplateNameT::OverrideDispatcherTemplate(tmpl) => {
        interner.intern_name(INameValT::OverrideDispatcher(OverrideDispatcherNameValT {
          template: tmpl,
          template_args,
          parameters: params,
        }))
      }
      IFunctionTemplateNameT::ExternFunction(e) => {
        INameT::ExternFunction(e)
      }
      IFunctionTemplateNameT::FunctionBoundTemplate(tmpl) => {
        interner.intern_name(INameValT::FunctionBound(FunctionBoundNameValT {
          template: tmpl,
          template_args,
          parameters: params,
        }))
      }
      IFunctionTemplateNameT::PredictedFunctionTemplate(tmpl) => {
        interner.intern_name(INameValT::PredictedFunction(PredictedFunctionNameValT {
          template: tmpl,
          template_args,
          parameters: params,
        }))
      }
      IFunctionTemplateNameT::LambdaCallFunctionTemplate(tmpl) => {
        interner.intern_name(INameValT::LambdaCallFunction(LambdaCallFunctionNameValT {
          template: tmpl,
          template_args,
          parameters: params,
        }))
      }
      IFunctionTemplateNameT::ForwarderFunctionTemplate(tmpl) => {
        let inner_name = tmpl.inner.make_function_name(interner, _keywords, template_args, params);
        let inner_func_name: IFunctionNameT<'s, 't> = inner_name.try_into()
            .unwrap_or_else(|_| panic!("ForwarderFunctionTemplate inner should produce a function name"));
        interner.intern_name(INameValT::ForwarderFunction(ForwarderFunctionNameT {
          template: tmpl,
          inner: inner_func_name,
        }))
      }
      IFunctionTemplateNameT::ConstructorTemplate(_) => {
        panic!("Unimplemented: make_function_name for ConstructorTemplate")
      }
      IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(tmpl) => {
        interner.intern_name(INameValT::AnonymousSubstructConstructor(AnonymousSubstructConstructorNameValT {
          template: tmpl,
          template_args,
          parameters: params,
        }))
      }
    }
  }
  /* */
}
// Proactive dispatch method (TL.md "Proactively Add Inherited Dispatch
// Methods"): Scala accesses `template.humanName` on IFunctionTemplateNameT
// at InferCompiler.scala:314 and elsewhere. Most concrete subtypes carry a
// `humanName: StrI` field; the few that don't (OverrideDispatcher,
// LambdaCallFunction, Constructor, AnonymousSubstructConstructor) panic
// until a test path requires them.
impl<'s, 't> IFunctionTemplateNameT<'s, 't> where 's: 't {
  pub fn human_name(&self) -> StrI<'s> {
    match self {
      IFunctionTemplateNameT::FunctionTemplate(x) => x.human_name,
      IFunctionTemplateNameT::FunctionBoundTemplate(x) => x.human_name,
      IFunctionTemplateNameT::PredictedFunctionTemplate(x) => x.human_name,
      IFunctionTemplateNameT::ExternFunction(x) => x.human_name,
      IFunctionTemplateNameT::ForwarderFunctionTemplate(x) => x.inner.human_name(),
      IFunctionTemplateNameT::OverrideDispatcherTemplate(_) => panic!("Unimplemented: human_name on OverrideDispatcherTemplate (no humanName field in Scala)"),
      IFunctionTemplateNameT::LambdaCallFunctionTemplate(_) => panic!("Unimplemented: human_name on LambdaCallFunctionTemplate (no humanName field in Scala)"),
      IFunctionTemplateNameT::ConstructorTemplate(_) => panic!("Unimplemented: human_name on ConstructorTemplate (no humanName field in Scala)"),
      IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(_) => panic!("Unimplemented: human_name on AnonymousSubstructConstructor (no humanName field in Scala)"),
    }
  }
  /* */
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IInstantiationNameT<'s, 't> {
    Export(&'t ExportNameT<'s, 't>),
    Impl(&'t ImplNameT<'s, 't>),
    ImplBound(&'t ImplBoundNameT<'s, 't>),
    StaticSizedArray(&'t StaticSizedArrayNameT<'s, 't>),
    RuntimeSizedArray(&'t RuntimeSizedArrayNameT<'s, 't>),
    KindPlaceholder(&'t KindPlaceholderNameT<'s, 't>),
    OverrideDispatcher(&'t OverrideDispatcherNameT<'s, 't>),
    OverrideDispatcherCase(&'t OverrideDispatcherCaseNameT<'s, 't>),
    Extern(&'t ExternNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),
    Function(&'t FunctionNameT<'s, 't>),
    ForwarderFunction(&'t ForwarderFunctionNameT<'s, 't>),
    FunctionBound(&'t FunctionBoundNameT<'s, 't>),
    PredictedFunction(&'t PredictedFunctionNameT<'s, 't>),
    LambdaCallFunction(&'t LambdaCallFunctionNameT<'s, 't>),
    Struct(&'t StructNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
    LambdaCitizen(&'t LambdaCitizenNameT<'s, 't>),
    AnonymousSubstructImpl(&'t AnonymousSubstructImplNameT<'s, 't>),
    AnonymousSubstructConstructor(&'t AnonymousSubstructConstructorNameT<'s, 't>),
    AnonymousSubstruct(&'t AnonymousSubstructNameT<'s, 't>),
}
/*
sealed trait IInstantiationNameT extends INameT {
*/
impl<'s, 't> IInstantiationNameT<'s, 't> where 's: 't {
    pub fn template(&self) -> ITemplateNameT<'s, 't> {
        match self {
            IInstantiationNameT::Export(x) => ITemplateNameT::ExportTemplate(x.template),
            IInstantiationNameT::Impl(x) => ITemplateNameT::ImplTemplate(x.template),
            IInstantiationNameT::ImplBound(x) => ITemplateNameT::ImplBoundTemplate(x.template),
            IInstantiationNameT::StaticSizedArray(x) => ITemplateNameT::StaticSizedArrayTemplate(x.template),
            IInstantiationNameT::RuntimeSizedArray(x) => ITemplateNameT::RuntimeSizedArrayTemplate(x.template),
            IInstantiationNameT::KindPlaceholder(x) => ITemplateNameT::KindPlaceholderTemplate(x.template),
            IInstantiationNameT::OverrideDispatcher(x) => ITemplateNameT::OverrideDispatcherTemplate(x.template),
            // Scala: `override def template: ITemplateNameT = this`. The
            // OverrideDispatcherCaseNameT extends both IInstantiationNameT
            // and ITemplateNameT, so it is its own template — Rust's wide
            // ITemplateNameT enum carries an `OverrideDispatcherCase` variant
            // that wraps the same `&'t OverrideDispatcherCaseNameT` payload.
            IInstantiationNameT::OverrideDispatcherCase(x) => ITemplateNameT::OverrideDispatcherCase(x),
            IInstantiationNameT::Extern(x) => ITemplateNameT::ExternTemplate(x.template),
            // Scala: `override def template: IFunctionTemplateNameT = this`.
            // ExternFunctionNameT extends both IFunctionNameT and
            // IFunctionTemplateNameT, so it is its own template — Rust's wide
            // ITemplateNameT enum carries an `ExternFunction` variant.
            IInstantiationNameT::ExternFunction(x) => ITemplateNameT::ExternFunction(x),
            IInstantiationNameT::Function(x) => ITemplateNameT::FunctionTemplate(x.template),
            IInstantiationNameT::ForwarderFunction(x) => ITemplateNameT::ForwarderFunctionTemplate(x.template),
            IInstantiationNameT::FunctionBound(x) => ITemplateNameT::FunctionBoundTemplate(x.template),
            IInstantiationNameT::PredictedFunction(x) => ITemplateNameT::PredictedFunctionTemplate(x.template),
            IInstantiationNameT::LambdaCallFunction(x) => ITemplateNameT::LambdaCallFunctionTemplate(x.template),
            // Scala: `template: IStructTemplateNameT` (covariant override
            // narrowing the trait's `ITemplateNameT` return). Rust flattens
            // IStructTemplateNameT's three variants into ITemplateNameT.
            IInstantiationNameT::Struct(x) => match x.template {
                IStructTemplateNameT::StructTemplate(t) => ITemplateNameT::StructTemplate(t),
                IStructTemplateNameT::LambdaCitizenTemplate(t) => ITemplateNameT::LambdaCitizenTemplate(t),
                IStructTemplateNameT::AnonymousSubstructTemplate(t) => ITemplateNameT::AnonymousSubstructTemplate(t),
            },
            IInstantiationNameT::Interface(x) => ITemplateNameT::InterfaceTemplate(x.template),
            IInstantiationNameT::LambdaCitizen(x) => ITemplateNameT::LambdaCitizenTemplate(x.template),
            IInstantiationNameT::AnonymousSubstructImpl(x) => ITemplateNameT::AnonymousSubstructImplTemplate(x.template),
            IInstantiationNameT::AnonymousSubstructConstructor(x) => ITemplateNameT::AnonymousSubstructConstructorTemplate(x.template),
            IInstantiationNameT::AnonymousSubstruct(x) => ITemplateNameT::AnonymousSubstructTemplate(x.template),
        }
    }
    /*
  def template: ITemplateNameT
*/
    /* Guardian: disable-all */
}
impl<'s, 't> IInstantiationNameT<'s, 't> where 's: 't {
    pub fn template_args(&self) -> &'t [ITemplataT<'s, 't>] {
        match self {
            IInstantiationNameT::Export(_) => &[],
            IInstantiationNameT::Impl(x) => x.template_args,
            IInstantiationNameT::ImplBound(x) => x.template_args,
            IInstantiationNameT::StaticSizedArray(_) => panic!("Unimplemented: template_args on StaticSizedArrayNameT (computed: Vector(size, arr.mutability, variability, CoordTemplataT(arr.elementType)) — needs interner to allocate slice)"),
            IInstantiationNameT::RuntimeSizedArray(_) => panic!("Unimplemented: template_args on RuntimeSizedArrayNameT (computed: Vector(arr.mutability, CoordTemplataT(arr.elementType)) — needs interner to allocate slice)"),
            IInstantiationNameT::KindPlaceholder(_) => &[],
            IInstantiationNameT::OverrideDispatcher(x) => x.template_args,
            IInstantiationNameT::OverrideDispatcherCase(x) => x.independent_impl_template_args,
            IInstantiationNameT::Extern(_) => &[],
            IInstantiationNameT::ExternFunction(x) => x.template_args,
            IInstantiationNameT::Function(x) => x.template_args,
            IInstantiationNameT::ForwarderFunction(x) => x.inner.template_args(),
            IInstantiationNameT::FunctionBound(x) => x.template_args,
            IInstantiationNameT::PredictedFunction(x) => x.template_args,
            IInstantiationNameT::LambdaCallFunction(x) => x.template_args,
            IInstantiationNameT::Struct(x) => x.template_args,
            IInstantiationNameT::Interface(x) => x.template_args,
            IInstantiationNameT::LambdaCitizen(_) => &[],
            IInstantiationNameT::AnonymousSubstructImpl(x) => x.template_args,
            IInstantiationNameT::AnonymousSubstructConstructor(x) => x.template_args,
            IInstantiationNameT::AnonymousSubstruct(x) => x.template_args,
        }
    }
    /*
  def templateArgs: Vector[ITemplataT[ITemplataType]]
*/
}
/*
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IFunctionNameT<'s, 't> {
    OverrideDispatcher(&'t OverrideDispatcherNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),
    Function(&'t FunctionNameT<'s, 't>),
    ForwarderFunction(&'t ForwarderFunctionNameT<'s, 't>),
    FunctionBound(&'t FunctionBoundNameT<'s, 't>),
    PredictedFunction(&'t PredictedFunctionNameT<'s, 't>),
    LambdaCallFunction(&'t LambdaCallFunctionNameT<'s, 't>),
    AnonymousSubstructConstructor(&'t AnonymousSubstructConstructorNameT<'s, 't>),
}
/*
sealed trait IFunctionNameT extends IInstantiationNameT {
*/
impl<'s, 't> IFunctionNameT<'s, 't> where 's: 't {
    pub fn template(&self) -> IFunctionTemplateNameT<'s, 't> {
        match self {
            IFunctionNameT::OverrideDispatcher(x) => IFunctionTemplateNameT::OverrideDispatcherTemplate(x.template),
            IFunctionNameT::ExternFunction(_) => panic!("Unimplemented: template on ExternFunctionNameT (Scala: override def template = ExternFunctionTemplateNameT(humanName) — needs interner)"),
            IFunctionNameT::Function(x) => IFunctionTemplateNameT::FunctionTemplate(x.template),
            IFunctionNameT::ForwarderFunction(x) => IFunctionTemplateNameT::ForwarderFunctionTemplate(x.template),
            IFunctionNameT::FunctionBound(x) => IFunctionTemplateNameT::FunctionBoundTemplate(x.template),
            IFunctionNameT::PredictedFunction(x) => IFunctionTemplateNameT::PredictedFunctionTemplate(x.template),
            IFunctionNameT::LambdaCallFunction(x) => IFunctionTemplateNameT::LambdaCallFunctionTemplate(x.template),
            IFunctionNameT::AnonymousSubstructConstructor(x) => IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(x.template),
        }
    }
    /*
  def template: IFunctionTemplateNameT
*/
    pub fn template_args(&self) -> &'t [ITemplataT<'s, 't>] {
        match self {
            IFunctionNameT::OverrideDispatcher(x) => x.template_args,
            IFunctionNameT::ExternFunction(_) => &[],
            IFunctionNameT::Function(x) => x.template_args,
            IFunctionNameT::ForwarderFunction(f) => f.inner.template_args(),
            IFunctionNameT::FunctionBound(x) => x.template_args,
            IFunctionNameT::PredictedFunction(x) => x.template_args,
            IFunctionNameT::LambdaCallFunction(x) => x.template_args,
            IFunctionNameT::AnonymousSubstructConstructor(x) => x.template_args,
        }
    }
    /*
  def templateArgs: Vector[ITemplataT[ITemplataType]]
*/
    pub fn parameters(&self) -> &'t [CoordT<'s, 't>] {
        match self {
            IFunctionNameT::OverrideDispatcher(f) => f.parameters,
            IFunctionNameT::ExternFunction(f) => f.parameters,
            IFunctionNameT::Function(f) => f.parameters,
            IFunctionNameT::ForwarderFunction(f) => f.inner.parameters(),
            IFunctionNameT::FunctionBound(f) => f.parameters,
            IFunctionNameT::PredictedFunction(f) => f.parameters,
            IFunctionNameT::LambdaCallFunction(f) => f.parameters,
            IFunctionNameT::AnonymousSubstructConstructor(f) => f.parameters,
        }
    }
    /*
  def parameters: Vector[CoordT]
*/
}
/*
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISuperKindTemplateNameT<'s, 't> {
    KindPlaceholderTemplate(&'t KindPlaceholderTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
}
/*
sealed trait ISuperKindTemplateNameT extends ITemplateNameT
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISubKindTemplateNameT<'s, 't> {
    StaticSizedArrayTemplate(&'t StaticSizedArrayTemplateNameT<'s, 't>),
    RuntimeSizedArrayTemplate(&'t RuntimeSizedArrayTemplateNameT<'s, 't>),
    KindPlaceholderTemplate(&'t KindPlaceholderTemplateNameT<'s, 't>),
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s, 't>),
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
}
/*
sealed trait ISubKindTemplateNameT extends ITemplateNameT
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICitizenTemplateNameT<'s, 't> {
    StaticSizedArrayTemplate(&'t StaticSizedArrayTemplateNameT<'s, 't>),
    RuntimeSizedArrayTemplate(&'t RuntimeSizedArrayTemplateNameT<'s, 't>),
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s, 't>),
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
}
/*
sealed trait ICitizenTemplateNameT extends ISubKindTemplateNameT {
*/

impl<'s, 't> ICitizenTemplateNameT<'s, 't> {
    pub fn make_citizen_name(
        &self,
        interner: &TypingInterner<'s, 't>,
        template_args: &[ITemplataT<'s, 't>],
    ) -> INameT<'s, 't> {
        match self {
            ICitizenTemplateNameT::StaticSizedArrayTemplate(t) =>
                t.make_citizen_name(interner, template_args),
            ICitizenTemplateNameT::RuntimeSizedArrayTemplate(t) =>
                t.make_citizen_name(interner, template_args),
            ICitizenTemplateNameT::LambdaCitizenTemplate(tmpl) => {
                assert!(template_args.is_empty());
                interner.intern_name(INameValT::LambdaCitizen(LambdaCitizenNameT {
                  template: tmpl,
                }))
            }
            ICitizenTemplateNameT::StructTemplate(tmpl) => {
                interner.intern_name(INameValT::Struct(StructNameValT {
                  template: IStructTemplateNameT::StructTemplate(tmpl),
                  template_args,
                }))
            }
            ICitizenTemplateNameT::InterfaceTemplate(tmpl) => {
                interner.intern_name(INameValT::Interface(InterfaceNameValT {
                  template: tmpl,
                  template_args,
                }))
            }
            ICitizenTemplateNameT::AnonymousSubstructTemplate(tmpl) => {
                interner.intern_name(INameValT::AnonymousSubstruct(AnonymousSubstructNameValT {
                  template: tmpl,
                  template_args,
                }))
            }
        }
    }
/*
  def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): ICitizenNameT
*/
}
/*
}
*/

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IStructTemplateNameT<'s, 't> {
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s, 't>),
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
}
/*
sealed trait IStructTemplateNameT extends ICitizenTemplateNameT {
  def makeStructName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IStructNameT
  override def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]):
  ICitizenNameT = {
    makeStructName(interner, templateArgs)
  }
}
*/
// Scala trait method: def makeStructName(...): IStructNameT
// Overrides: LambdaCitizenTemplate (line 569), StructTemplate (line 618), AnonymousSubstructTemplate (line 659)
impl<'s, 't> IStructTemplateNameT<'s, 't> where 's: 't {
  pub fn make_struct_name(
    &self,
    interner: &TypingInterner<'s, 't>,
    template_args: &[ITemplataT<'s, 't>],
  ) -> INameT<'s, 't> {
    match self {
      IStructTemplateNameT::LambdaCitizenTemplate(tmpl) => {
        assert!(template_args.is_empty());
        interner.intern_name(INameValT::LambdaCitizen(LambdaCitizenNameT {
          template: tmpl,
        }))
      }
      IStructTemplateNameT::StructTemplate(tmpl) => {
        interner.intern_name(INameValT::Struct(StructNameValT {
          template: IStructTemplateNameT::StructTemplate(tmpl),
          template_args,
        }))
      }
      IStructTemplateNameT::AnonymousSubstructTemplate(tmpl) => {
        interner.intern_name(INameValT::AnonymousSubstruct(AnonymousSubstructNameValT {
          template: tmpl,
          template_args,
        }))
      }
    }
  }
  /* */
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IInterfaceTemplateNameT<'s, 't> {
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
}
/*
sealed trait IInterfaceTemplateNameT extends ICitizenTemplateNameT with ISuperKindTemplateNameT {
  def makeInterfaceName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IInterfaceNameT
}
*/
// Scala trait method: def makeInterfaceName(...): IInterfaceNameT
// Override: InterfaceTemplate (line 632)
impl<'s, 't> IInterfaceTemplateNameT<'s, 't> where 's: 't {
  pub fn make_interface_name(
    &self,
    interner: &TypingInterner<'s, 't>,
    template_args: &[ITemplataT<'s, 't>],
  ) -> INameT<'s, 't> {
    match self {
      IInterfaceTemplateNameT::InterfaceTemplate(tmpl) => {
        interner.intern_name(INameValT::Interface(InterfaceNameValT {
          template: tmpl,
          template_args,
        }))
      }
    }
  }
  /* */
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISuperKindNameT<'s, 't> {
    KindPlaceholder(&'t KindPlaceholderNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
}
/*
sealed trait ISuperKindNameT extends IInstantiationNameT {
*/
impl<'s, 't> ISuperKindNameT<'s, 't> where 's: 't {
    pub fn template(&self) -> ISuperKindTemplateNameT<'s, 't> {
        match self {
            ISuperKindNameT::KindPlaceholder(x) => ISuperKindTemplateNameT::KindPlaceholderTemplate(x.template),
            ISuperKindNameT::Interface(x) => ISuperKindTemplateNameT::InterfaceTemplate(x.template),
        }
    }
    /*
  def template: ISuperKindTemplateNameT
*/
    pub fn template_args(&self) -> &'t [ITemplataT<'s, 't>] {
        match self {
            ISuperKindNameT::KindPlaceholder(_) => &[],
            ISuperKindNameT::Interface(x) => x.template_args,
        }
    }
    /*
  def templateArgs: Vector[ITemplataT[ITemplataType]]
*/
}
/*
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISubKindNameT<'s, 't> {
    StaticSizedArray(&'t StaticSizedArrayNameT<'s, 't>),
    RuntimeSizedArray(&'t RuntimeSizedArrayNameT<'s, 't>),
    KindPlaceholder(&'t KindPlaceholderNameT<'s, 't>),
    Struct(&'t StructNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
    LambdaCitizen(&'t LambdaCitizenNameT<'s, 't>),
    AnonymousSubstruct(&'t AnonymousSubstructNameT<'s, 't>),
}
/*
sealed trait ISubKindNameT extends IInstantiationNameT {
*/
impl<'s, 't> ISubKindNameT<'s, 't> where 's: 't {
    pub fn template(&self) -> ISubKindTemplateNameT<'s, 't> {
        match self {
            ISubKindNameT::StaticSizedArray(x) => ISubKindTemplateNameT::StaticSizedArrayTemplate(x.template),
            ISubKindNameT::RuntimeSizedArray(x) => ISubKindTemplateNameT::RuntimeSizedArrayTemplate(x.template),
            ISubKindNameT::KindPlaceholder(x) => ISubKindTemplateNameT::KindPlaceholderTemplate(x.template),
            ISubKindNameT::Struct(x) => ISubKindTemplateNameT::from(ICitizenTemplateNameT::from(x.template)),
            ISubKindNameT::Interface(x) => ISubKindTemplateNameT::InterfaceTemplate(x.template),
            ISubKindNameT::LambdaCitizen(x) => ISubKindTemplateNameT::LambdaCitizenTemplate(x.template),
            ISubKindNameT::AnonymousSubstruct(x) => ISubKindTemplateNameT::AnonymousSubstructTemplate(x.template),
        }
    }
    /*
  def template: ISubKindTemplateNameT
*/
    pub fn template_args(&self) -> &'t [ITemplataT<'s, 't>] {
        match self {
            ISubKindNameT::StaticSizedArray(_) => panic!("Unimplemented: template_args on StaticSizedArrayNameT (computed: Vector(size, arr.mutability, variability, CoordTemplataT(arr.elementType)) — needs interner to allocate slice)"),
            ISubKindNameT::RuntimeSizedArray(_) => panic!("Unimplemented: template_args on RuntimeSizedArrayNameT (computed: Vector(arr.mutability, CoordTemplataT(arr.elementType)) — needs interner to allocate slice)"),
            ISubKindNameT::KindPlaceholder(_) => &[],
            ISubKindNameT::Struct(x) => x.template_args,
            ISubKindNameT::Interface(x) => x.template_args,
            ISubKindNameT::LambdaCitizen(_) => &[],
            ISubKindNameT::AnonymousSubstruct(x) => x.template_args,
        }
    }
    /*
  def templateArgs: Vector[ITemplataT[ITemplataType]]
*/
}
/*
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICitizenNameT<'s, 't> {
    StaticSizedArray(&'t StaticSizedArrayNameT<'s, 't>),
    RuntimeSizedArray(&'t RuntimeSizedArrayNameT<'s, 't>),
    Struct(&'t StructNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
    LambdaCitizen(&'t LambdaCitizenNameT<'s, 't>),
    AnonymousSubstruct(&'t AnonymousSubstructNameT<'s, 't>),
}
/*
sealed trait ICitizenNameT extends ISubKindNameT {
*/
impl<'s, 't> ICitizenNameT<'s, 't> where 's: 't {
    pub fn template(&self) -> ICitizenTemplateNameT<'s, 't> {
        match self {
            ICitizenNameT::StaticSizedArray(x) => ICitizenTemplateNameT::StaticSizedArrayTemplate(x.template),
            ICitizenNameT::RuntimeSizedArray(x) => ICitizenTemplateNameT::RuntimeSizedArrayTemplate(x.template),
            ICitizenNameT::Struct(x) => ICitizenTemplateNameT::from(x.template),
            ICitizenNameT::Interface(x) => ICitizenTemplateNameT::InterfaceTemplate(x.template),
            ICitizenNameT::LambdaCitizen(x) => ICitizenTemplateNameT::LambdaCitizenTemplate(x.template),
            ICitizenNameT::AnonymousSubstruct(x) => ICitizenTemplateNameT::AnonymousSubstructTemplate(x.template),
        }
    }
    /*
  def template: ICitizenTemplateNameT
*/
    pub fn template_args(&self) -> &'t [ITemplataT<'s, 't>] {
        match self {
            ICitizenNameT::StaticSizedArray(_) => panic!("Unimplemented: template_args on StaticSizedArrayNameT (computed: Vector(size, arr.mutability, variability, CoordTemplataT(arr.elementType)) — needs interner to allocate slice)"),
            ICitizenNameT::RuntimeSizedArray(_) => panic!("Unimplemented: template_args on RuntimeSizedArrayNameT (computed: Vector(arr.mutability, CoordTemplataT(arr.elementType)) — needs interner to allocate slice)"),
            ICitizenNameT::Struct(x) => x.template_args,
            ICitizenNameT::Interface(x) => x.template_args,
            ICitizenNameT::LambdaCitizen(_) => &[],
            ICitizenNameT::AnonymousSubstruct(x) => x.template_args,
        }
    }
    /*
  def templateArgs: Vector[ITemplataT[ITemplataType]]
*/
}
/*
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IStructNameT<'s, 't> {
    Struct(&'t StructNameT<'s, 't>),
    LambdaCitizen(&'t LambdaCitizenNameT<'s, 't>),
    AnonymousSubstruct(&'t AnonymousSubstructNameT<'s, 't>),
}
/*
sealed trait IStructNameT extends ICitizenNameT with ISubKindNameT {
*/
impl<'s, 't> IStructNameT<'s, 't> where 's: 't {
    pub fn template(&self) -> IStructTemplateNameT<'s, 't> {
        match self {
            IStructNameT::Struct(x) => x.template,
            IStructNameT::LambdaCitizen(x) => IStructTemplateNameT::LambdaCitizenTemplate(x.template),
            IStructNameT::AnonymousSubstruct(x) => IStructTemplateNameT::AnonymousSubstructTemplate(x.template),
        }
    }
    /*
  override def template: IStructTemplateNameT
*/
    pub fn template_args(&self) -> &'t [ITemplataT<'s, 't>] {
        match self {
            IStructNameT::Struct(x) => x.template_args,
            IStructNameT::LambdaCitizen(_) => &[],
            IStructNameT::AnonymousSubstruct(x) => x.template_args,
        }
    }
    /*
  override def templateArgs: Vector[ITemplataT[ITemplataType]]
*/
}
/*
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IInterfaceNameT<'s, 't> {
    Interface(&'t InterfaceNameT<'s, 't>),
}
/*
sealed trait IInterfaceNameT extends ICitizenNameT with ISubKindNameT with ISuperKindNameT {
*/
impl<'s, 't> IInterfaceNameT<'s, 't> where 's: 't {
    pub fn template(&self) -> &'t InterfaceTemplateNameT<'s, 't> {
        match self {
            IInterfaceNameT::Interface(x) => x.template,
        }
    }
    /*
  override def template: InterfaceTemplateNameT
*/
    pub fn template_args(&self) -> &'t [ITemplataT<'s, 't>] {
        match self {
            IInterfaceNameT::Interface(x) => x.template_args,
        }
    }
    /*
  override def templateArgs: Vector[ITemplataT[ITemplataType]]
*/
}
/*
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IImplTemplateNameT<'s, 't> {
    ImplTemplate(&'t ImplTemplateNameT<'s, 't>),
    ImplBoundTemplate(&'t ImplBoundTemplateNameT<'s, 't>),
    AnonymousSubstructImplTemplate(&'t AnonymousSubstructImplTemplateNameT<'s, 't>),
}
/*
sealed trait IImplTemplateNameT extends ITemplateNameT {
  def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): IImplNameT
}
*/
// Scala trait method: def makeImplName(...): IImplNameT
// Overrides: ImplTemplate (line 160), ImplBoundTemplate (line 175), AnonymousSubstructImplTemplate (line 643)
impl<'s, 't> IImplTemplateNameT<'s, 't> where 's: 't {
  pub fn make_impl_name(
    &self,
    interner: &TypingInterner<'s, 't>,
    template_args: &[ITemplataT<'s, 't>],
    sub_citizen: ICitizenTT<'s, 't>,
  ) -> INameT<'s, 't> {
    match self {
      IImplTemplateNameT::ImplTemplate(tmpl) => {
        interner.intern_name(INameValT::Impl(ImplNameValT {
          template: tmpl,
          template_args,
          sub_citizen,
        }))
      }
      IImplTemplateNameT::ImplBoundTemplate(tmpl) => {
        interner.intern_name(INameValT::ImplBound(ImplBoundNameValT {
          template: tmpl,
          template_args,
        }))
      }
      IImplTemplateNameT::AnonymousSubstructImplTemplate(tmpl) => {
        interner.intern_name(INameValT::AnonymousSubstructImpl(AnonymousSubstructImplNameValT {
          template: tmpl,
          template_args,
          sub_citizen,
        }))
      }
    }
  }
  /* */
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IImplNameT<'s, 't> {
    Impl(&'t ImplNameT<'s, 't>),
    ImplBound(&'t ImplBoundNameT<'s, 't>),
    AnonymousSubstructImpl(&'t AnonymousSubstructImplNameT<'s, 't>),
}
/*
sealed trait IImplNameT extends IInstantiationNameT {
*/
impl<'s, 't> IImplNameT<'s, 't> where 's: 't {
    pub fn template(&self) -> IImplTemplateNameT<'s, 't> {
        match self {
            IImplNameT::Impl(x) => IImplTemplateNameT::ImplTemplate(x.template),
            IImplNameT::ImplBound(x) => IImplTemplateNameT::ImplBoundTemplate(x.template),
            IImplNameT::AnonymousSubstructImpl(x) => IImplTemplateNameT::AnonymousSubstructImplTemplate(x.template),
        }
    }
    /*
  def template: IImplTemplateNameT
*/
}
impl<'s, 't> IImplNameT<'s, 't> where 's: 't {
    pub fn template_args(&self) -> &'t [ITemplataT<'s, 't>] {
        match self {
            IImplNameT::Impl(x) => x.template_args,
            IImplNameT::ImplBound(x) => x.template_args,
            IImplNameT::AnonymousSubstructImpl(x) => x.template_args,
        }
    }
    /* Guardian: disable-all */
}
/*
}

*/
// TODO: placeholder PhantomData — replace with real fields during body migration
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IRegionNameT<'s, 't> { _Phantom(PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IRegionNameT extends INameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExportTemplateNameT<'s, 't> {
    pub code_loc: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class ExportTemplateNameT(codeLoc: CodeLocationS) extends ITemplateNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExportNameT<'s, 't> {
    pub template: &'t ExportTemplateNameT<'s, 't>,
    pub region: RegionT,
}
/*
case class ExportNameT(template: ExportTemplateNameT, region: RegionT) extends IInstantiationNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector()
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImplTemplateNameT<'s, 't> {
    pub code_location_s: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class ImplTemplateNameT(codeLocationS: CodeLocationS) extends IImplTemplateNameT {
  vpass()
  override def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): ImplNameT = {
    interner.intern(ImplNameT(this, templateArgs, subCitizen))
  }
}
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImplNameT<'s, 't> {
    pub template: &'t ImplTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub sub_citizen: ICitizenTT<'s, 't>,
    pub _must_intern: MustIntern,
}
/*
case class ImplNameT(
  template: ImplTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  // The instantiator wants this so it can know the struct type up-front before monomorphizing the
  // whole impl, so it can hoist some bounds out of the struct, like NBIFP.
  subCitizen: ICitizenTT
) extends IImplNameT {
  vpass()
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImplBoundTemplateNameT<'s, 't> {
    pub code_location_s: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class ImplBoundTemplateNameT(codeLocationS: CodeLocationS) extends IImplTemplateNameT {
  override def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): ImplBoundNameT = {
    interner.intern(ImplBoundNameT(this, templateArgs))
  }
}
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImplBoundNameT<'s, 't> {
    pub template: &'t ImplBoundTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
case class ImplBoundNameT(
  template: ImplBoundTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IImplNameT {

}

*/
/*
//// The name of an impl that is subclassing some interface. To find all impls subclassing an interface,
//// look for this name.
//case class ImplImplementingSuperInterfaceNameT(superInterface: FullNameT[IInterfaceTemplateNameT]) extends IImplTemplateNameT
*/
/*
//// The name of an impl that is augmenting some sub citizen. To find all impls subclassing an interface,
//// look for this name.
//case class ImplAugmentingSubCitizenNameT(subCitizen: FullNameT[ICitizenTemplateNameT]) extends IImplTemplateNameT

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LetNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class LetNameT(codeLocation: CodeLocationS) extends INameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExportAsNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class ExportAsNameT(codeLocation: CodeLocationS) extends INameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RawArrayNameT<'s, 't> {
    pub mutability: ITemplataT<'s, 't>,
    pub element_type: CoordT<'s, 't>,
    pub self_region: RegionT,
}
/*
case class RawArrayNameT(
  mutability: ITemplataT[MutabilityTemplataType],
  elementType: CoordT,
  selfRegion: RegionT
) extends INameT

// This num is really just here to disambiguate it from other reachable prototypes in the environment
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReachablePrototypeNameT<'s, 't> {
    pub num: i32,
    pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class ReachablePrototypeNameT(num: Int) extends INameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayTemplateNameT<'s, 't> {
    pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class StaticSizedArrayTemplateNameT() extends ICitizenTemplateNameT {
*/
impl<'s, 't> StaticSizedArrayTemplateNameT<'s, 't> {
    pub fn make_citizen_name(
        &self,
        interner: &TypingInterner<'s, 't>,
        template_args: &[ITemplataT<'s, 't>],
    ) -> INameT<'s, 't> {
        // vassert(templateArgs.size == 4)
        assert!(template_args.len() == 4);
        // val size = expectInteger(templateArgs(0))
        let size = expect_integer(template_args[0]);
        // val mutability = expectMutability(templateArgs(1))
        let mutability = expect_mutability(template_args[1]);
        // val variability = expectVariability(templateArgs(2))
        let variability = expect_variability(template_args[2]);
        // val elementType = expectCoordTemplata(templateArgs(3)).coord
        let element_type = expect_coord_templata(template_args[3]).coord;
        // val selfRegion = vregionmut(RegionT(DefaultRegionT))
        let self_region = RegionT { region: IRegionT::Default };
        // interner.intern(StaticSizedArrayNameT(this, size, variability, interner.intern(RawArrayNameT(mutability, elementType, selfRegion))))
        let raw_array_name = interner.intern_raw_array_name(RawArrayNameT {
            mutability,
            element_type,
            self_region,
        });
        let ssa_name = interner.intern_static_sized_array_name(StaticSizedArrayNameT {
            template: interner.alloc(*self),
            size,
            variability,
            arr: raw_array_name,
        });
        INameT::StaticSizedArray(ssa_name)
    }
/*
  override def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): ICitizenNameT = {
    vassert(templateArgs.size == 4)
    val size = expectInteger(templateArgs(0))
    val mutability = expectMutability(templateArgs(1))
    val variability = expectVariability(templateArgs(2))
    val elementType = expectCoordTemplata(templateArgs(3)).coord
    val selfRegion = vregionmut(RegionT(DefaultRegionT))
    interner.intern(StaticSizedArrayNameT(this, size, variability, interner.intern(RawArrayNameT(mutability, elementType, selfRegion))))
  }
*/
}
/*
    }
    */

/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayNameT<'s, 't> {
    pub template: &'t StaticSizedArrayTemplateNameT<'s, 't>,
    pub size: ITemplataT<'s, 't>,
    pub variability: ITemplataT<'s, 't>,
    pub arr: &'t RawArrayNameT<'s, 't>,
}
/*
case class StaticSizedArrayNameT(
  template: StaticSizedArrayTemplateNameT,
  size: ITemplataT[IntegerTemplataType],
  variability: ITemplataT[VariabilityTemplataType],
  arr: RawArrayNameT) extends ICitizenNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = {
    Vector(size, arr.mutability, variability, CoordTemplataT(arr.elementType))
  }
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayTemplateNameT<'s, 't> {
    pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class RuntimeSizedArrayTemplateNameT() extends ICitizenTemplateNameT {
*/
impl<'s, 't> RuntimeSizedArrayTemplateNameT<'s, 't> {
    pub fn make_citizen_name(
        &self,
        interner: &TypingInterner<'s, 't>,
        template_args: &[ITemplataT<'s, 't>],
    ) -> INameT<'s, 't> {
        // vassert(templateArgs.size == 2)
        assert!(template_args.len() == 2);
        // val mutability = expectMutability(templateArgs(0))
        let mutability = expect_mutability(template_args[0]);
        // val elementType = expectCoordTemplata(templateArgs(1)).coord
        let element_type = expect_coord_templata(template_args[1]).coord;
        // val region = vregionmut(RegionT(DefaultRegionT))
        let region = RegionT { region: IRegionT::Default };
        // interner.intern(RuntimeSizedArrayNameT(this, interner.intern(RawArrayNameT(mutability, elementType, region))))
        let raw_array_name = interner.intern_raw_array_name(RawArrayNameT {
            mutability,
            element_type: element_type,
            self_region: region,
        });
        let rsa_name = interner.intern_runtime_sized_array_name(RuntimeSizedArrayNameT {
            template: interner.alloc(*self),
            arr: raw_array_name,
        });
        INameT::RuntimeSizedArray(rsa_name)
    }
/*
  override def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): ICitizenNameT = {
    vassert(templateArgs.size == 2)
    val mutability = expectMutability(templateArgs(0))
    val elementType = expectCoordTemplata(templateArgs(1)).coord
    val region = vregionmut(RegionT(DefaultRegionT))
    interner.intern(RuntimeSizedArrayNameT(this, interner.intern(RawArrayNameT(mutability, elementType, region))))
  }
*/
}
/*
}
*/

/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayNameT<'s, 't> {
    pub template: &'t RuntimeSizedArrayTemplateNameT<'s, 't>,
    pub arr: &'t RawArrayNameT<'s, 't>,
}
/*
case class RuntimeSizedArrayNameT(template: RuntimeSizedArrayTemplateNameT, arr: RawArrayNameT) extends ICitizenNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = {
    Vector(arr.mutability, CoordTemplataT(arr.elementType))
  }
}

*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IPlaceholderNameT<'s, 't> {
    KindPlaceholder(&'t KindPlaceholderNameT<'s, 't>),
    NonKindNonRegionPlaceholder(&'t NonKindNonRegionPlaceholderNameT<'s, 't>),
}
/*
sealed trait IPlaceholderNameT extends INameT {
*/
impl<'s, 't> IPlaceholderNameT<'s, 't> {
    pub fn index(&self) -> i32 {
        match self {
            IPlaceholderNameT::KindPlaceholder(x) => x.template.index,
            IPlaceholderNameT::NonKindNonRegionPlaceholder(x) => x.index,
        }
    }
    /*
  def index: Int
  */
    /* Guardian: disable-all */
}
impl<'s, 't> IPlaceholderNameT<'s, 't> {
    pub fn rune(&self) -> IRuneS<'s> {
        match self {
            IPlaceholderNameT::KindPlaceholder(x) => x.template.rune,
            IPlaceholderNameT::NonKindNonRegionPlaceholder(x) => x.rune,
        }
    }
    /*
  def rune: IRuneS
  */
    /* Guardian: disable-all */
}
/*
}

// This exists because PlaceholderT is a kind, and all kinds need environments to assist
// in call/overload resolution. Environments are associated with templates, so it makes
// some sense to have a "placeholder template" notion.
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct KindPlaceholderTemplateNameT<'s, 't> {
    pub index: i32,
    pub rune: IRuneS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class KindPlaceholderTemplateNameT(index: Int, rune: IRuneS) extends ISubKindTemplateNameT with ISuperKindTemplateNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct KindPlaceholderNameT<'s, 't> {
    pub template: &'t KindPlaceholderTemplateNameT<'s, 't>,
}
/*
case class KindPlaceholderNameT(template: KindPlaceholderTemplateNameT) extends IPlaceholderNameT with ISubKindNameT with ISuperKindNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector()
  override def rune: IRuneS = template.rune
  override def index: Int = template.index
}

// This exists because we need a different way to refer to a coord generic param's other components,
// see MNRFGC.
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NonKindNonRegionPlaceholderNameT<'s, 't> {
    pub index: i32,
    pub rune: IRuneS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class NonKindNonRegionPlaceholderNameT(index: Int, rune: IRuneS) extends IPlaceholderNameT

// See NNSPAFOC.
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OverrideDispatcherTemplateNameT<'s, 't> {
    pub impl_id: IdT<'s, 't>,
}
/*
case class OverrideDispatcherTemplateNameT(
  implId: IdT[IImplTemplateNameT]
) extends IFunctionTemplateNameT {
  override def makeFunctionName(
    interner: Interner,
    keywords: Keywords,
    templateArgs: Vector[ITemplataT[ITemplataType]],
    params: Vector[CoordT]):
  OverrideDispatcherNameT = {
    interner.intern(OverrideDispatcherNameT(this, templateArgs, params))
  }
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OverrideDispatcherNameT<'s, 't> {
    pub template: &'t OverrideDispatcherTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
case class OverrideDispatcherNameT(
  template: OverrideDispatcherTemplateNameT,
  // This will have placeholders in it after the typing pass.
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT {
  vpass()
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OverrideDispatcherCaseNameT<'s, 't> {
    pub independent_impl_template_args: &'t [ITemplataT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
case class OverrideDispatcherCaseNameT(
  // These are the templatas for the independent runes from the impl, like the <ZZ> for Milano, see
  // OMCNAGP.
  independentImplTemplateArgs: Vector[ITemplataT[ITemplataType]]
) extends ITemplateNameT with IInstantiationNameT {
  override def template: ITemplateNameT = this
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = independentImplTemplateArgs
}

*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IVarNameT<'s, 't> {
    TypingPassBlockResultVar(&'t TypingPassBlockResultVarNameT<'s, 't>),
    TypingPassFunctionResultVar(&'t TypingPassFunctionResultVarNameT<'s, 't>),
    TypingPassTemporaryVar(&'t TypingPassTemporaryVarNameT<'s, 't>),
    TypingPassPatternMember(&'t TypingPassPatternMemberNameT<'s, 't>),
    TypingIgnoredParam(&'t TypingIgnoredParamNameT<'s, 't>),
    TypingPassPatternDestructuree(&'t TypingPassPatternDestructureeNameT<'s, 't>),
    UnnamedLocal(&'t UnnamedLocalNameT<'s, 't>),
    ClosureParam(&'t ClosureParamNameT<'s, 't>),
    ConstructingMember(&'t ConstructingMemberNameT<'s, 't>),
    WhileCondResult(&'t WhileCondResultNameT<'s, 't>),
    Iterable(&'t IterableNameT<'s, 't>),
    Iterator(&'t IteratorNameT<'s, 't>),
    IterationOption(&'t IterationOptionNameT<'s, 't>),
    MagicParam(&'t MagicParamNameT<'s, 't>),
    CodeVar(&'t CodeVarNameT<'s, 't>),
    AnonymousSubstructMember(&'t AnonymousSubstructMemberNameT<'s, 't>),
    Self_(&'t SelfNameT<'s, 't>),
}
/*
sealed trait IVarNameT extends INameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingPassBlockResultVarNameT<'s, 't> {
    pub life: LocationInFunctionEnvironmentT<'s, 't>,
}
/*
case class TypingPassBlockResultVarNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingPassFunctionResultVarNameT<'s, 't> {
    pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class TypingPassFunctionResultVarNameT() extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingPassTemporaryVarNameT<'s, 't> {
    pub life: LocationInFunctionEnvironmentT<'s, 't>,
}
/*
case class TypingPassTemporaryVarNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingPassPatternMemberNameT<'s, 't> {
    pub life: LocationInFunctionEnvironmentT<'s, 't>,
}
/*
case class TypingPassPatternMemberNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingIgnoredParamNameT<'s, 't> {
    pub num: i32,
    pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class TypingIgnoredParamNameT(num: Int) extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingPassPatternDestructureeNameT<'s, 't> {
    pub life: LocationInFunctionEnvironmentT<'s, 't>,
}
/*
case class TypingPassPatternDestructureeNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct UnnamedLocalNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class UnnamedLocalNameT(codeLocation: CodeLocationS) extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ClosureParamNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class ClosureParamNameT(codeLocation: CodeLocationS) extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ConstructingMemberNameT<'s, 't> {
    pub name: StrI<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class ConstructingMemberNameT(name: StrI) extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct WhileCondResultNameT<'s, 't> {
    pub range: RangeS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class WhileCondResultNameT(range: RangeS) extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IterableNameT<'s, 't> {
    pub range: RangeS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class IterableNameT(range: RangeS) extends IVarNameT {  }
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IteratorNameT<'s, 't> {
    pub range: RangeS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class IteratorNameT(range: RangeS) extends IVarNameT {  }
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IterationOptionNameT<'s, 't> {
    pub range: RangeS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class IterationOptionNameT(range: RangeS) extends IVarNameT {  }
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MagicParamNameT<'s, 't> {
    pub code_location2: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class MagicParamNameT(codeLocation2: CodeLocationS) extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CodeVarNameT<'s, 't> {
    pub name: StrI<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class CodeVarNameT(name: StrI) extends IVarNameT
// We dont use CodeVarName2(0), CodeVarName2(1) etc because we dont want the user to address these members directly.
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructMemberNameT<'s, 't> {
    pub index: i32,
    pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class AnonymousSubstructMemberNameT(index: Int) extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrimitiveNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class PrimitiveNameT(humanName: StrI) extends INameT
// Only made in typingpass
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PackageTopLevelNameT<'s, 't> {
    pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class PackageTopLevelNameT() extends INameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ProjectNameT<'s, 't> {
    pub name: StrI<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class ProjectNameT(name: StrI) extends INameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PackageNameT<'s, 't> {
    pub name: StrI<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class PackageNameT(name: StrI) extends INameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuneNameT<'s, 't> {
    pub rune: IRuneS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class RuneNameT(rune: IRuneS) extends INameT

// This is the name of a function that we're still figuring out in the function typingpass.
// We have its closured variables, but are still figuring out its template args and params.
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BuildingFunctionNameWithClosuredsT<'s, 't> {
    pub template_name: IFunctionTemplateNameT<'s, 't>,
}
/*
case class BuildingFunctionNameWithClosuredsT(
  templateName: IFunctionTemplateNameT,
) extends INameT {



}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternTemplateNameT<'s, 't> {
    pub code_loc: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class ExternTemplateNameT(
  codeLoc: CodeLocationS,
) extends ITemplateNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternNameT<'s, 't> {
    pub template: &'t ExternTemplateNameT<'s, 't>,
    pub template_arg: RegionT,
}
/*
case class ExternNameT(
  template: ExternTemplateNameT,
  templateArg: RegionT
) extends IInstantiationNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector()
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternFunctionNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
case class ExternFunctionNameT(
  humanName: StrI,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT with IFunctionTemplateNameT {
  override def template: IFunctionTemplateNameT = this

  override def makeFunctionName(
    interner: Interner,
    keywords: Keywords,
    templateArgs: Vector[ITemplataT[ITemplataType]],
    params: Vector[CoordT]):
  IFunctionNameT = this
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionNameT<'s, 't> {
    pub template: &'t FunctionTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
case class FunctionNameT(
  template: FunctionTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ForwarderFunctionNameT<'s, 't> {
    pub template: &'t ForwarderFunctionTemplateNameT<'s, 't>,
    pub inner: IFunctionNameT<'s, 't>,
}
/*
case class ForwarderFunctionNameT(
  template: ForwarderFunctionTemplateNameT,
  inner: IFunctionNameT
) extends IFunctionNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = inner.templateArgs
  override def parameters: Vector[CoordT] = inner.parameters
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionBoundTemplateNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class FunctionBoundTemplateNameT(
  humanName: StrI,
  // Removed this because we want various function bounds from various places to merge
  // together, see MFBFDP.
  // codeLocation: CodeLocationS
) extends INameT with IFunctionTemplateNameT {
  vpass()
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): FunctionBoundNameT = {
    interner.intern(FunctionBoundNameT(this, templateArgs, params))
  }
}

// We tried splitting this out into a ReachableFunctionNameT, so each function could
// keep separate its direct instantiation bound params (e.g. where func drop(T)void on
// the function itself) as opposed to its indirect instantiation bound params (ones
// declared on the params' kind struct/interfaces' definitions).
// See RFNTIOB for why we reverted that.
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionBoundNameT<'s, 't> {
    pub template: &'t FunctionBoundTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
case class FunctionBoundNameT(
  template: FunctionBoundTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

// PredictedFunctionNameT and PredictedFunctionTemplateNameT are special names similar to
// FunctionBoundNameT, they're temporary ones created during solving, to put into the result
// runes. At the end of solving, just afterward, they're turned into actual FunctionBoundNameT
// or resolved from the calling environment.
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PredictedFunctionTemplateNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class PredictedFunctionTemplateNameT(
    humanName: StrI
) extends INameT with IFunctionTemplateNameT {
  vpass()
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): PredictedFunctionNameT = {
    interner.intern(PredictedFunctionNameT(this, templateArgs, params))
  }
}
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PredictedFunctionNameT<'s, 't> {
    pub template: &'t PredictedFunctionTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
case class PredictedFunctionNameT(
    template: PredictedFunctionTemplateNameT,
    templateArgs: Vector[ITemplataT[ITemplataType]],
    parameters: Vector[CoordT]
) extends IFunctionNameT

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionTemplateNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub code_location: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class FunctionTemplateNameT(
    humanName: StrI,
    codeLocation: CodeLocationS
) extends INameT with IFunctionTemplateNameT {
  vpass()
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
    interner.intern(FunctionNameT(this, templateArgs, params))
  }
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LambdaCallFunctionTemplateNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub param_types: &'t [CoordT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
// Per @LAGTNGZ, paramTypes is part of the template name so different arg tuples produce distinct names.
case class LambdaCallFunctionTemplateNameT(
  codeLocation: CodeLocationS,
  paramTypes: Vector[CoordT]
) extends INameT with IFunctionTemplateNameT {
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
    // Post instantiator, the params will be real, but our template paramTypes will still be placeholders
    // vassert(params == paramTypes)
    interner.intern(LambdaCallFunctionNameT(this, templateArgs, params))
  }
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LambdaCallFunctionNameT<'s, 't> {
    pub template: &'t LambdaCallFunctionTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
// Per @LAGTNGZ, one closure struct can correspond to many of these — one per distinct call-site arg tuple.
case class LambdaCallFunctionNameT(
  template: LambdaCallFunctionTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ForwarderFunctionTemplateNameT<'s, 't> {
    pub inner: IFunctionTemplateNameT<'s, 't>,
    pub index: i32,
}
/*
case class ForwarderFunctionTemplateNameT(
  inner: IFunctionTemplateNameT,
  index: Int
) extends INameT with IFunctionTemplateNameT {
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
    interner.intern(ForwarderFunctionNameT(this, inner.makeFunctionName(interner, keywords, templateArgs, params)))//, index))
  }
}


*/
/*
//case class AbstractVirtualDropFunctionTemplateNameT(
//  implName: INameT
//) extends INameT with IFunctionTemplateNameT {
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    interner.intern(
//      AbstractVirtualDropFunctionNameT(implName, templateArgs, params))
//  }
//}

*/
/*
//case class AbstractVirtualDropFunctionNameT(
//  implName: INameT,
//  templateArgs: Vector[ITemplata[ITemplataType]],
//  parameters: Vector[CoordT]
//) extends INameT with IFunctionNameT

*/
/*
//case class OverrideVirtualDropFunctionTemplateNameT(
//  implName: INameT
//) extends INameT with IFunctionTemplateNameT {
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    interner.intern(
//      OverrideVirtualDropFunctionNameT(implName, templateArgs, params))
//  }
//}

*/
/*
//case class OverrideVirtualDropFunctionNameT(
//  implName: INameT,
//  templateArgs: Vector[ITemplata[ITemplataType]],
//  parameters: Vector[CoordT]
//) extends INameT with IFunctionNameT

*/
/*
//case class LambdaTemplateNameT(
//  codeLocation: CodeLocationS
//) extends INameT with IFunctionTemplateNameT {
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    interner.intern(FunctionNameT(interner.intern(FunctionTemplateNameT(keywords.underscoresCall, codeLocation)), templateArgs, params))
//  }
//}
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ConstructorTemplateNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class ConstructorTemplateNameT(
  codeLocation: CodeLocationS
) extends INameT with IFunctionTemplateNameT {
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = vimpl()
}

*/
/*
//case class FreeTemplateNameT(codeLoc: CodeLocationS) extends INameT with IFunctionTemplateNameT {
//  vpass()
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    params match {
//      case Vector(coord) => {
//        interner.intern(FreeNameT(this, templateArgs, coord))
//      }
//      case other => vwat(other)
//    }
//  }
//}
*/
/*
//case class FreeNameT(
//  template: FreeTemplateNameT,
//  templateArgs: Vector[ITemplata[ITemplataType]],
//  coordT: CoordT
//) extends IFunctionNameT {
//  override def parameters: Vector[CoordT] = Vector(coordT)
//}

*/
/*
//// See NSIDN for why we have these virtual names
//case class AbstractVirtualFreeTemplateNameT(codeLoc: CodeLocationS) extends INameT with IFunctionTemplateNameT {
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    val Vector(CoordT(ShareT, kind)) = params
//    interner.intern(AbstractVirtualFreeNameT(templateArgs, kind))
//  }
//}
*/
/*
//// See NSIDN for why we have these virtual names
//case class AbstractVirtualFreeNameT(templateArgs: Vector[ITemplata[ITemplataType]], param: KindT) extends IFunctionNameT {
//  override def parameters: Vector[CoordT] = Vector(CoordT(ShareT, param))
//}
//
*/
/*
//// See NSIDN for why we have these virtual names
//case class OverrideVirtualFreeTemplateNameT(codeLoc: CodeLocationS) extends INameT with IFunctionTemplateNameT {
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    val Vector(CoordT(ShareT, kind)) = params
//    interner.intern(OverrideVirtualFreeNameT(templateArgs, kind))
//  }
//}
*/
/*
//// See NSIDN for why we have these virtual names
//case class OverrideVirtualFreeNameT(templateArgs: Vector[ITemplata[ITemplataType]], param: KindT) extends IFunctionNameT {
//  override def parameters: Vector[CoordT] = Vector(CoordT(ShareT, param))
//}

// Vale has no Self, its just a convenient first name parameter.
// See also SelfNameS.
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SelfNameT<'s, 't> {
    pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class SelfNameT() extends IVarNameT
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ArbitraryNameT<'s, 't> {
    pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class ArbitraryNameT() extends INameT
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CitizenNameT<'s, 't> {
    Struct(&'t StructNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
}
/*
sealed trait CitizenNameT extends ICitizenNameT {
  def template: ICitizenTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}

*/
fn citizen_name_unapply() { panic!("Unmigrated unapply"); }
/*
object CitizenNameT {
  def unapply(c: CitizenNameT): Option[(ICitizenTemplateNameT, Vector[ITemplataT[ITemplataType]])] = {
    c match {
      case StructNameT(template, templateArgs) => Some((template, templateArgs))
      case InterfaceNameT(template, templateArgs) => Some((template, templateArgs))
    }
  }
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructNameT<'s, 't> {
    pub template: IStructTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
case class StructNameT(
  template: IStructTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IStructNameT with CitizenNameT {
  vpass()
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceNameT<'s, 't> {
    pub template: &'t InterfaceTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
case class InterfaceNameT(
  template: InterfaceTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IInterfaceNameT with CitizenNameT {
  vpass()
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LambdaCitizenTemplateNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
// Per @LAGTNGZ, the closure struct doesnt have its own generic parameters, but its associated LambdaCallFunctionTemplateNameT does.
case class LambdaCitizenTemplateNameT(
  codeLocation: CodeLocationS
) extends IStructTemplateNameT {
  override def makeStructName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IStructNameT = {
    vassert(templateArgs.isEmpty)
    interner.intern(LambdaCitizenNameT(this))
  }
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LambdaCitizenNameT<'s, 't> {
    pub template: &'t LambdaCitizenTemplateNameT<'s, 't>,
}
/*
case class LambdaCitizenNameT(
  template: LambdaCitizenTemplateNameT
) extends IStructNameT {
  def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector.empty
  vpass()
}

*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CitizenTemplateNameT<'s, 't> {
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
}
impl<'s, 't> CitizenTemplateNameT<'s, 't> where 's: 't {
  pub fn human_name(&self) -> StrI<'s> {
    match self {
      CitizenTemplateNameT::StructTemplate(x) => x.human_name,
      CitizenTemplateNameT::InterfaceTemplate(x) => x.human_namee,
    }
  }
  /* */
}
/*
sealed trait CitizenTemplateNameT extends ICitizenTemplateNameT {
  def humanName: StrI
  // We don't include a CodeLocation here because:
  // - There's no struct overloading, so there should only ever be one, we don't have to disambiguate
  //   with code locations
  // - It makes it easier to determine the CitizenTemplateNameT from a CitizenNameT which doesn't
  //   remember its code location.
  //codeLocation: CodeLocationS

//  override def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplata[ITemplataType]]): ICitizenNameT = {
//    interner.intern(CitizenNameT(this, templateArgs))
//  }
}

*/
fn citizen_template_name_unapply() { panic!("Unmigrated unapply"); }
/*
object CitizenTemplateNameT {
  def unapply(x: CitizenTemplateNameT): Option[StrI] = {
    x match {
      case StructTemplateNameT(humanName) => Some(humanName)
      case InterfaceTemplateNameT(humanName) => Some(humanName)
      case _ => None
    }
  }
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructTemplateNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class StructTemplateNameT(
  humanName: StrI,
  // We don't include a CodeLocation here because:
  // - There's no struct overloading, so there should only ever be one, we don't have to disambiguate
  //   with code locations
  // - It makes it easier to determine the StructTemplateNameT from a StructNameT which doesn't
  //   remember its code location.
  //   (note from later: not sure this is true anymore, since StructNameT contains a StructTemplateNameT)
  //codeLocation: CodeLocationS
) extends IStructTemplateNameT with CitizenTemplateNameT {
  vpass()

  override def makeStructName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IStructNameT = {
    interner.intern(StructNameT(this, templateArgs))
  }
}
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceTemplateNameT<'s, 't> {
    pub human_namee: StrI<'s>,
    pub _phantom: PhantomData<&'t ()>,
}
/*
case class InterfaceTemplateNameT(
  humanNamee: StrI,
  // We don't include a CodeLocation here because:
  // - There's no struct overloading, so there should only ever be one, we don't have to disambiguate
  //   with code locations
  // - It makes it easier to determine the InterfaceTemplateNameT from a InterfaceNameT which doesn't
  //   remember its code location.
  //codeLocation: CodeLocationS
) extends IInterfaceTemplateNameT with CitizenTemplateNameT with ICitizenTemplateNameT {
  override def humanName = humanNamee
  override def makeInterfaceName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IInterfaceNameT = {
    interner.intern(InterfaceNameT(this, templateArgs))
  }
  override def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): ICitizenNameT = {
    makeInterfaceName(interner, templateArgs)
  }
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructImplTemplateNameT<'s, 't> {
    pub interface: IInterfaceTemplateNameT<'s, 't>,
}
/*
case class AnonymousSubstructImplTemplateNameT(
  interface: IInterfaceTemplateNameT
) extends IImplTemplateNameT {
  override def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): IImplNameT = {
    interner.intern(AnonymousSubstructImplNameT(this, templateArgs, subCitizen))
  }
}
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructImplNameT<'s, 't> {
    pub template: &'t AnonymousSubstructImplTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub sub_citizen: ICitizenTT<'s, 't>,
    pub _must_intern: MustIntern,
}
/*
case class AnonymousSubstructImplNameT(
  template: AnonymousSubstructImplTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  subCitizen: ICitizenTT
) extends IImplNameT


*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructTemplateNameT<'s, 't> {
    pub interface: IInterfaceTemplateNameT<'s, 't>,
}
/*
case class AnonymousSubstructTemplateNameT(
  // This happens to be the same thing that appears before this AnonymousSubstructNameT in a FullNameT.
  // This is really only here to help us calculate the imprecise name for this thing.
  interface: IInterfaceTemplateNameT
) extends IStructTemplateNameT {
  override def makeStructName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IStructNameT = {
    interner.intern(AnonymousSubstructNameT(this, templateArgs))
  }
}
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructConstructorTemplateNameT<'s, 't> {
    pub substruct: ICitizenTemplateNameT<'s, 't>,
}
/*
case class AnonymousSubstructConstructorTemplateNameT(
  substruct: ICitizenTemplateNameT
) extends IFunctionTemplateNameT {
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
    interner.intern(AnonymousSubstructConstructorNameT(this, templateArgs, params))
  }
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructConstructorNameT<'s, 't> {
    pub template: &'t AnonymousSubstructConstructorTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
case class AnonymousSubstructConstructorNameT(
  template: AnonymousSubstructConstructorTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructNameT<'s, 't> {
    pub template: &'t AnonymousSubstructTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub _must_intern: MustIntern,
}
/*
case class AnonymousSubstructNameT(
  // This happens to be the same thing that appears before this AnonymousSubstructNameT in a FullNameT.
  // This is really only here to help us calculate the imprecise name for this thing.
  template: AnonymousSubstructTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IStructNameT {

}
*/
/*
//case class AnonymousSubstructImplNameT() extends INameT {
//
//}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ResolvingEnvNameT<'s, 't> {
    pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class ResolvingEnvNameT() extends INameT {
  vpass()
}

*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CallEnvNameT<'s, 't> {
    pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class CallEnvNameT() extends INameT {
  vpass()
}
*/

// ============================================================================
// From / TryFrom bridges between sub-enums and concrete names.
//
// No Scala counterpart — Scala's sealed-trait hierarchy handled all of these
// implicitly. Rust needs them spelled out.
//
// - From<&'t XxxNameT> for IYyyNameT  — wrap a concrete ref as a sub-enum.
// - From<&'t INarrowT> for IWideT     — upcast a narrow sub-enum to a wider one.
// - TryFrom<&'t INameT> for &'t IYyyNameT — narrow (arena ref) form; panic
//   stub per handoff §6.3 Gotcha — this path requires TypingInterner to intern
//   the narrower sub-enum, which is Slab 3+ work (intern_* are still `panic!()`).
// ============================================================================

// -- Concrete → INameT -------------------------------------------------------
impl<'s, 't> From<&'t ExportTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ExportTemplateNameT<'s, 't>) -> Self { INameT::ExportTemplate(x) }
}
impl<'s, 't> From<&'t ExportNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ExportNameT<'s, 't>) -> Self { INameT::Export(x) }
}
impl<'s, 't> From<&'t ImplTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ImplTemplateNameT<'s, 't>) -> Self { INameT::ImplTemplate(x) }
}
impl<'s, 't> From<&'t ImplNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ImplNameT<'s, 't>) -> Self { INameT::Impl(x) }
}
impl<'s, 't> From<&'t ImplBoundTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ImplBoundTemplateNameT<'s, 't>) -> Self { INameT::ImplBoundTemplate(x) }
}
impl<'s, 't> From<&'t ImplBoundNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ImplBoundNameT<'s, 't>) -> Self { INameT::ImplBound(x) }
}
impl<'s, 't> From<&'t LetNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t LetNameT<'s, 't>) -> Self { INameT::Let(x) }
}
impl<'s, 't> From<&'t ExportAsNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ExportAsNameT<'s, 't>) -> Self { INameT::ExportAs(x) }
}
impl<'s, 't> From<&'t RawArrayNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t RawArrayNameT<'s, 't>) -> Self { INameT::RawArray(x) }
}
impl<'s, 't> From<&'t ReachablePrototypeNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ReachablePrototypeNameT<'s, 't>) -> Self { INameT::ReachablePrototype(x) }
}
impl<'s, 't> From<&'t StaticSizedArrayTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t StaticSizedArrayTemplateNameT<'s, 't>) -> Self { INameT::StaticSizedArrayTemplate(x) }
}
impl<'s, 't> From<&'t StaticSizedArrayNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t StaticSizedArrayNameT<'s, 't>) -> Self { INameT::StaticSizedArray(x) }
}
impl<'s, 't> From<&'t RuntimeSizedArrayTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t RuntimeSizedArrayTemplateNameT<'s, 't>) -> Self { INameT::RuntimeSizedArrayTemplate(x) }
}
impl<'s, 't> From<&'t RuntimeSizedArrayNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t RuntimeSizedArrayNameT<'s, 't>) -> Self { INameT::RuntimeSizedArray(x) }
}
impl<'s, 't> From<&'t KindPlaceholderTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t KindPlaceholderTemplateNameT<'s, 't>) -> Self { INameT::KindPlaceholderTemplate(x) }
}
impl<'s, 't> From<&'t KindPlaceholderNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t KindPlaceholderNameT<'s, 't>) -> Self { INameT::KindPlaceholder(x) }
}
impl<'s, 't> From<&'t NonKindNonRegionPlaceholderNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t NonKindNonRegionPlaceholderNameT<'s, 't>) -> Self { INameT::NonKindNonRegionPlaceholder(x) }
}
impl<'s, 't> From<&'t OverrideDispatcherTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t OverrideDispatcherTemplateNameT<'s, 't>) -> Self { INameT::OverrideDispatcherTemplate(x) }
}
impl<'s, 't> From<&'t OverrideDispatcherNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t OverrideDispatcherNameT<'s, 't>) -> Self { INameT::OverrideDispatcher(x) }
}
impl<'s, 't> From<&'t OverrideDispatcherCaseNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t OverrideDispatcherCaseNameT<'s, 't>) -> Self { INameT::OverrideDispatcherCase(x) }
}
impl<'s, 't> From<&'t TypingPassBlockResultVarNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t TypingPassBlockResultVarNameT<'s, 't>) -> Self { INameT::TypingPassBlockResultVar(x) }
}
impl<'s, 't> From<&'t TypingPassFunctionResultVarNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t TypingPassFunctionResultVarNameT<'s, 't>) -> Self { INameT::TypingPassFunctionResultVar(x) }
}
impl<'s, 't> From<&'t TypingPassTemporaryVarNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t TypingPassTemporaryVarNameT<'s, 't>) -> Self { INameT::TypingPassTemporaryVar(x) }
}
impl<'s, 't> From<&'t TypingPassPatternMemberNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t TypingPassPatternMemberNameT<'s, 't>) -> Self { INameT::TypingPassPatternMember(x) }
}
impl<'s, 't> From<&'t TypingIgnoredParamNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t TypingIgnoredParamNameT<'s, 't>) -> Self { INameT::TypingIgnoredParam(x) }
}
impl<'s, 't> From<&'t TypingPassPatternDestructureeNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t TypingPassPatternDestructureeNameT<'s, 't>) -> Self { INameT::TypingPassPatternDestructuree(x) }
}
impl<'s, 't> From<&'t UnnamedLocalNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t UnnamedLocalNameT<'s, 't>) -> Self { INameT::UnnamedLocal(x) }
}
impl<'s, 't> From<&'t ClosureParamNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ClosureParamNameT<'s, 't>) -> Self { INameT::ClosureParam(x) }
}
impl<'s, 't> From<&'t ConstructingMemberNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ConstructingMemberNameT<'s, 't>) -> Self { INameT::ConstructingMember(x) }
}
impl<'s, 't> From<&'t WhileCondResultNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t WhileCondResultNameT<'s, 't>) -> Self { INameT::WhileCondResult(x) }
}
impl<'s, 't> From<&'t IterableNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t IterableNameT<'s, 't>) -> Self { INameT::Iterable(x) }
}
impl<'s, 't> From<&'t IteratorNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t IteratorNameT<'s, 't>) -> Self { INameT::Iterator(x) }
}
impl<'s, 't> From<&'t IterationOptionNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t IterationOptionNameT<'s, 't>) -> Self { INameT::IterationOption(x) }
}
impl<'s, 't> From<&'t MagicParamNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t MagicParamNameT<'s, 't>) -> Self { INameT::MagicParam(x) }
}
impl<'s, 't> From<&'t CodeVarNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t CodeVarNameT<'s, 't>) -> Self { INameT::CodeVar(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructMemberNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructMemberNameT<'s, 't>) -> Self { INameT::AnonymousSubstructMember(x) }
}
impl<'s, 't> From<&'t PrimitiveNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t PrimitiveNameT<'s, 't>) -> Self { INameT::Primitive(x) }
}
impl<'s, 't> From<&'t PackageTopLevelNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t PackageTopLevelNameT<'s, 't>) -> Self { INameT::PackageTopLevel(x) }
}
impl<'s, 't> From<&'t ProjectNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ProjectNameT<'s, 't>) -> Self { INameT::Project(x) }
}
impl<'s, 't> From<&'t PackageNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t PackageNameT<'s, 't>) -> Self { INameT::Package(x) }
}
impl<'s, 't> From<&'t RuneNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t RuneNameT<'s, 't>) -> Self { INameT::Rune(x) }
}
impl<'s, 't> From<&'t BuildingFunctionNameWithClosuredsT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t BuildingFunctionNameWithClosuredsT<'s, 't>) -> Self { INameT::BuildingFunctionNameWithClosureds(x) }
}
impl<'s, 't> From<&'t ExternTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ExternTemplateNameT<'s, 't>) -> Self { INameT::ExternTemplate(x) }
}
impl<'s, 't> From<&'t ExternNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ExternNameT<'s, 't>) -> Self { INameT::Extern(x) }
}
impl<'s, 't> From<&'t ExternFunctionNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ExternFunctionNameT<'s, 't>) -> Self { INameT::ExternFunction(x) }
}
impl<'s, 't> From<&'t FunctionNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t FunctionNameT<'s, 't>) -> Self { INameT::Function(x) }
}
impl<'s, 't> From<&'t ForwarderFunctionNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ForwarderFunctionNameT<'s, 't>) -> Self { INameT::ForwarderFunction(x) }
}
impl<'s, 't> From<&'t FunctionBoundTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t FunctionBoundTemplateNameT<'s, 't>) -> Self { INameT::FunctionBoundTemplate(x) }
}
impl<'s, 't> From<&'t FunctionBoundNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t FunctionBoundNameT<'s, 't>) -> Self { INameT::FunctionBound(x) }
}
impl<'s, 't> From<&'t PredictedFunctionTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t PredictedFunctionTemplateNameT<'s, 't>) -> Self { INameT::PredictedFunctionTemplate(x) }
}
impl<'s, 't> From<&'t PredictedFunctionNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t PredictedFunctionNameT<'s, 't>) -> Self { INameT::PredictedFunction(x) }
}
impl<'s, 't> From<&'t FunctionTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t FunctionTemplateNameT<'s, 't>) -> Self { INameT::FunctionTemplate(x) }
}
impl<'s, 't> From<&'t LambdaCallFunctionTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t LambdaCallFunctionTemplateNameT<'s, 't>) -> Self { INameT::LambdaCallFunctionTemplate(x) }
}
impl<'s, 't> From<&'t LambdaCallFunctionNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t LambdaCallFunctionNameT<'s, 't>) -> Self { INameT::LambdaCallFunction(x) }
}
impl<'s, 't> From<&'t ForwarderFunctionTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ForwarderFunctionTemplateNameT<'s, 't>) -> Self { INameT::ForwarderFunctionTemplate(x) }
}
impl<'s, 't> From<&'t ConstructorTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ConstructorTemplateNameT<'s, 't>) -> Self { INameT::ConstructorTemplate(x) }
}
impl<'s, 't> From<&'t SelfNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t SelfNameT<'s, 't>) -> Self { INameT::Self_(x) }
}
impl<'s, 't> From<&'t ArbitraryNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ArbitraryNameT<'s, 't>) -> Self { INameT::Arbitrary(x) }
}
impl<'s, 't> From<&'t StructNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t StructNameT<'s, 't>) -> Self { INameT::Struct(x) }
}
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { INameT::Interface(x) }
}
impl<'s, 't> From<&'t LambdaCitizenTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t LambdaCitizenTemplateNameT<'s, 't>) -> Self { INameT::LambdaCitizenTemplate(x) }
}
impl<'s, 't> From<&'t LambdaCitizenNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t LambdaCitizenNameT<'s, 't>) -> Self { INameT::LambdaCitizen(x) }
}
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { INameT::StructTemplate(x) }
}
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { INameT::InterfaceTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructImplTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructImplTemplateNameT<'s, 't>) -> Self { INameT::AnonymousSubstructImplTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructImplNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructImplNameT<'s, 't>) -> Self { INameT::AnonymousSubstructImpl(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructTemplateNameT<'s, 't>) -> Self { INameT::AnonymousSubstructTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructConstructorTemplateNameT<'s, 't>) -> Self { INameT::AnonymousSubstructConstructorTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructConstructorNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructConstructorNameT<'s, 't>) -> Self { INameT::AnonymousSubstructConstructor(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructNameT<'s, 't>) -> Self { INameT::AnonymousSubstruct(x) }
}
impl<'s, 't> From<&'t ResolvingEnvNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t ResolvingEnvNameT<'s, 't>) -> Self { INameT::ResolvingEnv(x) }
}
impl<'s, 't> From<&'t CallEnvNameT<'s, 't>> for INameT<'s, 't> {
    fn from(x: &'t CallEnvNameT<'s, 't>) -> Self { INameT::CallEnv(x) }
}

// -- Concrete → ITemplateNameT -----------------------------------------------
impl<'s, 't> From<&'t ExportTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t ExportTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ExportTemplate(x) }
}
impl<'s, 't> From<&'t ImplTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t ImplTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ImplTemplate(x) }
}
impl<'s, 't> From<&'t ImplBoundTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t ImplBoundTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ImplBoundTemplate(x) }
}
impl<'s, 't> From<&'t StaticSizedArrayTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t StaticSizedArrayTemplateNameT<'s, 't>) -> Self { ITemplateNameT::StaticSizedArrayTemplate(x) }
}
impl<'s, 't> From<&'t RuntimeSizedArrayTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t RuntimeSizedArrayTemplateNameT<'s, 't>) -> Self { ITemplateNameT::RuntimeSizedArrayTemplate(x) }
}
impl<'s, 't> From<&'t KindPlaceholderTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t KindPlaceholderTemplateNameT<'s, 't>) -> Self { ITemplateNameT::KindPlaceholderTemplate(x) }
}
impl<'s, 't> From<&'t OverrideDispatcherTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t OverrideDispatcherTemplateNameT<'s, 't>) -> Self { ITemplateNameT::OverrideDispatcherTemplate(x) }
}
impl<'s, 't> From<&'t OverrideDispatcherCaseNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t OverrideDispatcherCaseNameT<'s, 't>) -> Self { ITemplateNameT::OverrideDispatcherCase(x) }
}
impl<'s, 't> From<&'t ExternTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t ExternTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ExternTemplate(x) }
}
impl<'s, 't> From<&'t ExternFunctionNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t ExternFunctionNameT<'s, 't>) -> Self { ITemplateNameT::ExternFunction(x) }
}
impl<'s, 't> From<&'t FunctionBoundTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t FunctionBoundTemplateNameT<'s, 't>) -> Self { ITemplateNameT::FunctionBoundTemplate(x) }
}
impl<'s, 't> From<&'t PredictedFunctionTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t PredictedFunctionTemplateNameT<'s, 't>) -> Self { ITemplateNameT::PredictedFunctionTemplate(x) }
}
impl<'s, 't> From<&'t FunctionTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t FunctionTemplateNameT<'s, 't>) -> Self { ITemplateNameT::FunctionTemplate(x) }
}
impl<'s, 't> From<&'t LambdaCallFunctionTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t LambdaCallFunctionTemplateNameT<'s, 't>) -> Self { ITemplateNameT::LambdaCallFunctionTemplate(x) }
}
impl<'s, 't> From<&'t ForwarderFunctionTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t ForwarderFunctionTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ForwarderFunctionTemplate(x) }
}
impl<'s, 't> From<&'t ConstructorTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t ConstructorTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ConstructorTemplate(x) }
}
impl<'s, 't> From<&'t LambdaCitizenTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t LambdaCitizenTemplateNameT<'s, 't>) -> Self { ITemplateNameT::LambdaCitizenTemplate(x) }
}
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { ITemplateNameT::StructTemplate(x) }
}
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { ITemplateNameT::InterfaceTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructImplTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructImplTemplateNameT<'s, 't>) -> Self { ITemplateNameT::AnonymousSubstructImplTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructTemplateNameT<'s, 't>) -> Self { ITemplateNameT::AnonymousSubstructTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructConstructorTemplateNameT<'s, 't>) -> Self { ITemplateNameT::AnonymousSubstructConstructorTemplate(x) }
}

// -- Concrete → IInstantiationNameT ------------------------------------------
impl<'s, 't> From<&'t ExportNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t ExportNameT<'s, 't>) -> Self { IInstantiationNameT::Export(x) }
}
impl<'s, 't> From<&'t ImplNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t ImplNameT<'s, 't>) -> Self { IInstantiationNameT::Impl(x) }
}
impl<'s, 't> From<&'t ImplBoundNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t ImplBoundNameT<'s, 't>) -> Self { IInstantiationNameT::ImplBound(x) }
}
impl<'s, 't> From<&'t StaticSizedArrayNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t StaticSizedArrayNameT<'s, 't>) -> Self { IInstantiationNameT::StaticSizedArray(x) }
}
impl<'s, 't> From<&'t RuntimeSizedArrayNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t RuntimeSizedArrayNameT<'s, 't>) -> Self { IInstantiationNameT::RuntimeSizedArray(x) }
}
impl<'s, 't> From<&'t KindPlaceholderNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t KindPlaceholderNameT<'s, 't>) -> Self { IInstantiationNameT::KindPlaceholder(x) }
}
impl<'s, 't> From<&'t OverrideDispatcherNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t OverrideDispatcherNameT<'s, 't>) -> Self { IInstantiationNameT::OverrideDispatcher(x) }
}
impl<'s, 't> From<&'t OverrideDispatcherCaseNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t OverrideDispatcherCaseNameT<'s, 't>) -> Self { IInstantiationNameT::OverrideDispatcherCase(x) }
}
impl<'s, 't> From<&'t ExternNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t ExternNameT<'s, 't>) -> Self { IInstantiationNameT::Extern(x) }
}
impl<'s, 't> From<&'t ExternFunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t ExternFunctionNameT<'s, 't>) -> Self { IInstantiationNameT::ExternFunction(x) }
}
impl<'s, 't> From<&'t FunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t FunctionNameT<'s, 't>) -> Self { IInstantiationNameT::Function(x) }
}
impl<'s, 't> From<&'t ForwarderFunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t ForwarderFunctionNameT<'s, 't>) -> Self { IInstantiationNameT::ForwarderFunction(x) }
}
impl<'s, 't> From<&'t FunctionBoundNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t FunctionBoundNameT<'s, 't>) -> Self { IInstantiationNameT::FunctionBound(x) }
}
impl<'s, 't> From<&'t PredictedFunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t PredictedFunctionNameT<'s, 't>) -> Self { IInstantiationNameT::PredictedFunction(x) }
}
impl<'s, 't> From<&'t LambdaCallFunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t LambdaCallFunctionNameT<'s, 't>) -> Self { IInstantiationNameT::LambdaCallFunction(x) }
}
impl<'s, 't> From<&'t StructNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t StructNameT<'s, 't>) -> Self { IInstantiationNameT::Struct(x) }
}
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { IInstantiationNameT::Interface(x) }
}
impl<'s, 't> From<&'t LambdaCitizenNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t LambdaCitizenNameT<'s, 't>) -> Self { IInstantiationNameT::LambdaCitizen(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructImplNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructImplNameT<'s, 't>) -> Self { IInstantiationNameT::AnonymousSubstructImpl(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructConstructorNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructConstructorNameT<'s, 't>) -> Self { IInstantiationNameT::AnonymousSubstructConstructor(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructNameT<'s, 't>) -> Self { IInstantiationNameT::AnonymousSubstruct(x) }
}

// -- Concrete → IFunctionTemplateNameT --------------------------------------
impl<'s, 't> From<&'t OverrideDispatcherTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> {
    fn from(x: &'t OverrideDispatcherTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::OverrideDispatcherTemplate(x) }
}
impl<'s, 't> From<&'t ExternFunctionNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> {
    fn from(x: &'t ExternFunctionNameT<'s, 't>) -> Self { IFunctionTemplateNameT::ExternFunction(x) }
}
impl<'s, 't> From<&'t FunctionBoundTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> {
    fn from(x: &'t FunctionBoundTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::FunctionBoundTemplate(x) }
}
impl<'s, 't> From<&'t PredictedFunctionTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> {
    fn from(x: &'t PredictedFunctionTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::PredictedFunctionTemplate(x) }
}
impl<'s, 't> From<&'t FunctionTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> {
    fn from(x: &'t FunctionTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::FunctionTemplate(x) }
}
impl<'s, 't> From<&'t LambdaCallFunctionTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> {
    fn from(x: &'t LambdaCallFunctionTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::LambdaCallFunctionTemplate(x) }
}
impl<'s, 't> From<&'t ForwarderFunctionTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> {
    fn from(x: &'t ForwarderFunctionTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::ForwarderFunctionTemplate(x) }
}
impl<'s, 't> From<&'t ConstructorTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> {
    fn from(x: &'t ConstructorTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::ConstructorTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructConstructorTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(x) }
}

// -- Concrete → IFunctionNameT -----------------------------------------------
impl<'s, 't> From<&'t OverrideDispatcherNameT<'s, 't>> for IFunctionNameT<'s, 't> {
    fn from(x: &'t OverrideDispatcherNameT<'s, 't>) -> Self { IFunctionNameT::OverrideDispatcher(x) }
}
impl<'s, 't> From<&'t ExternFunctionNameT<'s, 't>> for IFunctionNameT<'s, 't> {
    fn from(x: &'t ExternFunctionNameT<'s, 't>) -> Self { IFunctionNameT::ExternFunction(x) }
}
impl<'s, 't> From<&'t FunctionNameT<'s, 't>> for IFunctionNameT<'s, 't> {
    fn from(x: &'t FunctionNameT<'s, 't>) -> Self { IFunctionNameT::Function(x) }
}
impl<'s, 't> From<&'t ForwarderFunctionNameT<'s, 't>> for IFunctionNameT<'s, 't> {
    fn from(x: &'t ForwarderFunctionNameT<'s, 't>) -> Self { IFunctionNameT::ForwarderFunction(x) }
}
impl<'s, 't> From<&'t FunctionBoundNameT<'s, 't>> for IFunctionNameT<'s, 't> {
    fn from(x: &'t FunctionBoundNameT<'s, 't>) -> Self { IFunctionNameT::FunctionBound(x) }
}
impl<'s, 't> From<&'t PredictedFunctionNameT<'s, 't>> for IFunctionNameT<'s, 't> {
    fn from(x: &'t PredictedFunctionNameT<'s, 't>) -> Self { IFunctionNameT::PredictedFunction(x) }
}
impl<'s, 't> From<&'t LambdaCallFunctionNameT<'s, 't>> for IFunctionNameT<'s, 't> {
    fn from(x: &'t LambdaCallFunctionNameT<'s, 't>) -> Self { IFunctionNameT::LambdaCallFunction(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructConstructorNameT<'s, 't>> for IFunctionNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructConstructorNameT<'s, 't>) -> Self { IFunctionNameT::AnonymousSubstructConstructor(x) }
}

// -- Concrete → ISuperKindTemplateNameT --------------------------------------
impl<'s, 't> From<&'t KindPlaceholderTemplateNameT<'s, 't>> for ISuperKindTemplateNameT<'s, 't> {
    fn from(x: &'t KindPlaceholderTemplateNameT<'s, 't>) -> Self { ISuperKindTemplateNameT::KindPlaceholderTemplate(x) }
}
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for ISuperKindTemplateNameT<'s, 't> {
    fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { ISuperKindTemplateNameT::InterfaceTemplate(x) }
}

// -- Concrete → ISubKindTemplateNameT ----------------------------------------
impl<'s, 't> From<&'t StaticSizedArrayTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> {
    fn from(x: &'t StaticSizedArrayTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::StaticSizedArrayTemplate(x) }
}
impl<'s, 't> From<&'t RuntimeSizedArrayTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> {
    fn from(x: &'t RuntimeSizedArrayTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::RuntimeSizedArrayTemplate(x) }
}
impl<'s, 't> From<&'t KindPlaceholderTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> {
    fn from(x: &'t KindPlaceholderTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::KindPlaceholderTemplate(x) }
}
impl<'s, 't> From<&'t LambdaCitizenTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> {
    fn from(x: &'t LambdaCitizenTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::LambdaCitizenTemplate(x) }
}
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> {
    fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::StructTemplate(x) }
}
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> {
    fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::InterfaceTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::AnonymousSubstructTemplate(x) }
}

// -- Concrete → ICitizenTemplateNameT ----------------------------------------
impl<'s, 't> From<&'t StaticSizedArrayTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(x: &'t StaticSizedArrayTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::StaticSizedArrayTemplate(x) }
}
impl<'s, 't> From<&'t RuntimeSizedArrayTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(x: &'t RuntimeSizedArrayTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::RuntimeSizedArrayTemplate(x) }
}
impl<'s, 't> From<&'t LambdaCitizenTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(x: &'t LambdaCitizenTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::LambdaCitizenTemplate(x) }
}
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::StructTemplate(x) }
}
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::InterfaceTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::AnonymousSubstructTemplate(x) }
}

// -- Concrete → IStructTemplateNameT -----------------------------------------
impl<'s, 't> From<&'t LambdaCitizenTemplateNameT<'s, 't>> for IStructTemplateNameT<'s, 't> {
    fn from(x: &'t LambdaCitizenTemplateNameT<'s, 't>) -> Self { IStructTemplateNameT::LambdaCitizenTemplate(x) }
}
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for IStructTemplateNameT<'s, 't> {
    fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { IStructTemplateNameT::StructTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructTemplateNameT<'s, 't>> for IStructTemplateNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructTemplateNameT<'s, 't>) -> Self { IStructTemplateNameT::AnonymousSubstructTemplate(x) }
}

// -- Concrete → IInterfaceTemplateNameT --------------------------------------
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for IInterfaceTemplateNameT<'s, 't> {
    fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { IInterfaceTemplateNameT::InterfaceTemplate(x) }
}

// -- Concrete → ISuperKindNameT ----------------------------------------------
impl<'s, 't> From<&'t KindPlaceholderNameT<'s, 't>> for ISuperKindNameT<'s, 't> {
    fn from(x: &'t KindPlaceholderNameT<'s, 't>) -> Self { ISuperKindNameT::KindPlaceholder(x) }
}
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for ISuperKindNameT<'s, 't> {
    fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { ISuperKindNameT::Interface(x) }
}

// -- Concrete → ISubKindNameT ------------------------------------------------
impl<'s, 't> From<&'t StaticSizedArrayNameT<'s, 't>> for ISubKindNameT<'s, 't> {
    fn from(x: &'t StaticSizedArrayNameT<'s, 't>) -> Self { ISubKindNameT::StaticSizedArray(x) }
}
impl<'s, 't> From<&'t RuntimeSizedArrayNameT<'s, 't>> for ISubKindNameT<'s, 't> {
    fn from(x: &'t RuntimeSizedArrayNameT<'s, 't>) -> Self { ISubKindNameT::RuntimeSizedArray(x) }
}
impl<'s, 't> From<&'t KindPlaceholderNameT<'s, 't>> for ISubKindNameT<'s, 't> {
    fn from(x: &'t KindPlaceholderNameT<'s, 't>) -> Self { ISubKindNameT::KindPlaceholder(x) }
}
impl<'s, 't> From<&'t StructNameT<'s, 't>> for ISubKindNameT<'s, 't> {
    fn from(x: &'t StructNameT<'s, 't>) -> Self { ISubKindNameT::Struct(x) }
}
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for ISubKindNameT<'s, 't> {
    fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { ISubKindNameT::Interface(x) }
}
impl<'s, 't> From<&'t LambdaCitizenNameT<'s, 't>> for ISubKindNameT<'s, 't> {
    fn from(x: &'t LambdaCitizenNameT<'s, 't>) -> Self { ISubKindNameT::LambdaCitizen(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructNameT<'s, 't>> for ISubKindNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructNameT<'s, 't>) -> Self { ISubKindNameT::AnonymousSubstruct(x) }
}

// -- Concrete → ICitizenNameT ------------------------------------------------
impl<'s, 't> From<&'t StaticSizedArrayNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(x: &'t StaticSizedArrayNameT<'s, 't>) -> Self { ICitizenNameT::StaticSizedArray(x) }
}
impl<'s, 't> From<&'t RuntimeSizedArrayNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(x: &'t RuntimeSizedArrayNameT<'s, 't>) -> Self { ICitizenNameT::RuntimeSizedArray(x) }
}
impl<'s, 't> From<&'t StructNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(x: &'t StructNameT<'s, 't>) -> Self { ICitizenNameT::Struct(x) }
}
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { ICitizenNameT::Interface(x) }
}
impl<'s, 't> From<&'t LambdaCitizenNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(x: &'t LambdaCitizenNameT<'s, 't>) -> Self { ICitizenNameT::LambdaCitizen(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructNameT<'s, 't>) -> Self { ICitizenNameT::AnonymousSubstruct(x) }
}

// -- Concrete → IStructNameT -------------------------------------------------
impl<'s, 't> From<&'t StructNameT<'s, 't>> for IStructNameT<'s, 't> {
    fn from(x: &'t StructNameT<'s, 't>) -> Self { IStructNameT::Struct(x) }
}
impl<'s, 't> From<&'t LambdaCitizenNameT<'s, 't>> for IStructNameT<'s, 't> {
    fn from(x: &'t LambdaCitizenNameT<'s, 't>) -> Self { IStructNameT::LambdaCitizen(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructNameT<'s, 't>> for IStructNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructNameT<'s, 't>) -> Self { IStructNameT::AnonymousSubstruct(x) }
}

// -- Concrete → IInterfaceNameT ----------------------------------------------
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for IInterfaceNameT<'s, 't> {
    fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { IInterfaceNameT::Interface(x) }
}

// -- Concrete → IImplTemplateNameT -------------------------------------------
impl<'s, 't> From<&'t ImplTemplateNameT<'s, 't>> for IImplTemplateNameT<'s, 't> {
    fn from(x: &'t ImplTemplateNameT<'s, 't>) -> Self { IImplTemplateNameT::ImplTemplate(x) }
}
impl<'s, 't> From<&'t ImplBoundTemplateNameT<'s, 't>> for IImplTemplateNameT<'s, 't> {
    fn from(x: &'t ImplBoundTemplateNameT<'s, 't>) -> Self { IImplTemplateNameT::ImplBoundTemplate(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructImplTemplateNameT<'s, 't>> for IImplTemplateNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructImplTemplateNameT<'s, 't>) -> Self { IImplTemplateNameT::AnonymousSubstructImplTemplate(x) }
}

// -- Concrete → IImplNameT ---------------------------------------------------
impl<'s, 't> From<&'t ImplNameT<'s, 't>> for IImplNameT<'s, 't> {
    fn from(x: &'t ImplNameT<'s, 't>) -> Self { IImplNameT::Impl(x) }
}
impl<'s, 't> From<&'t ImplBoundNameT<'s, 't>> for IImplNameT<'s, 't> {
    fn from(x: &'t ImplBoundNameT<'s, 't>) -> Self { IImplNameT::ImplBound(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructImplNameT<'s, 't>> for IImplNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructImplNameT<'s, 't>) -> Self { IImplNameT::AnonymousSubstructImpl(x) }
}

// -- Concrete → IPlaceholderNameT --------------------------------------------
impl<'s, 't> From<&'t KindPlaceholderNameT<'s, 't>> for IPlaceholderNameT<'s, 't> {
    fn from(x: &'t KindPlaceholderNameT<'s, 't>) -> Self { IPlaceholderNameT::KindPlaceholder(x) }
}
impl<'s, 't> From<&'t NonKindNonRegionPlaceholderNameT<'s, 't>> for IPlaceholderNameT<'s, 't> {
    fn from(x: &'t NonKindNonRegionPlaceholderNameT<'s, 't>) -> Self { IPlaceholderNameT::NonKindNonRegionPlaceholder(x) }
}

// -- Concrete → IVarNameT ----------------------------------------------------
impl<'s, 't> From<&'t TypingPassBlockResultVarNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t TypingPassBlockResultVarNameT<'s, 't>) -> Self { IVarNameT::TypingPassBlockResultVar(x) }
}
impl<'s, 't> From<&'t TypingPassFunctionResultVarNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t TypingPassFunctionResultVarNameT<'s, 't>) -> Self { IVarNameT::TypingPassFunctionResultVar(x) }
}
impl<'s, 't> From<&'t TypingPassTemporaryVarNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t TypingPassTemporaryVarNameT<'s, 't>) -> Self { IVarNameT::TypingPassTemporaryVar(x) }
}
impl<'s, 't> From<&'t TypingPassPatternMemberNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t TypingPassPatternMemberNameT<'s, 't>) -> Self { IVarNameT::TypingPassPatternMember(x) }
}
impl<'s, 't> From<&'t TypingIgnoredParamNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t TypingIgnoredParamNameT<'s, 't>) -> Self { IVarNameT::TypingIgnoredParam(x) }
}
impl<'s, 't> From<&'t TypingPassPatternDestructureeNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t TypingPassPatternDestructureeNameT<'s, 't>) -> Self { IVarNameT::TypingPassPatternDestructuree(x) }
}
impl<'s, 't> From<&'t UnnamedLocalNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t UnnamedLocalNameT<'s, 't>) -> Self { IVarNameT::UnnamedLocal(x) }
}
impl<'s, 't> From<&'t ClosureParamNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t ClosureParamNameT<'s, 't>) -> Self { IVarNameT::ClosureParam(x) }
}
impl<'s, 't> From<&'t ConstructingMemberNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t ConstructingMemberNameT<'s, 't>) -> Self { IVarNameT::ConstructingMember(x) }
}
impl<'s, 't> From<&'t WhileCondResultNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t WhileCondResultNameT<'s, 't>) -> Self { IVarNameT::WhileCondResult(x) }
}
impl<'s, 't> From<&'t IterableNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t IterableNameT<'s, 't>) -> Self { IVarNameT::Iterable(x) }
}
impl<'s, 't> From<&'t IteratorNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t IteratorNameT<'s, 't>) -> Self { IVarNameT::Iterator(x) }
}
impl<'s, 't> From<&'t IterationOptionNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t IterationOptionNameT<'s, 't>) -> Self { IVarNameT::IterationOption(x) }
}
impl<'s, 't> From<&'t MagicParamNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t MagicParamNameT<'s, 't>) -> Self { IVarNameT::MagicParam(x) }
}
impl<'s, 't> From<&'t CodeVarNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t CodeVarNameT<'s, 't>) -> Self { IVarNameT::CodeVar(x) }
}
impl<'s, 't> From<&'t AnonymousSubstructMemberNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t AnonymousSubstructMemberNameT<'s, 't>) -> Self { IVarNameT::AnonymousSubstructMember(x) }
}
impl<'s, 't> From<&'t SelfNameT<'s, 't>> for IVarNameT<'s, 't> {
    fn from(x: &'t SelfNameT<'s, 't>) -> Self { IVarNameT::Self_(x) }
}

// -- Concrete → CitizenNameT / CitizenTemplateNameT --------------------------
impl<'s, 't> From<&'t StructNameT<'s, 't>> for CitizenNameT<'s, 't> {
    fn from(x: &'t StructNameT<'s, 't>) -> Self { CitizenNameT::Struct(x) }
}
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for CitizenNameT<'s, 't> {
    fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { CitizenNameT::Interface(x) }
}
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for CitizenTemplateNameT<'s, 't> {
    fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { CitizenTemplateNameT::StructTemplate(x) }
}
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for CitizenTemplateNameT<'s, 't> {
    fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { CitizenTemplateNameT::InterfaceTemplate(x) }
}

// -- Sub-enum → wider sub-enum (owned input, cascade via .into() on inner ref) --

impl<'s, 't> From<IFunctionTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(f: IFunctionTemplateNameT<'s, 't>) -> Self {
        match f {
            IFunctionTemplateNameT::OverrideDispatcherTemplate(x) => x.into(),
            IFunctionTemplateNameT::ExternFunction(x) => x.into(),
            IFunctionTemplateNameT::FunctionBoundTemplate(x) => x.into(),
            IFunctionTemplateNameT::PredictedFunctionTemplate(x) => x.into(),
            IFunctionTemplateNameT::FunctionTemplate(x) => x.into(),
            IFunctionTemplateNameT::LambdaCallFunctionTemplate(x) => x.into(),
            IFunctionTemplateNameT::ForwarderFunctionTemplate(x) => x.into(),
            IFunctionTemplateNameT::ConstructorTemplate(x) => x.into(),
            IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IFunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(f: IFunctionNameT<'s, 't>) -> Self {
        match f {
            IFunctionNameT::OverrideDispatcher(x) => x.into(),
            IFunctionNameT::ExternFunction(x) => x.into(),
            IFunctionNameT::Function(x) => x.into(),
            IFunctionNameT::ForwarderFunction(x) => x.into(),
            IFunctionNameT::FunctionBound(x) => x.into(),
            IFunctionNameT::PredictedFunction(x) => x.into(),
            IFunctionNameT::LambdaCallFunction(x) => x.into(),
            IFunctionNameT::AnonymousSubstructConstructor(x) => x.into(),
        }
    }
}

impl<'s, 't> From<ISuperKindTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(f: ISuperKindTemplateNameT<'s, 't>) -> Self {
        match f {
            ISuperKindTemplateNameT::KindPlaceholderTemplate(x) => x.into(),
            ISuperKindTemplateNameT::InterfaceTemplate(x) => x.into(),
        }
    }
}

impl<'s, 't> From<ISubKindTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(f: ISubKindTemplateNameT<'s, 't>) -> Self {
        match f {
            ISubKindTemplateNameT::StaticSizedArrayTemplate(x) => x.into(),
            ISubKindTemplateNameT::RuntimeSizedArrayTemplate(x) => x.into(),
            ISubKindTemplateNameT::KindPlaceholderTemplate(x) => x.into(),
            ISubKindTemplateNameT::LambdaCitizenTemplate(x) => x.into(),
            ISubKindTemplateNameT::StructTemplate(x) => x.into(),
            ISubKindTemplateNameT::InterfaceTemplate(x) => x.into(),
            ISubKindTemplateNameT::AnonymousSubstructTemplate(x) => x.into(),
        }
    }
}

impl<'s, 't> From<ICitizenTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> {
    fn from(f: ICitizenTemplateNameT<'s, 't>) -> Self {
        match f {
            ICitizenTemplateNameT::StaticSizedArrayTemplate(x) => x.into(),
            ICitizenTemplateNameT::RuntimeSizedArrayTemplate(x) => x.into(),
            ICitizenTemplateNameT::LambdaCitizenTemplate(x) => x.into(),
            ICitizenTemplateNameT::StructTemplate(x) => x.into(),
            ICitizenTemplateNameT::InterfaceTemplate(x) => x.into(),
            ICitizenTemplateNameT::AnonymousSubstructTemplate(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IStructTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(f: IStructTemplateNameT<'s, 't>) -> Self {
        match f {
            IStructTemplateNameT::LambdaCitizenTemplate(x) => x.into(),
            IStructTemplateNameT::StructTemplate(x) => x.into(),
            IStructTemplateNameT::AnonymousSubstructTemplate(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IInterfaceTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(f: IInterfaceTemplateNameT<'s, 't>) -> Self {
        match f {
            IInterfaceTemplateNameT::InterfaceTemplate(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IInterfaceTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: IInterfaceTemplateNameT<'s, 't>) -> Self {
        match f {
            IInterfaceTemplateNameT::InterfaceTemplate(x) => x.into(),
        }
    }
}
/* Guardian: disable-all */

impl<'s, 't> From<ISuperKindNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(f: ISuperKindNameT<'s, 't>) -> Self {
        match f {
            ISuperKindNameT::KindPlaceholder(x) => x.into(),
            ISuperKindNameT::Interface(x) => x.into(),
        }
    }
}

impl<'s, 't> From<ISubKindNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(f: ISubKindNameT<'s, 't>) -> Self {
        match f {
            ISubKindNameT::StaticSizedArray(x) => x.into(),
            ISubKindNameT::RuntimeSizedArray(x) => x.into(),
            ISubKindNameT::KindPlaceholder(x) => x.into(),
            ISubKindNameT::Struct(x) => x.into(),
            ISubKindNameT::Interface(x) => x.into(),
            ISubKindNameT::LambdaCitizen(x) => x.into(),
            ISubKindNameT::AnonymousSubstruct(x) => x.into(),
        }
    }
}

impl<'s, 't> From<ICitizenNameT<'s, 't>> for ISubKindNameT<'s, 't> {
    fn from(f: ICitizenNameT<'s, 't>) -> Self {
        match f {
            ICitizenNameT::StaticSizedArray(x) => x.into(),
            ICitizenNameT::RuntimeSizedArray(x) => x.into(),
            ICitizenNameT::Struct(x) => x.into(),
            ICitizenNameT::Interface(x) => x.into(),
            ICitizenNameT::LambdaCitizen(x) => x.into(),
            ICitizenNameT::AnonymousSubstruct(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IStructNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(f: IStructNameT<'s, 't>) -> Self {
        match f {
            IStructNameT::Struct(x) => x.into(),
            IStructNameT::LambdaCitizen(x) => x.into(),
            IStructNameT::AnonymousSubstruct(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IInterfaceNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(f: IInterfaceNameT<'s, 't>) -> Self {
        match f {
            IInterfaceNameT::Interface(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IImplTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(f: IImplTemplateNameT<'s, 't>) -> Self {
        match f {
            IImplTemplateNameT::ImplTemplate(x) => x.into(),
            IImplTemplateNameT::ImplBoundTemplate(x) => x.into(),
            IImplTemplateNameT::AnonymousSubstructImplTemplate(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IImplNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(f: IImplNameT<'s, 't>) -> Self {
        match f {
            IImplNameT::Impl(x) => x.into(),
            IImplNameT::ImplBound(x) => x.into(),
            IImplNameT::AnonymousSubstructImpl(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IPlaceholderNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: IPlaceholderNameT<'s, 't>) -> Self {
        match f {
            IPlaceholderNameT::KindPlaceholder(x) => x.into(),
            IPlaceholderNameT::NonKindNonRegionPlaceholder(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IVarNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: IVarNameT<'s, 't>) -> Self {
        match f {
            IVarNameT::TypingPassBlockResultVar(x) => x.into(),
            IVarNameT::TypingPassFunctionResultVar(x) => x.into(),
            IVarNameT::TypingPassTemporaryVar(x) => x.into(),
            IVarNameT::TypingPassPatternMember(x) => x.into(),
            IVarNameT::TypingIgnoredParam(x) => x.into(),
            IVarNameT::TypingPassPatternDestructuree(x) => x.into(),
            IVarNameT::UnnamedLocal(x) => x.into(),
            IVarNameT::ClosureParam(x) => x.into(),
            IVarNameT::ConstructingMember(x) => x.into(),
            IVarNameT::WhileCondResult(x) => x.into(),
            IVarNameT::Iterable(x) => x.into(),
            IVarNameT::Iterator(x) => x.into(),
            IVarNameT::IterationOption(x) => x.into(),
            IVarNameT::MagicParam(x) => x.into(),
            IVarNameT::CodeVar(x) => x.into(),
            IVarNameT::AnonymousSubstructMember(x) => x.into(),
            IVarNameT::Self_(x) => x.into(),
        }
    }
}

impl<'s, 't> From<ITemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: ITemplateNameT<'s, 't>) -> Self {
        match f {
            ITemplateNameT::ExportTemplate(x) => x.into(),
            ITemplateNameT::ImplTemplate(x) => x.into(),
            ITemplateNameT::ImplBoundTemplate(x) => x.into(),
            ITemplateNameT::StaticSizedArrayTemplate(x) => x.into(),
            ITemplateNameT::RuntimeSizedArrayTemplate(x) => x.into(),
            ITemplateNameT::KindPlaceholderTemplate(x) => x.into(),
            ITemplateNameT::OverrideDispatcherTemplate(x) => x.into(),
            ITemplateNameT::OverrideDispatcherCase(x) => x.into(),
            ITemplateNameT::ExternTemplate(x) => x.into(),
            ITemplateNameT::ExternFunction(x) => x.into(),
            ITemplateNameT::FunctionBoundTemplate(x) => x.into(),
            ITemplateNameT::PredictedFunctionTemplate(x) => x.into(),
            ITemplateNameT::FunctionTemplate(x) => x.into(),
            ITemplateNameT::LambdaCallFunctionTemplate(x) => x.into(),
            ITemplateNameT::ForwarderFunctionTemplate(x) => x.into(),
            ITemplateNameT::ConstructorTemplate(x) => x.into(),
            ITemplateNameT::LambdaCitizenTemplate(x) => x.into(),
            ITemplateNameT::StructTemplate(x) => x.into(),
            ITemplateNameT::InterfaceTemplate(x) => x.into(),
            ITemplateNameT::AnonymousSubstructImplTemplate(x) => x.into(),
            ITemplateNameT::AnonymousSubstructTemplate(x) => x.into(),
            ITemplateNameT::AnonymousSubstructConstructorTemplate(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IStructTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: IStructTemplateNameT<'s, 't>) -> Self {
        match f {
            IStructTemplateNameT::StructTemplate(x) => x.into(),
            IStructTemplateNameT::LambdaCitizenTemplate(x) => x.into(),
            IStructTemplateNameT::AnonymousSubstructTemplate(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IFunctionTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: IFunctionTemplateNameT<'s, 't>) -> Self {
        match f {
            IFunctionTemplateNameT::OverrideDispatcherTemplate(x) => x.into(),
            IFunctionTemplateNameT::ExternFunction(x) => x.into(),
            IFunctionTemplateNameT::FunctionBoundTemplate(x) => x.into(),
            IFunctionTemplateNameT::PredictedFunctionTemplate(x) => x.into(),
            IFunctionTemplateNameT::FunctionTemplate(x) => x.into(),
            IFunctionTemplateNameT::LambdaCallFunctionTemplate(x) => x.into(),
            IFunctionTemplateNameT::ForwarderFunctionTemplate(x) => x.into(),
            IFunctionTemplateNameT::ConstructorTemplate(x) => x.into(),
            IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(x) => x.into(),
        }
    }
}

impl<'s, 't> From<IImplTemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: IImplTemplateNameT<'s, 't>) -> Self {
        match f {
            IImplTemplateNameT::ImplTemplate(x) => x.into(),
            IImplTemplateNameT::ImplBoundTemplate(x) => x.into(),
            IImplTemplateNameT::AnonymousSubstructImplTemplate(x) => x.into(),
        }
    }
}
/* Guardian: disable-all */

impl<'s, 't> From<IInstantiationNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: IInstantiationNameT<'s, 't>) -> Self {
        match f {
            IInstantiationNameT::Export(x) => x.into(),
            IInstantiationNameT::Impl(x) => x.into(),
            IInstantiationNameT::ImplBound(x) => x.into(),
            IInstantiationNameT::StaticSizedArray(x) => x.into(),
            IInstantiationNameT::RuntimeSizedArray(x) => x.into(),
            IInstantiationNameT::KindPlaceholder(x) => x.into(),
            IInstantiationNameT::OverrideDispatcher(x) => x.into(),
            IInstantiationNameT::OverrideDispatcherCase(x) => x.into(),
            IInstantiationNameT::Extern(x) => x.into(),
            IInstantiationNameT::ExternFunction(x) => x.into(),
            IInstantiationNameT::Function(x) => x.into(),
            IInstantiationNameT::ForwarderFunction(x) => x.into(),
            IInstantiationNameT::FunctionBound(x) => x.into(),
            IInstantiationNameT::PredictedFunction(x) => x.into(),
            IInstantiationNameT::LambdaCallFunction(x) => x.into(),
            IInstantiationNameT::Struct(x) => x.into(),
            IInstantiationNameT::Interface(x) => x.into(),
            IInstantiationNameT::LambdaCitizen(x) => x.into(),
            IInstantiationNameT::AnonymousSubstructImpl(x) => x.into(),
            IInstantiationNameT::AnonymousSubstructConstructor(x) => x.into(),
            IInstantiationNameT::AnonymousSubstruct(x) => x.into(),
        }
    }
}

impl<'s, 't> From<CitizenNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(f: CitizenNameT<'s, 't>) -> Self {
        match f {
            CitizenNameT::Struct(x) => x.into(),
            CitizenNameT::Interface(x) => x.into(),
        }
    }
}

impl<'s, 't> From<CitizenTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(f: CitizenTemplateNameT<'s, 't>) -> Self {
        match f {
            CitizenTemplateNameT::StructTemplate(x) => x.into(),
            CitizenTemplateNameT::InterfaceTemplate(x) => x.into(),
        }
    }
}

// -- TryFrom<INameT> for IYyyNameT (wide → narrow, owned values, no interner) --
// These are free stack-only conversions under the inline-owned sub-enum design:
// pattern-match INameT, pick the variants that belong to the narrower sub-enum,
// and rewrap. No arena allocation needed.

impl<'s, 't> TryFrom<INameT<'s, 't>> for ITemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::ExportTemplate(x) => Ok(ITemplateNameT::ExportTemplate(x)),
            INameT::ImplTemplate(x) => Ok(ITemplateNameT::ImplTemplate(x)),
            INameT::ImplBoundTemplate(x) => Ok(ITemplateNameT::ImplBoundTemplate(x)),
            INameT::StaticSizedArrayTemplate(x) => Ok(ITemplateNameT::StaticSizedArrayTemplate(x)),
            INameT::RuntimeSizedArrayTemplate(x) => Ok(ITemplateNameT::RuntimeSizedArrayTemplate(x)),
            INameT::KindPlaceholderTemplate(x) => Ok(ITemplateNameT::KindPlaceholderTemplate(x)),
            INameT::OverrideDispatcherTemplate(x) => Ok(ITemplateNameT::OverrideDispatcherTemplate(x)),
            INameT::OverrideDispatcherCase(x) => Ok(ITemplateNameT::OverrideDispatcherCase(x)),
            INameT::ExternTemplate(x) => Ok(ITemplateNameT::ExternTemplate(x)),
            INameT::ExternFunction(x) => Ok(ITemplateNameT::ExternFunction(x)),
            INameT::FunctionBoundTemplate(x) => Ok(ITemplateNameT::FunctionBoundTemplate(x)),
            INameT::PredictedFunctionTemplate(x) => Ok(ITemplateNameT::PredictedFunctionTemplate(x)),
            INameT::FunctionTemplate(x) => Ok(ITemplateNameT::FunctionTemplate(x)),
            INameT::LambdaCallFunctionTemplate(x) => Ok(ITemplateNameT::LambdaCallFunctionTemplate(x)),
            INameT::ForwarderFunctionTemplate(x) => Ok(ITemplateNameT::ForwarderFunctionTemplate(x)),
            INameT::ConstructorTemplate(x) => Ok(ITemplateNameT::ConstructorTemplate(x)),
            INameT::LambdaCitizenTemplate(x) => Ok(ITemplateNameT::LambdaCitizenTemplate(x)),
            INameT::StructTemplate(x) => Ok(ITemplateNameT::StructTemplate(x)),
            INameT::InterfaceTemplate(x) => Ok(ITemplateNameT::InterfaceTemplate(x)),
            INameT::AnonymousSubstructImplTemplate(x) => Ok(ITemplateNameT::AnonymousSubstructImplTemplate(x)),
            INameT::AnonymousSubstructTemplate(x) => Ok(ITemplateNameT::AnonymousSubstructTemplate(x)),
            INameT::AnonymousSubstructConstructorTemplate(x) => Ok(ITemplateNameT::AnonymousSubstructConstructorTemplate(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::Export(x) => Ok(IInstantiationNameT::Export(x)),
            INameT::Impl(x) => Ok(IInstantiationNameT::Impl(x)),
            INameT::ImplBound(x) => Ok(IInstantiationNameT::ImplBound(x)),
            INameT::StaticSizedArray(x) => Ok(IInstantiationNameT::StaticSizedArray(x)),
            INameT::RuntimeSizedArray(x) => Ok(IInstantiationNameT::RuntimeSizedArray(x)),
            INameT::KindPlaceholder(x) => Ok(IInstantiationNameT::KindPlaceholder(x)),
            INameT::OverrideDispatcher(x) => Ok(IInstantiationNameT::OverrideDispatcher(x)),
            INameT::OverrideDispatcherCase(x) => Ok(IInstantiationNameT::OverrideDispatcherCase(x)),
            INameT::Extern(x) => Ok(IInstantiationNameT::Extern(x)),
            INameT::ExternFunction(x) => Ok(IInstantiationNameT::ExternFunction(x)),
            INameT::Function(x) => Ok(IInstantiationNameT::Function(x)),
            INameT::ForwarderFunction(x) => Ok(IInstantiationNameT::ForwarderFunction(x)),
            INameT::FunctionBound(x) => Ok(IInstantiationNameT::FunctionBound(x)),
            INameT::PredictedFunction(x) => Ok(IInstantiationNameT::PredictedFunction(x)),
            INameT::LambdaCallFunction(x) => Ok(IInstantiationNameT::LambdaCallFunction(x)),
            INameT::Struct(x) => Ok(IInstantiationNameT::Struct(x)),
            INameT::Interface(x) => Ok(IInstantiationNameT::Interface(x)),
            INameT::LambdaCitizen(x) => Ok(IInstantiationNameT::LambdaCitizen(x)),
            INameT::AnonymousSubstructImpl(x) => Ok(IInstantiationNameT::AnonymousSubstructImpl(x)),
            INameT::AnonymousSubstructConstructor(x) => Ok(IInstantiationNameT::AnonymousSubstructConstructor(x)),
            INameT::AnonymousSubstruct(x) => Ok(IInstantiationNameT::AnonymousSubstruct(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::OverrideDispatcherTemplate(x) => Ok(IFunctionTemplateNameT::OverrideDispatcherTemplate(x)),
            INameT::ExternFunction(x) => Ok(IFunctionTemplateNameT::ExternFunction(x)),
            INameT::FunctionBoundTemplate(x) => Ok(IFunctionTemplateNameT::FunctionBoundTemplate(x)),
            INameT::PredictedFunctionTemplate(x) => Ok(IFunctionTemplateNameT::PredictedFunctionTemplate(x)),
            INameT::FunctionTemplate(x) => Ok(IFunctionTemplateNameT::FunctionTemplate(x)),
            INameT::LambdaCallFunctionTemplate(x) => Ok(IFunctionTemplateNameT::LambdaCallFunctionTemplate(x)),
            INameT::ForwarderFunctionTemplate(x) => Ok(IFunctionTemplateNameT::ForwarderFunctionTemplate(x)),
            INameT::ConstructorTemplate(x) => Ok(IFunctionTemplateNameT::ConstructorTemplate(x)),
            INameT::AnonymousSubstructConstructorTemplate(x) => Ok(IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for IFunctionNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::OverrideDispatcher(x) => Ok(IFunctionNameT::OverrideDispatcher(x)),
            INameT::ExternFunction(x) => Ok(IFunctionNameT::ExternFunction(x)),
            INameT::Function(x) => Ok(IFunctionNameT::Function(x)),
            INameT::ForwarderFunction(x) => Ok(IFunctionNameT::ForwarderFunction(x)),
            INameT::FunctionBound(x) => Ok(IFunctionNameT::FunctionBound(x)),
            INameT::PredictedFunction(x) => Ok(IFunctionNameT::PredictedFunction(x)),
            INameT::LambdaCallFunction(x) => Ok(IFunctionNameT::LambdaCallFunction(x)),
            INameT::AnonymousSubstructConstructor(x) => Ok(IFunctionNameT::AnonymousSubstructConstructor(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for ISuperKindTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::KindPlaceholderTemplate(x) => Ok(ISuperKindTemplateNameT::KindPlaceholderTemplate(x)),
            INameT::InterfaceTemplate(x) => Ok(ISuperKindTemplateNameT::InterfaceTemplate(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::StaticSizedArrayTemplate(x) => Ok(ISubKindTemplateNameT::StaticSizedArrayTemplate(x)),
            INameT::RuntimeSizedArrayTemplate(x) => Ok(ISubKindTemplateNameT::RuntimeSizedArrayTemplate(x)),
            INameT::KindPlaceholderTemplate(x) => Ok(ISubKindTemplateNameT::KindPlaceholderTemplate(x)),
            INameT::LambdaCitizenTemplate(x) => Ok(ISubKindTemplateNameT::LambdaCitizenTemplate(x)),
            INameT::StructTemplate(x) => Ok(ISubKindTemplateNameT::StructTemplate(x)),
            INameT::InterfaceTemplate(x) => Ok(ISubKindTemplateNameT::InterfaceTemplate(x)),
            INameT::AnonymousSubstructTemplate(x) => Ok(ISubKindTemplateNameT::AnonymousSubstructTemplate(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::StaticSizedArrayTemplate(x) => Ok(ICitizenTemplateNameT::StaticSizedArrayTemplate(x)),
            INameT::RuntimeSizedArrayTemplate(x) => Ok(ICitizenTemplateNameT::RuntimeSizedArrayTemplate(x)),
            INameT::LambdaCitizenTemplate(x) => Ok(ICitizenTemplateNameT::LambdaCitizenTemplate(x)),
            INameT::StructTemplate(x) => Ok(ICitizenTemplateNameT::StructTemplate(x)),
            INameT::InterfaceTemplate(x) => Ok(ICitizenTemplateNameT::InterfaceTemplate(x)),
            INameT::AnonymousSubstructTemplate(x) => Ok(ICitizenTemplateNameT::AnonymousSubstructTemplate(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for IStructTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::LambdaCitizenTemplate(x) => Ok(IStructTemplateNameT::LambdaCitizenTemplate(x)),
            INameT::StructTemplate(x) => Ok(IStructTemplateNameT::StructTemplate(x)),
            INameT::AnonymousSubstructTemplate(x) => Ok(IStructTemplateNameT::AnonymousSubstructTemplate(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for IInterfaceTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::InterfaceTemplate(x) => Ok(IInterfaceTemplateNameT::InterfaceTemplate(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for ISuperKindNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::KindPlaceholder(x) => Ok(ISuperKindNameT::KindPlaceholder(x)),
            INameT::Interface(x) => Ok(ISuperKindNameT::Interface(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for ISubKindNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::StaticSizedArray(x) => Ok(ISubKindNameT::StaticSizedArray(x)),
            INameT::RuntimeSizedArray(x) => Ok(ISubKindNameT::RuntimeSizedArray(x)),
            INameT::KindPlaceholder(x) => Ok(ISubKindNameT::KindPlaceholder(x)),
            INameT::Struct(x) => Ok(ISubKindNameT::Struct(x)),
            INameT::Interface(x) => Ok(ISubKindNameT::Interface(x)),
            INameT::LambdaCitizen(x) => Ok(ISubKindNameT::LambdaCitizen(x)),
            INameT::AnonymousSubstruct(x) => Ok(ISubKindNameT::AnonymousSubstruct(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for ICitizenNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::StaticSizedArray(x) => Ok(ICitizenNameT::StaticSizedArray(x)),
            INameT::RuntimeSizedArray(x) => Ok(ICitizenNameT::RuntimeSizedArray(x)),
            INameT::Struct(x) => Ok(ICitizenNameT::Struct(x)),
            INameT::Interface(x) => Ok(ICitizenNameT::Interface(x)),
            INameT::LambdaCitizen(x) => Ok(ICitizenNameT::LambdaCitizen(x)),
            INameT::AnonymousSubstruct(x) => Ok(ICitizenNameT::AnonymousSubstruct(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for IStructNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::Struct(x) => Ok(IStructNameT::Struct(x)),
            INameT::LambdaCitizen(x) => Ok(IStructNameT::LambdaCitizen(x)),
            INameT::AnonymousSubstruct(x) => Ok(IStructNameT::AnonymousSubstruct(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for IInterfaceNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::Interface(x) => Ok(IInterfaceNameT::Interface(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for IImplTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::ImplTemplate(x) => Ok(IImplTemplateNameT::ImplTemplate(x)),
            INameT::ImplBoundTemplate(x) => Ok(IImplTemplateNameT::ImplBoundTemplate(x)),
            INameT::AnonymousSubstructImplTemplate(x) => Ok(IImplTemplateNameT::AnonymousSubstructImplTemplate(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for IImplNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::Impl(x) => Ok(IImplNameT::Impl(x)),
            INameT::ImplBound(x) => Ok(IImplNameT::ImplBound(x)),
            INameT::AnonymousSubstructImpl(x) => Ok(IImplNameT::AnonymousSubstructImpl(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for IPlaceholderNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::KindPlaceholder(x) => Ok(IPlaceholderNameT::KindPlaceholder(x)),
            INameT::NonKindNonRegionPlaceholder(x) => Ok(IPlaceholderNameT::NonKindNonRegionPlaceholder(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for IVarNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::TypingPassBlockResultVar(x) => Ok(IVarNameT::TypingPassBlockResultVar(x)),
            INameT::TypingPassFunctionResultVar(x) => Ok(IVarNameT::TypingPassFunctionResultVar(x)),
            INameT::TypingPassTemporaryVar(x) => Ok(IVarNameT::TypingPassTemporaryVar(x)),
            INameT::TypingPassPatternMember(x) => Ok(IVarNameT::TypingPassPatternMember(x)),
            INameT::TypingIgnoredParam(x) => Ok(IVarNameT::TypingIgnoredParam(x)),
            INameT::TypingPassPatternDestructuree(x) => Ok(IVarNameT::TypingPassPatternDestructuree(x)),
            INameT::UnnamedLocal(x) => Ok(IVarNameT::UnnamedLocal(x)),
            INameT::ClosureParam(x) => Ok(IVarNameT::ClosureParam(x)),
            INameT::ConstructingMember(x) => Ok(IVarNameT::ConstructingMember(x)),
            INameT::WhileCondResult(x) => Ok(IVarNameT::WhileCondResult(x)),
            INameT::Iterable(x) => Ok(IVarNameT::Iterable(x)),
            INameT::Iterator(x) => Ok(IVarNameT::Iterator(x)),
            INameT::IterationOption(x) => Ok(IVarNameT::IterationOption(x)),
            INameT::MagicParam(x) => Ok(IVarNameT::MagicParam(x)),
            INameT::CodeVar(x) => Ok(IVarNameT::CodeVar(x)),
            INameT::AnonymousSubstructMember(x) => Ok(IVarNameT::AnonymousSubstructMember(x)),
            INameT::Self_(x) => Ok(IVarNameT::Self_(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for CitizenNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::Struct(x) => Ok(CitizenNameT::Struct(x)),
            INameT::Interface(x) => Ok(CitizenNameT::Interface(x)),
            _ => Err(()),
        }
    }
}

impl<'s, 't> TryFrom<INameT<'s, 't>> for CitizenTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(n: INameT<'s, 't>) -> Result<Self, ()> {
        match n {
            INameT::StructTemplate(x) => Ok(CitizenTemplateNameT::StructTemplate(x)),
            INameT::InterfaceTemplate(x) => Ok(CitizenTemplateNameT::InterfaceTemplate(x)),
            _ => Err(()),
        }
    }
}

// ============================================================================
// IDEPFL *ValT companion types (Slab 2 Step 6).
//
// The typing interner canonicalizes each concrete name struct — two
// structurally-equivalent values share the same `&'t XxxNameT` arena
// allocation. The lookup flow is the IDEPFL transient/permanent split:
//
//   1. Caller builds a transient `XxxNameValT<'s, 't, 'tmp>` on the stack.
//   2. Interner hashes the Val, probes its per-family HashMap.
//   3. On HIT: return the existing `&'t XxxNameT` — transient Val discarded.
//   4. On MISS: promote Val's slice fields into the arena (via
//      `promote_in()`), allocate the permanent `XxxNameT`, install in the
//      HashMap, return the new `&'t XxxNameT`.
//
// Three IDEPFL kinds (see `.claude/rules/postparser/IDEPFL-postparser-interning.md`):
//
// - **Simple** — struct fields are all Copy primitives or scout-lifetime refs
//   (`StrI<'s>`, `CodeLocationS<'s>`, `RangeS<'s>`, `IRuneS<'s>`, `i32`).
//   The struct itself is the Val — no separate type needed. The interner
//   passes the struct by value as its HashMap key.
//
// - **Shallow** — struct holds `&'t` refs to parent interned types, or
//   inline-owned sub-enum values (already canonical since concretes are
//   pointer-interned). The struct itself is still the Val; no 'tmp lifetime
//   is required because there's nothing transient to borrow.
//
// - **Transient-with-'tmp** — struct has `&'t [...]` arena slices
//   (`template_args`, `parameters`, `init_steps`, etc.). The transient Val
//   replaces those with `&'tmp [...]` slices borrowed from a stack-local Vec
//   so lookup can hash/compare without allocating. Slices are canonicalized
//   into `&'t` on miss.
//
// Under the inline-owned sub-enum design (§6.2 / §6.3): sub-enum families
// (`IFunctionNameT`, `IStructNameT`, `INameT`, etc.) are NOT interned — they
// are 16-byte inline Copy values constructed on the stack. Only concrete
// name structs and `IdT` need Val companions.
//
// Simple/shallow concretes below use the struct itself as their Val. The
// 15 transient concretes each get a `*ValT` struct defined explicitly.
// ============================================================================

// -- IdValT: transient Val for IdT --------------------------------------------
// `init_steps: &'tmp [INameT<'s, 't>]` replaces the permanent IdT's `&'t`
// slice so callers can hash a trial IdT against the interner without yet
// arena-allocating the slice. Monomorphic per
// `docs/reasoning/idt-typed-view-alternatives.md`.
// Derive Hash/PartialEq/Eq: content-based (iterates the init_steps slice,
// delegates to &ref's target). This is *required* for heterogeneous lookup:
// the hash must be consistent whether the Val's slice is 'tmp-borrowed (query)
// or 't-arena-allocated (stored). Pointer-based hashing would fail to match
// structurally-equal Vals with different slice pointers.
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct IdValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub package_coord: &'s PackageCoordinate<'s>,
    pub init_steps: &'tmp [INameT<'s, 't>],
    pub local_name: INameT<'s, 't>,
}
/* Guardian: disable-all */

// Query wrapper for heterogeneous lookup (IdValT<'s, 't, 'tmp> against stored
// IdValT<'s, 't, 't>). Mirrors postparsing::names::RuneValQuery.
/// Interning transient (see @TFITCX)
pub struct IdValQuery<'a, 's, 't, 'tmp>(pub &'a IdValT<'s, 't, 'tmp>)
where 's: 't, 't: 'tmp;
/* Guardian: disable-all */

impl<'a, 's, 't, 'tmp> Hash for IdValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state); }
}

impl<'a, 's, 't, 'tmp> hashbrown::Equivalent<IdValT<'s, 't, 't>> for IdValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn equivalent(&self, key: &IdValT<'s, 't, 't>) -> bool {
        self.0.package_coord == key.package_coord
            && self.0.init_steps == key.init_steps
            && self.0.local_name == key.local_name
    }
    /* Guardian: disable-all */
}

// -- Transient-with-'tmp Val types for the 15 concrete names with slices ----
// Fields match the permanent struct verbatim, except each `&'t [...]` slice
// is replaced by `&'tmp [...]`.

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct ImplNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: &'t ImplTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
    pub sub_citizen: ICitizenTT<'s, 't>,
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct ImplBoundNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: &'t ImplBoundTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct OverrideDispatcherNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: &'t OverrideDispatcherTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
    pub parameters: &'tmp [CoordT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct OverrideDispatcherCaseNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub independent_impl_template_args: &'tmp [ITemplataT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct ExternFunctionNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub human_name: StrI<'s>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
    pub parameters: &'tmp [CoordT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct FunctionNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: &'t FunctionTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
    pub parameters: &'tmp [CoordT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct FunctionBoundNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: &'t FunctionBoundTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
    pub parameters: &'tmp [CoordT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct PredictedFunctionNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: &'t PredictedFunctionTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
    pub parameters: &'tmp [CoordT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct LambdaCallFunctionTemplateNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub code_location: CodeLocationS<'s>,
    pub param_types: &'tmp [CoordT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct LambdaCallFunctionNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: &'t LambdaCallFunctionTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
    pub parameters: &'tmp [CoordT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct StructNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: IStructTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct InterfaceNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: &'t InterfaceTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct AnonymousSubstructImplNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: &'t AnonymousSubstructImplTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
    pub sub_citizen: ICitizenTT<'s, 't>,
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct AnonymousSubstructConstructorNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: &'t AnonymousSubstructConstructorTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
    pub parameters: &'tmp [CoordT<'s, 't>],
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct AnonymousSubstructNameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub template: &'t AnonymousSubstructTemplateNameT<'s, 't>,
    pub template_args: &'tmp [ITemplataT<'s, 't>],
}
/* Guardian: disable-all */

// -- Simple / shallow concretes (reuse struct itself as Val) ------------------
// The following ~45 concrete name structs have no `&'t [...]` slices, so
// their permanent struct doubles as the Val (they're already `Copy` and
// `Hash + Eq`). No separate `*ValT` type is defined for:
//
//   Fieldless (8): StaticSizedArrayTemplateNameT, RuntimeSizedArrayTemplateNameT,
//   TypingPassFunctionResultVarNameT, PackageTopLevelNameT, SelfNameT,
//   ArbitraryNameT, ResolvingEnvNameT, CallEnvNameT.
//
//   Scout-only fields (31): ExportTemplateNameT, ImplTemplateNameT,
//   ImplBoundTemplateNameT, LetNameT, ExportAsNameT, ReachablePrototypeNameT,
//   KindPlaceholderTemplateNameT, NonKindNonRegionPlaceholderNameT,
//   TypingIgnoredParamNameT, UnnamedLocalNameT, ClosureParamNameT,
//   ConstructingMemberNameT, WhileCondResultNameT, IterableNameT,
//   IteratorNameT, IterationOptionNameT, MagicParamNameT, CodeVarNameT,
//   AnonymousSubstructMemberNameT, PrimitiveNameT, ProjectNameT, PackageNameT,
//   RuneNameT, ExternTemplateNameT, FunctionBoundTemplateNameT,
//   PredictedFunctionTemplateNameT, FunctionTemplateNameT,
//   ConstructorTemplateNameT, LambdaCitizenTemplateNameT, StructTemplateNameT,
//   InterfaceTemplateNameT.
//
//   Typing refs / inline sub-enums (no slices, ~18): ExportNameT, RawArrayNameT,
//   StaticSizedArrayNameT, RuntimeSizedArrayNameT, KindPlaceholderNameT,
//   TypingPassBlockResultVarNameT, TypingPassTemporaryVarNameT,
//   TypingPassPatternMemberNameT, TypingPassPatternDestructureeNameT,
//   BuildingFunctionNameWithClosuredsT, ExternNameT, ForwarderFunctionNameT,
//   ForwarderFunctionTemplateNameT, LambdaCitizenNameT,
//   AnonymousSubstructImplTemplateNameT, AnonymousSubstructTemplateNameT,
//   AnonymousSubstructConstructorTemplateNameT, OverrideDispatcherTemplateNameT.
//
// (OverrideDispatcherTemplateNameT is shallow because it holds an inline
// `IdT<'s, 't>` — the IdT's own init_steps slice
// must be canonicalized via IdValT before this Val is constructed.)

// ============================================================================
// Hash/PartialEq/Eq + Query wrappers for the 15 transient Vals.
//
// Pattern per IDEPFL: stored values live at `'tmp = 't` and compare/hash via
// pointer identity for their slices (the interner canonicalizes them). The
// Query wrapper lets a `'tmp`-borrowed Val look up against a stored key by
// comparing slice CONTENTS, not pointers.
// ============================================================================

// Each transient Val uses derived Hash/PartialEq/Eq (added below via struct attr).
// This macro emits only the Query wrapper + its Equivalent impl for heterogeneous
// lookup; the derive on the Val struct itself gives content-based hash+eq that's
// consistent across 'tmp differences.
macro_rules! transient_name_val_impls {
    (
        $val:ident, $query:ident,
        refs = [ $( $r:ident ),* ],
        slices = [ $( $s:ident ),* ],
        inline = [ $( $i:ident ),* ]
    ) => {
        pub struct $query<'a, 's, 't, 'tmp>(pub &'a $val<'s, 't, 'tmp>)
        where 's: 't, 't: 'tmp;

        impl<'a, 's, 't, 'tmp> Hash for $query<'a, 's, 't, 'tmp>
        where 's: 't, 't: 'tmp,
        {
            fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state); }
        }

        impl<'a, 's, 't, 'tmp> hashbrown::Equivalent<$val<'s, 't, 't>> for $query<'a, 's, 't, 'tmp>
        where 's: 't, 't: 'tmp,
        {
            #[allow(unused_mut)]
            fn equivalent(&self, key: &$val<'s, 't, 't>) -> bool {
                let mut ok = true;
                $( ok = ok && self.0.$r == key.$r; )*
                $( ok = ok && self.0.$s == key.$s; )*
                $( ok = ok && self.0.$i == key.$i; )*
                ok
            }
            /* Guardian: disable-all */
        }
    };
}
/* Guardian: disable-all */

transient_name_val_impls!(ImplNameValT, ImplNameValQuery,
    refs = [template], slices = [template_args], inline = [sub_citizen]);
transient_name_val_impls!(ImplBoundNameValT, ImplBoundNameValQuery,
    refs = [template], slices = [template_args], inline = []);
transient_name_val_impls!(OverrideDispatcherNameValT, OverrideDispatcherNameValQuery,
    refs = [template], slices = [template_args, parameters], inline = []);
transient_name_val_impls!(OverrideDispatcherCaseNameValT, OverrideDispatcherCaseNameValQuery,
    refs = [], slices = [independent_impl_template_args], inline = []);
transient_name_val_impls!(ExternFunctionNameValT, ExternFunctionNameValQuery,
    refs = [], slices = [parameters], inline = [human_name]);
transient_name_val_impls!(FunctionNameValT, FunctionNameValQuery,
    refs = [template], slices = [template_args, parameters], inline = []);
transient_name_val_impls!(FunctionBoundNameValT, FunctionBoundNameValQuery,
    refs = [template], slices = [template_args, parameters], inline = []);
transient_name_val_impls!(PredictedFunctionNameValT, PredictedFunctionNameValQuery,
    refs = [template], slices = [template_args, parameters], inline = []);
transient_name_val_impls!(LambdaCallFunctionTemplateNameValT, LambdaCallFunctionTemplateNameValQuery,
    refs = [], slices = [param_types], inline = [code_location]);
transient_name_val_impls!(LambdaCallFunctionNameValT, LambdaCallFunctionNameValQuery,
    refs = [template], slices = [template_args, parameters], inline = []);
transient_name_val_impls!(StructNameValT, StructNameValQuery,
    refs = [], slices = [template_args], inline = [template]);
transient_name_val_impls!(InterfaceNameValT, InterfaceNameValQuery,
    refs = [template], slices = [template_args], inline = []);
transient_name_val_impls!(AnonymousSubstructImplNameValT, AnonymousSubstructImplNameValQuery,
    refs = [template], slices = [template_args], inline = [sub_citizen]);
transient_name_val_impls!(AnonymousSubstructConstructorNameValT, AnonymousSubstructConstructorNameValQuery,
    refs = [template], slices = [template_args, parameters], inline = []);
transient_name_val_impls!(AnonymousSubstructNameValT, AnonymousSubstructNameValQuery,
    refs = [template], slices = [template_args], inline = []);

// ============================================================================
// INameValT — the union Val enum for the name-interning family.
//
// Per handoff-slab-4.md Gotcha 2 (6-family-map design mirroring scout's
// INameValS/INameS). One variant per concrete name in INameT. For simple names
// the variant payload is the concrete struct by value; for transient names
// (15, carrying slices) the payload is the concrete `*ValT` struct.
//
// Hash is derived (content-based; iterates slice contents). Query wrapper
// provides heterogeneous lookup (`'tmp` → `'t`) via Equivalent.
// ============================================================================

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum INameValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    ExportTemplate(ExportTemplateNameT<'s, 't>),
    Export(ExportNameT<'s, 't>),
    ImplTemplate(ImplTemplateNameT<'s, 't>),
    Impl(ImplNameValT<'s, 't, 'tmp>),
    ImplBoundTemplate(ImplBoundTemplateNameT<'s, 't>),
    ImplBound(ImplBoundNameValT<'s, 't, 'tmp>),
    Let(LetNameT<'s, 't>),
    ExportAs(ExportAsNameT<'s, 't>),
    RawArray(RawArrayNameT<'s, 't>),
    ReachablePrototype(ReachablePrototypeNameT<'s, 't>),
    StaticSizedArrayTemplate(StaticSizedArrayTemplateNameT<'s, 't>),
    StaticSizedArray(StaticSizedArrayNameT<'s, 't>),
    RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateNameT<'s, 't>),
    RuntimeSizedArray(RuntimeSizedArrayNameT<'s, 't>),
    KindPlaceholderTemplate(KindPlaceholderTemplateNameT<'s, 't>),
    KindPlaceholder(KindPlaceholderNameT<'s, 't>),
    NonKindNonRegionPlaceholder(NonKindNonRegionPlaceholderNameT<'s, 't>),
    OverrideDispatcherTemplate(OverrideDispatcherTemplateNameT<'s, 't>),
    OverrideDispatcher(OverrideDispatcherNameValT<'s, 't, 'tmp>),
    OverrideDispatcherCase(OverrideDispatcherCaseNameValT<'s, 't, 'tmp>),
    TypingPassBlockResultVar(TypingPassBlockResultVarNameT<'s, 't>),
    TypingPassFunctionResultVar(TypingPassFunctionResultVarNameT<'s, 't>),
    TypingPassTemporaryVar(TypingPassTemporaryVarNameT<'s, 't>),
    TypingPassPatternMember(TypingPassPatternMemberNameT<'s, 't>),
    TypingIgnoredParam(TypingIgnoredParamNameT<'s, 't>),
    TypingPassPatternDestructuree(TypingPassPatternDestructureeNameT<'s, 't>),
    UnnamedLocal(UnnamedLocalNameT<'s, 't>),
    ClosureParam(ClosureParamNameT<'s, 't>),
    ConstructingMember(ConstructingMemberNameT<'s, 't>),
    WhileCondResult(WhileCondResultNameT<'s, 't>),
    Iterable(IterableNameT<'s, 't>),
    Iterator(IteratorNameT<'s, 't>),
    IterationOption(IterationOptionNameT<'s, 't>),
    MagicParam(MagicParamNameT<'s, 't>),
    CodeVar(CodeVarNameT<'s, 't>),
    AnonymousSubstructMember(AnonymousSubstructMemberNameT<'s, 't>),
    Primitive(PrimitiveNameT<'s, 't>),
    PackageTopLevel(PackageTopLevelNameT<'s, 't>),
    Project(ProjectNameT<'s, 't>),
    Package(PackageNameT<'s, 't>),
    Rune(RuneNameT<'s, 't>),
    BuildingFunctionNameWithClosureds(BuildingFunctionNameWithClosuredsT<'s, 't>),
    ExternTemplate(ExternTemplateNameT<'s, 't>),
    Extern(ExternNameT<'s, 't>),
    ExternFunction(ExternFunctionNameValT<'s, 't, 'tmp>),
    Function(FunctionNameValT<'s, 't, 'tmp>),
    ForwarderFunction(ForwarderFunctionNameT<'s, 't>),
    FunctionBoundTemplate(FunctionBoundTemplateNameT<'s, 't>),
    FunctionBound(FunctionBoundNameValT<'s, 't, 'tmp>),
    PredictedFunctionTemplate(PredictedFunctionTemplateNameT<'s, 't>),
    PredictedFunction(PredictedFunctionNameValT<'s, 't, 'tmp>),
    FunctionTemplate(FunctionTemplateNameT<'s, 't>),
    LambdaCallFunctionTemplate(LambdaCallFunctionTemplateNameValT<'s, 't, 'tmp>),
    LambdaCallFunction(LambdaCallFunctionNameValT<'s, 't, 'tmp>),
    ForwarderFunctionTemplate(ForwarderFunctionTemplateNameT<'s, 't>),
    ConstructorTemplate(ConstructorTemplateNameT<'s, 't>),
    Self_(SelfNameT<'s, 't>),
    Arbitrary(ArbitraryNameT<'s, 't>),
    Struct(StructNameValT<'s, 't, 'tmp>),
    Interface(InterfaceNameValT<'s, 't, 'tmp>),
    LambdaCitizenTemplate(LambdaCitizenTemplateNameT<'s, 't>),
    LambdaCitizen(LambdaCitizenNameT<'s, 't>),
    StructTemplate(StructTemplateNameT<'s, 't>),
    InterfaceTemplate(InterfaceTemplateNameT<'s, 't>),
    AnonymousSubstructImplTemplate(AnonymousSubstructImplTemplateNameT<'s, 't>),
    AnonymousSubstructImpl(AnonymousSubstructImplNameValT<'s, 't, 'tmp>),
    AnonymousSubstructTemplate(AnonymousSubstructTemplateNameT<'s, 't>),
    AnonymousSubstructConstructorTemplate(AnonymousSubstructConstructorTemplateNameT<'s, 't>),
    AnonymousSubstructConstructor(AnonymousSubstructConstructorNameValT<'s, 't, 'tmp>),
    AnonymousSubstruct(AnonymousSubstructNameValT<'s, 't, 'tmp>),
    ResolvingEnv(ResolvingEnvNameT<'s, 't>),
    CallEnv(CallEnvNameT<'s, 't>),
}
/* Guardian: disable-all */

/// Interning transient (see @TFITCX)
pub struct INameValQuery<'a, 's, 't, 'tmp>(pub &'a INameValT<'s, 't, 'tmp>)
where 's: 't, 't: 'tmp;
/* Guardian: disable-all */

impl<'a, 's, 't, 'tmp> Hash for INameValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state); }
}

impl<'a, 's, 't, 'tmp> hashbrown::Equivalent<INameValT<'s, 't, 't>> for INameValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn equivalent(&self, key: &INameValT<'s, 't, 't>) -> bool {
        match (self.0, key) {
            // 15 transient variants: delegate to per-concrete Query wrapper.
            (Impl(a), Impl(b)) => ImplNameValQuery(a).equivalent(b),
            (ImplBound(a), ImplBound(b)) => ImplBoundNameValQuery(a).equivalent(b),
            (OverrideDispatcher(a), OverrideDispatcher(b)) => OverrideDispatcherNameValQuery(a).equivalent(b),
            (OverrideDispatcherCase(a), OverrideDispatcherCase(b)) => OverrideDispatcherCaseNameValQuery(a).equivalent(b),
            (ExternFunction(a), ExternFunction(b)) => ExternFunctionNameValQuery(a).equivalent(b),
            (Function(a), Function(b)) => FunctionNameValQuery(a).equivalent(b),
            (FunctionBound(a), FunctionBound(b)) => FunctionBoundNameValQuery(a).equivalent(b),
            (PredictedFunction(a), PredictedFunction(b)) => PredictedFunctionNameValQuery(a).equivalent(b),
            (LambdaCallFunctionTemplate(a), LambdaCallFunctionTemplate(b)) => LambdaCallFunctionTemplateNameValQuery(a).equivalent(b),
            (LambdaCallFunction(a), LambdaCallFunction(b)) => LambdaCallFunctionNameValQuery(a).equivalent(b),
            (Struct(a), Struct(b)) => StructNameValQuery(a).equivalent(b),
            (Interface(a), Interface(b)) => InterfaceNameValQuery(a).equivalent(b),
            (AnonymousSubstructImpl(a), AnonymousSubstructImpl(b)) => AnonymousSubstructImplNameValQuery(a).equivalent(b),
            (AnonymousSubstructConstructor(a), AnonymousSubstructConstructor(b)) => AnonymousSubstructConstructorNameValQuery(a).equivalent(b),
            (AnonymousSubstruct(a), AnonymousSubstruct(b)) => AnonymousSubstructNameValQuery(a).equivalent(b),
            // 57 simple variants: payload types match (no 'tmp), direct ==.
            (ExportTemplate(a), ExportTemplate(b)) => a == b,
            (Export(a), Export(b)) => a == b,
            (ImplTemplate(a), ImplTemplate(b)) => a == b,
            (ImplBoundTemplate(a), ImplBoundTemplate(b)) => a == b,
            (Let(a), Let(b)) => a == b,
            (ExportAs(a), ExportAs(b)) => a == b,
            (RawArray(a), RawArray(b)) => a == b,
            (ReachablePrototype(a), ReachablePrototype(b)) => a == b,
            (StaticSizedArrayTemplate(a), StaticSizedArrayTemplate(b)) => a == b,
            (StaticSizedArray(a), StaticSizedArray(b)) => a == b,
            (RuntimeSizedArrayTemplate(a), RuntimeSizedArrayTemplate(b)) => a == b,
            (RuntimeSizedArray(a), RuntimeSizedArray(b)) => a == b,
            (KindPlaceholderTemplate(a), KindPlaceholderTemplate(b)) => a == b,
            (KindPlaceholder(a), KindPlaceholder(b)) => a == b,
            (NonKindNonRegionPlaceholder(a), NonKindNonRegionPlaceholder(b)) => a == b,
            (OverrideDispatcherTemplate(a), OverrideDispatcherTemplate(b)) => a == b,
            (TypingPassBlockResultVar(a), TypingPassBlockResultVar(b)) => a == b,
            (TypingPassFunctionResultVar(a), TypingPassFunctionResultVar(b)) => a == b,
            (TypingPassTemporaryVar(a), TypingPassTemporaryVar(b)) => a == b,
            (TypingPassPatternMember(a), TypingPassPatternMember(b)) => a == b,
            (TypingIgnoredParam(a), TypingIgnoredParam(b)) => a == b,
            (TypingPassPatternDestructuree(a), TypingPassPatternDestructuree(b)) => a == b,
            (UnnamedLocal(a), UnnamedLocal(b)) => a == b,
            (ClosureParam(a), ClosureParam(b)) => a == b,
            (ConstructingMember(a), ConstructingMember(b)) => a == b,
            (WhileCondResult(a), WhileCondResult(b)) => a == b,
            (Iterable(a), Iterable(b)) => a == b,
            (Iterator(a), Iterator(b)) => a == b,
            (IterationOption(a), IterationOption(b)) => a == b,
            (MagicParam(a), MagicParam(b)) => a == b,
            (CodeVar(a), CodeVar(b)) => a == b,
            (AnonymousSubstructMember(a), AnonymousSubstructMember(b)) => a == b,
            (Primitive(a), Primitive(b)) => a == b,
            (PackageTopLevel(a), PackageTopLevel(b)) => a == b,
            (Project(a), Project(b)) => a == b,
            (Package(a), Package(b)) => a == b,
            (Rune(a), Rune(b)) => a == b,
            (BuildingFunctionNameWithClosureds(a), BuildingFunctionNameWithClosureds(b)) => a == b,
            (ExternTemplate(a), ExternTemplate(b)) => a == b,
            (Extern(a), Extern(b)) => a == b,
            (ForwarderFunction(a), ForwarderFunction(b)) => a == b,
            (FunctionBoundTemplate(a), FunctionBoundTemplate(b)) => a == b,
            (PredictedFunctionTemplate(a), PredictedFunctionTemplate(b)) => a == b,
            (FunctionTemplate(a), FunctionTemplate(b)) => a == b,
            (ForwarderFunctionTemplate(a), ForwarderFunctionTemplate(b)) => a == b,
            (ConstructorTemplate(a), ConstructorTemplate(b)) => a == b,
            (Self_(a), Self_(b)) => a == b,
            (Arbitrary(a), Arbitrary(b)) => a == b,
            (LambdaCitizenTemplate(a), LambdaCitizenTemplate(b)) => a == b,
            (LambdaCitizen(a), LambdaCitizen(b)) => a == b,
            (StructTemplate(a), StructTemplate(b)) => a == b,
            (InterfaceTemplate(a), InterfaceTemplate(b)) => a == b,
            (AnonymousSubstructImplTemplate(a), AnonymousSubstructImplTemplate(b)) => a == b,
            (AnonymousSubstructTemplate(a), AnonymousSubstructTemplate(b)) => a == b,
            (AnonymousSubstructConstructorTemplate(a), AnonymousSubstructConstructorTemplate(b)) => a == b,
            (ResolvingEnv(a), ResolvingEnv(b)) => a == b,
            (CallEnv(a), CallEnv(b)) => a == b,
            _ => false,
        }
    }
    /* Guardian: disable-all */
}
