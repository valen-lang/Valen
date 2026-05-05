use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use rustdoc_types::{Crate, Id, Use, Item, ItemEnum, Type};
use crate::ResolveError::{NotFound, ResolveFatal};
use crate::{ResolveError, UId};

// TODO: optimize: All over the place we're calling .keys() and .collect()
// on some crate.index.
pub(crate) fn tuple_id(
  primitive_name_to_uid: &HashMap<String, UId>,
) -> UId {
  primitive_name_to_uid.get("tuple").unwrap().clone()
}
pub(crate) fn str_id(
  primitive_name_to_uid: &HashMap<String, UId>,
) -> UId {
  primitive_name_to_uid.get("str").unwrap().clone()
}
fn slice_id(
  primitive_name_to_uid: &HashMap<String, UId>,
) -> UId {
  primitive_name_to_uid.get("slice").unwrap().clone()
}
fn lifetime_id(
) -> UId {
  UId{ crate_name: "".to_string(), id: Id(u32::MAX) }
}
pub(crate) fn primitive_id(
  primitive_name_to_uid: &HashMap<String, UId>,
  name: &str
) -> UId {
  primitive_name_to_uid.get(name).unwrap().clone()
}

// Any other primitive can roughly be treated as a normal type
pub(crate) fn is_special_primitive(primitive_name_to_uid: &HashMap<String, UId>, id: &UId) -> bool {
  id == &tuple_id(primitive_name_to_uid) ||
      id == &str_id(primitive_name_to_uid) ||
      id == &slice_id(primitive_name_to_uid) ||
      id == &lifetime_id() ||
      is_generic(id)
}
pub(crate) fn is_primitive(
  primitive_uid_to_name: &HashMap<UId, String>,
  id: &UId
) -> bool {
  id == &lifetime_id() ||
      is_generic(id) ||
      primitive_uid_to_name.contains_key(id)
}

pub(crate) fn is_generic(id: &UId) -> bool {
  id.crate_name.starts_with("$")
}

pub(crate) fn generic_name(id: &UId) -> &str {
  assert!(is_generic(id));
  &id.crate_name[1..]
}


