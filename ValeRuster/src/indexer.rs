use std::collections::{HashMap, HashSet};
use rustdoc_types::{Crate, Enum, Id, Item, ItemEnum, Primitive, Struct};
use crate::{GenealogyKey, item_has_name, resolve_id, ResolveError, UId};
use crate::GenealogyKey::Normal;
use crate::resolve_id::{collapse_children, get_expanded_direct_child_uids, get_unexpanded_direct_child_uids_exclude_impl_children, include_impls_children, lookup_uid, resolve_uid};
use crate::ResolveError::ResolveFatal;

pub struct ItemIndex {
  pub(crate) child_key_to_parent_uid: HashMap<GenealogyKey, UId>,
  pub(crate) primitive_name_to_uid: HashMap<String, UId>,
  pub(crate) primitive_uid_to_name: HashMap<UId, String>,
  pub(crate) drop_trait_uid: UId,
  pub(crate) drop_method_uid: UId,
}

pub fn genealogize(
  crates: &HashMap<String, Crate>
) -> anyhow::Result<ItemIndex> {
  let core_crate = crates.get("core").unwrap();
  let core_root_module = core_crate.index.get(&core_crate.root).unwrap();
  let core_root_module_member_ids =
      match &core_root_module.inner {
        ItemEnum::Module(module) => &module.items,
        _ => panic!("wat")
      };
  let mut primitive_name_to_uid: HashMap<String, UId> = HashMap::new();
  let mut primitive_uid_to_name: HashMap<UId, String> = HashMap::new();
  for core_root_module_member_id in core_root_module_member_ids {
    let item = core_crate.index.get(&core_root_module_member_id).unwrap();
    match &item.inner {
      ItemEnum::Primitive(Primitive { name: name, .. }) => {
        let real_name = name;
          match name.as_str() {
            "fn" | "reference" | "tuple" | "slice" | "array" | "pointer" | "unit" | "never" | "bool" | "char" | "f16" | "f32" | "f64" | "f128" | "i128" | "i16" | "i32" | "i64" | "i8" | "isize" | "str" | "u128" | "u16" | "u32" | "u64" | "u8" | "usize" => {
              name.as_str()
            }
            _ => unimplemented!(),
          };
        let uid = UId { crate_name: "core".to_string(), id: core_root_module_member_id.clone() };
        primitive_name_to_uid.insert(real_name.to_string(), uid.clone());
        primitive_uid_to_name.insert(uid, real_name.to_string());
      }
      _ => {} // Skip
    }
  }

  let mut child_key_to_parent_uid: HashMap<GenealogyKey, UId> = HashMap::new();
  for (crate_name, crate_) in crates {
    for (self_id, item) in &crate_.index {
      let self_uid =
          UId { crate_name: crate_name.to_string(), id: self_id.clone() };
      let child_uids =
          match &item.inner {
            ItemEnum::Module(_) => {
              let direct_child_uids_without_methods =
                get_unexpanded_direct_child_uids_exclude_impl_children(crates, &primitive_name_to_uid, &self_uid)?;
              let direct_child_keys =
                  include_impls_children(crates, &primitive_name_to_uid, direct_child_uids_without_methods)?;
              let direct_child_uids = collapse_children(&direct_child_keys);
              direct_child_uids
                  .into_iter()
                  .map(|x| GenealogyKey::Normal(x.clone()))
                  .collect::<Vec<GenealogyKey>>()
            },
            ItemEnum::Primitive(Primitive { impls: impl_ids, .. }) |
            ItemEnum::Struct(Struct { impls: impl_ids, .. }) |
            ItemEnum::Enum(Enum { impls: impl_ids, .. }) => {
              let mut result = Vec::new();
              let direct_child_uids_without_methods =
                  get_unexpanded_direct_child_uids_exclude_impl_children(crates, &primitive_name_to_uid, &self_uid)?;
              let direct_child_keys =
                  include_impls_children(crates, &primitive_name_to_uid, direct_child_uids_without_methods)?;
              let direct_child_uids = collapse_children(&direct_child_keys);
              for child_id in direct_child_uids {
                result.push(GenealogyKey::ImplOrMethod { struct_uid: self_uid.clone(), child_uid: child_id.clone() });
              }
              // // Now add all the impls' children.
              // for impl_id in impl_ids {
              //   let impl_uid = UId { crate_name: crate_name.clone(), id: impl_id.clone() };
              //   for method_uid in get_unexpanded_direct_child_uids(crates, &primitive_name_to_uid, &impl_uid, true)? {
              //     result.push(GenealogyKey::ImplOrMethod { struct_uid: self_uid.clone(), child_uid: method_uid.clone() });
              //   }
              // }
              result
            },
            ItemEnum::Import(_import) => {
              continue;
            }
            _ => continue
          };
      for child_uid in child_uids {
        if let Some(existing_parent_uid) = child_key_to_parent_uid.get(&child_uid) {
          if *existing_parent_uid != self_uid {
            // TODO: resultify all these panics
            panic!("Parent collision {:?} and {:?} for child {:?}", existing_parent_uid, self_uid, child_uid);
          }
        } else {
          child_key_to_parent_uid.insert(child_uid, self_uid.clone());
        }
      }
    }
  }

  let mut importee_uid_to_imports: HashMap<UId, HashSet<UId>> = HashMap::new();
  // This loop modifies importee_uid_to_imports *and* the child_key_to_parent_uid.
  // Specifically, if it doesn't find a parent for something in child_key_to_parent_uid, it will
  // use an import for it.
  for (crate_name, crate_) in crates {
    for (self_id, item) in &crate_.index {
      let self_uid =
          UId { crate_name: crate_name.to_string(), id: self_id.clone() };
      match &item.inner {
        ItemEnum::Module(_) => {
          let direct_child_uids_without_methods =
              get_unexpanded_direct_child_uids_exclude_impl_children(crates, &primitive_name_to_uid, &self_uid)?;
          let direct_child_keys =
              include_impls_children(crates, &primitive_name_to_uid, direct_child_uids_without_methods)?;
          let direct_child_uids = collapse_children(&direct_child_keys);
          for direct_child_uid in &direct_child_uids {
            let direct_child_item = lookup_uid(crates, &direct_child_uid);
            match &direct_child_item.inner {
              ItemEnum::Import(import) => {
                let importee_uids =
                  if !import.glob {
                    if let Some(imported_id) = import.id.as_ref() {
                      vec![UId { crate_name: crate_name.clone(), id: imported_id.clone() }]
                    } else {
                      vec![]
                    }
                  } else {
                    // We treat glob imports as if we're directly importing
                    // everything matching them.
                    match import.name.as_str() {
                      // std references this in itself but doesn't include its definition.
                      "_Unwind_Reason_Code" => vec![],
                      _ => {
                        let target_module_uid =
                            match resolve_uid(crates, &primitive_name_to_uid, &direct_child_uid) {
                              Ok(x) => x,
                              Err(ResolveError::NotFound) => unimplemented!(),
                              Err(ResolveFatal(e)) => return Err(e)
                            };
                        match get_expanded_direct_child_uids(
                          crates, &primitive_name_to_uid, &target_module_uid, true) {
                          Ok(x) => {
                            x
                          },
                          Err(ResolveError::NotFound) => unimplemented!(),
                          Err(ResolveFatal(e)) => return Err(e)
                        }
                      }
                    }
                  };
                for importee_uid in importee_uids {
                  if !importee_uid_to_imports.contains_key(&importee_uid) {
                    importee_uid_to_imports.insert(importee_uid.clone(), HashSet::new());
                  }
                  // println!("Noting importee {:?} imported by {:?}", importee_uid, direct_child_uid.clone());
                  importee_uid_to_imports.get_mut(&importee_uid).unwrap().insert(direct_child_uid.clone());
                }
              }
              _ => {}
            }
          }
        },
        // We would do this code down here instead, if we want to handle non-modules' imports.
        // ItemEnum::Import(import) => {
        //   if let Some(imported_id) = import.id.as_ref() {
        //     let importee_uid =
        //         UId { crate_name: crate_name.clone(), id: imported_id.clone() };
        //     // We handle glob imports in the case handling Module items.
        //     if !import.glob {
        //       if !importee_uid_to_imports.contains_key(&importee_uid) {
        //         importee_uid_to_imports.insert(importee_uid.clone(), HashSet::new());
        //       }
        //       eprintln!("Noting importee {:?} imported by import {:?}", importee_uid, self_uid);
        //       importee_uid_to_imports.get_mut(&importee_uid).unwrap().insert(self_uid);
        //     }
        //   }
        // }
        _ => {}
      }
    }
  }

  let final_child_key_to_parent_uid =
    infer_missing_owners(
      crates, &primitive_name_to_uid, &mut child_key_to_parent_uid, &mut importee_uid_to_imports)?;

  let std =
      match resolve_id::extend_and_resolve_uid(
        crates, &primitive_name_to_uid, None, "std") {
        Ok(x) => x,
        Err(ResolveError::NotFound) => unimplemented!(),
        Err(ResolveFatal(e)) => return Err(e)
      };
  let ops =
      match resolve_id::extend_and_resolve_uid(
        crates, &primitive_name_to_uid, Some(&std), "ops") {
        Ok(x) => x,
        Err(ResolveError::NotFound) => unimplemented!(),
        Err(ResolveFatal(e)) => return Err(e)
      };
  let drop_trait =
      match resolve_id::extend_and_resolve_uid(
        crates, &primitive_name_to_uid, Some(&ops), "Drop") {
        Ok(x) => x,
        Err(ResolveError::NotFound) => unimplemented!(),
        Err(ResolveFatal(e)) => return Err(e)
      };
  let drop_method_uid =
      match resolve_id::extend_and_resolve_uid(
        crates, &primitive_name_to_uid, Some(&drop_trait), "drop") {
        Ok(x) => x,
        Err(ResolveError::NotFound) => unimplemented!(),
        Err(ResolveFatal(e)) => return Err(e)
      };

  return Ok(
    ItemIndex {
      child_key_to_parent_uid: final_child_key_to_parent_uid,
      primitive_uid_to_name,
      primitive_name_to_uid,
      drop_trait_uid: drop_trait,
      drop_method_uid: drop_method_uid
    })
}

