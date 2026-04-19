use std::collections::HashMap as StdHashMap;

use crate::postparsing::names::IImpreciseNameS;
use crate::typing::env::function_environment_t::{
  BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT,
  BuildingFunctionEnvironmentWithClosuredsT, FunctionEnvironmentT, NodeEnvironmentT,
};
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::names::names::{IdT, INameT};
use crate::typing::typing_interner::TypingInterner;

/*
package dev.vale.typing.env

import dev.vale._
import dev.vale.postparsing._
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.macros.citizen._
import dev.vale.typing.macros.{AnonymousInterfaceMacro, FunctorHelper, IFunctionBodyMacro, IOnImplDefinedMacro, IOnInterfaceDefinedMacro, IOnStructDefinedMacro, StructConstructorMacro}
import dev.vale.highertyping._
import dev.vale.postparsing._
import dev.vale.typing._
import TemplatasStore.{entryMatchesFilter, entryToTemplata, getImpreciseName}
import dev.vale.typing.names._
import dev.vale.typing.templata
import dev.vale.typing.templata._
import dev.vale.typing.macros.citizen._
import dev.vale.typing.macros.IOnImplDefinedMacro
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types.{InterfaceTT, KindPlaceholderT, StructTT}

import scala.collection.immutable.{List, Map, Set}
import scala.collection.mutable
*/

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IEnvironmentT<'s, 't>
where 's: 't,
{
    Package(&'t PackageEnvironmentT<'s, 't>),
    Citizen(&'t CitizenEnvironmentT<'s, 't>),
    Function(&'t FunctionEnvironmentT<'s, 't>),
    Node(&'t NodeEnvironmentT<'s, 't>),
    BuildingWithClosureds(&'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>),
    BuildingWithClosuredsAndTemplateArgs(&'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>),
    General(&'t GeneralEnvironmentT<'s, 't>),
    Export(&'t ExportEnvironmentT<'s, 't>),
    Extern(&'t ExternEnvironmentT<'s, 't>),
}
/*
trait IEnvironmentT {
  override def toString: String = {
    "#Environment:" + id
  }
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vfail() // Shouldnt hash these, too big.

  def globalEnv: GlobalEnvironment

  def templatas: TemplatasStore

  private[env] def lookupWithImpreciseNameInner(
    nameS: IImpreciseNameS,
    lookupFilter: Set[ILookupContext],
    getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]]

  private[env] def lookupWithNameInner(
    nameS: INameT,
    lookupFilter: Set[ILookupContext],
    getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]]

  def lookupAllWithImpreciseName(
    nameS: IImpreciseNameS,
    lookupFilter: Set[ILookupContext]):
  Array[ITemplataT[ITemplataType]] = {
    Profiler.frame(() => {
      lookupWithImpreciseNameInner(nameS, lookupFilter, false)
    })
  }

  def lookupAllWithName(
    nameS: INameT,
    lookupFilter: Set[ILookupContext]):
  Iterable[ITemplataT[ITemplataType]] = {
    Profiler.frame(() => {
      lookupWithNameInner(nameS, lookupFilter, false)
    })
  }

  def lookupNearestWithName(
    nameS: INameT,
    lookupFilter: Set[ILookupContext]):
  Option[ITemplataT[ITemplataType]] = {
    Profiler.frame(() => {
      lookupWithNameInner(nameS, lookupFilter, true).toList match {
        case List() => None
        case List(only) => Some(only)
        case multiple => vfail("Too many with name " + nameS + ": " + multiple)
      }
    })
  }

  def lookupNearestWithImpreciseName(
    nameS: IImpreciseNameS,
    lookupFilter: Set[ILookupContext]):
  Option[ITemplataT[ITemplataType]] = {
    Profiler.frame(() => {
      lookupWithImpreciseNameInner(nameS, lookupFilter, true).toList match {
        case List() => None
        case List(only) => Some(only)
        case many => vfail("Too many with name: " + nameS + ":\n" + many.mkString("\n"))
      }
    })
  }

  def id: IdT[INameT]
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IInDenizenEnvironmentT<'s, 't>
where 's: 't,
{
    Citizen(&'t CitizenEnvironmentT<'s, 't>),
    Function(&'t FunctionEnvironmentT<'s, 't>),
    Node(&'t NodeEnvironmentT<'s, 't>),
    BuildingWithClosureds(&'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>),
    BuildingWithClosuredsAndTemplateArgs(&'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>),
    General(&'t GeneralEnvironmentT<'s, 't>),
}
/*
trait IInDenizenEnvironmentT extends IEnvironmentT {
  // This is the denizen that we're currently compiling.
  // If we're compiling a generic, it's the denizen that currently has placeholders defined.
  def rootCompilingDenizenEnv: IInDenizenEnvironmentT

  def denizenId: IdT[INameT]
  def denizenTemplateId: IdT[ITemplateNameT]
}
*/
/*
trait IDenizenEnvironmentBoxT extends IInDenizenEnvironmentT {
  def snapshot: IInDenizenEnvironmentT
  override def toString: String = {
    "#Environment:" + id
  }
  def globalEnv: GlobalEnvironment

  def id: IdT[INameT]
}
*/
pub enum ILookupContext {
  TemplataLookupContext,
  ExpressionLookupContext,
}
/*
sealed trait ILookupContext
case object TemplataLookupContext extends ILookupContext
case object ExpressionLookupContext extends ILookupContext
*/
// Macro-dispatch fields (functorHelper, *Macro, nameToStructDefinedMacro, etc.)
// from the Scala case class below are omitted here; they moved to `Compiler` as
// part of the god-struct refactor. See docs/migration/handoff-god-struct-progress.md.
#[derive(Debug)]
pub struct GlobalEnvironmentT<'s, 't>
where 's: 't,
{
  pub name_to_top_level_environment:
    &'t [(&'t IdT<'s, 't>, TemplatasStoreT<'s, 't>)],
  pub builtins: TemplatasStoreT<'s, 't>,
}
/*
case class GlobalEnvironment(
  functorHelper: FunctorHelper,
  structConstructorMacro: StructConstructorMacro,
  structDropMacro: StructDropMacro,
//  structFreeMacro: StructFreeMacro,
  interfaceDropMacro: InterfaceDropMacro,
//  interfaceFreeMacro: InterfaceFreeMacro,
  anonymousInterfaceMacro: AnonymousInterfaceMacro,
  nameToStructDefinedMacro: Map[StrI, IOnStructDefinedMacro],
  nameToInterfaceDefinedMacro: Map[StrI, IOnInterfaceDefinedMacro],
  nameToImplDefinedMacro: Map[StrI, IOnImplDefinedMacro],
  nameToFunctionBodyMacro: Map[StrI, IFunctionBodyMacro],
  // We *dont* search through these in lookupWithName etc.
  // This doesn't just contain the user's things, it can contain generated things
  // like struct constructors, interface constructors, etc.
  // This isn't just packages, structs can have entries here too, because their
  // environments might have things, like a struct's methods might be here.
  // Any particular IEnvironment subclass has a subset of these.
  nameToTopLevelEnvironment: Map[IdT[PackageTopLevelNameT], TemplatasStore],
  // Primitives and other builtins
  builtins: TemplatasStore
)
*/
fn entry_matches_filter() {
  panic!("Unimplemented: entry_matches_filter");
}
/*
object TemplatasStore {
  def entryMatchesFilter(entry: IEnvEntry, contexts: Set[ILookupContext]): Boolean = {
    entry match {
      case FunctionEnvEntry(_) => contexts.contains(ExpressionLookupContext)
      case ImplEnvEntry(_) => contexts.contains(TemplataLookupContext)
      case StructEnvEntry(_) => contexts.contains(TemplataLookupContext)
      case InterfaceEnvEntry(_) => contexts.contains(TemplataLookupContext)
      case TemplataEnvEntry(templata) => {
        templata match {
          case PlaceholderTemplataT(_, _) => contexts.contains(TemplataLookupContext)
          case IsaTemplataT(_, _, _, _) => contexts.contains(TemplataLookupContext)
//          case PrototypeTemplata(_, _, _) => true
          case CoordTemplataT(_) => contexts.contains(TemplataLookupContext)
          case CoordListTemplataT(_) => contexts.contains(TemplataLookupContext)
          case PrototypeTemplataT(_) => true
          case KindTemplataT(_) => contexts.contains(TemplataLookupContext)
          case StructDefinitionTemplataT(_, _) => contexts.contains(TemplataLookupContext)
          case InterfaceDefinitionTemplataT(_, _) => contexts.contains(TemplataLookupContext)
          case RuntimeSizedArrayTemplateTemplataT() => contexts.contains(TemplataLookupContext)
          case StaticSizedArrayTemplateTemplataT() => contexts.contains(TemplataLookupContext)
          case BooleanTemplataT(_) => true
          case FunctionTemplataT(_, _) => contexts.contains(ExpressionLookupContext)
          case ImplDefinitionTemplataT(_, _) => contexts.contains(ExpressionLookupContext)
          case IntegerTemplataT(_) => true
          case StringTemplataT(_) => true
          case LocationTemplataT(_) => contexts.contains(TemplataLookupContext)
          case MutabilityTemplataT(_) => contexts.contains(TemplataLookupContext)
          case OwnershipTemplataT(_) => contexts.contains(TemplataLookupContext)
          case VariabilityTemplataT(_) => contexts.contains(TemplataLookupContext)
//          case ExternImplTemplata(_, _) => contexts.contains(TemplataLookupContext)
          case ExternFunctionTemplataT(_) => contexts.contains(ExpressionLookupContext)
        }
      }
    }
  }
*/
fn entry_to_templata() {
  panic!("Unimplemented: entry_to_templata");
}
/*
  def entryToTemplata(definingEnv: IEnvironmentT, entry: IEnvEntry): ITemplataT[ITemplataType] = {
    //    vassert(env.fullName != FullName2(PackageCoordinate.BUILTIN, Vector.empty, PackageTopLevelName2()))
    entry match {
      case FunctionEnvEntry(func) => templata.FunctionTemplataT(definingEnv, func)
      case StructEnvEntry(struct) => templata.StructDefinitionTemplataT(definingEnv, struct)
      case InterfaceEnvEntry(interface) => templata.InterfaceDefinitionTemplataT(definingEnv, interface)
      case ImplEnvEntry(impl) => templata.ImplDefinitionTemplataT(definingEnv, impl)
      case TemplataEnvEntry(templata) => templata
    }
  }
*/
fn get_imprecise_name() {
  panic!("Unimplemented: get_imprecise_name");
}
/*
  def getImpreciseName(interner: Interner, name2: INameT): Option[IImpreciseNameS] = {
    name2 match {
      case StructTemplateNameT(humanName) => Some(interner.intern(CodeNameS(humanName)))
      case InterfaceTemplateNameT(humanName) => Some(interner.intern(CodeNameS(humanName)))
      case PrimitiveNameT(humanName) => Some(interner.intern(CodeNameS(humanName)))
      case CitizenNameT(templateName, _) => getImpreciseName(interner, templateName)
      case FunctionTemplateNameT(humanName, _) => Some(interner.intern(CodeNameS(humanName)))
      case FunctionNameT(FunctionTemplateNameT(humanName, _), _, _) => Some(interner.intern(CodeNameS(humanName)))
      case RuneNameT(r) => Some(interner.intern(RuneNameS(r)))
      case LambdaCitizenNameT(template) => getImpreciseName(interner, template)
      case LambdaCitizenTemplateNameT(loc) => Some(interner.intern(LambdaStructImpreciseNameS(interner.intern(LambdaImpreciseNameS()))))
      case ClosureParamNameT(codeLoc) => Some(interner.intern(ClosureParamImpreciseNameS()))
      case SelfNameT() => Some(interner.intern(SelfNameS()))
      case ArbitraryNameT() => Some(interner.intern(ArbitraryNameS()))
      case AnonymousSubstructImplNameT(_, _, _) => None
      case AnonymousSubstructConstructorTemplateNameT(StructTemplateNameT(humanName)) => {
        Some(interner.intern(CodeNameS(humanName)))
      }
      case AnonymousSubstructTemplateNameT(ctn) => {
        getImpreciseName(interner, ctn).map(x => interner.intern(AnonymousSubstructTemplateImpreciseNameS(x)))
      }
      case AnonymousSubstructConstructorTemplateNameT(AnonymousSubstructTemplateNameT(InterfaceTemplateNameT(humanName))) => {
        Some(interner.intern(CodeNameS(humanName)))
      }
      case AnonymousSubstructNameT(interfaceName, _) => getImpreciseName(interner, interfaceName)
      case ImplTemplateNameT(_) => {
        // We shouldn't get here, caller shouldn't pass these in. Should instead get the impl
        // imprecise name from the ImplA or somewhere else.
        vwat()
      }
//      case LambdaTemplateNameT(codeLocation) => Some(interner.intern(LambdaImpreciseNameS()))
      case KindPlaceholderNameT(KindPlaceholderTemplateNameT(index, rune)) => Some(interner.intern(PlaceholderImpreciseNameS(index)))
      case ReachablePrototypeNameT(num) => None
//      case AbstractVirtualFreeTemplateNameT(codeLoc) => Some(interner.intern(VirtualFreeImpreciseNameS()))
      case ForwarderFunctionTemplateNameT(inner, index) => getImpreciseName(interner, inner)
      case ForwarderFunctionNameT(_, inner) => getImpreciseName(interner, inner)
      case FunctionBoundNameT(inner, _, _) => getImpreciseName(interner, inner)
      case FunctionBoundTemplateNameT(humanName) => Some(interner.intern(CodeNameS(humanName)))
      case FunctionBoundNameT(inner, _, _) => getImpreciseName(interner, inner)
      case FunctionBoundTemplateNameT(humanName) => Some(interner.intern(CodeNameS(humanName)))
      case PredictedFunctionNameT(inner, _, _) => getImpreciseName(interner, inner)
      case PredictedFunctionTemplateNameT(humanName) => Some(interner.intern(CodeNameS(humanName)))
      case LambdaCallFunctionNameT(_, _, _) => {
        None // I don't think anyone will ever need to look up a specific lambda incarnation by name
      }
      case FunctionBoundTemplateNameT(humanName) => Some(interner.intern(CodeNameS(humanName)))
      case FunctionBoundNameT(inner, _, _) => getImpreciseName(interner, inner)
//      case AnonymousSubstructImplTemplateNameT(inner) => getImpreciseName(interner, inner).map(ImplImpreciseNameS)
//      case OverrideVirtualFreeTemplateNameT(codeLoc) => Some(interner.intern(VirtualFreeImpreciseNameS()))
//      case AbstractVirtualFreeNameT(_, _) => Some(interner.intern(VirtualFreeImpreciseNameS()))
//      case OverrideVirtualFreeNameT(_, _) => Some(interner.intern(VirtualFreeImpreciseNameS()))
//      case OverrideVirtualDropFunctionTemplateNameT(_) => Some(interner.intern(CodeNameS(Scout.VIRTUAL_DROP_FUNCTION_NAME)))
//      case AbstractVirtualDropFunctionTemplateNameT(_) => Some(interner.intern(CodeNameS(Scout.VIRTUAL_DROP_FUNCTION_NAME)))
//      case OverrideVirtualDropFunctionNameT(_, _, _) => Some(interner.intern(CodeNameS(Scout.VIRTUAL_DROP_FUNCTION_NAME)))
//      case AbstractVirtualDropFunctionNameT(_, _, _) => Some(interner.intern(CodeNameS(Scout.VIRTUAL_DROP_FUNCTION_NAME)))
      case other => vimpl(other.toString)
    }
  }
*/
fn code_locations_match() {
  panic!("Unimplemented: code_locations_match");
}
/*
  def codeLocationsMatch(codeLocationA: CodeLocationS, codeLocation2: CodeLocationS): Boolean = {
    val CodeLocationS(lineS, charS) = codeLocationA
    val CodeLocationS(line2, char2) = codeLocation2
    lineS == line2 && charS == char2
  }
}
*/
#[derive(Copy, Clone, Debug)]
pub struct TemplatasStoreT<'s, 't>
where 's: 't,
{
  pub templatas_store_name: &'t IdT<'s, 't>,
  pub name_to_entry: &'t [(INameT<'s, 't>, IEnvEntryT<'s, 't>)],
  pub imprecise_to_entries:
    &'t [(&'s IImpreciseNameS<'s>, &'t [IEnvEntryT<'s, 't>])],
}

// Scala `override def equals/hashCode = vcurious()` — mirror with panic.
impl<'s, 't> PartialEq for TemplatasStoreT<'s, 't> where 's: 't {
  fn eq(&self, _other: &Self) -> bool { panic!("vcurious: TemplatasStoreT.eq") }
}
impl<'s, 't> Eq for TemplatasStoreT<'s, 't> where 's: 't {}
impl<'s, 't> std::hash::Hash for TemplatasStoreT<'s, 't> where 's: 't {
  fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
    panic!("vcurious: TemplatasStoreT.hash")
  }
}

// (no scala counterpart — builder for TemplatasStoreT. Heap Vec/HashMap during
// construction, frozen to arena slices at build_in.)
pub struct TemplatasStoreBuilder<'s, 't>
where 's: 't,
{
  pub templatas_store_name: &'t IdT<'s, 't>,
  pub name_to_entry: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)>,
  pub imprecise_to_entries:
    StdHashMap<&'s IImpreciseNameS<'s>, Vec<IEnvEntryT<'s, 't>>>,
}

impl<'s, 't> TemplatasStoreBuilder<'s, 't>
where 's: 't,
{
  pub fn new(templatas_store_name: &'t IdT<'s, 't>) -> Self {
    TemplatasStoreBuilder {
      templatas_store_name,
      name_to_entry: Vec::new(),
      imprecise_to_entries: StdHashMap::new(),
    }
  }

  pub fn build_in(
    self,
    interner: &TypingInterner<'s, 't>,
  ) -> TemplatasStoreT<'s, 't> {
    let name_to_entry = interner.alloc_slice_from_vec(self.name_to_entry);
    let mut pairs: Vec<(&'s IImpreciseNameS<'s>, &'t [IEnvEntryT<'s, 't>])> =
      Vec::with_capacity(self.imprecise_to_entries.len());
    for (k, v) in self.imprecise_to_entries.into_iter() {
      let entries = interner.alloc_slice_from_vec(v);
      pairs.push((k, entries));
    }
    let imprecise_to_entries = interner.alloc_slice_from_vec(pairs);
    TemplatasStoreT {
      templatas_store_name: self.templatas_store_name,
      name_to_entry,
      imprecise_to_entries,
    }
  }
}
/*
// See DBTSAE for difference between TemplatasStore and Environment.
case class TemplatasStore(
  templatasStoreName: IdT[INameT],
  // This is the source of truth. Anything in the environment is in here.
  entriesByNameT: Map[INameT, IEnvEntry],
  // This is just an index for quick looking up of things by their imprecise name.
  // Not everything in the above entriesByNameT will have something in here.
  // Vector because multiple things can share an INameS; function overloads.
  entriesByImpreciseNameS: Map[IImpreciseNameS, Vector[IEnvEntry]]
) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

  entriesByNameT.values.foreach({
    case FunctionEnvEntry(function) => vassert(function.name.packageCoordinate == templatasStoreName.packageCoord)
    case StructEnvEntry(struct) => vassert(struct.range.file.packageCoordinate == templatasStoreName.packageCoord)
    case InterfaceEnvEntry(interface) => vassert(interface.name.range.file.packageCoordinate == templatasStoreName.packageCoord)
    case _ =>
  })

  //  // The above map, indexed by human name. If it has no human name, it won't be in here.
  //  private var entriesByHumanName = Map[String, Vector[IEnvEntry]]()

  def addEntries(interner: Interner, newEntriesList: Vector[(INameT, IEnvEntry)]): TemplatasStore = {
    val newEntries = newEntriesList.toMap
    vassert(newEntries.size == newEntriesList.size)

    val oldEntries = entriesByNameT

    val combinedEntries = oldEntries ++ newEntries
    val intersection = oldEntries.keySet.intersect(newEntries.keySet)

    oldEntries.keySet.intersect(newEntries.keySet).foreach(key => {
      vassert(oldEntries(key) == newEntries(key))
      // We can get here  if we use RuneEnvLookup rules,
      // those "figure out" the rune, though it already existed.
      // They end up reintroducing those rules to the env, even though
      // they were already there.
    })

    val newEntriesByNameS =
      newEntries
        .toVector
        .flatMap({
          case (key, value @ TemplataEnvEntry(PrototypeTemplataT(prototype))) => {
            // This is so if we have:
            //    where func moo(T)T
            // then that prototype will be accessible via not only ImplicitRune(1.4.6.1)
            // but also CodeNameS("moo").
            getImpreciseName(interner, key).toList.map(_ -> value) ++
              getImpreciseName(interner, prototype.id.localName).map(_ -> value) ++
              List(interner.intern(PrototypeNameS()) -> value)
          }
          case (key, entry @ ImplEnvEntry(implA)) => {
            List(
              interner.intern(ImplImpreciseNameS(implA.subCitizenImpreciseName, implA.superInterfaceImpreciseName)) -> entry,
              interner.intern(ImplSubCitizenImpreciseNameS(implA.subCitizenImpreciseName)) -> entry,
              interner.intern(ImplSuperInterfaceImpreciseNameS(implA.superInterfaceImpreciseName)) -> entry)
          }
          case (key, entry @ TemplataEnvEntry(IsaTemplataT(_, _, subKind, superKind))) => {
            val subImpreciseName =
              subKind match {
                case StructTT(id) => vassertSome(getImpreciseName(interner, id.localName))
                case InterfaceTT(id) => vassertSome(getImpreciseName(interner, id.localName))
                case KindPlaceholderT(id) => vassertSome(getImpreciseName(interner, id.localName))
                case _ => vwat()
              }
            val superImpreciseName =
              superKind match {
                case InterfaceTT(id) => vassertSome(getImpreciseName(interner, id.localName))
                case KindPlaceholderT(id) => vassertSome(getImpreciseName(interner, id.localName))
                case _ => vwat()
              }
            getImpreciseName(interner, key).toList.map(_ -> entry) ++
            List(
              interner.intern(ImplImpreciseNameS(subImpreciseName, superImpreciseName)) -> entry,
              interner.intern(ImplSubCitizenImpreciseNameS(subImpreciseName)) -> entry,
              interner.intern(ImplSuperInterfaceImpreciseNameS(superImpreciseName)) -> entry)
          }
          case (key, value) => {
            getImpreciseName(interner, key).toList.map(_ -> value)
          }
        })
        .groupBy(_._1)
        .mapValues(_.map(_._2))
    val combinedEntriesByNameS =
      entriesByImpreciseNameS ++
        newEntriesByNameS ++
        entriesByImpreciseNameS.keySet.intersect(newEntriesByNameS.keySet)
          .map(key => (key -> (entriesByImpreciseNameS(key) ++ newEntriesByNameS(key))))
          .toMap

    TemplatasStore(templatasStoreName, combinedEntries, combinedEntriesByNameS)
  }

  def addEntry(interner: Interner, name: INameT, entry: IEnvEntry): TemplatasStore = {
    addEntries(interner, Vector(name -> entry))
  }

  private[env] def lookupWithNameInner(
    definingEnv: IEnvironmentT,

    name: INameT,
    lookupFilter: Set[ILookupContext]):
  Option[ITemplataT[ITemplataType]] = {
    entriesByNameT.get(name)
      .filter(entryMatchesFilter(_, lookupFilter))
      .map(entryToTemplata(definingEnv, _))
  }

  private[env] def lookupWithImpreciseNameInner(
    definingEnv: IEnvironmentT,

    name: IImpreciseNameS,
    lookupFilter: Set[ILookupContext]):
  Array[ITemplataT[ITemplataType]] = {
    val a1 = entriesByImpreciseNameS.getOrElse(name, Vector())
    val a2 = a1.filter(entryMatchesFilter(_, lookupFilter))
    val a3 = a2.map(entryToTemplata(definingEnv, _))
    a3.toArray
  }
}
*/
fn make_top_level_environment() {
  panic!("Unimplemented: make_top_level_environment");
}
/*
object PackageEnvironmentT {
  // THIS IS TEMPORARY, it pulls in all global namespaces!
  // See https://github.com/ValeLang/Vale/issues/356
  def makeTopLevelEnvironment(globalEnv: GlobalEnvironment, namespaceName: IdT[INameT]): PackageEnvironmentT[INameT] = {
    PackageEnvironmentT(
      globalEnv,
      namespaceName,
      globalEnv.nameToTopLevelEnvironment.values.toVector)
  }
}
*/
#[derive(Debug)]
pub struct PackageEnvironmentT<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub global_namespaces: &'t [TemplatasStoreT<'s, 't>],
}

// Id-based Hash/PartialEq per Gotcha 13.
impl<'s, 't> PartialEq for PackageEnvironmentT<'s, 't> where 's: 't {
  fn eq(&self, other: &Self) -> bool { self.id == other.id }
}
impl<'s, 't> Eq for PackageEnvironmentT<'s, 't> where 's: 't {}
impl<'s, 't> std::hash::Hash for PackageEnvironmentT<'s, 't> where 's: 't {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.id.hash(state); }
}
/*
case class PackageEnvironmentT[+T <: INameT](
  globalEnv: GlobalEnvironment,
  id: IdT[T],

  // These are ones that the user imports (or the ancestors that we implicitly import)
  globalNamespaces: Vector[TemplatasStore]
) extends IEnvironmentT {
  val hash = runtime.ScalaRunTime._hashCode(id);
override def hashCode(): Int = hash;

  override def templatas: TemplatasStore = {
    vimpl()
  }

//  override def rootCompilingDenizenEnv: IInDenizenEnvironment = vwat()

  override def equals(obj: Any): Boolean = {
    if (!obj.isInstanceOf[PackageEnvironmentT[T]]) {
      return false
    }
    return id.equals(obj.asInstanceOf[PackageEnvironmentT[T]].id)
  }

  private[env] override def lookupWithNameInner(
    name: INameT,
    lookupFilter: Set[ILookupContext],
    getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    globalEnv.builtins.lookupWithNameInner(this, name, lookupFilter).toArray ++
    globalNamespaces
        .toArray
        .flatMap(ns => {
      val env = PackageEnvironmentT(globalEnv, ns.templatasStoreName, globalNamespaces)
      ns.lookupWithNameInner(env, name, lookupFilter)
    })
  }

  private[env] override def lookupWithImpreciseNameInner(
    name: IImpreciseNameS,
    lookupFilter: Set[ILookupContext],
    getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    val result = mutable.ArrayBuffer[ITemplataT[ITemplataType]]();
    U.foreachArr[ITemplataT[ITemplataType]](
      globalEnv.builtins.lookupWithImpreciseNameInner(this, name, lookupFilter),
      (a) => result += a)
    U.foreach[TemplatasStore](globalNamespaces, globalNamespace => {
      U.foreachIterable[ITemplataT[ITemplataType]](
        globalNamespace.lookupWithImpreciseNameInner(
          PackageEnvironmentT(globalEnv, globalNamespace.templatasStoreName, globalNamespaces),
          name, lookupFilter),
        thing => {
          result += thing
    })
    })
    result.toArray
  }
}
*/
#[derive(Debug)]
pub struct CitizenEnvironmentT<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas: TemplatasStoreT<'s, 't>,
}

impl<'s, 't> PartialEq for CitizenEnvironmentT<'s, 't> where 's: 't {
  fn eq(&self, other: &Self) -> bool { self.id == other.id }
}
impl<'s, 't> Eq for CitizenEnvironmentT<'s, 't> where 's: 't {}
impl<'s, 't> std::hash::Hash for CitizenEnvironmentT<'s, 't> where 's: 't {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.id.hash(state); }
}
/*
case class CitizenEnvironmentT[+T <: INameT, +Y <: ITemplateNameT](
  globalEnv: GlobalEnvironment,
  parentEnv: IEnvironmentT,
  templateId: IdT[Y],
  id: IdT[T],
  templatas: TemplatasStore
) extends IInDenizenEnvironmentT {
  vassert(templatas.templatasStoreName == id)

  override def denizenId: IdT[INameT] = templateId
  override def denizenTemplateId: IdT[ITemplateNameT] = templateId

  val hash = runtime.ScalaRunTime._hashCode(id);
override def hashCode(): Int = hash;
  override def equals(obj: Any): Boolean = {
    if (!obj.isInstanceOf[IInDenizenEnvironmentT]) {
      return false
    }
    return id.equals(obj.asInstanceOf[IInDenizenEnvironmentT].id)
  }

  override def rootCompilingDenizenEnv: IInDenizenEnvironmentT = {
    (id.localName, parentEnv.id.localName) match {
      case (_ : IInstantiationNameT, _ : ITemplateNameT) => this
      case (_, PackageTopLevelNameT()) => this
      case _ => {
        parentEnv match {
          case parentInDenizenEnv : IInDenizenEnvironmentT => {
            val result = parentInDenizenEnv.rootCompilingDenizenEnv
            result.id.localName match {
              case _ : IInstantiationNameT =>
              case other => vwat(other)
            }
            result
          }
          case _ => vwat()
        }
      }
    }
  }

  private[env] override def lookupWithNameInner(

    name: INameT,
    lookupFilter: Set[ILookupContext],
    getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    val result = templatas.lookupWithNameInner(this, name, lookupFilter).toArray
    if (result.nonEmpty && getOnlyNearest) {
      result
    } else {
      result ++ parentEnv.lookupWithNameInner(name, lookupFilter, getOnlyNearest)
    }
  }

  private[env] override def lookupWithImpreciseNameInner(

    name: IImpreciseNameS,
    lookupFilter: Set[ILookupContext],
    getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    val result = templatas.lookupWithImpreciseNameInner(this, name, lookupFilter)
    if (result.nonEmpty && getOnlyNearest) {
      result
    } else {
      result ++ parentEnv.lookupWithImpreciseNameInner(name, lookupFilter, getOnlyNearest)
    }
  }
}
*/
fn child_of() {
  panic!("Unimplemented: child_of");
}
/*
object GeneralEnvironmentT {
  def childOf[Y <: INameT](
    interner: Interner,
    parentEnv: IInDenizenEnvironmentT,
    newTemplateId: IdT[ITemplateNameT],
    newId: IdT[Y],
    newEntriesList: Vector[(INameT, IEnvEntry)] = Vector()):
  GeneralEnvironmentT[Y] = {
    GeneralEnvironmentT(
      parentEnv.globalEnv,
      parentEnv,
      newTemplateId,
      newId,
      new TemplatasStore(newId, Map(), Map())
        .addEntries(interner, newEntriesList))
  }
}
*/
#[derive(Debug)]
pub struct ExportEnvironmentT<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: &'t PackageEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas: TemplatasStoreT<'s, 't>,
}

