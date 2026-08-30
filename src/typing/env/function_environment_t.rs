use crate::postparsing::ast::FunctionS;
use crate::postparsing::expressions::IExpressionSE;
use crate::postparsing::names::IImpreciseNameS;
use crate::scout_arena::ScoutArena;
use crate::typing::ast::ast::LocT;
use crate::typing::env::environment::{
  GlobalEnvironmentT, IEnvironmentT, IInDenizenEnvironmentT, ILookupContext,
  TemplatasStoreBuilder, TemplatasStoreT,
};
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::names::names::{INameT, IVarNameT, IdT};
use crate::typing::templata::templata::{FunctionTemplataT, ITemplataT};
use crate::typing::types::types::{KindT, RegionT, StructTT};
use crate::typing::typing_interner::TypingInterner;
use crate::utils::fx::HashSet;
use crate::utils::fx::IndexSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::ptr::eq;
use std::ptr::hash;

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BuildingFunctionEnvironmentWithClosuredsT<'s, 't>
where
  's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas: &'t TemplatasStoreT<'s, 't>,
  pub function: &'s FunctionS<'s>,
  pub variables: &'t [IVariableT<'s, 't>],
  pub is_root_compiling_denizen: bool,
}

impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsT<'s, 't>
where
  's: 't,
{
  pub fn templata(&'t self) -> FunctionTemplataT<'s, 't> {
    FunctionTemplataT { outer_env: self.parent_env, function_template_id: &self.id }
  }
}

impl<'s, 't> Hash for BuildingFunctionEnvironmentWithClosuredsT<'s, 't>
where
  's: 't,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}

impl<'s, 't> PartialEq for BuildingFunctionEnvironmentWithClosuredsT<'s, 't>
where
  's: 't,
{
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}
impl<'s, 't> Eq for BuildingFunctionEnvironmentWithClosuredsT<'s, 't> where 's: 't {}

impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsT<'s, 't>
where
  's: 't,
{
  pub fn root_compiling_denizen_env(&'t self) -> IInDenizenEnvironmentT<'s, 't> {
    panic!("Unimplemented: root_compiling_denizen_env");
    // if (isRootCompilingDenizen) {
    //   this
    // } else {
    //   parentEnv match {
    //     case PackageEnvironmentT(_, _, _) => vwat()
    //     case _ => {
    //       parentEnv match {
    //         case parentInDenizenEnv : IInDenizenEnvironmentT => {
    //           parentInDenizenEnv.rootCompilingDenizenEnv
    //         }
    //         case _ => vwat()
    //       }
    //     }
    //   }
    // }
  }
}

impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsT<'s, 't>
where
  's: 't,
{
  pub fn lookup_with_name_inner(
    &'t self,
    name: INameT<'s, 't>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
  ) -> Vec<ITemplataT<'s, 't>> {
    panic!("Unimplemented: lookup_with_name_inner");
    // EnvironmentHelper.lookupWithNameInner(
    //   this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
  }

  pub fn lookup_with_imprecise_name_inner(
    &'t self,
    name: IImpreciseNameS<'s>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
    interner: &TypingInterner<'s, 't>,
  ) -> Vec<ITemplataT<'s, 't>> {
    lookup_with_imprecise_name_inner(
      IEnvironmentT::BuildingWithClosureds(self),
      &self.templatas,
      self.parent_env,
      name,
      lookup_filter,
      get_only_nearest,
      interner,
    )
  }
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>
where
  's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub template_args: &'t [ITemplataT<'s, 't>],
  pub templatas: &'t TemplatasStoreT<'s, 't>,
  pub function: &'s FunctionS<'s>,
  pub variables: &'t [IVariableT<'s, 't>],
  pub is_root_compiling_denizen: bool,
  pub default_region: RegionT,
}

impl<'s, 't> Hash for BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>
where
  's: 't,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}

impl<'s, 't> PartialEq for BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>
where
  's: 't,
{
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}
impl<'s, 't> Eq for BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> where 's: 't {}

impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>
where
  's: 't,
{
  pub fn root_compiling_denizen_env(&'t self) -> IInDenizenEnvironmentT<'s, 't> {
    panic!("Unimplemented: root_compiling_denizen_env");
    // if (isRootCompilingDenizen) {
    //   this
    // } else {
    //   parentEnv match {
    //     case PackageEnvironmentT(_, _, _) => vwat()
    //     case _ => {
    //       parentEnv match {
    //         case parentInDenizenEnv : IInDenizenEnvironmentT => {
    //           parentInDenizenEnv.rootCompilingDenizenEnv
    //         }
    //         case _ => vwat()
    //       }
    //     }
    //   }
    // }
  }
}

impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>
where
  's: 't,
{
  pub fn lookup_with_name_inner(
    &'t self,
    name: INameT<'s, 't>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
  ) -> Vec<ITemplataT<'s, 't>> {
    panic!("Unimplemented: lookup_with_name_inner");
    // EnvironmentHelper.lookupWithNameInner(
    //   this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
  }

  pub fn lookup_with_imprecise_name_inner(
    &'t self,
    name: IImpreciseNameS<'s>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
    interner: &TypingInterner<'s, 't>,
  ) -> Vec<ITemplataT<'s, 't>> {
    // EnvironmentHelper.lookupWithImpreciseNameInner(this, templatas, parentEnv, name, lookupFilter, getOnlyNearest)
    lookup_with_imprecise_name_inner(
      IEnvironmentT::BuildingWithClosuredsAndTemplateArgs(self),
      &self.templatas,
      self.parent_env,
      name,
      lookup_filter,
      get_only_nearest,
      interner,
    )
  }
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct NodeEnvironmentT<'s, 't>
where
  's: 't,
{
  pub parent_function_env: &'t FunctionEnvironmentT<'s, 't>,
  pub parent_node_env: Option<&'t NodeEnvironmentT<'s, 't>>,
  pub node: &'s IExpressionSE<'s>,
  pub loct: LocT<'t>,
  pub templatas: &'t TemplatasStoreT<'s, 't>,
  pub declared_locals: &'t [IVariableT<'s, 't>],
  pub unstackified_locals: &'t [IVarNameT<'s, 't>],
  pub restackified_locals: &'t [IVarNameT<'s, 't>],
  pub default_region: RegionT,
}

impl<'s, 't> Hash for NodeEnvironmentT<'s, 't>
where
  's: 't,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.parent_function_env.id.hash(state);
    self.loct.hash(state);
  }
}

impl<'s, 't> PartialEq for NodeEnvironmentT<'s, 't>
where
  's: 't,
{
  fn eq(&self, other: &Self) -> bool {
    self.parent_function_env.id == other.parent_function_env.id && self.loct == other.loct
  }
}
impl<'s, 't> Eq for NodeEnvironmentT<'s, 't> where 's: 't {}

impl<'s, 't> NodeEnvironmentT<'s, 't>
where
  's: 't,
{
  pub fn root_compiling_denizen_env(&'t self) -> IInDenizenEnvironmentT<'s, 't> {
    panic!("Unimplemented: root_compiling_denizen_env");
    // parentEnv.rootCompilingDenizenEnv
  }
}

