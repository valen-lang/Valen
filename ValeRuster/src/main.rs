use std::collections::{HashMap, HashSet};
use std::{fs, io};
use std::cmp::Ordering;
use std::fmt::{Debug, Pointer};
use clap::Arg;
use std::process::{Command};
use clap::ArgAction;
use std::fs::File;
use std::io::{BufRead, Read};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use anyhow::{Context, Result};
use rustdoc_types::{Crate, Enum, Function, GenericArg, GenericArgs, Id, Item, ItemEnum, Struct, Type};
use regex::{Regex};
use crate::GenealogyKey::{ImplOrMethod, Normal};

// TODO: optimize: All over the place we're calling .keys() and .collect()
// on some crate.index.
fn tuple_id() -> UId { UId{ crate_name: "".to_string(), id: Id("".to_string()) } }
fn lifetime_id() -> UId { UId{ crate_name: "".to_string(), id: Id("life".to_string()) } }
fn primitive_id(name: &str) -> UId { UId{ crate_name: "".to_string(), id: Id(name.to_string()) } }

fn generic_placeholder_id() -> UId { UId{ crate_name: "".to_string(), id: Id("ph".to_string()) } }

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

// Universal id.
// This can refer to anything, including type aliases, imports, etc.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct UId {
  crate_name: String,
  id: Id,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct SimpleType {
  imm_ref: bool,
  mut_ref: bool,
  valtype: SimpleValType,
}
// It's guaranteed (well, in progress) that this thing's ID will not be
// pointing at a type alias or an import.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct SimpleValType {
  // This is the underlying actual ID, after we've resolved any
  // imports or type aliases or whatnot.
  // This wouldn't be std::io::Result's ID, since that's an import.
  // That import imports an alias. This ID isn't for that type alias.
  // This is the ID of the final core::result::Result.
  id: UId,
  // These are the generic args for the underlying thing, e.g.
  // core::result::Result not std::io::Result.
  generic_args: Vec<SimpleType>,

  // If this is a method, then the parent will be the struct.
  maybe_parent: Option<Box<SimpleValType>>
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct GenericArgName {
  id: Id,
  name: String
}

