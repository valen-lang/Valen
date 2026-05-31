use std::collections::HashMap;
use indexmap::IndexMap;
use std::marker::PhantomData;
use crate::higher_typing::ast::{ProgramA, StructA, InterfaceA, FunctionA};
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::postparsing::ast::{ICitizenAttributeS, LocationInDenizen, MacroCallS};
use crate::typing::citizen::struct_compiler::UncheckedDefiningConclusions;
use crate::typing::ast::citizens::{IStructMemberT, NormalStructMemberT, IMemberTypeT, ReferenceMemberTypeT};
use crate::typing::templata_compiler::IBoundArgumentsSource;
use crate::postparsing::names::{IImpreciseNameS, IRuneS};
use crate::scout_arena::ScoutArena;
use crate::typing::ast::expressions::{ReferenceExpressionTE, ConsecutorTE, VoidLiteralTE};
use crate::typing::ast::ast::{FunctionHeaderT, InterfaceEdgeBlueprintT, KindExportT, PrototypeT};
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::compilation::TypingPassOptions;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::{CompilerOutputs, DeferredActionT};
use crate::typing::infer_compiler::InferEnv;
use crate::typing::templata::templata::ImplDefinitionTemplataT;
use crate::typing::macros::macros::{OnStructDefinedMacro, OnInterfaceDefinedMacro, FunctionBodyMacro};
use crate::typing::env::environment::{get_imprecise_name, make_top_level_environment, GlobalEnvironmentT, IEnvironmentT, IInDenizenEnvironmentT, PackageEnvironmentT, TemplatasStoreT, TemplatasStoreBuilder};
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::names::names::{
    IdT, IdValT, INameT, IFunctionTemplateNameT, IInstantiationNameT, ITemplateNameT,
    IStructTemplateNameT, IInterfaceTemplateNameT, IImplTemplateNameT, PackageTopLevelNameT, PrimitiveNameT,
};
use crate::typing::templata::templata::{
    CoordTemplataT, FunctionTemplataT, ITemplataT, InterfaceDefinitionTemplataT, KindTemplataT, MutabilityTemplataT, PlaceholderTemplataT,
    PrototypeTemplataT, RuntimeSizedArrayTemplateTemplataT, StaticSizedArrayTemplateTemplataT, StructDefinitionTemplataT,
};
use crate::typing::types::types::CoordT;
use crate::typing::types::types::{BoolT, FloatT, IntT, KindT, MutabilityT, NeverT, StrT, VoidT};
use crate::typing::typing_interner::TypingInterner;
use crate::typing::types::types::{IRegionT, RegionT};
use crate::typing::function::function_compiler::StampFunctionSuccess;
use crate::typing::overload_resolver::FindFunctionFailure;
use crate::utils::code_hierarchy::{FileCoordinateMap, PackageCoordinate, PackageCoordinateMap};
use crate::utils::range::RangeS;
use crate::typing::types::types::ISubKindTT;
use crate::typing::types::types::ICitizenTT;
use crate::typing::names::names::{PredictedFunctionTemplateNameT, PredictedFunctionNameValT};
use crate::typing::ast::ast::PrototypeValT;
use crate::typing::names::names::FunctionBoundTemplateNameT;
use crate::typing::names::names::FunctionBoundNameValT;
use crate::typing::names::names::{ImplBoundTemplateNameT, ImplBoundNameValT};
use crate::typing::templata::templata::IsaTemplataT;
use crate::typing::names::names::ExportTemplateNameT;
use crate::typing::names::names::ExportNameT;
use crate::typing::env::environment::ExportEnvironmentT;
use crate::typing::citizen::struct_compiler::IResolveOutcome;
use crate::postparsing::names::IStructDeclarationNameS;
use crate::postparsing::ast::IFunctionAttributeS;
use crate::typing::names::names::ExternTemplateNameT;
use crate::typing::names::names::ExternNameT;
use crate::typing::names::names::ExternFunctionNameValT;
use crate::typing::env::environment::ExternEnvironmentT;
use crate::typing::function::function_compiler::IResolveFunctionResult;
use crate::postparsing::names::IFunctionDeclarationNameS;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::parsing::ast::ast::IMacroInclusionP;
use crate::typing::types::types::StaticSizedArrayTT;
use crate::typing::types::types::RuntimeSizedArrayTT;
use crate::typing::types::types::StructTT;
use crate::postparsing::names::IImpreciseNameValS;
use crate::postparsing::names::CodeNameS;
use crate::postparsing::rules::rules::IRulexSR;
use crate::typing::env::function_environment_t::FunctionEnvironmentT;
use crate::typing::ast::ast::LocationInFunctionEnvironmentT;
use crate::typing::ast::ast::ParameterT;

/*
package dev.vale.typing

import dev.vale._
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.{CallMacroP, DontCallMacroP, UseP}
import dev.vale.postparsing.patterns.AtomSP
import dev.vale.postparsing.rules.{IRulexSR, RuneUsage}
import dev.vale.postparsing._
import dev.vale.typing.OverloadResolver.FindFunctionFailure
import dev.vale.typing.citizen._
import dev.vale.typing.expression.{ExpressionCompiler, IExpressionCompilerDelegate}
import dev.vale.typing.function._
import dev.vale.typing.infer.IInfererDelegate
import dev.vale.typing.types._
import dev.vale.highertyping._
import OverloadResolver.FindFunctionFailure
import dev.vale.typing.function._
import dev.vale
import dev.vale.highertyping.{ExportAsA, FunctionA, InterfaceA, ProgramA, StructA}
import dev.vale.typing.Compiler.isPrimitive
import dev.vale.typing.ast.{ConsecutorTE, EdgeT, ExternT, FunctionHeaderT, LocationInFunctionEnvironmentT, ParameterT, PrototypeT, ReferenceExpressionTE, VoidLiteralTE}
import dev.vale.typing.env._
import dev.vale.typing.macros.{AbstractBodyMacro, AnonymousInterfaceMacro, AsSubtypeMacro, FunctorHelper, IOnImplDefinedMacro, IOnInterfaceDefinedMacro, IOnStructDefinedMacro, LockWeakMacro, SameInstanceMacro, StructConstructorMacro}
import dev.vale.typing.macros.citizen._
import dev.vale.typing.macros.rsa.{RSADropIntoMacro, RSAImmutableNewMacro, RSALenMacro, RSAMutableCapacityMacro, RSAMutableNewMacro, RSAMutablePopMacro, RSAMutablePushMacro}
import dev.vale.typing.macros.ssa.{SSADropIntoMacro, SSALenMacro}
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.ast._
import dev.vale.typing.citizen.ImplCompiler
import dev.vale.typing.env._
import dev.vale.typing.expression.LocalHelper
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.typing.function.FunctionCompiler
import dev.vale.typing.macros.citizen.StructDropMacro
import dev.vale.typing.macros.ssa.SSALenMacro

import scala.collection.immutable.{List, ListMap, Map, Set}
import scala.collection.mutable
import scala.util.control.Breaks._

*/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IFunctionGenerator {
    StructConstructor,
    StructDrop,
    InterfaceDrop,
    RsaDropInto,
    RsaImmutableNew,
    RsaLen,
    RsaMutableCapacity,
    RsaMutableNew,
    RsaMutablePop,
    RsaMutablePush,
    SsaDropInto,
    SsaLen,
    LockWeak,
    SameInstance,
    AsSubtype,
    AbstractBody,
}
/*
trait IFunctionGenerator {
*/
/*
  def generate(
    // These serve as the API that a function generator can use.
    // TODO: Give a trait with a reduced API.
    // Maybe this functionCompilerCore can be a lambda we can use to finalize and add &This* function.

    functionCompilerCore: FunctionCompilerCore,
    structCompiler: StructCompiler,
    destructorCompiler: DestructorCompiler,
    arrayCompiler: ArrayCompiler,
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    life: LocationInFunctionEnvironmentT,
    callRange: List[RangeS],
    // We might be able to move these all into the function environment... maybe....
    originFunction: Option[FunctionA],
    paramCoords: Vector[ParameterT],
    maybeRetCoord: Option[CoordT]):
  (FunctionHeaderT)
}

*/

/*
object DefaultPrintyThing {
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn print(
        &self,
        // TODO: Slab 14 — Scala uses a by-name parameter here; pick impl Display or &str as the Rust equivalent when porting the body.
        x: (),
    ) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
    /*
      def print(x: => Object) = {
        println("###: " + x)
      }
    }



    */
}
pub struct Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub scout_arena: &'ctx ScoutArena<'s>,
    pub typing_interner: &'ctx TypingInterner<'s, 't>,
    pub keywords: &'ctx Keywords<'s>,
    pub opts: &'ctx TypingPassOptions<'s>,
}
/*
class Compiler(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords) {
*/

// (no direct Scala counterpart — derived from `class Compiler(opts, interner, keywords)` in the Scala block above)
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn new(
        scout_arena: &'ctx ScoutArena<'s>,
        typing_interner: &'ctx TypingInterner<'s, 't>,
        keywords: &'ctx Keywords<'s>,
        opts: &'ctx TypingPassOptions<'s>,
    ) -> Self {
        Compiler { scout_arena, typing_interner, keywords, opts }
    }
    /*
  val debugOut = opts.debugOut
  val globalOptions = opts.globalOptions

  val nameTranslator = new NameTranslator(interner)

  val templataCompiler =
    new TemplataCompiler(
      interner,
      opts,
      nameTranslator,
      new ITemplataCompilerDelegate {
        override def isParent(
          coutputs: CompilerOutputs,
          callingEnv: IInDenizenEnvironmentT,
          parentRanges: List[RangeS],
          callLocation: LocationInDenizen,
          subKindTT: ISubKindTT,
          superKindTT: ISuperKindTT):
        IsParentResult = {
          implCompiler.isParent(coutputs, callingEnv, parentRanges, callLocation, subKindTT, superKindTT)
        }

        override def resolveStruct(
          coutputs: CompilerOutputs,
          callingEnv: IInDenizenEnvironmentT,
          callRange: List[RangeS],
          callLocation: LocationInDenizen,
          structTemplata: StructDefinitionTemplataT,
          uncoercedTemplateArgs: Vector[ITemplataT[ITemplataType]]):
        IResolveOutcome[StructTT] = {
          structCompiler.resolveStruct(
            coutputs, callingEnv, callRange, callLocation, structTemplata, uncoercedTemplateArgs)
        }

        override def resolveInterface(
            coutputs: CompilerOutputs,
            callingEnv: IInDenizenEnvironmentT, // See CSSNCE
            callRange: List[RangeS],
            callLocation: LocationInDenizen,
            interfaceTemplata: InterfaceDefinitionTemplataT,
            uncoercedTemplateArgs: Vector[ITemplataT[ITemplataType]]):
        IResolveOutcome[InterfaceTT] = {
          structCompiler.resolveInterface(
            coutputs, callingEnv, callRange, callLocation, interfaceTemplata, uncoercedTemplateArgs)
        }

      })
  val inferCompiler: InferCompiler =
    new InferCompiler(
      opts,
      interner,
      keywords,
      nameTranslator,
      new IInfererDelegate {
*/
    pub fn get_placeholders_in_id(&self, accum: &mut Vec<IdT<'s, 't>>, id: IdT<'s, 't>) {
        match id.local_name {
            INameT::KindPlaceholder(_) => accum.push(id),
            INameT::KindPlaceholderTemplate(_) => accum.push(id),
            _ => {}
        }
    }
/*
        def getPlaceholdersInId(accum: Accumulator[IdT[INameT]], id: IdT[INameT]): Unit = {
          id.localName match {
            case KindPlaceholderNameT(_) => accum.add(id)
            case KindPlaceholderTemplateNameT(_, _) => accum.add(id)
            case _ =>
          }
        }
*/
    pub fn get_placeholders_in_templata(&self, accum: &mut Vec<IdT<'s, 't>>, templata: ITemplataT<'s, 't>) {
        match templata {
            ITemplataT::Kind(KindTemplataT { kind }) => self.get_placeholders_in_kind(accum, *kind),
            ITemplataT::Coord(CoordTemplataT { coord: CoordT { kind, .. } }) => self.get_placeholders_in_kind(accum, *kind),
            ITemplataT::Placeholder(PlaceholderTemplataT { id, .. }) => accum.push(*id),
            ITemplataT::Integer(_) => {}
            ITemplataT::Boolean(_) => {}
            ITemplataT::String(_) => {}
            ITemplataT::RuntimeSizedArrayTemplate(_) => {}
            ITemplataT::StaticSizedArrayTemplate(_) => {}
            ITemplataT::Variability(_) => {}
            ITemplataT::Ownership(_) => {}
            ITemplataT::Mutability(_) => {}
            ITemplataT::InterfaceDefinition(_) => {}
            ITemplataT::StructDefinition(_) => {}
            ITemplataT::ImplDefinition(_) => {}
            ITemplataT::CoordList(_) => { panic!("implement: get_placeholders_in_templata CoordList"); }
            ITemplataT::Prototype(_) => { panic!("implement: get_placeholders_in_templata Prototype"); }
            ITemplataT::Isa(_) => { panic!("implement: get_placeholders_in_templata Isa"); }
            _ => { panic!("implement: get_placeholders_in_templata other"); }
        }
    }
/*
        def getPlaceholdersInTemplata(accum: Accumulator[IdT[INameT]], templata: ITemplataT[ITemplataType]): Unit = {
          templata match {
            case KindTemplataT(kind) => getPlaceholdersInKind(accum, kind)
            case CoordTemplataT(CoordT(_, _, kind)) => getPlaceholdersInKind(accum, kind)
            case CoordTemplataT(CoordT(_, _, _)) =>
            case PlaceholderTemplataT(id, _) => accum.add(id)
            case IntegerTemplataT(_) =>
            case BooleanTemplataT(_) =>
            case StringTemplataT(_) =>
            case RuntimeSizedArrayTemplateTemplataT() =>
            case StaticSizedArrayTemplateTemplataT() =>
            case VariabilityTemplataT(_) =>
            case OwnershipTemplataT(_) =>
            case MutabilityTemplataT(_) =>
            case InterfaceDefinitionTemplataT(_,_) =>
            case StructDefinitionTemplataT(_,_) =>
            case ImplDefinitionTemplataT(_,_) =>
            case CoordListTemplataT(coords) => coords.foreach(c => getPlaceholdersInKind(accum, c.kind))
            case PrototypeTemplataT(prototype) => {
              getPlaceholdersInId(accum, prototype.id)
              prototype.paramTypes.foreach(c => getPlaceholdersInKind(accum, c.kind))
              getPlaceholdersInKind(accum, prototype.returnType.kind)
            }
            case IsaTemplataT(_, _, subKind, superKind) => {
              getPlaceholdersInKind(accum, subKind)
              getPlaceholdersInKind(accum, superKind)
            }
            case other => vimpl(other)
          }
        }
*/
    pub fn get_placeholders_in_kind(&self, accum: &mut Vec<IdT<'s, 't>>, kind: KindT<'s, 't>) {
        match kind {
            KindT::Int(_) => {}
            KindT::Bool(_) => {}
            KindT::Float(_) => {}
            KindT::Void(_) => {}
            KindT::Never(_) => {}
            KindT::Str(_) => {}
            KindT::RuntimeSizedArray(rsa) => {
                self.get_placeholders_in_templata(accum, rsa.mutability());
                self.get_placeholders_in_kind(accum, rsa.element_type().kind);
            }
            KindT::StaticSizedArray(ssa) => {
                self.get_placeholders_in_templata(accum, ssa.size());
                self.get_placeholders_in_templata(accum, ssa.mutability());
                self.get_placeholders_in_templata(accum, ssa.variability());
                self.get_placeholders_in_kind(accum, ssa.element_type().kind);
            }
            // Rust adaptation (SPDMX-B): IdT.local_name is type-erased INameT in Rust; narrow via TryFrom<INameT> for IInstantiationNameT to call the dispatch method (per AASSNCMCX-session precedent in templata_compiler.rs).
            KindT::Struct(s) => {
                let inst_name = IInstantiationNameT::try_from(s.id.local_name).expect(
                    "StructTT id local_name must be an IInstantiationNameT");
                for arg in inst_name.template_args() {
                    self.get_placeholders_in_templata(accum, *arg);
                }
            }
            // Rust adaptation (SPDMX-B): IdT.local_name is type-erased INameT in Rust; narrow via TryFrom<INameT> for IInstantiationNameT to call the dispatch method (per AASSNCMCX-session precedent in templata_compiler.rs).
            KindT::Interface(i) => {
                let inst_name = IInstantiationNameT::try_from(i.id.local_name).expect(
                    "InterfaceTT id local_name must be an IInstantiationNameT");
                for arg in inst_name.template_args() {
                    self.get_placeholders_in_templata(accum, *arg);
                }
            }
            KindT::KindPlaceholder(p) => accum.push(p.id),
            KindT::OverloadSet(_) => {}
        }
    }
/*
        def getPlaceholdersInKind(accum: Accumulator[IdT[INameT]], kind: KindT): Unit = {
          kind match {
            case IntT(_) =>
            case BoolT() =>
            case FloatT() =>
            case VoidT() =>
            case NeverT(_) =>
            case StrT() =>
            case contentsRuntimeSizedArrayTT(mutability, elementType, selfRegion) => {
              getPlaceholdersInTemplata(accum, mutability)
              getPlaceholdersInKind(accum, elementType.kind)
            }
            case contentsStaticSizedArrayTT(size, mutability, variability, elementType, selfRegion) => {
              getPlaceholdersInTemplata(accum, size)
              getPlaceholdersInTemplata(accum, mutability)
              getPlaceholdersInTemplata(accum, variability)
              getPlaceholdersInKind(accum, elementType.kind)
            }
            case StructTT(IdT(_,_,name)) => name.templateArgs.foreach(getPlaceholdersInTemplata(accum, _))
            case InterfaceTT(IdT(_,_,name)) => name.templateArgs.foreach(getPlaceholdersInTemplata(accum, _))
            case KindPlaceholderT(id) => accum.add(id)
            case OverloadSetT(env, name) =>
            case other => vimpl(other)
          }
        }
*/
    pub fn sanity_check_conclusion(&self, envs: &InferEnv<'s, 't>, _state: &mut CompilerOutputs<'s, 't>, _rune: IRuneS<'s>, templata: ITemplataT<'s, 't>) {
        let mut accum: Vec<IdT<'s, 't>> = Vec::new();
        self.get_placeholders_in_templata(&mut accum, templata);

        if !accum.is_empty() {
            let root_denizen_env = envs.original_calling_env.root_compiling_denizen_env();
            let root_id = root_denizen_env.id();
            // Rust adaptation (SPDMX-B): Scala constructs IdT freely as a case class;
            // Rust must intern it via typing_interner.
            let original_calling_env_template_name: IdT<'s, 't> =
                match ITemplateNameT::try_from(root_id.local_name) {
                    Ok(_x) => root_id,
                    Err(_) => {
                        match IInstantiationNameT::try_from(root_id.local_name) {
                            Ok(x) => {
                                *self.typing_interner.intern_id(IdValT {
                                    package_coord: root_id.package_coord,
                                    init_steps: root_id.init_steps,
                                    local_name: INameT::from(x.template()),
                                })
                            }
                            Err(_) => panic!("sanityCheckConclusion: unexpected root id local_name: {:?}", root_id.local_name),
                        }
                    }
                };
            let template_steps = original_calling_env_template_name.steps();
            for placeholder_name in &accum {
                let placeholder_steps = placeholder_name.steps();
                assert!(
                    placeholder_steps.starts_with(&template_steps),
                    "Placeholder {:?} steps don't start with template steps",
                    placeholder_name
                );
            }
        }
    }