impl<'s, 't> NodeEnvironmentT<'s, 't>
where
  's: 't,
{
  pub fn id(&self) -> IdT<'s, 't> {
    self.parent_function_env.id
  }

  pub fn function(&self) -> &'s FunctionS<'s> {
    panic!("Unimplemented: function");
    // parentFunctionEnv.function
  }

  pub fn lookup_with_name_inner(
    &'t self,
    name: INameT<'s, 't>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
    interner: &TypingInterner<'s, 't>,
  ) -> Vec<ITemplataT<'s, 't>> {
    let parent: IEnvironmentT<'s, 't> = match self.parent_node_env {
      Some(p) => IEnvironmentT::Node(p),
      None => IEnvironmentT::Function(self.parent_function_env),
    };
    lookup_with_name_inner(
      IEnvironmentT::Node(self),
      &self.templatas,
      parent,
      name,
      lookup_filter,
      get_only_nearest,
      interner,
    )
  }

  pub fn lookup_with_imprecise_name_inner(
    &'t self,
    name: IImpreciseNameS<'s>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
    interner: &TypingInterner<'s, 't>,
  ) -> Vec<ITemplataT<'s, 't>> {
    let parent: IEnvironmentT<'s, 't> = match self.parent_node_env {
      Some(p) => IEnvironmentT::Node(p),
      None => IEnvironmentT::Function(self.parent_function_env),
    };
    lookup_with_imprecise_name_inner(
      IEnvironmentT::Node(self),
      &self.templatas,
      parent,
      name,
      lookup_filter,
      get_only_nearest,
      interner,
    )
  }

  pub fn global_env(&self) -> &'t GlobalEnvironmentT<'s, 't> {
    self.parent_function_env.global_env
  }

  pub fn parent_env(&self) -> IInDenizenEnvironmentT<'s, 't> {
    panic!("Unimplemented: parent_env");
    // parentNodeEnv.getOrElse(parentFunctionEnv)
  }

  pub fn get_variable(
    &self,
    name: IImpreciseNameS<'s>,
  ) -> Option<IVariableT<'s, 't>> {
    match self.declared_locals.iter().find(|v| v.name().imprecise_name() == Some(name)) {
      Some(v) => Some(*v),
      None => match self.parent_node_env {
        Some(p) => p.get_variable(name),
        None => self
          .parent_function_env
          .closured_locals
          .iter()
          .find(|v| v.name().imprecise_name() == Some(name))
          .copied(),
      },
    }
  }

  pub fn get_all_locals(&self) -> Vec<&'t LocalVariable<'s, 't>> {
    panic!("Unimplemented: get_all_locals");
    // declaredLocals.collect({ case i : ILocalVariableT => i })
  }

  pub fn get_all_unstackified_locals(&self) -> Vec<IVarNameT<'s, 't>> {
    self.unstackified_locals.to_vec()
  }

  pub fn add_variables(&self, new_vars: &[IVariableT<'s, 't>]) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: add_variables");
    // NodeEnvironmentT(parentFunctionEnv, parentNodeEnv, node, life, templatas,
    //   declaredLocals ++ newVars, unstackifiedLocals, restackifiedLocals, defaultRegion)
  }

  pub fn add_variable(&self, new_var: IVariableT<'s, 't>) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: add_variable");
    // NodeEnvironmentT(parentFunctionEnv, parentNodeEnv, node, life, templatas,
    //   declaredLocals :+ newVar, unstackifiedLocals, restackifiedLocals, defaultRegion)
  }

  pub fn get_all_restackified_locals(&self) -> Vec<IVarNameT<'s, 't>> {
    panic!("Unimplemented: get_all_restackified_locals");
    // restackifiedLocals.toVector
  }

  pub fn mark_local_unstackified(
    &self,
    new_unstackified: IVarNameT<'s, 't>,
  ) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: mark_local_unstackified");
    // vassert(getAllLocals().exists(_.name == newUnstackified))
    // vassert(!getAllUnstackifiedLocals().contains(newUnstackified))
    // if (getAllRestackifiedLocals().contains(newUnstackified)) {
    //   NodeEnvironmentT(..., declaredLocals, unstackifiedLocals, restackifiedLocals - newUnstackified, defaultRegion)
    // } else {
    //   NodeEnvironmentT(..., declaredLocals, unstackifiedLocals + newUnstackified, restackifiedLocals, defaultRegion)
    // }
  }

  pub fn mark_local_restackified(
    &self,
    new_restackified: IVarNameT<'s, 't>,
  ) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: mark_local_restackified");
    // vassert(getAllLocals().exists(_.name == newRestackified))
    // vassert(!getAllRestackifiedLocals().contains(newRestackified))
    // if (getAllUnstackifiedLocals().contains(newRestackified)) {
    //   NodeEnvironmentT(..., declaredLocals, unstackifiedLocals - newRestackified, restackifiedLocals, defaultRegion)
    // } else {
    //   NodeEnvironmentT(..., declaredLocals, unstackifiedLocals, restackifiedLocals + newRestackified, defaultRegion)
    // }
  }

  pub fn get_effects_since(
    &self,
    earlier_node_env: &NodeEnvironmentT<'s, 't>,
  ) -> (IndexSet<IVarNameT<'s, 't>>, IndexSet<IVarNameT<'s, 't>>) {
    assert!(eq(self.parent_function_env, earlier_node_env.parent_function_env));
    let earlier_node_env_declared_locals: HashSet<IVarNameT<'s, 't>> =
      earlier_node_env.declared_locals.iter().map(|v| v.name()).collect();
    let earlier_node_env_unstackified: HashSet<IVarNameT<'s, 't>> =
      earlier_node_env.unstackified_locals.iter().copied().collect();
    let earlier_node_env_live_locals: HashSet<IVarNameT<'s, 't>> = earlier_node_env_declared_locals
      .difference(&earlier_node_env_unstackified)
      .copied()
      .collect();
    let live_locals_introduced_since_earlier: HashSet<IVarNameT<'s, 't>> = self
      .declared_locals
      .iter()
      .map(|v| v.name())
      .filter(|x| !earlier_node_env_live_locals.contains(x))
      .collect();
    let unstackified_ancestor_locals: IndexSet<IVarNameT<'s, 't>> = self
      .unstackified_locals
      .iter()
      .copied()
      .filter(|x| !live_locals_introduced_since_earlier.contains(x))
      .collect();
    let restackified_ancestor_locals: IndexSet<IVarNameT<'s, 't>> = self
      .restackified_locals
      .iter()
      .copied()
      .filter(|x| !live_locals_introduced_since_earlier.contains(x))
      .collect();
    (unstackified_ancestor_locals, restackified_ancestor_locals)
  }

  pub fn get_live_variables_introduced_since(
    &self,
    since_nenv: &NodeEnvironmentT<'s, 't>,
  ) -> Vec<&'t LocalVariable<'s, 't>> {
    let locals_as_of_then: Vec<&'t LocalVariable<'s, 't>> = since_nenv
      .declared_locals
      .iter()
      .filter_map(|v| match v {
        IVariableT::Local(r) => Some(*r),
        _ => None,
      })
      .collect();
    let locals_as_of_now: Vec<&'t LocalVariable<'s, 't>> = self
      .declared_locals
      .iter()
      .filter_map(|v| match v {
        IVariableT::Local(r) => Some(*r),
        _ => None,
      })
      .collect();

    assert!(locals_as_of_now.starts_with(&locals_as_of_then));
    let locals_declared_since_then = &locals_as_of_now[locals_as_of_then.len()..];
    assert!(locals_declared_since_then.len() == locals_as_of_now.len() - locals_as_of_then.len());

    locals_declared_since_then
      .iter()
      .filter(|x| !self.unstackified_locals.contains(&x.name))
      .copied()
      .collect()
  }

  pub fn make_child(
    &'t self,
    interner: &TypingInterner<'s, 't>,
    node: &'s IExpressionSE<'s>,
    maybe_new_default_region: Option<RegionT>,
  ) -> &'t NodeEnvironmentT<'s, 't> {
    let empty_templatas =
      TemplatasStoreBuilder::new(&self.parent_function_env.id).build_in(interner);
    interner.alloc(NodeEnvironmentT {
      parent_function_env: self.parent_function_env,
      parent_node_env: Some(self),
      node,
      loct: self.loct.clone(),
      templatas: empty_templatas,
      declared_locals: self.declared_locals, // See WTHPFE.
      unstackified_locals: self.unstackified_locals, // See WTHPFE
      restackified_locals: self.restackified_locals,
      default_region: maybe_new_default_region.unwrap_or(self.default_region), // See WTHPFE.
    })
  }

  pub fn add_entry(
    &self,
    interner: &TypingInterner<'s, 't>,
    name: INameT<'s, 't>,
    entry: IEnvEntryT<'s, 't>,
  ) -> &'t NodeEnvironmentT<'s, 't> {
    panic!("Unimplemented: add_entry");
    // NodeEnvironmentT(
    //   parentFunctionEnv,
    //   parentNodeEnv,
    //   node,
    //   life,
    //   templatas.addEntry(interner, name, entry),
    //   declaredLocals,
    //   unstackifiedLocals,
    //   restackifiedLocals,
    //   defaultRegion)
  }

  pub fn add_entries(
    &self,
    interner: &TypingInterner<'s, 't>,
    scout_arena: &ScoutArena<'s>,
    new_entries: &[(INameT<'s, 't>, IEnvEntryT<'s, 't>)],
  ) -> &'t NodeEnvironmentT<'s, 't> {
    interner.alloc(NodeEnvironmentT {
      parent_function_env: self.parent_function_env,
      parent_node_env: self.parent_node_env,
      node: self.node,
      loct: self.loct,
      templatas: interner.alloc(self.templatas.add_entries(
        interner,
        scout_arena,
        new_entries.to_vec(),
      )),
      declared_locals: self.declared_locals,
      unstackified_locals: self.unstackified_locals,
      restackified_locals: self.restackified_locals,
      default_region: self.default_region,
    })
  }

  pub fn nearest_block_env(
    &'t self,
  ) -> Option<(&'t NodeEnvironmentT<'s, 't>, &'s IExpressionSE<'s>)> {
    match self.node {
      IExpressionSE::Block(_) => Some((self, self.node)),
      _ => self.parent_node_env.and_then(|p| p.nearest_block_env()),
    }
  }

  pub fn nearest_loop_env(
    &'t self,
  ) -> Option<(&'t NodeEnvironmentT<'s, 't>, &'s IExpressionSE<'s>)> {
    match self.node {
      IExpressionSE::While(_) => Some((self, self.node)),
      IExpressionSE::Map(_) => Some((self, self.node)),
      _ => self.parent_node_env.and_then(|p| p.nearest_loop_env()),
    }
  }
}