fn main() -> Result<(), anyhow::Error> {
  // _VR_2_std_collections_HashMap__i32__String
  // _VR_2_std_collections_HashMap__i32__2_std_collections_Vec__String

  let root_matches =
      clap::Command::new("ValeRuster")
          .version("1.0")
          .author("Evan Ovadia")
          .about("Creates bindings for arbitrary Rust libraries")
          .subcommand(
            clap::Command::new("list")
                .about("List all generic structs and functions")
                .arg(Arg::new("struct")
                    .long("struct")
                    .help("The struct to list methods for")
                    .action(ArgAction::Set)))
          .subcommand(
            clap::Command::new("instantiate")
                .about("Instantiate either a function or a struct.")
                  .arg(Arg::new("output_dir")
                      .long("output_dir")
                      .help("Directory to output to.")
                      .action(ArgAction::Set)
                      .required(true))
                  .arg(Arg::new("input_file")
                      .long("input_file")
                      .help("File to read from.")
                      .action(ArgAction::Set))
                // .arg(Arg::new("as")
                //     .long("as")
                //     .help("Sets the name for the newly generated type.")
                //     .action(ArgAction::Set))
                // .arg(Arg::new("c_folder")
                //     .long("c_folder")
                //     .help("The folder to output the C code to")
                //     .action(ArgAction::Set)
                //     .required(true))

                // .arg(Arg::new("generated")
                //     .long("generated")
                //     .help("The folder to output the Rust code to")
                //     .action(ArgAction::Set)
                //     .required(true)))
  )
          .arg(Arg::new("crate")
              .long("crate")
              .help("The crate name to generate bindings for")
              .action(ArgAction::Set)
              .required(true))
          .arg(Arg::new("cargo_toml")
              .long("cargo_toml")
              .help("The Cargo.toml to use for dependencies.")
              .action(ArgAction::Set)
              .required(true))
          .arg(Arg::new("rustc_path")
              .long("rustc_path")
              .help("Sets the path to rustc")
              .action(ArgAction::Set))
          .arg(Arg::new("cargo_path")
              .long("cargo_path")
              .help("Sets the path to cargo")
              .action(ArgAction::Set))
          .get_matches();

  let crate_name: String = root_matches.get_one::<String>("crate").unwrap().to_string();
  let rustc_path: String = root_matches.get_one::<String>("rustc_path").unwrap_or(&"rustc".to_string()).to_string();
  let cargo_path: String = root_matches.get_one::<String>("cargo_path").unwrap_or(&"cargo".to_string()).to_string();
  let cargo_toml_path: String = root_matches.get_one::<String>("cargo_toml").unwrap().to_string();

  // `rustc --print sysroot`/share/doc/rust/json
  let command = &rustc_path;
  let args = ["--print", "sysroot"];
  let output =
      Command::new(&rustc_path).args(&args).output()
          .with_context(|| format!("Failed to read {}", &rustc_path))?;
  if !output.status.success() {
    let error = String::from_utf8_lossy(&output.stderr);
    return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
  }
  let rustc_sysroot_path: String =
      String::from_utf8_lossy(&output.stdout).trim().to_string();
  if rustc_sysroot_path.len() == 0 {
    return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, format!("No stdout from command: {} {}", command, args.join(" ")))));
  }

  match root_matches.subcommand() {
    Some(("list", list_matches)) => {
      let maybe_struct_name = list_matches.get_one::<String>("struct");
      let v = get_crate(&rustc_sysroot_path, &crate_name)?;
      match maybe_struct_name {
        None => list_generics(&v),
        Some(struct_name) => list_struct_and_methods(&v, struct_name),
      }
    }
    Some(("instantiate", instantiate_matches)) => {
      // let c_folder = instantiate_matches.get_one::<String>("c_folder").unwrap();
      // if Path::new(c_folder).exists() {
      //   fs::remove_dir_all(c_folder)?;
      // }
      // fs::create_dir(c_folder)
      //     .with_context(|| "Failed to create directory ".to_owned() + c_folder)?;

      // let rust_folder = instantiate_matches.get_one::<String>("generated").unwrap();
      // if Path::new(rust_folder).exists() {
      //   fs::remove_dir_all(rust_folder)?;
      // }
      // fs::create_dir(rust_folder)
      //     .with_context(|| "Failed to create directory ".to_owned() + rust_folder)?;

      let mut type_to_alias: HashMap<SimpleValType, String> = HashMap::new();
      let mut valtype_and_original_line_and_type_str_and_maybe_alias: Vec<(SimpleValType, String, Option<String>)> = Vec::new();

      let output_dir_path = instantiate_matches.get_one::<String>("output_dir").unwrap();

      let maybe_input_file_path = instantiate_matches.get_one::<String>("input_file");

      let mut crates = HashMap::new();
      crates.insert("std".to_string(), get_crate(&rustc_sysroot_path, "std")?);
      crates.insert("alloc".to_string(), get_crate(&rustc_sysroot_path, "alloc")?);
      crates.insert("core".to_string(), get_crate(&rustc_sysroot_path, "core")?);

      let child_key_to_parent_uid: HashMap<GenealogyKey, UId> = genealogize(&crates)?;

      let mut lines: Vec<String> = Vec::new();
      if let Some(input_file_path) = maybe_input_file_path {
        let file = File::open(input_file_path)?;
        let reader = io::BufReader::new(file);
        for line_res in reader.lines() {
          let line = line_res?;
          if Regex::new(r#"^#pragma\s+vrinclude"#).unwrap().is_match(&line) {
            lines.push(line);
          }
        }
      } else {
        for line in io::stdin().lock().lines() {
          lines.push(line?);
        }
      }

      for line in lines {
        let line = line.trim().to_string();

        let maybe_aliasing_line_captures =
            Regex::new(r#"^\s*(#pragma\s+vrinclude\s+)?(\S.+)\s+as\s+(\w+)\s*$"#).unwrap()
                .captures(&line);
        let (target_type_str, maybe_alias) =
          if let Some(aliasing_line_captures) = maybe_aliasing_line_captures {
            // Getting the first capture group
            let target_type_str =
                aliasing_line_captures.get(2)
                    .expect("Blork")
                    .as_str();
            let maybe_alias =
                aliasing_line_captures.get(3).map(|x| x.as_str().to_string());
            (target_type_str, maybe_alias)
          } else {
            let simple_line_captures =
                Regex::new(r#"^\s*(#pragma\s+vrinclude\s+)?(\S.+)\s*$"#).unwrap()
                    .captures(&line)
                    .expect(&("Bad line: ".to_owned() + &line));;
            let target_type_str =
                simple_line_captures.get(2)
                    .expect("Blork")
                    .as_str();
            (target_type_str, None)
          };

        let (target_type, _) =
            parse_type(&crates, None, &target_type_str)?;

        if let Some(alias) = &maybe_alias {
          type_to_alias.insert(target_type.valtype.clone(), alias.clone());
        }
        eprintln!("Adding {:?}", target_type);
        valtype_and_original_line_and_type_str_and_maybe_alias.push(
          (target_type.valtype, line.clone(), maybe_alias));
      }

      if !Path::new(&output_dir_path).exists() {
        fs::create_dir(&output_dir_path)
            .with_context(|| "Failed to create ".to_owned() + output_dir_path + " directory")?;
      }
      if !Path::new(&(output_dir_path.to_string() + "/src")).exists() {
        fs::create_dir(output_dir_path.to_string() + "/src")
            .with_context(|| "Failed to create ".to_owned() + output_dir_path + "/src directory")?;
      }

      let cargo_toml_contents =
          fs::read_to_string(&cargo_toml_path)
              .with_context(|| "Failed to read Cargo toml at given path: ".to_owned() + &cargo_toml_path)?;

      // let mut cargo_toml_contents = String::with_capacity(1000);
      // cargo_toml_contents += "[package]\n";
      // cargo_toml_contents += "name = \"vale-rust-sizer\"\n";
      // cargo_toml_contents += "version = \"0.1.0\"\n";
      // cargo_toml_contents += "edition = \"2021\"\n";
      // cargo_toml_contents += "[dependencies]\n";
      // cargo_toml_contents += "subprocess = \"0.2.9\"\n";
      fs::write(output_dir_path.to_owned() + "/Cargo.toml", cargo_toml_contents)
          .with_context(|| "Failed to write ".to_owned() + output_dir_path + "/Cargo.toml")?;

      let mut cbindgen_toml_contents = String::with_capacity(1000);
      cbindgen_toml_contents += "include_guard = \"EXAMPLE_PROJECT_H\"\n";
      cbindgen_toml_contents += "include_version = true\n";
      cbindgen_toml_contents += "language = \"C\"\n";
      cbindgen_toml_contents += "cpp_compat = true\n";
      fs::write(output_dir_path.to_owned() + "/cbindgen.toml", cbindgen_toml_contents)
          .with_context(|| "Failed to write ".to_owned() + output_dir_path + "/Cargo.toml")?;

      let mut additions_for_valtype_to_original_line_and_type_str_and_maybe_alias: Vec<(SimpleValType, String, Option<String>)> = Vec::new();


      let mut sizer_strings = Vec::new();

      // TODO: Expensive call to string_path
      valtype_and_original_line_and_type_str_and_maybe_alias.sort_by_key(|x| x.0.id.id.0.clone());
      for (target_type, line, maybe_alias) in &valtype_and_original_line_and_type_str_and_maybe_alias {
        let target_valtype = target_type;
        // let target_name: String = unimplemented!();
        let rustified_type =
            rustify_simple_valtype(
              &crates, &child_key_to_parent_uid, &target_type, false);

        // let path = find_item_path(&crates, target_valtype.string_path())?;
        let unresolved_id = &target_valtype.id;
        let (id, item) =
            lookup_item(&crates, None, &unresolved_id)?.unwrap();

        match &item.inner {
          ItemEnum::Struct(_) => {
            // eprintln!("Sizing struct {}...", &target_name);
            sizer_strings.push(
              get_sizer_string(
                &rustify_simple_valtype(
                  &crates, &child_key_to_parent_uid, &target_valtype, false)));
          }
          ItemEnum::Function(func) => {
            let mut signature_types: Vec<&Type> = Vec::new();
            for (name, type_) in &func.decl.inputs {
              signature_types.push(type_);
            }
            if let Some(thing) = &func.decl.output {
              signature_types.push(thing);
            }
            for type_ in signature_types {
              match type_ {
                Type::ResolvedPath(_) | Type::Generic(_) | Type::FunctionPointer(_) | Type::Tuple(_) | Type::Slice(_) | Type::Array { .. } | Type::RawPointer { .. } | Type::BorrowedRef { .. } | Type::QualifiedPath { .. } => {
                  let generics = assemble_generics(&crates, func, &target_valtype)?;

                  let signature_part_type =
                      simplify_type(&crates, /*Some(target_valtype),*/ &generics, &target_valtype.id.crate_name, &type_)?;
                  let type_str =
                      rustify_simple_valtype(
                        &crates, &child_key_to_parent_uid, &signature_part_type.valtype, false);
                  eprintln!("Sizing {} from {}", &type_str, item.name.as_ref().unwrap_or(&"(none)".to_string()));
                  sizer_strings.push(get_sizer_string(&type_str));

                  additions_for_valtype_to_original_line_and_type_str_and_maybe_alias.push(
                    (signature_part_type.valtype, "Required from ".to_owned() + &rustified_type, None));
                }
                _ => {}
              }
            }
          }
          _ => {}
        }
      }

      // TODO: O(n^2) lol
      for (type_, original_line, maybe_alias) in additions_for_valtype_to_original_line_and_type_str_and_maybe_alias {
        // Check first; we don't want to overwrite it in case the user requested it and
        // perhaps even aliased it.
        if valtype_and_original_line_and_type_str_and_maybe_alias.iter().find(|x| x.0 == type_).is_none() {
          eprintln!("Adding {:?}", type_);
          valtype_and_original_line_and_type_str_and_maybe_alias.push((type_, original_line, maybe_alias));
        }
      }

      eprintln!("Running sizer program on {} types...", sizer_strings.len());

      let sizer_program_str =
        preamble().into_iter()
            .chain(std::iter::once("fn main() {".to_owned()))
            .chain(sizer_strings)
            .chain(std::iter::once("}".to_owned()))
            .collect::<Vec<String>>()
            .join("\n");

      if Path::new(&(output_dir_path.to_owned() + "/src/lib.rs")).exists() {
        fs::remove_file(output_dir_path.to_owned() + "/src/lib.rs")
            .with_context(|| "Failed to remove ".to_owned() + output_dir_path + "/src/lib.rs")?;
      }

      let sizer_program_output_str =
          get_rust_program_output(&cargo_path, output_dir_path, &sizer_program_str)?;
      let mut target_type_str_to_size: HashMap<String, usize> = HashMap::new();
      for line in sizer_program_output_str.split("\n") {
        let parts =
            line.split("=").map(|x| x.to_string()).collect::<Vec<_>>();
        let name: String =
            parts.get(0)
                .with_context(|| "Invalid output from sizer program: ".to_owned() + &line)?
                .to_owned();
        let size: usize =
            parts.get(1)
                .with_context(|| "Invalid output from sizer program: ".to_owned() + &line)?
                .parse()?;
        target_type_str_to_size.insert(name, size);
      }

      let mut struct_strings: Vec<String> = Vec::new();
      let mut func_strings: Vec<String> = Vec::new();

      valtype_and_original_line_and_type_str_and_maybe_alias
          .sort_by_key(|x| x.0.id.id.0.clone());
      for (target_type, line, maybe_alias) in &valtype_and_original_line_and_type_str_and_maybe_alias {
        let target_valtype = &target_type;
        let target_rust_type_string =
            rustify_simple_valtype(
              &crates, &child_key_to_parent_uid, &target_valtype, false);// target_valtype.name().clone();

        // let crate_ = get_crate(&crate_name, &rustc_sysroot_path)?;
        // let path = &target_valtype.id_path();// find_item_path(&crates, target_valtype.string_path)?;
        let unresolved_id = &target_valtype.id;// path.last().unwrap();
        let (id, item) =
            lookup_item(&crates, None, &unresolved_id)?.unwrap();
        // if let Some((id, item)) = crate_.index.iter().find(|(id, item)| item.name.as_ref() == target_valtype.name.as_ref()) {
        match &item.inner {
          ItemEnum::Struct(_) | ItemEnum::Enum(_) | ItemEnum::TypeAlias(_) | ItemEnum::Import(_) => {
            eprintln!("Instantiating type {}...", &target_rust_type_string);
            let type_str = rustify_simple_valtype(&crates, &child_key_to_parent_uid, &target_type, false);
            let size =
                target_type_str_to_size.get(&type_str)
                    .with_context(|| {
                      "Couldn't find size entry for struct: ".to_owned() + &type_str
                    })?
                    .clone();
            struct_strings.push(
              instantiate_struct(
                &crates, &child_key_to_parent_uid,
                &target_valtype, &maybe_alias, size)?);
          }
          ItemEnum::Function(func) => {
            eprintln!("Instantiating function {}...", &target_rust_type_string);

            // let (needle_type, _) =
            //     parse_type(crates, None, &needle_full_name_str)?;
            //
            // let (needle_type, _) =
            //     parse_type(crates, None, &needle_full_name_str)?;
            // let default_name = mangle_simple_valtype_name(&needle_valtype);
            // let as_ = maybe_alias.as_ref().unwrap_or(&default_name);

            // let mangled_func_name =
            //     needle_type.valtype.module.iter().map(|x| x.to_string() + &"_").collect::<Vec<_>>().join("") +
            //     &needle_type.valtype.name.as_ref().map(|x| x.to_string()).unwrap_or("Tup".to_string()); // TODO

            let generics = assemble_generics(&crates, func, &target_valtype)?;

            // let mut c_builder = String::with_capacity(1000);
            // c_builder += "#include <stdint.h>\n";
            // c_builder += &mangle_simple_type_name(&simplify_type(crates, &generics, func.decl.output.as_ref().with_context(|| "Couldn't get output type.")?));
            // c_builder += " ";
            // c_builder += &mangled_func_name;
            // c_builder += "(";
            // c_builder +=
            //     &func.decl.inputs.iter().map(|(param_name, param_type)| {
            //       mangle_simple_type_name(&simplify_type(crates, &generics, &param_type)) +
            //       " " +
            //       &param_name
            //     }).collect::<Vec<_>>().join(", ");
            // c_builder += ");";
            // fs::write(c_folder.to_owned() + "/" + &mangled_func_name + ".h", c_builder)
            //     .with_context(|| "Failed to write ".to_owned() + c_folder + "/" + &mangled_func_name + ".h")?;

            let mut params: Vec<(&String, SimpleType)> = Vec::new();
            for (name, param_type) in func.decl.inputs.iter() {
              params.push(
                (
                    name,
                    simplify_type(&crates, /*Some(target_valtype),*/ &generics, &target_valtype.id.crate_name, param_type)?));
            }
            let maybe_return_type: Option<SimpleType> =
                if let Some(output) = &func.decl.output {
                  Some(simplify_type(&crates, /*Some(target_valtype),*/ &generics, &target_valtype.id.crate_name, &output)?)
                } else {
                  None
                };
            func_strings.push(
              instantiate_func(
                &crates, &child_key_to_parent_uid,
                &type_to_alias, target_type, &maybe_alias, &params, &maybe_return_type)?);
          }
          _ => {
            unimplemented!();
          }
        }
      }


      let final_program_str =
          preamble().into_iter()
              .chain(struct_strings.into_iter())
              .chain(func_strings.into_iter())
              .collect::<Vec<String>>()
              .join("\n");

      fs::write(output_dir_path.to_string() + "/src/capi.rs", final_program_str)
          .with_context(|| "Failed to write ".to_string() + output_dir_path + "/src/capi.rs")?;

      fs::write(output_dir_path.to_string() + "/src/lib.rs", "#[cfg(feature = \"capi\")] mod capi;")
          .with_context(|| "Failed to write ".to_string() + output_dir_path + "/src/lib.rs")?;

      fs::remove_file(output_dir_path.to_string() + "/src/main.rs")
          .with_context(|| "Failed to remove ".to_string() + output_dir_path + "/src/main.rs")?;

      let output = Command::new(&cargo_path)
          .args(&[
            "cbuild",
            &("--manifest-path=".to_string() + output_dir_path + "/Cargo.toml"),
            "--destdir=clibthing",
            "--library-type", "staticlib"])
          .output()
          .with_context(|| "Failed to execute cbuild command")?;
      if output.status.code() == Some(0) {
        // Continue
      } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let error = "Error from cbuild command: ".to_string() + &stderr;
        return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
      }

      let output = Command::new(&cargo_path)
          .args(&[
            "cinstall",
            &("--manifest-path=".to_string() + output_dir_path + "/Cargo.toml"),
            "--destdir=clibthing",
            "--library-type", "staticlib"])
          .output()
          .with_context(|| "Failed to execute cbuild command")?;
      if output.status.code() == Some(0) {
        // Continue
      } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let error = "Error from cinstall command: ".to_string() + &stderr;
        return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
      }
    }
    _ => {
      unimplemented!();
    }
  }

  // println!("{:?}", v);

  return Ok(());
}