// Recurses.
pub fn resolve_uid(
  crates: &HashMap<String, Crate>,
  primitive_name_to_uid: &HashMap<String, UId>,
  tentative_item_id: &UId
) -> anyhow::Result<UId, ResolveError> {
  // let tentative_item_id = &tentative_path.steps.last().unwrap().id;
  let local_crate = crates.get(&tentative_item_id.crate_name).unwrap();
  match local_crate.index.get(&tentative_item_id.id) {
    None => {
      // Then this might be just referring to something in the paths.
      match local_crate.paths.get(&tentative_item_id.id) {
        None => panic!("Unknown ID: {:?}", &tentative_item_id.id),
        Some(path) => {
          let foreign_crate_name = &path.path[0];
          // let foreign_crate = crates.get(foreign_crate_name).unwrap();
          let mut current =
              extend_and_resolve_uid(crates, &primitive_name_to_uid, None, foreign_crate_name)?;
          for i in 1.. path.path.len() {
            let step = &path.path[i];
            current =
                match extend_and_resolve_uid(crates, &primitive_name_to_uid, Some(&current), step) {
                  Ok(trait_uid) => trait_uid,
                  Err(ResolveError::NotFound) => {
                    // Sometimes when we get one of these path steps, it's actually a private module
                    // such as the unwind_safe in core::panic::unwind_safe::RefUnwindSafe.
                    // The RefUnwindSafe was actually pub use'd in core::panic.
                    // If this isn't the last step, let's skip it and see if things work.
                    // TODO: document better or something? seems sketchy.
                    if i != path.path.len() - 1 {
                      continue;
                    } else {
                      return Err(ResolveError::NotFound);
                    }
                  }
                  Err(ResolveError::Unsupported(reason)) => return Err(ResolveError::Unsupported(reason)),
                  Err(ResolveError::ResolveFatal(fatal)) => return Err(ResolveError::ResolveFatal(fatal))
                };
          }
          Ok(current)
        }
      }
    }
    Some(tentative_item) => {
      // // TODO: impl
      // assert!(tentative_path.steps.last().unwrap().generic_args.is_empty());

      match &tentative_item.inner {
        ItemEnum::ExternCrate { .. } => unimplemented!(),
        ItemEnum::Use(import) => {
          // When we do an import, we actually want the module the
          // import is referring to, not the import's id.

          let local_crate = crates.get(&tentative_item_id.crate_name).unwrap();
          let import_id = import.id.as_ref().unwrap();
          match local_crate.paths.get(&import_id) {
            None => {
              resolve_uid(
                crates,
                primitive_name_to_uid,
                &UId {
                  crate_name: tentative_item_id.crate_name.clone(),
                  id: import_id.clone()
                })
            }
            Some(path) => {
              if path.path[0] == "std_detect" {
                unimplemented!();
                // return Ok(None)
              }
              if path.path[0] == "core" && path.path[1] == "macros" {
                unimplemented!();
                // return Ok(None)
              }

              // let foreign_crate_path = SimpleValType { steps: Vec::new() };
              let foreign_crate_root_name = &path.path[0];
              // TODO: if we're not addressing a foreign crate, we might need to add
              // a case here
              let foreign_crate = crates.get(foreign_crate_root_name).unwrap();
              let foreign_crate_root_module_id = UId { crate_name: foreign_crate_root_name.clone(), id: foreign_crate.root.clone() };
              // let foreign_crate_root_module = foreign_crate.index.get(&foreign_crate_root_module_id.id).unwrap();

              let mut result_uid = foreign_crate_root_module_id.clone();
              for next_foreign_crate_name in &path.path[1..] {
                let mut found_child_uids: Vec<UId> = Vec::new();
                let hay_child_uids =
                    match get_unexpanded_direct_child_uids_exclude_impl_children(
                      crates, &primitive_name_to_uid, &result_uid) {
                      Ok(x) => x,
                      Err(e) => return Err(ResolveFatal(e))
                    };
                for hay_child_id in hay_child_uids {
                  let hay_child = foreign_crate.index.get(&hay_child_id.id).unwrap();
                  if item_has_name(&hay_child, next_foreign_crate_name) {
                    found_child_uids.push(hay_child_id);
                  }
                }
                if found_child_uids.len() > 1 {
                  // Let's filter out any macros, sometimes they collide with real things,
                  // like how "vec" is both a macro and a module.
                  found_child_uids = filter_out_macro_uids(crates, found_child_uids);
                }
                if found_child_uids.len() != 1 {
                  unimplemented!();
                }
                result_uid = found_child_uids[0].clone();
              }
              // Recurse
              assert_ne!(&result_uid, tentative_item_id); // Otherwise infinite loop
              resolve_uid(crates, primitive_name_to_uid, &result_uid)
            }
          }
        }
        ItemEnum::TraitAlias(_) => unimplemented!(),
        ItemEnum::TypeAlias(type_alias) => {
          match resolve_type_uid(
            crates,
            primitive_name_to_uid,
            &tentative_item_id.crate_name,
            &type_alias.type_) {
            Ok(thing) => Ok(thing),
            Err(_) => {
              unimplemented!()
            }
          }
        },
        ItemEnum::ExternType => unimplemented!(),
        _ => Ok(tentative_item_id.clone())
      }
    }
  }
}