/// Temporary state (see @TFITCX)
pub struct NodeEnvironmentBox<'s, 't>
where
  's: 't,
{
  pub parent_function_env: &'t FunctionEnvironmentT<'s, 't>,
  pub parent_node_env: Option<&'t NodeEnvironmentT<'s, 't>>,
  pub node: &'s IExpressionSE<'s>,
  pub loct: LocT<'t>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
  pub declared_locals: Vec<IVariableT<'s, 't>>,
  pub unstackified_locals: Vec<IVarNameT<'s, 't>>,
  pub restackified_locals: Vec<IVarNameT<'s, 't>>,
  pub default_region: RegionT,
}

impl<'s, 't> NodeEnvironmentBox<'s, 't>
where
  's: 't,
{
  pub fn new(node_env: &'t NodeEnvironmentT<'s, 't>) -> Self {
    NodeEnvironmentBox {
      parent_function_env: node_env.parent_function_env,
      parent_node_env: node_env.parent_node_env,
      node: node_env.node,
      loct: node_env.loct.clone(),
      templatas_builder: TemplatasStoreBuilder::from_store(&node_env.templatas),
      declared_locals: node_env.declared_locals.to_vec(),
      unstackified_locals: node_env.unstackified_locals.to_vec(),
      restackified_locals: node_env.restackified_locals.to_vec(),
      default_region: node_env.default_region,
    }
  }

  pub fn snapshot(&self, interner: &TypingInterner<'s, 't>) -> &'t NodeEnvironmentT<'s, 't> {
    let templatas = self.templatas_builder.snapshot(interner);
    let declared_locals = interner.alloc_slice_from_vec(self.declared_locals.clone());
    let unstackified_locals = interner.alloc_slice_from_vec(self.unstackified_locals.clone());
    let restackified_locals = interner.alloc_slice_from_vec(self.restackified_locals.clone());
    interner.alloc(NodeEnvironmentT {
      parent_function_env: self.parent_function_env,
      parent_node_env: self.parent_node_env,
      node: self.node,
      loct: self.loct.clone(),
      templatas,
      declared_locals,
      unstackified_locals,
      restackified_locals,
      default_region: self.default_region,
    })
  }

  pub fn default_region(&self) -> RegionT {
    self.default_region
  }

  pub fn id(&self) -> IdT<'s, 't> {
    self.parent_function_env.id
  }

  pub fn node(&self) -> &'s IExpressionSE<'s> {
    panic!("Unimplemented: node");
    // nodeEnvironment.node
  }

  pub fn maybe_return_type(&self) -> Option<KindT<'s, 't>> {
    self.parent_function_env.maybe_return_type
  }

  pub fn global_env(&self) -> &'t GlobalEnvironmentT<'s, 't> {
    self.parent_function_env.global_env
  }

  pub fn declared_locals(&self) -> &[IVariableT<'s, 't>] {
    &self.declared_locals
  }

  pub fn unstackifieds(&self) -> &[IVarNameT<'s, 't>] {
    &self.unstackified_locals
  }

  pub fn function(&self) -> &'s FunctionS<'s> {
    panic!("Unimplemented: function");
    // nodeEnvironment.function
  }

  pub fn function_environment(&self) -> &'t FunctionEnvironmentT<'s, 't> {
    self.parent_function_env
  }

  pub fn add_variable(&mut self, new_var: IVariableT<'s, 't>) {
    self.declared_locals.push(new_var);
  }

  pub fn mark_local_unstackified(&mut self, new_unstackified: IVarNameT<'s, 't>) {
    assert!(self.get_all_locals().iter().any(|l| l.name == new_unstackified));
    assert!(!self.unstackified_locals.contains(&new_unstackified));

    if self.restackified_locals.contains(&new_unstackified) {
      // It was a restackified local, so don't mark it as unstackified, just undo the
      // restackification.
      // Even if the local belongs to a parent env, we still mark it unstackified here, see UCRTVPE.
      self.restackified_locals.retain(|x| *x != new_unstackified);
    } else {
      // Even if the local belongs to a parent env, we still mark it unstackified here, see UCRTVPE.
      self.unstackified_locals.push(new_unstackified);
    }
  }

  pub fn mark_local_restackified(&mut self, new_restackified: IVarNameT<'s, 't>) {
    assert!(self.get_all_locals().iter().any(|l| l.name == new_restackified));
    assert!(!self.restackified_locals.contains(&new_restackified));
    if self.unstackified_locals.contains(&new_restackified) {
      // It was an unstackified local, so don't mark it as restackified, just undo the
      // unstackification.
      // Even if the local belongs to a parent env, we still mark it restackified here, see UCRTVPE.
      self.unstackified_locals.retain(|x| *x != new_restackified);
    } else {
      // Even if the local belongs to a parent env, we still mark it restackified here, see UCRTVPE.
      self.restackified_locals.push(new_restackified);
    }
  }

  // AFTERM: remove the needless snapshot — transcribe the inner's `def getVariable`
  // body directly off the Box's fields (declared_locals / parent_node_env /
  // parent_function_env.closured_locals), drop the interner parameter, and update
  // call sites. `get_all_locals` / `get_all_unstackified_locals` below show the
  // same shape.
  pub fn get_variable(
    &self,
    name: IImpreciseNameS<'s>,
    interner: &TypingInterner<'s, 't>,
  ) -> Option<IVariableT<'s, 't>> {
    self.snapshot(interner).get_variable(name)
  }

  pub fn get_all_locals(&self) -> Vec<&'t LocalVariable<'s, 't>> {
    self
      .declared_locals
      .iter()
      .filter_map(|v| match v {
        IVariableT::Local(a) => Some(*a),
        IVariableT::Capture(_) => None,
      })
      .collect()
  }

  pub fn get_all_unstackified_locals(&self) -> Vec<IVarNameT<'s, 't>> {
    self.unstackified_locals.clone()
  }

  pub fn lookup_nearest_with_imprecise_name(
    &self,
    name_s: IImpreciseNameS<'s>,
    lookup_filter: &HashSet<ILookupContext>,
    interner: &TypingInterner<'s, 't>,
  ) -> Option<ITemplataT<'s, 't>> {
    let node_env = self.snapshot(interner);
    IEnvironmentT::Node(node_env).lookup_nearest_with_imprecise_name(
      name_s,
      lookup_filter.clone(),
      interner,
    )
  }

  pub fn lookup_nearest_with_name(
    &self,
    _name_s: INameT<'s, 't>,
    _lookup_filter: &HashSet<ILookupContext>,
  ) -> Option<ITemplataT<'s, 't>> {
    panic!("Unimplemented: lookup_nearest_with_name");
    // nodeEnvironment.lookupNearestWithName(nameS, lookupFilter)
  }

  pub fn lookup_all_with_imprecise_name(
    &self,
    name_s: IImpreciseNameS<'s>,
    lookup_filter: &HashSet<ILookupContext>,
    interner: &TypingInterner<'s, 't>,
  ) -> Vec<ITemplataT<'s, 't>> {
    let node_env = self.snapshot(interner);
    IEnvironmentT::Node(node_env).lookup_all_with_imprecise_name(
      name_s,
      lookup_filter.clone(),
      interner,
    )
  }

  pub fn lookup_all_with_name(
    &self,
    _name_s: INameT<'s, 't>,
    _lookup_filter: &HashSet<ILookupContext>,
  ) -> Vec<ITemplataT<'s, 't>> {
    panic!("Unimplemented: lookup_all_with_name");
    // nodeEnvironment.lookupAllWithName(nameS, lookupFilter)
  }

  pub fn lookup_with_imprecise_name_inner(
    &self,
    _name_s: IImpreciseNameS<'s>,
    _lookup_filter: &HashSet<ILookupContext>,
    _get_only_nearest: bool,
    _interner: &TypingInterner<'s, 't>,
  ) -> Vec<ITemplataT<'s, 't>> {
    panic!("Unimplemented: lookup_with_imprecise_name_inner");
    // nodeEnvironment.lookupWithImpreciseNameInner(nameS, lookupFilter, getOnlyNearest)
  }

  pub fn lookup_with_name_inner(
    &self,
    _name_s: INameT<'s, 't>,
    _lookup_filter: &HashSet<ILookupContext>,
    _get_only_nearest: bool,
  ) -> Vec<ITemplataT<'s, 't>> {
    panic!("Unimplemented: lookup_with_name_inner");
    // nodeEnvironment.lookupWithNameInner(nameS, lookupFilter, getOnlyNearest)
  }

  pub fn make_child(
    &self,
    interner: &TypingInterner<'s, 't>,
    node: &'s IExpressionSE<'s>,
    maybe_new_default_region: Option<RegionT>,
  ) -> &'t NodeEnvironmentT<'s, 't> {
    self.snapshot(interner).make_child(interner, node, maybe_new_default_region)
  }

  pub fn add_entry(
    &mut self,
    _interner: &TypingInterner<'s, 't>,
    _name: INameT<'s, 't>,
    _entry: IEnvEntryT<'s, 't>,
  ) {
    panic!("Unimplemented: add_entry");
    // nodeEnvironment = nodeEnvironment.addEntry(interner, name, entry)
  }

  pub fn add_entries(
    &mut self,
    scout_arena: &ScoutArena<'s>,
    _interner: &TypingInterner<'s, 't>,
    new_entries: &[(INameT<'s, 't>, IEnvEntryT<'s, 't>)],
  ) {
    self.templatas_builder.add_entries(scout_arena, new_entries.to_vec());
  }

  pub fn nearest_block_env(
    &self,
    interner: &TypingInterner<'s, 't>,
  ) -> Option<(&'t NodeEnvironmentT<'s, 't>, &'s IExpressionSE<'s>)> {
    let snap = self.snapshot(interner);
    snap.nearest_block_env()
  }

  pub fn nearest_loop_env(
    &self,
    interner: &TypingInterner<'s, 't>,
  ) -> Option<(&'t NodeEnvironmentT<'s, 't>, &'s IExpressionSE<'s>)> {
    let snap = self.snapshot(interner);
    snap.nearest_loop_env()
  }
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct FunctionEnvironmentT<'s, 't>
where
  's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas: &'t TemplatasStoreT<'s, 't>,
  pub function: &'s FunctionS<'s>,
  pub maybe_return_type: Option<KindT<'s, 't>>,
  pub closured_locals: &'t [IVariableT<'s, 't>],
  pub is_root_compiling_denizen: bool,
  pub default_region: RegionT,
}