fn preamble() -> [String; 5] {
  [
    "use static_assertions::const_assert_eq;".to_owned(),
    "use std::mem;".to_owned(),
    "extern crate alloc;".to_owned(),
    "use core;".to_owned(),
    "".to_owned()
  ]
}

fn genealogize(
    crates: &HashMap<String, Crate>
) -> Result<HashMap<GenealogyKey, UId>> {
  let mut child_key_to_parent_uid: HashMap<GenealogyKey, UId> = HashMap::new();

  let mut importee_uid_to_imports: HashMap<UId, HashSet<UId>> = HashMap::new();

  for (crate_name, crate_) in crates {
    for (self_id, item) in &crate_.index {
      let self_uid =
          UId { crate_name: crate_name.to_string(), id: self_id.clone() };
      let child_uids =
        match &item.inner {
          ItemEnum::Module(_) => {
            get_direct_child_uids(crates, &self_uid)
                .into_iter()
                .map(|x| GenealogyKey::Normal(x.clone()))
                .collect::<Vec<GenealogyKey>>()
          },
          ItemEnum::Struct(Struct { impls: impl_ids, .. }) |
          ItemEnum::Enum(Enum { impls: impl_ids, .. }) => {
            get_direct_child_uids(crates, &self_uid)
                .into_iter()
                .map(|impl_uid| {
                  GenealogyKey::ImplOrMethod { struct_uid: self_uid.clone(), child_uid: impl_uid.clone() }
                })
                // Now add all the impls' children.
                .chain(
                  impl_ids
                        .iter()
                        .flat_map(|impl_id| {
                          let impl_uid = UId { crate_name: crate_name.clone(), id: impl_id.clone() };
                          get_direct_child_uids(crates, &impl_uid)
                              .iter()
                              .map(|method_uid| {
                                GenealogyKey::ImplOrMethod { struct_uid: self_uid.clone(), child_uid: method_uid.clone() }
                              })
                              .collect::<Vec<_>>()
                        }))
                .collect::<Vec<GenealogyKey>>()
          },
          ItemEnum::Import(import) => {
            if let Some(imported_id) = import.id.as_ref() {
              let importee_uid =
                UId { crate_name: crate_name.clone(), id: imported_id.clone() };
              if !importee_uid_to_imports.contains_key(&importee_uid) {
                importee_uid_to_imports.insert(importee_uid.clone(), HashSet::new());
              }
              eprintln!("Noting importee {:?} imported by import {:?}", importee_uid, self_uid);
              importee_uid_to_imports.get_mut(&importee_uid).unwrap().insert(self_uid);
            }
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

  // Sanity check:
  for (crate_name, crate_) in crates {
    for (item_id, item) in &crate_.index {
      let item_uid =
          UId { crate_name: crate_name.to_string(), id: item_id.clone() };
      let item = crate_.index.get(&item_uid.id).unwrap();
      match item.inner {
        ItemEnum::Module(_) | ItemEnum::Struct(_) | ItemEnum::Enum(_) => {
          let item_key = Normal(item_uid.clone());
          if !child_key_to_parent_uid.contains_key(&item_key) {
            if let Some(imports) = importee_uid_to_imports.get(item_key.uid()) {
              if imports.len() == 0 {
                eprintln!("No owners or imports for {:?}", item_key);
                continue;
              } else if imports.len() > 1 {
                eprintln!("Disputed orphan module {:?}", item_key);
                continue;
              }
              let import_key = Normal(imports.iter().next().unwrap().clone());
              if let Some(import_parent_id) = child_key_to_parent_uid.get(&import_key) {
                eprintln!("Noting new owner for {:?}, import {:?}", item_key, import_key.uid());
                child_key_to_parent_uid.insert(item_key, import_parent_id.clone());
              } else {
                eprintln!("New owner for {:?}, import {:?} has no owner itself!", item_key, import_key.uid());
                continue;
              }
            } else {
              eprintln!("Orphan module: {:?}", item.name);
            }
          }
        }
        // ItemEnum::Function(_) => {}
        _ => {}
      }
    }
  }
  return Ok(child_key_to_parent_uid);
}

// Returns None for blacklisted modules like std_detect
fn item_matches_name<'a>(
  crates: &'a HashMap<String, Crate>,
  needle: &str,
  id: &UId,
) -> bool {
  let context_crate = crates.get(&id.crate_name).unwrap();
  if let Some(item) = context_crate.index.get(&id.id) {
    match &item.inner {
      ItemEnum::Import(import) => {
        // When we do an import, we actually want the module the
        // import is referring to, not the import's id.

        if import.name.starts_with(needle) && import.name != needle {
          unimplemented!();
        }
        import.name == needle
        //
        // let local_crate = crates.get(&id.crate_name).unwrap();
        // let import_id = import.id.as_ref().unwrap();
        // match local_crate.paths.get(&import_id) {
        //   None => {
        //     return Ok(
        //       Some(
        //         (
        //           UId { crate_name: id.crate_name.clone(), id: import_id.clone() },
        //           local_crate.index.get(import_id).unwrap())));
        //   }
        //   Some(path) => {
        //     if path.path[0] == "std_detect" {
        //       return Ok(None)
        //     }
        //     if path.path[0] == "core" && path.path[1] == "macros" {
        //       return Ok(None)
        //     }
        //
        //     let foreign_crate_path = SimpleValType { steps: Vec::new() };
        //     let foreign_crate_root_name = &path.path[0];
        //     let foreign_crate = crates.get(foreign_crate_root_name).unwrap();
        //     let foreign_crate_root_module_id = UId { crate_name: foreign_crate_root_name.clone(), id: foreign_crate.root.clone() };
        //     let foreign_crate_root_module = foreign_crate.index.get(&foreign_crate_root_module_id.id).unwrap();
        //
        //     let mut result = SimpleValType { steps: Vec::new() };
        //     result.steps.push(
        //       SimpleValTypeStep {
        //         id: foreign_crate_root_module_id.clone(),
        //         name: foreign_crate_root_name.clone(),
        //         generic_args: Vec::new()
        //       });
        //     for foreign_crate_name in &path.path[1..] {
        //       let foreign_crate_module_id = find_id_with_name(crates, &result, foreign_crate_name)?;
        //       result.steps.push(
        //         SimpleValTypeStep {
        //           id: foreign_crate_module_id.clone(),
        //           name: foreign_crate_name.clone(),
        //           generic_args: Vec::new()
        //         });
        //     }
        //     return Ok(
        //       Some(
        //         (
        //           result.steps.last().unwrap().id.clone(),
        //           foreign_crate.index
        //               .get(&result.steps.last().unwrap().id.id.clone())
        //               .unwrap())));
        //   }
        // }
      },
      _ => {
        item.name.as_ref().map(|x| &x[..]) == Some(needle)
      }
    }
  } else {
    false
  }
}

// Returns None for blacklisted modules like std_detect
fn lookup_item<'a>(
  crates: &'a HashMap<String, Crate>,
  context_container: Option<&SimpleValType>,
  id: &UId
) -> Result<Option<(UId, &'a Item)>> {
  let context_crate = crates.get(&id.crate_name).unwrap();
  if let Some(item) = context_crate.index.get(&id.id) {
    match &item.inner {
      ItemEnum::Import(import) => {
        // When we do an import, we actually want the module the
        // import is referring to, not the import's id.

        let local_crate = crates.get(&id.crate_name).unwrap();
        let import_id = import.id.as_ref().unwrap();
        match local_crate.paths.get(&import_id) {
          None => {
            return Ok(
              Some(
                (
                    UId { crate_name: id.crate_name.clone(), id: import_id.clone() },
                    local_crate.index.get(import_id).unwrap())));
          }
          Some(path) => {
            if path.path[0] == "std_detect" {
              return Ok(None)
            }
            if path.path[0] == "core" && path.path[1] == "macros" {
              return Ok(None)
            }

            // let foreign_crate_path = SimpleValType { steps: Vec::new() };
            let foreign_crate_root_name = &path.path[0];
            let foreign_crate = crates.get(foreign_crate_root_name).unwrap();
            let foreign_crate_root_module_id = UId { crate_name: foreign_crate_root_name.clone(), id: foreign_crate.root.clone() };
            let foreign_crate_root_module = foreign_crate.index.get(&foreign_crate_root_module_id.id).unwrap();

            let mut result =
                SimpleValType {
                  id: foreign_crate_root_module_id.clone(),
                  generic_args: Vec::new(),
                  maybe_parent: None
                };
            for foreign_crate_name in &path.path[1..] {
              let foreign_crate_module_id = find_id_with_name(crates, Some(&result), foreign_crate_name)?;
              result =
                  SimpleValType {
                    id: foreign_crate_module_id.clone(),
                    generic_args: Vec::new(),
                    maybe_parent: None
                  };
            }
            return Ok(
              Some(
                (
                  result.id.clone(),
                  foreign_crate.index
                      .get(&result.id.id.clone())
                      .unwrap())));
          }
        }
      },
      _ => {} // skip
    }

    return Ok(Some((id.clone(), item)));
  }
  panic!("Item not found with Id {:?}", id);
}

fn assemble_generics(
  crates: &HashMap<String, Crate>,
  func: &Function,
  target_valtype: &SimpleValType
) -> Result<HashMap<String, SimpleType>> {
  let mut generics: HashMap<String, SimpleType> = HashMap::new();
  // TODO: dedup with instantiate_func's
  for (generic_param, generic_arg_type) in func.generics.params.iter().zip(&target_valtype.generic_args) {
    // println!("Got generic arg: {:?}", generic_arg_type);
    generics.insert(generic_param.name.to_string(), generic_arg_type.clone());
  }

  if let Some(parent) = &target_valtype.maybe_parent {
    generics.insert(
      "Self".to_owned(),
      SimpleType {
        imm_ref: false,
        mut_ref: false,
        valtype: (**parent).clone()
      });
    // let (_, parent_item) = lookup_item(crates, Some(target_valtype), &parent.id)?.unwrap();
    // match &parent_item.inner {
    //   ItemEnum::Module(_) => {} // Do nothing
    //   ItemEnum::Struct(struct_) => {
    //     // Get the parent struct's type and add it as Self generic.
    //     // let mut struct_valtype = target_valtype.clone();
    //     // struct_valtype.steps.remove(struct_valtype.steps.len() - 1);
    //     let struct_type =
    //         SimpleType { imm_ref: false, mut_ref: false, valtype: *parent.clone() };
    //     generics.insert("Self".to_owned(), struct_type);
    //   }
    //   ItemEnum::Impl(impl_) => {
    //     unimplemented!();
    //     // let struct_type =
    //     //     simplify_type(
    //     //       &crate_name, &crates, &Vec::new(), &HashMap::new(), &impl_.for_)?;
    //     // generics.insert("Self".to_owned(), struct_type);
    //   }
    //   _ => {
    //     panic!("Wat func in");
    //   }
    // }
  }
  Ok(generics)
}

// fn find_item_path(
//   crates: &HashMap<String, Crate>,
//   in_ids: &Vec<UId>,
//   mut path: Vec<String>
// ) -> Result<Vec<UId>> {
//   if path.len() == 1 && path[0] == "" {
//     return Ok(Vec::new());
//   }
//   if path.len() == 0 {
//     return Ok(Vec::new());
//   }
//   // eprintln!("Looking through candidates.");
//   let id = find_id_with_name(crates, in_ids, &path[0])?.clone();
//
//   let item = lookup_item(crates, &id);
//
//   path.remove(0);
//   if path.len() == 0 {
//     return Ok(vec![id]);
//   } else {
//     match &item.inner {
//       ItemEnum::Module(m) => {
//         let mut result =
//             find_item_path(
//               crates,
//               &m.items.iter().map(|child_id| UId { crate_name: id.crate_name.clone(), id: child_id.clone() }).collect(),
//               path)?;
//         result.insert(0, id);
//         return Ok(result);
//       }
//       ItemEnum::Struct(_) => {
//         let impl_matches = get_struct_impl_children(crates, &id);
//         let mut result = find_item_path(crates, &impl_matches, path)?;
//         result.insert(0, id);
//         return Ok(result);
//       }
//       _ => {
//         let error = format!("Couldn't find type {}, path dead-ended.", path.last().unwrap());
//         eprintln!("{}", error);
//         return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
//       }
//     }
//   }
// }


// This adds the given step to the end, and if it lands on an
// import or a typealias it will follow it and return the new corrected
// path.
fn extend_and_resolve(
  crates: &HashMap<String, Crate>,
  // This is the containing module or impl.
  previous: Option<&SimpleValType>,
  name: &str,
  generic_args: Vec<SimpleType>
) -> Result<SimpleValType> {
  if name.len() == 0 {
    // For the life of me I can't figure out the Id for tuples, I suspect they don't have one
    // because they have a special entry in the Type enum.
    // We'll just use the empty string.
    return Ok(
      SimpleValType {
        id: tuple_id(),
        generic_args: generic_args,
        maybe_parent: None,
      });
  }

  match previous {
    None => {
      match crates.get(name) {
        None => {
          let error = format!("Couldn't find any crate named {}", name);
          return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
        }
        Some(crate_) => {
          let root_module_id = &crate_.root;
          Ok(SimpleValType {
            id: UId { crate_name: name.to_string(), id: root_module_id.clone() },
            generic_args: Vec::new(),
            maybe_parent: None
          })
        }
      }
    }
    Some(previous_last_step) => {
      let previous_container_id = &previous_last_step.id;
      let previous_crate_name = &previous_container_id.crate_name;
      let previous_crate = crates.get(previous_crate_name).unwrap();
      // let previous_container_item = previous_crate.index.get(&previous_container_id.id).unwrap();

      let direct_child_ids =
          get_direct_child_uids(crates, &previous_container_id);
      let mut found_items: Vec<&Item> = Vec::new();
      for direct_child_id in direct_child_ids {
        let direct_child_item =
            previous_crate.index.get(&direct_child_id.id).unwrap();
        if item_has_name(&direct_child_item, name) {
          found_items.push(&direct_child_item);
        }
      }
      if found_items.len() == 0 {
        let error = format!("Couldn't find anything with name: {}", name);
        return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
      } else if found_items.len() > 1 {
        unimplemented!();
      }

      let maybe_self_struct =
          match previous_crate.index.get(&previous_container_id.id).unwrap().inner {
            ItemEnum::Struct(_) | ItemEnum::Enum(_) => previous,
            _ => None
          };

      let found_item = found_items[0];
      let found_item_unresolved_id = UId { crate_name: previous_crate_name.clone(), id: found_item.id.clone() };
      let result_step =
          resolve(crates, maybe_self_struct, &found_item_unresolved_id, generic_args)?;

      // let mut tentative_path = previous.clone();
      // tentative_path.steps.push(result_step);
      Ok(result_step)
    }
  }

  //
  // let unresolved_matches =
  //     filter_ids_by_name(crates, previous, &name)?;
  //
  // let mut resolved_matches = Vec::new();
  // for unresolved_id in &unresolved_matches {
  //   match lookup_item(crates, previous, &unresolved_id)? {
  //     None => {
  //       // It's a blacklisted module like std_detect, don't lookup in that direction
  //     }
  //     Some((resolved_id, item)) => {
  //       // Some conditions to skip things if there are multiple
  //       if unresolved_matches.len() > 1 {
  //         match item.inner {
  //           // Ignore macros, because their names sometimes collide with real things,
  //           // like "vec"
  //           ItemEnum::Macro(_) | ItemEnum::ProcMacro(_) => {
  //             continue;
  //           }
  //           // "array" is both a module and a primitive
  //           ItemEnum::Primitive(_) => {
  //             continue;
  //           }
  //           _ => {} // proceed
  //         }
  //       }
  //       resolved_matches.push(resolved_id);
  //     }
  //   }
  // }
  //
  // if resolved_matches.len() > 1 {
  //   resolved_matches.dedup();
  //   if resolved_matches.len() == 1 {
  //     eprintln!("Warning: duplicates in find_id_with_name.");
  //   }
  // }
  //
  // if resolved_matches.len() == 0 {
  //   let error = format!("Couldn't find {}, no matches.", name);
  //   // eprint!("{} Candidates: ", error);
  //   // for id in in_ids.into_iter() {
  //   //   eprint!("{}, ", lookup_item(crates, None, &id)?.unwrap().1.name.as_ref().unwrap_or(&"(no name)".to_string()));
  //   // }
  //
  //   // TODO: for debugging, remove
  //   filter_ids_by_name(crates, previous, &name)?;
  //
  //   return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
  // } else if resolved_matches.len() > 1 {
  //   let error = format!("Too many matches for {}", name);
  //   eprintln!("{}", error);
  //   for id in resolved_matches {
  //     let (resolved_id, item) = lookup_item(crates, previous, &id)?.unwrap();
  //     eprintln!(
  //       "Id {:?} resolved {:?} name {}",
  //       id,
  //       resolved_id,
  //       item.name.as_ref().unwrap_or(&"(none)".to_owned()));
  //     // if let Some(thing) = crate_.paths.get(&id) {
  //     //   eprintln!("  {:?} {:?}", thing.kind, thing.path.join("."));
  //     // }
  //   }
  //   return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
  // }
  // Ok(resolved_matches[0].clone())
}

// Recurses.
fn resolve(
  crates: &HashMap<String, Crate>,
  // If we're resolving an impl or a method, we'll want to know what
  // struct we're coming from.
  maybe_self_struct: Option<&SimpleValType>,
  tentative_item_id: &UId,
  final_generic_args: Vec<SimpleType>
) -> Result<SimpleValType> {
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
          let mut current = extend_and_resolve(crates, None, foreign_crate_name, Vec::new())?;
          for step in &path.path[1..] {
            current = extend_and_resolve(crates, Some(&current), step, Vec::new())?;
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
                maybe_self_struct,
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
                let mut maybe_found_child_id: Option<UId> = None;
                for hay_child_id in get_direct_child_uids(crates, &result_uid) {
                  let hay_child = foreign_crate.index.get(&hay_child_id.id).unwrap();
                  if item_has_name(&hay_child, next_foreign_crate_name) {
                    maybe_found_child_id = Some(hay_child_id);
                  }
                }
                if maybe_found_child_id.is_none() {
                  unimplemented!();
                }
                result_uid = maybe_found_child_id.unwrap();
              }
              // Recurse
              assert_ne!(&result_uid, tentative_item_id); // Otherwise infinite loop
              resolve(
                crates,
                maybe_self_struct,
                &result_uid,
                final_generic_args)
            }
          }
        }
        ItemEnum::TraitAlias(_) => unimplemented!(),
        ItemEnum::TypeAlias(type_alias) => {
          let mut generics = HashMap::new();
          for (generic_param, generic_arg_type) in type_alias.generics.params.iter().zip(&final_generic_args) {
            // println!("Got generic arg: {:?}", generic_arg_type);
            generics.insert(generic_param.name.to_string(), generic_arg_type.clone());
          }

          let type_ =
              simplify_type(
                crates, &generics, &tentative_item_id.crate_name, &type_alias.type_)?;
          Ok(type_.valtype)

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
              maybe_parent: maybe_self_struct.map(|x| Box::new(x.clone()))
            })
        }
      }
    }
  }
}

