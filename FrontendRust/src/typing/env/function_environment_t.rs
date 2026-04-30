use std::collections::HashSet;
use crate::higher_typing::ast::FunctionA;
use crate::postparsing::expressions::IExpressionSE;
use crate::postparsing::names::IImpreciseNameS;
use crate::typing::ast::ast::LocationInFunctionEnvironmentT;
use crate::typing::env::environment::{
  GlobalEnvironmentT, IEnvironmentT, IInDenizenEnvironmentT, ILookupContext, TemplatasStoreBuilder, TemplatasStoreT,
};
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::names::names::{IdT, INameT, IVarNameT};
use crate::typing::templata::templata::{FunctionTemplataT, ITemplataT};
use crate::typing::types::types::{CoordT, RegionT, StructTT, VariabilityT};
use crate::typing::typing_interner::TypingInterner;

/*
package dev.vale.typing.env

import dev.vale.highertyping.FunctionA
import dev.vale.{Interner, vassert, vcurious, vfail, vpass}
import dev.vale.postparsing._
import dev.vale.typing.ast.{LocationInFunctionEnvironmentT, ParameterT}
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.highertyping._
import dev.vale.postparsing.IImpreciseNameS
import dev.vale.typing._
import dev.vale.typing.types.StructTT
import dev.vale.{Interner, Profiler, vassert, vcurious, vfail, vimpl, vpass, vwat}

import scala.collection.immutable.{List, Map, Set}

*/

// mig: struct BuildingFunctionEnvironmentWithClosuredsT
// mig: impl BuildingFunctionEnvironmentWithClosuredsT
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BuildingFunctionEnvironmentWithClosuredsT<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas: &'t TemplatasStoreT<'s, 't>,
  pub function: &'s FunctionA<'s>,
  pub variables: &'t [IVariableT<'s, 't>],
  pub is_root_compiling_denizen: bool,
}
/*
case class BuildingFunctionEnvironmentWithClosuredsT(
  globalEnv: GlobalEnvironment,
  parentEnv: IEnvironmentT,
  id: IdT[IFunctionTemplateNameT],
  templatas: TemplatasStore,
  function: FunctionA,
  variables: Vector[IVariableT],
  isRootCompilingDenizen: Boolean
) extends IInDenizenEnvironmentT {
*/
// mig: fn templata
impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsT<'s, 't> where 's: 't {
  pub fn templata(&self) -> FunctionTemplataT<'s, 't> {
    panic!("Unimplemented: templata");
  }
  /*
    def templata = FunctionTemplataT(parentEnv, function)
  */
}
// mig: override fn hashCode
impl<'s, 't> std::hash::Hash for BuildingFunctionEnvironmentWithClosuredsT<'s, 't> where 's: 't {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.id.hash(state); }
  /* Guardian: disable-all */
}
/*
  override def denizenTemplateId: IdT[ITemplateNameT] = id
  override def denizenId: IdT[INameT] = id

  val hash = runtime.ScalaRunTime._hashCode(id);
override def hashCode(): Int = hash;
*/
// mig: override fn eq
impl<'s, 't> PartialEq for BuildingFunctionEnvironmentWithClosuredsT<'s, 't> where 's: 't {
  fn eq(&self, other: &Self) -> bool { self.id == other.id }
  /* Guardian: disable-all */
}
impl<'s, 't> Eq for BuildingFunctionEnvironmentWithClosuredsT<'s, 't> where 's: 't {}
/*
  override def equals(obj: Any): Boolean = {
    if (!obj.isInstanceOf[IInDenizenEnvironmentT]) {
      return false
    }
    return id.equals(obj.asInstanceOf[IInDenizenEnvironmentT].id)
  }
*/
// mig: override fn root_compiling_denizen_env
impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsT<'s, 't> where 's: 't {
  pub fn root_compiling_denizen_env(&'t self) -> IInDenizenEnvironmentT<'s, 't> {
    panic!("Unimplemented: root_compiling_denizen_env");
  }
  /*
    override def rootCompilingDenizenEnv: IInDenizenEnvironmentT = {
      if (isRootCompilingDenizen) {
        this
      } else {
        parentEnv match {
          case PackageEnvironmentT(_, _, _) => vwat()
          case _ => {
            parentEnv match {
              case parentInDenizenEnv : IInDenizenEnvironmentT => {
                parentInDenizenEnv.rootCompilingDenizenEnv
              }
              case _ => vwat()
            }
          }
        }
      }
    }
  */
}
// mig: fn lookup_with_name_inner
impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsT<'s, 't> where 's: 't {
  pub fn lookup_with_name_inner(
    &'t self,
    name: INameT<'s, 't>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
  ) -> Vec<ITemplataT<'s, 't>> {
    panic!("Unimplemented: lookup_with_name_inner");
  }
  /*
    private[env] override def lookupWithNameInner(

      name: INameT,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
    Array[ITemplataT[ITemplataType]] = {
      EnvironmentHelper.lookupWithNameInner(
        this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
    }
  */
}
// mig: fn lookup_with_imprecise_name_inner
impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsT<'s, 't> where 's: 't {
  pub fn lookup_with_imprecise_name_inner(
    &'t self,
    name: IImpreciseNameS<'s>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
  ) -> Vec<ITemplataT<'s, 't>> {
    lookup_with_imprecise_name_inner(
      IEnvironmentT::BuildingWithClosureds(self), &self.templatas, self.parent_env, name, lookup_filter, get_only_nearest)
  }
  /*
    private[env] override def lookupWithImpreciseNameInner(

      name: IImpreciseNameS,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
    Array[ITemplataT[ITemplataType]] = {
      EnvironmentHelper.lookupWithImpreciseNameInner(
        this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
    }
  }
  */
}

// mig: struct BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT
// mig: impl BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub template_args: &'t [ITemplataT<'s, 't>],
  pub templatas: &'t TemplatasStoreT<'s, 't>,
  pub function: &'s FunctionA<'s>,
  pub variables: &'t [IVariableT<'s, 't>],
  pub is_root_compiling_denizen: bool,
  pub default_region: RegionT,
}
/*
case class BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT(
  globalEnv: GlobalEnvironment,
  parentEnv: IEnvironmentT,
  id: IdT[IFunctionTemplateNameT],
  templateArgs: Vector[ITemplataT[ITemplataType]],
  templatas: TemplatasStore,
  function: FunctionA,
  variables: Vector[IVariableT],
  isRootCompilingDenizen: Boolean,
  defaultRegion: RegionT
) extends IInDenizenEnvironmentT {
*/
// mig: override fn hashCode
impl<'s, 't> std::hash::Hash for BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> where 's: 't {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.id.hash(state); }
  /* Guardian: disable-all */
}
/*
  override def denizenTemplateId: IdT[ITemplateNameT] = id
  override def denizenId: IdT[INameT] = id

  val hash = runtime.ScalaRunTime._hashCode(id);
override def hashCode(): Int = hash;
*/
// mig: override fn eq
impl<'s, 't> PartialEq for BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> where 's: 't {
  fn eq(&self, other: &Self) -> bool { self.id == other.id }
  /* Guardian: disable-all */
}
impl<'s, 't> Eq for BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> where 's: 't {}
/*
  override def equals(obj: Any): Boolean = {
    if (!obj.isInstanceOf[IInDenizenEnvironmentT]) {
      return false
    }
    return id.equals(obj.asInstanceOf[IInDenizenEnvironmentT].id)
  }
*/
// mig: override fn root_compiling_denizen_env
impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> where 's: 't {
  pub fn root_compiling_denizen_env(&'t self) -> IInDenizenEnvironmentT<'s, 't> {
    panic!("Unimplemented: root_compiling_denizen_env");
  }
  /*
    override def rootCompilingDenizenEnv: IInDenizenEnvironmentT = {
      if (isRootCompilingDenizen) {
        this
      } else {
        parentEnv match {
          case PackageEnvironmentT(_, _, _) => vwat()
          case _ => {
            parentEnv match {
              case parentInDenizenEnv : IInDenizenEnvironmentT => {
                parentInDenizenEnv.rootCompilingDenizenEnv
              }
              case _ => vwat()
            }
          }
        }
      }
    }
  */
}
// mig: fn lookup_with_name_inner
impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> where 's: 't {
  pub fn lookup_with_name_inner(
    &'t self,
    name: INameT<'s, 't>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
  ) -> Vec<ITemplataT<'s, 't>> {
    panic!("Unimplemented: lookup_with_name_inner");
  }
  /*
    private[env] override def lookupWithNameInner(

      name: INameT,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
    Array[ITemplataT[ITemplataType]] = {
      EnvironmentHelper.lookupWithNameInner(
        this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
    }
  */
}
// mig: fn lookup_with_imprecise_name_inner
impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> where 's: 't {
  pub fn lookup_with_imprecise_name_inner(
    &'t self,
    name: IImpreciseNameS<'s>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
  ) -> Vec<ITemplataT<'s, 't>> {
    // EnvironmentHelper.lookupWithImpreciseNameInner(this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
    lookup_with_imprecise_name_inner(
      IEnvironmentT::BuildingWithClosuredsAndTemplateArgs(self), &self.templatas, self.parent_env, name, lookup_filter, get_only_nearest)
  }
  /*
    private[env] override def lookupWithImpreciseNameInner(

      name: IImpreciseNameS,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
    Array[ITemplataT[ITemplataType]] = {
      EnvironmentHelper.lookupWithImpreciseNameInner(
        this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
    }

  }
  */
}

