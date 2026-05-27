use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use rustdoc_types::{Crate, Enum, Id, Item, ItemEnum, Primitive, Struct};
use anyhow::{Context, Result};
use crate::{resolve_id, ResolveError, UId};
use crate::resolve_id::{collapse_children, get_expanded_direct_child_uids, get_unexpanded_direct_child_uids_exclude_impl_children, include_impls_children, lookup_uid, resolve_uid};
use crate::ResolveError::ResolveFatal;
use crate::indexer::GenealogyKey::{Normal, ImplOrMethod};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum GenealogyKey {
  Normal(UId),
  ImplOrMethod{ struct_uid: UId, child_uid: UId }, // TODO: rename
}
impl GenealogyKey {
  fn uid(&self) -> &UId {
    match self {
      Normal(uid) => uid,
      ImplOrMethod { struct_uid: _, child_uid: child_uid } => child_uid
    }
  }
}

pub struct ItemIndex {
  pub(crate) primitive_name_to_uid: HashMap<String, UId>,
  pub(crate) primitive_uid_to_name: HashMap<UId, String>,
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

  return Ok(
    ItemIndex {
      primitive_uid_to_name,
      primitive_name_to_uid,
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

pub(crate) fn get_crate(rustc_sysroot_path: &str, cargo_path: &str, output_dir_path: &str, crate_name: &str) -> Result<Crate, anyhow::Error> {
  let json =
      if let Some(crate_json) = get_stdlib_json(&rustc_sysroot_path, crate_name)? {
        crate_json
      } else {
        get_dependency_json(cargo_path, output_dir_path, crate_name)?
      };
  let v: Crate = serde_json::from_str(&json)?;
  Ok(v)
}

fn get_stdlib_json(rustc_sysroot_path: &str, crate_name: &str) -> Result<Option<String>, anyhow::Error> {
  let stdlib_json_file_path: String =
      rustc_sysroot_path.to_string() + "/share/doc/rust/json/" + &crate_name + ".json";
  eprintln!("Reading json from {}", stdlib_json_file_path);

  if !Path::new(&stdlib_json_file_path).exists() {
    return Ok(None)
  }

  let mut stdlib_json_file =
      File::open(stdlib_json_file_path.clone())
          .with_context(|| format!("Failed to open {}", stdlib_json_file_path))?;

  let mut stdlib_json_str: String = String::new();
  stdlib_json_file.read_to_string(&mut stdlib_json_str)
      .with_context(|| format!("Failed to read {} into string", stdlib_json_file_path))?;

  return Ok(Some(stdlib_json_str));
}

fn get_dependency_json(cargo_path: &str, output_dir_path: &str, crate_name: &str) -> Result<String, anyhow::Error> {
  let output = Command::new(&cargo_path)
      .args(&[
        "rustdoc",
        &("--manifest-path=".to_string() + output_dir_path + "/Cargo.toml"),
        "-Zunstable-options",
        "--output-format=json",
        "--package",
        crate_name
      ])
      .output()
      .with_context(|| "Failed to execute cargo rustdoc command")?;
  if output.status.code() == Some(0) {
    // Continue
  } else {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let error = "Error from cargo rustdoc command: ".to_string() + &stderr;
    return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
  }

  let json_file_path: String =
      output_dir_path.to_owned() + "/target/doc/" + &crate_name.replace("-", "_") + ".json";

  if !Path::new(&json_file_path).exists() {
    let error = format!("cargo rustdoc command didn't generate json in the expected place ({})", json_file_path);
    return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
  }

  let mut stdlib_json_file =
      File::open(json_file_path.clone())
          .with_context(|| format!("Failed to open {}", json_file_path))?;

  let mut stdlib_json_str: String = String::new();
  stdlib_json_file.read_to_string(&mut stdlib_json_str)
      .with_context(|| format!("Failed to read {} into string", json_file_path))?;

  return Ok(stdlib_json_str);
}

pub(crate) fn get_dependency_crates(
  rustc_sysroot_path: &str,
  cargo_path: &str,
  output_dir_path: &str,
  cargo_toml_path: &str,
  crates: &mut HashMap<String, Crate>
) -> Result<()> {

  // Open the Cargo.toml file
  let mut file = match File::open(cargo_toml_path) {
    Ok(file) => file,
    Err(err) => {
      let error = format!("Error opening {}: {}", cargo_toml_path, err);
      return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
    }
  };

  // Read the contents of the file into a string
  let mut contents = String::new();
  if let Err(err) = file.read_to_string(&mut contents) {
    let error = format!("Error reading {}: {}", cargo_toml_path, err);
    return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
  }

  // Parse the TOML contents
  let toml_contents = match contents.parse::<toml::Value>() {
    Ok(toml_contents) => toml_contents,
    Err(err) => {
      let error = format!("Error parsing {}: {}", cargo_toml_path, err);
      return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
    }
  };

  // Get the dependencies table
  let dependencies = match toml_contents.get("dependencies") {
    Some(dependencies) => dependencies,
    None => {
      let error = format!("No dependencies found in {}", cargo_toml_path);
      return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
    }
  };

  // Iterate over the dependencies
  let dependencies_table =
      if let Some(dependencies_table) = dependencies.as_table() {
        dependencies_table
      } else {
        let error = format!("No dependencies found in {}", cargo_toml_path);
        return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
      };

  for (name, _) in dependencies_table.iter() {
    crates.insert(name.clone(), get_crate(rustc_sysroot_path, cargo_path, output_dir_path, name)?);
  }
  Ok(())
}