fn item_has_name(direct_child_item: &&Item, name: &str) -> bool {
  match &direct_child_item.inner {
    ItemEnum::Import(import) => {
      import.name == name
    }
    _ => {
      direct_child_item.name.as_ref().map(|x| &x[..]) == Some(name)
    }
  }
}

fn get_direct_child_uids(
  crates: &HashMap<String, Crate>,
  container_id: &UId,
) -> Vec<UId> {
  let container_item =
      crates
          .get(&container_id.crate_name).unwrap()
          .index.get(&container_id.id).unwrap();
  match &container_item.inner {
    ItemEnum::Module(m) => {
      m.items.iter()
          .map(|x| UId { crate_name: container_id.crate_name.clone(), id: x.clone() })
          .collect()
    },
    ItemEnum::Struct(_) | ItemEnum::Enum(_) => {
      get_concrete_impl_children(crates, &container_id)
    }
    ItemEnum::Impl(impl_) => {
      impl_.items.iter().map(|x| UId { crate_name: container_id.crate_name.clone(), id: x.clone() }).collect()
    },
    _ => unimplemented!()
  }
}

fn find_id_with_name(
  crates: &HashMap<String, Crate>,
  context_container: Option<&SimpleValType>,
  name: &str
) -> Result<UId> {
  if name.len() == 0 {
    // For the life of me I can't figure out the Id for tuples, I suspect they don't have one
    // because they have a special entry in the Type enum.
    // We'll just use the empty string.
    return Ok(tuple_id());
  }

  let unresolved_matches =
      filter_ids_by_name(crates, context_container, &name)?;

  let mut resolved_matches = Vec::new();
  for unresolved_id in &unresolved_matches {
    match lookup_item(crates, context_container, &unresolved_id)? {
      None => {
        // It's a blacklisted module like std_detect, don't lookup in that direction
      }
      Some((resolved_id, item)) => {
        // Some conditions to skip things if there are multiple
        if unresolved_matches.len() > 1 {
          match item.inner {
            // Ignore macros, because their names sometimes collide with real things,
            // like "vec"
            ItemEnum::Macro(_) | ItemEnum::ProcMacro(_) => {
              continue;
            }
            // "array" is both a module and a primitive
            ItemEnum::Primitive(_) => {
              continue;
            }
            _ => {} // proceed
          }
        }
        resolved_matches.push(resolved_id);
      }
    }
  }

  if resolved_matches.len() > 1 {
    resolved_matches.dedup();
    if resolved_matches.len() == 1 {
      eprintln!("Warning: duplicates in find_id_with_name.");
    }
  }

  if resolved_matches.len() == 0 {
    let error = format!("Couldn't find {}, no matches.", name);
    // eprint!("{} Candidates: ", error);
    // for id in in_ids.into_iter() {
    //   eprint!("{}, ", lookup_item(crates, None, &id)?.unwrap().1.name.as_ref().unwrap_or(&"(no name)".to_string()));
    // }

    // TODO: for debugging, remove
    filter_ids_by_name(crates, context_container, &name)?;

    return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
  } else if resolved_matches.len() > 1 {
    let error = format!("Too many matches for {}", name);
    eprintln!("{}", error);
    for id in resolved_matches {
      let (resolved_id, item) = lookup_item(crates, context_container, &id)?.unwrap();
      eprintln!(
        "Id {:?} resolved {:?} name {}",
        id,
        resolved_id,
        item.name.as_ref().unwrap_or(&"(none)".to_owned()));
      // if let Some(thing) = crate_.paths.get(&id) {
      //   eprintln!("  {:?} {:?}", thing.kind, thing.path.join("."));
      // }
    }
    return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
  }
  Ok(resolved_matches[0].clone())
}