// mig: struct NodeEnvironmentT
// mig: impl NodeEnvironmentT
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct NodeEnvironmentT<'s, 't>
where 's: 't,
{
  pub parent_function_env: &'t FunctionEnvironmentT<'s, 't>,
  pub parent_node_env: Option<&'t NodeEnvironmentT<'s, 't>>,
  pub node: &'s IExpressionSE<'s>,
  pub life: LocationInFunctionEnvironmentT<'s>,
  pub templatas: &'t TemplatasStoreT<'s, 't>,
  pub declared_locals: &'t [IVariableT<'s, 't>],
  pub unstackified_locals: &'t [IVarNameT<'s, 't>],
  pub restackified_locals: &'t [IVarNameT<'s, 't>],
  pub default_region: RegionT,
}
/*
case class NodeEnvironmentT(
  parentFunctionEnv: FunctionEnvironmentT,
  parentNodeEnv: Option[NodeEnvironmentT],
  node: IExpressionSE,
  life: LocationInFunctionEnvironmentT,

  // The things below are the "state"; they can be different for any given line in a function.
  templatas: TemplatasStore,
  // This contains locals from parent blocks, see WTHPFE.
  declaredLocals: Vector[IVariableT],
  // This can refer to vars in parent blocks, see UCRTVPE.
  unstackifiedLocals: Set[IVarNameT],
  // This can refer to vars in parent blocks, see UCRTVPE.
  restackifiedLocals: Set[IVarNameT],

  defaultRegion: RegionT,
) extends IInDenizenEnvironmentT {
*/
/*
  vassert(declaredLocals.map(_.name) == declaredLocals.map(_.name).distinct)
*/
/*
*/
// mig: override fn hashCode
// Scala hashes `id.hashCode ^ life.hashCode` and compares `(id, life)`. The id
// delegates to parent_function_env.id.
impl<'s, 't> std::hash::Hash for NodeEnvironmentT<'s, 't> where 's: 't {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.parent_function_env.id.hash(state);
    self.life.hash(state);
  }
  /* Guardian: disable-all */
}
/*
  val hash = id.hashCode() ^ life.hashCode();
  override def hashCode(): Int = hash;
*/
// mig: override fn eq
impl<'s, 't> PartialEq for NodeEnvironmentT<'s, 't> where 's: 't {
  fn eq(&self, other: &Self) -> bool {
    self.parent_function_env.id == other.parent_function_env.id
      && self.life == other.life
  }
  /* Guardian: disable-all */
}
impl<'s, 't> Eq for NodeEnvironmentT<'s, 't> where 's: 't {}
/*
  override def equals(obj: Any): Boolean = {
    obj match {
      case that @ NodeEnvironmentT(_, _, _, _, _, _, _, _, _) => {
        id == that.id && life == that.life
      }
    }
  }
*/
// mig: override fn root_compiling_denizen_env
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn root_compiling_denizen_env(&'t self) -> IInDenizenEnvironmentT<'s, 't> {
    panic!("Unimplemented: root_compiling_denizen_env");
  }
  /*
    override def denizenTemplateId: IdT[ITemplateNameT] = parentFunctionEnv.denizenTemplateId
    override def denizenId: IdT[INameT] = parentFunctionEnv.denizenId

    override def rootCompilingDenizenEnv: IInDenizenEnvironmentT = {
  //    parentEnv match {
  //      case PackageEnvironment(_, _, _) => this
  //      case _ => parentEnv.rootCompilingDenizenEnv
  //    }
      parentEnv.rootCompilingDenizenEnv
    }
  */
}
// mig: override fn id
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn id(&self) -> IdT<'s, 't> {
    panic!("Unimplemented: id");
  }
  /*
    override def id: IdT[IFunctionNameT] = parentFunctionEnv.id
  */
}
// mig: fn function
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn function(&self) -> &'s FunctionA<'s> {
    panic!("Unimplemented: function");
  }
  /*
    def function = parentFunctionEnv.function
  */
}
// mig: fn lookup_with_name_inner
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn lookup_with_name_inner(
    &'t self,
    name: INameT<'s, 't>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
  ) -> Vec<ITemplataT<'s, 't>> {
    panic!("Unimplemented: lookup_with_name_inner");
  }
  /*
    private[env] override def lookupWithNameInner(

      name: INameT,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
    Array[ITemplataT[ITemplataType]] = {
      EnvironmentHelper.lookupWithNameInner(
        this, templatas, parentNodeEnv.getOrElse(parentFunctionEnv), name, lookupFilter, getOnlyNearest)
    }
  */
}
// mig: fn lookup_with_imprecise_name_inner
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn lookup_with_imprecise_name_inner(
    &'t self,
    name: IImpreciseNameS<'s>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
  ) -> Vec<ITemplataT<'s, 't>> {
    panic!("Unimplemented: lookup_with_imprecise_name_inner");
  }
  /*
    private[env] override def lookupWithImpreciseNameInner(

      name: IImpreciseNameS,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
    Array[ITemplataT[ITemplataType]] = {
      EnvironmentHelper.lookupWithImpreciseNameInner(
        this, templatas, parentNodeEnv.getOrElse(parentFunctionEnv), name, lookupFilter, getOnlyNearest)
    }
  */
}
// mig: fn global_env
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn global_env(&self) -> &'t GlobalEnvironmentT<'s, 't> {
    panic!("Unimplemented: global_env");
  }
  /*
    def globalEnv: GlobalEnvironment = parentFunctionEnv.globalEnv
  */
}
// mig: fn parent_env
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn parent_env(&self) -> IInDenizenEnvironmentT<'s, 't> {
    panic!("Unimplemented: parent_env");
  }
  /*
    def parentEnv: IInDenizenEnvironmentT = {
      parentNodeEnv.getOrElse(parentFunctionEnv)
    }
  */
}
// mig: fn get_variable
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn get_variable(&self, name: IVarNameT<'s, 't>) -> Option<IVariableT<'s, 't>> {
    panic!("Unimplemented: get_variable");
  }
  /*
    def getVariable(name: IVarNameT): Option[IVariableT] = {
      declaredLocals.find(_.name == name) match {
        case Some(v) => Some(v)
        case None => {
          parentNodeEnv match {
            case Some(p) => p.getVariable(name)
            case None => {
              parentFunctionEnv.closuredLocals.find(_.name == name)
            }
          }
        }
      }
    }

    // Dont have a getAllUnstackifiedLocals or getAllLiveLocals here. We learned that the hard way.
    // See UCRTVPE, child environments would be the ones that know about their unstackifying of locals
    // from parent envs.
  */
}
// mig: fn get_all_locals
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn get_all_locals(&self) -> Vec<ILocalVariableT<'s, 't>> {
    panic!("Unimplemented: get_all_locals");
  }
  /*
    def getAllLocals(): Vector[ILocalVariableT] = {
      declaredLocals.collect({ case i : ILocalVariableT => i })
    }
  */
}
// mig: fn get_all_unstackified_locals
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn get_all_unstackified_locals(&self) -> Vec<IVarNameT<'s, 't>> {
    panic!("Unimplemented: get_all_unstackified_locals");
  }
  /*
    def getAllUnstackifiedLocals(): Vector[IVarNameT] = {
      unstackifiedLocals.toVector
    }
  */
}
// mig: fn add_variables
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn add_variables(&self, new_vars: &[IVariableT<'s, 't>]) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: add_variables");
  }
  /*
    def addVariables(newVars: Vector[IVariableT]): NodeEnvironmentT = {
      NodeEnvironmentT(
        parentFunctionEnv,
        parentNodeEnv,
        node,
        life,
        templatas,
        declaredLocals ++ newVars,
        unstackifiedLocals,
        restackifiedLocals,
        defaultRegion)
    }
  */
}
// mig: fn add_variable
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn add_variable(&self, new_var: IVariableT<'s, 't>) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: add_variable");
  }
  /*
    def addVariable(newVar: IVariableT): NodeEnvironmentT = {
      NodeEnvironmentT(
        parentFunctionEnv,
        parentNodeEnv,
        node,
        life,
        templatas,
        declaredLocals :+ newVar,
        unstackifiedLocals,
        restackifiedLocals,
        defaultRegion)
    }
  */
}
// mig: fn get_all_restackified_locals
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn get_all_restackified_locals(&self) -> Vec<IVarNameT<'s, 't>> {
    panic!("Unimplemented: get_all_restackified_locals");
  }
  /*
    def getAllRestackifiedLocals(): Vector[IVarNameT] = {
      restackifiedLocals.toVector
    }
  */
}
// mig: fn mark_local_unstackified
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn mark_local_unstackified(&self, new_unstackified: IVarNameT<'s, 't>) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: mark_local_unstackified");
  }
  /*
    def markLocalUnstackified(newUnstackified: IVarNameT): NodeEnvironmentT = {
      vassert(getAllLocals().exists(_.name == newUnstackified))
      vassert(!getAllUnstackifiedLocals().contains(newUnstackified))

      if (getAllRestackifiedLocals().contains(newUnstackified)) {
        // It was a restackified local, so don't mark it as unstackified, just undo the
        // restackification.
        // Even if the local belongs to a parent env, we still mark it unstackified here, see UCRTVPE.
        NodeEnvironmentT(
          parentFunctionEnv,
          parentNodeEnv,
          node,
          life,
          templatas,
          declaredLocals,
          unstackifiedLocals,
          restackifiedLocals - newUnstackified,
          defaultRegion)
      } else {
        // Even if the local belongs to a parent env, we still mark it unstackified here, see UCRTVPE.
        NodeEnvironmentT(
          parentFunctionEnv,
          parentNodeEnv,
          node,
          life,
          templatas,
          declaredLocals,
          unstackifiedLocals + newUnstackified,
          restackifiedLocals,
          defaultRegion)
      }
    }
  */
}
// mig: fn mark_local_restackified
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn mark_local_restackified(&self, new_restackified: IVarNameT<'s, 't>) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: mark_local_restackified");
  }
  /*
    def markLocalRestackified(newRestackified: IVarNameT): NodeEnvironmentT = {
      vassert(getAllLocals().exists(_.name == newRestackified))
      vassert(!getAllRestackifiedLocals().contains(newRestackified))
      if (getAllUnstackifiedLocals().contains(newRestackified)) {
        // It was an unstackified local, so don't mark it as restackified, just undo the
        // unstackification.
        // Even if the local belongs to a parent env, we still mark it restackified here, see UCRTVPE.
        NodeEnvironmentT(
          parentFunctionEnv,
          parentNodeEnv,
          node,
          life,
          templatas,
          declaredLocals,
          unstackifiedLocals - newRestackified,
          restackifiedLocals,
          defaultRegion)
      } else {
        // Even if the local belongs to a parent env, we still mark it restackified here, see UCRTVPE.
        NodeEnvironmentT(
          parentFunctionEnv,
          parentNodeEnv,
          node,
          life,
          templatas,
          declaredLocals,
          unstackifiedLocals,
          restackifiedLocals + newRestackified,
          defaultRegion)
      }
    }
  */
}
// mig: fn get_effects_since
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn get_effects_since(
    &self,
    earlier_node_env: &NodeEnvironmentT<'s, 't>,
  ) -> (Vec<IVarNameT<'s, 't>>, Vec<IVarNameT<'s, 't>>) {
    panic!("Unimplemented: get_effects_since");
  }
  /*
    // Gets the effects that this environment had on the outside world (on its parent
    // environments). In other words, parent locals that were unstackified.
    def getEffectsSince(earlierNodeEnv: NodeEnvironmentT): (Set[IVarNameT], Set[IVarNameT]) = {
      vassert(parentFunctionEnv == earlierNodeEnv.parentFunctionEnv)

      // We may have unstackified outside locals from inside the block, make sure
      // the parent environment knows about that.

      // declaredLocals contains things from parent environment, which is why we need to receive
      // an earlier environment to compare to, see WTHPFE.
      val earlierNodeEnvDeclaredLocals = earlierNodeEnv.declaredLocals.map(_.name).toSet
      val earlierNodeEnvLiveLocals = earlierNodeEnvDeclaredLocals -- earlierNodeEnv.unstackifiedLocals
      val liveLocalsIntroducedSinceEarlier =
        declaredLocals.map(_.name).filter(x => !earlierNodeEnvLiveLocals.contains(x))

      val unstackifiedAncestorLocals = unstackifiedLocals -- liveLocalsIntroducedSinceEarlier

      val restackifiedAncestorLocals = restackifiedLocals -- liveLocalsIntroducedSinceEarlier

      (unstackifiedAncestorLocals, restackifiedAncestorLocals)
    }
  */
}
// mig: fn get_live_variables_introduced_since
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn get_live_variables_introduced_since(
    &self,
    since_nenv: &NodeEnvironmentT<'s, 't>,
  ) -> Vec<ILocalVariableT<'s, 't>> {
    panic!("Unimplemented: get_live_variables_introduced_since");
  }
  /*
    def getLiveVariablesIntroducedSince(
      sinceNenv: NodeEnvironmentT):
    Vector[ILocalVariableT] = {
      val localsAsOfThen =
        sinceNenv.declaredLocals.collect({
          case x @ ReferenceLocalVariableT(_, _, _) => x
          case x @ AddressibleLocalVariableT(_, _, _) => x
        })
      val localsAsOfNow =
        declaredLocals.collect({
          case x @ ReferenceLocalVariableT(_, _, _) => x
          case x @ AddressibleLocalVariableT(_, _, _) => x
        })

      vassert(localsAsOfNow.startsWith(localsAsOfThen))
      val localsDeclaredSinceThen = localsAsOfNow.slice(localsAsOfThen.size, localsAsOfNow.size)
      vassert(localsDeclaredSinceThen.size == localsAsOfNow.size - localsAsOfThen.size)

      val unmovedLocalsDeclaredSinceThen =
        localsDeclaredSinceThen.filter(x => !unstackifiedLocals.contains(x.name))

      unmovedLocalsDeclaredSinceThen
    }
  */
}
// mig: fn make_child
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn make_child(
    &'t self,
    node: &'s IExpressionSE<'s>,
    maybe_new_default_region: Option<RegionT>,
  ) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: make_child");
  }
  /*
    def makeChild(
        node: IExpressionSE,
        maybeNewDefaultRegion: Option[RegionT]):
    NodeEnvironmentT = {
      NodeEnvironmentT(
        parentFunctionEnv,
        Some(this),
        node,
        life,
        TemplatasStore(id, Map(), Map()),
        declaredLocals, // See WTHPFE.
        unstackifiedLocals, // See WTHPFE
        restackifiedLocals,
        maybeNewDefaultRegion.getOrElse(defaultRegion)) // See WTHPFE.
    }
  */
}
// mig: fn add_entry
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn add_entry(
    &self,
    interner: &TypingInterner<'s, 't>,
    name: INameT<'s, 't>,
    entry: IEnvEntryT<'s, 't>,
  ) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: add_entry");
  }
  /*
    def addEntry(interner: Interner, name: INameT, entry: IEnvEntry): NodeEnvironmentT = {
      NodeEnvironmentT(
        parentFunctionEnv,
        parentNodeEnv,
        node,
        life,
        templatas.addEntry(interner, name, entry),
        declaredLocals,
        unstackifiedLocals,
        restackifiedLocals,
        defaultRegion)
    }
  */
}
// mig: fn add_entries
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn add_entries(
    &self,
    interner: &TypingInterner<'s, 't>,
    new_entries: &[(INameT<'s, 't>, IEnvEntryT<'s, 't>)],
  ) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: add_entries");
  }
  /*
    def addEntries(interner: Interner, newEntries: Vector[(INameT, IEnvEntry)]): NodeEnvironmentT = {
      NodeEnvironmentT(
        parentFunctionEnv,
        parentNodeEnv,
        node,
        life,
        templatas.addEntries(interner, newEntries),
        declaredLocals,
        unstackifiedLocals,
        restackifiedLocals,
        defaultRegion)
    }
  */
}
// mig: fn nearest_block_env
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn nearest_block_env(&self) -> Option<(&'t NodeEnvironmentT<'s, 't>, &'s IExpressionSE<'s>)> {
    panic!("Unimplemented: nearest_block_env");
  }
  /*
    def nearestBlockEnv(): Option[(NodeEnvironmentT, BlockSE)] = {
      node match {
        case b @ BlockSE(_, _, _) => Some((this, b))
        case _ => parentNodeEnv.flatMap(_.nearestBlockEnv())
      }
    }
  */
}
// mig: fn nearest_loop_env
impl<'s, 't> NodeEnvironmentT<'s, 't> where 's: 't {
  pub fn nearest_loop_env(&self) -> Option<(&'t NodeEnvironmentT<'s, 't>, &'s IExpressionSE<'s>)> {
    panic!("Unimplemented: nearest_loop_env");
  }
  /*
    def nearestLoopEnv(): Option[(NodeEnvironmentT, IExpressionSE)] = {
      node match {
        case w @ WhileSE(_, _) => Some((this, w))
        case w @ MapSE(_, _) => Some((this, w))
        case _ => parentNodeEnv.flatMap(_.nearestLoopEnv())
      }
    }
  }

  */
}