/*
        override def sanityCheckConclusion(env: InferEnv, state: CompilerOutputs, rune: IRuneS, templata: ITemplataT[ITemplataType]): Unit = {
          val accum = new Accumulator[IdT[INameT]]()
          getPlaceholdersInTemplata(accum, templata)

          if (accum.elementsReversed.nonEmpty) {
            val rootDenizenEnv = env.originalCallingEnv.rootCompilingDenizenEnv
            val originalCallingEnvTemplateName =
              rootDenizenEnv.id match {
                case IdT(packageCoord, initSteps, x: ITemplateNameT) => {
                  IdT(packageCoord, initSteps, x)
                }
                // When we compile a generic function, we populate some placeholders for its template
                // args. Then, we start compiling its body expressions. At that point, we're in an
                // environment that has a FullName with placeholders in it.
                // That's what we'll see in this case.
                case IdT(packageCoord, initSteps, x: IInstantiationNameT) => {
                  IdT(packageCoord, initSteps, x.template)
                }
                case other => vfail(other)
              }
            accum.elementsReversed.foreach(placeholderName => {
              // There should only ever be placeholders from the original calling environment, we should
              // *never* mix placeholders from two environments.
              // If this assert trips, that means we're not correctly phrasing everything in terms of
              // placeholders from this top level denizen.
              // See OWPFRD.
              vassert(placeholderName.steps.startsWith(originalCallingEnvTemplateName.steps))
            })
          }
        }

        override def lookupTemplata(
          envs: InferEnv,
          coutputs: CompilerOutputs,
          range: List[RangeS],
          name: INameT):
        ITemplataT[ITemplataType] = {
          templataCompiler.lookupTemplata(envs.selfEnv, coutputs, range, name)
        }

*/
    // mig: fn is_descendant_kind
    // Rust adaptation: collides with Compiler::is_descendant lifted from
    // ImplCompiler.scala (which Rust flattened onto Compiler); appended `_kind`
    // suffix to disambiguate this delegate-class isDescendant from
    // ImplCompiler's. Scala uses class-level disambiguation (Compiler's
    // anonymous CompilerSolverDelegate vs ImplCompiler) that Rust lacks.
    pub fn is_descendant_kind(
        &self,
        _envs: &InferEnv<'s, 't>,
        _coutputs: &mut CompilerOutputs<'s, 't>,
        kind: KindT<'s, 't>,
    ) -> bool {
        match kind {
            KindT::KindPlaceholder(kp) => {
                self.is_descendant(_coutputs, _envs.parent_ranges, _envs.call_location, _envs.original_calling_env,
                    ISubKindTT::KindPlaceholder(kp))
            }
            KindT::RuntimeSizedArray(_) => false,
            KindT::OverloadSet(_) => false,
            KindT::Never(_) => true,
            KindT::StaticSizedArray(_) => false,
            KindT::Struct(s) => {
                self.is_descendant(_coutputs, _envs.parent_ranges, _envs.call_location, _envs.original_calling_env,
                    ISubKindTT::Struct(s))
            }
            KindT::Interface(i) => {
                self.is_descendant(_coutputs, _envs.parent_ranges, _envs.call_location, _envs.original_calling_env,
                    ISubKindTT::Interface(i))
            }
            KindT::Int(_) | KindT::Bool(_) | KindT::Float(_) | KindT::Str(_) | KindT::Void(_) => false,
        }
    }
/*
        override def isDescendant(
          envs: InferEnv,
          coutputs: CompilerOutputs,
          kind: KindT):
        Boolean = {
          kind match {
            case p @ KindPlaceholderT(_) => implCompiler.isDescendant(coutputs, envs.parentRanges, envs.callLocation, envs.originalCallingEnv, p)
            case contentsRuntimeSizedArrayTT(_, _, _) => false
            case OverloadSetT(_, _) => false
            case NeverT(fromBreak) => true
            case contentsStaticSizedArrayTT(_, _, _, _, _) => false
            case s @ StructTT(_) => implCompiler.isDescendant(coutputs, envs.parentRanges, envs.callLocation, envs.originalCallingEnv, s)
            case i @ InterfaceTT(_) => implCompiler.isDescendant(coutputs, envs.parentRanges, envs.callLocation, envs.originalCallingEnv, i)
            case IntT(_) | BoolT() | FloatT() | StrT() | VoidT() => false
          }
        }
*/
    // mig: fn is_ancestor_kind
    // Rust adaptation: see is_descendant_kind above for the `_kind` suffix
    // rationale (ImplCompiler/Compiler flattening collision).
    pub fn is_ancestor_kind(
        &self,
        _envs: &InferEnv<'s, 't>,
        _coutputs: &mut CompilerOutputs<'s, 't>,
        kind: KindT<'s, 't>,
    ) -> bool {
        match kind {
            KindT::Interface(_) => true,
            _ => false,
        }
    }
/*
        override def isAncestor(
          envs: InferEnv,
          coutputs: CompilerOutputs,
          kind: KindT):
        Boolean = {
          kind match {
            case InterfaceTT(_) => true
            case _ => false
          }
        }

        override def isParent(
          env: InferEnv,
          coutputs: CompilerOutputs,
          parentRanges: List[RangeS],
          subKindTT: ISubKindTT,
          superKindTT: ISuperKindTT,
          includeSelf: Boolean):
        Option[ITemplataT[ImplTemplataType]] = {
          implCompiler.isParent(coutputs, env.originalCallingEnv, parentRanges, env.callLocation, subKindTT, superKindTT) match {
            case IsParent(implTemplata, _, _) => Some(implTemplata)
            case IsntParent(candidates) => None
          }
        }

        def coerceToCoord(
          envs: InferEnv,
          state: CompilerOutputs,
          range: List[RangeS],
          templata: ITemplataT[ITemplataType],
          region: RegionT):
        ITemplataT[ITemplataType] = {
          templataCompiler.coerceToCoord(state, envs.originalCallingEnv, range, templata, region)
        }
*/
    // mig: fn lookup_templata_imprecise
    pub fn lookup_templata_imprecise(
        &self,
        envs: InferEnv<'s, 't>,
        state: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        name: IImpreciseNameS<'s>,
    ) -> Option<ITemplataT<'s, 't>> {
        self.lookup_templata_by_rune(envs.self_env, state, range, name)
    }
    /*
        override def lookupTemplataImprecise(envs: InferEnv, state: CompilerOutputs, range: List[RangeS], name: IImpreciseNameS): Option[ITemplataT[ITemplataType]] = {
          templataCompiler.lookupTemplata(envs.selfEnv, state, range, name)
        }
    */
    /*

        override def getMutability(state: CompilerOutputs, kind: KindT): ITemplataT[MutabilityTemplataType] = {
            Compiler.getMutability(state, kind)
        }
*/
    // mig: fn predict_static_sized_array_kind
    // Rust adaptation: lifted from Compiler.scala's anonymous IInfererDelegate
    // (which Rust flattened onto Compiler).
    pub fn predict_static_sized_array_kind(
        &self,
        _envs: InferEnv<'s, 't>,
        _state: &mut CompilerOutputs<'s, 't>,
        mutability: ITemplataT<'s, 't>,
        variability: ITemplataT<'s, 't>,
        size: ITemplataT<'s, 't>,
        element: CoordT<'s, 't>,
        region: RegionT,
    ) -> StaticSizedArrayTT<'s, 't> {
        self.resolve_static_sized_array(mutability, variability, size, element, region)
    }
    /*
        override def predictStaticSizedArrayKind(
          envs: InferEnv,
          state: CompilerOutputs,
          mutability: ITemplataT[MutabilityTemplataType],
          variability: ITemplataT[VariabilityTemplataType],
          size: ITemplataT[IntegerTemplataType],
          element: CoordT,
          region: RegionT):
        StaticSizedArrayTT = {
          arrayCompiler.resolveStaticSizedArray(mutability, variability, size, element, region)
        }

*/
    // mig: fn predict_runtime_sized_array_kind
    // Rust adaptation: lifted from Compiler.scala's anonymous IInfererDelegate
    // (which Rust flattened onto Compiler).
    pub fn predict_runtime_sized_array_kind(
        &self,
        _envs: InferEnv<'s, 't>,
        _state: &mut CompilerOutputs<'s, 't>,
        element: CoordT<'s, 't>,
        array_mutability: ITemplataT<'s, 't>,
        region: RegionT,
    ) -> RuntimeSizedArrayTT<'s, 't> {
        self.resolve_runtime_sized_array(element, array_mutability, region)
    }
    /*
        override def predictRuntimeSizedArrayKind(
          envs: InferEnv,
          state: CompilerOutputs,
          element: CoordT,
          arrayMutability: ITemplataT[MutabilityTemplataType],
          region: RegionT):
        RuntimeSizedArrayTT = {
            arrayCompiler.resolveRuntimeSizedArray(element, arrayMutability, region)
        }

        override def predictInterface(
          env: InferEnv,
          state: CompilerOutputs,
          templata: InterfaceDefinitionTemplataT,
          templateArgs: Vector[ITemplataT[ITemplataType]]):
        (KindT) = {
            structCompiler.predictInterface(
              state, env.originalCallingEnv, env.parentRanges, env.callLocation, templata, templateArgs)
        }

        override def predictStruct(
          env: InferEnv,
          state: CompilerOutputs,
          templata: StructDefinitionTemplataT,
          templateArgs: Vector[ITemplataT[ITemplataType]]):
        (KindT) = {
          structCompiler.predictStruct(
            state, env.originalCallingEnv, env.parentRanges, env.callLocation, templata, templateArgs)
        }

*/
    // mig: fn kind_is_from_template
    // Rust adaptation: lifted from Compiler.scala's anonymous IInfererDelegate
    // (which Rust flattened onto Compiler).
    pub fn kind_is_from_template(
        &self,
        _coutputs: &mut CompilerOutputs<'s, 't>,
        actual_citizen_ref: KindT<'s, 't>,
        expected_citizen_templata: ITemplataT<'s, 't>,
    ) -> bool {
        match actual_citizen_ref {
            KindT::RuntimeSizedArray(_) => matches!(expected_citizen_templata, ITemplataT::RuntimeSizedArrayTemplate(_)),
            KindT::StaticSizedArray(_) => matches!(expected_citizen_templata, ITemplataT::StaticSizedArrayTemplate(_)),
            other => {
                match ICitizenTT::try_from(other) {
                    Ok(s) => self.citizen_is_from_template(s, expected_citizen_templata),
                    Err(_) => false,
                }
            }
        }
    }
    /*
        override def kindIsFromTemplate(
          coutputs: CompilerOutputs,
          actualCitizenRef: KindT,
          expectedCitizenTemplata: ITemplataT[ITemplataType]):
        Boolean = {
          actualCitizenRef match {
            case s : ICitizenTT => templataCompiler.citizenIsFromTemplate(s, expectedCitizenTemplata)
            case contentsRuntimeSizedArrayTT(_, _, _) => (expectedCitizenTemplata == RuntimeSizedArrayTemplateTemplataT())
            case contentsStaticSizedArrayTT(_, _, _, _, _) => (expectedCitizenTemplata == StaticSizedArrayTemplateTemplataT())
            case _ => false
          }
        }

*/
    // mig: fn get_ancestors
    // Rust adaptation: lifted from Compiler.scala's anonymous IInfererDelegate
    // (which Rust flattened onto Compiler).
    pub fn get_ancestors(
        &self,
        envs: InferEnv<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        descendant: KindT<'s, 't>,
        include_self: bool,
    ) -> std::collections::HashSet<KindT<'s, 't>> {
        let mut result: std::collections::HashSet<KindT<'s, 't>> = std::collections::HashSet::new();
        if include_self {
            result.insert(descendant);
        }
        match ISubKindTT::try_from(descendant) {
            Ok(s) => {
                for parent in self.get_parents(coutputs, envs.parent_ranges, envs.call_location, envs.original_calling_env, s) {
                    result.insert(KindT::from(parent));
                }
            }
            Err(_) => {}
        }
        result
    }
    /*
        override def getAncestors(
          envs: InferEnv,
          coutputs: CompilerOutputs,
          descendant: KindT,
          includeSelf: Boolean):
        Set[KindT] = {
            (if (includeSelf) {
              Set[KindT](descendant)
            } else {
              Set[KindT]()
            }) ++
              (descendant match {
                case s : ISubKindTT => implCompiler.getParents(coutputs, envs.parentRanges, envs.callLocation, envs.originalCallingEnv, s)
                case _ => Vector[KindT]()
              })
        }

*/
    // mig: fn struct_is_closure
    // Rust adaptation: lifted from Compiler.scala's anonymous IInfererDelegate
    // (which Rust flattened onto Compiler).
    pub fn struct_is_closure(
        &self,
        _state: &mut CompilerOutputs<'s, 't>,
        _struct_tt: StructTT<'s, 't>,
    ) -> bool {
        panic!("Unimplemented: struct_is_closure");
    }
    /*
        override def structIsClosure(state: CompilerOutputs, structTT: StructTT): Boolean = {
            val structDef = state.lookupStruct(structTT.id)
            structDef.isClosure
        }
*/
    // mig: fn predict_function
    // Rust adaptation: lifted from Compiler.scala's anonymous IInfererDelegate
    // (which Rust flattened onto Compiler).
    pub fn predict_function(
        &self,
        envs: InferEnv<'s, 't>,
        _state: &mut CompilerOutputs<'s, 't>,
        _function_range: RangeS<'s>,
        name: StrI<'s>,
        param_coords: &'t [CoordT<'s, 't>],
        return_coord: CoordT<'s, 't>,
    ) -> PrototypeTemplataT<'s, 't> {
        let tmpl = self.typing_interner.intern_predicted_function_template_name(PredictedFunctionTemplateNameT { human_name: name, _phantom: std::marker::PhantomData });
        let pred_name = self.typing_interner.intern_predicted_function_name(PredictedFunctionNameValT { template: tmpl, template_args: &[], parameters: param_coords });
        let id = envs.original_calling_env.denizen_id().add_step(self.typing_interner, INameT::PredictedFunction(pred_name));
        let prototype = self.typing_interner.intern_prototype(PrototypeValT { id: IdValT { package_coord: id.package_coord, init_steps: id.init_steps, local_name: id.local_name }, return_type: return_coord });
        PrototypeTemplataT { prototype }
    }
    /*
        def predictFunction(
          envs: InferEnv,
          state: CompilerOutputs,
          functionRange: RangeS,
          name: StrI,
          paramCoords: Vector[CoordT],
          returnCoord: CoordT):
        PrototypeTemplataT[IFunctionNameT] = {
          PrototypeTemplataT(
            PrototypeT(
              envs.originalCallingEnv.id.addStep(
                interner.intern(
                  PredictedFunctionNameT(
                    interner.intern(PredictedFunctionTemplateNameT(name)),
                    Vector(),
                    paramCoords))),
              returnCoord))
        }
        // Per @BRRZ, this is the real overload lookup invoked from inside the ResolveSR
        // handler when the return rune isn't yet known. Mirrors the outer delegate's
        // resolveFunction at line 455-477 below, but takes InferEnv so the solver-side
        // delegate has a uniform shape with predictFunction/assemblePrototype.
        override def resolveFunction(
            envs: InferEnv,
            state: CompilerOutputs,
            range: List[RangeS],
            name: StrI,
            paramCoords: Vector[CoordT]):
        Result[StampFunctionSuccess, FindFunctionFailure] = {
          overloadResolver.findFunction(
            envs.originalCallingEnv,
            state,
            range,
            envs.callLocation,
            interner.intern(CodeNameS(interner.intern(name))),
            Vector.empty,
            Vector.empty,
            Vector.empty,
            envs.contextRegion,
            paramCoords,
            Vector.empty,
            true)
        }
*/
    // mig: fn assemble_prototype
    // Rust adaptation: lifted from Compiler.scala's anonymous IInfererDelegate
    // (which Rust flattened onto Compiler).
    pub fn assemble_prototype(
        &self,
        envs: InferEnv<'s, 't>,
        state: &mut CompilerOutputs<'s, 't>,
        _range: RangeS<'s>,
        name: StrI<'s>,
        coords: &'t [CoordT<'s, 't>],
        return_type: CoordT<'s, 't>,
    ) -> &'t PrototypeT<'s, 't> {
        let tmpl = self.typing_interner.intern_function_bound_template_name(FunctionBoundTemplateNameT { human_name: name, _phantom: std::marker::PhantomData });
        let bound_name = self.typing_interner.intern_function_bound_name(FunctionBoundNameValT { template: tmpl, template_args: &[], parameters: coords });
        let id = envs.original_calling_env.denizen_id().add_step(self.typing_interner, INameT::FunctionBound(bound_name));
        let result = self.typing_interner.intern_prototype(PrototypeValT { id: IdValT { package_coord: id.package_coord, init_steps: id.init_steps, local_name: id.local_name }, return_type });
        // This is a function bound, and there's no such thing as a function bound with function bounds.
        let empty_bounds = self.typing_interner.alloc(InstantiationBoundArgumentsT {
            rune_to_bound_prototype: self.typing_interner.alloc_index_map_from_iter(std::iter::empty()),
            rune_to_citizen_rune_to_reachable_prototype: self.typing_interner.alloc_index_map_from_iter(std::iter::empty()),
            rune_to_bound_impl: self.typing_interner.alloc_index_map_from_iter(std::iter::empty()),
        });
        state.add_instantiation_bounds(self.opts.global_options.sanity_check, self.typing_interner, envs.original_calling_env.denizen_template_id(), result.id, empty_bounds);
        result
    }
    /*
        override def assemblePrototype(
            envs: InferEnv,
          state: CompilerOutputs,
            range: RangeS,
            name: StrI,
            coords: Vector[CoordT],
            returnType: CoordT):
        PrototypeT[IFunctionNameT] = {
          val result =
            PrototypeT(
              envs.originalCallingEnv.id.addStep(
                interner.intern(FunctionBoundNameT(
                  interner.intern(FunctionBoundTemplateNameT(name)), Vector(), coords))),
              returnType)

          // This is a function bound, and there's no such thing as a function bound with function bounds.
          state.addInstantiationBounds(
            opts.globalOptions.sanityCheck,
            interner,
            envs.originalCallingEnv.denizenTemplateId,
            result.id,
            InstantiationBoundArgumentsT(
              scala.collection.immutable.HashMap(),
              scala.collection.immutable.HashMap(),
              scala.collection.immutable.HashMap()))

          result
        }
*/
    // mig: fn assemble_impl
    // Rust adaptation: lifted from Compiler.scala's anonymous IInfererDelegate
    // (which Rust flattened onto Compiler).
    pub fn assemble_impl(
        &self,
        env: InferEnv<'s, 't>,
        range: RangeS<'s>,
        sub_kind: KindT<'s, 't>,
        super_kind: KindT<'s, 't>,
    ) -> IsaTemplataT<'s, 't> {
        let tmpl = self.typing_interner.intern_impl_bound_template_name(
            ImplBoundTemplateNameT { code_location_s: range.begin, _phantom: std::marker::PhantomData });
        let bound_name = self.typing_interner.intern_impl_bound_name(
            ImplBoundNameValT { template: tmpl, template_args: &[] });
        let id = *env.original_calling_env.denizen_id().add_step(
            self.typing_interner, INameT::ImplBound(bound_name));
        IsaTemplataT { declaration_range: range, impl_name: id, sub_kind, super_kind }
    }
    /*
        override def assembleImpl(env: InferEnv, range: RangeS, subKind: KindT, superKind: KindT): IsaTemplataT = {
          IsaTemplataT(
            range,
            env.originalCallingEnv.id.addStep(
              interner.intern(
                ImplBoundNameT(
                  interner.intern(ImplBoundTemplateNameT(range.begin)),
                  Vector()))),
            subKind,
            superKind)
        }
      },
      new IInferCompilerDelegate {
        override def resolveInterface(
          callingEnv: IInDenizenEnvironmentT,
          state: CompilerOutputs,
          callRange: List[RangeS],
          callLocation: LocationInDenizen,
          templata: InterfaceDefinitionTemplataT,
          templateArgs: Vector[ITemplataT[ITemplataType]]):
        IResolveOutcome[InterfaceTT] = {
          structCompiler.resolveInterface(state, callingEnv, callRange, callLocation, templata, templateArgs)
        }

        override def resolveStruct(
          callingEnv: IInDenizenEnvironmentT,
          state: CompilerOutputs,
          callRange: List[RangeS],
          callLocation: LocationInDenizen,
          templata: StructDefinitionTemplataT,
          templateArgs: Vector[ITemplataT[ITemplataType]]):
        IResolveOutcome[StructTT] = {
          structCompiler.resolveStruct(state, callingEnv, callRange,callLocation, templata, templateArgs)
        }
*/
    // Per "Compiler/ImplCompiler Name-Collision Disambiguation": Scala's IInferCompilerDelegate
    // anonymous-class `resolveFunction` (Compiler.scala:455-477) is flattened onto Rust's Compiler.
    pub fn resolve_function(
        &self,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        state: &mut CompilerOutputs<'s, 't>,
        ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        name: StrI<'s>,
        coords: &[CoordT<'s, 't>],
        context_region: RegionT,
        verify_conclusions: bool,
    ) -> Result<Result<StampFunctionSuccess<'s, 't>, FindFunctionFailure<'s, 't>>, ICompileErrorT<'s, 't>> {
        let _ = verify_conclusions;
        self.find_function(
            calling_env,
            state,
            ranges,
            call_location,
            self.scout_arena.intern_imprecise_name(
                IImpreciseNameValS::CodeName(
                    CodeNameS { name })),
            &[],
            &[],
            &[],
            context_region,
            coords,
            &[],
            true)
    }
    /*
Guardian: temp-disable: SPDMX — Scala's `overloadResolver.findFunction` throws `CompileErrorExceptionT`; `resolveFunction` does not catch it, so the exception transparently propagates — SPDMX Exception I. Nested `Result<Result<_, FindFunctionFailure>, ICompileErrorT>` is the Rust mirror of that passthrough. Architect-approved for Addendum 6 option 1. — /Volumes/V/Sylvan/FrontendRust/guardian-logs/request-1067-1778812154320/hook-1067/resolve_function--900.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
        override def resolveFunction(
          callingEnv: IInDenizenEnvironmentT,
          state: CompilerOutputs,
          range: List[RangeS],
          callLocation: LocationInDenizen,
          name: StrI,
          coords: Vector[CoordT],
          contextRegion: RegionT,
          verifyConclusions: Boolean):
        Result[StampFunctionSuccess, FindFunctionFailure] = {
          overloadResolver.findFunction(
            callingEnv,
            state,
            range,
            callLocation,
            interner.intern(CodeNameS(interner.intern(name))),
            Vector.empty,
            Vector.empty,
            Vector.empty,
            contextRegion,
            coords,
            Vector.empty,
            true)
        }
    */