fn filter_ids_by_name(
  crates: &HashMap<String, Crate>,
  context_container: Option<&SimpleValType>,
  name: &str
) -> Result<Vec<UId>> {
  unimplemented!();
  // let candidate_ids =
  //     get_child_or_root_ids(
  //       crates,
  //       context_container,
  //       &context.steps.last().map(|x: &SimpleValTypeStep| &x.id))?;
  //
  // let mut result = Vec::new();
  // for unresolved_candidate_id in candidate_ids {
  //   if item_matches_name(crates, name, &unresolved_candidate_id) {
  //     result.push(unresolved_candidate_id.clone());
  //   }
  // }
  // return Ok(result);
}

// TODO: optimize: super expensive
// TODO: look for impls in other crates
// A concrete is a struct or an enum
fn get_concrete_impl_children(crates: &HashMap<String, Crate>, concrete_id: &UId) -> Vec<UId> {
  let empty: Vec<Id> = Vec::new();
  let crate_ = crates.get(&concrete_id.crate_name).unwrap();
  crate_.index.iter()
      .flat_map(|(neighbor_id, item)| {
        match &item.inner {
          ItemEnum::Impl(impl_) => {
            match &impl_.for_ {
              Type::ResolvedPath(path) => {
                if &path.id == &concrete_id.id {
                  &impl_.items
                } else {
                  &empty
                }
              },
              _ => &empty
            }
          }
          _ => &empty
        }
      })
      .map(|x| UId { crate_name: concrete_id.crate_name.clone(), id: x.clone() })
      .collect::<Vec<_>>()
}

fn get_sizer_string(needle_full_name: &str) -> String {
  let mut rust_src = String::with_capacity(1000);
  rust_src += "  println!(\"";
  rust_src += needle_full_name;
  rust_src += "={}\", std::mem::size_of::<";
  rust_src += needle_full_name;
  rust_src += ">());";
  return rust_src;
}

fn get_rust_program_output(cargo_path: &String, output_dir_path: &String, rust_src: &str) -> anyhow::Result<String> {
  fs::write(output_dir_path.to_string() + "/src/main.rs", rust_src)
      .with_context(|| "Failed to write ".to_string() + output_dir_path + "/src/main.rs")?;

  let output = Command::new(cargo_path)
      .args(&["run", &("--manifest-path=".to_string() + output_dir_path + "/Cargo.toml")])
      .output()
      .with_context(|| "Failed to execute cargo run command")?;
  let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
  if output.status.code() == Some(0) {
    return Ok(stdout);
  } else {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let error = "Error from cargo run command: ".to_string() + &stderr;
    return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
  }
}