impl<'s, 't> PartialEq for ExportEnvironmentT<'s, 't> where 's: 't {
  fn eq(&self, other: &Self) -> bool { self.id == other.id }
}
impl<'s, 't> Eq for ExportEnvironmentT<'s, 't> where 's: 't {}
impl<'s, 't> std::hash::Hash for ExportEnvironmentT<'s, 't> where 's: 't {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.id.hash(state); }
}
/*
case class ExportEnvironmentT(
    globalEnv: GlobalEnvironment,
    parentEnv: PackageEnvironmentT[INameT],
    templateId: IdT[ITemplateNameT],
    id: IdT[INameT],
    //  defaultRegion: ITemplata[RegionTemplataType],
    templatas: TemplatasStore
) extends IInDenizenEnvironmentT {
  override def rootCompilingDenizenEnv: IInDenizenEnvironmentT = this
  override def denizenId: IdT[INameT] = id
  override def denizenTemplateId: IdT[ITemplateNameT] = templateId

  override def lookupWithNameInner(
      name: INameT,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    EnvironmentHelper.lookupWithNameInner(
      this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
  }

  override def lookupWithImpreciseNameInner(
      name: IImpreciseNameS,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    EnvironmentHelper.lookupWithImpreciseNameInner(
      this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
  }
}
*/
#[derive(Debug)]
pub struct ExternEnvironmentT<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: &'t PackageEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas: TemplatasStoreT<'s, 't>,
}