impl<'s, 't> Hash for FunctionEnvironmentT<'s, 't>
where
  's: 't,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}

impl<'s, 't> PartialEq for FunctionEnvironmentT<'s, 't>
where
  's: 't,
{
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}
impl<'s, 't> Eq for FunctionEnvironmentT<'s, 't> where 's: 't {}

impl<'s, 't> FunctionEnvironmentT<'s, 't>
where
  's: 't,
{
  pub fn root_compiling_denizen_env(&'t self) -> IInDenizenEnvironmentT<'s, 't> {
    if self.is_root_compiling_denizen {
      IInDenizenEnvironmentT::Function(self)
    } else {
      match self.parent_env {
        IEnvironmentT::Package(_) => panic!("vwat: root_compiling_denizen_env parent is Package"),
        _ => match IInDenizenEnvironmentT::try_from(self.parent_env) {
          Ok(parent_in_denizen_env) => parent_in_denizen_env.root_compiling_denizen_env(),
          Err(_) => panic!("vwat: root_compiling_denizen_env parent is not IInDenizenEnvironmentT"),
        },
      }
    }
  }
}

impl<'s, 't> FunctionEnvironmentT<'s, 't>
where
  's: 't,
{
  pub fn templata(&'t self) -> FunctionTemplataT<'s, 't> {
    FunctionTemplataT { outer_env: self.parent_env, function_template_id: &self.template_id }
  }

  pub fn add_entry(
    &self,
    interner: &TypingInterner<'s, 't>,
    name: INameT<'s, 't>,
    entry: IEnvEntryT<'s, 't>,
  ) -> &'t FunctionEnvironmentT<'s, 't> {
    panic!("Unimplemented: add_entry");
    // FunctionEnvironmentT(
    //   globalEnv,
    //   parentEnv,
    //   templateId,
    //   id,
    //   templatas.addEntry(interner, name, entry),
    //   function,
    //   maybeReturnType,
    //   closuredLocals,
    //   isRootCompilingDenizen,
    //   defaultRegion)
  }

  pub fn add_entries(
    &self,
    interner: &TypingInterner<'s, 't>,
    new_entries: &[(INameT<'s, 't>, IEnvEntryT<'s, 't>)],
  ) -> &'t FunctionEnvironmentT<'s, 't> {
    panic!("Unimplemented: add_entries");
    // FunctionEnvironmentT(globalEnv, parentEnv, templateId, id,
    //   templatas.addEntries(interner, newEntries),
    //   function, maybeReturnType, closuredLocals, isRootCompilingDenizen, defaultRegion)
  }

  pub fn lookup_with_name_inner(
    &'t self,
    name: INameT<'s, 't>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
    interner: &TypingInterner<'s, 't>,
  ) -> Vec<ITemplataT<'s, 't>> {
    lookup_with_name_inner(
      IEnvironmentT::Function(self),
      self.templatas,
      self.parent_env,
      name,
      lookup_filter,
      get_only_nearest,
      interner,
    )
  }

  pub fn lookup_with_imprecise_name_inner(
    &'t self,
    name: IImpreciseNameS<'s>,
    lookup_filter: &HashSet<ILookupContext>,
    get_only_nearest: bool,
    interner: &TypingInterner<'s, 't>,
  ) -> Vec<ITemplataT<'s, 't>> {
    lookup_with_imprecise_name_inner(
      IEnvironmentT::Function(self),
      self.templatas,
      self.parent_env,
      name,
      lookup_filter,
      get_only_nearest,
      interner,
    )
  }

  pub fn make_child_node_environment(
    &'t self,
    node: &'s IExpressionSE<'s>,
    loct: LocT<'t>,
  ) -> NodeEnvironmentBox<'s, 't> {
    // See WTHPFE, if this is a lambda, we let our blocks start with
    // locals from the parent function.
    let (declared_locals, unstackified_locals, restackified_locals) = match &self.parent_env {
      IEnvironmentT::Node(_node_env) => {
        panic!("implement: make_child_node_environment — NodeEnvironmentT parent");
        // (declaredLocals, unstackifiedLocals, restackifiedLocals)
      }
      _ => (Vec::new(), Vec::new(), Vec::new()),
    };
    NodeEnvironmentBox {
      parent_function_env: self,
      parent_node_env: None,
      node,
      loct,
      templatas_builder: TemplatasStoreBuilder::new(&self.id),
      declared_locals,
      unstackified_locals,
      restackified_locals,
      default_region: self.default_region,
    }
  }

  pub fn get_closured_declared_locals(&self) -> Vec<IVariableT<'s, 't>> {
    panic!("Unimplemented: get_closured_declared_locals");
  }
}