fn instantiate_struct(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  needle_valtype: &SimpleValType,
  maybe_alias: &Option<String>,
  size: usize
) -> anyhow::Result<String> {
  let default_name =
      mangle_simple_valtype_name(
        crates, child_key_to_parent_uid, needle_valtype);
  let as_ = maybe_alias.as_ref().unwrap_or(&default_name);


  let mut builder = String::with_capacity(1000);
  builder += "#[repr(C)]\n";
  builder += "pub struct ";
  builder += as_;
  builder += " ([u8; ";
  builder += &size.to_string();
  builder += "]);\n";
  builder += "const_assert_eq!(std::mem::size_of::<";
  builder += &rustify_simple_valtype(&crates, &child_key_to_parent_uid, &needle_valtype, false);
  builder += ">(), ";
  builder += &size.to_string();
  builder += ");\n";

  return Ok(builder);
}

fn instantiate_func(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  // crate_name: &str,
  // crates: &HashMap<String, Crate>,
  //
  aliases: &HashMap<SimpleValType, String>,
  // needle_full_name_str: &str,
  needle_type: &SimpleValType,
  // item: &Item,
  // context_container: Option<&SimpleValType>,
  // func: &Function,
  maybe_alias: &Option<String>,
  params: &Vec<(&String, SimpleType)>,
  maybe_output_type: &Option<SimpleType>
  // c_folder: &str,
  // rust_folder: &str
) -> anyhow::Result<String> {
  let default_name =
      mangle_simple_valtype_name(
        crates,
        child_key_to_parent_uid,
        &needle_type);
  let as_ = maybe_alias.as_ref().unwrap_or(&default_name);

  // let generics = assemble_generics(crates, func, &needle_valtype);

  let mut rust_builder = String::with_capacity(1000);
  rust_builder += "#[no_mangle]\n";
  rust_builder += "pub extern \"C\" fn ";
  rust_builder += &as_;
  rust_builder += "(\n";
  for (param_name, param_type) in params {
    rust_builder += &"  ";
    rust_builder += &param_name;
    rust_builder += &"_c: ";
    rust_builder += &crustify_simple_type(crates, child_key_to_parent_uid, aliases, &param_type);
    rust_builder += ",\n";
  }
  rust_builder += ")";
  if let Some(return_type_simple) = &maybe_output_type {
    rust_builder += " -> ";
    rust_builder += &crustify_simple_type(crates, child_key_to_parent_uid, aliases, &return_type_simple);
  }
  rust_builder += " {\n";
  for (param_name, param_type) in params {
    rust_builder += "  let ";
    rust_builder += param_name;
    rust_builder += "_rs: ";
    rust_builder += &rustify_simple_type(&crates, &child_key_to_parent_uid, &param_type, false);
    rust_builder += " = unsafe { mem::transmute(";
    rust_builder += param_name;
    rust_builder += "_c) };\n";
  }

  rust_builder += "  ";
  if let Some(return_type_simple) = maybe_output_type {
    rust_builder += "let result_rs: ";
    rust_builder += &rustify_simple_type(&crates, &child_key_to_parent_uid, &return_type_simple, false);
    rust_builder += " = ";
  }
  rust_builder += &rustify_simple_valtype(&crates, &child_key_to_parent_uid, needle_type, true);
  rust_builder += "(";
  for (param_name, param_type) in params {
    rust_builder += param_name;
    rust_builder += "_rs,";
  }
  rust_builder += ");\n";

  if let Some(return_type_simple) = &maybe_output_type {
    rust_builder += "  let result_c: ";
    rust_builder += &crustify_simple_type(crates, child_key_to_parent_uid, aliases, &return_type_simple);
    rust_builder += " = unsafe { mem::transmute(result_rs) };\n";
    rust_builder += "  return result_c;\n";
    }
  rust_builder += "}\n";

  // fs::write(rust_folder.to_owned() + "/" + &mangled_func_name + ".rs", rust_builder)
  //     .with_context(|| "Failed to write ".to_owned() + rust_folder + "/" + &mangled_func_name + ".rs")?;

  return Ok(rust_builder);
}

fn list_struct_and_methods(v: &Crate, struct_name: &String) {
  for (id, item) in &v.index {
    match &item.inner {
      ItemEnum::Struct(struuct) => {
        match &item.name {
          None => {
            eprintln!("{:?}", "No name, skipping struct!")
          }
          Some(name) => {
            if name == struct_name {
              println!("{:?} {:?}", name, struuct);
            }
          }
        }
      }
      ItemEnum::Impl(impl_) => {
        // println!("{:?}", impl_);
        match &impl_.for_ {
          Type::QualifiedPath { name, args, self_type, trait_ } => {
            if name == struct_name {
              // println!("{:?} {:?}", name, impl_);
            }
          }
          Type::ResolvedPath(path) => {
            // TODO: stop using ResolvedPath's name, it's inconsistent and we have an ID instead
            if &path.name == struct_name && impl_.trait_.is_none() {
              // println!("{:?} {:?}", path.name, impl_);

              for item_id in &impl_.items {
                let item = v.index.get(&item_id).unwrap();
                match &item.inner {
                  ItemEnum::Function(func) => {
                    unimplemented!();
                    // print_function(v, &Some(&impl_.for_), item, func);
                  }
                  ItemEnum::Module(_) => {}
                  ItemEnum::ExternCrate { .. } => {}
                  ItemEnum::Import(_) => {}
                  ItemEnum::Union(_) => {}
                  ItemEnum::Struct(_) => {}
                  ItemEnum::StructField(_) => {}
                  ItemEnum::Enum(_) => {}
                  ItemEnum::Variant(_) => {}
                  ItemEnum::Trait(_) => {}
                  ItemEnum::TraitAlias(_) => {}
                  ItemEnum::Impl(_) => {}
                  ItemEnum::TypeAlias(_) => {}
                  ItemEnum::OpaqueTy(_) => {}
                  ItemEnum::Constant(_) => {}
                  ItemEnum::Static(_) => {}
                  ItemEnum::ForeignType => {}
                  ItemEnum::Macro(_) => {}
                  ItemEnum::ProcMacro(_) => {}
                  ItemEnum::Primitive(_) => {}
                  ItemEnum::AssocConst { .. } => {}
                  ItemEnum::AssocType { .. } => {}
                }
                // println!("Item: {:?}", item);
              }
            }
          }
          Type::DynTrait(_) => {}
          Type::Generic(_) => {}
          Type::Primitive(_) => {}
          Type::FunctionPointer(_) => {}
          Type::Tuple(_) => {}
          Type::Slice(_) => {}
          Type::Array { .. } => {}
          Type::Pat { .. } => {}
          Type::ImplTrait(_) => {}
          Type::Infer => {}
          Type::RawPointer { .. } => {}
          Type::BorrowedRef { .. } => {}
        }
      }
      ItemEnum::Module(m) => {}
      ItemEnum::ExternCrate { .. } => {}
      ItemEnum::Import(_) => {}
      ItemEnum::Union(_) => {}
      ItemEnum::StructField(_) => {}
      ItemEnum::Enum(_) => {}
      ItemEnum::Variant(_) => {}
      ItemEnum::Function(_) => {}
      ItemEnum::Trait(_) => {}
      ItemEnum::TraitAlias(_) => {}
      ItemEnum::TypeAlias(_) => {}
      ItemEnum::OpaqueTy(_) => {}
      ItemEnum::Constant(_) => {}
      ItemEnum::Static(_) => {}
      ItemEnum::ForeignType => {}
      ItemEnum::Macro(_) => {}
      ItemEnum::ProcMacro(_) => {}
      ItemEnum::Primitive(_) => {}
      ItemEnum::AssocConst { .. } => {}
      ItemEnum::AssocType { .. } => {}
    }
    // if item.clone().name.is_some_and(|x| x == "File") {
    //   println!("{:?}", item.clone().name.unwrap_or("(none)".to_string()));
    //   // println!("{:?}", item.);
    // }
  }
}

fn list_generics(v: &Crate) {
  for (id, item) in &v.index {
    match &item.inner {
      ItemEnum::Struct(struuct) => {
        match &item.name {
          None => {
            eprintln!("{:?}", "No name, skipping struct!")
          }
          Some(name) => {
            if struuct.generics.params.len() > 0 {
              println!("generic: {:?}", name);
            } else {
              println!("non-generic: {:?}", name);
            }
          }
        }
      }
      ItemEnum::Function(func) => {}
      ItemEnum::Module(_) => {}
      ItemEnum::ExternCrate { .. } => {}
      ItemEnum::Import(_) => {}
      ItemEnum::Union(_) => {}
      ItemEnum::StructField(_) => {}
      ItemEnum::Enum(_) => {}
      ItemEnum::Variant(_) => {}
      ItemEnum::Trait(_) => {}
      ItemEnum::TraitAlias(_) => {}
      ItemEnum::Impl(_) => {}
      ItemEnum::TypeAlias(_) => {}
      ItemEnum::OpaqueTy(_) => {}
      ItemEnum::Constant(_) => {}
      ItemEnum::Static(_) => {}
      ItemEnum::ForeignType => {}
      ItemEnum::Macro(_) => {}
      ItemEnum::ProcMacro(_) => {}
      ItemEnum::Primitive(_) => {}
      ItemEnum::AssocConst { .. } => {}
      ItemEnum::AssocType { .. } => {}
    }
  }
}

fn get_crate(rustc_sysroot_path: &String, crate_name: &str) -> Result<Crate, anyhow::Error> {
  let json =
    if let Some(crate_json) = get_stdlib_json(&rustc_sysroot_path, crate_name)? {
      crate_json
    } else {
      unimplemented!()
    };
  let v: Crate = serde_json::from_str(&json)?;
  Ok(v)
}