impl<'s, 't> PartialEq for ExternEnvironmentT<'s, 't> where 's: 't {
  fn eq(&self, other: &Self) -> bool { self.id == other.id }
}
impl<'s, 't> Eq for ExternEnvironmentT<'s, 't> where 's: 't {}
impl<'s, 't> std::hash::Hash for ExternEnvironmentT<'s, 't> where 's: 't {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.id.hash(state); }
}
/*
case class ExternEnvironmentT(
    globalEnv: GlobalEnvironment,
    parentEnv: PackageEnvironmentT[INameT],
    templateId: IdT[ITemplateNameT],
    id: IdT[INameT],
    //  defaultRegion: ITemplata[RegionTemplataType],
    templatas: TemplatasStore
) extends IInDenizenEnvironmentT {
  override def rootCompilingDenizenEnv: IInDenizenEnvironmentT = this
  override def denizenId: IdT[INameT] = id
  override def denizenTemplateId: IdT[ITemplateNameT] = templateId

  override def lookupWithNameInner(
      name: INameT,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    EnvironmentHelper.lookupWithNameInner(
      this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
  }

  override def lookupWithImpreciseNameInner(
      name: IImpreciseNameS,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    EnvironmentHelper.lookupWithImpreciseNameInner(
      this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
  }
}
*/
#[derive(Debug)]
pub struct GeneralEnvironmentT<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IInDenizenEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas: TemplatasStoreT<'s, 't>,
}