/*
        override def resolveStaticSizedArrayKind(
          coutputs: CompilerOutputs,
          mutability: ITemplataT[MutabilityTemplataType],
          variability: ITemplataT[VariabilityTemplataType],
          size: ITemplataT[IntegerTemplataType],
          element: CoordT,
          region: RegionT):
        StaticSizedArrayTT = {
          arrayCompiler.resolveStaticSizedArray(mutability, variability, size, element, region)
        }

        override def resolveRuntimeSizedArrayKind(
          coutputs: CompilerOutputs,
          element: CoordT,
          arrayMutability: ITemplataT[MutabilityTemplataType],
          region: RegionT):
        RuntimeSizedArrayTT = {
          arrayCompiler.resolveRuntimeSizedArray(element, arrayMutability, region)
        }

        override def resolveImpl(
          callingEnv: IInDenizenEnvironmentT,
          state: CompilerOutputs,
          range: List[RangeS],
          callLocation: LocationInDenizen,
          subKind: ISubKindTT,
          superKind: ISuperKindTT):
        IsParentResult = {
          implCompiler.isParent(state, callingEnv, range, callLocation, subKind, superKind)
        }
      })
  val convertHelper =
    new ConvertHelper(
      opts,
      new IConvertHelperDelegate {
        override def isParent(
          coutputs: CompilerOutputs,
          callingEnv: IInDenizenEnvironmentT,
          parentRanges: List[RangeS],
          callLocation: LocationInDenizen,
          descendantCitizenRef: ISubKindTT,
          ancestorInterfaceRef: ISuperKindTT):
        IsParentResult = {
          implCompiler.isParent(
            coutputs, callingEnv, parentRanges, callLocation, descendantCitizenRef, ancestorInterfaceRef)
        }
      })

  val structCompiler: StructCompiler =
    new StructCompiler(
      opts,
      interner,
      keywords,
      nameTranslator,
      templataCompiler,
      inferCompiler,
      new IStructCompilerDelegate {
    */
    // mig: fn evaluate_generic_function_from_non_call_for_header
    // Per "Compiler/ImplCompiler Name-Collision Disambiguation": Scala's IStructCompilerDelegate
    // anonymous-class `evaluateGenericFunctionFromNonCallForHeader` (Compiler.scala:536-544) is
    // flattened onto Rust's Compiler struct. Its body delegates to functionCompiler.evaluateGenericFunctionFromNonCall.
    pub fn evaluate_generic_function_from_non_call_for_header(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function_templata: FunctionTemplataT<'s, 't>,
    ) -> Result<&'t FunctionHeaderT<'s, 't>, ICompileErrorT<'s, 't>> {
        self.evaluate_generic_function_from_non_call(coutputs, parent_ranges, call_location, function_templata)
    }
    /*
        override def evaluateGenericFunctionFromNonCallForHeader(
          coutputs: CompilerOutputs,
          parentRanges: List[RangeS],
          callLocation: LocationInDenizen,
          functionTemplata: FunctionTemplataT):
        FunctionHeaderT = {
          functionCompiler.evaluateGenericFunctionFromNonCall(
            coutputs, parentRanges, callLocation, functionTemplata)
        }

*/
    // mig: fn scout_expected_function_for_prototype
    // Rust adaptation: lifted from Compiler.scala's anonymous IInfererDelegate
    // (which Rust flattened onto Compiler).
    pub fn scout_expected_function_for_prototype(
        &self,
        _env: IInDenizenEnvironmentT<'s, 't>,
        _coutputs: &mut CompilerOutputs<'s, 't>,
        _call_range: &[RangeS<'s>],
        _call_location: LocationInDenizen<'s>,
        _function_name: IImpreciseNameS<'s>,
        _explicit_template_arg_rules_s: &[IRulexSR<'s>],
        _explicit_template_arg_runes_s: &[IRuneS<'s>],
        _context_region: RegionT,
        _args: &[CoordT<'s, 't>],
        _extra_envs_to_look_in: &[IInDenizenEnvironmentT<'s, 't>],
        _exact: bool,
    ) -> StampFunctionSuccess<'s, 't> {
        panic!("Unimplemented: scout_expected_function_for_prototype");
    }
    /*
        override def scoutExpectedFunctionForPrototype(
          env: IInDenizenEnvironmentT,
          coutputs: CompilerOutputs,
          callRange: List[RangeS],
          callLocation: LocationInDenizen,
          functionName: IImpreciseNameS,
          explicitTemplateArgRulesS: Vector[IRulexSR],
          positionalExplicitTemplateArgRunesS: Vector[IRuneS],
          receivingRuneToExplicitTemplateArgRune: Vector[(RuneUsage, RuneUsage)],
          contextRegion: RegionT,
          args: Vector[CoordT],
          extraEnvsToLookIn: Vector[IInDenizenEnvironmentT],
          exact: Boolean):
        StampFunctionSuccess = {
          overloadResolver.findFunction(
            env,
            coutputs,
            callRange,
            callLocation,
            functionName,
            explicitTemplateArgRulesS,
            positionalExplicitTemplateArgRunesS,
            receivingRuneToExplicitTemplateArgRune,
            contextRegion,
            args,
            extraEnvsToLookIn,
            exact) match {
            case Err(e) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(callRange, e))
            case Ok(x) => x
          }
        }
      })

  val implCompiler: ImplCompiler =
    new ImplCompiler(opts, interner, nameTranslator, structCompiler, templataCompiler, inferCompiler)

  val functionCompiler: FunctionCompiler =
    new FunctionCompiler(opts, interner, keywords, nameTranslator, templataCompiler, inferCompiler, convertHelper, structCompiler,
      new IFunctionCompilerDelegate {
    override def evaluateBlockStatements(
        coutputs: CompilerOutputs,
        startingNenv: NodeEnvironmentT,
        nenv: NodeEnvironmentBox,
        life: LocationInFunctionEnvironmentT,
      ranges: List[RangeS],
      callLocation: LocationInDenizen,
        region: RegionT,
        exprs: BlockSE
    ): (ReferenceExpressionTE, Set[CoordT]) = {
      expressionCompiler.evaluateBlockStatements(
        coutputs, startingNenv, nenv, life, ranges, callLocation, region, exprs)
    }

    override def translatePatternList(
      coutputs: CompilerOutputs,
      nenv: NodeEnvironmentBox,
      life: LocationInFunctionEnvironmentT,
      ranges: List[RangeS],
      callLocation: LocationInDenizen,
      region: RegionT,
      patterns1: Vector[AtomSP],
      patternInputExprs2: Vector[ReferenceExpressionTE]
    ): ReferenceExpressionTE = {
      expressionCompiler.translatePatternList(coutputs, nenv, life, ranges, callLocation, patterns1, patternInputExprs2, region)
    }

//    override def evaluateParent(env: IEnvironment, coutputs: CompilerOutputs, callRange: List[RangeS], sparkHeader: FunctionHeaderT): Unit = {
//      virtualCompiler.evaluateParent(env, coutputs, callRange, sparkHeader)
//    }

*/
    // mig: fn generate_function
    // Rust adaptation: lifted from Compiler.scala's anonymous IFunctionCompilerDelegate
    // (which Rust flattened onto Compiler). Scala's `functionCompilerCore: FunctionCompilerCore`,
    // `structCompiler`, `destructorCompiler`, `arrayCompiler` parameters are absorbed
    // into `&self` since all four compilers are flattened onto `Compiler` in Rust.
    pub fn generate_function(
        &self,
        _generator: IFunctionGenerator,
        _full_env: &'t FunctionEnvironmentT<'s, 't>,
        _coutputs: &mut CompilerOutputs<'s, 't>,
        _life: LocationInFunctionEnvironmentT<'s, 't>,
        _call_range: &[RangeS<'s>],
        _origin_function: Option<&'s FunctionA<'s>>,
        _param_coords: &[ParameterT<'s, 't>],
        _maybe_ret_coord: Option<CoordT<'s, 't>>,
    ) -> &'t FunctionHeaderT<'s, 't> {
        panic!("Unimplemented: generate_function");
    }
    /*
    override def generateFunction(
      functionCompilerCore: FunctionCompilerCore,
      generator: IFunctionGenerator,
      fullEnv: FunctionEnvironmentT,
      coutputs: CompilerOutputs,
      life: LocationInFunctionEnvironmentT,
      callRange: List[RangeS],
      originFunction: Option[FunctionA],
      paramCoords: Vector[ParameterT],
      maybeRetCoord: Option[CoordT]):
    FunctionHeaderT = {
      generator.generate(

        functionCompilerCore, structCompiler, destructorCompiler, arrayCompiler, fullEnv, coutputs, life, callRange, originFunction, paramCoords, maybeRetCoord)
    }
  })
  val overloadResolver: OverloadResolver = new OverloadResolver(opts, interner, keywords, templataCompiler, inferCompiler, functionCompiler)
  val destructorCompiler: DestructorCompiler = new DestructorCompiler(opts, interner, keywords, structCompiler, overloadResolver)

  val virtualCompiler = new VirtualCompiler(opts, interner, overloadResolver)

  val sequenceCompiler = new SequenceCompiler(opts, interner, keywords, structCompiler, templataCompiler)

  val arrayCompiler: ArrayCompiler =
    new ArrayCompiler(
      opts,
      interner,
      keywords,
      inferCompiler,
      overloadResolver,
      destructorCompiler,
      templataCompiler)

  val expressionCompiler: ExpressionCompiler =
    new ExpressionCompiler(
      opts,
      interner,
      keywords,
      nameTranslator,
      templataCompiler,
      inferCompiler,
      arrayCompiler,
      structCompiler,
      implCompiler,
      sequenceCompiler,
      overloadResolver,
      destructorCompiler,
      implCompiler,
      convertHelper,
      new IExpressionCompilerDelegate {
        override def evaluateTemplatedFunctionFromCallForPrototype(
            coutputs: CompilerOutputs,
            callingEnv: IInDenizenEnvironmentT, // See CSSNCE
            callRange: List[RangeS],
          callLocation: LocationInDenizen,
            functionTemplata: FunctionTemplataT,
            explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
          contextRegion: RegionT,
            args: Vector[CoordT]):
        IEvaluateFunctionResult = {
          functionCompiler.evaluateTemplatedFunctionFromCallForPrototype(
            coutputs, callRange, callLocation, callingEnv, functionTemplata, explicitTemplateArgs, contextRegion, args)
        }

        override def evaluateGenericFunctionFromCallForPrototype(
          coutputs: CompilerOutputs,
          callingEnv: IInDenizenEnvironmentT, // See CSSNCE
          callRange: List[RangeS],
          callLocation: LocationInDenizen,
          functionTemplata: FunctionTemplataT,
          explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
          contextRegion: RegionT,
          args: Vector[CoordT]):
        IResolveFunctionResult = {
          functionCompiler.evaluateGenericLightFunctionFromCallForPrototype(
            coutputs, callRange, callLocation, callingEnv, functionTemplata, explicitTemplateArgs, contextRegion, args)
        }

        override def evaluateClosureStruct(
            coutputs: CompilerOutputs,
            containingNodeEnv: NodeEnvironmentT,
            callRange: List[RangeS],
          callLocation: LocationInDenizen,
            name: IFunctionDeclarationNameS,
            function1: FunctionA):
        StructTT = {
          functionCompiler.evaluateClosureStruct(coutputs, containingNodeEnv, callRange, callLocation, name, function1, true)
        }
      })

  val edgeCompiler = new EdgeCompiler(opts, interner, keywords, functionCompiler, overloadResolver, implCompiler)

  val functorHelper = new FunctorHelper(interner, keywords)
  val structConstructorMacro = new StructConstructorMacro(opts, interner, keywords, nameTranslator, destructorCompiler)
  val structDropMacro = new StructDropMacro(opts, interner, keywords, nameTranslator, destructorCompiler)
//  val structFreeMacro = new StructFreeMacro(interner, keywords, nameTranslator, destructorCompiler)
//  val interfaceFreeMacro = new InterfaceFreeMacro(interner, keywords, nameTranslator)
  val asSubtypeMacro = new AsSubtypeMacro(keywords, implCompiler, expressionCompiler, destructorCompiler)
  val rsaLenMacro = new RSALenMacro(keywords)
  val rsaMutNewMacro = new RSAMutableNewMacro(interner, keywords, arrayCompiler, destructorCompiler)
  val rsaImmNewMacro = new RSAImmutableNewMacro(interner, keywords, overloadResolver, arrayCompiler, destructorCompiler)
  val rsaPushMacro = new RSAMutablePushMacro(interner, keywords)
  val rsaPopMacro = new RSAMutablePopMacro(interner, keywords)
  val rsaCapacityMacro = new RSAMutableCapacityMacro(interner, keywords)
  val ssaLenMacro = new SSALenMacro(keywords)
  val rsaDropMacro = new RSADropIntoMacro(keywords, arrayCompiler)
  val ssaDropMacro = new SSADropIntoMacro(keywords, arrayCompiler)
//  val ssaLenMacro = new SSALenMacro(keywords)
//  val implDropMacro = new ImplDropMacro(interner, nameTranslator)
//  val implFreeMacro = new ImplFreeMacro(interner, keywords, nameTranslator)
  val interfaceDropMacro = new InterfaceDropMacro(interner, keywords, nameTranslator)
  val abstractBodyMacro = new AbstractBodyMacro(interner, keywords, overloadResolver)
  val lockWeakMacro = new LockWeakMacro(keywords, expressionCompiler)
  val sameInstanceMacro = new SameInstanceMacro(keywords)
  val anonymousInterfaceMacro =
    new AnonymousInterfaceMacro(
      opts, interner, keywords, nameTranslator, overloadResolver, structCompiler, structConstructorMacro, structDropMacro)

    */
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate<'p>(
        &self,
        _code_map: &FileCoordinateMap<'p, String>,
        package_to_program_a: &PackageCoordinateMap<'s, ProgramA<'s>>,
    ) -> Result<HinputsT<'s, 't>, ICompileErrorT<'s, 't>> {
        let name_to_struct_defined_macro: HashMap<StrI<'s>, OnStructDefinedMacro> = {
            let mut m = HashMap::new();
            m.insert(self.keywords.derive_struct_constructor, OnStructDefinedMacro::StructConstructor);
            m.insert(self.keywords.derive_struct_drop, OnStructDefinedMacro::StructDrop);
            m
        };
        let name_to_interface_defined_macro: HashMap<StrI<'s>, OnInterfaceDefinedMacro> = {
            let mut m = HashMap::new();
            m.insert(self.keywords.derive_interface_drop, OnInterfaceDefinedMacro::InterfaceDrop);
            m.insert(self.keywords.derive_anonymous_substruct, OnInterfaceDefinedMacro::AnonymousInterface);
            m
        };
        let mut id_and_env_entry: Vec<(&'t IdT<'s, 't>, IEnvEntryT<'s, 't>)> = Vec::new();
        for (coord, program_a) in &package_to_program_a.package_coord_to_contents {
            let pkg_top_level_name =
                self.typing_interner.intern_package_top_level_name(PackageTopLevelNameT { _phantom: PhantomData });
            let pkg_top_level = INameT::PackageTopLevel(pkg_top_level_name);
            for struct_a in program_a.structs.iter() {
                let struct_template_name = self.translate_struct_name(struct_a.name);
                let struct_name_local: INameT<'s, 't> = match struct_template_name {
                    IStructTemplateNameT::StructTemplate(r) => INameT::StructTemplate(r),
                    IStructTemplateNameT::AnonymousSubstructTemplate(r) => INameT::AnonymousSubstructTemplate(r),
                    IStructTemplateNameT::LambdaCitizenTemplate(_) => panic!("Unimplemented: LambdaCitizenTemplate in struct translation"),
                };
                let package_name = self.typing_interner.intern_id(IdValT {
                    package_coord: coord,
                    init_steps: &[],
                    local_name: pkg_top_level,
                });
                let struct_name_t = package_name.add_step(self.typing_interner, struct_name_local);
                id_and_env_entry.push((struct_name_t, IEnvEntryT::Struct(struct_a)));
                let preprocess_entries = self.preprocess_struct(&name_to_struct_defined_macro, *struct_name_t, struct_a);
                for entry in preprocess_entries {
                    id_and_env_entry.push((entry.0, entry.1));
                }
            }
            for interface_a in program_a.interfaces.iter() {
                let interface_template_name = self.translate_interface_name(*interface_a.name);
                let interface_name_local: INameT<'s, 't> = match interface_template_name {
                    IInterfaceTemplateNameT::InterfaceTemplate(r) => INameT::InterfaceTemplate(r),
                };
                let package_name = self.typing_interner.intern_id(IdValT {
                    package_coord: coord,
                    init_steps: &[],
                    local_name: pkg_top_level,
                });
                let interface_name_t = package_name.add_step(self.typing_interner, interface_name_local);
                id_and_env_entry.push((interface_name_t, IEnvEntryT::Interface(interface_a)));
                let preprocess_entries = self.preprocess_interface(&name_to_interface_defined_macro, *interface_name_t, interface_a);
                for entry in preprocess_entries {
                    id_and_env_entry.push((entry.0, entry.1));
                }
            }
            for impl_a in program_a.impls.iter() {
                let impl_template_name = self.translate_impl_name(impl_a.name);
                let impl_name_local: INameT<'s, 't> = match impl_template_name {
                    IImplTemplateNameT::ImplTemplate(r) => INameT::ImplTemplate(r),
                    IImplTemplateNameT::ImplBoundTemplate(_) => panic!("Unimplemented: ImplBoundTemplate in impl translation"),
                    IImplTemplateNameT::AnonymousSubstructImplTemplate(_) => panic!("Unimplemented: AnonymousSubstructImplTemplate in impl translation"),
                };
                let package_name = self.typing_interner.intern_id(IdValT {
                    package_coord: coord,
                    init_steps: &[],
                    local_name: pkg_top_level,
                });
                let impl_name_t = package_name.add_step(self.typing_interner, impl_name_local);
                id_and_env_entry.push((impl_name_t, IEnvEntryT::Impl(impl_a)));
            }
            for function_a in program_a.functions.iter() {
                let function_template_name =
                    self.translate_generic_function_name(function_a.name);
                let function_name_local: INameT<'s, 't> = match function_template_name {
                    IFunctionTemplateNameT::FunctionTemplate(r) => INameT::FunctionTemplate(r),
                    IFunctionTemplateNameT::ForwarderFunctionTemplate(r) => INameT::ForwarderFunctionTemplate(r),
                    IFunctionTemplateNameT::ConstructorTemplate(r) => INameT::ConstructorTemplate(r),
                    IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(r) => INameT::AnonymousSubstructConstructorTemplate(r),
                    IFunctionTemplateNameT::LambdaCallFunctionTemplate(r) => INameT::LambdaCallFunctionTemplate(r),
                    IFunctionTemplateNameT::OverrideDispatcherTemplate(r) => INameT::OverrideDispatcherTemplate(r),
                    IFunctionTemplateNameT::ExternFunction(r) => INameT::ExternFunction(r),
                    IFunctionTemplateNameT::FunctionBoundTemplate(r) => INameT::FunctionBoundTemplate(r),
                    IFunctionTemplateNameT::PredictedFunctionTemplate(r) => INameT::PredictedFunctionTemplate(r),
                };
                let package_name = self.typing_interner.intern_id(IdValT {
                    package_coord: coord,
                    init_steps: &[],
                    local_name: pkg_top_level,
                });
                let function_name_t = package_name.add_step(self.typing_interner, function_name_local);
                id_and_env_entry.push((function_name_t, IEnvEntryT::Function(function_a)));
            }
        }

        let pkg_top_level_for_group = INameT::PackageTopLevel(
            self.typing_interner.intern_package_top_level_name(PackageTopLevelNameT { _phantom: PhantomData })
        );
        // Per @IIIOZ: IndexMap so iteration at line ~1350 (into global_env.name_to_top_level_environment)
        // preserves id_and_env_entry source order — otherwise the package env's `global_namespaces`
        // slice ends up in random per-process HashMap order, and lookups that walk it nondeterministically
        // pick a different "drop" overload per run.
        let mut namespace_name_to_entries: IndexMap<&'t IdT<'s, 't>, Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)>> = IndexMap::new();
        for (name, env_entry) in &id_and_env_entry {
            let package_id = self.typing_interner.intern_id(IdValT {
                package_coord: name.package_coord,
                init_steps: name.init_steps,
                local_name: pkg_top_level_for_group,
            });
            namespace_name_to_entries
                .entry(package_id)
                .or_insert_with(Vec::new)
                .push((name.local_name, *env_entry));
        }
        let mut namespace_name_to_templatas_vec: Vec<(&'t IdT<'s, 't>, &'t TemplatasStoreT<'s, 't>)> = Vec::new();
        for (package_id, entries) in namespace_name_to_entries {
            let mut builder = TemplatasStoreBuilder::new(package_id);
            builder.add_entries(self.scout_arena, entries);
            namespace_name_to_templatas_vec.push((package_id, builder.build_in(self.typing_interner)));
        }

        let builtin_coord: &'s PackageCoordinate<'s> =
            self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[]);
        let builtin_id = self.typing_interner.intern_id(IdValT {
            package_coord: builtin_coord,
            init_steps: &[],
            local_name: INameT::PackageTopLevel(
                self.typing_interner.intern_package_top_level_name(PackageTopLevelNameT { _phantom: PhantomData })
            ),
        });
        let mut builtins_builder = TemplatasStoreBuilder::new(builtin_id);
        let primitives: &[(StrI<'s>, KindT<'s, 't>)] = &[
            (self.keywords.int, KindT::Int(IntT::I32)),
            (self.keywords.i64, KindT::Int(IntT::I64)),
            (self.keywords.bool, KindT::Bool(BoolT)),
            (self.keywords.float, KindT::Float(FloatT)),
            (self.keywords.__never, KindT::Never(NeverT { from_break: false })),
            (self.keywords.str, KindT::Str(StrT)),
            (self.keywords.void, KindT::Void(VoidT)),
        ];
        for (human_name, kind) in primitives {
            let prim = INameT::Primitive(self.typing_interner.intern_primitive_name(
                PrimitiveNameT { human_name: *human_name, _phantom: PhantomData }
            ));
            let kind_t = ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: *kind }));
            builtins_builder.name_to_entry.push((prim, IEnvEntryT::Templata(kind_t)));
            if let Some(imprecise) = get_imprecise_name(self.scout_arena, prim) {
                builtins_builder.imprecise_to_entries.entry(imprecise).or_insert_with(Vec::new).push(IEnvEntryT::Templata(kind_t));
            }
        }
        {
            let prim = INameT::Primitive(self.typing_interner.intern_primitive_name(
                PrimitiveNameT { human_name: self.keywords.array, _phantom: PhantomData }
            ));
            let entry = IEnvEntryT::Templata(
                ITemplataT::RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateTemplataT { _phantom: PhantomData })
            );
            builtins_builder.name_to_entry.push((prim, entry));
            if let Some(imprecise) = get_imprecise_name(self.scout_arena, prim) {
                builtins_builder.imprecise_to_entries.entry(imprecise).or_insert_with(Vec::new).push(entry);
            }
        }
        {
            let prim = INameT::Primitive(self.typing_interner.intern_primitive_name(
                PrimitiveNameT { human_name: self.keywords.static_array, _phantom: PhantomData }
            ));
            let entry = IEnvEntryT::Templata(
                ITemplataT::StaticSizedArrayTemplate(StaticSizedArrayTemplateTemplataT { _phantom: PhantomData })
            );
            builtins_builder.name_to_entry.push((prim, entry));
            if let Some(imprecise) = get_imprecise_name(self.scout_arena, prim) {
                builtins_builder.imprecise_to_entries.entry(imprecise).or_insert_with(Vec::new).push(entry);
            }
        }
        let builtins = builtins_builder.build_in(self.typing_interner);

        let name_to_top_level_environment =
            self.typing_interner.alloc_slice_from_vec(namespace_name_to_templatas_vec);

        // Mirrors Scala compiler.scala:1170-1187 nameToFunctionBodyMacro Map population.
        let mut name_to_function_body_macro =
            self.typing_interner.alloc_index_map::<StrI<'s>, FunctionBodyMacro>();
        name_to_function_body_macro.insert(self.keywords.abstract_body, FunctionBodyMacro::AbstractBody);
        name_to_function_body_macro.insert(self.keywords.struct_constructor_generator, FunctionBodyMacro::StructConstructor);
        name_to_function_body_macro.insert(self.keywords.drop_generator, FunctionBodyMacro::StructDrop);
        name_to_function_body_macro.insert(self.keywords.vale_runtime_sized_array_len, FunctionBodyMacro::RsaLen);
        name_to_function_body_macro.insert(self.keywords.vale_runtime_sized_array_mut_new, FunctionBodyMacro::RsaMutableNew);
        name_to_function_body_macro.insert(self.keywords.vale_runtime_sized_array_imm_new, FunctionBodyMacro::RsaImmutableNew);
        name_to_function_body_macro.insert(self.keywords.vale_runtime_sized_array_push, FunctionBodyMacro::RsaMutablePush);
        name_to_function_body_macro.insert(self.keywords.vale_runtime_sized_array_pop, FunctionBodyMacro::RsaMutablePop);
        name_to_function_body_macro.insert(self.keywords.vale_runtime_sized_array_capacity, FunctionBodyMacro::RsaMutableCapacity);
        name_to_function_body_macro.insert(self.keywords.vale_static_sized_array_len, FunctionBodyMacro::SsaLen);
        name_to_function_body_macro.insert(self.keywords.vale_runtime_sized_array_drop_into, FunctionBodyMacro::RsaDropInto);
        name_to_function_body_macro.insert(self.keywords.vale_static_sized_array_drop_into, FunctionBodyMacro::SsaDropInto);
        name_to_function_body_macro.insert(self.keywords.vale_lock_weak, FunctionBodyMacro::LockWeak);
        name_to_function_body_macro.insert(self.keywords.vale_same_instance, FunctionBodyMacro::SameInstance);
        name_to_function_body_macro.insert(self.keywords.vale_as_subtype, FunctionBodyMacro::AsSubtype);

        let global_env: &'t GlobalEnvironmentT<'s, 't> = self.typing_interner.alloc(GlobalEnvironmentT {
            name_to_top_level_environment,
            name_to_function_body_macro,
            builtins,
        });

        let mut coutputs = CompilerOutputs::new();

        self.compile_static_sized_array(global_env, &mut coutputs);
        self.compile_runtime_sized_array(global_env, &mut coutputs);

        // Indexing phase
        for (package_id, templatas) in global_env.name_to_top_level_environment {
            let env = make_top_level_environment(global_env, **package_id, self.typing_interner);
            let env_ref: IEnvironmentT<'s, 't> =
                IEnvironmentT::Package(env);
            for (_name, entry) in templatas.name_to_entry.iter() {
                match entry {
                    IEnvEntryT::Struct(struct_a) => {
                        let templata = StructDefinitionTemplataT { declaring_env: env_ref, origin_struct: struct_a };
                        self.precompile_struct(&mut coutputs, templata);
                    }
                    IEnvEntryT::Interface(interface_a) => {
                        let templata = InterfaceDefinitionTemplataT { declaring_env: env_ref, origin_interface: interface_a };
                        self.precompile_interface(&mut coutputs, templata);
                    }
                    _ => {}
                }
            }
        }

        // Compiling phase
        let mut unchecked_defining_conclusionses: Vec<UncheckedDefiningConclusions<'s, 't>> = Vec::new();
        for (package_id, templatas) in global_env.name_to_top_level_environment {
            let env = make_top_level_environment(global_env, **package_id, self.typing_interner);
            let env_ref: IEnvironmentT<'s, 't> =
                IEnvironmentT::Package(env);
            // This makes it so anything starting with an underscore is compiled in the order
            // of their names.
            // AFTERM: is there a better solution here? should we always order things?
            let mut orderable_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = Vec::new();
            let mut unordered_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = Vec::new();
            for (name, entry) in templatas.name_to_entry.iter() {
                match name {
                    INameT::StructTemplate(s) if s.human_name.0.starts_with("_") =>
                        orderable_entries.push((*name, *entry)),
                    INameT::InterfaceTemplate(i) if i.human_namee.0.starts_with("_") =>
                        orderable_entries.push((*name, *entry)),
                    _ => unordered_entries.push((*name, *entry)),
                }
            }
            // orderedEntries = orderableEntries.sortBy(_._1.humanName.str)
            orderable_entries.sort_by(|(a, _), (b, _)| panic!("Unimplemented: sort orderable entries"));
            let all_entries = orderable_entries.into_iter().chain(unordered_entries.into_iter());
            for (_name, entry) in all_entries {
                match entry {
                    IEnvEntryT::Struct(struct_a) => {
                        let templata = StructDefinitionTemplataT { declaring_env: env_ref, origin_struct: struct_a };
                        let unchecked_conclusions =
                            self.compile_struct(&mut coutputs, &[], LocationInDenizen { path: &[] }, templata)?;
                        let maybe_export =
                            struct_a.attributes.iter().find_map(|a| match a { ICitizenAttributeS::Export(e) => Some(e), _ => None });
                        match maybe_export {
                            None => {}
                            Some(export_s) => {
                                let template_name = self.typing_interner.intern_export_template_name(ExportTemplateNameT {
                                    code_loc: struct_a.range.begin,
                                    _phantom: PhantomData,
                                });
                                let template_id_steps: Vec<INameT<'s, 't>> = vec![];
                                let template_id = *self.typing_interner.intern_id(IdValT {
                                    package_coord: package_id.package_coord,
                                    init_steps: &template_id_steps,
                                    local_name: INameT::ExportTemplate(template_name),
                                });
                                let template_id_ref = self.typing_interner.alloc(template_id);
                                let export_outer_templatas = TemplatasStoreBuilder::new(template_id_ref).build_in(self.typing_interner);
                                let _export_outer_env = self.typing_interner.alloc(ExportEnvironmentT {
                                    global_env,
                                    parent_env: env,
                                    template_id,
                                    id: template_id,
                                    templatas: export_outer_templatas,
                                });
                                let placeholdered_export_name = self.typing_interner.intern_export_name(ExportNameT {
                                    template: template_name,
                                    region: RegionT { region: IRegionT::Default },
                                });
                                let placeholdered_export_id_steps: Vec<INameT<'s, 't>> = vec![];
                                let placeholdered_export_id = *self.typing_interner.intern_id(IdValT {
                                    package_coord: package_id.package_coord,
                                    init_steps: &placeholdered_export_id_steps,
                                    local_name: INameT::Export(placeholdered_export_name),
                                });
                                let placeholdered_export_id_ref = self.typing_interner.alloc(placeholdered_export_id);
                                let export_templatas = TemplatasStoreBuilder::new(placeholdered_export_id_ref).build_in(self.typing_interner);
                                let export_env = self.typing_interner.alloc(ExportEnvironmentT {
                                    global_env,
                                    parent_env: env,
                                    template_id,
                                    id: placeholdered_export_id,
                                    templatas: export_templatas,
                                });
                                let export_env_as_iindenizen = IInDenizenEnvironmentT::Export(export_env);
                                let export_call_range = self.typing_interner.alloc_slice_copy(&[struct_a.range]);
                                let export_placeholdered_struct = match self.resolve_struct(
                                    &mut coutputs,
                                    export_env_as_iindenizen,
                                    export_call_range,
                                    LocationInDenizen { path: &[] },
                                    templata,
                                    &[],
                                ) {
                                    IResolveOutcome::ResolveSuccess(s) => self.typing_interner.alloc(s.kind),
                                    IResolveOutcome::ResolveFailure(_f) => panic!("vwat: resolve struct failed for export"),
                                };
                                let export_name = match struct_a.name {
                                    IStructDeclarationNameS::TopLevelStructDeclarationName(n) => n.name,
                                    other => panic!("vwat: {:?}", other),
                                };
                                coutputs.add_kind_export(
                                    struct_a.range,
                                    KindT::Struct(export_placeholdered_struct),
                                    placeholdered_export_id,
                                    export_name,
                                    self.typing_interner,
                                );
                            }
                        }
                        unchecked_defining_conclusionses.push(unchecked_conclusions);
                    }
                    IEnvEntryT::Interface(interface_a) => {
                        let templata = InterfaceDefinitionTemplataT { declaring_env: env_ref, origin_interface: interface_a };
                        let unchecked_conclusions =
                            self.compile_interface(&mut coutputs, &[], LocationInDenizen { path: &[] }, templata)?;
                        let maybe_export =
                            interface_a.attributes.iter().find_map(|a| match a { ICitizenAttributeS::Export(e) => Some(e), _ => None });
                        match maybe_export {
                            None => {}
                            Some(_export_s) => {
                                let template_name = self.typing_interner.intern_export_template_name(ExportTemplateNameT {
                                    code_loc: interface_a.range.begin,
                                    _phantom: PhantomData,
                                });
                                let template_id_steps: Vec<INameT<'s, 't>> = vec![];
                                let template_id = *self.typing_interner.intern_id(IdValT {
                                    package_coord: package_id.package_coord,
                                    init_steps: &template_id_steps,
                                    local_name: INameT::ExportTemplate(template_name),
                                });
                                let template_id_ref = self.typing_interner.alloc(template_id);
                                let export_outer_templatas = TemplatasStoreBuilder::new(template_id_ref).build_in(self.typing_interner);
                                let _export_outer_env = self.typing_interner.alloc(ExportEnvironmentT {
                                    global_env,
                                    parent_env: env,
                                    template_id,
                                    id: template_id,
                                    templatas: export_outer_templatas,
                                });
                                let placeholdered_export_name = self.typing_interner.intern_export_name(ExportNameT {
                                    template: template_name,
                                    region: RegionT { region: IRegionT::Default },
                                });
                                let placeholdered_export_id_steps: Vec<INameT<'s, 't>> = vec![];
                                let placeholdered_export_id = *self.typing_interner.intern_id(IdValT {
                                    package_coord: package_id.package_coord,
                                    init_steps: &placeholdered_export_id_steps,
                                    local_name: INameT::Export(placeholdered_export_name),
                                });
                                let placeholdered_export_id_ref = self.typing_interner.alloc(placeholdered_export_id);
                                let export_templatas = TemplatasStoreBuilder::new(placeholdered_export_id_ref).build_in(self.typing_interner);
                                let export_env = self.typing_interner.alloc(ExportEnvironmentT {
                                    global_env,
                                    parent_env: env,
                                    template_id,
                                    id: placeholdered_export_id,
                                    templatas: export_templatas,
                                });
                                let export_env_as_iindenizen = IInDenizenEnvironmentT::Export(export_env);
                                let export_call_range = self.typing_interner.alloc_slice_copy(&[interface_a.range]);
                                let export_placeholdered_kind = match self.resolve_interface(
                                    &mut coutputs,
                                    export_env_as_iindenizen,
                                    export_call_range,
                                    LocationInDenizen { path: &[] },
                                    templata,
                                    &[],
                                ) {
                                    IResolveOutcome::ResolveSuccess(s) => self.typing_interner.alloc(s.kind),
                                    IResolveOutcome::ResolveFailure(_f) => panic!("vwat: resolve interface failed for export"),
                                };
                                let export_name = interface_a.name.name;
                                coutputs.add_kind_export(
                                    interface_a.range,
                                    KindT::Interface(export_placeholdered_kind),
                                    placeholdered_export_id,
                                    export_name,
                                    self.typing_interner,
                                );
                            }
                        }
                        unchecked_defining_conclusionses.push(unchecked_conclusions);
                    }
                    _ => {}
                }
            }
        }

        // Struct/interface resolution phase
        for unchecked in unchecked_defining_conclusionses.into_iter() {
            let _instantiation_bound_args_unused =
                match self.check_defining_conclusions_and_resolve(
                    unchecked.envs,
                    &mut coutputs,
                    &unchecked.ranges,
                    unchecked.call_location,
                    &unchecked.definition_rules,
                    &[],
                    &unchecked.conclusions,
                ) {
                    Err(_f) => panic!("implement: check_defining_conclusions_and_resolve error in resolution phase"),
                    Ok(c) => c,
                };
        }

        // Impl compile phase
        for (package_id, templatas) in global_env.name_to_top_level_environment {
            let package_env = make_top_level_environment(global_env, **package_id, self.typing_interner);
            let package_env_t: IEnvironmentT<'s, 't> =
                IEnvironmentT::Package(package_env);
            for (_name, entry) in templatas.name_to_entry.iter() {
                match entry {
                    IEnvEntryT::Impl(impl_a) => {
                        let impl_templata = self.typing_interner.alloc(ImplDefinitionTemplataT {
                            env: package_env_t,
                            impl_: impl_a,
                        });
                        self.compile_impl(&mut coutputs, LocationInDenizen { path: &[] }, *impl_templata)?;
                    }
                    _ => {}
                }
            }
        }

        // Function compile phase
        for (package_id, templatas) in global_env.name_to_top_level_environment {
            if !package_id.init_steps.is_empty() {
                continue;
            }
            let global_namespaces: Vec<&TemplatasStoreT<'s, 't>> =
                global_env.name_to_top_level_environment.iter().map(|(_, ts)| *ts).collect();
            let global_namespaces = self.typing_interner.alloc_slice_from_vec(global_namespaces);
            let package_env = self.typing_interner.alloc(PackageEnvironmentT {
                global_env,
                id: **package_id,
                global_namespaces,
            });
            let package_env_t: IEnvironmentT<'s, 't> =
                IEnvironmentT::Package(package_env);
            for (_name, entry) in templatas.name_to_entry.iter() {
                match entry {
                    IEnvEntryT::Function(function_a) => {
                        let templata = FunctionTemplataT {
                            outer_env: package_env_t,
                            function: function_a,
                        };
                        let _header = self.evaluate_generic_function_from_non_call(
                            &mut coutputs, &[], LocationInDenizen { path: &[] }, templata)?;
                        let maybe_export = function_a.attributes.iter().find_map(|a| match a { IFunctionAttributeS::Export(e) => Some(e), _ => None });
                        match maybe_export {
                            None => {}
                            Some(_export_s) => {

                                let template_name = self.typing_interner.intern_export_template_name(ExportTemplateNameT {
                                    code_loc: function_a.range.begin,
                                    _phantom: PhantomData,
                                });
                                let template_id_steps: Vec<INameT<'s, 't>> = vec![];
                                let template_id = *self.typing_interner.intern_id(IdValT {
                                    package_coord: package_id.package_coord,
                                    init_steps: &template_id_steps,
                                    local_name: INameT::ExportTemplate(template_name),
                                });
                                let template_id_ref = self.typing_interner.alloc(template_id);
                                let export_outer_templatas = TemplatasStoreBuilder::new(template_id_ref).build_in(self.typing_interner);
                                let _export_outer_env = self.typing_interner.alloc(ExportEnvironmentT {
                                    global_env,
                                    parent_env: package_env,
                                    template_id,
                                    id: template_id,
                                    templatas: export_outer_templatas,
                                });
                                let region_placeholder = RegionT { region: IRegionT::Default };
                                let placeholdered_export_name = self.typing_interner.intern_export_name(ExportNameT {
                                    template: template_name,
                                    region: region_placeholder,
                                });
                                let placeholdered_export_id_steps: Vec<INameT<'s, 't>> = vec![];
                                let placeholdered_export_id = *self.typing_interner.intern_id(IdValT {
                                    package_coord: package_id.package_coord,
                                    init_steps: &placeholdered_export_id_steps,
                                    local_name: INameT::Export(placeholdered_export_name),
                                });
                                let placeholdered_export_id_ref = self.typing_interner.alloc(placeholdered_export_id);
                                let export_templatas = TemplatasStoreBuilder::new(placeholdered_export_id_ref).build_in(self.typing_interner);
                                let export_env = self.typing_interner.alloc(ExportEnvironmentT {
                                    global_env,
                                    parent_env: package_env,
                                    template_id,
                                    id: placeholdered_export_id,
                                    templatas: export_templatas,
                                });
                                let export_env_as_iindenizen = IInDenizenEnvironmentT::Export(export_env);
                                let call_ranges = self.typing_interner.alloc_slice_copy(&[function_a.range]);
                                let export_placeholdered_prototype =
                                    match self.evaluate_generic_light_function_from_call_for_prototype(
                                        &mut coutputs,
                                        call_ranges,
                                        LocationInDenizen { path: &[] },
                                        export_env_as_iindenizen,
                                        templata,
                                        &[],
                                        region_placeholder,
                                        &[],
                                        &[],
                                    )? {
                                        IResolveFunctionResult::ResolveFunctionSuccess(success) => success.prototype.prototype,
                                        IResolveFunctionResult::ResolveFunctionFailure(failure) => {
                                            return Err(ICompileErrorT::TypingPassResolvingError {
                                                range: self.typing_interner.alloc_slice_copy(&[function_a.range]),
                                                inner: failure.reason,
                                            });
                                        }
                                    };
                                let export_name = match function_a.name {
                                    IFunctionDeclarationNameS::FunctionName(fn_name_s) => fn_name_s.name,
                                    other => panic!("vwat: {:?}", other),
                                };
                                coutputs.add_function_export(
                                    function_a.range,
                                    export_placeholdered_prototype,
                                    placeholdered_export_id,
                                    export_name,
                                    self.typing_interner,
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Export compile phase
        // packageToProgramA.flatMap({ case (packageCoord, programA) => ... programA.exports.foreach(...) })
        for (coord, program_a) in &package_to_program_a.package_coord_to_contents {
            for export in program_a.exports.iter() {

                let package_top_level_name = self.typing_interner.intern_package_top_level_name(PackageTopLevelNameT { _phantom: PhantomData });
                let package_id_steps: Vec<INameT<'s, 't>> = vec![];
                let package_id = *self.typing_interner.intern_id(IdValT {
                    package_coord: coord,
                    init_steps: &package_id_steps,
                    local_name: INameT::PackageTopLevel(package_top_level_name),
                });
                let package_env = make_top_level_environment(global_env, package_id, self.typing_interner);

                let type_rune_t = export.type_rune.clone();

                let template_name = self.typing_interner.intern_export_template_name(ExportTemplateNameT {
                    code_loc: export.range.begin,
                    _phantom: PhantomData,
                });
                let template_id_steps: Vec<INameT<'s, 't>> = vec![];
                let template_id = *self.typing_interner.intern_id(IdValT {
                    package_coord: coord,
                    init_steps: &template_id_steps,
                    local_name: INameT::ExportTemplate(template_name),
                });
                let template_id_ref = self.typing_interner.alloc(template_id);
                let export_outer_templatas = TemplatasStoreBuilder::new(template_id_ref).build_in(self.typing_interner);
                let _export_outer_env = self.typing_interner.alloc(ExportEnvironmentT {
                    global_env,
                    parent_env: package_env,
                    template_id,
                    id: template_id,
                    templatas: export_outer_templatas,
                });

                let region_placeholder = RegionT { region: IRegionT::Default };

                let placeholdered_export_name = self.typing_interner.intern_export_name(ExportNameT {
                    template: template_name,
                    region: region_placeholder,
                });
                let placeholdered_export_id_steps: Vec<INameT<'s, 't>> = vec![];
                let placeholdered_export_id = *self.typing_interner.intern_id(IdValT {
                    package_coord: coord,
                    init_steps: &placeholdered_export_id_steps,
                    local_name: INameT::Export(placeholdered_export_name),
                });
                let placeholdered_export_id_ref = self.typing_interner.alloc(placeholdered_export_id);
                let export_templatas = TemplatasStoreBuilder::new(placeholdered_export_id_ref).build_in(self.typing_interner);
                let export_env = self.typing_interner.alloc(ExportEnvironmentT {
                    global_env,
                    parent_env: package_env,
                    template_id,
                    id: placeholdered_export_id,
                    templatas: export_templatas,
                });
                let export_env_as_iindenizen = IInDenizenEnvironmentT::Export(export_env);
                let export_env_as_ienv = IEnvironmentT::Export(export_env);

                let rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
                    export.rune_to_type.iter().map(|(k, v)| (*k, *v)).collect();

                let parent_ranges_t: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(&[export.range]);

                let complete_define_solve = match self.solve_for_defining(
                    InferEnv {
                        original_calling_env: export_env_as_iindenizen,
                        parent_ranges: parent_ranges_t,
                        call_location: LocationInDenizen { path: &[] },
                        self_env: export_env_as_ienv,
                        context_region: region_placeholder,
                    },
                    &mut coutputs,
                    export.rules,
                    &rune_to_type,
                    parent_ranges_t,
                    LocationInDenizen { path: &[] },
                    &[],
                    &[],
                    &[],
                ) {
                    Err(_f) => panic!("implement: TypingPassDefiningError from export solve_for_defining"),
                    Ok(c) => c,
                };

                match complete_define_solve.conclusions.get(&type_rune_t.rune) {
                    Some(ITemplataT::Kind(kt)) => {
                        coutputs.add_kind_export(
                            export.range,
                            kt.kind,
                            placeholdered_export_id,
                            export.exported_name,
                            self.typing_interner,
                        );
                    }
                    Some(_) => panic!("vimpl"),
                    None => panic!("vfail"),
                }
            }
        }

        // val (interfaceEdgeBlueprints, interfaceToSubCitizenToEdge) =
        //   Profiler.frame(() => { edgeCompiler.compileITables(coutputs) })
        let (interface_edge_blueprints, interface_to_sub_citizen_to_edge) =
            self.compile_i_tables(&mut coutputs);

        // Deferred function compilation loop
        // while (coutputs.peekNextDeferredFunctionBodyCompile().nonEmpty || coutputs.peekNextDeferredFunctionCompile().nonEmpty)
        while coutputs.peek_next_deferred_function_body_compile().is_some() || coutputs.peek_next_deferred_function_compile().is_some() {
            // while (coutputs.peekNextDeferredFunctionCompile().nonEmpty)
            while coutputs.peek_next_deferred_function_compile().is_some() {
                // val nextDeferredEvaluatingFunction = coutputs.peekNextDeferredFunctionCompile().get
                let next_deferred = coutputs.peek_next_deferred_function_compile().unwrap();
                match next_deferred {
                    DeferredActionT::EvaluateFunction {
                        name, calling_env, origin, template_args: _,
                    } => {
                        let name = *name;
                        let calling_env = *calling_env;
                        let origin: &'s FunctionA<'s> = origin;

                        // (nextDeferredEvaluatingFunction.call)(coutputs)
                        // delegate.evaluateGenericFunctionFromNonCallForHeader(
                        //   coutputs, parentRanges, callLocation, FunctionTemplataT(outerEnv, functionA))
                        let outer_env: IEnvironmentT<'s, 't> =
                            IEnvironmentT::from(calling_env);
                        let templata = FunctionTemplataT { outer_env, function: origin };
                        self.evaluate_generic_function_from_non_call_for_header(
                            &mut coutputs, &[], LocationInDenizen { path: &[] }, templata)?;

                        // coutputs.markDeferredFunctionCompiled(nextDeferredEvaluatingFunction.name)
                        coutputs.mark_deferred_function_compiled(name);
                    }
                    _ => panic!("vcurious: unexpected deferred action variant in function-compile loop"),
                }
            }
            // if (coutputs.peekNextDeferredFunctionBodyCompile().nonEmpty)
            if coutputs.peek_next_deferred_function_body_compile().is_some() {
                let next_deferred = coutputs.peek_next_deferred_function_body_compile().unwrap();
                match next_deferred {
                    DeferredActionT::EvaluateFunctionBody {
                        prototype, full_env_snapshot,
                        call_range, call_location, life,
                        attributes_t, params_t, is_destructor,
                        maybe_explicit_return_coord, instantiation_bound_params,
                    } => {
                        let prototype = *prototype;
                        let full_env_snapshot = *full_env_snapshot;
                        let call_range = *call_range;
                        let call_location = *call_location;
                        let life = *life;
                        let attributes_t = *attributes_t;
                        let params_t = *params_t;
                        let is_destructor = *is_destructor;
                        let maybe_explicit_return_coord = *maybe_explicit_return_coord;
                        let instantiation_bound_params = *instantiation_bound_params;

                        // (nextDeferredEvaluatingFunctionBody.call)(coutputs)
                        self.finish_function_maybe_deferred(
                            &mut coutputs, full_env_snapshot, call_range, call_location,
                            life, attributes_t, params_t, is_destructor,
                            maybe_explicit_return_coord, instantiation_bound_params)?;

                        // coutputs.markDeferredFunctionBodyCompiled(nextDeferredEvaluatingFunctionBody.prototypeT)
                        coutputs.mark_deferred_function_body_compiled(prototype);
                    }
                    _ => panic!("implement: unexpected deferred action type"),
                }
            }
        }

        // ensureDeepExports(coutputs)
        self.ensure_deep_exports(&mut coutputs)?;

        // val (reachableInterfaces, reachableStructs, reachableFunctions) =
        //   (coutputs.getAllInterfaces(), coutputs.getAllStructs(), coutputs.getAllFunctions())
        let reachable_interfaces = coutputs.get_all_interfaces();
        let reachable_structs = coutputs.get_all_structs();
        let reachable_functions = coutputs.get_all_functions();

        // interfaceEdgeBlueprints.groupBy(_.interface).mapValues(vassertOne(_))
        let mut interface_to_edge_blueprints: HashMap<IdT<'s, 't>, &'t InterfaceEdgeBlueprintT<'s, 't>> = HashMap::new();
        for blueprint in interface_edge_blueprints.iter() {
            let prev = interface_to_edge_blueprints.insert(blueprint.interface, blueprint);
            assert!(prev.is_none(), "vassertOne: multiple blueprints for same interface");
        }

        // coutputs.getInstantiationNameToFunctionBoundToRune()
        let raw_instantiation_bounds = coutputs.get_instantiation_name_to_function_bound_to_rune();
        let mut instantiation_name_to_instantiation_bounds: HashMap<IdT<'s, 't>, &'t InstantiationBoundArgumentsT<'s, 't>> = HashMap::new();
        for (id, bounds) in raw_instantiation_bounds.iter() {
            instantiation_name_to_instantiation_bounds.insert(*id, *bounds);
        }

        let hinputs = HinputsT {
            interfaces: reachable_interfaces,
            structs: reachable_structs,
            functions: reachable_functions.clone(),
            interface_to_edge_blueprints,
            interface_to_sub_citizen_to_edge,
            instantiation_name_to_instantiation_bounds,
            kind_exports: coutputs.get_kind_exports(),
            function_exports: coutputs.get_function_exports(),
            kind_externs: coutputs.get_kind_externs(),
            function_externs: coutputs.get_function_externs(),
            // sub_citizen_to_interface_to_edge will be populated by instantiator (Scala comment WPBI)
            sub_citizen_to_interface_to_edge: HashMap::new(),
        };

        // vassert(reachableFunctions.toVector.map(_.header.id).distinct.size == reachableFunctions.toVector.map(_.header.id).size)
        {
            let ids: Vec<_> = reachable_functions.iter().map(|f| f.header.id).collect();
            let distinct: std::collections::HashSet<_> = ids.iter().collect();
            assert!(ids.len() == distinct.len());
        }

        Ok(hinputs)
    }
/*
Guardian: temp-disable: TUCMPX — None is the legitimate Scala value here, not a fabricated default. Scala's FunctionCompilerCore.makeExternFunction computes maybeInheritance via pattern-match on functionA.containingFunction: case IdT(_,_,citizenTemplateName) => Some(...), case _ => None. This Rust call site is for export functions whose containing function is never a citizen template (lambdas/top-level), so the Scala equivalent always returns None for this branch. Panic was breaking lambda tests; None matches Scala behavior. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-744-1779425878790/hook-744/evaluate--1288.0.TodosAndUnimplementedCodeMustPanic-TUCMPX.TodosAndUnimplementedCodeMustPanic-TUCMPX.verdict.md
Guardian: temp-disable: SPDMX — This Rust call site has no Scala counterpart in Compiler.scala (Scala adds extern functions via FunctionCompilerCore.makeExternFunction, in a different file). The local audit-trail block shows phantom/stale Scala. The 5th arg matches the canonical addFunctionExtern signature in CompilerOutputs.scala which gained a genericParameterInheritance param. Caller passes panic! pending port of containingFunction-lookup logic. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-671-1779422748369/hook-671/evaluate--1288.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def evaluate(
      codeMap: FileCoordinateMap[String],
      packageToProgramA: PackageCoordinateMap[ProgramA]):
  Result[HinputsT, ICompileErrorT] = {
    try {
      Profiler.frame(() => {
        if (opts.globalOptions.verboseErrors) {
          println("Using overload index? " + opts.globalOptions.useOverloadIndex)
          println("Using sanity check? " + opts.globalOptions.sanityCheck)
        }

        val nameToStructDefinedMacro =
          Map(
            structConstructorMacro.macroName -> structConstructorMacro,
            structDropMacro.macroName -> structDropMacro)//,
//            structFreeMacro.macroName -> structFreeMacro,
//            implFreeMacro.macroName -> implFreeMacro)
        val nameToInterfaceDefinedMacro =
          Map(
            interfaceDropMacro.macroName -> interfaceDropMacro,
//            interfaceFreeMacro.macroName -> interfaceFreeMacro,
            anonymousInterfaceMacro.macroName -> anonymousInterfaceMacro)
        val nameToImplDefinedMacro = Map[StrI, IOnImplDefinedMacro]()
        val nameToFunctionBodyMacro =
          Map(
            abstractBodyMacro.generatorId -> abstractBodyMacro,
            structConstructorMacro.generatorId -> structConstructorMacro,
//            structFreeMacro.freeGeneratorId -> structFreeMacro,
            structDropMacro.dropGeneratorId -> structDropMacro,
            rsaLenMacro.generatorId -> rsaLenMacro,
            rsaMutNewMacro.generatorId -> rsaMutNewMacro,
            rsaImmNewMacro.generatorId -> rsaImmNewMacro,
            rsaPushMacro.generatorId -> rsaPushMacro,
            rsaPopMacro.generatorId -> rsaPopMacro,
            rsaCapacityMacro.generatorId -> rsaCapacityMacro,
            ssaLenMacro.generatorId -> ssaLenMacro,
            rsaDropMacro.generatorId -> rsaDropMacro,
            ssaDropMacro.generatorId -> ssaDropMacro,
            lockWeakMacro.generatorId -> lockWeakMacro,
            sameInstanceMacro.generatorId -> sameInstanceMacro,
            asSubtypeMacro.generatorId -> asSubtypeMacro)

        val idAndEnvEntry: Vector[(IdT[INameT], IEnvEntry)] =
          packageToProgramA.flatMap({ case (coord, programA) =>
            val packageName = IdT(coord, Vector(), interner.intern(PackageTopLevelNameT()))
            programA.structs.map(structA => {
              val structNameT = packageName.addStep(nameTranslator.translateNameStep(structA.name))
              Vector((structNameT, StructEnvEntry(structA))) ++
                preprocessStruct(nameToStructDefinedMacro, structNameT, structA)
            }) ++
            programA.interfaces.map(interfaceA => {
              val interfaceNameT = packageName.addStep(nameTranslator.translateNameStep(interfaceA.name))
              Vector((interfaceNameT, InterfaceEnvEntry(interfaceA))) ++
                preprocessInterface(nameToInterfaceDefinedMacro, interfaceNameT, interfaceA)
            }) ++
            programA.impls.map(implA => {
              val implNameT = packageName.addStep(nameTranslator.translateImplName(implA.name))
              Vector((implNameT, ImplEnvEntry(implA)))
            }) ++
            programA.functions.map(functionA => {
              val functionNameT = packageName.addStep(nameTranslator.translateGenericFunctionName(functionA.name))
              Vector((functionNameT, FunctionEnvEntry(functionA)))
            })
          }).flatten.flatten.toVector

        val namespaceNameToTemplatas =
          idAndEnvEntry
            .map({ case (name, envEntry) =>
              (name.copy(localName = interner.intern(PackageTopLevelNameT())), name.localName, envEntry)
            })
            .groupBy(_._1)
            .map({ case (packageId, envEntries) =>
              packageId ->
                TemplatasStore(packageId, Map(), Map())
                  .addEntries(interner, envEntries.map({ case (_, b, c) => (b, c) }))
             }).toMap

        val globalEnv =
          GlobalEnvironment(
            functorHelper,
            structConstructorMacro,
            structDropMacro,
//            structFreeMacro,
            interfaceDropMacro,
//            interfaceFreeMacro,
            anonymousInterfaceMacro,
            nameToStructDefinedMacro,
            nameToInterfaceDefinedMacro,
            nameToImplDefinedMacro,
            nameToFunctionBodyMacro,
            namespaceNameToTemplatas,
            // Bulitins
            env.TemplatasStore(IdT(PackageCoordinate.BUILTIN(interner, keywords), Vector(), interner.intern(PackageTopLevelNameT())), Map(), Map()).addEntries(
              interner,
              Vector[(INameT, IEnvEntry)](
                interner.intern(PrimitiveNameT(keywords.int)) -> TemplataEnvEntry(KindTemplataT(IntT.i32)),
                interner.intern(PrimitiveNameT(keywords.i64)) -> TemplataEnvEntry(KindTemplataT(IntT.i64)),
                interner.intern(PrimitiveNameT(keywords.Array)) -> TemplataEnvEntry(RuntimeSizedArrayTemplateTemplataT()),
                interner.intern(PrimitiveNameT(keywords.StaticArray)) -> TemplataEnvEntry(StaticSizedArrayTemplateTemplataT()),
                interner.intern(PrimitiveNameT(keywords.bool)) -> TemplataEnvEntry(KindTemplataT(BoolT())),
                interner.intern(PrimitiveNameT(keywords.float)) -> TemplataEnvEntry(KindTemplataT(FloatT())),
                interner.intern(PrimitiveNameT(keywords.__Never)) -> TemplataEnvEntry(KindTemplataT(NeverT(false))),
                interner.intern(PrimitiveNameT(keywords.str)) -> TemplataEnvEntry(KindTemplataT(StrT())),
                interner.intern(PrimitiveNameT(keywords.void)) -> TemplataEnvEntry(KindTemplataT(VoidT())))))

        val coutputs = CompilerOutputs()

//        val emptyTupleStruct =
//          sequenceCompiler.makeTupleKind(
//            PackageEnvironment.makeTopLevelEnvironment(
//              globalEnv, FullNameT(PackageCoordinate.BUILTIN, Vector(), PackageTopLevelNameT())),
//            coutputs,
//            Vector())
//        val emptyTupleStructRef =
//          sequenceCompiler.makeTupleCoord(
//            PackageEnvironment.makeTopLevelEnvironment(
//              globalEnv, FullNameT(PackageCoordinate.BUILTIN, Vector(), PackageTopLevelNameT())),
//            coutputs,
//            Vector())

        arrayCompiler.compileStaticSizedArray(globalEnv, coutputs)
        arrayCompiler.compileRuntimeSizedArray(globalEnv, coutputs)


        // Indexing phase

        opts.debugOut("Starting indexing phase.")

        globalEnv.nameToTopLevelEnvironment.foreach({ case (packageId, templatas) =>
          val env = PackageEnvironmentT.makeTopLevelEnvironment(globalEnv, packageId)
          templatas.entriesByNameT.map({ case (name, entry) =>
            entry match {
              case StructEnvEntry(structA) => {
                val templata = StructDefinitionTemplataT(env, structA)
                structCompiler.precompileStruct(coutputs, templata)
              }
              case InterfaceEnvEntry(interfaceA) => {
                val templata = InterfaceDefinitionTemplataT(env, interfaceA)
                structCompiler.precompileInterface(coutputs, templata)
              }
              case _ =>
            }
          })
        })

        // Compiling phase

        opts.debugOut("Starting struct/interface compiling phase.")

        val uncheckedDefiningConclusionses =
          globalEnv.nameToTopLevelEnvironment.toArray.flatMap({ case (packageId, templatas) =>
            val packageEnv = PackageEnvironmentT.makeTopLevelEnvironment(globalEnv, packageId)
            // This makes it so anything starting with an underscore is compiled in the order
            // of their names.
            // AFTERM: is there a better solution here? should we always order things?
            val (orderableEntries, unorderedEntries) =
                U.filterOut[(INameT, IEnvEntry), (CitizenTemplateNameT, IEnvEntry)](
                  templatas.entriesByNameT.toArray,
                  {
                    case (c @ CitizenTemplateNameT(s), entry) if s.str.startsWith("_") =>
                      (c, entry)
                  })
            val orderedEntries = orderableEntries.sortBy(_._1.humanName.str)
            def inner(entry: IEnvEntry): Option[UncheckedDefiningConclusions] = {
              entry match {
                case StructEnvEntry(structA) => {
                  opts.debugOut("Compiling struct:" + PostParserErrorHumanizer.humanizeName(structA.name))

                  val templata = StructDefinitionTemplataT(packageEnv, structA)
                  val uncheckedConclusions =
                      structCompiler.compileStruct(
                        coutputs, List(), LocationInDenizen(Vector()), templata)

                  val maybeExport =
                    structA.attributes.collectFirst { case e@ExportS(_) => e }
                  maybeExport match {
                    case None =>
                    case Some(ExportS(packageCoordinate)) => {
                      val templateName = interner.intern(ExportTemplateNameT(structA.range.begin))
                      val templateId = IdT(packageId.packageCoord, Vector(), templateName)
                      val exportOuterEnv =
                        ExportEnvironmentT(
                          globalEnv, packageEnv, templateId, templateId, TemplatasStore(templateId, Map(), Map()))

                      val regionPlaceholder = RegionT(DefaultRegionT)

                      val placeholderedExportName = interner.intern(ExportNameT(templateName, RegionT(DefaultRegionT)))
                      val placeholderedExportId = templateId.copy(localName = placeholderedExportName)
                      val exportEnv =
                        ExportEnvironmentT(
                          globalEnv, packageEnv, templateId, placeholderedExportId, TemplatasStore(placeholderedExportId, Map(), Map()))

                      val exportPlaceholderedStruct =
                        structCompiler.resolveStruct(
                          coutputs, exportEnv, List(structA.range), LocationInDenizen(Vector()), templata, Vector()) match {
                          case ResolveSuccess(kind) => kind
                          case ResolveFailure(range, reason) => {
                            throw CompileErrorExceptionT(TypingPassResolvingError(range, reason))
                          }
                        }

                      val exportName =
                        structA.name match {
                          case TopLevelCitizenDeclarationNameS(name, range) => name
                          case other => vwat(other)
                        }

                      coutputs.addKindExport(
                        structA.range, exportPlaceholderedStruct, placeholderedExportId, exportName)
                    }
                  }

                  Some(uncheckedConclusions)
                }
                case InterfaceEnvEntry(interfaceA) => {
                  opts.debugOut("Compiling interface:" + PostParserErrorHumanizer.humanizeName(interfaceA.name))

                  val templata = InterfaceDefinitionTemplataT(packageEnv, interfaceA)
                  val uncheckedConclusions =
                      structCompiler.compileInterface(
                        coutputs, List(), LocationInDenizen(Vector()), templata)

                  val maybeExport =
                    interfaceA.attributes.collectFirst { case e@ExportS(_) => e }
                  maybeExport match {
                    case None =>
                    case Some(ExportS(packageCoordinate)) => {
                      val templateName = interner.intern(ExportTemplateNameT(interfaceA.range.begin))
                      val templateId = IdT(packageId.packageCoord, Vector(), templateName)
                      val exportOuterEnv =
                        ExportEnvironmentT(
                          globalEnv, packageEnv, templateId, templateId, TemplatasStore(templateId, Map(), Map()))

                      val placeholderedExportName = interner.intern(ExportNameT(templateName, RegionT(DefaultRegionT)))
                      val placeholderedExportId = templateId.copy(localName = placeholderedExportName)
                      val exportEnv =
                        ExportEnvironmentT(
                          globalEnv, packageEnv, templateId, placeholderedExportId, TemplatasStore(placeholderedExportId, Map(), Map()))

                      val exportPlaceholderedKind =
                        structCompiler.resolveInterface(
                          coutputs, exportEnv, List(interfaceA.range), LocationInDenizen(Vector()), templata, Vector()) match {
                          case ResolveSuccess(kind) => kind
                          case ResolveFailure(range, reason) => {
                            throw CompileErrorExceptionT(TypingPassResolvingError(range, reason))
                          }
                        }

                      val exportName =
                        interfaceA.name match {
                          case TopLevelCitizenDeclarationNameS(name, range) => name
                          case other => vwat(other)
                        }

                      coutputs.addKindExport(
                        interfaceA.range, exportPlaceholderedKind, placeholderedExportId, exportName)
                    }
                  }

                  Some(uncheckedConclusions)
                }
                case _ => None
              }
            }
            (U.mapArr[(CitizenTemplateNameT, IEnvEntry), Option[UncheckedDefiningConclusions]](
              orderedEntries, { case (name, entry) => inner(entry) }).toVector.flatten ++
            U.mapArr[(INameT, IEnvEntry), Option[UncheckedDefiningConclusions]](
              unorderedEntries, { case (name, entry) => inner(entry) }).toVector.flatten)
          })

        opts.debugOut("Starting struct/interface resolution phase.")

        U.foreachArr[UncheckedDefiningConclusions](uncheckedDefiningConclusionses, {
          case UncheckedDefiningConclusions(envs, ranges, callLocation, definitionRules, conclusions) => {
            val instantiationBoundArgsUNUSED =
              inferCompiler.checkDefiningConclusionsAndResolve(
                envs, coutputs, ranges, callLocation, definitionRules, Vector(), conclusions) match {
                case Err(f) => throw CompileErrorExceptionT(TypingPassDefiningError(ranges, DefiningResolveConclusionError(f)))
                case Ok(c) => c
              }
            // We don't care about these, we just wanted things to be added to the coutputs.
            val _ = instantiationBoundArgsUNUSED
          }
        })

        opts.debugOut("Starting impl compile phase.")

        globalEnv.nameToTopLevelEnvironment.foreach({ case (packageId, templatas) =>
          val env = PackageEnvironmentT.makeTopLevelEnvironment(globalEnv, packageId)
          templatas.entriesByNameT.map({ case (name, entry) =>
            entry match {
              case ImplEnvEntry(impl) => {
                opts.debugOut("Compiling impl: " + PostParserErrorHumanizer.humanizeName(impl.name))

                implCompiler.compileImpl(coutputs, LocationInDenizen(Vector()), ImplDefinitionTemplataT(env, impl))
              }
              case _ =>
            }
          })
        })

        opts.debugOut("Starting function compile phase.")

        globalEnv.nameToTopLevelEnvironment.foreach({
          // Anything in global scope should be compiled
          case (packageId @ IdT(_, Vector(), PackageTopLevelNameT()), templatas) => {
            val packageEnv = PackageEnvironmentT.makeTopLevelEnvironment(globalEnv, packageId)
            templatas.entriesByNameT.map({ case (name, entry) =>
              entry match {
                case FunctionEnvEntry(functionA) => {
                  opts.debugOut("Compiling function: " + PostParserErrorHumanizer.humanizeName(functionA.name))

                  val templata = FunctionTemplataT(packageEnv, functionA)
                  val header =
                  functionCompiler.evaluateGenericFunctionFromNonCall(
                      coutputs, List(), LocationInDenizen(Vector()), templata)

                  val maybeExport =
                    functionA.attributes.collectFirst { case e@ExportS(_) => e }
                  maybeExport match {
                    case None =>
                    case Some(ExportS(packageCoordinate)) => {
                      val templateName = interner.intern(ExportTemplateNameT(functionA.range.begin))
                      val templateId = IdT(packageId.packageCoord, Vector(), templateName)
                      val exportOuterEnv =
                        ExportEnvironmentT(
                          globalEnv, packageEnv, templateId, templateId, TemplatasStore(templateId, Map(), Map()))

                      val regionPlaceholder = RegionT(DefaultRegionT)

                      val placeholderedExportName = interner.intern(ExportNameT(templateName, regionPlaceholder))
                      val placeholderedExportId = templateId.copy(localName = placeholderedExportName)
                      val exportEnv =
                        ExportEnvironmentT(
                          globalEnv, packageEnv, templateId, placeholderedExportId, TemplatasStore(placeholderedExportId, Map(), Map()))

                      val exportPlaceholderedPrototype =
                        functionCompiler.evaluateGenericLightFunctionFromCallForPrototype(
                          coutputs, List(functionA.range), LocationInDenizen(Vector()), exportEnv, templata, Vector(), regionPlaceholder, Vector()) match {
                          case ResolveFunctionSuccess(prototype, inferences) => prototype.prototype
                          case ResolveFunctionFailure(reason) => {
                            throw CompileErrorExceptionT(TypingPassResolvingError(List(functionA.range), reason))
                          }
                        }

                      val exportName =
                        functionA.name match {
                          case FunctionNameS(name, range) => name
                          case other => vwat(other)
                        }

                      coutputs.addFunctionExport(
                        functionA.range, exportPlaceholderedPrototype, placeholderedExportId, exportName)
                    }
                  }
                }
                case _ =>
              }
            })
          }
          // Anything underneath something else should be skipped, we'll evaluate those later on.
          case (IdT(_, anythingElse, PackageTopLevelNameT()), _) =>
        })

        opts.debugOut("Starting export compile phase.")

        packageToProgramA.flatMap({ case (packageCoord, programA) =>
          val packageEnv =
            PackageEnvironmentT.makeTopLevelEnvironment(
              globalEnv, IdT(packageCoord, Vector(), interner.intern(PackageTopLevelNameT())))

          programA.exports.foreach({ case ExportAsA(range, exportedName, rules, runeToType, typeRuneA) =>

            opts.debugOut("Compiling export: " + exportedName.str)

            val typeRuneT = typeRuneA

            val templateName = interner.intern(ExportTemplateNameT(range.begin))
            val templateId = IdT(packageCoord, Vector(), templateName)
            val exportOuterEnv =
              ExportEnvironmentT(
                globalEnv, packageEnv, templateId, templateId, TemplatasStore(templateId, Map(), Map()))

            val regionPlaceholder = RegionT(DefaultRegionT)

            val placeholderedExportName = interner.intern(ExportNameT(templateName, regionPlaceholder))
            val placeholderedExportId = templateId.copy(localName = placeholderedExportName)
            val exportEnv =
              ExportEnvironmentT(
                globalEnv, packageEnv, templateId, placeholderedExportId, TemplatasStore(placeholderedExportId, Map(), Map()))

            val CompleteDefineSolve(templataByRune, _) =
              inferCompiler.solveForDefining(
                InferEnv(exportEnv, List(range), LocationInDenizen(Vector()), exportEnv, regionPlaceholder),
                coutputs, rules, runeToType, List(range),
                LocationInDenizen(Vector()), Vector(), Vector(), Vector()) match {
                case Err(f) => throw CompileErrorExceptionT(TypingPassDefiningError(List(range), f))
                case Ok(c) => c
              }
              templataByRune.get(typeRuneT.rune) match {
                case Some(KindTemplataT(kind)) => {
                coutputs.addKindExport(range, kind, placeholderedExportId, exportedName)
                }
                case Some(prototype) => {
                  vimpl()
                }
                case _ => vfail()
              }
          })
        })

//        breakable {
//          while (true) {
        val denizensAtStart = coutputs.countDenizens()

        val builtinPackageCoord = PackageCoordinate.BUILTIN(interner, keywords)
        val rootPackageEnv =
          PackageEnvironmentT.makeTopLevelEnvironment(
            globalEnv,
            IdT(builtinPackageCoord, Vector(), interner.intern(PackageTopLevelNameT())))

//        val freeImpreciseName = interner.intern(FreeImpreciseNameS())
//        val dropImpreciseName = interner.intern(CodeNameS(keywords.drop))

//        val immutableKinds =
//          coutputs.getAllStructs().filter(_.mutability == MutabilityTemplata(ImmutableT)).map(_.templateName) ++
//            coutputs.getAllInterfaces().filter(_.mutability == ImmutableT).map(_.templateName) ++
//            coutputs.getAllRuntimeSizedArrays().filter(_.mutability == MutabilityTemplata(ImmutableT)) ++
//            coutputs.getAllStaticSizedArrays().filter(_.mutability == MutabilityTemplata(ImmutableT))
//        immutableKinds.foreach(kind => {
//          val kindEnv = coutputs.getEnvForTemplate(kind)
//          functionCompiler.evaluateGenericFunctionFromNonCall(
//            coutputs,
//            kindEnv.lookupNearestWithImpreciseName(freeImpreciseName, Set(ExpressionLookupContext)) match {
//              case Some(ft@FunctionTemplata(_, _)) => ft
//              case _ => throw CompileErrorExceptionT(RangedInternalErrorT(RangeS.internal(interner, -1663), "Couldn't find free for immutable struct!"))
//            })
//          functionCompiler.evaluateGenericFunctionFromNonCall(
//            coutputs,
//            kindEnv.lookupNearestWithImpreciseName(dropImpreciseName, Set(ExpressionLookupContext)) match {
//              case Some(ft@FunctionTemplata(_, _)) => ft
//              case _ => throw CompileErrorExceptionT(RangedInternalErrorT(RangeS.internal(interner, -1663), "Couldn't find free for immutable struct!"))
//            })
//        })

        val (interfaceEdgeBlueprints, interfaceToSubCitizenToEdge) =
          Profiler.frame(() => {
//                val env =
//                  PackageEnvironment.makeTopLevelEnvironment(
//                    globalEnv, FullNameT(PackageCoordinate.BUILTIN, Vector(), interner.intern(PackageTopLevelNameT())))

                // Returns the number of overrides stamped
                // This doesnt actually stamp *all* overrides, just the ones we can immediately
                // see missing. We don't know if, in the process of stamping these, we'll find more.
                // Also note, these don't stamp them right now, they defer them for later evaluating.
            edgeCompiler.compileITables(coutputs)
          })

        while (coutputs.peekNextDeferredFunctionBodyCompile().nonEmpty || coutputs.peekNextDeferredFunctionCompile().nonEmpty) {
          while (coutputs.peekNextDeferredFunctionCompile().nonEmpty) {
            val nextDeferredEvaluatingFunction = coutputs.peekNextDeferredFunctionCompile().get
            opts.debugOut("Compiling deferred: " + CompilerErrorHumanizer.humanizeName(SourceCodeUtils.humanizePos(codeMap, _), nextDeferredEvaluatingFunction.name.localName, None))

            // No, IntelliJ, I assure you this has side effects
            (nextDeferredEvaluatingFunction.call) (coutputs)
            coutputs.markDeferredFunctionCompiled(nextDeferredEvaluatingFunction.name)
          }

          // No particular reason for this if/while mismatch, it just feels a bit better to get started on more before
          // we finish any.
          if (coutputs.peekNextDeferredFunctionBodyCompile().nonEmpty) {
            val nextDeferredEvaluatingFunctionBody = coutputs.peekNextDeferredFunctionBodyCompile().get
            opts.debugOut("Compiling deferred: " + CompilerErrorHumanizer.humanizeName(SourceCodeUtils.humanizePos(codeMap, _), nextDeferredEvaluatingFunctionBody.prototypeT.id.localName, None))

            // No, IntelliJ, I assure you this has side effects
            (nextDeferredEvaluatingFunctionBody.call) (coutputs)
            coutputs.markDeferredFunctionBodyCompiled(nextDeferredEvaluatingFunctionBody.prototypeT)
          }
        }


//            val denizensAtEnd = coutputs.countDenizens()
//            if (denizensAtStart == denizensAtEnd)
//              break
//          }
//        }

//        // NEVER ZIP TWO SETS TOGETHER
//        val edgeBlueprintsAsList = edgeBlueprints.toVector
//        val interfaceToEdgeBlueprints = edgeBlueprintsAsList.map(_.interface).zip(edgeBlueprintsAsList).toMap;
//
//        interfaceToEdgeBlueprints.foreach({ case (interfaceTT, edgeBlueprint) =>
//          vassert(edgeBlueprint.interface == interfaceTT)
//        })

        ensureDeepExports(coutputs)

        val (
          reachableInterfaces,
          reachableStructs,
//          reachableSSAs,
//          reachableRSAs,
          reachableFunctions) =
//        if (opts.treeShakingEnabled) {
//          Profiler.frame(() => {
//            val reachables = Reachability.findReachables(coutputs, interfaceEdgeBlueprints, interfaceToStructToMethods)
//
//            val categorizedFunctions = coutputs.getAllFunctions().groupBy(f => reachables.functions.contains(f.header.toSignature))
//            val reachableFunctions = categorizedFunctions.getOrElse(true, Vector.empty)
//            val unreachableFunctions = categorizedFunctions.getOrElse(false, Vector.empty)
//            unreachableFunctions.foreach(f => debugOut("Shaking out unreachable: " + f.header.fullName))
//            reachableFunctions.foreach(f => debugOut("Including: " + f.header.fullName))
//
//            val categorizedSSAs = coutputs.getAllStaticSizedArrays().groupBy(f => reachables.staticSizedArrays.contains(f))
//            val reachableSSAs = categorizedSSAs.getOrElse(true, Vector.empty)
//            val unreachableSSAs = categorizedSSAs.getOrElse(false, Vector.empty)
//            unreachableSSAs.foreach(f => debugOut("Shaking out unreachable: " + f))
//            reachableSSAs.foreach(f => debugOut("Including: " + f))
//
//            val categorizedRSAs = coutputs.getAllRuntimeSizedArrays().groupBy(f => reachables.runtimeSizedArrays.contains(f))
//            val reachableRSAs = categorizedRSAs.getOrElse(true, Vector.empty)
//            val unreachableRSAs = categorizedRSAs.getOrElse(false, Vector.empty)
//            unreachableRSAs.foreach(f => debugOut("Shaking out unreachable: " + f))
//            reachableRSAs.foreach(f => debugOut("Including: " + f))
//
//            val categorizedStructs = coutputs.getAllStructs().groupBy(f => reachables.structs.contains(f.getRef))
//            val reachableStructs = categorizedStructs.getOrElse(true, Vector.empty)
//            val unreachableStructs = categorizedStructs.getOrElse(false, Vector.empty)
//            unreachableStructs.foreach(f => {
//              debugOut("Shaking out unreachable: " + f.fullName)
//            })
//            reachableStructs.foreach(f => debugOut("Including: " + f.fullName))
//
//            val categorizedInterfaces = coutputs.getAllInterfaces().groupBy(f => reachables.interfaces.contains(f.getRef))
//            val reachableInterfaces = categorizedInterfaces.getOrElse(true, Vector.empty)
//            val unreachableInterfaces = categorizedInterfaces.getOrElse(false, Vector.empty)
//            unreachableInterfaces.foreach(f => debugOut("Shaking out unreachable: " + f.fullName))
//            reachableInterfaces.foreach(f => debugOut("Including: " + f.fullName))
//
//            val categorizedEdges =
//              edges.groupBy(f => reachables.edges.contains(f))
//            val reachableEdges = categorizedEdges.getOrElse(true, Vector.empty)
//            val unreachableEdges = categorizedEdges.getOrElse(false, Vector.empty)
//            unreachableEdges.foreach(f => debugOut("Shaking out unreachable: " + f))
//            reachableEdges.foreach(f => debugOut("Including: " + f))
//
//            (reachableInterfaces, reachableStructs, reachableSSAs, reachableRSAs, reachableFunctions)
//          })
//        } else {
          (
            coutputs.getAllInterfaces(),
            coutputs.getAllStructs(),
//            coutputs.getAllStaticSizedArrays(),
//            coutputs.getAllRuntimeSizedArrays(),
            coutputs.getAllFunctions())
//        }

//      val allKinds =
//        reachableStructs.map(_.place) ++ reachableInterfaces.map(_.getRef) ++ reachableSSAs ++ reachableRSAs
//      val reachableImmKinds: Vector[KindT] =
//        allKinds
//          .filter({
//            case s@StructTT(_) => coutputs.lookupMutability(s) == ImmutableT
//            case i@InterfaceTT(_) => coutputs.lookupMutability(i) == ImmutableT
//            case contentsStaticSizedArrayTT(_, m, _, _) => m == ImmutableT
//            case contentsRuntimeSizedArrayTT(m, _) => m == ImmutableT
//            case _ => true
//          })
//          .toVector
//      val reachableImmKindToDestructor = reachableImmKinds.zip(reachableImmKinds.map(coutputs.findImmDestructor)).toMap

      val hinputs =
          vale.typing.HinputsT(
            reachableInterfaces.toVector,
            reachableStructs.toVector,
            reachableFunctions.toVector,
//            Map(), // Will be populated by instantiator
            interfaceEdgeBlueprints.groupBy(_.interface).mapValues(vassertOne(_)),
            interfaceToSubCitizenToEdge,
            coutputs.getInstantiationNameToFunctionBoundToRune(),
            coutputs.getKindExports,
            coutputs.getFunctionExports,
            coutputs.getKindExterns,
            coutputs.getFunctionExterns)

        vassert(reachableFunctions.toVector.map(_.header.id).distinct.size == reachableFunctions.toVector.map(_.header.id).size)

        Ok(hinputs)
      })
    } catch {
      case CompileErrorExceptionT(err) => Err(err)
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn preprocess_struct(
        &self,
        name_to_struct_defined_macro: &HashMap<StrI<'s>, OnStructDefinedMacro>,
        struct_name_t: IdT<'s, 't>,
        struct_a: &'s StructA<'s>,
    ) -> Vec<(&'t IdT<'s, 't>, IEnvEntryT<'s, 't>)> {

        let macro1 = self.scout_arena.alloc(MacroCallS {
            range: struct_a.range,
            include: IMacroInclusionP::CallMacro,
            macro_name: self.keywords.derive_struct_constructor,
        }) as &'s MacroCallS<'s>;
        let macro2 = self.scout_arena.alloc(MacroCallS {
            range: struct_a.range,
            include: IMacroInclusionP::CallMacro,
            macro_name: self.keywords.derive_struct_drop,
        }) as &'s MacroCallS<'s>;
        let default_called_macros = [macro1, macro2];
        let attr_refs: Vec<&'s ICitizenAttributeS<'s>> = struct_a.attributes.iter().collect();
        let macros_to_call = self.determine_macros_to_call(
            name_to_struct_defined_macro,
            &default_called_macros[..],
            &[struct_a.range],
            &attr_refs,
        );
        let mut result = Vec::new();
        for macro_ in macros_to_call {
            for (id, entry) in macro_.get_struct_sibling_entries(self, struct_name_t, struct_a) {
                let id_val = IdValT { package_coord: id.package_coord, init_steps: id.init_steps, local_name: id.local_name };
                result.push((self.typing_interner.intern_id(id_val), entry));
            }
        }
        result
    }
    /*
      private def preprocessStruct(
        nameToStructDefinedMacro: Map[StrI, IOnStructDefinedMacro],
        structNameT: IdT[INameT],
        structA: StructA): Vector[(IdT[INameT], IEnvEntry)] = {
        val defaultCalledMacros =
          Vector(
            MacroCallS(structA.range, CallMacroP, keywords.DeriveStructConstructor),
            MacroCallS(structA.range, CallMacroP, keywords.DeriveStructDrop))//,
    //        MacroCallS(structA.range, CallMacroP, keywords.DeriveStructFree),
    //        MacroCallS(structA.range, CallMacroP, keywords.DeriveImplFree))
        determineMacrosToCall(nameToStructDefinedMacro, defaultCalledMacros, List(structA.range), structA.attributes)
          .flatMap(_.getStructSiblingEntries(structNameT, structA))
      }

    */
}
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn preprocess_interface(
        &self,
        name_to_interface_defined_macro: &HashMap<StrI<'s>, OnInterfaceDefinedMacro>,
        _interface_name_t: IdT<'s, 't>,
        interface_a: &'s InterfaceA<'s>,
    ) -> Vec<(&'t IdT<'s, 't>, IEnvEntryT<'s, 't>)> {

        let macro1 = self.scout_arena.alloc(MacroCallS {
            range: interface_a.range,
            include: IMacroInclusionP::CallMacro,
            macro_name: self.keywords.derive_interface_drop,
        }) as &'s MacroCallS<'s>;
        let macro2 = self.scout_arena.alloc(MacroCallS {
            range: interface_a.range,
            include: IMacroInclusionP::CallMacro,
            macro_name: self.keywords.derive_anonymous_substruct,
        }) as &'s MacroCallS<'s>;
        let default_called_macros = [macro1, macro2];
        let attr_refs: Vec<&'s ICitizenAttributeS<'s>> = interface_a.attributes.iter().collect();
        let macros_to_call = self.determine_macros_to_call(
            name_to_interface_defined_macro,
            &default_called_macros[..],
            &[interface_a.range],
            &attr_refs,
        );
        let mut result = Vec::new();
        for macro_ in macros_to_call {
            for (id, entry) in macro_.get_interface_sibling_entries(self, _interface_name_t, interface_a) {
                let id_val = IdValT { package_coord: id.package_coord, init_steps: id.init_steps, local_name: id.local_name };
                result.push((self.typing_interner.intern_id(id_val), entry));
            }
        }
        result
    }
    /*
      private def preprocessInterface(
        nameToInterfaceDefinedMacro: Map[StrI, IOnInterfaceDefinedMacro],
        interfaceNameT: IdT[INameT],
        interfaceA: InterfaceA):
      Vector[(IdT[INameT], IEnvEntry)] = {
        val defaultCalledMacros =
          Vector(
            MacroCallS(interfaceA.range, CallMacroP, keywords.DeriveInterfaceDrop),
    //        MacroCallS(interfaceA.range, CallMacroP, keywords.DeriveInterfaceFree),
            MacroCallS(interfaceA.range, CallMacroP, keywords.DeriveAnonymousSubstruct))
        val macrosToCall =
          determineMacrosToCall(nameToInterfaceDefinedMacro, defaultCalledMacros, List(interfaceA.range), interfaceA.attributes)
        vpass()
        val results =
          macrosToCall.flatMap(_.getInterfaceSiblingEntries(interfaceNameT, interfaceA))
        vpass()
        results
      }

    */
}
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn determine_macros_to_call<T: Clone>(
        &self,
        name_to_macro: &HashMap<StrI<'s>, T>,
        default_called_macros: &[&'s MacroCallS<'s>],
        parent_ranges: &[RangeS<'s>],
        attributes: &[&'s ICitizenAttributeS<'s>],
    ) -> Vec<T> {
        let macros_to_call: Vec<&'s MacroCallS<'s>> =
            attributes.iter().fold(default_called_macros.to_vec(), |macros_to_call, attr| {
                match attr {
                    ICitizenAttributeS::MacroCall(mc) if mc.include == IMacroInclusionP::CallMacro => {
                        if macros_to_call.iter().any(|m| m.macro_name == mc.macro_name) {
                            panic!("Calling macro twice: {:?}", mc.macro_name);
                        }
                        let mut result = macros_to_call;
                        result.push(mc);
                        result
                    }
                    ICitizenAttributeS::MacroCall(mc) if mc.include == IMacroInclusionP::DontCallMacro => {
                        macros_to_call.into_iter().filter(|m| m.macro_name != mc.macro_name).collect()
                    }
                    _ => macros_to_call,
                }
            });
        macros_to_call.into_iter().map(|macro_call| {
            match name_to_macro.get(&macro_call.macro_name) {
                None => panic!("Macro not found: {:?}", macro_call.macro_name),
                Some(m) => m.clone(),
            }
        }).collect()
    }
    /*
      private def determineMacrosToCall[T](
        nameToMacro: Map[StrI, T],
        defaultCalledMacros: Vector[MacroCallS],
        parentRanges: List[RangeS],
        attributes: Vector[ICitizenAttributeS]):
      Vector[T] = {
        attributes.foldLeft(defaultCalledMacros)({
          case (macrosToCall, mc@MacroCallS(range, CallMacroP, macroName)) => {
            if (macrosToCall.exists(_.macroName == macroName)) {
              throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Calling macro twice: " + macroName))
            }
            macrosToCall :+ mc
          }
          case (macrosToCall, MacroCallS(_, DontCallMacroP, macroName)) => macrosToCall.filter(_.macroName != macroName)
          case (macrosToCall, _) => macrosToCall
        }).map(macroCall => {
          nameToMacro.get(macroCall.macroName) match {
            case None => {
              throw CompileErrorExceptionT(RangedInternalErrorT(macroCall.range :: parentRanges, "Macro not found: " + macroCall.macroName))
            }
            case Some(m) => m
          }
        })
      }

    */
}
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn ensure_deep_exports(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
    ) -> Result<(), ICompileErrorT<'s, 't>> {
        // val packageToKindToExport =
        //   coutputs.getKindExports
        //     .map(kindExport => (kindExport.id.packageCoord, kindExport.tyype, kindExport))
        //     .groupBy(_._1)
        //     .mapValues(
        //       _.map(x => (x._2, x._3))
        //         .groupBy(_._1)
        //         .mapValues({
        //           case Vector() => vwat()
        //           case Vector(only) => only
        //           case multiple => throw CompileErrorExceptionT(TypeExportedMultipleTimes(...))
        //         }))
        let kind_export_triples: Vec<(&'s PackageCoordinate<'s>, KindT<'s, 't>, &'t KindExportT<'s, 't>)> =
            coutputs.get_kind_exports().iter()
                .map(|ke| (ke.id.package_coord, ke.tyype, *ke))
                .collect();
        // Per @IIIOZ: IndexMap so iteration at the package/kind loops below is deterministic.
        // Upstream kind_export_triples is from coutputs.get_kind_exports() (Vec, deterministic).
        let mut grouped_by_package: IndexMap<&'s PackageCoordinate<'s>, Vec<(KindT<'s, 't>, &'t KindExportT<'s, 't>)>> = IndexMap::new();
        for (pc, k, ke) in kind_export_triples.into_iter() {
            grouped_by_package.entry(pc).or_insert_with(Vec::new).push((k, ke));
        }
        let package_to_kind_to_export: IndexMap<&'s PackageCoordinate<'s>, IndexMap<KindT<'s, 't>, &'t KindExportT<'s, 't>>> = {
            let mut result: IndexMap<&'s PackageCoordinate<'s>, IndexMap<KindT<'s, 't>, &'t KindExportT<'s, 't>>> = IndexMap::new();
            for (pc, kind_pairs) in grouped_by_package.into_iter() {
                let mut grouped_by_kind: IndexMap<KindT<'s, 't>, Vec<&'t KindExportT<'s, 't>>> = IndexMap::new();
                for (k, ke) in kind_pairs.into_iter() {
                    grouped_by_kind.entry(k).or_insert_with(Vec::new).push(ke);
                }
                let mut inner: IndexMap<KindT<'s, 't>, &'t KindExportT<'s, 't>> = IndexMap::new();
                for (k, exports) in grouped_by_kind.into_iter() {
                    let only = match exports.as_slice() {
                        [] => panic!("vwat"),
                        [only] => *only,
                        _ => {
                            let exports_copies: Vec<KindExportT<'s, 't>> = exports.iter().map(|ke| KindExportT {
                                range: ke.range,
                                tyype: ke.tyype,
                                id: ke.id,
                                exported_name: ke.exported_name,
                            }).collect();
                            let exports_slice = self.typing_interner.alloc_slice_from_vec(exports_copies);
                            let range_slice = self.typing_interner.alloc_slice_copy(&[exports[0].range]);
                            return Err(ICompileErrorT::TypeExportedMultipleTimes {
                                range: range_slice,
                                paackage: *exports[0].id.package_coord,
                                exports: exports_slice,
                            });
                        }
                    };
                    inner.insert(k, only);
                }
                result.insert(pc, inner);
            }
            result
        };

        // coutputs.getFunctionExports.foreach(funcExport => {
        //   val exportedKindToExport = packageToKindToExport.getOrElse(funcExport.exportId.packageCoord, Map())
        //   (Vector(funcExport.prototype.returnType) ++ funcExport.prototype.paramTypes)
        //     .foreach(paramType => {
        //       if (!Compiler.isPrimitive(paramType.kind) && !exportedKindToExport.contains(paramType.kind)) {
        //         throw CompileErrorExceptionT(ExportedFunctionDependedOnNonExportedKind(...))
        //       }
        //     })
        // })
        let empty_kind_map: IndexMap<KindT<'s, 't>, &'t KindExportT<'s, 't>> = IndexMap::new();
        for func_export in coutputs.get_function_exports().iter() {
            let exported_kind_to_export = package_to_kind_to_export.get(func_export.export_id.package_coord).unwrap_or(&empty_kind_map);
            let all_types: Vec<CoordT<'s, 't>> = std::iter::once(func_export.prototype.return_type).chain(func_export.prototype.param_types().iter().copied()).collect();
            for param_type in all_types {
                if !self.is_primitive(param_type.kind) && !exported_kind_to_export.contains_key(&param_type.kind) {
                    let range_t = self.typing_interner.alloc_slice_copy(&[func_export.range]);
                    let signature_t = self.typing_interner.alloc(func_export.prototype.to_signature());
                    return Err(ICompileErrorT::ExportedFunctionDependedOnNonExportedKind {
                        range: range_t,
                        paackage: *func_export.export_id.package_coord,
                        signature: signature_t,
                        non_exported_kind: param_type.kind,
                    });
                }
            }
        }

        for function_extern in coutputs.get_function_externs().iter() {
            let exported_kind_to_export = package_to_kind_to_export.get(function_extern.extern_placeholdered_id.package_coord).unwrap_or(&empty_kind_map);
            let all_types: Vec<CoordT<'s, 't>> = std::iter::once(function_extern.prototype.return_type).chain(function_extern.prototype.param_types().iter().copied()).collect();
            for param_type in all_types {
                if !self.is_primitive(param_type.kind) && !exported_kind_to_export.contains_key(&param_type.kind) {
                    // Method-own and container-inherited template params surface here as
                    // placeholders at definition time (e.g. `extern func bar<C>(c C)` inside
                    // `extern struct Foo<A>` has C and A as KindPlaceholderTs in the wrapper
                    // prototype). Placeholders are substitution slots, not concrete types; the
                    // actual concrete kind for each monomorphization is what matters for ABI,
                    // and gets checked at instantiation.
                    let kind_is_fine_in_extern_func = match param_type.kind {
                        KindT::Struct(s) => coutputs.lookup_struct(s.id, self).attributes.iter().any(|a| matches!(a, crate::typing::ast::ast::ICitizenAttributeT::Extern(_))),
                        KindT::KindPlaceholder(_) => true,
                        _ => false,
                    };
                    if !kind_is_fine_in_extern_func {
                        let range_t = self.typing_interner.alloc_slice_copy(&[function_extern.range]);
                        let signature_t = self.typing_interner.alloc(function_extern.prototype.to_signature());
                        return Err(ICompileErrorT::ExternFunctionDependedOnNonExportedKind {
                            range: range_t,
                            paackage: *function_extern.extern_placeholdered_id.package_coord,
                            signature: signature_t,
                            non_exported_kind: param_type.kind,
                        });
                    }
                }
            }
        }

        // packageToKindToExport.foreach((packageCoord, exportedKindToExport) =>
        //   exportedKindToExport.foreach((exportedKind, (kind, export)) =>
        //     exportedKind match { case StructTT(_) => ...; case contentsStaticSizedArrayTT(...) => ...; ... }))
        for (package_coord, exported_kind_to_export) in package_to_kind_to_export.iter() {
            for (exported_kind, export) in exported_kind_to_export.iter() {
                match exported_kind {
                    KindT::Struct(sr) => {
                        let struct_def = coutputs.lookup_struct(sr.id, self);
                        let substituter =
                            self.get_placeholder_substituter(
                                self.opts.global_options.sanity_check,
                                struct_def.template_name,
                                sr.id,
                                IBoundArgumentsSource::InheritBoundsFromTypeItself,
                            );
                        for member in struct_def.members.iter() {
                            match member {
                                IStructMemberT::Variadic(_) => {
                                    panic!("implement: ensure_deep_exports — VariadicStructMemberT");
                                }
                                IStructMemberT::Normal(NormalStructMemberT { tyype: IMemberTypeT::Address(_), .. }) => {
                                    panic!("implement: ensure_deep_exports — AddressMemberTypeT");
                                }
                                IStructMemberT::Normal(NormalStructMemberT { tyype: IMemberTypeT::Reference(ReferenceMemberTypeT { reference: unsubstituted_member_coord }), .. }) => {
                                    let member_coord = substituter.substitute_for_coord(coutputs, *unsubstituted_member_coord);
                                    let member_kind = member_coord.kind;
                                    if struct_def.mutability == ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable })
                                        && !self.is_primitive(member_kind)
                                        && !exported_kind_to_export.contains_key(&member_kind)
                                    {
                                        let range_t = self.typing_interner.alloc_slice_copy(&[export.range]);
                                        return Err(ICompileErrorT::ExportedImmutableKindDependedOnNonExportedKind {
                                            range: range_t,
                                            paackage: **package_coord,
                                            exported_kind: *exported_kind,
                                            non_exported_kind: member_kind,
                                        });
                                    }
                                }
                            }
                        }
                    }
                    KindT::StaticSizedArray(as_tt) => {
                        let element_kind = as_tt.element_type().kind;
                        if as_tt.mutability() == ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable })
                            && !self.is_primitive(element_kind)
                            && !exported_kind_to_export.contains_key(&element_kind)
                        {
                            let range_t = self.typing_interner.alloc_slice_copy(&[export.range]);
                            return Err(ICompileErrorT::ExportedImmutableKindDependedOnNonExportedKind {
                                range: range_t,
                                paackage: **package_coord,
                                exported_kind: *exported_kind,
                                non_exported_kind: element_kind,
                            });
                        }
                    }
                    KindT::RuntimeSizedArray(rsa) => {
                        let mutability = match rsa.name.local_name {
                            INameT::RuntimeSizedArray(rsan) => rsan.arr.mutability,
                            _ => panic!("vwat"),
                        };
                        let element_kind = match rsa.name.local_name {
                            INameT::RuntimeSizedArray(rsan) => rsan.arr.element_type.kind,
                            _ => panic!("vwat"),
                        };
                        if mutability == ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable })
                            && !self.is_primitive(element_kind)
                            && !exported_kind_to_export.contains_key(&element_kind)
                        {
                            let range_t = self.typing_interner.alloc_slice_copy(&[export.range]);
                            return Err(ICompileErrorT::ExportedImmutableKindDependedOnNonExportedKind {
                                range: range_t,
                                paackage: **package_coord,
                                exported_kind: *exported_kind,
                                non_exported_kind: element_kind,
                            });
                        }
                    }
                    KindT::Interface(_) => {}
                    KindT::KindPlaceholder(_) | KindT::OverloadSet(_) |
                    KindT::Void(_) | KindT::Int(_) | KindT::Bool(_) | KindT::Str(_) | KindT::Float(_) | KindT::Never(_) => {
                        panic!("vwat: unexpected kind in exportedKindToExport");
                    }
                }
            }
        }
        Ok(())
    }
    /*
      def ensureDeepExports(coutputs: CompilerOutputs): Unit = {
        val packageToKindToExport =
          coutputs.getKindExports
            .map(kindExport => (kindExport.id.packageCoord, kindExport.tyype, kindExport))
            .groupBy(_._1)
            .mapValues(
              _.map(x => (x._2, x._3))
                .groupBy(_._1)
                .mapValues({
                  case Vector() => vwat()
                  case Vector(only) => only
                  case multiple => {
                    val exports = multiple.map(_._2)
                    throw CompileErrorExceptionT(
                      TypeExportedMultipleTimes(
                        List(exports.head.range),
                        exports.head.id.packageCoord,
                        exports))
                  }
                }))

        coutputs.getFunctionExports.foreach(funcExport => {
          val exportedKindToExport = packageToKindToExport.getOrElse(funcExport.exportId.packageCoord, Map())
          (Vector(funcExport.prototype.returnType) ++ funcExport.prototype.paramTypes)
            .foreach(paramType => {
              if (!Compiler.isPrimitive(paramType.kind) && !exportedKindToExport.contains(paramType.kind)) {
                throw CompileErrorExceptionT(
                  ExportedFunctionDependedOnNonExportedKind(
                    List(funcExport.range), funcExport.exportId.packageCoord, funcExport.prototype.toSignature, paramType.kind))
              }
            })
        })
        coutputs.getFunctionExterns.foreach(functionExtern => {
          val exportedKindToExport = packageToKindToExport.getOrElse(functionExtern.externPlaceholderedId.packageCoord, Map())
          (Vector(functionExtern.prototype.returnType) ++ functionExtern.prototype.paramTypes)
            .foreach(paramType => {
              if (!Compiler.isPrimitive(paramType.kind) && !exportedKindToExport.contains(paramType.kind)) {
                val kindIsFineInExternFunc =
                  paramType.kind match {
                    case StructTT(id) => coutputs.lookupStruct(id).attributes.exists(_.isInstanceOf[ExternT])
                    // Method-own and container-inherited template params surface here as
                    // placeholders at definition time (e.g. `extern func bar<C>(c C)` inside
                    // `extern struct Foo<A>` has C and A as KindPlaceholderTs in the wrapper
                    // prototype). Placeholders are substitution slots, not concrete types; the
                    // actual concrete kind for each monomorphization is what matters for ABI,
                    // and gets checked at instantiation.
                    case KindPlaceholderT(_) => true
                    case _ => false
                  }
                if (!kindIsFineInExternFunc) {
                  throw CompileErrorExceptionT(
                    ExternFunctionDependedOnNonExportedKind(
                      List(functionExtern.range), functionExtern.externPlaceholderedId.packageCoord, functionExtern.prototype.toSignature, paramType.kind))
                }
              }
            })
        })
        packageToKindToExport.foreach({ case (packageCoord, exportedKindToExport) =>
          exportedKindToExport.foreach({ case (exportedKind, (kind, export)) =>
            exportedKind match {
              case sr@StructTT(_) => {
                val structDef = coutputs.lookupStruct(sr.id)

                val substituter =
                  TemplataCompiler.getPlaceholderSubstituter(
                    opts.globalOptions.sanityCheck,
                    interner,
                    keywords,
                    structDef.templateName,
                    sr.id,
                    InheritBoundsFromTypeItself)

                structDef.members.foreach({
                  case VariadicStructMemberT(name, tyype) => {
                    vimpl()
                  }
                  case NormalStructMemberT(name, variability, AddressMemberTypeT(reference)) => {
                    vimpl()
                  }
                  case NormalStructMemberT(_, _, ReferenceMemberTypeT(unsubstitutedMemberCoord)) => {
                    val memberCoord = substituter.substituteForCoord(coutputs, unsubstitutedMemberCoord)
                    val memberKind = memberCoord.kind
                    if (structDef.mutability == MutabilityTemplataT(ImmutableT) && !Compiler.isPrimitive(memberKind) && !exportedKindToExport.contains(memberKind)) {
                      throw CompileErrorExceptionT(
                        vale.typing.ExportedImmutableKindDependedOnNonExportedKind(
                          List(export.range), packageCoord, exportedKind, memberKind))
                    }
                  }
                })
              }
              case contentsStaticSizedArrayTT(_, mutability, _, CoordT(_, _, elementKind), _) => {
                if (mutability == MutabilityTemplataT(ImmutableT) && !Compiler.isPrimitive(elementKind) && !exportedKindToExport.contains(elementKind)) {
                  throw CompileErrorExceptionT(
                    vale.typing.ExportedImmutableKindDependedOnNonExportedKind(
                      List(export.range), packageCoord, exportedKind, elementKind))
                }
              }
              case contentsRuntimeSizedArrayTT(mutability, CoordT(_, _, elementKind), _) => {
                if (mutability == MutabilityTemplataT(ImmutableT) && !Compiler.isPrimitive(elementKind) && !exportedKindToExport.contains(elementKind)) {
                  throw CompileErrorExceptionT(
                    vale.typing.ExportedImmutableKindDependedOnNonExportedKind(
                      List(export.range), packageCoord, exportedKind, elementKind))
                }
              }
              case InterfaceTT(_) =>
            }
          })
        })
      }
    */
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn is_root_function(
        &self,
        function_a: &'s FunctionA<'s>,
    ) -> bool {
        panic!("Unimplemented: Slab 15 — body migration");
    }
    /*
      // Returns whether we should eagerly compile this and anything it depends on.
      def isRootFunction(functionA: FunctionA): Boolean = {
        functionA.name match {
          case FunctionNameS(StrI("main"), _) => return true
          case _ =>
        }
        functionA.attributes.exists({
          case ExportS(_) => true
          case ExternS(_) => true
          case _ => false
        })
      }
    */
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn is_root_struct(
        &self,
        struct_a: &'s StructA<'s>,
    ) -> bool {
        panic!("Unimplemented: Slab 15 — body migration");
    }
    /*
      // Returns whether we should eagerly compile this and anything it depends on.
      def isRootStruct(structA: StructA): Boolean = {
        structA.attributes.exists({ case ExportS(_) => true case _ => false })
      }
    */
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn is_root_interface(
        &self,
        interface_a: &'s InterfaceA<'s>,
    ) -> bool {
        panic!("Unimplemented: Slab 15 — body migration");
    }
    /*
      // Returns whether we should eagerly compile this and anything it depends on.
      def isRootInterface(interfaceA: InterfaceA): Boolean = {
        interfaceA.attributes.exists({ case ExportS(_) => true case _ => false })
      }
    }


    object Compiler {
    */
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn consecutive(
        &self,
        exprs: &[ReferenceExpressionTE<'s, 't>],
    ) -> ReferenceExpressionTE<'s, 't> {
        match exprs {
            [] => panic!("Shouldn't have zero-element consecutors!"),
            [only] => *only,
            _ => {
                let flattened: Vec<ReferenceExpressionTE<'s, 't>> =
                    exprs.iter().flat_map(|e| {
                        match e {
                            ReferenceExpressionTE::Consecutor(c) => c.exprs.to_vec(),
                            other => vec![*other],
                        }
                    }).collect();

                let without_init_voids: Vec<ReferenceExpressionTE<'s, 't>> = {
                    let (init, last) = flattened.split_at(flattened.len() - 1);
                    let mut filtered: Vec<ReferenceExpressionTE<'s, 't>> = init.iter()
                        .filter(|e| !matches!(e, ReferenceExpressionTE::VoidLiteral(_)))
                        .copied()
                        .collect();
                    filtered.push(last[0]);
                    filtered
                };

                match without_init_voids.as_slice() {
                    [] => panic!("Shouldn't have zero-element consecutors!"),
                    [only] => *only,
                    _ => {
                        let exprs_slice = self.typing_interner.alloc_slice_copy(&without_init_voids);
                        ReferenceExpressionTE::Consecutor(self.typing_interner.alloc(ConsecutorTE { exprs: exprs_slice }))
                    }
                }
            }
        }
    }
    /*
      // Flattens any nested ConsecutorTEs
      def consecutive(exprs: Vector[ReferenceExpressionTE]): ReferenceExpressionTE = {
        exprs match {
          case Vector() => vwat("Shouldn't have zero-element consecutors!")
          case Vector(only) => only
          case _ => {
            val flattened =
              exprs.flatMap({
                case ConsecutorTE(exprs) => exprs
                case other => Vector(other)
              })

            val withoutInitVoids =
              flattened.init
                .filter({ case VoidLiteralTE(_) => false case _ => true }) :+
                flattened.last

            withoutInitVoids match {
              case Vector() => vwat("Shouldn't have zero-element consecutors!")
              case Vector(only) => only
              case _ => ConsecutorTE(withoutInitVoids)
            }
          }
        }
      }
    */
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn is_primitive(
        &self,
        kind: KindT<'s, 't>,
    ) -> bool {
        match kind {
            KindT::Void(_) | KindT::Int(_) | KindT::Bool(_) | KindT::Str(_) | KindT::Never(_) | KindT::Float(_) => true,
            KindT::KindPlaceholder(_) => false,
            KindT::Struct(_) => false,
            KindT::Interface(_) => false,
            KindT::StaticSizedArray(_) => false,
            KindT::RuntimeSizedArray(_) => false,
            KindT::OverloadSet(_) => false,
        }
    }
    /*
      def isPrimitive(kind: KindT): Boolean = {
        kind match {
          case VoidT() | IntT(_) | BoolT() | StrT() | NeverT(_) | FloatT() => true
    //      case TupleTT(_, understruct) => isPrimitive(understruct)
          case KindPlaceholderT(_) => false
          case StructTT(_) => false
          case InterfaceTT(_) => false
          case contentsStaticSizedArrayTT(_, _, _, _, _) => false
          case contentsRuntimeSizedArrayTT(_, _, _) => false
        }
      }
    */
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_mutabilities(
        &self,
        coutputs: &CompilerOutputs<'s, 't>,
        concrete_values2: &[KindT<'s, 't>],
    ) -> Vec<ITemplataT<'s, 't>> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
    /*
      def getMutabilities(coutputs: CompilerOutputs, concreteValues2: Vector[KindT]):
      Vector[ITemplataT[MutabilityTemplataType]] = {
        concreteValues2.map(concreteValue2 => getMutability(coutputs, concreteValue2))
      }
    */
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_mutability(
        &self,
        coutputs: &CompilerOutputs<'s, 't>,
        concrete_value2: KindT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        match concrete_value2 {
            KindT::Never(_) => ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }),
            KindT::Int(_) => ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }),
            KindT::Float(_) => ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }),
            KindT::Bool(_) => ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }),
            KindT::Str(_) => ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }),
            KindT::Void(_) => ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }),
            KindT::KindPlaceholder(kp) => coutputs.lookup_mutability(self.get_placeholder_template(kp.id)),
            KindT::RuntimeSizedArray(rsa) => {
                match rsa.name.local_name {
                    INameT::RuntimeSizedArray(rsan) => rsan.arr.mutability,
                    _ => panic!("Expected RuntimeSizedArray local_name in get_mutability"),
                }
            }
            KindT::StaticSizedArray(ssa) => ssa.mutability(),
            KindT::Struct(s) => coutputs.lookup_mutability(self.get_struct_template(s.id)),
            KindT::Interface(i) => coutputs.lookup_mutability(self.get_interface_template(i.id)),
            KindT::OverloadSet(_) => ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }),
        }
    }
    /*
Guardian: temp-disable: SPDMX — The Scala uses `contentsRuntimeSizedArrayTT(mutability, _, _)` which is a Scala extractor equivalent to `rsa.name.local_name → RuntimeSizedArrayNameT { arr, .. } → arr.mutability`. The Rust data model stores mutability on `RawArrayNameT` (accessed via `name.local_name → INameT::RuntimeSizedArray → .arr.mutability`), so an extra nested match arm is structurally required for exhaustiveness. This is the Rust-idiomatic translation of the Scala extractor, not novel logic (Exception R). — FrontendRust/guardian-logs/request-413-1778705498733/hook-413/get_mutability--3129.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
      def getMutability(coutputs: CompilerOutputs, concreteValue2: KindT):
      ITemplataT[MutabilityTemplataType] = {
        concreteValue2 match {
          case KindPlaceholderT(id) => coutputs.lookupMutability(TemplataCompiler.getPlaceholderTemplate(id))
          case NeverT(_) => MutabilityTemplataT(ImmutableT)
          case IntT(_) => MutabilityTemplataT(ImmutableT)
          case FloatT() => MutabilityTemplataT(ImmutableT)
          case BoolT() => MutabilityTemplataT(ImmutableT)
          case StrT() => MutabilityTemplataT(ImmutableT)
          case VoidT() => MutabilityTemplataT(ImmutableT)
          case contentsRuntimeSizedArrayTT(mutability, _, _) => mutability
          case contentsStaticSizedArrayTT(_, mutability, _, _, _) => mutability
          case sr @ StructTT(name) => coutputs.lookupMutability(TemplataCompiler.getStructTemplate(name))
          case ir @ InterfaceTT(name) => coutputs.lookupMutability(TemplataCompiler.getInterfaceTemplate(name))
    //      case PackTT(_, sr) => coutputs.lookupMutability(sr)
    //      case TupleTT(_, sr) => coutputs.lookupMutability(sr)
          case OverloadSetT(_, _) => {
            // Just like FunctionT2
            MutabilityTemplataT(ImmutableT)
          }
        }
      }
    }
    */
}