/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IVariableT<'s, 't>
where
  's: 't,
{
  Local(&'t LocalVariable<'s, 't>),
  Capture(&'t CapturedVariableT<'s, 't>),
}

impl<'s, 't> IVariableT<'s, 't>
where
  's: 't,
{
  pub fn name(&self) -> IVarNameT<'s, 't> {
    match self {
      IVariableT::Local(v) => v.name,
      IVariableT::Capture(v) => v.name,
    }
  }

  pub fn coord(&self) -> KindT<'s, 't> {
    panic!("Unimplemented: coord");
  }
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct LocalVariable<'s, 't>
where
  's: 't,
{
  pub name: IVarNameT<'s, 't>,
  pub tyype: KindT<'s, 't>,
}

// Identity equality per @IEOIBZ — `LocalVariable` is arena-allocated.
impl<'s, 't> PartialEq for LocalVariable<'s, 't>
where
  's: 't,
{
  fn eq(&self, other: &Self) -> bool {
    eq(self, other)
  }
}
impl<'s, 't> Eq for LocalVariable<'s, 't> where 's: 't {}
impl<'s, 't> Hash for LocalVariable<'s, 't>
where
  's: 't,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    hash(self, state)
  }
}

/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct CapturedVariableT<'s, 't>
where
  's: 't,
{
  pub name: IVarNameT<'s, 't>,
  pub closured_vars_struct_type: &'t StructTT<'s, 't>,
  pub kind: KindT<'s, 't>,
}

// Identity equality per @IEOIBZ — `CapturedVariableT` is arena-allocated.
impl<'s, 't> PartialEq for CapturedVariableT<'s, 't>
where
  's: 't,
{
  fn eq(&self, other: &Self) -> bool {
    eq(self, other)
  }
}
impl<'s, 't> Eq for CapturedVariableT<'s, 't> where 's: 't {}
impl<'s, 't> Hash for CapturedVariableT<'s, 't>
where
  's: 't,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    hash(self, state)
  }
}