// Scala `override def equals/hashCode = vcurious()` — mirror with panic.
impl<'s, 't> PartialEq for GeneralEnvironmentT<'s, 't> where 's: 't {
  fn eq(&self, _other: &Self) -> bool { panic!("vcurious: GeneralEnvironmentT.eq") }
}
impl<'s, 't> Eq for GeneralEnvironmentT<'s, 't> where 's: 't {}
impl<'s, 't> std::hash::Hash for GeneralEnvironmentT<'s, 't> where 's: 't {
  fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
    panic!("vcurious: GeneralEnvironmentT.hash")
  }
}
/*
case class GeneralEnvironmentT[+T <: INameT](
  globalEnv: GlobalEnvironment,
  parentEnv: IInDenizenEnvironmentT,
  templateId: IdT[ITemplateNameT],
  id: IdT[T],
  templatas: TemplatasStore
) extends IInDenizenEnvironmentT {
  override def denizenId: IdT[INameT] = id
  override def denizenTemplateId: IdT[ITemplateNameT] = templateId

  override def equals(obj: Any): Boolean = vcurious();

  override def hashCode(): Int = vcurious()

  override def rootCompilingDenizenEnv: IInDenizenEnvironmentT = {
//    parentEnv match {
//      case PackageEnvironment(_, _, _) => this
//      case _ => parentEnv.rootCompilingDenizenEnv
//    }
    parentEnv.rootCompilingDenizenEnv
  }

  override def lookupWithNameInner(
    name: INameT,
    lookupFilter: Set[ILookupContext],
    getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    EnvironmentHelper.lookupWithNameInner(
      this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
  }

  override def lookupWithImpreciseNameInner(
    name: IImpreciseNameS,
    lookupFilter: Set[ILookupContext],
    getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    EnvironmentHelper.lookupWithImpreciseNameInner(
      this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
  }
}
*/

