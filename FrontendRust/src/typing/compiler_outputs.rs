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
use crate::interner::{Interner, StrI};
use std::collections::HashMap;
use crate::utils::range::RangeS;
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

pub struct DeferredEvaluatingFunctionBody<'s, 't> {
    prototype_t: PrototypeT<'s, 't>,
    call: fn(),
}
/*
case class DeferredEvaluatingFunctionBody(
  prototypeT: PrototypeT[IFunctionNameT],
  call: (CompilerOutputs) => Unit)
*/
pub struct DeferredEvaluatingFunction<'s, 't> {
    name: IdT<'s, 't>,
    call: fn(),
}
/*
case class DeferredEvaluatingFunction(
  name: IdT[INameT],
  call: (CompilerOutputs) => Unit)
*/
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct CompilerOutputs<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
impl<'s, 't> CompilerOutputs<'s, 't> {
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
fn count_denizens() -> i32 { panic!("Unimplemented: count_denizens"); }
/*
  def countDenizens(): Int = {
//    staticSizedArrayTypes.size +
//      runtimeSizedArrayTypes.size +
      signatureToFunction.size +
      structTemplateNameToDefinition.size +
      interfaceTemplateNameToDefinition.size
  }
*/
fn peek_next_deferred_function_body_compile<'s, 't>() -> Option<DeferredEvaluatingFunctionBody<'s, 't>> { panic!("Unimplemented: peek_next_deferred_function_body_compile"); }
/*
  def peekNextDeferredFunctionBodyCompile(): Option[DeferredEvaluatingFunctionBody] = {
    deferredFunctionBodyCompiles.headOption.map(_._2)
  }
*/
fn mark_deferred_function_body_compiled<'s, 't>(prototype_t: PrototypeT<'s, 't>) { panic!("Unimplemented: mark_deferred_function_body_compiled"); }
/*
  def markDeferredFunctionBodyCompiled(prototypeT: PrototypeT[IFunctionNameT]): Unit = {
    vassert(prototypeT == vassertSome(deferredFunctionBodyCompiles.headOption)._1)
    finishedDeferredFunctionBodyCompiles += prototypeT
    deferredFunctionBodyCompiles -= prototypeT
  }
*/
fn peek_next_deferred_function_compile<'s, 't>() -> Option<DeferredEvaluatingFunction<'s, 't>> { panic!("Unimplemented: peek_next_deferred_function_compile"); }
/*
  def peekNextDeferredFunctionCompile(): Option[DeferredEvaluatingFunction] = {
    deferredFunctionCompiles.headOption.map(_._2)
  }
*/
fn mark_deferred_function_compiled<'s, 't>(name: IdT<'s, 't>) { panic!("Unimplemented: mark_deferred_function_compiled"); }
/*
  def markDeferredFunctionCompiled(name: IdT[INameT]): Unit = {
    vassert(name == vassertSome(deferredFunctionCompiles.headOption)._1)
    finishedDeferredFunctionCompiles += name
    deferredFunctionCompiles -= name
  }
*/
fn get_instantiation_name_to_function_bound_to_rune<'s, 't>() -> HashMap<IdT<'s, 't>, InstantiationBoundArgumentsT<'s, 't>> { panic!("Unimplemented: get_instantiation_name_to_function_bound_to_rune"); }
/*
  def getInstantiationNameToFunctionBoundToRune(): Map[IdT[IInstantiationNameT], InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]] = {
    instantiationNameToInstantiationBounds.toMap
  }
*/
fn lookup_function<'s, 't>(signature: SignatureT<'s, 't>) -> Option<FunctionDefinitionT<'s, 't>> { panic!("Unimplemented: lookup_function"); }
/*
  def lookupFunction(signature: SignatureT): Option[FunctionDefinitionT] = {
    signatureToFunction.get(signature)
  }
*/
fn get_instantiation_bounds<'s, 't>(instantiation_id: IdT<'s, 't>) -> Option<InstantiationBoundArgumentsT<'s, 't>> { panic!("Unimplemented: get_instantiation_bounds"); }
/*
  def getInstantiationBounds(
    instantiationId: IdT[IInstantiationNameT]):
  Option[InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]] = {
    instantiationNameToInstantiationBounds.get(instantiationId)
  }
*/
fn add_instantiation_bounds<'s, 't>(sanity_check: bool, interner: Interner<'s>, original_calling_template_id: IdT<'s, 't>, instantiation_id: IdT<'s, 't>, instantiation_bound_args: InstantiationBoundArgumentsT<'s, 't>) { panic!("Unimplemented: add_instantiation_bounds"); }
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
      case IdT(_,Vector(),FunctionNameT(FunctionTemplateNameT(StrI("Bork"),_),Vector(CoordTemplataT(CoordT(_,RegionT(),IntT(32)))),Vector(CoordT(_,RegionT(),IntT(32))))) => {
        vpass()
      }
      case IdT(_,Vector(),InterfaceNameT(InterfaceTemplateNameT(StrI("XOpt")),Vector(CoordTemplataT(CoordT(own,RegionT(),KindPlaceholderT(IdT(_,Vector(InterfaceTemplateNameT(StrI("XOpt")), FunctionTemplateNameT(StrI("harvest"),_), OverrideDispatcherTemplateNameT(IdT(_,Vector(),ImplTemplateNameT(_)))),KindPlaceholderNameT(KindPlaceholderTemplateNameT(0,DispatcherRuneFromImplS(CodeRuneS(StrI("T")))))))))))) => {
        vpass()
      }
      case IdT(_,Vector(),InterfaceNameT(InterfaceTemplateNameT(StrI("IXOption")),Vector(CoordTemplataT(CoordT(own,RegionT(),KindPlaceholderT(IdT(_,Vector(FunctionTemplateNameT(StrI("drop"),_), OverrideDispatcherTemplateNameT(IdT(_,Vector(),ImplTemplateNameT(_)))),KindPlaceholderNameT(KindPlaceholderTemplateNameT(0,DispatcherRuneFromImplS(CodeRuneS(StrI("T")))))))))))) => {
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
      case IdT(PackageCoordinate(StrI("stdlib"),Vector(StrI("ifunction"))),Vector(),AnonymousSubstructNameT(AnonymousSubstructTemplateNameT(InterfaceTemplateNameT(StrI("IFunction1"))),Vector(MutabilityTemplataT(MutableT), CoordTemplataT(CoordT(BorrowT,RegionT(),StructTT(IdT(PackageCoordinate(StrI("parseiter"),Vector()),Vector(),StructNameT(StructTemplateNameT(StrI("ParseIter")),Vector()))))), CoordTemplataT(CoordT(ShareT,RegionT(),BoolT())), CoordTemplataT(CoordT(ShareT,RegionT(),StructTT(IdT(PackageCoordinate(StrI("vmdparse"),Vector()),Vector(FunctionNameT(FunctionTemplateNameT(StrI("parseSlice"),_),Vector(),Vector(CoordT(BorrowT,RegionT(),StructTT(IdT(PackageCoordinate(StrI("stdlib"),Vector(StrI("path"))),Vector(),StructNameT(StructTemplateNameT(StrI("Path")),Vector())))), CoordT(OwnT,RegionT(),StructTT(IdT(PackageCoordinate(StrI("vmdparse"),Vector()),Vector(),StructNameT(StructTemplateNameT(StrI("NotesCollector")),Vector())))), CoordT(BorrowT,RegionT(),StructTT(IdT(PackageCoordinate(StrI("parseiter"),Vector()),Vector(),StructNameT(StructTemplateNameT(StrI("ParseIter")),Vector()))))))),LambdaCitizenNameT(LambdaCitizenTemplateNameT(_))))))))) => {
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
fn declare_function_return_type(signature: SignatureT, return_type_2: CoordT) { panic!("Unimplemented: declare_function_return_type"); }
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
fn add_function(function: FunctionDefinitionT) { panic!("Unimplemented: add_function"); }
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
fn declare_function<'s, 't>(call_ranges: Vec<RangeS<'s>>, name: IdT<'s, 't>) { panic!("Unimplemented: declare_function"); }
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
fn declare_type<'s, 't>(template_name: IdT<'s, 't>) { panic!("Unimplemented: declare_type"); }
/*
  // We can't declare the struct at the same time as we declare its mutability or environment,
  // see MFDBRE.
  def declareType(templateName: IdT[ITemplateNameT]): Unit = {
    vassert(!typeDeclaredNames.contains(templateName))
    typeDeclaredNames += templateName
  }
*/
fn declare_type_mutability<'s, 't>(template_name: IdT<'s, 't>, mutability: ITemplataT<'s, 't>) { panic!("Unimplemented: declare_type_mutability"); }
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
fn declare_type_sealed<'s, 't>(template_name: IdT<'s, 't>, sealed: bool) { panic!("Unimplemented: declare_type_sealed"); }
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
fn declare_function_inner_env<'s, 't>(name_t: IdT<'s, 't>, env: IInDenizenEnvironmentT<'s, 't>) { panic!("Unimplemented: declare_function_inner_env"); }
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
fn declare_function_outer_env<'s, 't>(name_t: IdT<'s, 't>, env: IInDenizenEnvironmentT<'s, 't>) { panic!("Unimplemented: declare_function_outer_env"); }
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
fn declare_type_outer_env<'s, 't>(name_t: IdT<'s, 't>, env: IInDenizenEnvironmentT<'s, 't>) { panic!("Unimplemented: declare_type_outer_env"); }
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
fn declare_type_inner_env<'s, 't>(template_id: IdT<'s, 't>, env: IInDenizenEnvironmentT<'s, 't>) { panic!("Unimplemented: declare_type_inner_env"); }
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
fn add_struct(struct_def: StructDefinitionT) { panic!("Unimplemented: add_struct"); }
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
fn add_interface(interface_def: InterfaceDefinitionT) { panic!("Unimplemented: add_interface"); }
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
fn add_impl(impl_t: ImplT) { panic!("Unimplemented: add_impl"); }
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
fn get_parent_impls_for_sub_citizen_template<'s, 't>(sub_citizen_template: IdT<'s, 't>) -> Vec<ImplT<'s, 't>> { panic!("Unimplemented: get_parent_impls_for_sub_citizen_template"); }
/*
  def getParentImplsForSubCitizenTemplate(subCitizenTemplate: IdT[ICitizenTemplateNameT]): Vector[ImplT] = {
    subCitizenTemplateToImpls.getOrElse(subCitizenTemplate, Vector[ImplT]())
  }
*/
fn get_child_impls_for_super_interface_template<'s, 't>(super_interface_template: IdT<'s, 't>) -> Vec<ImplT<'s, 't>> { panic!("Unimplemented: get_child_impls_for_super_interface_template"); }
/*
  def getChildImplsForSuperInterfaceTemplate(superInterfaceTemplate: IdT[IInterfaceTemplateNameT]): Vector[ImplT] = {
    superInterfaceTemplateToImpls.getOrElse(superInterfaceTemplate, Vector[ImplT]())
  }
*/
fn add_kind_export<'s, 't>(range: RangeS<'s>, kind: KindT<'s, 't>, id: IdT<'s, 't>, exported_name: StrI<'s>) { panic!("Unimplemented: add_kind_export"); }
/*
  def addKindExport(range: RangeS, kind: KindT, id: IdT[ExportNameT], exportedName: StrI): Unit = {
    kindExports += KindExportT(range, kind, id, exportedName)
  }
*/
fn add_function_export<'s, 't>(range: RangeS<'s>, function: PrototypeT<'s, 't>, export_id: IdT<'s, 't>, exported_name: StrI<'s>) { panic!("Unimplemented: add_function_export"); }
/*
  def addFunctionExport(range: RangeS, function: PrototypeT[IFunctionNameT], exportId: IdT[ExportNameT], exportedName: StrI): Unit = {
    vassert(getInstantiationBounds(function.id).nonEmpty)
    functionExports += FunctionExportT(range, function, exportId, exportedName)
  }
*/
fn add_kind_extern<'s, 't>(kind: KindT<'s, 't>, package_coord: PackageCoordinate<'s>, exported_name: StrI<'s>) { panic!("Unimplemented: add_kind_extern"); }
/*
  def addKindExtern(kind: KindT, packageCoord: PackageCoordinate, exportedName: StrI): Unit = {
    kindExterns += KindExternT(kind, packageCoord, exportedName)
  }
*/
fn add_function_extern<'s, 't>(range: RangeS<'s>, extern_placeholdered_id: IdT<'s, 't>, function: PrototypeT<'s, 't>, exported_name: StrI<'s>) { panic!("Unimplemented: add_function_extern"); }
/*
  def addFunctionExtern(range: RangeS, externPlaceholderedId: IdT[ExternNameT], function: PrototypeT[IFunctionNameT], exportedName: StrI): Unit = {
    functionExterns += FunctionExternT(range, externPlaceholderedId, function, exportedName)
  }
*/
fn defer_evaluating_function_body<'s, 't>(devf: DeferredEvaluatingFunctionBody<'s, 't>) { panic!("Unimplemented: defer_evaluating_function_body"); }
/*
  def deferEvaluatingFunctionBody(devf: DeferredEvaluatingFunctionBody): Unit = {
    deferredFunctionBodyCompiles.put(devf.prototypeT, devf)
  }
*/
fn defer_evaluating_function<'s, 't>(devf: DeferredEvaluatingFunction<'s, 't>) { panic!("Unimplemented: defer_evaluating_function"); }
/*
  def deferEvaluatingFunction(devf: DeferredEvaluatingFunction): Unit = {
    deferredFunctionCompiles.put(devf.name, devf)
  }
*/
fn struct_declared<'s, 't>(template_name: IdT<'s, 't>) -> bool { panic!("Unimplemented: struct_declared"); }
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
fn lookup_mutability<'s, 't>(template_name: IdT<'s, 't>) -> ITemplataT<'s, 't> { panic!("Unimplemented: lookup_mutability"); }
/*
  def lookupMutability(templateName: IdT[ITemplateNameT]): ITemplataT[MutabilityTemplataType] = {
    // If it has a structTT, then we've at least started to evaluate this citizen
    typeNameToMutability.get(templateName) match {
      case None => vfail("Still figuring out mutability for struct: " + templateName) // See MFDBRE
      case Some(m) => m
    }
  }
*/
fn lookup_sealed<'s, 't>(template_name: IdT<'s, 't>) -> bool { panic!("Unimplemented: lookup_sealed"); }
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
fn interface_declared<'s, 't>(template_name: IdT<'s, 't>) -> bool { panic!("Unimplemented: interface_declared"); }
/*
  def interfaceDeclared(templateName: IdT[ITemplateNameT]): Boolean = {
    // This is the only place besides InterfaceDefinition2 and declareInterface thats allowed to make one of these
    typeDeclaredNames.contains(templateName)
  }
*/
fn lookup_struct<'s, 't>(struct_tt: IdT<'s, 't>) -> StructDefinitionT<'s, 't> { panic!("Unimplemented: lookup_struct"); }
/*
  def lookupStruct(structTT: IdT[IStructNameT]): StructDefinitionT = {
    lookupStructTemplate(TemplataCompiler.getStructTemplate(structTT))
  }
*/
fn lookup_struct_template<'s, 't>(template_name: IdT<'s, 't>) -> StructDefinitionT<'s, 't> { panic!("Unimplemented: lookup_struct_template"); }
/*
  def lookupStructTemplate(templateName: IdT[IStructTemplateNameT]): StructDefinitionT = {
    vassertSome(structTemplateNameToDefinition.get(templateName))
  }
*/
fn lookup_interface<'s, 't>(interface_tt: InterfaceTT<'s, 't>) -> InterfaceDefinitionT<'s, 't> { panic!("Unimplemented: lookup_interface"); }
/*
  def lookupInterface(interfaceTT: InterfaceTT): InterfaceDefinitionT = {
    lookupInterface(TemplataCompiler.getInterfaceTemplate(interfaceTT.id))
  }
*/
fn lookup_interface_by_template_name<'s, 't>(template_name: IdT<'s, 't>) -> InterfaceDefinitionT<'s, 't> { panic!("Unimplemented: lookup_interface"); }
/*
  def lookupInterface(templateName: IdT[IInterfaceTemplateNameT]): InterfaceDefinitionT = {
    vassertSome(interfaceTemplateNameToDefinition.get(templateName))
  }
*/
fn lookup_citizen_by_template_name<'s, 't>(template_name: IdT<'s, 't>) -> CitizenDefinitionT<'s, 't> { panic!("Unimplemented: lookup_citizen"); }
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
fn lookup_citizen_by_tt<'s, 't>(citizen_tt: ICitizenTT<'s, 't>) -> CitizenDefinitionT<'s, 't> { panic!("Unimplemented: lookup_citizen"); }
/*
  def lookupCitizen(citizenTT: ICitizenTT): CitizenDefinitionT = {
    citizenTT match {
      case s @ StructTT(_) => lookupStruct(s.id)
      case s @ InterfaceTT(_) => lookupInterface(s)
    }
  }
*/
fn get_all_structs<'s, 't>() -> Vec<StructDefinitionT<'s, 't>> { panic!("Unimplemented: get_all_structs"); }
/*
  def getAllStructs(): Iterable[StructDefinitionT] = structTemplateNameToDefinition.values
*/
fn get_all_interfaces<'s, 't>() -> Vec<InterfaceDefinitionT<'s, 't>> { panic!("Unimplemented: get_all_interfaces"); }
/*
  def getAllInterfaces(): Iterable[InterfaceDefinitionT] = interfaceTemplateNameToDefinition.values
*/
fn get_all_functions<'s, 't>() -> Vec<FunctionDefinitionT<'s, 't>> { panic!("Unimplemented: get_all_functions"); }
/*
  def getAllFunctions(): Iterable[FunctionDefinitionT] = signatureToFunction.values
*/
fn get_all_impls<'s, 't>() -> Vec<ImplT<'s, 't>> { panic!("Unimplemented: get_all_impls"); }
/*
  def getAllImpls(): Iterable[ImplT] = allImpls.values
//  def getAllStaticSizedArrays(): Iterable[StaticSizedArrayTT] = staticSizedArrayTypes.values
//  def getAllRuntimeSizedArrays(): Iterable[RuntimeSizedArrayTT] = runtimeSizedArrayTypes.values
//  def getKindToDestructorMap(): Map[KindT, PrototypeT] = kindToDestructor.toMap

//  def getStaticSizedArrayType(size: ITemplata[IntegerTemplataType], mutability: ITemplata[MutabilityTemplataType], variability: ITemplata[VariabilityTemplataType], elementType: CoordT): Option[StaticSizedArrayTT] = {
//    staticSizedArrayTypes.get((size, mutability, variability, elementType))
//  }
*/
fn get_env_for_function_signature<'s, 't>(sig: SignatureT<'s, 't>) -> FunctionEnvironmentT<'s, 't> { panic!("Unimplemented: get_env_for_function_signature"); }
/*
  def getEnvForFunctionSignature(sig: SignatureT): FunctionEnvironmentT = {
    vassertSome(envByFunctionSignature.get(sig))
  }
*/
fn get_outer_env_for_type<'s, 't>(range: Vec<RangeS<'s>>, name: IdT<'s, 't>) -> IInDenizenEnvironmentT<'s, 't> { panic!("Unimplemented: get_outer_env_for_type"); }
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
fn get_inner_env_for_type<'s, 't>(name: IdT<'s, 't>) -> IInDenizenEnvironmentT<'s, 't> { panic!("Unimplemented: get_inner_env_for_type"); }
/*
  def getInnerEnvForType(name: IdT[ITemplateNameT]): IInDenizenEnvironmentT = {
    vassertSome(typeNameToInnerEnv.get(name))
  }
*/
fn get_inner_env_for_function<'s, 't>(name: IdT<'s, 't>) -> IInDenizenEnvironmentT<'s, 't> { panic!("Unimplemented: get_inner_env_for_function"); }
/*
  def getInnerEnvForFunction(name: IdT[INameT]): IInDenizenEnvironmentT = {
    vassertSome(functionNameToInnerEnv.get(name))
  }
*/
fn get_outer_env_for_function<'s, 't>(name: IdT<'s, 't>) -> IInDenizenEnvironmentT<'s, 't> { panic!("Unimplemented: get_outer_env_for_function"); }
/*
  def getOuterEnvForFunction(name: IdT[IFunctionTemplateNameT]): IInDenizenEnvironmentT = {
    vassertSome(functionNameToOuterEnv.get(name))
  }
*/
fn get_return_type_for_signature<'s, 't>(sig: SignatureT<'s, 't>) -> Option<CoordT<'s, 't>> { panic!("Unimplemented: get_return_type_for_signature"); }
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
fn get_kind_exports<'s, 't>() -> Vec<KindExportT<'s, 't>> { panic!("Unimplemented: get_kind_exports"); }
/*
  def getKindExports: Vector[KindExportT] = {
    kindExports.toVector
  }
*/
fn get_function_exports<'s, 't>() -> Vec<FunctionExportT<'s, 't>> { panic!("Unimplemented: get_function_exports"); }
/*
  def getFunctionExports: Vector[FunctionExportT] = {
    functionExports.toVector
  }
*/
fn get_kind_externs<'s, 't>() -> Vec<KindExternT<'s, 't>> { panic!("Unimplemented: get_kind_externs"); }
/*
  def getKindExterns: Vector[KindExternT] = {
    kindExterns.toVector
  }
*/
fn get_function_externs<'s, 't>() -> Vec<FunctionExternT<'s, 't>> { panic!("Unimplemented: get_function_externs"); }
/*
  def getFunctionExterns: Vector[FunctionExternT] = {
    functionExterns.toVector
  }
}
*/