// mig: struct NodeEnvironmentBox
// mig: impl NodeEnvironmentBox
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox / FunctionEnvironmentBoxT / IDenizenEnvironmentBoxT
//  Scala mutable wrappers are subsumed by the builder-freeze pattern in Rust.)
/*
case class NodeEnvironmentBox(var nodeEnvironment: NodeEnvironmentT) {
*/
// mig: override fn eq
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: override fn hashCode
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
override def hashCode(): Int = vfail() // Shouldnt hash, is mutable
*/
// mig: fn snapshot
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def snapshot: NodeEnvironmentT = nodeEnvironment
*/
// mig: fn default_region
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def defaultRegion: RegionT = nodeEnvironment.defaultRegion
*/
// mig: fn id
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def id: IdT[IFunctionNameT] = nodeEnvironment.parentFunctionEnv.id
*/
// mig: fn node
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def node: IExpressionSE = nodeEnvironment.node
*/
// mig: fn maybe_return_type
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def maybeReturnType: Option[CoordT] = nodeEnvironment.parentFunctionEnv.maybeReturnType
*/
// mig: fn global_env
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def globalEnv: GlobalEnvironment = nodeEnvironment.globalEnv
*/
// mig: fn declared_locals
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def declaredLocals: Vector[IVariableT] = nodeEnvironment.declaredLocals
*/
// mig: fn unstackifieds
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def unstackifieds: Set[IVarNameT] = nodeEnvironment.unstackifiedLocals
*/
// mig: fn function
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def function = nodeEnvironment.function
*/
// mig: fn function_environment
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def functionEnvironment = nodeEnvironment.parentFunctionEnv
*/
// mig: fn add_variable
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def addVariable(newVar: IVariableT): Unit= {
    nodeEnvironment = nodeEnvironment.addVariable(newVar)
  }