// Concrete → IEnvironmentT
impl<'s, 't> From<&'t PackageEnvironmentT<'s, 't>> for IEnvironmentT<'s, 't> {
  fn from(e: &'t PackageEnvironmentT<'s, 't>) -> Self { IEnvironmentT::Package(e) }
}
impl<'s, 't> From<&'t CitizenEnvironmentT<'s, 't>> for IEnvironmentT<'s, 't> {
  fn from(e: &'t CitizenEnvironmentT<'s, 't>) -> Self { IEnvironmentT::Citizen(e) }
}
impl<'s, 't> From<&'t FunctionEnvironmentT<'s, 't>> for IEnvironmentT<'s, 't> {
  fn from(e: &'t FunctionEnvironmentT<'s, 't>) -> Self { IEnvironmentT::Function(e) }
}
impl<'s, 't> From<&'t NodeEnvironmentT<'s, 't>> for IEnvironmentT<'s, 't> {
  fn from(e: &'t NodeEnvironmentT<'s, 't>) -> Self { IEnvironmentT::Node(e) }
}
impl<'s, 't> From<&'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>> for IEnvironmentT<'s, 't> {
  fn from(e: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>) -> Self {
    IEnvironmentT::BuildingWithClosureds(e)
  }
}
impl<'s, 't> From<&'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>> for IEnvironmentT<'s, 't> {
  fn from(e: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>) -> Self {
    IEnvironmentT::BuildingWithClosuredsAndTemplateArgs(e)
  }
}
impl<'s, 't> From<&'t GeneralEnvironmentT<'s, 't>> for IEnvironmentT<'s, 't> {
  fn from(e: &'t GeneralEnvironmentT<'s, 't>) -> Self { IEnvironmentT::General(e) }
}
impl<'s, 't> From<&'t ExportEnvironmentT<'s, 't>> for IEnvironmentT<'s, 't> {
  fn from(e: &'t ExportEnvironmentT<'s, 't>) -> Self { IEnvironmentT::Export(e) }
}
impl<'s, 't> From<&'t ExternEnvironmentT<'s, 't>> for IEnvironmentT<'s, 't> {
  fn from(e: &'t ExternEnvironmentT<'s, 't>) -> Self { IEnvironmentT::Extern(e) }
}