// TODO: optimize: expensive clones
// TODO: optimize: memoize results, use past calculations
// Returns true if found a path to the root.
fn search_owner_paths(
  results: &mut Vec<Vec<UId>>,
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  importee_uid_to_imports: &HashMap<UId, HashSet<UId>>,
  path_so_far_from_child: Vec<UId>
) -> bool {
  assert!(path_so_far_from_child.len() > 0);
  let this_uid = path_so_far_from_child.last().unwrap();
  if item_is_crate_root_module(crates, this_uid) {
    results.push(path_so_far_from_child.clone());
    return true;
  }
  if let Some(parent_uid) = child_key_to_parent_uid.get(&Normal(this_uid.clone())) {
    let mut new_path = path_so_far_from_child.clone();
    new_path.push(parent_uid.clone());
    if search_owner_paths(results, crates, child_key_to_parent_uid, importee_uid_to_imports, new_path) {
      return true;
    }
  }
  if let Some(import_uids) = importee_uid_to_imports.get(&this_uid) {
    // Nothing, we've hit a dead end.
    // This can happen for example in the regex crate, to the crate::string module which nobody
    // imports or ever mentions.
    // Instead, the root module imports crate::string's children directly.

    let mut found = false;
    for import_uid in import_uids {
      let importer_uid = child_key_to_parent_uid.get(&Normal(import_uid.clone())).unwrap();
      let mut new_path = path_so_far_from_child.clone();
      new_path.push(importer_uid.clone());
      search_owner_paths(results, crates, child_key_to_parent_uid, importee_uid_to_imports, new_path);
      found = true;
    }
    return found;
  }
  return false;
}