*/
// mig: fn mark_local_unstackified
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def markLocalUnstackified(newMoved: IVarNameT): Unit= {
    nodeEnvironment = nodeEnvironment.markLocalUnstackified(newMoved)
  }
*/
// mig: fn mark_local_restackified
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def markLocalRestackified(newMoved: IVarNameT): Unit= {
    nodeEnvironment = nodeEnvironment.markLocalRestackified(newMoved)
  }
*/
// mig: fn get_variable
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def getVariable(name: IVarNameT): Option[IVariableT] = {
    nodeEnvironment.getVariable(name)
  }
*/
// mig: fn get_all_locals
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def getAllLocals(): Vector[ILocalVariableT] = {
    nodeEnvironment.getAllLocals()
  }
*/
// mig: fn get_all_unstackified_locals
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def getAllUnstackifiedLocals(): Vector[IVarNameT] = {
    nodeEnvironment.getAllUnstackifiedLocals()
  }
*/
// mig: fn lookup_nearest_with_imprecise_name
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def lookupNearestWithImpreciseName(

    nameS: IImpreciseNameS,
    lookupFilter: Set[ILookupContext]):
  Option[ITemplataT[ITemplataType]] = {
    nodeEnvironment.lookupNearestWithImpreciseName(nameS, lookupFilter)
  }
*/
// mig: fn lookup_nearest_with_name
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def lookupNearestWithName(

    nameS: INameT,
    lookupFilter: Set[ILookupContext]):
  Option[ITemplataT[ITemplataType]] = {
    nodeEnvironment.lookupNearestWithName(nameS, lookupFilter)
  }
*/
// mig: fn lookup_all_with_imprecise_name
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def lookupAllWithImpreciseName( nameS: IImpreciseNameS, lookupFilter: Set[ILookupContext]): Array[ITemplataT[ITemplataType]] = {
    nodeEnvironment.lookupAllWithImpreciseName(nameS, lookupFilter)
  }
*/
// mig: fn lookup_all_with_name
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def lookupAllWithName( nameS: INameT, lookupFilter: Set[ILookupContext]): Iterable[ITemplataT[ITemplataType]] = {
    nodeEnvironment.lookupAllWithName(nameS, lookupFilter)
  }
*/
// mig: fn lookup_with_imprecise_name_inner
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  private[env] def lookupWithImpreciseNameInner( nameS: IImpreciseNameS, lookupFilter: Set[ILookupContext], getOnlyNearest: Boolean) = {
    nodeEnvironment.lookupWithImpreciseNameInner(nameS, lookupFilter, getOnlyNearest)
  }
*/
// mig: fn lookup_with_name_inner
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  private[env] def lookupWithNameInner( nameS: INameT, lookupFilter: Set[ILookupContext], getOnlyNearest: Boolean) = {
    nodeEnvironment.lookupWithNameInner(nameS, lookupFilter, getOnlyNearest)
  }
*/
// mig: fn make_child
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def makeChild(
    node: IExpressionSE,
    maybeNewDefaultRegion: Option[RegionT]):
  NodeEnvironmentT = {
    nodeEnvironment.makeChild(node, maybeNewDefaultRegion)
  }
*/
// mig: fn add_entry
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def addEntry(interner: Interner, name: INameT, entry: IEnvEntry): Unit = {
    nodeEnvironment = nodeEnvironment.addEntry(interner, name, entry)
  }
*/
// mig: fn add_entries
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def addEntries(interner: Interner, newEntries: Vector[(INameT, IEnvEntry)]): Unit= {
    nodeEnvironment = nodeEnvironment.addEntries(interner, newEntries)
  }
*/
// mig: fn nearest_block_env
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def nearestBlockEnv(): Option[(NodeEnvironmentT, BlockSE)] = {
    nodeEnvironment.nearestBlockEnv()
  }