// This adds the given step to the end, and if it lands on an
// import or a typealias it will follow it and return the new corrected
// path.
pub(crate) fn extend_and_resolve_uid(
  crates: &HashMap<String, Crate>,
  primitive_name_to_uid: &HashMap<String, UId>,
  // This is the containing module or impl.
  previous: Option<&UId>,
  name: &str
) -> anyhow::Result<UId, ResolveError> {
  if name.len() == 0 {
    // For the life of me I can't figure out the Id for tuples, I suspect they don't have one
    // because they have a special entry in the Type enum.
    // We'll just use the empty string.
    return Ok(tuple_id(&primitive_name_to_uid));
    // return Ok(ChildKey::Normal { id: crate::tuple_id(&primitive_name_to_uid) });
  }

  match previous {
    None => {
      match name {
        "bool" | "char" | "f32" | "f64" | "f128" | "i128" | "i16" | "i32" | "i64" | "i8" | "isize" | "str" | "u128" | "u16" | "u32" | "u64" | "u8" | "usize" => {
          Ok(primitive_id(&primitive_name_to_uid, name))
          // Ok(ChildKey::Normal { id: crate::primitive_id(&primitive_name_to_uid, name) })
        }
        _ => {
          match crates.get(name) {
            None => {
              let error = format!("Couldn't find crate: {}", name);
              return Err(ResolveFatal(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error))));
            }
            Some(crate_) => {
              let root_module_id = &crate_.root;
              Ok(UId { crate_name: name.to_string(), id: root_module_id.clone() })
              // Ok(ChildKey::Normal { id: UId { crate_name: name.to_string(), id: root_module_id.clone() }})
            }
          }
        }
      }
    }
    Some(previous_last_step) => {
      let previous_container_id = &previous_last_step;
      let previous_crate_name = &previous_container_id.crate_name;
      let previous_crate = crates.get(previous_crate_name).unwrap();
      // let previous_container_item = previous_crate.index.get(&previous_container_id.id).unwrap();

      let direct_child_uids =
          match get_expanded_direct_child_uids(crates, &primitive_name_to_uid, &previous_container_id, true) {
        Ok(x) => x,
        Err(e) => return Err(e)
      };

      let mut found_items: Vec<(UId, &Item)> = Vec::new();
      for direct_child_uid in direct_child_uids {
        let direct_child_item = lookup_uid(crates, &direct_child_uid);
        if item_has_name(&direct_child_item, name) {
          found_items.push((direct_child_uid, direct_child_item));
        }
      }

      let mut new_found_items: Vec<(UId, &Item)> = Vec::new();
      for (found_child_unresolved_uid, item) in &found_items {
        let found_child_uid =
            match resolve_uid(crates, primitive_name_to_uid, &found_child_unresolved_uid) {
              Ok(found_child_id) => found_child_id,
              Err(ResolveError::NotFound) => {
                unimplemented!()
              }
              Err(ResolveError::Unsupported(reason)) => return Err(ResolveError::Unsupported(reason)),
              Err(ResolveFatal(e)) => return Err(ResolveFatal(e))
            };
        let found_child = lookup_uid(crates, &found_child_uid);
        match found_child.inner {
          ItemEnum::Use(_) => {
            println!("wat");
          }
          ItemEnum::Macro(_) => {} // skip
          ItemEnum::ProcMacro(_) => {} // skip
          ItemEnum::Primitive(_) => {} // skip
          _ => {
            new_found_items.push((found_child_uid.clone(), found_child));
          }
        }
      }
      found_items = new_found_items;
      found_items = found_items.into_iter().unique_by(|(x, _)| x.clone()).collect(); // DO NOT SUBMIT optimize

      if found_items.len() == 0 {
        return Err(NotFound)
      } else if found_items.len() > 1 {
        let error = format!("Found too many things with name: {}", name);
        eprintln!("Error: {}", error);
        for found_item in &found_items {
          eprintln!("Candidate: {:?}", found_item);
        }
        return Err(ResolveFatal(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error))));
        // return Err(
        //
        //   ResolveError::TooManyFound(
        //     found_items.into_iter()
        //         .map(|(a, b)| (a, b.clone()))
        //         .collect()));
      }
      let (found_item_uid, _found_item) = found_items.remove(0);

      let result_step = resolve_uid(crates, &primitive_name_to_uid, &found_item_uid)?;

      // let mut tentative_path = previous.clone();
      // tentative_path.steps.push(result_step);
      Ok(result_step)
    }
  }
}


