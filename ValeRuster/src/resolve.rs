use std::collections::HashMap;
use rustdoc_types::{Crate, Impl, Import, Item, ItemEnum};
use crate::{resolve_id, ResolveError, SimpleType, SimpleValType, UId};
use crate::indexer::ItemIndex;
use crate::ResolveError::{NotFound, ResolveFatal};
use crate::resolve_id::get_expanded_direct_child_uids;
use crate::resolve_id::get_unexpanded_direct_child_uids_exclude_impl_children;
use crate::resolve_id::include_impls_children;

// Recurses.
pub fn resolve(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  // If we're resolving an impl or a method, we'll want to know what
  // struct we're coming from.
  maybe_parent_concrete: Option<&SimpleValType>,
  maybe_parent_impl: Option<&SimpleValType>,
  tentative_item_id: &UId,
  final_generic_args: Vec<SimpleType>
) -> anyhow::Result<SimpleValType, ResolveError> {
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
          let mut current = extend_and_resolve(crates, item_index, None, foreign_crate_name, Vec::new())?;
          for i in 1.. path.path.len() {
            let step = &path.path[i];
            current =
                match extend_and_resolve(crates, item_index, Some(&current), step, Vec::new()) {
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
                  Err(ResolveError::ResolveFatal(fatal)) => return Err(ResolveError::ResolveFatal(fatal))
                };
          }
          // We should only be overwriting the Vec::new() above
          assert!(current.generic_args.is_empty());
          current.generic_args = final_generic_args;
          Ok(current)
        }
      }
    }
    Some(tentative_item) => {
      // // TODO: impl
      // assert!(tentative_path.steps.last().unwrap().generic_args.is_empty());

      match &tentative_item.inner {
        ItemEnum::ExternCrate { .. } => unimplemented!(),
        ItemEnum::Import(import) => {
          // When we do an import, we actually want the module the
          // import is referring to, not the import's id.

          let local_crate = crates.get(&tentative_item_id.crate_name).unwrap();
          let import_id = import.id.as_ref().unwrap();
          match local_crate.paths.get(&import_id) {
            None => {
              resolve(
                crates,
                item_index,
                maybe_parent_concrete,
                maybe_parent_impl,
                &UId {
                  crate_name: tentative_item_id.crate_name.clone(),
                  id: import_id.clone()
                },
                final_generic_args)
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
                let hey_direct_child_ids =
                    match get_unexpanded_direct_child_uids_exclude_impl_children(crates, &item_index.primitive_name_to_uid, &result_uid) {
                      Ok(x) => x,
                      Err(e) => return Err(ResolveFatal(e))
                    };
                // let hay_child_keys =
                //     match include_impls_children(crates, &item_index.primitive_name_to_uid, hey_direct_child_ids) {
                //       Ok(x) => x,
                //       Err(e) => return Err(ResolveFatal(e))
                //     };
                let mut maybe_found_child_ids: Vec<UId> = Vec::new();
                for hay_child_key in hey_direct_child_ids {
                  let hay_child = foreign_crate.index.get(&hay_child_key.id).unwrap();
                  if crate::item_has_name(&hay_child, next_foreign_crate_name) {
                    maybe_found_child_ids.push(hay_child_key.clone());
                  }
                }
                if maybe_found_child_ids.len() == 0 {
                  unimplemented!();
                } else if maybe_found_child_ids.len() > 1 {
                  unimplemented!();
                }
                result_uid = maybe_found_child_ids.get(0).unwrap().clone();
              }
              // Recurse
              assert_ne!(&result_uid, tentative_item_id); // Otherwise infinite loop
              resolve(
                crates,
                item_index,
                maybe_parent_concrete,
                maybe_parent_impl,
                &result_uid,
                final_generic_args)
            }
          }
        }
        ItemEnum::TraitAlias(_) => unimplemented!(),
        ItemEnum::TypeAlias(type_alias) => {
          let mut generics: HashMap<String, SimpleType> = HashMap::new();
          for (generic_param, generic_arg_type) in type_alias.generics.params.iter().zip(&final_generic_args) {
            // println!("Got generic arg: {:?}", generic_arg_type);
            generics.insert(generic_param.name.to_string(), generic_arg_type.clone());
          }

          match crate::simplify_type(crates, &item_index, &generics, &tentative_item_id.crate_name, &type_alias.type_) {
            Ok(type_) => Ok(type_.valtype),
            Err(_) => {
              unimplemented!()
            }
          }

          //     // We can't use path.id because it literally refers to nothing WHY, RUST
          //     let generics = HashMap::new();
          //     let type_ = simplify_type(crates, context_container, &generics, &type_alias.type_)?;
          //     get_child_ids(
          //       crates, context_container, &type_.valtype.id)?
          //
          //     // get_child_ids(
          //     //   crates, &UId{crate_name: resolved_id.crate_name, id: path.id.clone() })?
        },
        ItemEnum::ForeignType => unimplemented!(),
        _ => {

          Ok(
            SimpleValType {
              id: tentative_item_id.clone(),
              generic_args: final_generic_args,
              maybe_parent_concrete: {
                maybe_parent_concrete.map(|x| Box::new(x.clone()))
              },
              maybe_parent_impl: {
                maybe_parent_impl.map(|x| Box::new(x.clone()))
              }
            })
        }
      }
    }
  }
}