*/
// mig: fn nearest_loop_env
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — NodeEnvironmentBox subsumed by builder-freeze pattern.)
/*
  def nearestLoopEnv(): Option[(NodeEnvironmentT, IExpressionSE)] = {
    nodeEnvironment.nearestLoopEnv()
  }
}

*/
// mig: struct FunctionEnvironmentT
// mig: impl FunctionEnvironmentT
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct FunctionEnvironmentT<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas: &'t TemplatasStoreT<'s, 't>,
  pub function: &'s FunctionA<'s>,
  pub maybe_return_type: Option<CoordT<'s, 't>>,
  pub closured_locals: &'t [IVariableT<'s, 't>],
  pub is_root_compiling_denizen: bool,
  pub default_region: RegionT,
}
/*
case class FunctionEnvironmentT(
  // These things are the "environment"; they are the same for every line in a function.
  globalEnv: GlobalEnvironment,
  // This points to the environment containing the function, not parent blocks, see WTHPFE.
  parentEnv: IEnvironmentT,
  templateId: IdT[IFunctionTemplateNameT],
  id: IdT[IFunctionNameT], // Includes the name of the function

  templatas: TemplatasStore,

  function: FunctionA,
  maybeReturnType: Option[CoordT],

  closuredLocals: Vector[IVariableT],

  isRootCompilingDenizen: Boolean,

  defaultRegion: RegionT,

  // Eventually we might have a list of imported environments here, pointing at the
  // environments in the global environment.
) extends IInDenizenEnvironmentT {
*/
// mig: override fn hashCode
impl<'s, 't> std::hash::Hash for FunctionEnvironmentT<'s, 't> where 's: 't {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.id.hash(state); }
  /* Guardian: disable-all */
}
/*
  val hash = runtime.ScalaRunTime._hashCode(id);
override def hashCode(): Int = hash;
*/
// mig: override fn eq
impl<'s, 't> PartialEq for FunctionEnvironmentT<'s, 't> where 's: 't {
  fn eq(&self, other: &Self) -> bool { self.id == other.id }
  /* Guardian: disable-all */
}
impl<'s, 't> Eq for FunctionEnvironmentT<'s, 't> where 's: 't {}
/*
  override def denizenTemplateId: IdT[ITemplateNameT] = templateId
  override def denizenId: IdT[INameT] = templateId

  override def equals(obj: Any): Boolean = {
    if (!obj.isInstanceOf[IInDenizenEnvironmentT]) {
      return false
    }
    return id.equals(obj.asInstanceOf[IInDenizenEnvironmentT].id)
  }
*/
// mig: override fn root_compiling_denizen_env
impl<'s, 't> FunctionEnvironmentT<'s, 't> where 's: 't {
  pub fn root_compiling_denizen_env(&'t self) -> IInDenizenEnvironmentT<'s, 't> {
    panic!("Unimplemented: root_compiling_denizen_env");
  }
  /*
    override def rootCompilingDenizenEnv: IInDenizenEnvironmentT = {
      if (isRootCompilingDenizen) {
        this
      } else {
        parentEnv match {
          case PackageEnvironmentT(_, _, _) => vwat()
          case _ => {
            parentEnv match {
              case parentInDenizenEnv : IInDenizenEnvironmentT => {
                parentInDenizenEnv.rootCompilingDenizenEnv
              }
              case _ => vwat()
            }
          }
        }
      }
    }
  */
}
// mig: fn templata
impl<'s, 't> FunctionEnvironmentT<'s, 't> where 's: 't {
  pub fn templata(&self) -> FunctionTemplataT<'s, 't> {
    panic!("Unimplemented: templata");
  }
  /*
    def templata = FunctionTemplataT(parentEnv, function)
  */
}
// mig: fn add_entry
impl<'s, 't> FunctionEnvironmentT<'s, 't> where 's: 't {
  pub fn add_entry(
    &self,
    interner: &TypingInterner<'s, 't>,
    name: INameT<'s, 't>,
    entry: IEnvEntryT<'s, 't>,
  ) -> &'t FunctionEnvironmentT<'s, 't> {
    panic!("Unimplemented: add_entry");
  }
  /*
    def addEntry(interner: Interner, name: INameT, entry: IEnvEntry): FunctionEnvironmentT = {
      FunctionEnvironmentT(
        globalEnv,
        parentEnv,
        templateId,
        id,
        templatas.addEntry(interner, name, entry),
        function,
        maybeReturnType,
        closuredLocals,
        isRootCompilingDenizen,
        defaultRegion)
    }
  */
}
// mig: fn add_entries
impl<'s, 't> FunctionEnvironmentT<'s, 't> where 's: 't {
  pub fn add_entries(
    &self,
    interner: &TypingInterner<'s, 't>,
    new_entries: &[(INameT<'s, 't>, IEnvEntryT<'s, 't>)],
  ) -> &'t FunctionEnvironmentT<'s, 't> {
    panic!("Unimplemented: add_entries");
  }
  /*
    def addEntries(interner: Interner, newEntries: Vector[(INameT, IEnvEntry)]): FunctionEnvironmentT = {
      FunctionEnvironmentT(
        globalEnv,
        parentEnv,
        templateId,
        id,
        templatas.addEntries(interner, newEntries),
        function,
        maybeReturnType,
        closuredLocals,
        isRootCompilingDenizen,
        defaultRegion)
    }
  */
}
// mig: fn lookup_with_name_inner
impl<'s, 't> FunctionEnvironmentT<'s, 't> where 's: 't {
  pub fn lookup_with_name_inner(
    &'t self,
    name: INameT<'s, 't>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
  ) -> Vec<ITemplataT<'s, 't>> {
    panic!("Unimplemented: lookup_with_name_inner");
  }
  /*
    private[env] override def lookupWithNameInner(

      name: INameT,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
    Array[ITemplataT[ITemplataType]] = {
      EnvironmentHelper.lookupWithNameInner(
        this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
    }
  */
}
// mig: fn lookup_with_imprecise_name_inner
impl<'s, 't> FunctionEnvironmentT<'s, 't> where 's: 't {
  pub fn lookup_with_imprecise_name_inner(
    &'t self,
    name: IImpreciseNameS<'s>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
  ) -> Vec<ITemplataT<'s, 't>> {
    lookup_with_imprecise_name_inner(
      IEnvironmentT::Function(self), self.templatas, self.parent_env, name, lookup_filter, get_only_nearest)
  }
  /*
    private[env] override def lookupWithImpreciseNameInner(

      name: IImpreciseNameS,
      lookupFilter: Set[ILookupContext],
      getOnlyNearest: Boolean):
    Array[ITemplataT[ITemplataType]] = {
      EnvironmentHelper.lookupWithImpreciseNameInner(
        this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
    }
  */
}
// mig: fn make_child_node_environment
impl<'s, 't> FunctionEnvironmentT<'s, 't> where 's: 't {
  pub fn make_child_node_environment(
    &'t self,
    node: &'s IExpressionSE<'s>,
    life: LocationInFunctionEnvironmentT<'s>,
  ) -> NodeEnvironmentBuilder<'s, 't> {
    // See WTHPFE, if this is a lambda, we let our blocks start with
    // locals from the parent function.
    let (declared_locals, unstackified_locals, restackified_locals) =
      match &self.parent_env {
        IEnvironmentT::Node(_node_env) => {
          panic!("implement: make_child_node_environment — NodeEnvironmentT parent");
        }
        _ => (Vec::new(), Vec::new(), Vec::new()),
      };
    NodeEnvironmentBuilder {
      parent_function_env: self,
      parent_node_env: None,
      node,
      life,
      templatas_builder: TemplatasStoreBuilder::new(&self.id),
      declared_locals,
      unstackified_locals,
      restackified_locals,
      default_region: self.default_region,
    }
  }
  /*
    def makeChildNodeEnvironment(node: IExpressionSE, life: LocationInFunctionEnvironmentT): NodeEnvironmentT = {
      // See WTHPFE, if this is a lambda, we let our blocks start with
      // locals from the parent function.
      val (declaredLocals, unstackifiedLocals, restackifiedLocals) =
        parentEnv match {
          case NodeEnvironmentT(_, _, _, _, _, declaredLocals, unstackifiedLocals, restackifiedLocals, _) => {
            (declaredLocals, unstackifiedLocals, restackifiedLocals)
          }
          case _ => (Vector(), Set[IVarNameT](), Set[IVarNameT]())
        }

      NodeEnvironmentT(
        this,
        None,
        node,
        life,
        TemplatasStore(id, Map(), Map()),
        declaredLocals, // See WTHPFE.
        unstackifiedLocals, // See WTHPFE.
        restackifiedLocals, // See WTHPFE.
        defaultRegion)
    }
  */
}
// mig: fn get_closured_declared_locals
impl<'s, 't> FunctionEnvironmentT<'s, 't> where 's: 't {
  pub fn get_closured_declared_locals(&self) -> Vec<IVariableT<'s, 't>> {
    panic!("Unimplemented: get_closured_declared_locals");
  }
  /*
    def getClosuredDeclaredLocals(): Vector[IVariableT] = {
      parentEnv match {
        case n @ NodeEnvironmentT(_, _, _, _, _, _, _, _, _) => n.declaredLocals
        case f @ FunctionEnvironmentT(_, _, _, _, _, _, _, _, _, _) => f.getClosuredDeclaredLocals()
        case _ => Vector()
      }
    }

  //  def getClosuredUnstackifiedLocals(): Vector[IVariableT] = {
  //    parentEnv match {
  //      case n @ NodeEnvironment(_, _, _, _, _, _, _) => n.unstackifiedLocals
  //      case f @ FunctionEnvironment(_, _, _, _, _, _) => f.getClosuredDeclaredLocals()
  //      case _ => Vector()
  //    }
  //  }

    // No particular reason we don't have an addFunction like PackageEnvironment does
  }

  */
}