fn print_function(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  crate_name: &str,
  context_container: Option<&SimpleValType>,
  generics: &HashMap<String, SimpleType>,
  item: &Item,
  func: &Function
) -> Result<()> {
  println!("Function: {:?}",
      "VR_".to_string() +
      &(match generics.get("Self") {
        None => "".to_string(),
        Some(self_type) => mangle_simple_type(crates, child_key_to_parent_uid, &self_type) + "_"
      }) +
      &item.clone().name.unwrap_or("(none)".to_string()));

  for (name, input) in &func.decl.inputs {
    println!("  {:?}: {:?}", name, mangle_simple_type(
      crates,
      child_key_to_parent_uid,
      &simplify_type(
        crates, /*context_container,*/ generics, crate_name, &input)?));
  }

  Ok(())
}

fn get_stdlib_json(rustc_sysroot_path: &str, crate_name: &str) -> Result<Option<String>, anyhow::Error> {
  let stdlib_json_file_path: String =
      rustc_sysroot_path.to_string() + "/share/doc/rust/json/" + &crate_name + ".json";

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

fn parse_type<'a>(
  crates: &HashMap<String, Crate>,
  context_container: Option<&SimpleValType>,
  original: &'a str
) -> anyhow::Result<(SimpleType, &'a str)> {
  let mut rest = original;

  let has_mut_ref = rest.starts_with("&mut"); // TODO: maybe handle &
  if has_mut_ref {
    rest = &rest["&mut".len()..].trim();
  }
  let has_imm_ref = rest.starts_with("&"); // TODO: maybe handle &
  if has_imm_ref {
    rest = &rest[1..].trim();
  }

  let (path, new_rest) =
      parse_type_path(crates, context_container, rest)?;
  rest = new_rest;

  return Ok(
    (
      SimpleType {
        imm_ref: has_imm_ref,
        mut_ref: has_mut_ref,
        valtype: path
      },
      rest));
}

fn parse_type_path<'a>(
  crates: &HashMap<String, Crate>,
  original_context_container: Option<&SimpleValType>,
  original: &'a str
) -> anyhow::Result<(SimpleValType, &'a str)> {
  let mut rest = original;
  let mut current =
      original_context_container.map(|x| x.clone());
  loop {
    let (new_steps, new_rest) =
        parse_type_path_step(crates, current.as_ref(), rest)?;
    current = Some(new_steps);
    rest = new_rest;
    if rest.starts_with("::") {
      rest = &rest["::".len()..].trim();
      // continue
    } else {
      break;
    }
  }
  Ok((current.unwrap(), rest))
}

fn parse_type_path_step<'a>(
  crates: &HashMap<String, Crate>,
  context_container: Option<&SimpleValType>,
  original: &'a str
) -> anyhow::Result<(SimpleValType, &'a str)> {

  let mut rest = original;
  let re = Regex::new(r"(::<|::|<|>|\(|\)|$)").unwrap();
  let name_end =
    if let Some(generic_name_match) = re.find(&rest) {
      generic_name_match.start()
    } else {
      rest.len()
    };
  let name = &rest[0..name_end];
  rest = &rest[name.len()..].trim();

  let mut generic_args = Vec::new();
  if rest.starts_with("::<") || rest.starts_with("<") || rest.starts_with("(") {
    if rest.starts_with("::<") {
      rest = &rest["::<".len()..].trim();
    } else if rest.starts_with("<") {
      rest = &rest["<".len()..].trim();
    } else if rest.starts_with("(") {
      rest = &rest["(".len()..].trim();
    } else {
      panic!("wat");
    }

    if rest.starts_with(">") {
      // Do nothing
    } else if rest.starts_with(")") {
      // Do nothing
    } else {
      loop {
        let (generic_arg, new_rest) = parse_type(crates, None, rest)?;
        rest = new_rest;
        generic_args.push(generic_arg);
        if rest.starts_with(",") {
          rest = &rest[",".len()..].trim();
          // continue
        } else if rest.starts_with(">") || rest.starts_with(")") {
          break;
        } else {
          return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Bad type string: {}", original))));
        }
      }
    }
    if rest.starts_with(">") {
      rest = &rest[">".len()..].trim();
    } else if rest.starts_with(")") {
      rest = &rest[")".len()..].trim();
    } else {
      panic!("Wat");
    }
  }

  let new_path =
      extend_and_resolve(crates, context_container, &name, generic_args)?;

  Ok((new_path, rest))
}

fn simplify_generic_arg_inner(
  crates: &HashMap<String, Crate>,
  // context_container: Option<&SimpleValType>,
  generics: &HashMap<String, SimpleType>,
  arg_crate_name: &str,
  arg: &GenericArg
) -> Result<SimpleType> {
  match arg {
    GenericArg::Lifetime(_) => {
      Ok(
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype: SimpleValType {
            id: lifetime_id(),
            generic_args: Vec::new(),
            maybe_parent: None
          }
        })
    },
    GenericArg::Type(type_) => Ok(simplify_type(crates, generics, arg_crate_name, type_)?),
    GenericArg::Const(_) => unimplemented!(),
    GenericArg::Infer => unimplemented!(),
  }
}

fn mangle_simple_type(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  original_type: &SimpleType
) -> String {
  let mut type_ = original_type.clone();
  let has_pointer = type_.imm_ref || type_.mut_ref;
  if original_type.valtype.id.crate_name == "" {
    // Then it's a primitive
    original_type.valtype.id.id.0.clone()
  } else {
    "VR_".to_owned() +
        &mangle_simple_valtype(crates, child_key_to_parent_uid, &type_.valtype) +
        (if has_pointer { "*" } else { "" })
  }
}

fn mangle_simple_valtype_name(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  original_type: &SimpleValType
) -> String {
  if original_type.id.crate_name == "" {
    // // Then its a primitive
    // mangle_simple_valtype_name_inner(&original_type)
    unimplemented!();
  } else {
    "VR_".to_owned() +
        &mangle_simple_valtype(
          crates,
          child_key_to_parent_uid,
          original_type)
  }
}

fn crustify_simple_type(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  aliases: &HashMap<SimpleValType, String>,
  type_: &SimpleType
) -> String {
  let valtype = &type_.valtype;
  (if type_.imm_ref { "*const ".to_owned() }
  else if type_.mut_ref { "*mut ".to_owned() }
  else { "".to_owned() }) +
  &(if let Some(alias) = aliases.get(&valtype) {
    alias.to_owned()
  } else {
    if type_.valtype.id.crate_name == "" {
      // It's a primitive
      mangle_simple_valtype(crates, child_key_to_parent_uid, &valtype)
    } else {
      "VR_".to_owned() +
          &mangle_simple_valtype(crates, child_key_to_parent_uid, &valtype)
    }
  })
}

fn get_name(uid: &UId) -> String {
  unimplemented!()
}

fn rustify_simple_valtype(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  valtype: &SimpleValType,
  is_func: bool
) -> String {
  let this_part_name =
      if valtype.id.crate_name == "" { // Then it's a primitive
        if valtype.id == tuple_id() {
          "".to_owned()
        } else {
          valtype.id.id.0.clone()
        }
      } else {
        let item =
            crates.get(&valtype.id.crate_name).unwrap()
                .index.get(&valtype.id.id).unwrap();
        item.name.as_ref().map(|x| &x[..]).unwrap_or("unnamed").to_string()
      };

  let key =
    match &valtype.maybe_parent {
      None => Normal(valtype.id.clone()),
      Some(parent) => {
        ImplOrMethod { struct_uid: parent.id.clone(), child_uid: valtype.id.clone() }
      }
    };
  let prefix =
    if let Some(parent_uid) = child_key_to_parent_uid.get(&key) {
      rustify_simple_valtype(
        crates,
        child_key_to_parent_uid,
        &SimpleValType {
          id: parent_uid.clone(),
          generic_args: Vec::new(),
          maybe_parent: None
        },
        is_func) + "::"
    } else {
      "".to_string()
    };

  let generics_part =
      // If it's a tuple then we still want to print out the () even if
      // there's nothing inside it.
      if valtype.generic_args.len() > 0 || valtype.id == tuple_id() {
        let (start, end) =
          if this_part_name == "" { ("(", ")") }
          else if is_func { ("::<", ">") }
          else { ("<", ">") };
        "".to_owned() +
        start +
        &valtype.generic_args
            .iter()
            .map(|x| {
              rustify_simple_type(
                crates, child_key_to_parent_uid, x, false)
            })
            .collect::<Vec<_>>()
            .join(", ") +
        end
      } else {
        "".to_owned()
      };
  prefix + &this_part_name + &generics_part
}
// TODO: It might be nice to get that is_func from the SimpleType...
//    not sure if that's possible though.
fn rustify_simple_type(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  type_: &SimpleType,
  is_func: bool
) -> String {
  (if type_.imm_ref { "&".to_owned() }
  else if type_.mut_ref { "&mut ".to_owned() }
  else { "".to_owned() }) +
      &rustify_simple_valtype(
        &crates, &child_key_to_parent_uid,
        &type_.valtype, is_func)
}

fn mangle_simple_type_name_inner(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  type_: &SimpleType
) -> String {
  (if type_.imm_ref {
    "1Ref_"
  } else if type_.mut_ref {
    "1MRef_"
  } else {
    ""
  }).to_string() +
      &mangle_simple_valtype(crates, child_key_to_parent_uid, &type_.valtype)
}