// Concrete → IInDenizenEnvironmentT (6 variants; no Package/Export/Extern)
impl<'s, 't> From<&'t CitizenEnvironmentT<'s, 't>> for IInDenizenEnvironmentT<'s, 't> {
  fn from(e: &'t CitizenEnvironmentT<'s, 't>) -> Self { IInDenizenEnvironmentT::Citizen(e) }
}
impl<'s, 't> From<&'t FunctionEnvironmentT<'s, 't>> for IInDenizenEnvironmentT<'s, 't> {
  fn from(e: &'t FunctionEnvironmentT<'s, 't>) -> Self { IInDenizenEnvironmentT::Function(e) }
}
impl<'s, 't> From<&'t NodeEnvironmentT<'s, 't>> for IInDenizenEnvironmentT<'s, 't> {
  fn from(e: &'t NodeEnvironmentT<'s, 't>) -> Self { IInDenizenEnvironmentT::Node(e) }
}
impl<'s, 't> From<&'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>> for IInDenizenEnvironmentT<'s, 't> {
  fn from(e: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>) -> Self {
    IInDenizenEnvironmentT::BuildingWithClosureds(e)
  }
}
impl<'s, 't> From<&'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>> for IInDenizenEnvironmentT<'s, 't> {
  fn from(e: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>) -> Self {
    IInDenizenEnvironmentT::BuildingWithClosuredsAndTemplateArgs(e)
  }
}
impl<'s, 't> From<&'t GeneralEnvironmentT<'s, 't>> for IInDenizenEnvironmentT<'s, 't> {
  fn from(e: &'t GeneralEnvironmentT<'s, 't>) -> Self { IInDenizenEnvironmentT::General(e) }
}