// mig: struct FunctionEnvironmentBoxT
// mig: impl FunctionEnvironmentBoxT
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT / IDenizenEnvironmentBoxT
//  Scala mutable wrappers are subsumed by the builder-freeze pattern in Rust.)
/*
case class FunctionEnvironmentBoxT(var functionEnvironment: FunctionEnvironmentT) extends IDenizenEnvironmentBoxT {
*/
// mig: override fn eq
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: override fn hashCode
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
override def hashCode(): Int = vfail() // Shouldnt hash, is mutable
*/
// mig: override fn denizen_template_id
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override def denizenTemplateId: IdT[ITemplateNameT] = functionEnvironment.denizenTemplateId
*/
// mig: override fn denizen_id
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override def denizenId: IdT[INameT] = functionEnvironment.denizenId
*/
// mig: override fn snapshot
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override def snapshot: FunctionEnvironmentT = functionEnvironment
*/
// mig: def id
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  def id: IdT[IFunctionNameT] = functionEnvironment.id
*/
// mig: fn function
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  def function: FunctionA = functionEnvironment.function
*/
// mig: fn maybe_return_type
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  def maybeReturnType: Option[CoordT] = functionEnvironment.maybeReturnType
*/
// mig: override fn global_env
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override def globalEnv: GlobalEnvironment = functionEnvironment.globalEnv
*/
// mig: override fn templatas
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override def templatas: TemplatasStore = functionEnvironment.templatas
*/
// mig: override fn root_compiling_denizen_env
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override def rootCompilingDenizenEnv: IInDenizenEnvironmentT = functionEnvironment.rootCompilingDenizenEnv
*/
// mig: fn set_return_type
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  def setReturnType(returnType: Option[CoordT]): Unit = {
    functionEnvironment = functionEnvironment.copy(maybeReturnType = returnType)
  }
*/
// mig: fn add_entry
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  def addEntry(interner: Interner, name: INameT, entry: IEnvEntry): Unit = {
    functionEnvironment = functionEnvironment.addEntry(interner, name, entry)
  }
*/
// mig: fn add_entries
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  def addEntries(interner: Interner, newEntries: Vector[(INameT, IEnvEntry)]): Unit= {
    functionEnvironment = functionEnvironment.addEntries(interner, newEntries)
  }
*/
// mig: override fn lookup_nearest_with_imprecise_name
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override def lookupNearestWithImpreciseName(

    nameS: IImpreciseNameS,
    lookupFilter: Set[ILookupContext]):
  Option[ITemplataT[ITemplataType]] = {
    functionEnvironment.lookupNearestWithImpreciseName(nameS, lookupFilter)
  }
*/
// mig: override fn lookup_nearest_with_name
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override def lookupNearestWithName(

    nameS: INameT,
    lookupFilter: Set[ILookupContext]):
  Option[ITemplataT[ITemplataType]] = {
    functionEnvironment.lookupNearestWithName(nameS, lookupFilter)
  }
*/
// mig: override fn lookup_all_with_imprecise_name
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override def lookupAllWithImpreciseName( nameS: IImpreciseNameS, lookupFilter: Set[ILookupContext]): Array[ITemplataT[ITemplataType]] = {
    functionEnvironment.lookupAllWithImpreciseName(nameS, lookupFilter)
  }
*/
// mig: override fn lookup_all_with_name
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override def lookupAllWithName( nameS: INameT, lookupFilter: Set[ILookupContext]): Iterable[ITemplataT[ITemplataType]] = {
    functionEnvironment.lookupAllWithName(nameS, lookupFilter)
  }
*/
// mig: override fn lookup_with_imprecise_name_inner
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override private[env] def lookupWithImpreciseNameInner( nameS: IImpreciseNameS, lookupFilter: Set[ILookupContext], getOnlyNearest: Boolean) = {
    functionEnvironment.lookupWithImpreciseNameInner(nameS, lookupFilter, getOnlyNearest)
  }
*/
// mig: override fn lookup_with_name_inner
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  override private[env] def lookupWithNameInner( nameS: INameT, lookupFilter: Set[ILookupContext], getOnlyNearest: Boolean): Array[ITemplataT[ITemplataType]] = {
    functionEnvironment.lookupWithNameInner(nameS, lookupFilter, getOnlyNearest)
  }