// TODO: use this more
pub fn lookup_uid<'a>(crates: &'a HashMap<String, Crate>, uid: &UId) -> &'a Item {
  match crates.get(&uid.crate_name) {
    None => panic!("No crate by name: {}", uid.crate_name),
    Some(crate_) => {
      match crate_.index.get(&uid.id) {
        None => panic!("No ID {} in crate {}", uid.id.0, uid.crate_name),
        Some(thing) => thing
      }
    }
  }
}

fn resolve_type_uid(
  crates: &HashMap<String, Crate>,
  primitive_name_to_uid: &HashMap<String, UId>,
  type_crate_name: &str,
  type_: &Type
) -> anyhow::Result<UId> {
  Ok(
    match type_ {
      Type::ResolvedPath(path) => {
        // Note that this type might not really exist.
        // If we did:
        //   crates.get(type_crate_name).unwrap()
        //     .index.get(&path.id).unwrap();
        // it might panic.
        // This happens because ResolvedPath sometimes refers to an ID
        // that isn't defined in this crate.
        let tentative_uid =
            UId { crate_name: type_crate_name.to_string(), id: path.id.clone() };
        match resolve_uid(&crates, primitive_name_to_uid, &tentative_uid) {
          Ok(thing) => thing,
          Err(ResolveError::NotFound) => {
            unimplemented!();
          }
          Err(ResolveError::Unsupported(_)) => unreachable!("resolve_uid never returns Unsupported"),
          Err(ResolveError::ResolveFatal(fatal)) => return Err(fatal)
        }
      }
      Type::DynTrait(_dynTrait) => {
        println!("what");
        unimplemented!();
      }
      Type::Generic(_name) => unimplemented!(),
      Type::BorrowedRef { type_, .. } => resolve_type_uid(crates, primitive_name_to_uid, type_crate_name, type_)?,
      Type::Primitive(name) => primitive_id(primitive_name_to_uid, name),
      Type::FunctionPointer(_) => unimplemented!(),
      Type::Tuple(_inners) => tuple_id(&primitive_name_to_uid, ),
      Type::Slice(_) => unimplemented!(),
      Type::Array { .. } => unimplemented!(),
      Type::Pat { .. } => unimplemented!(),
      Type::ImplTrait(_) => unimplemented!(),
      Type::Infer => unimplemented!(),
      Type::RawPointer { .. } => unimplemented!(),
      Type::QualifiedPath { .. } => unimplemented!(),
    })
}

pub(crate) fn get_expanded_direct_child_uids(
  crates: &HashMap<String, Crate>,
  primitive_name_to_uid: &HashMap<String, UId>,
  previous_container_id: &UId,
  include_impls_children_: bool
) -> Result<Vec<UId>, ResolveError> {
  let unexpanded_direct_child_uids_without_methods: Vec<UId> =
      match get_unexpanded_direct_child_uids_exclude_impl_children(crates, &primitive_name_to_uid, &previous_container_id) {
        Ok(x) => x,
        Err(e) => return Err(ResolveFatal(e))
      };
  assert!(include_impls_children_); // false is unimplemented
  let unexpanded_direct_child_keys =
      match include_impls_children(crates, primitive_name_to_uid, unexpanded_direct_child_uids_without_methods) {
        Ok(x) => x,
        Err(e) => return Err(ResolveFatal(e))
      };
  let unexpanded_direct_child_uids = collapse_children(&unexpanded_direct_child_keys);

  let mut direct_child_uids: Vec<UId> = vec![];
  for direct_child_uid in unexpanded_direct_child_uids.clone() {
    match &lookup_uid(crates, &direct_child_uid).inner {
      ItemEnum::Use(Use { id: Some(target_module_id), is_glob: true, .. }) => {
        // We treat glob imports as if we're directly importing
        // everything matching them.
        let target_module_uid =
            resolve_uid(
              crates,
              &primitive_name_to_uid,
              &UId {
                crate_name: direct_child_uid.crate_name,
                id: target_module_id.clone()
              })?;
        direct_child_uids.append(
          &mut get_expanded_direct_child_uids(
            crates, &primitive_name_to_uid, &target_module_uid, true)?);
      }
      _ => {
        direct_child_uids.push(direct_child_uid);
      }
    }
  }
  Ok(direct_child_uids)
}