fn mangle_simple_valtype_step_name_inner(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  valtype_: &SimpleValType
) -> String {
  // let path_name_mangled = path.name.replace("::", "_");
  let generic_name = get_name(&valtype_.id);// valtype_.name_or("Tup");
  if valtype_.generic_args.len() == 0 {
    generic_name
  } else {
    // TODO: what about default args? args.len != params.len
    valtype_.generic_args.len().to_string() +
        &generic_name +
        &valtype_.generic_args.iter()
            .map(|x| "__".to_owned() + &mangle_simple_type_name_inner(crates, child_key_to_parent_uid, &x))
            .collect::<Vec<_>>()
            .join("")
  }
}

fn mangle_simple_valtype(
  crates: &HashMap<String, Crate>,
  child_key_to_parent_uid: &HashMap<GenealogyKey, UId>,
  valtype: &SimpleValType
) -> String {
  let this_part_name =
      if valtype.id.crate_name == "" { // Then it's a primitive
        if valtype.id == tuple_id() {
          "".to_owned()
        } else {
          valtype.id.id.0.clone()
        }
      } else {
        let item =
            crates.get(&valtype.id.crate_name).unwrap()
                .index.get(&valtype.id.id).unwrap();
        item.name.as_ref().map(|x| &x[..]).unwrap_or("unnamed").to_string()
      };

  let key =
      match &valtype.maybe_parent {
        None => Normal(valtype.id.clone()),
        Some(parent) => {
          ImplOrMethod { struct_uid: parent.id.clone(), child_uid: valtype.id.clone() }
        }
      };
  let prefix =
      if let Some(parent_uid) = child_key_to_parent_uid.get(&key) {
        mangle_simple_valtype(
          crates,
          child_key_to_parent_uid,
          &SimpleValType {
            id: parent_uid.clone(),
            generic_args: Vec::new(),
            maybe_parent: None
          }) + "_"
      } else {
        "".to_string()
      };

  let generics_part =
      // If it's a tuple then we still want to print out the () even if
      // there's nothing inside it.
      if valtype.generic_args.len() > 0 || valtype.id == tuple_id() {
        "".to_owned() +
            &valtype.generic_args
                .iter()
                .map(|x| {
                  mangle_simple_type(
                    crates, child_key_to_parent_uid, x)//, false)
                })
                .collect::<Vec<_>>()
                .join("__")
      } else {
        "".to_owned()
      };
  prefix + &this_part_name + &generics_part
}

// TODO: these clones could be super expensive
fn get_child_or_root_ids<'a>(
  crates: &HashMap<String, Crate>,
  context_container: Option<&SimpleValType>,
  maybe_id: &Option<&UId>
) -> Result<Vec<UId>> {
  Ok(
    if let Some(id) = maybe_id {
      get_child_ids(crates, context_container, id)?
    } else {
      crates.iter().map(|(crate_name, crate_)| {
        let item_id = &crate_.root;
        let item = crate_.index.get(item_id).unwrap();

        // Look for the thing with the crate name. It should be a
        // module with is_crate=true.
        assert!(item.name.as_ref() == Some(crate_name));
        match &item.inner {
          ItemEnum::Module(m) => {
            assert!(m.is_crate)
          },
          _ => {}
        }
        UId { crate_name: crate_name.clone(), id: item_id.clone() }
      }).collect()
      // let crate_ = crates.get(context_crate_name).unwrap();
      // crate_.index.keys()
      //     .map(|x| UId { crate_name: context_crate_name.clone(), id: x.clone() })
      //     .collect()
    })
}

// TODO: these clones could be super expensive
fn get_child_ids(
  crates: &HashMap<String, Crate>,
  context_container: Option<&SimpleValType>,
  unresolved_id: &UId
) -> Result<Vec<UId>> {
  let (resolved_id, item) =
      lookup_item(crates, context_container, &unresolved_id)?.unwrap();
  Ok(
    match &item.inner {
      ItemEnum::Module(m) => {
        m.items.iter().map(|x| UId { crate_name: resolved_id.crate_name.clone(), id: x.clone() }).collect()
      },
      ItemEnum::Struct(_) => get_concrete_impl_children(crates, &resolved_id),
      ItemEnum::Impl(impl_) => {
        impl_.items.iter().map(|x| UId { crate_name: resolved_id.crate_name.clone(), id: x.clone() }).collect()
      },
      ItemEnum::ExternCrate { .. } => unimplemented!(),
      ItemEnum::Import(_) => unimplemented!(),
      ItemEnum::Union(_) => unimplemented!(),
      ItemEnum::StructField(_) => unimplemented!(),
      ItemEnum::Enum(_) => unimplemented!(),
      ItemEnum::Variant(_) => unimplemented!(),
      ItemEnum::Function(_) => unimplemented!(),
      ItemEnum::Trait(_) => unimplemented!(),
      ItemEnum::TraitAlias(_) => unimplemented!(),
      ItemEnum::TypeAlias(type_alias) => {
        unimplemented!();
      },
      ItemEnum::OpaqueTy(_) => unimplemented!(),
      ItemEnum::Constant(_) => unimplemented!(),
      ItemEnum::Static(_) => unimplemented!(),
      ItemEnum::ForeignType => unimplemented!(),
      ItemEnum::Macro(_) => unimplemented!(),
      ItemEnum::ProcMacro(_) => unimplemented!(),
      ItemEnum::Primitive(_) => unimplemented!(),
      ItemEnum::AssocConst { .. } => unimplemented!(),
      ItemEnum::AssocType { .. } => unimplemented!(),
    })
}

fn simplify_type(
  crates: &HashMap<String, Crate>,
  // context_container: Option<&SimpleValType>,
  generics: &HashMap<String, SimpleType>,
  type_crate_name: &str,
  type_: &Type
) -> anyhow::Result<SimpleType> {
  Ok(
    match type_ {
      Type::ResolvedPath(path) => {
        let generic_args =
          if let Some(outer_args) = &path.args {
            match outer_args.deref() {
              GenericArgs::AngleBracketed { args, bindings } => {
                let mut result = Vec::new();
                for arg in args {
                  result.push(
                    simplify_generic_arg_inner(crates, generics, type_crate_name, arg)?);
                }
                result
              }
              GenericArgs::Parenthesized { inputs, output } => {
                let mut result = Vec::new();
                for arg in inputs {
                  result.push(
                    simplify_type(crates, generics, unimplemented!(), arg)?);
                }
                for output in output {
                  result.push(
                    simplify_type(crates, generics, unimplemented!(), output)?);
                }
                result
              }
            }
          } else {
            Vec::new()
          };

        // Note that this type might not really exist.
        // If we did:
        //   crates.get(type_crate_name).unwrap()
        //     .index.get(&path.id).unwrap();
        // it might panic.
        // This happens because ResolvedPath sometimes refers to an ID
        // that isn't defined in this crate.
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype:
          resolve(
            &crates,
            None,
            &UId { crate_name: type_crate_name.to_string(), id: path.id.clone() },
            generic_args)?
        }
      }
      Type::DynTrait(dynTrait) => {
        println!("what");
        unimplemented!();
      }
      Type::Generic(name) => {
        generics.get(name)
            .expect(&("Unknown generic: ".to_owned() + &name))
            .clone()
      }
      Type::BorrowedRef { lifetime, mutable, type_ } => {
        let mut thing =
            simplify_type(crates, generics, type_crate_name, type_)?;
        if *mutable {
          thing.mut_ref = true;
        } else {
          thing.imm_ref = true;
        }
        thing
      }
      Type::Primitive(name) => {
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype: SimpleValType {
            id: primitive_id(name),
            generic_args: Vec::new(),
            maybe_parent: None
          }
        }
      },
      Type::FunctionPointer(_) => unimplemented!(),
      Type::Tuple(inners) => {
        let mut generic_args = Vec::new();
        for inner in inners {
          generic_args.push(
            simplify_type(crates, generics, unimplemented!(), inner)?);
        }
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype: SimpleValType {
            id: tuple_id(),
            generic_args: generic_args,
            maybe_parent: None
          }
        }
      }
      Type::Slice(_) => unimplemented!(),
      Type::Array { .. } => unimplemented!(),
      Type::Pat { .. } => unimplemented!(),
      Type::ImplTrait(_) => unimplemented!(),
      Type::Infer => unimplemented!(),
      Type::RawPointer { .. } => unimplemented!(),
      Type::QualifiedPath { .. } => unimplemented!(),
    })
}

// fn get_type_id(
//   crates: &HashMap<String, Crate>,
//   type_crate_name: &str,
//   type_: &Type
// ) -> anyhow::Result<UId> {
//   Ok(
//     match type_ {
//       Type::ResolvedPath(path) => {
//         let resolved =
//           resolve(
//             &crates,
//             &UId { crate_name: type_crate_name.to_string(), id: path.id.clone() },
//             Vec::new())?;
//         resolved.id
//       }
//       Type::DynTrait(dynTrait) => {
//         println!("what");
//         unimplemented!();
//       }
//       Type::Generic(name) => unimplemented!(),
//       Type::BorrowedRef { lifetime, mutable, type_ } => {
//         get_type_id(crates, type_crate_name, type_)?
//       }
//       Type::Primitive(name) => primitive_id(name),
//       Type::FunctionPointer(_) => unimplemented!(),
//       Type::Tuple(inners) => tuple_id(),
//       Type::Slice(_) => unimplemented!(),
//       Type::Array { .. } => unimplemented!(),
//       Type::Pat { .. } => unimplemented!(),
//       Type::ImplTrait(_) => unimplemented!(),
//       Type::Infer => unimplemented!(),
//       Type::RawPointer { .. } => unimplemented!(),
//       Type::QualifiedPath { .. } => unimplemented!(),
//     })
// }