*/
// mig: fn make_child_node_environment
// (Deleted in Rust per typing-pass-design-v3.md §3.3 — FunctionEnvironmentBoxT subsumed by builder-freeze pattern.)
/*
  def makeChildNodeEnvironment(node: IExpressionSE, life: LocationInFunctionEnvironmentT): NodeEnvironmentT = {
    functionEnvironment.makeChildNodeEnvironment(node, life)
  }

  // No particular reason we don't have an addFunction like PackageEnvironment does
}

*/
// mig: enum IVariableT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IVariableT<'s, 't>
where 's: 't,
{
  AddressibleLocal(AddressibleLocalVariableT<'s, 't>),
  ReferenceLocal(ReferenceLocalVariableT<'s, 't>),
  AddressibleClosure(AddressibleClosureVariableT<'s, 't>),
  ReferenceClosure(ReferenceClosureVariableT<'s, 't>),
}
/*
sealed trait IVariableT  {
*/
// mig: fn name
impl<'s, 't> IVariableT<'s, 't> where 's: 't {
  pub fn name(&self) -> IVarNameT<'s, 't> {
    panic!("Unimplemented: name");
  }
  /*
    def name: IVarNameT
  */
}
// mig: fn variability
impl<'s, 't> IVariableT<'s, 't> where 's: 't {
  pub fn variability(&self) -> VariabilityT {
    panic!("Unimplemented: variability");
  }
  /*
    def variability: VariabilityT
  */
}
// mig: fn coord
impl<'s, 't> IVariableT<'s, 't> where 's: 't {
  pub fn coord(&self) -> CoordT<'s, 't> {
    panic!("Unimplemented: coord");
  }
  /*
    def coord: CoordT
  }
  */
}
// mig: enum ILocalVariableT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ILocalVariableT<'s, 't>
where 's: 't,
{
  Addressible(AddressibleLocalVariableT<'s, 't>),
  Reference(ReferenceLocalVariableT<'s, 't>),
}
/*
sealed trait ILocalVariableT extends IVariableT {
*/
// mig: fn name
impl<'s, 't> ILocalVariableT<'s, 't> where 's: 't {
  pub fn name(&self) -> IVarNameT<'s, 't> {
    panic!("Unimplemented: name");
  }
  /*
    def name: IVarNameT
  */
}
// mig: fn coord
impl<'s, 't> ILocalVariableT<'s, 't> where 's: 't {
  pub fn coord(&self) -> CoordT<'s, 't> {
    panic!("Unimplemented: coord");
  }
  /*
    def coord: CoordT
  }
  // Why the difference between reference and addressible:
  // If we mutate/move a variable from inside a closure, we need to put
  // the local's address into the struct. But, if the closures don't
  // mutate/move, then we could just put a regular reference in the struct.
  // Lucky for us, the parser figured out if any of our child closures did
  // any mutates/moves/borrows.
  */
}
// mig: struct AddressibleLocalVariableT
// mig: impl AddressibleLocalVariableT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressibleLocalVariableT<'s, 't>
where 's: 't,
{
  pub name: IVarNameT<'s, 't>,
  pub variability: VariabilityT,
  pub coord: CoordT<'s, 't>,
}
/*
case class AddressibleLocalVariableT(
  name: IVarNameT,
  variability: VariabilityT,
  coord: CoordT
) extends ILocalVariableT {
*/
// mig: override fn hashCode
// (Realized by `#[derive(Hash)]` on AddressibleLocalVariableT above.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: override fn eq
// (Realized by `#[derive(PartialEq, Eq)]` on AddressibleLocalVariableT above.)
/*
override def equals(obj: Any): Boolean = vcurious();

}
*/
// mig: struct ReferenceLocalVariableT
// mig: impl ReferenceLocalVariableT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceLocalVariableT<'s, 't>
where 's: 't,
{
  pub name: IVarNameT<'s, 't>,
  pub variability: VariabilityT,
  pub coord: CoordT<'s, 't>,
}
/*
case class ReferenceLocalVariableT(
  name: IVarNameT,
  variability: VariabilityT,
  coord: CoordT
) extends ILocalVariableT {
*/
// mig: override def hashCode
// (Realized by `#[derive(Hash)]` on ReferenceLocalVariableT above.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: override fn eq
// (Realized by `#[derive(PartialEq, Eq)]` on ReferenceLocalVariableT above.)
/*
override def equals(obj: Any): Boolean = vcurious();
  vpass()
}
*/
// mig: struct AddressibleClosureVariableT
// mig: impl AddressibleClosureVariableT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressibleClosureVariableT<'s, 't>
where 's: 't,
{
  pub name: IVarNameT<'s, 't>,
  pub closured_vars_struct_type: &'t StructTT<'s, 't>,
  pub variability: VariabilityT,
  pub coord: CoordT<'s, 't>,
}
/*
case class AddressibleClosureVariableT(
  name: IVarNameT,
  closuredVarsStructType: StructTT,
  variability: VariabilityT,
  coord: CoordT
) extends IVariableT {
  vpass()
}
*/
// mig: struct ReferenceClosureVariableT
// mig: impl ReferenceClosureVariableT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceClosureVariableT<'s, 't>
where 's: 't,
{
  pub name: IVarNameT<'s, 't>,
  pub closured_vars_struct_type: &'t StructTT<'s, 't>,
  pub variability: VariabilityT,
  pub coord: CoordT<'s, 't>,
}
/*
case class ReferenceClosureVariableT(
  name: IVarNameT,
  closuredVarsStructType: StructTT,
  variability: VariabilityT,
  coord: CoordT
) extends IVariableT {
*/
// mig: override fn hashCode
// (Realized by `#[derive(Hash)]` on ReferenceClosureVariableT above.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: override fn eq
// (Realized by `#[derive(PartialEq, Eq)]` on ReferenceClosureVariableT above.)
/*
override def equals(obj: Any): Boolean = vcurious();

}
*/
/*

object EnvironmentHelper {
*/

impl<'s, 't> From<AddressibleLocalVariableT<'s, 't>> for ILocalVariableT<'s, 't> {
  fn from(v: AddressibleLocalVariableT<'s, 't>) -> Self { ILocalVariableT::Addressible(v) }
  /* Guardian: disable-all */
}
impl<'s, 't> From<ReferenceLocalVariableT<'s, 't>> for ILocalVariableT<'s, 't> {
  fn from(v: ReferenceLocalVariableT<'s, 't>) -> Self { ILocalVariableT::Reference(v) }
  /* Guardian: disable-all */
}

impl<'s, 't> From<AddressibleLocalVariableT<'s, 't>> for IVariableT<'s, 't> {
  fn from(v: AddressibleLocalVariableT<'s, 't>) -> Self { IVariableT::AddressibleLocal(v) }
  /* Guardian: disable-all */
}
impl<'s, 't> From<ReferenceLocalVariableT<'s, 't>> for IVariableT<'s, 't> {
  fn from(v: ReferenceLocalVariableT<'s, 't>) -> Self { IVariableT::ReferenceLocal(v) }
  /* Guardian: disable-all */
}
impl<'s, 't> From<AddressibleClosureVariableT<'s, 't>> for IVariableT<'s, 't> {
  fn from(v: AddressibleClosureVariableT<'s, 't>) -> Self { IVariableT::AddressibleClosure(v) }
  /* Guardian: disable-all */
}
impl<'s, 't> From<ReferenceClosureVariableT<'s, 't>> for IVariableT<'s, 't> {
  fn from(v: ReferenceClosureVariableT<'s, 't>) -> Self { IVariableT::ReferenceClosure(v) }
  /* Guardian: disable-all */
}

impl<'s, 't> From<ILocalVariableT<'s, 't>> for IVariableT<'s, 't> {
  fn from(v: ILocalVariableT<'s, 't>) -> Self {
    match v {
      ILocalVariableT::Addressible(a) => IVariableT::AddressibleLocal(a),
      ILocalVariableT::Reference(r) => IVariableT::ReferenceLocal(r),
    }
  }
  /* Guardian: disable-all */
}

impl<'s, 't> TryFrom<IVariableT<'s, 't>> for ILocalVariableT<'s, 't> {
  type Error = IVariableT<'s, 't>;
  fn try_from(v: IVariableT<'s, 't>) -> Result<Self, Self::Error> {
    match v {
      IVariableT::AddressibleLocal(a) => Ok(ILocalVariableT::Addressible(a)),
      IVariableT::ReferenceLocal(r) => Ok(ILocalVariableT::Reference(r)),
      other => Err(other),
    }
  }
  /* Guardian: disable-all */
}

impl<'s, 't> TryFrom<IVariableT<'s, 't>> for AddressibleLocalVariableT<'s, 't> {
  type Error = IVariableT<'s, 't>;
  fn try_from(v: IVariableT<'s, 't>) -> Result<Self, Self::Error> {
    match v { IVariableT::AddressibleLocal(a) => Ok(a), other => Err(other) }
  }
  /* Guardian: disable-all */
}
impl<'s, 't> TryFrom<IVariableT<'s, 't>> for ReferenceLocalVariableT<'s, 't> {
  type Error = IVariableT<'s, 't>;
  fn try_from(v: IVariableT<'s, 't>) -> Result<Self, Self::Error> {
    match v { IVariableT::ReferenceLocal(r) => Ok(r), other => Err(other) }
  }
  /* Guardian: disable-all */
}
impl<'s, 't> TryFrom<IVariableT<'s, 't>> for AddressibleClosureVariableT<'s, 't> {
  type Error = IVariableT<'s, 't>;
  fn try_from(v: IVariableT<'s, 't>) -> Result<Self, Self::Error> {
    match v { IVariableT::AddressibleClosure(a) => Ok(a), other => Err(other) }
  }
  /* Guardian: disable-all */
}
impl<'s, 't> TryFrom<IVariableT<'s, 't>> for ReferenceClosureVariableT<'s, 't> {
  type Error = IVariableT<'s, 't>;
  fn try_from(v: IVariableT<'s, 't>) -> Result<Self, Self::Error> {
    match v { IVariableT::ReferenceClosure(r) => Ok(r), other => Err(other) }
  }
  /* Guardian: disable-all */
}

impl<'s, 't> TryFrom<ILocalVariableT<'s, 't>> for AddressibleLocalVariableT<'s, 't> {
  type Error = ILocalVariableT<'s, 't>;
  fn try_from(v: ILocalVariableT<'s, 't>) -> Result<Self, Self::Error> {
    match v { ILocalVariableT::Addressible(a) => Ok(a), other => Err(other) }
  }
  /* Guardian: disable-all */
}
impl<'s, 't> TryFrom<ILocalVariableT<'s, 't>> for ReferenceLocalVariableT<'s, 't> {
  type Error = ILocalVariableT<'s, 't>;
  fn try_from(v: ILocalVariableT<'s, 't>) -> Result<Self, Self::Error> {
    match v { ILocalVariableT::Reference(r) => Ok(r), other => Err(other) }
  }
  /* Guardian: disable-all */
}