#[derive(Clone, Debug)]
pub(crate) enum ChildKey {
  Normal { id: UId },
  ImplChild { impl_id: UId, child_id: UId }
}
impl ChildKey {
  pub(crate) fn uid(&self) -> UId {
    match self {
      ChildKey::Normal { id: uid } => uid.clone(),
      ChildKey::ImplChild { child_id, .. } => child_id.clone(),
    }
  }
  fn expect_normal(&self) -> UId {
    match self {
      ChildKey::Normal { id: uid } => uid.clone(),
      ChildKey::ImplChild { .. } => panic!("expect_normal failed!"),
    }
  }
}

pub(crate) fn include_impls_children(
  crates: &HashMap<String, Crate>,
  primitive_name_to_uid: &HashMap<String, UId>,
  original_ids: Vec<UId>
) -> anyhow::Result<Vec<ChildKey>> {
  let mut result= Vec::new();
  for direct_child_id in &original_ids {
    result.push(ChildKey::Normal{ id: direct_child_id.clone() });
  }

  for original_id in original_ids {
    match &lookup_uid(crates, &original_id).inner {
      ItemEnum::Impl(_) => {
        let impl_uid = original_id;
        match get_impl_children(crates, &primitive_name_to_uid, &impl_uid) {
          Ok(Some(impl_child_uids)) => {
            for impl_child_uid in impl_child_uids {
              // println!("Found impl {:?}'s direct child {:?}", impl_uid, impl_child_uid);
              result.push(
                ChildKey::ImplChild {
                  impl_id: impl_uid.clone(),
                  child_id: impl_child_uid.clone()
                });
            }
          },
          Ok(None) => {}
          Err(ResolveError::NotFound) => unimplemented!(),
          Err(ResolveError::Unsupported(_)) => unreachable!("get_impl_children never returns Unsupported"),
          Err(ResolveFatal(e)) => return Err(e)
        }
      }
      _ => {}
    }
  }

  Ok(result)
}

pub(crate) fn collapse_children(
  original_ids: &Vec<ChildKey>
) -> Vec<UId> {
  let mut result: Vec<UId> = Vec::new();
  for direct_child_id in original_ids {
    result.push(direct_child_id.uid().clone());
  }
  // std::io::Cursor: "0:8994:8529"
  // has impls "0:2960", "0:2955", "0:2966", "0:2972", "0:2978":
  // - impl Write for Cursor<&mut [u8]>
  // - impl<A> Write for Cursor<&mut Vec<u8, A>> where A: Allocator
  // - impl<const N: usize> Write for Cursor<[u8; N]>
  // - impl<A> Write for Cursor<Box<[u8], A>> where A: Allocator
  // - impl<A> Write for Cursor<Vec<u8, A>> where A: Allocator
  // which all have method write_all "0:3610:7588"
  // So here we dedup them.
  result
      .into_iter().collect::<HashSet<_>>()
      .into_iter().collect::<Vec<_>>()
}