fn item_is_crate_root_module(
  crates: &HashMap<String, Crate>,
  this_uid: &UId
) -> bool {
  let this_item = lookup_uid(crates, this_uid);
  match &this_item.inner {
    ItemEnum::Module(m) => {
      if m.is_crate {
        return true;
      }
    }
    _ => {}
  }
  false
}

fn infer_missing_owners(
  crates: &HashMap<String, Crate>,
  primitive_name_to_uid: &HashMap<String, UId>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  importee_uid_to_imports: &HashMap<UId, HashSet<UId>>
) -> anyhow::Result<HashMap<GenealogyKey, UId>> {
  let mut result: HashMap<GenealogyKey, UId> = HashMap::new();

  // Sanity check:
  for (crate_name, crate_) in crates {
    for (item_id, _item) in &crate_.index {
      let item_uid =
          UId { crate_name: crate_name.to_string(), id: item_id.clone() };
      if item_uid.id.0 == "0:462:2678" {

      }
      let item = crate_.index.get(&item_uid.id).unwrap();
      match &item.inner {
        ItemEnum::Module(_) => {
          if item_is_crate_root_module(crates, &item_uid) {
            continue;
          }
          let parent_uid =
              match determine_ultimate_owner(crates, child_key_to_parent_uid, importee_uid_to_imports, &item_uid) {
                None => {
                  println!("No owners or imports for {:?}", item_uid);
                  continue
                },
                Some(value) => value,
              };
          // let import_key = Normal(parent_uid);
          // println!("{:?} Noting new owner, module {:?} parent import {:?}", result.len(), item_uid.clone(), parent_uid);
          assert!(!result.contains_key(&Normal(item_uid.clone())));
          result.insert(Normal(item_uid), parent_uid.clone());
        }
        ItemEnum::Primitive(Primitive { impls: impl_ids, .. }) |
        ItemEnum::Struct(Struct { impls: impl_ids, .. }) |
        ItemEnum::Enum(Enum { impls: impl_ids, .. }) => {
          if item_has_name(&item, "Chars") {

          }
          let parent_uid =
              match determine_ultimate_owner(crates, child_key_to_parent_uid, importee_uid_to_imports, &item_uid) {
                None => {
                  println!("No owners or imports for {:?}", item_uid);
                  continue
                },
                Some(value) => value,
              };
          // println!("{:?} Noting new owner for {:?}, parent {:?}", result.len(), item_uid.clone(), parent_uid);
          assert!(!result.contains_key(&Normal(item_uid.clone())));
          result.insert(Normal(item_uid.clone()), parent_uid.clone());

          let direct_child_uids_without_methods =
              get_unexpanded_direct_child_uids_exclude_impl_children(
                crates, &primitive_name_to_uid, &item_uid)?;
          let direct_child_keys =
            include_impls_children(
              crates, &primitive_name_to_uid, direct_child_uids_without_methods)?;
          // println!("Direct child keys: {:?}", &direct_child_keys);
          let direct_child_uids = collapse_children(&direct_child_keys);
          // println!("Direct child uids: {:?}", &direct_child_uids);
          // Now look for all their methods.
          let mut method_keys = Vec::new();
          for direct_child_uid in direct_child_uids {
            // println!("Pushing A {:?}", GenealogyKey::ImplOrMethod { struct_uid: item_uid.clone(), child_uid: direct_child_uid.clone() });
            method_keys.push(GenealogyKey::ImplOrMethod { struct_uid: item_uid.clone(), child_uid: direct_child_uid.clone() });
          }
          // // Now add all the impls' children.
          // for impl_id in impl_ids {
          //   let impl_uid = UId { crate_name: crate_name.clone(), id: impl_id.clone() };
          //   for method_uid in get_unexpanded_direct_child_uids(crates, &primitive_name_to_uid, &impl_uid, true)? {
          //     println!("Pushing B {:?}", GenealogyKey::ImplOrMethod { struct_uid: item_uid.clone(), child_uid: method_uid.clone() });
          //     method_keys.push(GenealogyKey::ImplOrMethod { struct_uid: item_uid.clone(), child_uid: method_uid.clone() });
          //   }
          // }
          for method_key in method_keys {
            match &method_key {
              GenealogyKey::ImplOrMethod {
                struct_uid: UId { crate_name: _, id: Id(x) },
                child_uid: UId { crate_name: _, id: Id(y) },
              } if x == "0:3816:3157" && y == "1:3721:1799" => {

              }
              _ => {}
            }

            // println!("{:?} Noting impl-method {:?} has owner {:?}", result.len(), &method_key, &item_uid);
            assert!(!result.contains_key(&method_key));
            result.insert(method_key, item_uid.clone());
          }
        }
        ItemEnum::Function(_func) => {
          if child_key_to_parent_uid.contains_key(&Normal(item_uid.clone())) {
            // Then it's a free function.
            let parent_uid =
                match determine_ultimate_owner(crates, child_key_to_parent_uid, importee_uid_to_imports, &item_uid) {
                  None => {
                    println!("No owners or imports for {:?}", &item_uid);
                    continue
                  },
                  Some(value) => value,
                };
            // let import_key = Normal(parent_uid);
            // println!("{:?} Noting new owner for free function {:?}, import {:?}", result.len(), item_uid.clone(), parent_uid);
            assert!(!result.contains_key(&Normal(item_uid.clone())));
            result.insert(Normal(item_uid), parent_uid.clone());
          } else {
            // It's a method, skip it. We'll get it in the struct|trait|enum case.
          }
        }
        // ItemEnum::Function(_) => {}
        _ => {}
      }
    }
  }

  // for (crate_name, crate_) in crates {
  //   for (item_id, _item) in &crate_.index {
  //     let item_uid =
  //         UId { crate_name: crate_name.to_string(), id: item_id.clone() };
  //     let item = crate_.index.get(&item_uid.id).unwrap();
  //     match item.inner {
  //       ItemEnum::Module(_) | ItemEnum::Struct(_) | ItemEnum::Enum(_) | ItemEnum::Primitive(_) => {
  //         let item_key = Normal(item_uid.clone());
  //         if !result.contains_key(&item_key) {
  //           if let Some(unnarrowed_imports) = importee_uid_to_imports.get(item_key.uid()) {
  //             let imports: Vec<UId> =
  //                 if unnarrowed_imports.len() == 0 {
  //                   println!("No owners or imports for {:?}", item_key);
  //                   continue;
  //                 } else if unnarrowed_imports.len() > 1 {
  //                   // Narrow it down by the smallest path. For example, LineWriter
  //                   // is imported in two places:
  //                   //   "library/std/src/io/buffered/mod.rs",
  //                   //   "library/std/src/io/mod.rs",
  //                   // so we'll go with the second one.
  //                   // TODO: use a better resolution approach
  //                   let mut bork: Vec<(UId, &Item)> =
  //                       unnarrowed_imports.iter()
  //                           .map(|id| (id.clone(), lookup_uid(crates, id)))
  //                           .collect::<Vec<_>>();
  //                   bork.sort_by_key(|(_uid, item)| item.span.as_ref().unwrap().filename.to_str().as_ref().unwrap().len());
  //                   println!("Heuristic, estimating owner for {:?} is {:?}", item_key, bork.iter().next().unwrap().clone());
  //                   bork.into_iter().map(|x| x.0).collect()
  //                 } else {
  //                   unnarrowed_imports.iter().map(|x| x.clone()).collect()
  //                 };
  //             let import_key = Normal(imports.iter().next().unwrap().clone());
  //             if let Some(import_parent_id) = child_key_to_parent_uid.get(&import_key) {
  //               println!("{:?} Noting new owner for {:?}, import {:?}'s parent {:?}", result.len(), item_key, import_key.uid(), &import_parent_id);
  //               match result.get(&item_key) {
  //                 None => {}
  //                 Some(existing_id) => panic!("Already existing owner: {:?}", existing_id)
  //               }
  //               assert!(!result.contains_key(&item_key));
  //               result.insert(item_key, import_parent_id.clone());
  //             } else {
  //               println!("New owner for {:?}, import {:?} has no owner itself!", item_key, import_key.uid());
  //               continue;
  //             }
  //           } else {
  //             println!("Orphan module: {:?}", item.name);
  //           }
  //         }
  //       }
  //       // ItemEnum::Function(_) => {}
  //       _ => {}
  //     }
  //   }
  // }

  Ok(result)
}