// mig: fn lookup_with_name_inner
pub fn lookup_with_name_inner<'s, 't>(
  requesting_env: IEnvironmentT<'s, 't>,
  templatas: &TemplatasStoreT<'s, 't>,
  parent: IEnvironmentT<'s, 't>,
  name: INameT<'s, 't>,
  lookup_filter: &HashSet<ILookupContext>,
  get_only_nearest: bool,
) -> Vec<ITemplataT<'s, 't>>
where 's: 't,
{
  panic!("Unimplemented: lookup_with_name_inner");
}
/*
  def lookupWithNameInner(
    requestingEnv: IEnvironmentT,
    templatas: TemplatasStore,
    parent: IEnvironmentT,

    name: INameT,
    lookupFilter: Set[ILookupContext],
    getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    val result = templatas.lookupWithNameInner(requestingEnv, name, lookupFilter)
    if (result.nonEmpty && getOnlyNearest) {
      result.toArray
    } else {
      result.toArray ++ parent.lookupWithNameInner(name, lookupFilter, getOnlyNearest)
    }
  }

*/
// mig: fn lookup_with_imprecise_name_inner
pub fn lookup_with_imprecise_name_inner<'s, 't>(
  requesting_env: IEnvironmentT<'s, 't>,
  templatas: &TemplatasStoreT<'s, 't>,
  parent: IEnvironmentT<'s, 't>,
  name: IImpreciseNameS<'s>,
  lookup_filter: &HashSet<ILookupContext>,
  get_only_nearest: bool,
) -> Vec<ITemplataT<'s, 't>>
where 's: 't,
{
  let result = templatas.lookup_with_imprecise_name_inner(requesting_env, name, lookup_filter);
  if !result.is_empty() && get_only_nearest {
    result
  } else {
    let mut combined = result;
    combined.extend(parent.lookup_with_imprecise_name_inner(name, lookup_filter.clone(), get_only_nearest));
    combined
  }
}
/*
  def lookupWithImpreciseNameInner(
    requestingEnv: IEnvironmentT,
    templatas: TemplatasStore,
    parent: IEnvironmentT,

    name: IImpreciseNameS,
    lookupFilter: Set[ILookupContext],
    getOnlyNearest: Boolean):
  Array[ITemplataT[ITemplataType]] = {
    val result = templatas.lookupWithImpreciseNameInner(requestingEnv, name, lookupFilter)
    if (result.nonEmpty && getOnlyNearest) {
      result
    } else {
      result ++ parent.lookupWithImpreciseNameInner(name, lookupFilter, getOnlyNearest)
    }
  }
}
*/

// Builders — see environment.rs for the Package/Citizen/Export/Extern/General
// builders; these 4 finish out the set for the function-env family.

/// Temporary state (see @TFITCX)
pub struct BuildingFunctionEnvironmentWithClosuredsBuilder<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
  pub function: &'s FunctionA<'s>,
  pub variables: Vec<IVariableT<'s, 't>>,
  pub is_root_compiling_denizen: bool,
}

impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsBuilder<'s, 't>
where 's: 't,
{
  pub fn build_in(
    self,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't> {
    let templatas = self.templatas_builder.build_in(interner);
    let variables = interner.alloc_slice_from_vec(self.variables);
    interner.alloc(BuildingFunctionEnvironmentWithClosuredsT {
      global_env: self.global_env,
      parent_env: self.parent_env,
      id: self.id,
      templatas,
      function: self.function,
      variables,
      is_root_compiling_denizen: self.is_root_compiling_denizen,
    })
  }
  /* Guardian: disable-all */
}

/// Temporary state (see @TFITCX)
pub struct BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsBuilder<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub template_args: Vec<ITemplataT<'s, 't>>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
  pub function: &'s FunctionA<'s>,
  pub variables: Vec<IVariableT<'s, 't>>,
  pub is_root_compiling_denizen: bool,
  pub default_region: RegionT,
}

impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsBuilder<'s, 't>
where 's: 't,
{
  pub fn build_in(
    self,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> {
    let templatas = self.templatas_builder.build_in(interner);
    let template_args = interner.alloc_slice_from_vec(self.template_args);
    let variables = interner.alloc_slice_from_vec(self.variables);
    interner.alloc(BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT {
      global_env: self.global_env,
      parent_env: self.parent_env,
      id: self.id,
      template_args,
      templatas,
      function: self.function,
      variables,
      is_root_compiling_denizen: self.is_root_compiling_denizen,
      default_region: self.default_region,
    })
  }
  /* Guardian: disable-all */
}

/// Temporary state (see @TFITCX)
pub struct FunctionEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
  pub function: &'s FunctionA<'s>,
  pub maybe_return_type: Option<CoordT<'s, 't>>,
  pub closured_locals: Vec<IVariableT<'s, 't>>,
  pub is_root_compiling_denizen: bool,
  pub default_region: RegionT,
}

impl<'s, 't> FunctionEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub fn build_in(
    self,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t FunctionEnvironmentT<'s, 't> {
    let templatas = self.templatas_builder.build_in(interner);
    let closured_locals = interner.alloc_slice_from_vec(self.closured_locals);
    interner.alloc(FunctionEnvironmentT {
      global_env: self.global_env,
      parent_env: self.parent_env,
      template_id: self.template_id,
      id: self.id,
      templatas,
      function: self.function,
      maybe_return_type: self.maybe_return_type,
      closured_locals,
      is_root_compiling_denizen: self.is_root_compiling_denizen,
      default_region: self.default_region,
    })
  }
  /* Guardian: disable-all */

  pub fn snapshot(
    &self,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t FunctionEnvironmentT<'s, 't> {
    let templatas = self.templatas_builder.snapshot(interner);
    let closured_locals = interner.alloc_slice_from_vec(self.closured_locals.clone());
    interner.alloc(FunctionEnvironmentT {
      global_env: self.global_env,
      parent_env: self.parent_env,
      template_id: self.template_id,
      id: self.id,
      templatas,
      function: self.function,
      maybe_return_type: self.maybe_return_type,
      closured_locals,
      is_root_compiling_denizen: self.is_root_compiling_denizen,
      default_region: self.default_region,
    })
  }
  /* Guardian: disable-all */
}

/// Temporary state (see @TFITCX)
pub struct NodeEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub parent_function_env: &'t FunctionEnvironmentT<'s, 't>,
  pub parent_node_env: Option<&'t NodeEnvironmentT<'s, 't>>,
  pub node: &'s IExpressionSE<'s>,
  pub life: LocationInFunctionEnvironmentT<'s>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
  pub declared_locals: Vec<IVariableT<'s, 't>>,
  pub unstackified_locals: Vec<IVarNameT<'s, 't>>,
  pub restackified_locals: Vec<IVarNameT<'s, 't>>,
  pub default_region: RegionT,
}

impl<'s, 't> NodeEnvironmentBuilder<'s, 't>
where 's: 't,
{
  pub fn build_in(
    self,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t NodeEnvironmentT<'s, 't> {
    let templatas = self.templatas_builder.build_in(interner);
    let declared_locals = interner.alloc_slice_from_vec(self.declared_locals);
    let unstackified_locals = interner.alloc_slice_from_vec(self.unstackified_locals);
    let restackified_locals = interner.alloc_slice_from_vec(self.restackified_locals);
    interner.alloc(NodeEnvironmentT {
      parent_function_env: self.parent_function_env,
      parent_node_env: self.parent_node_env,
      node: self.node,
      life: self.life,
      templatas,
      declared_locals,
      unstackified_locals,
      restackified_locals,
      default_region: self.default_region,
    })
  }
  /* Guardian: disable-all */

  pub fn snapshot(
    &self,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t NodeEnvironmentT<'s, 't> {
    let templatas = self.templatas_builder.snapshot(interner);
    let declared_locals = interner.alloc_slice_from_vec(self.declared_locals.clone());
    let unstackified_locals = interner.alloc_slice_from_vec(self.unstackified_locals.clone());
    let restackified_locals = interner.alloc_slice_from_vec(self.restackified_locals.clone());
    interner.alloc(NodeEnvironmentT {
      parent_function_env: self.parent_function_env,
      parent_node_env: self.parent_node_env,
      node: self.node,
      life: self.life.clone(),
      templatas,
      declared_locals,
      unstackified_locals,
      restackified_locals,
      default_region: self.default_region,
    })
  }
  /* Guardian: disable-all */
}