use crate::higher_typing::ast::FunctionA;
use crate::interner::{Interner, StrI};
use std::collections::{HashMap, HashSet};
use indexmap::IndexMap;
use crate::utils::range::RangeS;
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::names::*;
use crate::postparsing::*;
use crate::typing::hinputs_t::*;
use crate::typing::compilation::TypingPassOptions;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::typing::infer_compiler::{InitialKnown, InitialSend};
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::typing_interner::TypingInterner;
use crate::typing::compiler::Compiler;

/*
package dev.vale.typing

import dev.vale.postparsing._
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.names._
import dev.vale.typing.types._
import dev.vale._
import dev.vale.typing.ast._
import dev.vale.typing.templata._
import dev.vale.typing.types.InterfaceTT

import scala.collection.immutable.{List, Map}
import scala.collection.mutable
*/
/// Temporary state (see @TFITCX)
pub enum DeferredActionT<'s, 't>
where 's: 't,
{
    EvaluateFunctionBody {
        prototype: &'t PrototypeT<'s, 't>,
        full_env_snapshot: &'t FunctionEnvironmentT<'s, 't>,
        call_range: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        life: LocationInFunctionEnvironmentT<'s, 't>,
        attributes_t: &'t [IFunctionAttributeT<'s>],
        params_t: &'t [ParameterT<'s, 't>],
        is_destructor: bool,
        maybe_explicit_return_coord: Option<CoordT<'s, 't>>,
        instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
    },
    /*
    case class DeferredEvaluatingFunctionBody(
      prototypeT: PrototypeT[IFunctionNameT],
      call: (CompilerOutputs) => Unit)
    */
    EvaluateFunction {
        name: &'t IdT<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        origin: &'s FunctionA<'s>,
        template_args: &'t [ITemplataT<'s, 't>],
    },
    /*
    case class DeferredEvaluatingFunction(
      name: IdT[INameT],
      call: (CompilerOutputs) => Unit)
    */
}
/// Temporary state (see @TFITCX)
pub struct CompilerOutputs<'s, 't>
where 's: 't,
{
    pub return_types_by_signature:
        HashMap<SignatureT<'s, 't>, CoordT<'s, 't>>,
    // Per @IIIOZ, iterated by get_all_functions → IndexMap for cross-run determinism.
    pub signature_to_function:
        IndexMap<SignatureT<'s, 't>, &'t FunctionDefinitionT<'s, 't>>,

    pub function_declared_names:
        HashMap<IdT<'s, 't>, RangeS<'s>>,
    pub type_declared_names:
        HashSet<IdT<'s, 't>>,

    pub function_name_to_outer_env:
        HashMap<IdT<'s, 't>, IInDenizenEnvironmentT<'s, 't>>,
    pub function_name_to_inner_env:
        HashMap<IdT<'s, 't>, IInDenizenEnvironmentT<'s, 't>>,
    pub type_name_to_outer_env:
        HashMap<IdT<'s, 't>, IInDenizenEnvironmentT<'s, 't>>,
    pub type_name_to_inner_env:
        HashMap<IdT<'s, 't>, IInDenizenEnvironmentT<'s, 't>>,

    pub type_name_to_mutability:
        HashMap<IdT<'s, 't>, ITemplataT<'s, 't>>,
    pub interface_name_to_sealed:
        HashMap<IdT<'s, 't>, bool>,

    // Per @IIIOZ, iterated by get_all_structs / get_all_interfaces → IndexMap for cross-run determinism.
    pub struct_template_name_to_definition:
        IndexMap<IdT<'s, 't>, &'t StructDefinitionT<'s, 't>>,
    pub interface_template_name_to_definition:
        IndexMap<IdT<'s, 't>, &'t InterfaceDefinitionT<'s, 't>>,

    pub all_impls:
        HashMap<IdT<'s, 't>, &'t ImplT<'s, 't>>,
    pub sub_citizen_template_to_impls:
        HashMap<IdT<'s, 't>, Vec<&'t ImplT<'s, 't>>>,
    pub super_interface_template_to_impls:
        HashMap<IdT<'s, 't>, Vec<&'t ImplT<'s, 't>>>,

    pub kind_exports: Vec<&'t KindExportT<'s, 't>>,
    pub function_exports: Vec<&'t FunctionExportT<'s, 't>>,
    pub kind_externs: Vec<&'t KindExternT<'s, 't>>,
    pub function_externs: Vec<&'t FunctionExternT<'s, 't>>,

    pub instantiation_name_to_bounds:
        HashMap<IdT<'s, 't>, &'t InstantiationBoundArgumentsT<'s, 't>>,

    // Per @IIIOZ, deferred queues are IndexMap so drain order is insertion-ordered and deterministic across runs.
    pub deferred_function_body_compiles: IndexMap<PrototypeT<'s, 't>, DeferredActionT<'s, 't>>,
    pub deferred_function_compiles: IndexMap<IdT<'s, 't>, DeferredActionT<'s, 't>>,
    pub finished_deferred_function_body_compiles:
        HashSet<PrototypeT<'s, 't>>,
    pub finished_deferred_function_compiles:
        HashSet<IdT<'s, 't>>,
}
/*
case class CompilerOutputs() {
  // Not all signatures/banners will have a return type here, it might not have been processed yet.
  private val returnTypesBySignature: mutable.HashMap[SignatureT, CoordT] = mutable.HashMap()

  // Not all signatures/banners or even return types will have a function here, it might not have
  // been processed yet.
  private val signatureToFunction: mutable.HashMap[SignatureT, FunctionDefinitionT] = mutable.HashMap()
//  private val functionsByPrototype: mutable.HashMap[PrototypeT, FunctionT] = mutable.HashMap()
  private val envByFunctionSignature: mutable.HashMap[SignatureT, FunctionEnvironmentT] = mutable.HashMap()

  // declaredNames is the structs that we're currently in the process of defining
  // Things will appear here before they appear in structTemplateNameToDefinition/interfaceTemplateNameToDefinition
  // This is to prevent infinite recursion / stack overflow when typingpassing recursive types
  // This will be the instantiated name, not just the template name, see UINIT.
  private val functionDeclaredNames: mutable.HashMap[IdT[INameT], RangeS] = mutable.HashMap()
  // Outer env is the env that contains the template.
  // This will be the instantiated name, not just the template name, see UINIT.
  private val functionNameToOuterEnv: mutable.HashMap[IdT[IFunctionTemplateNameT], IInDenizenEnvironmentT] = mutable.HashMap()
  // Inner env is the env that contains the solved rules for the declaration, given placeholders.
  // This will be the instantiated name, not just the template name, see UINIT.
  private val functionNameToInnerEnv: mutable.HashMap[IdT[INameT], IInDenizenEnvironmentT] = mutable.HashMap()


  // declaredNames is the structs that we're currently in the process of defining
  // Things will appear here before they appear in structTemplateNameToDefinition/interfaceTemplateNameToDefinition
  // This is to prevent infinite recursion / stack overflow when typingpassing recursive types
  private val typeDeclaredNames: mutable.HashSet[IdT[ITemplateNameT]] = mutable.HashSet()
  // Outer env is the env that contains the template.
  private val typeNameToOuterEnv: mutable.HashMap[IdT[ITemplateNameT], IInDenizenEnvironmentT] = mutable.HashMap()
  // Inner env is the env that contains the solved rules for the declaration, given placeholders.
  // We can key by template name here because there's only one inner env per template. This is the env
  // that has placeholders and stuff.
  // Also, if it's keyed by template name, we can access it earlier, before the definition is even made.
  // This is important for when we want to be compiling a struct/interface and one of its internal methods
  // wants to look in its inner env to get some bounds.
  private val typeNameToInnerEnv: mutable.HashMap[IdT[ITemplateNameT], IInDenizenEnvironmentT] = mutable.HashMap()
  // One must fill this in when putting things into declaredNames.
  private val typeNameToMutability: mutable.HashMap[IdT[ITemplateNameT], ITemplataT[MutabilityTemplataType]] = mutable.HashMap()
  // One must fill this in when putting things into declaredNames.
  private val interfaceNameToSealed: mutable.HashMap[IdT[IInterfaceTemplateNameT], Boolean] = mutable.HashMap()


  private val structTemplateNameToDefinition: mutable.HashMap[IdT[IStructTemplateNameT], StructDefinitionT] = mutable.HashMap()
  private val interfaceTemplateNameToDefinition: mutable.HashMap[IdT[IInterfaceTemplateNameT], InterfaceDefinitionT] = mutable.HashMap()

  private val allImpls: mutable.HashMap[IdT[IImplTemplateNameT], ImplT] = mutable.HashMap()
  private val subCitizenTemplateToImpls: mutable.HashMap[IdT[ICitizenTemplateNameT], Vector[ImplT]] = mutable.HashMap()
  private val superInterfaceTemplateToImpls: mutable.HashMap[IdT[IInterfaceTemplateNameT], Vector[ImplT]] = mutable.HashMap()

  private val kindExports: mutable.ArrayBuffer[KindExportT] = mutable.ArrayBuffer()
  private val functionExports: mutable.ArrayBuffer[FunctionExportT] = mutable.ArrayBuffer()
  private val kindExterns: mutable.ArrayBuffer[KindExternT] = mutable.ArrayBuffer()
  private val functionExterns: mutable.ArrayBuffer[FunctionExternT] = mutable.ArrayBuffer()

  // When we call a function, for example this one:
  //   abstract func drop<T>(virtual opt Opt<T>) where func drop(T)void;
  // and we instantiate it, drop<int>(Opt<int>), we need to figure out the bounds, ensure that
  // drop(int) exists. Then we have to remember it for the instantiator.
  // This map is how we remember it.
  // Here, we'd remember: [drop<int>(Opt<int>), [Rune1337, drop(int)]].
  // We also do this for structs and interfaces too.
  private val instantiationNameToInstantiationBounds: mutable.HashMap[IdT[IInstantiationNameT], InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]] =
    mutable.HashMap[IdT[IInstantiationNameT], InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]]()

//  // Only ArrayCompiler can make an RawArrayT2.
//  private val staticSizedArrayTypes:
//    mutable.HashMap[(ITemplata[IntegerTemplataType], ITemplata[MutabilityTemplataType], ITemplata[VariabilityTemplataType], CoordT), StaticSizedArrayTT] =
//    mutable.HashMap()
//  // Only ArrayCompiler can make an RawArrayT2.
//  private val runtimeSizedArrayTypes: mutable.HashMap[(ITemplata[MutabilityTemplataType], CoordT), RuntimeSizedArrayTT] = mutable.HashMap()

  // A queue of functions that our code uses, but we don't need to compile them right away.
  // We can compile them later. Perhaps in parallel, someday!
  private val deferredFunctionBodyCompiles: mutable.LinkedHashMap[PrototypeT[IFunctionNameT], DeferredEvaluatingFunctionBody] = mutable.LinkedHashMap()
  private val finishedDeferredFunctionBodyCompiles: mutable.LinkedHashSet[PrototypeT[IFunctionNameT]] = mutable.LinkedHashSet()

  private val deferredFunctionCompiles: mutable.LinkedHashMap[IdT[INameT], DeferredEvaluatingFunction] = mutable.LinkedHashMap()
  private val finishedDeferredFunctionCompiles: mutable.LinkedHashSet[IdT[INameT]] = mutable.LinkedHashSet()
*/

impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn new() -> Self {
        Self {
            return_types_by_signature: HashMap::new(),
            signature_to_function: IndexMap::new(),
            function_declared_names: HashMap::new(),
            type_declared_names: HashSet::new(),
            function_name_to_outer_env: HashMap::new(),
            function_name_to_inner_env: HashMap::new(),
            type_name_to_outer_env: HashMap::new(),
            type_name_to_inner_env: HashMap::new(),
            type_name_to_mutability: HashMap::new(),
            interface_name_to_sealed: HashMap::new(),
            struct_template_name_to_definition: IndexMap::new(),
            interface_template_name_to_definition: IndexMap::new(),
            all_impls: HashMap::new(),
            sub_citizen_template_to_impls: HashMap::new(),
            super_interface_template_to_impls: HashMap::new(),
            kind_exports: Vec::new(),
            function_exports: Vec::new(),
            kind_externs: Vec::new(),
            function_externs: Vec::new(),
            instantiation_name_to_bounds: HashMap::new(),
            deferred_function_body_compiles: IndexMap::new(),
            deferred_function_compiles: IndexMap::new(),
            finished_deferred_function_body_compiles: HashSet::new(),
            finished_deferred_function_compiles: HashSet::new(),
        }
    }
    /*
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn count_denizens(&self) -> i32 {
        panic!("Unimplemented: Slab 10 — body migration");
    }
    /*
      def countDenizens(): Int = {
    //    staticSizedArrayTypes.size +
    //      runtimeSizedArrayTypes.size +
          signatureToFunction.size +
          structTemplateNameToDefinition.size +
          interfaceTemplateNameToDefinition.size
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn peek_next_deferred_function_body_compile(&self) -> Option<&DeferredActionT<'s, 't>> {
        self.deferred_function_body_compiles.values().next()
    }
    /*
      def peekNextDeferredFunctionBodyCompile(): Option[DeferredEvaluatingFunctionBody] = {
        deferredFunctionBodyCompiles.headOption.map(_._2)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn mark_deferred_function_body_compiled(
        &mut self,
        prototype_t: &'t PrototypeT<'s, 't>,
    ) {
        // vassert(prototypeT == vassertSome(deferredFunctionBodyCompiles.headOption)._1)
        let first_key = *self.deferred_function_body_compiles.keys().next().unwrap();
        assert!(*prototype_t == first_key);
        // finishedDeferredFunctionBodyCompiles += prototypeT
        self.finished_deferred_function_body_compiles.insert(*prototype_t);
        // deferredFunctionBodyCompiles -= prototypeT
        self.deferred_function_body_compiles.shift_remove(prototype_t);
    }
    /*
      def markDeferredFunctionBodyCompiled(prototypeT: PrototypeT[IFunctionNameT]): Unit = {
        vassert(prototypeT == vassertSome(deferredFunctionBodyCompiles.headOption)._1)
        finishedDeferredFunctionBodyCompiles += prototypeT
        deferredFunctionBodyCompiles -= prototypeT
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn peek_next_deferred_function_compile(&self) -> Option<&DeferredActionT<'s, 't>> {
        self.deferred_function_compiles.values().next()
    }
    /*
      def peekNextDeferredFunctionCompile(): Option[DeferredEvaluatingFunction] = {
        deferredFunctionCompiles.headOption.map(_._2)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn mark_deferred_function_compiled(
        &mut self,
        name: &'t IdT<'s, 't>,
    ) {
        // vassert(name == vassertSome(deferredFunctionCompiles.headOption)._1)
        let first_key = *self.deferred_function_compiles.keys().next().unwrap();
        assert!(*name == first_key);
        // finishedDeferredFunctionCompiles += name
        self.finished_deferred_function_compiles.insert(*name);
        // deferredFunctionCompiles -= name
        self.deferred_function_compiles.shift_remove(name);
    }
    /*
      def markDeferredFunctionCompiled(name: IdT[INameT]): Unit = {
        vassert(name == vassertSome(deferredFunctionCompiles.headOption)._1)
        finishedDeferredFunctionCompiles += name
        deferredFunctionCompiles -= name
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_instantiation_name_to_function_bound_to_rune(
        &self,
    ) -> HashMap<IdT<'s, 't>, &'t InstantiationBoundArgumentsT<'s, 't>> {
        self.instantiation_name_to_bounds.clone()
    }
    /*
      def getInstantiationNameToFunctionBoundToRune(): Map[IdT[IInstantiationNameT], InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]] = {
        instantiationNameToInstantiationBounds.toMap
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn lookup_function(
        &self,
        signature: &'t SignatureT<'s, 't>,
    ) -> Option<&'t FunctionDefinitionT<'s, 't>> {
        // signatureToFunction.get(signature)
        self.signature_to_function.get(signature).copied()
    }
    /*
      def lookupFunction(signature: SignatureT): Option[FunctionDefinitionT] = {
        signatureToFunction.get(signature)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_instantiation_bounds(
        &self,
        interner: &TypingInterner<'s, 't>,
        instantiation_id: IdT<'s, 't>,
    ) -> Option<&'t InstantiationBoundArgumentsT<'s, 't>> {
        let instantiation_id_ref = interner.intern_id(IdValT {
            package_coord: instantiation_id.package_coord,
            init_steps: instantiation_id.init_steps,
            local_name: instantiation_id.local_name,
        });
        self.instantiation_name_to_bounds.get(instantiation_id_ref).copied()
    }
    /*
      def getInstantiationBounds(
        instantiationId: IdT[IInstantiationNameT]):
      Option[InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]] = {
        instantiationNameToInstantiationBounds.get(instantiationId)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn add_instantiation_bounds(
        &mut self,
        _sanity_check: bool,
        interner: &TypingInterner<'s, 't>,
        _original_calling_template_id: IdT<'s, 't>,
        instantiation_id: IdT<'s, 't>,
        instantiation_bound_args: &'t InstantiationBoundArgumentsT<'s, 't>,
    ) {
        for (_rune, reachable_bound_args) in &instantiation_bound_args.rune_to_citizen_rune_to_reachable_prototype {
            for (_callee_rune, reachable_prototype) in &reachable_bound_args.citizen_rune_to_reachable_prototype {
                match reachable_prototype.id.local_name {
                    INameT::FunctionBound(_) => {
                        let reachable_func_super_template_id_init_steps =
                            Compiler::get_super_template(interner, reachable_prototype.id).init_steps;
                        let original_calling_super_template_id_init_steps =
                            Compiler::get_super_template(interner, _original_calling_template_id).init_steps;
                        assert!(
                            reachable_func_super_template_id_init_steps.starts_with(original_calling_super_template_id_init_steps),
                            "addInstantiationBounds: reachable func super template id init steps doesn't start with original calling super template id init steps"
                        );
                    }
                    _ => {}
                }
            }
        }
        for (_rune, caller_bound_arg_function) in &instantiation_bound_args.rune_to_bound_prototype {
            match caller_bound_arg_function.id.local_name {
                INameT::FunctionBound(_) => {
                    if _sanity_check {
                        let caller_bound_arg_func_super_template_id_init_steps =
                            Compiler::get_super_template(interner, caller_bound_arg_function.id).init_steps;
                        let original_calling_super_template_id_steps =
                            Compiler::get_root_super_template(interner, _original_calling_template_id).init_steps;
                        assert!(
                            caller_bound_arg_func_super_template_id_init_steps.starts_with(original_calling_super_template_id_steps),
                            "addInstantiationBounds: caller bound arg func super template id init steps doesn't start with original calling super template id steps"
                        );
                    }
                }
                _ => {}
            }
        }

        let instantiation_id_ref = interner.intern_id(IdValT {
            package_coord: instantiation_id.package_coord,
            init_steps: instantiation_id.init_steps,
            local_name: instantiation_id.local_name,
        });
        if let Some(existing) = self.instantiation_name_to_bounds.get(instantiation_id_ref) {
            // Theres some ambiguities or something here. sometimes when we evaluate
            // the same thing twice we get different results.
            // It's gonna be especially tricky because we get each function bounds from the overload
            // resolver which only returns one.
            // We avoid this by merging all sorts of function bounds, see MFBFDP.
            assert!(
                existing.rune_to_bound_prototype == instantiation_bound_args.rune_to_bound_prototype &&
                existing.rune_to_citizen_rune_to_reachable_prototype == instantiation_bound_args.rune_to_citizen_rune_to_reachable_prototype &&
                existing.rune_to_bound_impl == instantiation_bound_args.rune_to_bound_impl,
                "addInstantiationBounds: existing bounds != new bounds"
            );
            return;
        }

        self.instantiation_name_to_bounds.insert(*instantiation_id_ref, instantiation_bound_args);
    }
    /*
      def addInstantiationBounds(
        sanityCheck: Boolean,
        interner: Interner,
        originalCallingTemplateId: IdT[ITemplateNameT],
        instantiationId: IdT[IInstantiationNameT],
        instantiationBoundArgs: InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]):
      Unit = {
        val InstantiationBoundArgumentsT(
        runeToBoundPrototype,
        runeToCitizenRuneToReachablePrototype,
        runeToBoundImpl) = instantiationBoundArgs

        instantiationId match {
          case IdT(_,Vector(),FunctionNameT(FunctionTemplateNameT(StrI("Bork"),_),Vector(CoordTemplataT(CoordT(_,RegionT(DefaultRegionT),IntT(32)))),Vector(CoordT(_,RegionT(DefaultRegionT),IntT(32))))) => {
            vpass()
          }
          case IdT(_,Vector(),InterfaceNameT(InterfaceTemplateNameT(StrI("XOpt")),Vector(CoordTemplataT(CoordT(own,RegionT(DefaultRegionT),KindPlaceholderT(IdT(_,Vector(InterfaceTemplateNameT(StrI("XOpt")), FunctionTemplateNameT(StrI("harvest"),_), OverrideDispatcherTemplateNameT(IdT(_,Vector(),ImplTemplateNameT(_)))),KindPlaceholderNameT(KindPlaceholderTemplateNameT(0,DispatcherRuneFromImplS(CodeRuneS(StrI("T")))))))))))) => {
            vpass()
          }
          case IdT(_,Vector(),InterfaceNameT(InterfaceTemplateNameT(StrI("IXOption")),Vector(CoordTemplataT(CoordT(own,RegionT(DefaultRegionT),KindPlaceholderT(IdT(_,Vector(FunctionTemplateNameT(StrI("drop"),_), OverrideDispatcherTemplateNameT(IdT(_,Vector(),ImplTemplateNameT(_)))),KindPlaceholderNameT(KindPlaceholderTemplateNameT(0,DispatcherRuneFromImplS(CodeRuneS(StrI("T")))))))))))) => {
            vpass()
          }
          case _ =>
        }

        // We do this so that there's no random selection of where we get a particular bound from, see MFBFDP.
        // Keeps things nice and consistent so we dont run into any oddities with the overload index.
        runeToCitizenRuneToReachablePrototype.foreach({ case (callerRUne, reachableBoundArgs) =>
          val InstantiationReachableBoundArgumentsT(citizenAndRuneAndReachablePrototypes) =
            reachableBoundArgs
          citizenAndRuneAndReachablePrototypes.foreach({
            case (calleeRune, reachablePrototype) => {
              reachablePrototype.id.localName match {
                case FunctionBoundNameT(_, _, _) => {
                  val reachableFuncSuperTemplateIdInitSteps =
                    TemplataCompiler.getSuperTemplate(reachablePrototype.id).initSteps
                  val originalCallingSuperTemplateIdInitSteps =
                    TemplataCompiler.getSuperTemplate(originalCallingTemplateId).initSteps
                  vassert(reachableFuncSuperTemplateIdInitSteps.startsWith(originalCallingSuperTemplateIdInitSteps))
                }
                case _ =>
              }
            }
          })
        })
        // If we're instantiating with a bound, then make sure that it's one that comes from our root compiling denizen env;
        // make sure we imported it correctly, see MFBFDP.
        // That'll help ensure that we're not doing anything tricky, and ensure we don't trigger any mismatches below.
        runeToBoundPrototype.foreach({ case (rune, callerBoundArgFunction) =>
          callerBoundArgFunction.id.localName match {
            case FunctionBoundNameT(_, _, _) => {
              if (sanityCheck) {
                val callerBoundArgFuncSuperTemplateIdInitSteps =
                  TemplataCompiler.getSuperTemplate(callerBoundArgFunction.id).steps
                val originalCallingSuperTemplateIdInitSteps =
                  TemplataCompiler.getRootSuperTemplate(interner, originalCallingTemplateId).steps
                vassert(callerBoundArgFuncSuperTemplateIdInitSteps.startsWith(originalCallingSuperTemplateIdInitSteps))
              }
            }
            case _ =>
          }
        })
        // TODO: have asserts for the impls too. Might become moot if we don't need to register
        //   bounds with coutputs one day.

        // If there are any placeholders in the thing we're calling, make sure they're from the original calling template,
        // otherwise we probably forgot to do a substitution or something.
        if (sanityCheck) {
          Collector.all(instantiationId, {
            case id@IdT(_, initSteps, KindPlaceholderNameT(_)) => {
              val x: IdT[INameT] = id
              vassert(
                TemplataCompiler.getSuperTemplate(x).initSteps
                    .startsWith(TemplataCompiler.getRootSuperTemplate(interner, originalCallingTemplateId).initSteps))
            }
          })
        }

        // We'll do this when we can cache instantiations from StructTemplar etc.
        // // We should only add instantiation bounds in exactly one place: the place that makes the
        // // PrototypeT/StructTT/InterfaceTT.
        // vassert(!instantiationNameToInstantiationBounds.contains(instantiationFullName))
        instantiationNameToInstantiationBounds.get(instantiationId) match {
          case Some(existing) => {
            // Theres some ambiguities or something here. sometimes when we evaluate
            // the same thing twice we get different results.
            // It's gonna be especially tricky because we get each function bounds from the overload
            // resolver which only returns one.
            // We avoid this by merging all sorts of function bounds, see MFBFDP.
            vassert(existing == instantiationBoundArgs)
          }
          case None =>
        }

        instantiationId match {
          case IdT(PackageCoordinate(StrI("stdlib"),Vector(StrI("ifunction"))),Vector(),AnonymousSubstructNameT(AnonymousSubstructTemplateNameT(InterfaceTemplateNameT(StrI("IFunction1"))),Vector(MutabilityTemplataT(MutableT), CoordTemplataT(CoordT(BorrowT,RegionT(DefaultRegionT),StructTT(IdT(PackageCoordinate(StrI("parseiter"),Vector()),Vector(),StructNameT(StructTemplateNameT(StrI("ParseIter")),Vector()))))), CoordTemplataT(CoordT(ShareT,RegionT(DefaultRegionT),BoolT())), CoordTemplataT(CoordT(ShareT,RegionT(DefaultRegionT),StructTT(IdT(PackageCoordinate(StrI("vmdparse"),Vector()),Vector(FunctionNameT(FunctionTemplateNameT(StrI("parseSlice"),_),Vector(),Vector(CoordT(BorrowT,RegionT(DefaultRegionT),StructTT(IdT(PackageCoordinate(StrI("stdlib"),Vector(StrI("path"))),Vector(),StructNameT(StructTemplateNameT(StrI("Path")),Vector())))), CoordT(OwnT,RegionT(DefaultRegionT),StructTT(IdT(PackageCoordinate(StrI("vmdparse"),Vector()),Vector(),StructNameT(StructTemplateNameT(StrI("NotesCollector")),Vector())))), CoordT(BorrowT,RegionT(DefaultRegionT),StructTT(IdT(PackageCoordinate(StrI("parseiter"),Vector()),Vector(),StructNameT(StructTemplateNameT(StrI("ParseIter")),Vector()))))))),LambdaCitizenNameT(LambdaCitizenTemplateNameT(_))))))))) => {
    //        println(instantiationBoundArgs.runeToBoundPrototype.size)
    //        println(instantiationBoundArgs.runeToBoundImpl.size)
    //        println(instantiationBoundArgs.runeToCitizenRuneToReachablePrototype.size)
    //        start here // just run it. it seems to die after 83rd, and we set the pass count to 83.
    //        // it should break when we're adding the broken thing.

            vpass() // InstantiationBoundArgumentsT@5134
          }
          case _ =>
        }
        instantiationNameToInstantiationBounds.put(instantiationId, instantiationBoundArgs)
      }

    //  // This means we've at least started to evaluate this function's body.
    //  // We use this to cut short any infinite looping that might happen when,
    //  // for example, there's a recursive function call.
    //  def declareFunctionSignature(range: RangeS, signature: SignatureT, maybeEnv: Option[FunctionEnvironment]): Unit = {
    //    // The only difference between this and declareNonGlobalFunctionSignature is
    //    // that we put an environment in here.
    //
    //    // This should have been checked outside
    //    vassert(!declaredSignatures.contains(signature))
    //
    //    declaredSignatures += signature -> range
    //    envByFunctionSignature ++= maybeEnv.map(env => Map(signature -> env)).getOrElse(Map())
    //    this
    //  }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn declare_function_return_type(
        &mut self,
        signature: &'t SignatureT<'s, 't>,
        return_type_2: CoordT<'s, 't>,
    ) {
        match self.return_types_by_signature.get(signature) {
            None => {}
            Some(existing) => assert!(*existing == return_type_2),
        }
        self.return_types_by_signature.insert(*signature, return_type_2);
    }
    /*
      def declareFunctionReturnType(signature: SignatureT, returnType2: CoordT): Unit = {
        returnTypesBySignature.get(signature) match {
          case None =>
          case Some(existingReturnType2) => vassert(existingReturnType2 == returnType2)
        }
    //    if (!declaredSignatures.contains(signature)) {
    //      vfail("wot")
    //    }
        returnTypesBySignature += (signature -> returnType2)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn add_function(
        &mut self,
        signature: &'t SignatureT<'s, 't>,
        function: &'t FunctionDefinitionT<'s, 't>,
    ) {
        assert!(
            function.body.result().coord.kind == KindT::Never(NeverT { from_break: false }) ||
            function.body.result().coord == function.header.return_type);

        assert!(!self.signature_to_function.contains_key(signature),
            "wot");

        self.signature_to_function.insert(*signature, function);
    }
    /*
      def addFunction(function: FunctionDefinitionT): Unit = {
    //    vassert(declaredSignatures.contains(function.header.toSignature))
        vassert(
          function.body.result.coord.kind == NeverT(false) ||
          function.body.result.coord == function.header.returnType)

    //    if (!useOptimization) {
    //      Collector.all(function, {
    //        case ReturnTE(innerExpr) => {
    //          vassert(
    //            innerExpr.result.reference.kind == NeverT(false) ||
    //              innerExpr.result.reference == function.header.returnType)
    //        }
    //      })
    //    }

    //    if (functionsByPrototype.contains(function.header.toPrototype)) {
    //      vfail("wot")
    //    }
        if (signatureToFunction.contains(function.header.toSignature)) {
          vfail("wot")
        }

        signatureToFunction.put(function.header.toSignature, function)
    //    functionsByPrototype.put(function.header.toPrototype, function)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn declare_function(
        &mut self,
        call_ranges: &[RangeS<'s>],
        name: &'t IdT<'s, 't>,
    ) {
        // functionDeclaredNames.get(name) match {
        //   case Some(oldFunctionRange) => {
        //     throw CompileErrorExceptionT(FunctionAlreadyExists(oldFunctionRange, callRanges.head, name))
        //   }
        //   case None =>
        // }
        if let Some(_old_function_range) = self.function_declared_names.get(name) {
            panic!("implement CompileErrorExceptionT(FunctionAlreadyExists(oldFunctionRange, callRanges.head, name))");
        }
        // functionDeclaredNames.put(name, callRanges.head)
        self.function_declared_names.insert(*name, call_ranges[0]);
    }
    /*
      def declareFunction(callRanges: List[RangeS], name: IdT[IFunctionNameT]): Unit = {
        functionDeclaredNames.get(name) match {
          case Some(oldFunctionRange) => {
            throw CompileErrorExceptionT(FunctionAlreadyExists(oldFunctionRange, callRanges.head, name))
          }
          case None =>
        }
        functionDeclaredNames.put(name, callRanges.head)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn declare_type(
        &mut self,
        template_name: &'t IdT<'s, 't>,
    ) {
        // vassert(!typeDeclaredNames.contains(templateName))
        assert!(!self.type_declared_names.contains(template_name));
        // typeDeclaredNames += templateName
        self.type_declared_names.insert(*template_name);
    }
    /*
      // We can't declare the struct at the same time as we declare its mutability or environment,
      // see MFDBRE.
      def declareType(templateName: IdT[ITemplateNameT]): Unit = {
        vassert(!typeDeclaredNames.contains(templateName))
        typeDeclaredNames += templateName
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn declare_type_mutability(
        &mut self,
        template_name: &'t IdT<'s, 't>,
        mutability: ITemplataT<'s, 't>,
    ) {
        // vassert(typeDeclaredNames.contains(templateName))
        assert!(self.type_declared_names.contains(template_name));
        // vassert(!typeNameToMutability.contains(templateName))
        assert!(!self.type_name_to_mutability.contains_key(template_name));
        // typeNameToMutability += (templateName -> mutability)
        self.type_name_to_mutability.insert(*template_name, mutability);
    }
    /*
      def declareTypeMutability(
        templateName: IdT[ITemplateNameT],
        mutability: ITemplataT[MutabilityTemplataType]
      ): Unit = {
        vassert(typeDeclaredNames.contains(templateName))
        vassert(!typeNameToMutability.contains(templateName))
        typeNameToMutability += (templateName -> mutability)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn declare_type_sealed(
        &mut self,
        template_name: IdT<'s, 't>,
        sealed: bool,
    ) {
        assert!(self.type_declared_names.contains(&template_name));
        assert!(!self.interface_name_to_sealed.contains_key(&template_name));
        self.interface_name_to_sealed.insert(template_name, sealed);
    }
    /*
      def declareTypeSealed(
        templateName: IdT[IInterfaceTemplateNameT],
        seealed: Boolean
      ): Unit = {
        vassert(typeDeclaredNames.contains(templateName))
        vassert(!interfaceNameToSealed.contains(templateName))
        interfaceNameToSealed += (templateName -> seealed)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn declare_function_inner_env(
        &mut self,
        name_t: &'t IdT<'s, 't>,
        env: IInDenizenEnvironmentT<'s, 't>,
    ) {
        // vassert(functionDeclaredNames.contains(nameT))
        assert!(self.function_declared_names.contains_key(name_t));
        // vassert(!functionNameToInnerEnv.contains(nameT))
        assert!(!self.function_name_to_inner_env.contains_key(name_t));
        // functionNameToInnerEnv += (nameT -> env)
        self.function_name_to_inner_env.insert(*name_t, env);
    }
    /*
      def declareFunctionInnerEnv(
        nameT: IdT[IFunctionNameT],
        env: IInDenizenEnvironmentT,
      ): Unit = {
        vassert(functionDeclaredNames.contains(nameT))
        // One should declare the outer env first
        vassert(!functionNameToInnerEnv.contains(nameT))
    //    vassert(nameT == env.fullName)
        functionNameToInnerEnv += (nameT -> env)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn declare_function_outer_env(
        &mut self,
        name_t: &'t IdT<'s, 't>,
        env: IInDenizenEnvironmentT<'s, 't>,
    ) {
        // vassert(!functionNameToOuterEnv.contains(nameT))
        assert!(!self.function_name_to_outer_env.contains_key(name_t));
        // functionNameToOuterEnv += (nameT -> env)
        self.function_name_to_outer_env.insert(*name_t, env);
    }
    /*
      def declareFunctionOuterEnv(
        nameT: IdT[IFunctionTemplateNameT],
        env: IInDenizenEnvironmentT,
      ): Unit = {
        vassert(!functionNameToOuterEnv.contains(nameT))
        //    vassert(nameT == env.fullName)
        functionNameToOuterEnv += (nameT -> env)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn declare_type_outer_env(
        &mut self,
        name_t: &'t IdT<'s, 't>,
        env: IInDenizenEnvironmentT<'s, 't>,
    ) {
        // vassert(typeDeclaredNames.contains(nameT))
        assert!(self.type_declared_names.contains(name_t));
        // vassert(!typeNameToOuterEnv.contains(nameT))
        assert!(!self.type_name_to_outer_env.contains_key(name_t));
        // vassert(nameT == env.id)
        // (skipped — requires pattern-matching all IInDenizenEnvironmentT variants to extract id)
        // typeNameToOuterEnv += (nameT -> env)
        self.type_name_to_outer_env.insert(*name_t, env);
    }
    /*
      def declareTypeOuterEnv(
        nameT: IdT[ITemplateNameT],
        env: IInDenizenEnvironmentT,
      ): Unit = {
        vassert(typeDeclaredNames.contains(nameT))
        vassert(!typeNameToOuterEnv.contains(nameT))
        vassert(nameT == env.id)
        typeNameToOuterEnv += (nameT -> env)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn declare_type_inner_env(
        &mut self,
        template_id: &'t IdT<'s, 't>,
        env: IInDenizenEnvironmentT<'s, 't>,
    ) {
        // vassert(typeDeclaredNames.contains(templateId))
        assert!(self.type_declared_names.contains(template_id));
        // One should declare the outer env first
        // vassert(typeNameToOuterEnv.contains(templateId))
        assert!(self.type_name_to_outer_env.contains_key(template_id));
        // vassert(!typeNameToInnerEnv.contains(templateId))
        assert!(!self.type_name_to_inner_env.contains_key(template_id));
        // typeNameToInnerEnv += (templateId -> env)
        self.type_name_to_inner_env.insert(*template_id, env);
    }
    /*
      def declareTypeInnerEnv(
        templateId: IdT[ITemplateNameT],
        env: IInDenizenEnvironmentT,
      ): Unit = {
    //    val templateFullName = TemplataCompiler.getTemplate(nameT)
        vassert(typeDeclaredNames.contains(templateId))
        // One should declare the outer env first
        vassert(typeNameToOuterEnv.contains(templateId))
        vassert(!typeNameToInnerEnv.contains(templateId))
        //    vassert(nameT == env.fullName)
        typeNameToInnerEnv += (templateId -> env)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn add_struct(
        &mut self,
        struct_def: &'t StructDefinitionT<'s, 't>,
    ) {
        if struct_def.mutability == ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) {
            struct_def.members.iter().for_each(|m| {
                match m {
                    IStructMemberT::Normal(NormalStructMemberT { tyype: IMemberTypeT::Address(_), .. }) => {
                        panic!("Immutable structs cant contain address members");
                    }
                    IStructMemberT::Normal(NormalStructMemberT { tyype: IMemberTypeT::Reference(r), .. }) => {
                        if r.reference.ownership != OwnershipT::Share {
                            panic!("ImmutableP contains a non-immutable!");
                        }
                    }
                    IStructMemberT::Variadic(_) => {
                        panic!("implement: immutable struct with variadic members");
                    }
                }
            });
        }
        assert!(self.type_name_to_mutability.contains_key(&struct_def.template_name));
        assert!(!self.struct_template_name_to_definition.contains_key(&struct_def.template_name));
        self.struct_template_name_to_definition.insert(struct_def.template_name, struct_def);
    }
    /*
      def addStruct(structDef: StructDefinitionT): Unit = {
        if (structDef.mutability == MutabilityTemplataT(ImmutableT)) {
          structDef.members.foreach({
            case NormalStructMemberT(name, variability, AddressMemberTypeT(reference)) => {
              vwat() // Immutable structs cant contain address members
            }
            case NormalStructMemberT(name, variability, ReferenceMemberTypeT(reference)) => {
              if (reference.ownership != ShareT) {
                vfail("ImmutableP contains a non-immutable!")
              }
            }
            case VariadicStructMemberT(name, tyype) => {
              vimpl() // We dont yet have immutable structs with variadic members
            }
          })
        }
        vassert(typeNameToMutability.contains(structDef.templateName))
        vassert(!structTemplateNameToDefinition.contains(structDef.templateName))
        structTemplateNameToDefinition += (structDef.templateName -> structDef)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn add_interface(
        &mut self,
        interface_def: &'t InterfaceDefinitionT<'s, 't>,
    ) {
        assert!(self.type_name_to_mutability.contains_key(&interface_def.template_name));
        assert!(self.interface_name_to_sealed.contains_key(&interface_def.template_name));
        assert!(!self.interface_template_name_to_definition.contains_key(&interface_def.template_name));
        self.interface_template_name_to_definition.insert(interface_def.template_name, interface_def);
    }
    /*
      def addInterface(interfaceDef: InterfaceDefinitionT): Unit = {
        vassert(typeNameToMutability.contains(interfaceDef.templateName))
        vassert(interfaceNameToSealed.contains(interfaceDef.templateName))
        vassert(!interfaceTemplateNameToDefinition.contains(interfaceDef.templateName))
        interfaceTemplateNameToDefinition += (interfaceDef.templateName -> interfaceDef)
      }

    //  def addStaticSizedArray(ssaTT: StaticSizedArrayTT): Unit = {
    //    val contentsStaticSizedArrayTT(size, elementType, mutability, variability) = ssaTT
    //    staticSizedArrayTypes += ((size, elementType, mutability, variability) -> ssaTT)
    //  }
    //
    //  def addRuntimeSizedArray(rsaTT: RuntimeSizedArrayTT): Unit = {
    //    val contentsRuntimeSizedArrayTT(elementType, mutability) = rsaTT
    //    runtimeSizedArrayTypes += ((elementType, mutability) -> rsaTT)
    //  }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn add_impl(
        &mut self,
        impl_t: &'t ImplT<'s, 't>,
    ) {
        assert!(!self.all_impls.contains_key(&impl_t.template_id));
        self.all_impls.insert(impl_t.template_id, impl_t);
        self.sub_citizen_template_to_impls
            .entry(impl_t.sub_citizen_template_id)
            .or_insert_with(Vec::new)
            .push(impl_t);
        self.super_interface_template_to_impls
            .entry(impl_t.super_interface_template_id)
            .or_insert_with(Vec::new)
            .push(impl_t);
    }
    /*
      def addImpl(impl: ImplT): Unit = {
        vassert(!allImpls.contains(impl.templateId))
        allImpls.put(impl.templateId, impl)
        subCitizenTemplateToImpls.put(
          impl.subCitizenTemplateId,
          subCitizenTemplateToImpls.getOrElse(impl.subCitizenTemplateId, Vector()) :+ impl)
        superInterfaceTemplateToImpls.put(
          impl.superInterfaceTemplateId,
          superInterfaceTemplateToImpls.getOrElse(impl.superInterfaceTemplateId, Vector()) :+ impl)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_parent_impls_for_sub_citizen_template(
        &self,
        sub_citizen_template: IdT<'s, 't>,
    ) -> Vec<&'t ImplT<'s, 't>> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
    /*
      def getParentImplsForSubCitizenTemplate(subCitizenTemplate: IdT[ICitizenTemplateNameT]): Vector[ImplT] = {
        subCitizenTemplateToImpls.getOrElse(subCitizenTemplate, Vector[ImplT]())
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_child_impls_for_super_interface_template(
        &self,
        super_interface_template: IdT<'s, 't>,
    ) -> Vec<&'t ImplT<'s, 't>> {
        self.super_interface_template_to_impls
            .get(&super_interface_template)
            .map(|v| v.clone())
            .unwrap_or_default()
    }
    /*
      def getChildImplsForSuperInterfaceTemplate(superInterfaceTemplate: IdT[IInterfaceTemplateNameT]): Vector[ImplT] = {
        superInterfaceTemplateToImpls.getOrElse(superInterfaceTemplate, Vector[ImplT]())
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn add_kind_export(
        &mut self,
        range: RangeS<'s>,
        kind: KindT<'s, 't>,
        id: IdT<'s, 't>,
        exported_name: StrI<'s>,
        interner: &TypingInterner<'s, 't>,
    ) {
        let export = interner.alloc(KindExportT { range, tyype: kind, id, exported_name });
        self.kind_exports.push(export);
    }
    /*
      def addKindExport(range: RangeS, kind: KindT, id: IdT[ExportNameT], exportedName: StrI): Unit = {
        kindExports += KindExportT(range, kind, id, exportedName)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn add_function_export(
        &mut self,
        range: RangeS<'s>,
        function: &'t PrototypeT<'s, 't>,
        export_id: IdT<'s, 't>,
        exported_name: StrI<'s>,
        interner: &TypingInterner<'s, 't>,
    ) {
        assert!(self.get_instantiation_bounds(interner, function.id).is_some());
        let export = interner.alloc(FunctionExportT { range, prototype: *function, export_id, exported_name });
        self.function_exports.push(export);
    }
    /*
      def addFunctionExport(range: RangeS, function: PrototypeT[IFunctionNameT], exportId: IdT[ExportNameT], exportedName: StrI): Unit = {
        vassert(getInstantiationBounds(function.id).nonEmpty)
        functionExports += FunctionExportT(range, function, exportId, exportedName)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn add_kind_extern(
        &mut self,
        kind: KindT<'s, 't>,
        package_coord: PackageCoordinate<'s>,
        exported_name: StrI<'s>,
    ) {
        panic!("Unimplemented: Slab 10 — body migration");
    }
    /*
      def addKindExtern(kind: KindT, packageCoord: PackageCoordinate, exportedName: StrI): Unit = {
        kindExterns += KindExternT(kind, packageCoord, exportedName)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn add_function_extern(
        &mut self,
        range: RangeS<'s>,
        extern_placeholdered_id: IdT<'s, 't>,
        function: &'t PrototypeT<'s, 't>,
        exported_name: StrI<'s>,
        generic_parameter_inheritance: Option<GenericParametersInheritance>,
        interner: &TypingInterner<'s, 't>,
    ) {
        let function_extern = interner.alloc(FunctionExternT { range, extern_placeholdered_id, prototype: *function, extern_name: exported_name, generic_parameter_inheritance });
        self.function_externs.push(function_extern);
    }
    /*
      def addFunctionExtern(
        range: RangeS,
        externPlaceholderedId: IdT[ExternNameT],
        function: PrototypeT[IFunctionNameT],
        exportedName: StrI,
        genericParameterInheritance: Option[GenericParametersInheritance]): Unit = {
        functionExterns +=
          FunctionExternT(
            range, externPlaceholderedId, function, exportedName, genericParameterInheritance)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn defer_evaluating_function_body(
        &mut self,
        devf: DeferredActionT<'s, 't>,
    ) {
        let prototype = match &devf {
            DeferredActionT::EvaluateFunctionBody { prototype, .. } => *prototype,
            _ => panic!("Expected EvaluateFunctionBody"),
        };
        self.deferred_function_body_compiles.insert(*prototype, devf);
    }
    /*
      def deferEvaluatingFunctionBody(devf: DeferredEvaluatingFunctionBody): Unit = {
        deferredFunctionBodyCompiles.put(devf.prototypeT, devf)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn defer_evaluating_function(
        &mut self,
        devf: DeferredActionT<'s, 't>,
    ) {
        let name = match &devf {
            DeferredActionT::EvaluateFunction { name, .. } => *name,
            _ => panic!("Expected EvaluateFunction"),
        };
        self.deferred_function_compiles.insert(*name, devf);
    }
    /*
      def deferEvaluatingFunction(devf: DeferredEvaluatingFunction): Unit = {
        deferredFunctionCompiles.put(devf.name, devf)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn struct_declared(
        &self,
        template_name: IdT<'s, 't>,
    ) -> bool {
        panic!("Unimplemented: Slab 10 — body migration");
    }
    /*
      def structDeclared(templateName: IdT[IStructTemplateNameT]): Boolean = {
        // This is the only place besides StructDefinition2 and declareStruct thats allowed to make one of these
    //    val templateName = StructTT(fullName)
        typeDeclaredNames.contains(templateName)
      }

    //  def prototypeDeclared(fullName: FullNameT[IFunctionNameT]): Option[PrototypeT] = {
    //    declaredSignatures.find(_._1.fullName == fullName) match {
    //      case None => None
    //      case Some((sig, _)) => {
    //        returnTypesBySignature.get(sig) match {
    //          case None => None
    //          case Some(ret) => Some(ast.PrototypeT(sig.fullName, ret))
    //        }
    //      }
    //    }
    //  }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn lookup_mutability(
        &self,
        template_name: IdT<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        match self.type_name_to_mutability.get(&template_name) {
            None => panic!("Still figuring out mutability for struct: {:?}", template_name),
            Some(m) => *m,
        }
    }
    /*
      def lookupMutability(templateName: IdT[ITemplateNameT]): ITemplataT[MutabilityTemplataType] = {
        // If it has a structTT, then we've at least started to evaluate this citizen
        typeNameToMutability.get(templateName) match {
          case None => vfail("Still figuring out mutability for struct: " + templateName) // See MFDBRE
          case Some(m) => m
        }
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn lookup_sealed(
        &self,
        template_name: IdT<'s, 't>,
    ) -> bool {
        match self.interface_name_to_sealed.get(&template_name) {
            None => panic!("vfail: Still figuring out sealed for struct: {:?}", template_name), // See MFDBRE
            Some(m) => *m,
        }
    }
    /*
      def lookupSealed(templateName: IdT[IInterfaceTemplateNameT]): Boolean = {
        // If it has a structTT, then we've at least started to evaluate this citizen
        interfaceNameToSealed.get(templateName) match {
          case None => vfail("Still figuring out sealed for struct: " + templateName) // See MFDBRE
          case Some(m) => m
        }
      }

    //  def lookupCitizen(citizenRef: CitizenRefT): CitizenDefinitionT = {
    //    citizenRef match {
    //      case s @ StructTT(_) => lookupStruct(s)
    //      case i @ InterfaceTT(_) => lookupInterface(i)
    //    }
    //  }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn interface_declared(
        &self,
        template_name: IdT<'s, 't>,
    ) -> bool {
        panic!("Unimplemented: Slab 10 — body migration");
    }
    /*
      def interfaceDeclared(templateName: IdT[ITemplateNameT]): Boolean = {
        // This is the only place besides InterfaceDefinition2 and declareInterface thats allowed to make one of these
        typeDeclaredNames.contains(templateName)
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn lookup_struct(
        &self,
        struct_tt: IdT<'s, 't>,
        compiler: &Compiler<'s, '_, 't>,
    ) -> &'t StructDefinitionT<'s, 't> {
        let template_id = compiler.get_struct_template(struct_tt);
        self.lookup_struct_template(template_id)
    }
    /*
      def lookupStruct(structTT: IdT[IStructNameT]): StructDefinitionT = {
        lookupStructTemplate(TemplataCompiler.getStructTemplate(structTT))
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn lookup_struct_template(
        &self,
        template_name: IdT<'s, 't>,
    ) -> &'t StructDefinitionT<'s, 't> {
        *self.struct_template_name_to_definition.get(&template_name)
            .expect("Struct template not found")
    }
    /*
      def lookupStructTemplate(templateName: IdT[IStructTemplateNameT]): StructDefinitionT = {
        vassertSome(structTemplateNameToDefinition.get(templateName))
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn lookup_interface(
        &self,
        interface_tt: InterfaceTT<'s, 't>,
    ) -> &'t InterfaceDefinitionT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
    /*
      def lookupInterface(interfaceTT: InterfaceTT): InterfaceDefinitionT = {
        lookupInterface(TemplataCompiler.getInterfaceTemplate(interfaceTT.id))
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn lookup_interface_by_template_name(
        &self,
        template_name: IdT<'s, 't>,
    ) -> &'t InterfaceDefinitionT<'s, 't> {
        match self.interface_template_name_to_definition.get(&template_name) {
            None => panic!("vfail: vassertSome: lookupInterface templateName not found: {:?}", template_name),
            Some(d) => *d,
        }
    }
    /*
      def lookupInterface(templateName: IdT[IInterfaceTemplateNameT]): InterfaceDefinitionT = {
        vassertSome(interfaceTemplateNameToDefinition.get(templateName))
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn lookup_citizen_by_template_name(
        &self,
        template_name: IdT<'s, 't>,
    ) -> CitizenDefinitionT<'s, 't> {
        match template_name.local_name {
            INameT::AnonymousSubstructTemplate(_) => CitizenDefinitionT::Struct(self.lookup_struct_template(template_name)),
            INameT::StructTemplate(_) => CitizenDefinitionT::Struct(self.lookup_struct_template(template_name)),
            INameT::InterfaceTemplate(_) => CitizenDefinitionT::Interface(self.lookup_interface_by_template_name(template_name)),
            _ => panic!("lookup_citizen_by_template_name: unexpected local_name variant: {:?}", template_name),
        }
    }
    /*
      def lookupCitizen(templateName: IdT[ICitizenTemplateNameT]): CitizenDefinitionT = {
        val IdT(packageCoord, initSteps, last) = templateName
        last match {
          case s @ AnonymousSubstructTemplateNameT(_) => lookupStructTemplate(IdT(packageCoord, initSteps, s))
          case s @ StructTemplateNameT(_) => lookupStructTemplate(IdT(packageCoord, initSteps, s))
          case s @ InterfaceTemplateNameT(_) => lookupInterface(IdT(packageCoord, initSteps, s))
        }
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn lookup_citizen_by_tt(
        &self,
        citizen_tt: ICitizenTT<'s, 't>,
    ) -> &'t CitizenDefinitionT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
    /*
      def lookupCitizen(citizenTT: ICitizenTT): CitizenDefinitionT = {
        citizenTT match {
          case s @ StructTT(_) => lookupStruct(s.id)
          case s @ InterfaceTT(_) => lookupInterface(s)
        }
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_all_structs(&self) -> Vec<&'t StructDefinitionT<'s, 't>> {
        self.struct_template_name_to_definition.values().copied().collect()
    }
    /*
      def getAllStructs(): Iterable[StructDefinitionT] = structTemplateNameToDefinition.values
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_all_interfaces(&self) -> Vec<&'t InterfaceDefinitionT<'s, 't>> {
        self.interface_template_name_to_definition.values().copied().collect()
    }
    /*
      def getAllInterfaces(): Iterable[InterfaceDefinitionT] = interfaceTemplateNameToDefinition.values
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_all_functions(&self) -> Vec<&'t FunctionDefinitionT<'s, 't>> {
        self.signature_to_function.values().copied().collect()
    }
    /*
      def getAllFunctions(): Iterable[FunctionDefinitionT] = signatureToFunction.values
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_all_impls(&self) -> Vec<&'t ImplT<'s, 't>> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
    /*
      def getAllImpls(): Iterable[ImplT] = allImpls.values
    //  def getAllStaticSizedArrays(): Iterable[StaticSizedArrayTT] = staticSizedArrayTypes.values
    //  def getAllRuntimeSizedArrays(): Iterable[RuntimeSizedArrayTT] = runtimeSizedArrayTypes.values
    //  def getKindToDestructorMap(): Map[KindT, PrototypeT] = kindToDestructor.toMap

    //  def getStaticSizedArrayType(size: ITemplata[IntegerTemplataType], mutability: ITemplata[MutabilityTemplataType], variability: ITemplata[VariabilityTemplataType], elementType: CoordT): Option[StaticSizedArrayTT] = {
    //    staticSizedArrayTypes.get((size, mutability, variability, elementType))
    //  }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_env_for_function_signature(
        &self,
        sig: &'t SignatureT<'s, 't>,
    ) -> &'t FunctionEnvironmentT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
    /*
      def getEnvForFunctionSignature(sig: SignatureT): FunctionEnvironmentT = {
        vassertSome(envByFunctionSignature.get(sig))
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_outer_env_for_type(
        &self,
        range: &[RangeS<'s>],
        name: IdT<'s, 't>,
    ) -> IInDenizenEnvironmentT<'s, 't> {
        match self.type_name_to_outer_env.get(&name) {
            None => {
                panic!("No outer env for type: {:?}", name);
            }
            Some(x) => *x,
        }
    }
    /*
      def getOuterEnvForType(range: List[RangeS], name: IdT[ITemplateNameT]): IInDenizenEnvironmentT = {
        typeNameToOuterEnv.get(name) match {
          case None => {
            throw CompileErrorExceptionT(RangedInternalErrorT(range, "No outer env for type: " + name))
          }
          case Some(x) => x
        }
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_inner_env_for_type(
        &self,
        name: IdT<'s, 't>,
    ) -> IInDenizenEnvironmentT<'s, 't> {
        *self.type_name_to_inner_env.get(&name).unwrap()
    }
    /*
      def getInnerEnvForType(name: IdT[ITemplateNameT]): IInDenizenEnvironmentT = {
        vassertSome(typeNameToInnerEnv.get(name))
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_inner_env_for_function(
        &self,
        name: IdT<'s, 't>,
    ) -> IInDenizenEnvironmentT<'s, 't> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
    /*
      def getInnerEnvForFunction(name: IdT[INameT]): IInDenizenEnvironmentT = {
        vassertSome(functionNameToInnerEnv.get(name))
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_outer_env_for_function(
        &self,
        name: IdT<'s, 't>,
    ) -> IInDenizenEnvironmentT<'s, 't> {
        *self.function_name_to_outer_env.get(&name)
            .expect("vassertSome: get_outer_env_for_function")
    }
    /*
      def getOuterEnvForFunction(name: IdT[IFunctionTemplateNameT]): IInDenizenEnvironmentT = {
        vassertSome(functionNameToOuterEnv.get(name))
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_return_type_for_signature(
        &self,
        sig: &'t SignatureT<'s, 't>,
    ) -> Option<CoordT<'s, 't>> {
        panic!("Unimplemented: Slab 10 — body migration");
    }
    /*
      def getReturnTypeForSignature(sig: SignatureT): Option[CoordT] = {
        returnTypesBySignature.get(sig)
      }
    //  def getDeclaredSignatureOrigin(sig: SignatureT): Option[RangeS] = {
    //    declaredSignatures.get(sig)
    //  }
    //  def getDeclaredSignatureOrigin(name: FullNameT[IFunctionNameT]): Option[RangeS] = {
    //    declaredSignatures.get(ast.SignatureT(name))
    //  }
    //  def getRuntimeSizedArray(mutabilityT: ITemplata[MutabilityTemplataType], elementType: CoordT): Option[RuntimeSizedArrayTT] = {
    //    runtimeSizedArrayTypes.get((mutabilityT, elementType))
    //  }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_kind_exports(&self) -> Vec<&'t KindExportT<'s, 't>> {
        self.kind_exports.clone()
    }
    /*
      def getKindExports: Vector[KindExportT] = {
        kindExports.toVector
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_function_exports(&self) -> Vec<&'t FunctionExportT<'s, 't>> {
        self.function_exports.clone()
    }
    /*
      def getFunctionExports: Vector[FunctionExportT] = {
        functionExports.toVector
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_kind_externs(&self) -> Vec<&'t KindExternT<'s, 't>> {
        self.kind_externs.clone()
    }
    /*
      def getKindExterns: Vector[KindExternT] = {
        kindExterns.toVector
      }
    */
}
impl<'s, 't> CompilerOutputs<'s, 't>
where 's: 't,
{
    pub fn get_function_externs(&self) -> Vec<&'t FunctionExternT<'s, 't>> {
        self.function_externs.clone()
    }
    /*
      def getFunctionExterns: Vector[FunctionExternT] = {
        functionExterns.toVector
      }
    }
    */
}