impl<'s, 't> From<&'t LocalVariable<'s, 't>> for IVariableT<'s, 't> {
  fn from(v: &'t LocalVariable<'s, 't>) -> Self {
    IVariableT::Local(v)
  }
}
impl<'s, 't> From<&'t CapturedVariableT<'s, 't>> for IVariableT<'s, 't> {
  fn from(v: &'t CapturedVariableT<'s, 't>) -> Self {
    IVariableT::Capture(v)
  }
}

pub fn lookup_with_name_inner<'s, 't>(
  requesting_env: IEnvironmentT<'s, 't>,
  templatas: &TemplatasStoreT<'s, 't>,
  parent: IEnvironmentT<'s, 't>,
  name: INameT<'s, 't>,
  lookup_filter: &HashSet<ILookupContext>,
  get_only_nearest: bool,
  interner: &TypingInterner<'s, 't>,
) -> Vec<ITemplataT<'s, 't>>
where
  's: 't,
{
  let result: Vec<ITemplataT<'s, 't>> = templatas
    .lookup_with_name_inner(requesting_env, name, lookup_filter, interner)
    .into_iter()
    .collect();
  if !result.is_empty() && get_only_nearest {
    result
  } else {
    let mut combined = result;
    combined.extend(parent.lookup_with_name_inner(
      name,
      lookup_filter.clone(),
      get_only_nearest,
      interner,
    ));
    combined
  }
}