// Unexpanded refers to any potential glob imports.
pub(crate) fn get_unexpanded_direct_child_uids_exclude_impl_children(
  crates: &HashMap<String, Crate>,
  primitive_name_to_uid: &HashMap<String, UId>,
  container_id: &UId
) -> anyhow::Result<Vec<UId>> {
  let include_impls_children = false;
  let container_item =
      crates
          .get(&container_id.crate_name).unwrap()
          .index.get(&container_id.id).unwrap();
  match &container_item.inner {
    ItemEnum::Module(m) => {
      let mut result = Vec::new();
      for child in &m.items {
        // eprintln!("Found module's direct child {:?}", child);
        result.push(UId { crate_name: container_id.crate_name.clone(), id: child.clone() });
      }
      Ok(result)
    },
    ItemEnum::Trait(t) => {
      let mut result = Vec::new();
      for child in &t.items {
        // eprintln!("Found trait's direct child {:?}", child);
        result.push(UId { crate_name: container_id.crate_name.clone(), id: child.clone() });
      }
      Ok(result)
    },
    ItemEnum::Struct(_) | ItemEnum::Enum(_) | ItemEnum::Primitive(_) => {
      // TODO: optimize: get_concrete_impls_children is repeating get_concrete_impls's work
      let mut result: Vec<UId> = Vec::new();
      // eprintln!("Looking through things...");
      for impl_uid in get_concrete_impls(crates, primitive_name_to_uid, &container_id) {
        // eprintln!("Found concrete's direct child {:?}", impl_uid);
        result.push(impl_uid.clone());
      }
      Ok(result)
    }
    ItemEnum::Impl(impl_) => {
      let mut result = Vec::new();
      for child in &impl_.items {
        // eprintln!("Found standalone impl's direct child {:?}", container_id);
        result.push(UId { crate_name: container_id.crate_name.clone(), id: child.clone() });
      }
      Ok(result)
    }
    _ => unimplemented!()
  }
}

pub(crate) fn item_has_name(direct_child_item: &Item, name: &str) -> bool {
  match &direct_child_item.inner {
    ItemEnum::Use(import) => {
      import.name == name && !import.is_glob
    }
    // When we import e.g.
    //   std::string::String::From<&str>::from as RustStringFromStrRef
    // we need to search for the "From" impl, thats what we're doing here.
    ItemEnum::Impl(impl_) => {
      impl_.trait_.as_ref().map(|x| &x.path[..]) == Some(name)
    }
    _ => {
      direct_child_item.name.as_ref().map(|x| &x[..]) == Some(name)
    }
  }
}

fn filter_out_macro_uids(
  crates: &HashMap<String, Crate>,
  found_child_uids: Vec<UId>
) -> Vec<UId> {
  let mut narrowed_found_child_ids = Vec::new();
  for found_child_uid in &found_child_uids {
    // Note we're doing a shallow lookup here, these might refer to more imports
    // or type aliases. We could resolve them fully in the future.
    let found_child = lookup_uid(crates, found_child_uid);
    if !matches!(found_child.inner, ItemEnum::Macro(_)) {
      narrowed_found_child_ids.push(found_child_uid.clone());
    }
  }
  return narrowed_found_child_ids;
}