// This adds the given step to the end, and if it lands on an
// import or a typealias it will follow it and return the new corrected
// path.
pub fn extend_and_resolve(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  // This is the containing module or impl.
  previous: Option<&SimpleValType>,
  name: &str,
  initial_generic_args: Vec<SimpleType>
) -> anyhow::Result<SimpleValType, ResolveError> {
  if name.len() == 0 {
    // For the life of me I can't figure out the Id for tuples, I suspect they don't have one
    // because they have a special entry in the Type enum.
    // We'll just use the empty string.
    return Ok(
      SimpleValType {
        id: crate::tuple_id(&item_index.primitive_name_to_uid),
        generic_args: initial_generic_args,
        maybe_parent_concrete: None,
        maybe_parent_impl: None,
      });
  }

  match previous {
    None => {
      match name {
        "bool" | "char" | "f32" | "f64" | "f128" | "i128" | "i16" | "i32" | "i64" | "i8" | "isize" | "str" | "u128" | "u16" | "u32" | "u64" | "u8" | "usize" => {
          Ok(
            SimpleValType {
              id: crate::primitive_id(&item_index.primitive_name_to_uid, name),
              generic_args: Vec::new(),
              maybe_parent_concrete: None,
              maybe_parent_impl: None
            })
        }
        _ => {
          match crates.get(name) {
            None => {
              return Err(NotFound);
              // let error = format!("Couldn't find any crate named {}", name);
              // return Err(ResolveFatal(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error))));
            }
            Some(crate_) => {
              let root_module_id = &crate_.root;
              Ok(SimpleValType {
                id: UId { crate_name: name.to_string(), id: root_module_id.clone() },
                generic_args: Vec::new(),
                maybe_parent_concrete: None,
                maybe_parent_impl: None
              })
            }
          }
        }
      }
    }
    Some(previous_last_step) => {
      let previous_container_id = &previous_last_step.id;
      let previous_crate_name = &previous_container_id.crate_name;
      // let previous_container_item = previous_crate.index.get(&previous_container_id.id).unwrap();

      let direct_child_uids =
          get_expanded_direct_child_uids(
            &crates, &item_index.primitive_name_to_uid, &previous_container_id, true)?;
      let mut found_items: Vec<(UId, &Item)> = Vec::new();
      for direct_child_uid in direct_child_uids {
        let direct_child_item =
            crates.get(&direct_child_uid.crate_name).unwrap()
                .index.get(&direct_child_uid.id).unwrap();
        if crate::item_has_name(&direct_child_item, name) {
          found_items.push((direct_child_uid, direct_child_item));
        }
      }

      let mut unnarrowed_imports: Vec<(UId, &Import)> = Vec::new();
      let mut unfiltered_unnarrowed_others: Vec<(UId, &Item)> = Vec::new();
      let mut unnarrowed_impls: Vec<(UId, &Impl)> = Vec::new();
      for (item_uid, item) in found_items {
        match &item.inner {
          ItemEnum::Impl(impl_) => unnarrowed_impls.push((item_uid.clone(), impl_)),
          ItemEnum::Import(import_) => unnarrowed_imports.push((item_uid.clone(), import_)),
          _ => unfiltered_unnarrowed_others.push((item_uid.clone(), item)),
        }
      }
      let (imports, impls, others) =
          if unnarrowed_imports.len() + unnarrowed_impls.len() + unfiltered_unnarrowed_others.len() <= 1 {
            // Then no need for anything more.
            (
              unnarrowed_imports,
              unnarrowed_impls
                  .into_iter()
                  .map(|(uid, import)| (uid, import, None))
                  .collect::<Vec<_>>(),
              unfiltered_unnarrowed_others)
          } else {
            // Then lets do some overload resolution.

            // Narrow down imports. Sometimes there are two imports pointing to the same place.
            // WARNING: THIS MODIFIES unnarrowed_impls which is read below.
            for (import_uid, _import) in &unnarrowed_imports {
              let resolved_import_uid =
                  match resolve_id::resolve_uid(crates, &item_index.primitive_name_to_uid, import_uid) {
                    Ok(thing) => thing,
                    Err(ResolveError::NotFound) => {
                      unimplemented!();
                    }
                    Err(ResolveError::ResolveFatal(fatal)) => return Err(ResolveFatal(fatal))
                  };
              let import_item =
                  resolve_id::lookup_uid(crates, &resolved_import_uid);
              match &import_item.inner {
                ItemEnum::Import(_) | ItemEnum::TypeAlias(_) => panic!("Resolve didn't work!"),
                ItemEnum::Impl(impl_) => {
                  unnarrowed_impls.push((resolved_import_uid, impl_));
                }
                ItemEnum::Module(_) | ItemEnum::Struct(_) | ItemEnum::Trait(_) | ItemEnum::Function(_) | ItemEnum::Enum(_) => {
                  unfiltered_unnarrowed_others.push((resolved_import_uid, &import_item));
                }
                ItemEnum::Macro(_) => unimplemented!(), // is this possible? i think we want to ignore these
                ItemEnum::Primitive(_) => {
                  // Do nothing, we resolve these manually elsewhere.
                }
                _ => unimplemented!(),
              }
            }

            let mut unnarrowed_others: Vec<(UId, &Item)> = Vec::new();
            for (item_uid, item) in unfiltered_unnarrowed_others {
              match &item.inner {
                ItemEnum::Import(_) | ItemEnum::TypeAlias(_) => panic!("Resolve didn't work!"),
                ItemEnum::Impl(_impl_) => panic!("Impl shouldnt be in this list"),
                ItemEnum::Module(_) | ItemEnum::Struct(_) | ItemEnum::Trait(_) | ItemEnum::Function(_) | ItemEnum::Enum(_) => {
                  unnarrowed_others.push((item_uid, &item));
                }
                ItemEnum::Primitive(_) => {
                  // Do nothing, we resolve these manually elsewhere.
                }
                _ => unimplemented!(),
              }
            }

            // Dedup
            unnarrowed_others =
                unnarrowed_others
                    .into_iter().collect::<HashMap<UId, &Item>>()
                    .into_iter().collect();
            unnarrowed_impls =
                unnarrowed_impls
                    .into_iter().collect::<HashMap<UId, &Impl>>()
                    .into_iter().collect();

            // Narrow down impls
            let mut impl_matches: Vec<(UId, &Impl, i64, Vec<SimpleType>)> = Vec::new();
            for (impl_uid, impl_) in unnarrowed_impls {
              if let Some((score, generics)) = crate::impl_from_matches_generic_args(crates, &item_index, impl_, &initial_generic_args) {
                impl_matches.push((impl_uid.clone(), impl_, score, generics));
              }
            }
            if impl_matches.len() + unnarrowed_others.len() > 1 &&
                impl_matches.len() > 0 {
              eprintln!("Too many matches! Doing impl overload resolution.");
              for m in &impl_matches {
                eprintln!("Candidate: {:?}", m);
              }
              impl_matches.sort_by_key(|(_id, _impl_, score, _generics)| {
                -score // We want highest first
              });
              if impl_matches.len() > 1 {
                if impl_matches[0].2 == impl_matches[1].2 {
                  // Then the scores are the same, we couldn't decide.
                  unimplemented!();
                }
              }
              impl_matches = vec![impl_matches.remove(0)];
            }

            if impl_matches.len() + unnarrowed_others.len() > 1 {
              let error = format!("Too many matches for name: {}", name);
              return Err(ResolveFatal(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error))));
            }

            let mut narrowed_impls: Vec<(UId, &Impl, i64, Vec<SimpleType>)> = Vec::new();
            if impl_matches.len() > 0 {
              narrowed_impls.push(impl_matches[0].clone());
            }

            (
              Vec::new(),
              narrowed_impls.into_iter()
                  .map(|(uid, import, score, generics)| {
                    (uid, import, Some((score, generics)))
                  })
                  .collect(),
              unnarrowed_others)
          };
      let num_found_items = impls.len() + imports.len() + others.len();
      if num_found_items == 0 {
        return Err(NotFound)
        // let error = format!("Couldn't find anything with name: {}", name);
        // return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
      } else if num_found_items > 1 {
        // If they're all impls, then let's do some overload resolution.

        let error = format!("Found too many things with name: {}", name);
        eprintln!("Error: {}", error);
        for found_item in &imports {
          eprintln!("Candidate: {:?}", found_item);
        }
        for found_item in &others {
          eprintln!("Candidate: {:?}", found_item);
        }
        for found_item in &impls {
          eprintln!("Candidate: {:?}", found_item);
        }
        return Err(ResolveFatal(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error))));
      }

      let (maybe_self_struct, maybe_self_impl): (Option<&SimpleValType>, Option<&SimpleValType>) =
          if let Some(previous_parent) = &previous.unwrap().maybe_parent_concrete {
            // If the previous had a parent, then the previous was an impl and the previous parent
            // was a struct and we're a method.
            // Use the same parent as the impl.
            // TODO: explain more with examples, this is very nuanced
            (Some(previous_parent.as_ref()), Some(&previous.unwrap()))
          } else {
            match crates.get(previous_crate_name).unwrap().index.get(&previous_container_id.id).unwrap().inner {
              ItemEnum::Struct(_) | ItemEnum::Enum(_) | ItemEnum::Primitive(_) => {
                (previous, None)
              },
              _ => (None, None)
            }
          };

      // TODO: lets have two different types for uid, resolved and unresolved
      let (perhaps_unresolved_uid, generics) =
        if impls.len() > 0 {
          let impl_id = impls[0].0.clone();
          if let Some((_score, generics)) = &impls[0].2 {
            (impl_id, generics.clone())
          } else {
            (impl_id, Vec::new())
          }
        } else if imports.len() > 0 {
          (imports[0].0.clone(), initial_generic_args)
        } else if others.len() > 0 {
          (others[0].0.clone(), initial_generic_args)
        } else {
          panic!("wat");
        };

      let result_step =
          resolve(
            crates, &item_index, maybe_self_struct, maybe_self_impl, &perhaps_unresolved_uid, generics)?;

      // let mut tentative_path = previous.clone();
      // tentative_path.steps.push(result_step);
      Ok(result_step)
    }
  }
}