pub fn lookup_with_imprecise_name_inner<'s, 't>(
  requesting_env: IEnvironmentT<'s, 't>,
  templatas: &TemplatasStoreT<'s, 't>,
  parent: IEnvironmentT<'s, 't>,
  name: IImpreciseNameS<'s>,
  lookup_filter: &HashSet<ILookupContext>,
  get_only_nearest: bool,
  interner: &TypingInterner<'s, 't>,
) -> Vec<ITemplataT<'s, 't>>
where
  's: 't,
{
  let result =
    templatas.lookup_with_imprecise_name_inner(requesting_env, name, lookup_filter, interner);
  if !result.is_empty() && get_only_nearest {
    result
  } else {
    let mut combined = result;
    combined.extend(parent.lookup_with_imprecise_name_inner(
      name,
      lookup_filter.clone(),
      get_only_nearest,
      interner,
    ));
    combined
  }
}

// Builders — see environment.rs for the Package/Citizen/Export/Extern/General
// builders; these 4 finish out the set for the function-env family.

/// Temporary state (see @TFITCX)
pub struct BuildingFunctionEnvironmentWithClosuredsBuilder<'s, 't>
where
  's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
  pub function: &'s FunctionS<'s>,
  pub variables: Vec<IVariableT<'s, 't>>,
  pub is_root_compiling_denizen: bool,
}

impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsBuilder<'s, 't>
where
  's: 't,
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
}

/// Temporary state (see @TFITCX)
pub struct BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsBuilder<'s, 't>
where
  's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub template_args: Vec<ITemplataT<'s, 't>>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
  pub function: &'s FunctionS<'s>,
  pub variables: Vec<IVariableT<'s, 't>>,
  pub is_root_compiling_denizen: bool,
  pub default_region: RegionT,
}

impl<'s, 't> BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsBuilder<'s, 't>
where
  's: 't,
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
}

/// Temporary state (see @TFITCX)
pub struct FunctionEnvironmentBuilder<'s, 't>
where
  's: 't,
{
  pub global_env: &'t GlobalEnvironmentT<'s, 't>,
  pub parent_env: IEnvironmentT<'s, 't>,
  pub template_id: IdT<'s, 't>,
  pub id: IdT<'s, 't>,
  pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
  pub function: &'s FunctionS<'s>,
  pub maybe_return_type: Option<KindT<'s, 't>>,
  pub closured_locals: Vec<IVariableT<'s, 't>>,
  pub is_root_compiling_denizen: bool,
  pub default_region: RegionT,
}

impl<'s, 't> FunctionEnvironmentBuilder<'s, 't>
where
  's: 't,
{
  pub fn snapshot(&self, interner: &TypingInterner<'s, 't>) -> &'t FunctionEnvironmentT<'s, 't> {
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
}