fn get_impl_children(
  crates: &HashMap<String, Crate>,
  primitive_name_to_uid: &HashMap<String, UId>,
  impl_uid: &UId
) -> anyhow::Result<Option<Vec<UId>>, ResolveError> {
  let item = lookup_uid(crates, &impl_uid);
  if let ItemEnum::Impl(impl_) = &item.inner {
    if let Some(name) = &item.name {
      if name == "Any" {
        // Other crates seem to reference core::any::Any but don't have a path to it.
        // If this becomes a problem with a lot of other things we might have to make
        // lookup_uid return an optional.
        return Ok(None);
      }
    }

    // This impl_.items won't actually have everything.
    // some impls dont actually implement all the methods in their trait, they instead use the default implementation. for example, this
    // impl<'a> Iterator for Chars<'a> {
    //   file:///Users/verdagon/.rustup/toolchains/nightly-aarch64-apple-darwin/share/doc/rust/html/src/core/str/iter.rs.html#38
    // only overrides 6 methods of Iterator. luckily the doc lists the names of them so we can bring them in from the trait.
    // TODO: doc better
    let mut impl_method_uids: Vec<UId> =
        impl_.items
            .iter()
            .map(|x| UId { crate_name: impl_uid.crate_name.clone(), id: x.clone() })
            .collect();

    let mut impl_methods_names = HashSet::new();
    for impl_method_uid_unresolved in &impl_method_uids {
      let impl_method_item =
          lookup_uid(crates, &impl_method_uid_unresolved);
      if let Some(name) = &impl_method_item.name {
        if matches!(impl_method_item.inner, ItemEnum::Function(_)) {
          impl_methods_names.insert(name);
        } else {
          // There are sometimes associated types in there, maybe other things too
        }
      }
    }
    if let Some(trait_) = &impl_.trait_ {
      let trait_id = &trait_.id;
      let trait_unresolved_uid = UId { crate_name: impl_uid.crate_name.clone(), id: trait_id.clone() };
      let trait_uid =
          match resolve_uid(crates, &primitive_name_to_uid, &trait_unresolved_uid) {
            Ok(trait_uid) => trait_uid,
            Err(ResolveError::NotFound) => {
              let _ = resolve_uid(crates, &primitive_name_to_uid, &trait_unresolved_uid);
              unimplemented!();
            }
            Err(ResolveError::Unsupported(_)) => unreachable!("resolve_uid never returns Unsupported"),
            Err(ResolveError::ResolveFatal(fatal)) => {
              return Err(ResolveError::ResolveFatal(fatal))
            }
          };
      let trait_item = lookup_uid(crates, &trait_uid);
      let trait_ =
          match &trait_item.inner {
            ItemEnum::Trait(trait_) => trait_,
            _ => panic!("Not an impl!")
          };
      // Here, we grab the rest from the parent trait.
      let mut needed_names = HashSet::new();
      for name in &impl_.provided_trait_methods {
        if impl_methods_names.contains(&name) {
          continue;
        }
        // If we get here, then the impl didn't define this method, so it's
        // inheriting it from the parent.
        needed_names.insert(name);
      }

      for trait_child_id in &trait_.items {
        let trait_child_uid = UId { crate_name: trait_uid.crate_name.clone(), id: trait_child_id.clone() };
        let trait_child_item = lookup_uid(crates, &trait_child_uid);
        if let Some(name) = &trait_child_item.name {
          if needed_names.contains(name) {
            impl_method_uids.push(trait_child_uid);
          }
        }
      }
    }

    return Ok(Some(impl_method_uids));
  } else {
    panic!("Impl item id isn't impl.");
  }
}

// TODO: optimize: super expensive
// TODO: look for impls in other crates
// A concrete is a struct or an enum
fn get_concrete_impls(
  crates: &HashMap<String, Crate>,
  primitive_name_to_uid: &HashMap<String, UId>,
  concrete_id: &UId
) -> Vec<UId> {
  let crate_ = crates.get(&concrete_id.crate_name).unwrap();

  let mut result = Vec::new();
  for (neighbor_id, item) in &crate_.index {
    match &item.inner {
      ItemEnum::Impl(impl_) => {
        match &impl_.for_ {
          Type::ResolvedPath(path) => {
            if &path.id == &concrete_id.id {
              let uid =
                  UId { crate_name: concrete_id.crate_name.clone(), id: neighbor_id.clone() };
              result.push(uid);
            }
          },
          Type::Primitive(name) => {
            if let Some(item_for_primitive_id) = primitive_name_to_uid.get(name) {
              if item_for_primitive_id == concrete_id {
                let uid =
                    UId { crate_name: concrete_id.crate_name.clone(), id: neighbor_id.clone() };
                result.push(uid);
              }
            } else {
              // DO NOT SUBMIT put this warning back in
              // eprintln!("Primitive not found: {}", name);
            }
          }
          _ => {}
        }
      }
      _ => {}
    }
  }
  return result;
}