// Widening: IInDenizenEnvironmentT → IEnvironmentT (always succeeds)
impl<'s, 't> From<IInDenizenEnvironmentT<'s, 't>> for IEnvironmentT<'s, 't> {
  fn from(e: IInDenizenEnvironmentT<'s, 't>) -> Self {
    match e {
      IInDenizenEnvironmentT::Citizen(c) => IEnvironmentT::Citizen(c),
      IInDenizenEnvironmentT::Function(f) => IEnvironmentT::Function(f),
      IInDenizenEnvironmentT::Node(n) => IEnvironmentT::Node(n),
      IInDenizenEnvironmentT::BuildingWithClosureds(b) => IEnvironmentT::BuildingWithClosureds(b),
      IInDenizenEnvironmentT::BuildingWithClosuredsAndTemplateArgs(b) =>
        IEnvironmentT::BuildingWithClosuredsAndTemplateArgs(b),
      IInDenizenEnvironmentT::General(g) => IEnvironmentT::General(g),
    }
  }
}

// Narrowing: IEnvironmentT → IInDenizenEnvironmentT (errors on Package/Export/Extern)
impl<'s, 't> TryFrom<IEnvironmentT<'s, 't>> for IInDenizenEnvironmentT<'s, 't> {
  type Error = IEnvironmentT<'s, 't>;
  fn try_from(e: IEnvironmentT<'s, 't>) -> Result<Self, Self::Error> {
    match e {
      IEnvironmentT::Citizen(c) => Ok(IInDenizenEnvironmentT::Citizen(c)),
      IEnvironmentT::Function(f) => Ok(IInDenizenEnvironmentT::Function(f)),
      IEnvironmentT::Node(n) => Ok(IInDenizenEnvironmentT::Node(n)),
      IEnvironmentT::BuildingWithClosureds(b) => Ok(IInDenizenEnvironmentT::BuildingWithClosureds(b)),
      IEnvironmentT::BuildingWithClosuredsAndTemplateArgs(b) =>
        Ok(IInDenizenEnvironmentT::BuildingWithClosuredsAndTemplateArgs(b)),
      IEnvironmentT::General(g) => Ok(IInDenizenEnvironmentT::General(g)),
      other @ (IEnvironmentT::Package(_)
        | IEnvironmentT::Export(_)
        | IEnvironmentT::Extern(_)) => Err(other),
    }
  }
}

