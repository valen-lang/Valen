use indexmap::IndexMap;
use crate::typing::ast::ast::LocT;
use crate::typing::borrow_checker::ast_g::GroupStep;
use crate::typing::names::names::IVarNameT;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum RefKey<'s, 't> {
  Named(IVarNameT<'s, 't>),
  Held(u32),
}

// A subtree for a group as the containing function knows it. This grows over time as the function learns about new groups.
#[derive(Debug, Clone)] // Has clone because of if-statements
pub struct GroupSubtree<'s, 't> {
  pub locals: IndexMap<RefKey<'s, 't>, LocalEntry<'t>>,

  // The locals pointing at an ellipsis inside a certain group.
  // For example, in this function:
  //     func foo(vec &Vec<Ship>) {
  //       first_ref &Ship in vec... = vec[0];
  //       vec.append(Ship(42));
  //       print(first_ref.hp);
  //     }
  // At the start we'll just have GroupSubtree{[{vec,None}],[],[]}.
  // After `first_ref =` we'll have GroupSubtree{[{vec,None}],[{first_ref,None}],[]}
  // After `vec.append` we'll have GroupSubtree{[{vec,None}],[{first_ref,Some(...)}],[]}
  //
  // There's no such thing as a child of an ellipsis; doing `&x.hp` on a `&Ship in g...` produces a `&i32 in g...`.
  pub locals_in_ellipsis: IndexMap<RefKey<'s, 't>, LocalEntry<'t>>,

  pub name_to_child: IndexMap<GroupStep<'s, 't>, GroupSubtree<'s, 't>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct LocalEntry<'t> {
  pub invalidated_by: Option<LocT<'t>>,
}