fn determine_ultimate_owner(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  importee_uid_to_imports: &HashMap<UId, HashSet<UId>>,
  item_uid: &UId
) -> Option<UId> {
// let item_key = Normal(item_uid.clone());
  let mut unnarrowed_paths = Vec::new();
  search_owner_paths(
    &mut unnarrowed_paths,
    crates,
    child_key_to_parent_uid,
    importee_uid_to_imports,
    vec![item_uid.clone()]);
  let mut unnarrowed_parent_ids: Vec<UId> =
      unnarrowed_paths.into_iter()
          .map(|path| {
            // The first element will be this module, the 1th element will be its next parent.
            let x: UId = path.get(1).unwrap().clone();
            x
          })
          .collect::<Vec<UId>>();
  let parent_uid: UId =
      if unnarrowed_parent_ids.len() == 0 {
        return None;
      } else if unnarrowed_parent_ids.len() == 1 {
        unnarrowed_parent_ids.iter().next().unwrap().clone()
      } else {
        // Narrow it down by the smallest path. For example, LineWriter
        // is imported in two places:
        //   "library/std/src/io/buffered/mod.rs",
        //   "library/std/src/io/mod.rs",
        // so we'll go with the second one.
        // TODO: use a better resolution approach
        let mut parent_uids_and_items: Vec<(UId, &Item)> =
            unnarrowed_parent_ids.iter()
                .map(|id| (id.clone(), lookup_uid(crates, id)))
                .collect::<Vec<(UId, &Item)>>();
        parent_uids_and_items.sort_by_key(|(_uid, item)| item.span.as_ref().unwrap().filename.to_str().as_ref().unwrap().len());
        // println!("Heuristic, estimating owner for {:?} is {:?}", item_uid, parent_uids_and_items.iter().next().unwrap().clone());
        parent_uids_and_items
            .into_iter()
            .map(|x| x.0)
            .next()
            .unwrap()
            .clone()
      };
  Some(parent_uid)
}