// ============================================================================
// Builders — one per env kind. Each owns heap Vec/HashMap for incrementally
// built fields (templatas + slices), then freezes via build_in(interner) into
// an arena-allocated &'t FooEnvironmentT.
// ============================================================================

pub struct PackageEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub global_namespaces: Vec<TemplatasStoreT<'s, 't>>,
}

impl<'s, 't> PackageEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub fn build_in(
    self,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t PackageEnvironmentT<'s, 't> {
    let global_namespaces = interner.alloc_slice_from_vec(self.global_namespaces);
    interner.alloc(PackageEnvironmentT {
      global_env: self.global_env,
      id: self.id,
      global_namespaces,
    })
  }
}

pub struct CitizenEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
}

impl<'s, 't> CitizenEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub fn build_in(
    self,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t CitizenEnvironmentT<'s, 't> {
    let templatas = self.templatas_builder.build_in(interner);
    interner.alloc(CitizenEnvironmentT {
      global_env: self.global_env,
      parent_env: self.parent_env,
      template_id: self.template_id,
      id: self.id,
      templatas,
    })
  }
}

pub struct ExportEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: &'t PackageEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
}

impl<'s, 't> ExportEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub fn build_in(
    self,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t ExportEnvironmentT<'s, 't> {
    let templatas = self.templatas_builder.build_in(interner);
    interner.alloc(ExportEnvironmentT {
      global_env: self.global_env,
      parent_env: self.parent_env,
      template_id: self.template_id,
      id: self.id,
      templatas,
    })
  }
}

pub struct ExternEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: &'t PackageEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
}

impl<'s, 't> ExternEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub fn build_in(
    self,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t ExternEnvironmentT<'s, 't> {
    let templatas = self.templatas_builder.build_in(interner);
    interner.alloc(ExternEnvironmentT {
      global_env: self.global_env,
      parent_env: self.parent_env,
      template_id: self.template_id,
      id: self.id,
      templatas,
    })
  }
}

pub struct GeneralEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IInDenizenEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
}

impl<'s, 't> GeneralEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub fn build_in(
    self,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t GeneralEnvironmentT<'s, 't> {
    let templatas = self.templatas_builder.build_in(interner);
    interner.alloc(GeneralEnvironmentT {
      global_env: self.global_env,
      parent_env: self.parent_env,
      template_id: self.template_id,
      id: self.id,
      templatas,
    })
  }
}