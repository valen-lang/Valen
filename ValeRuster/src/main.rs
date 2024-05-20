extern crate toml;

use std::collections::{HashMap, HashSet};
use std::{fs, io};
use std::cmp::max;
use std::fmt::{Debug, Pointer};
use clap::Arg;
use std::process::Command;
use clap::ArgAction;
use std::fs::File;
use std::io::{BufRead, Read};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use anyhow::{Context, Result};
use rustdoc_types::{Crate, Enum, Function, GenericArg, GenericArgs, GenericBound, Id, Impl, Item, ItemEnum, Primitive, Struct, Type, WherePredicate};
use regex::Regex;
use crate::GenealogyKey::{ImplOrMethod, Normal};
use crate::resolve_id::lookup_uid;
use crate::ResolveError::ResolveFatal;

mod resolve;
mod resolve_id;

// TODO: optimize: All over the place we're calling .keys() and .collect()
// on some crate.index.
fn tuple_id(
  primitive_name_to_uid: &HashMap<String, UId>,
) -> UId {
  primitive_name_to_uid.get("tuple").unwrap().clone()
}
fn str_id(
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
  UId{ crate_name: "".to_string(), id: Id("life".to_string()) }
}
fn primitive_id(
  primitive_name_to_uid: &HashMap<String, UId>,
  name: &str
) -> UId {
  primitive_name_to_uid.get(name).unwrap().clone()
}

// Any other primitive can roughly be treated as a normal type
fn is_special_primitive(primitive_name_to_uid: &HashMap<String, UId>, id: &UId) -> bool {
  id == &tuple_id(primitive_name_to_uid) ||
      id == &str_id(primitive_name_to_uid) ||
      id == &slice_id(primitive_name_to_uid) ||
      id == &lifetime_id()
}
fn is_primitive(
  primitive_uid_to_name: &HashMap<UId, String>,
  id: &UId
) -> bool {
  id == &lifetime_id() ||
  primitive_uid_to_name.contains_key(id)
}


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
  maybe_parent_concrete: Option<Box<SimpleValType>>,
  // We'll need this in case we import like:
  //   #pragma rsuse std::ffi::OsString::From<&str>::from as RustOsStringFrom
  // because when we figure out from's argument's we'll need to know what the impl's <T> is.
  maybe_parent_impl: Option<Box<SimpleValType>>,
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
          .arg(Arg::new("output_dir")
              .long("output_dir")
              .help("Directory to output to.")
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

  let output_dir_path = root_matches.get_one::<String>("output_dir").unwrap();

  // This should be done before read_toml, so it can read the generated docs from it.
  setup_output_dir(&cargo_toml_path, &output_dir_path)?;

  match root_matches.subcommand() {
    Some(("list", list_matches)) => {
      let maybe_struct_name = list_matches.get_one::<String>("struct");
      let v = get_crate(&rustc_sysroot_path, &cargo_path, &output_dir_path, &crate_name)?;
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
      let mut alias_to_type: HashMap<String, SimpleValType> = HashMap::new();
      let mut type_and_original_line_and_type_str_and_maybe_alias: Vec<(SimpleType, String, Option<String>)> = Vec::new();

      let maybe_input_file_path = instantiate_matches.get_one::<String>("input_file");

      let mut crates = HashMap::new();
      crates.insert("std".to_string(), get_crate(&rustc_sysroot_path, &cargo_path, &output_dir_path, "std")?);
      crates.insert("alloc".to_string(), get_crate(&rustc_sysroot_path, &cargo_path, &output_dir_path, "alloc")?);
      crates.insert("core".to_string(), get_crate(&rustc_sysroot_path, &cargo_path, &output_dir_path, "core")?);
      get_dependency_crates(&rustc_sysroot_path, &cargo_path, &output_dir_path, &cargo_toml_path, &mut crates)?;

      let item_index = genealogize(&crates)?;

      let mut lines: Vec<String> = Vec::new();
      if let Some(input_file_path) = maybe_input_file_path {
        let file = File::open(input_file_path)?;
        let reader = io::BufReader::new(file);
        for line_res in reader.lines() {
          let line = line_res?;
          if Regex::new(r#"^#pragma\s+rsuse"#).unwrap().is_match(&line) {
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
            Regex::new(r#"^\s*(#pragma\s+rsuse\s+)?(\w+)\s+=\s+(\S.+)\s*$"#).unwrap()
                .captures(&line);
        let (target_type_str, maybe_alias) =
            if let Some(aliasing_line_captures) = maybe_aliasing_line_captures {
              // Getting the first capture group
              let maybe_alias =
                  aliasing_line_captures.get(2).map(|x| x.as_str().to_string());
              let target_type_str =
                  aliasing_line_captures.get(3)
                      .expect("Blork")
                      .as_str();
              (target_type_str, maybe_alias)
            } else {
              let simple_line_captures =
                  Regex::new(r#"^\s*(#pragma\s+rsuse\s+)?(\S.+)\s*$"#).unwrap()
                      .captures(&line)
                      .expect(&("Bad line: ".to_owned() + &line));;
              let target_type_str =
                  simple_line_captures.get(2)
                      .expect("Blork")
                      .as_str();
              (target_type_str, None)
            };

        let (target_type, _) =
            parse_type(&crates, &item_index, &alias_to_type, None, &target_type_str)?;

        if let Some(alias) = &maybe_alias {
          type_to_alias.insert(target_type.valtype.clone(), alias.clone());
          alias_to_type.insert(alias.clone(), target_type.valtype.clone());
        }
        eprintln!("Adding {:?}", target_type);
        type_and_original_line_and_type_str_and_maybe_alias.push(
          (target_type, line.clone(), maybe_alias));
      }

      let str_ref_type =
          SimpleType{
            imm_ref: true,
            mut_ref: false,
            valtype: SimpleValType {
              id: str_id(&item_index.primitive_name_to_uid),
              generic_args: Vec::new(),
              maybe_parent_concrete: None,
              maybe_parent_impl: None,
            }
          };
      if !type_to_alias.contains_key(&str_ref_type.valtype) {
        type_to_alias.insert(str_ref_type.valtype.clone(), "VR_str_ref".to_owned());
        alias_to_type.insert("VR_str_ref".to_owned(), str_ref_type.valtype.clone());
        type_and_original_line_and_type_str_and_maybe_alias.push(
          (str_ref_type.clone(), "(builtin)".to_owned(), Some("VR_str_ref".to_owned())));
      }
      let str_ref_alias = type_to_alias.get(&str_ref_type.valtype).unwrap();

      let mut cbindgen_toml_contents = String::with_capacity(1000);
      cbindgen_toml_contents += "include_guard = \"EXAMPLE_PROJECT_H\"\n";
      cbindgen_toml_contents += "include_version = true\n";
      cbindgen_toml_contents += "language = \"C\"\n";
      cbindgen_toml_contents += "cpp_compat = true\n";
      fs::write(output_dir_path.to_owned() + "/cbindgen.toml", cbindgen_toml_contents)
          .with_context(|| "Failed to write ".to_owned() + output_dir_path + "/Cargo.toml")?;

      let mut additions_for_type_to_original_line_and_type_str_and_maybe_alias: Vec<(SimpleType, String, Option<String>)> = Vec::new();

      let mut sizer_strings: Vec<String> = Vec::new();

      // TODO: Expensive call to string_path
      type_and_original_line_and_type_str_and_maybe_alias.sort_by_key(|x| x.0.valtype.id.id.0.clone());
      for (target_type, line, maybe_alias) in &type_and_original_line_and_type_str_and_maybe_alias {
        let target_valtype = &target_type.valtype;
        // let target_name: String = unimplemented!();
        let rustified_type =
            rustify_simple_valtype(
              &crates, &item_index, &target_valtype, None);

        // let path = find_item_path(&crates, target_valtype.string_path())?;
        let unresolved_id = &target_valtype.id;
        if item_index.primitive_uid_to_name.contains_key(&target_valtype.id) {
          let valtype_str =
              rustify_simple_valtype(
                &crates, &item_index, &target_valtype, None);
          let is_slice = is_dynamically_sized(&item_index.primitive_name_to_uid, &target_valtype.id);
          let type_str =
              if is_slice {
                "&".to_owned() + &valtype_str
              } else {
                valtype_str.clone()
              };
          // TODO: check duplicates, i think we're doing extra work here
          sizer_strings.push(get_sizer_string(&valtype_str, &type_str));
        } else {
          let item = resolve_id::lookup_uid(&crates, &unresolved_id);

          match &item.inner {
            ItemEnum::Struct(_) => {
              let valtype_str =
                  rustify_simple_valtype(
                    &crates, &item_index, &target_valtype, None);
              let is_slice = is_dynamically_sized(&item_index.primitive_name_to_uid, &target_valtype.id);
              let type_str =
                  if is_slice {
                    "&".to_owned() + &valtype_str
                  } else {
                    valtype_str.clone()
                  };
              // TODO: check duplicates, i think we're doing extra work here
              sizer_strings.push(get_sizer_string(&valtype_str, &type_str));
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
                    // println!("bork");
                    let generics =
                        assemble_generics(&crates, func, &target_valtype)?;

                    eprintln!("Doing things with type {:?}", target_valtype);
                    let signature_part_type =
                        simplify_type(&crates, &item_index, /*Some(target_valtype),*/ &generics, &target_valtype.id.crate_name, &type_)?;
                    let valtype_str =
                        rustify_simple_valtype(
                          &crates, &item_index, &signature_part_type.valtype, None);
                    // TODO: dedupe
                    let is_slice = is_dynamically_sized(&item_index.primitive_name_to_uid, &signature_part_type.valtype.id);
                    let type_str =
                        if is_slice {
                          "&".to_owned() + &valtype_str
                        } else {
                          valtype_str.clone()
                        };
                    eprintln!("Sizing {} from {}", &type_str, item.name.as_ref().unwrap_or(&"(none)".to_string()));
                    // TODO: check duplicates, i think we're doing extra work here
                    sizer_strings.push(get_sizer_string(&valtype_str, &type_str));

                    additions_for_type_to_original_line_and_type_str_and_maybe_alias.push(
                      (signature_part_type, "Required from ".to_owned() + &rustified_type, None));
                  }
                  _ => {}
                }
              }
            }
            _ => {}
          }
        }
      }

      // TODO: O(n^2) lol
      for (type_, original_line, maybe_alias) in additions_for_type_to_original_line_and_type_str_and_maybe_alias {
        // Check first; we don't want to overwrite it in case the user requested it and
        // perhaps even aliased it.
        if type_and_original_line_and_type_str_and_maybe_alias.iter().find(|x| x.0 == type_).is_none() {
          eprintln!("Adding {:?}", type_);
          type_and_original_line_and_type_str_and_maybe_alias.push((type_, original_line, maybe_alias));
        }
      }

      eprintln!("Running sizer program on {} types...", sizer_strings.len());

      let sizer_program_str =
          std::iter::once(common_preamble().to_owned()).into_iter()
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

      type_and_original_line_and_type_str_and_maybe_alias
          .sort_by_key(|x| x.0.valtype.id.id.0.clone());
      for (target_type, line, maybe_alias) in &type_and_original_line_and_type_str_and_maybe_alias {
        let target_valtype = &target_type.valtype;
        let target_rust_type_string =
            rustify_simple_type(
              &crates, &item_index, &target_type, None);// target_valtype.name().clone();

        // let crate_ = get_crate(&crate_name, &rustc_sysroot_path)?;
        // let path = &target_valtype.id_path();// find_item_path(&crates, target_valtype.string_path)?;
        let unresolved_id = &target_valtype.id;// path.last().unwrap();
        if is_primitive(&item_index.primitive_uid_to_name, &unresolved_id) {
          let is_dst = is_dynamically_sized(&item_index.primitive_name_to_uid, unresolved_id);
          let asking_for_ref = target_type.mut_ref || target_type.imm_ref;
          // Only instantiate if we're asking for a reference to a slice, or a non-reference to a non-slice.
          if asking_for_ref == is_dst {
            let type_str = rustify_simple_valtype(&crates, &item_index, &target_valtype, None);
            let size =
                target_type_str_to_size.get(&type_str)
                    .with_context(|| {
                      "Couldn't find size entry for struct: ".to_owned() + &type_str
                    })?
                    .clone();
            // TODO: dedupe with other call to instantiate_struct
            struct_strings.push(
              instantiate_struct(
                &crates, &item_index,
                &target_type, &maybe_alias, size)?);
          }
        } else {
          let item = resolve_id::lookup_uid(&crates, &unresolved_id);
          // if let Some((id, item)) = crate_.index.iter().find(|(id, item)| item.name.as_ref() == target_valtype.name.as_ref()) {
          match &item.inner {
            ItemEnum::Struct(_) | ItemEnum::Enum(_) | ItemEnum::TypeAlias(_) | ItemEnum::Import(_) => {
              // Skip if we're asking for an instantiation of a pointer
              if !target_type.imm_ref && !target_type.mut_ref {
                eprintln!("Instantiating type {}...", &target_rust_type_string);
                let type_str = rustify_simple_valtype(&crates, &item_index, &target_valtype, None);
                let size =
                    target_type_str_to_size.get(&type_str)
                        .with_context(|| {
                          "Couldn't find size entry for struct: ".to_owned() + &type_str
                        })?
                        .clone();
                struct_strings.push(
                  instantiate_struct(
                    &crates, &item_index,
                    &target_type, &maybe_alias, size)?);
              }
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
                    simplify_type(&crates, &item_index, /*Some(target_valtype),*/ &generics, &target_valtype.id.crate_name, param_type)?));
              }
              let maybe_return_type: Option<SimpleType> =
                  if let Some(output) = &func.decl.output {
                    eprintln!("name {:?} Generics: {:?}", item.name, generics);
                    Some(simplify_type(&crates, &item_index, /*Some(target_valtype),*/ &generics, &target_valtype.id.crate_name, &output)?)
                  } else {
                    None
                  };
              func_strings.push(
                instantiate_func(
                  &crates, &item_index,
                  &type_to_alias, &target_type, func, &maybe_alias, &params, &maybe_return_type)?);
            }
            _ => {
              unimplemented!();
            }
          }
        }
      }


      let final_program_str =
          [common_preamble().to_owned(), instantiations_preamble(str_ref_alias).to_owned()].into_iter()
              .chain(struct_strings.into_iter())
              .chain(func_strings.into_iter())
              .collect::<Vec<String>>()
              .join("\n");

      fs::write(output_dir_path.to_string() + "/src/capi.rs", final_program_str)
          .with_context(|| "Failed to write ".to_string() + output_dir_path + "/src/capi.rs")?;

      fs::write(
        output_dir_path.to_string() + "/src/lib.rs",
        r#"
#![feature(os_str_display)]

#[cfg(feature = "capi")]
mod capi;
"#)
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

fn is_dynamically_sized(
  primitive_name_to_uid: &HashMap<String, UId>,
  unresolved_id: &UId) -> bool {
  *unresolved_id == str_id(&primitive_name_to_uid, ) || *unresolved_id == slice_id(&primitive_name_to_uid, )
}

fn setup_output_dir(cargo_toml_path: &String, output_dir_path: &String) -> Result<()> {
  if !Path::new(&output_dir_path).exists() {
    fs::create_dir(&output_dir_path)
        .with_context(|| "Failed to create ".to_owned() + output_dir_path + " directory")?;
  }

  let cargo_toml_contents =
      fs::read_to_string(&cargo_toml_path)
          .with_context(|| "Failed to read Cargo toml at given path: ".to_owned() + &cargo_toml_path)?;

  fs::write(output_dir_path.to_owned() + "/Cargo.toml", cargo_toml_contents)
      .with_context(|| "Failed to write ".to_owned() + output_dir_path + "/Cargo.toml")?;

  if !Path::new(&(output_dir_path.to_string() + "/src")).exists() {
    fs::create_dir(output_dir_path.to_string() + "/src")
        .with_context(|| "Failed to create ".to_owned() + output_dir_path + "/src directory")?;
  }
  // Cargo need something in main.rs or lib.rs to even be able to *parse* the toml.
  fs::write(output_dir_path.to_string() + "/src/main.rs", "")
      .with_context(|| "Failed to write ".to_string() + output_dir_path + "/src/main.rs")?;

  Ok(())
}

fn get_dependency_crates(
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

fn common_preamble() -> &'static str {
  r#"
#![feature(os_str_display)]

use static_assertions::const_assert_eq;
use std::mem;
extern crate alloc;
use core;
use core::ffi::c_char;
"#
}

fn instantiations_preamble(str_ref_alias: &str) -> String {
  r#"
#[no_mangle]
pub extern "C" fn VR_StrFromCStr(char_ptr: *const c_char) -> VR_str_ref {
  let c_str = unsafe { core::ffi::CStr::from_ptr(char_ptr) };
  if let Ok(rust_str) = c_str.to_str() {
    let s_rs: VR_str_ref = unsafe { mem::transmute(rust_str) };
    return s_rs;
  } else {
    panic!("Error: c_str.to_str() failed.");
  }
}

// TODO: Is it okay to use u8 here instead of c_char?
#[no_mangle]
pub extern "C" fn VR_StrNew(length: usize, char_ptr: *const u8) -> VR_str_ref {
  let c_str = unsafe { std::slice::from_raw_parts(char_ptr, length) };
  if let Ok(rust_str) = core::ffi::CStr::from_bytes_with_nul(c_str) {
    if let Ok(rust_str) = rust_str.to_str() {
      let s_rs: VR_str_ref = unsafe { mem::transmute(rust_str) };
      return s_rs;
    } else {
      panic!("Error: c_str.to_str() failed.");
    }
  } else {
    panic!("Error: CStr::from_bytes_with_nul() failed.");
  }
}

#[no_mangle]
pub extern "C" fn VR_StrToCStr(str_c: VR_str_ref) -> *const c_char {
  let str_rs: &str = unsafe { mem::transmute(str_c) };
  let ptr = str_rs.as_ptr() as *const c_char;
  return ptr;
}

#[no_mangle]
pub extern "C" fn VR_StrLen(str_c: VR_str_ref) -> usize {
  let str_rs: &str = unsafe { mem::transmute(str_c) };
  let len: usize = str_rs.len();
  return len;
}

"#.replace("VR_str_ref", str_ref_alias)
}

struct ItemIndex {
  child_key_to_parent_uid: HashMap<GenealogyKey, UId>,
  primitive_name_to_uid: HashMap<String, UId>,
  primitive_uid_to_name: HashMap<UId, String>
}

fn genealogize(
  crates: &HashMap<String, Crate>
) -> Result<ItemIndex> {
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

  let mut importee_uid_to_imports: HashMap<UId, HashSet<UId>> = HashMap::new();

  for (crate_name, crate_) in crates {
    for (self_id, item) in &crate_.index {
      let self_uid =
          UId { crate_name: crate_name.to_string(), id: self_id.clone() };
      let child_uids =
          match &item.inner {
            ItemEnum::Module(_) => {
              get_direct_child_uids(crates, &primitive_name_to_uid, &self_uid)?
                  .into_iter()
                  .map(|x| GenealogyKey::Normal(x.clone()))
                  .collect::<Vec<GenealogyKey>>()
            },
            ItemEnum::Primitive(Primitive { impls: impl_ids, .. }) |
            ItemEnum::Struct(Struct { impls: impl_ids, .. }) |
            ItemEnum::Enum(Enum { impls: impl_ids, .. }) => {
              let mut result = Vec::new();
              for impl_uid in get_direct_child_uids(crates, &primitive_name_to_uid, &self_uid)? {
                result.push(GenealogyKey::ImplOrMethod { struct_uid: self_uid.clone(), child_uid: impl_uid.clone() });
              }
              // Now add all the impls' children.
              for impl_id in impl_ids {
                let impl_uid = UId { crate_name: crate_name.clone(), id: impl_id.clone() };
                for method_uid in get_direct_child_uids(crates, &primitive_name_to_uid, &impl_uid)? {
                  result.push(GenealogyKey::ImplOrMethod { struct_uid: self_uid.clone(), child_uid: method_uid.clone() });
                }
              }
              result
            },
            ItemEnum::Import(import) => {
              if let Some(imported_id) = import.id.as_ref() {
                let importee_uid =
                    UId { crate_name: crate_name.clone(), id: imported_id.clone() };
                if !importee_uid_to_imports.contains_key(&importee_uid) {
                  importee_uid_to_imports.insert(importee_uid.clone(), HashSet::new());
                }
                eprintln!("Noting importee {:?} imported by import {:?}", importee_uid, self_uid);
                let set = importee_uid_to_imports.get_mut(&importee_uid).unwrap().insert(self_uid);
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
        ItemEnum::Module(_) | ItemEnum::Struct(_) | ItemEnum::Enum(_) | ItemEnum::Primitive(_) => {
          let item_key = Normal(item_uid.clone());
          if !child_key_to_parent_uid.contains_key(&item_key) {
            if let Some(unnarrowed_imports) = importee_uid_to_imports.get(item_key.uid()) {
              let imports: Vec<UId> =
                if unnarrowed_imports.len() == 0 {
                  eprintln!("No owners or imports for {:?}", item_key);
                  continue;
                } else if unnarrowed_imports.len() > 1 {
                  // Narrow it down by the smallest path. For example, LineWriter
                  // is imported in two places:
                  //   "library/std/src/io/buffered/mod.rs",
                  //   "library/std/src/io/mod.rs",
                  // so we'll go with the second one.
                  // TODO: use a better resolution approach
                  let mut bork: Vec<(UId, &Item)> =
                      unnarrowed_imports.iter()
                          .map(|id| (id.clone(), lookup_uid(crates, id)))
                          .collect::<Vec<_>>();
                  bork.sort_by_key(|(uid, item)| item.span.as_ref().unwrap().filename.to_str().as_ref().unwrap().len());
                  eprintln!("Heuristic, estimating owner for {:?} is {:?}", item_key, bork.iter().next().unwrap().clone());
                  bork.into_iter().map(|x| x.0).collect()
                } else {
                  unnarrowed_imports.iter().map(|x| x.clone()).collect()
                };
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
  return Ok(
    ItemIndex {
      child_key_to_parent_uid,
      primitive_uid_to_name,
      primitive_name_to_uid
    })
}

fn assemble_generics(
  crates: &HashMap<String, Crate>,
  func: &Function,
  target_valtype: &SimpleValType
) -> Result<HashMap<String, SimpleType>> {
  let mut generics: HashMap<String, SimpleType> = HashMap::new();

  // Do the parents' first, in case the more local scopes want to override
  // some generic param name.
  if let Some(parent) = &target_valtype.maybe_parent_concrete {
    generics.insert(
      "Self".to_owned(),
      SimpleType {
        imm_ref: false,
        mut_ref: false,
        valtype: (**parent).clone()
      });
    // For example,
    //   std::vec::Vec<&std::ffi::OsString>::push
    // needs to know its containing struct's <T>.
    let parent_concrete_item = resolve_id::lookup_uid(crates, &parent.id);
    let empty = Vec::new();
    let parent_concrete_generic_params =
      match &parent_concrete_item.inner {
        ItemEnum::Struct(struct_) => &struct_.generics.params,
        ItemEnum::Enum(enum_) => &enum_.generics.params,
        ItemEnum::Primitive(prim) => &empty,
        _ => panic!("parent id not referring to a concrete?"),
      };
    for (generic_param, generic_arg_type) in parent_concrete_generic_params.iter().zip(&parent.generic_args) {
      // println!("Got generic arg: {:?}", generic_arg_type);
      generics.insert(generic_param.name.to_string(), generic_arg_type.clone());
    }
  }
  if let Some(parent) = &target_valtype.maybe_parent_impl {
    let parent_impl_item = resolve_id::lookup_uid(crates, &parent.id);
    let parent_impl_generic_params =
        match &parent_impl_item.inner {
          ItemEnum::Impl(impl_) => &impl_.generics.params,
          _ => panic!("parent impl id not referring to an impl?"),
        };
    // TODO: maybe dedup with instantiate_func's
    for (generic_param, generic_arg_type) in parent_impl_generic_params.iter().zip(&parent.generic_args) {
      // println!("Got generic arg: {:?}", generic_arg_type);
      generics.insert(generic_param.name.to_string(), generic_arg_type.clone());
    }
  }

  // TODO: dedup with instantiate_func's
  for (generic_param, generic_arg_type) in func.generics.params.iter().zip(&target_valtype.generic_args) {
    // println!("Got generic arg: {:?}", generic_arg_type);
    generics.insert(generic_param.name.to_string(), generic_arg_type.clone());
  }

  Ok(generics)
}

enum ResolveError {
  NotFound,
  ResolveFatal(anyhow::Error)
}

// If successful match, returns a height score and the deduced generics.
fn impl_from_matches_generic_args(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  impl_: &Impl,
  generics_args: &Vec<SimpleType>
) -> Option<(i64, Vec<SimpleType>)> {
  let empty = Vec::new();
  let impl_generic_params =
      if let Some(trait_) = &impl_.trait_ {
        if let Some(args) = &trait_.args {
          match args.as_ref() {
            GenericArgs::AngleBracketed { args: args, .. } => {
              args
            }
            GenericArgs::Parenthesized { .. } => unimplemented!(),
          }
        } else {
          &empty
        }
      } else {
        &empty
      };
  if generics_args.len() > impl_generic_params.len() {
    // TODO: Resultify
    panic!("Too many generic args!");
  }
  let mut generics_map: HashMap<String, SimpleType> = HashMap::new();


  let mut highest_height_score: i64 = 0;
  // This may ignore excess impl_generic_params, that's fine.
  for (generic_arg, generic_param) in generics_args.into_iter().zip(impl_generic_params) {
    match generic_param {
      GenericArg::Lifetime(_) => unimplemented!(),
      GenericArg::Const(_) => unimplemented!(),
      GenericArg::Infer => unimplemented!(),
      GenericArg::Type(type_) => {
        if let Some(height_score) = match_generic_arg_type(crates, &item_index, &mut generics_map, generic_arg, type_, 1) {
          highest_height_score = max(highest_height_score, height_score)
        } else {
          return None
        }
      }
    }
  }

  let mut results = Vec::new();
  for generic_param in &impl_.generics.params {
    match generics_map.get(&generic_param.name) {
      None => unimplemented!(),
      Some(generic_arg) => {
        results.push(generic_arg.clone())
      }
    }
  }
  Some((highest_height_score, results))
}

fn match_generic_arg_type(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  generics: &mut HashMap<String, SimpleType>,
  generic_arg: &SimpleType,
  generic_param: &Type,
  current_height: i64
) -> Option<i64> {
  eprintln!("arg {:?} param {:?}", generic_arg, generic_param);
  match (generic_arg, generic_param) {
    (_, Type::Generic(generic_param_name)) => {
      if let Some(existing) = generics.get(generic_param_name) {
        assert!(existing == generic_arg); // TODO: do result or something?
      } else {
        generics.insert(generic_param_name.clone(), generic_arg.clone());
      }
      Some(current_height)
    }
    (
      SimpleType { imm_ref: true, valtype: inner_arg, ..},
      Type::BorrowedRef { mutable: false, type_: inner_param, .. }
    ) => {
      match_generic_arg_valtype(crates, item_index, generics, inner_arg, inner_param, current_height + 1)
    }
    (
      SimpleType { mut_ref: true, valtype: inner_arg, ..},
      Type::BorrowedRef { mutable: true, type_: inner_param, .. }
    ) => {
      match_generic_arg_valtype(crates, item_index, generics, inner_arg, inner_param, current_height + 1)
    }
    (
      SimpleType { valtype: inner_arg, ..},
      other_param
    ) => {
      match_generic_arg_valtype(
        crates, &item_index, generics, inner_arg, other_param, current_height + 1)
    }
  }
}

fn match_generic_arg_valtype(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  generics: &mut HashMap<String, SimpleType>,
  generic_arg: &SimpleValType,
  generic_param: &Type,
  current_height: i64,
) -> Option<i64> {
  match (generic_arg, generic_param) {
    (_, _) if is_primitive(&item_index.primitive_uid_to_name, &generic_arg.id) => {
      if let Type::Primitive(generic_param_primitive_name) = generic_param {
        if item_index.primitive_uid_to_name.get(&generic_arg.id).unwrap() == generic_param_primitive_name {
          Some(current_height)
        } else {
          None
        }
      } else {
        None
      }
    }
    (_, Type::Primitive(generic_param_primitive_name)) => {
      if is_primitive(&item_index.primitive_uid_to_name, &generic_arg.id) &&
          item_index.primitive_uid_to_name.get(&generic_arg.id).unwrap() == generic_param_primitive_name {
        Some(current_height)
      } else {
        None
      }
    }
    (_, Type::Generic(generic_param_name)) => {
      generics.insert(
        generic_param_name.clone(),
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype: generic_arg.clone()
        });
      Some(current_height)
    }
    (
      _,
      Type::ResolvedPath(rustdoc_types::Path { name: generic_param_name, args: generic_params, .. })
    ) => {
      if is_primitive(&item_index.primitive_uid_to_name, &generic_arg.id) {
        return None;
      }
      if &lookup_name(crates, generic_arg) == generic_param_name {
        unimplemented!();
      } else {
        None
      }
    }
    _ => unimplemented!()
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
    let found_child = resolve_id::lookup_uid(crates, found_child_uid);
    if !matches!(found_child.inner, ItemEnum::Macro(_)) {
      narrowed_found_child_ids.push(found_child_uid.clone());
    }
  }
  return narrowed_found_child_ids;
}

fn item_has_name(direct_child_item: &&Item, name: &str) -> bool {
  match &direct_child_item.inner {
    ItemEnum::Import(import) => {
      import.name == name
    }
    // When we import e.g.
    //   std::string::String::From<&str>::from as RustStringFromStrRef
    // we need to search for the "From" impl, thats what we're doing here.
    ItemEnum::Impl(impl_) => {
      impl_.trait_.as_ref().map(|x| &x.name[..]) == Some(name)
    }
    _ => {
      direct_child_item.name.as_ref().map(|x| &x[..]) == Some(name)
    }
  }
}

fn get_direct_child_uids(
  crates: &HashMap<String, Crate>,
  primitive_name_to_uid: &HashMap<String, UId>,
  container_id: &UId,
) -> Result<Vec<UId>> {
  let container_item =
      crates
          .get(&container_id.crate_name).unwrap()
          .index.get(&container_id.id).unwrap();
  match &container_item.inner {
    ItemEnum::Module(m) => {
      Ok(
        m.items.iter()
            .map(|x| UId { crate_name: container_id.crate_name.clone(), id: x.clone() })
            .collect())
    },
    ItemEnum::Struct(_) | ItemEnum::Enum(_) | ItemEnum::Primitive(_) => {
      // TODO: optimize: get_concrete_impls_children is repeating get_concrete_impls's work
      let mut result = Vec::new();
      for thing in get_concrete_impls(crates, primitive_name_to_uid, &container_id) {
        result.push(thing);
      }
      for children in get_concrete_impls_children(crates, &primitive_name_to_uid, &container_id)? {
        result.push(children);
      }
      Ok(result)
    }
    ItemEnum::Impl(impl_) => {
      Ok(impl_.items.iter().map(|x| UId { crate_name: container_id.crate_name.clone(), id: x.clone() }).collect())
    }
    _ => unimplemented!()
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
              eprintln!("Primitive not found: {}", name);
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

// TODO: optimize: super expensive
// TODO: look for impls in other crates
// A concrete is a struct or an enum
fn get_concrete_impls_children(
  crates: &HashMap<String, Crate>,
  primitive_name_to_uid: &HashMap<String, UId>,
  concrete_id: &UId
) -> Result<Vec<UId>> {
  let mut result = Vec::new();
  for impl_uid in get_concrete_impls(crates, primitive_name_to_uid, concrete_id) {
    let item = resolve_id::lookup_uid(crates, &impl_uid);
    if let ItemEnum::Impl(impl_) = &item.inner {
      if let Some(name) = &item.name {
        if name == "Any" {
          // Other crates seem to reference core::any::Any but don't have a path to it.
          // If this becomes a problem with a lot of other things we might have to make
          // lookup_uid return an optional.
          continue;
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
              .map(|x| UId { crate_name: concrete_id.crate_name.clone(), id: x.clone() })
              .collect();

      let mut impl_methods_names = HashSet::new();
      for impl_method_uid_unresolved in &impl_method_uids {
        let impl_method_item =
            resolve_id::lookup_uid(crates, &impl_method_uid_unresolved);
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
            match resolve_id::resolve_uid(crates, &primitive_name_to_uid, &trait_unresolved_uid) {
              Ok(trait_uid) => trait_uid,
              Err(ResolveError::NotFound) => {
                resolve_id::resolve_uid(crates, &primitive_name_to_uid, &trait_unresolved_uid);
                unimplemented!();
              }
              Err(ResolveError::ResolveFatal(fatal)) => {
                return Err(fatal)
              }
            };
        let trait_item = resolve_id::lookup_uid(crates, &trait_uid);
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
          let trait_child_item = resolve_id::lookup_uid(crates, &trait_child_uid);
          if let Some(name) = &trait_child_item.name {
            if needed_names.contains(name) {
              impl_method_uids.push(trait_child_uid);
            }
          }
        }

      }

      result.append(&mut impl_method_uids);
    } else {
      panic!("Impl item id isn't impl.");
    }
  }
  return Ok(result);
}

fn get_sizer_string(valtype_str: &str, needle_full_name: &str) -> String {
  let mut rust_src = String::with_capacity(1000);
  rust_src += "  println!(\"";
  rust_src += valtype_str;
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
  item_index: &ItemIndex,
  needle_type: &SimpleType,
  maybe_alias: &Option<String>,
  size: usize
) -> anyhow::Result<String> {
  let default_name =
      mangle_simple_valtype(
        crates, item_index, &needle_type.valtype, true);
  let as_ = maybe_alias.as_ref().unwrap_or(&default_name);


  let mut builder = String::with_capacity(1000);
  builder += "#[repr(C)]\n";
  builder += "pub struct ";
  builder += as_;
  builder += " ([u8; ";
  builder += &size.to_string();
  builder += "]);\n";
  builder += "const_assert_eq!(std::mem::size_of::<";
  builder += &rustify_simple_type(&crates, &item_index, &needle_type, None);
  builder += ">(), ";
  builder += &size.to_string();
  builder += ");\n";

  return Ok(builder);
}

fn instantiate_func(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  // crate_name: &str,
  // crates: &HashMap<String, Crate>,
  //
  aliases: &HashMap<SimpleValType, String>,
  // needle_full_name_str: &str,
  needle_type: &SimpleType,
  // item: &Item,
  // context_container: Option<&SimpleValType>,
  func: &Function,
  maybe_alias: &Option<String>,
  params: &Vec<(&String, SimpleType)>,
  maybe_output_type: &Option<SimpleType>
  // c_folder: &str,
  // rust_folder: &str
) -> anyhow::Result<String> {
  let default_name =
      mangle_simple_type(
        crates,
        item_index,
        &needle_type,
        true);
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
    rust_builder += &crustify_simple_type(crates, item_index, aliases, &param_type, true);
    rust_builder += ",\n";
  }
  rust_builder += ")";
  if let Some(return_type_simple) = &maybe_output_type {
    rust_builder += " -> ";
    rust_builder += &crustify_simple_type(crates, item_index, aliases, &return_type_simple, true);
  }
  rust_builder += " {\n";
  for (param_name, param_type) in params {
    rust_builder += "  const_assert_eq!(std::mem::size_of::<";
    rust_builder += &rustify_simple_type(&crates, &item_index, &param_type, None);
    rust_builder += ">(), std::mem::size_of::<";
    rust_builder += &crustify_simple_type(crates, item_index, aliases, &param_type, true);
    rust_builder += ">());\n";

    rust_builder += "  let ";
    rust_builder += param_name;
    rust_builder += "_rs: ";
    rust_builder += &rustify_simple_type(&crates, &item_index, &param_type, None);
    rust_builder += " = unsafe { mem::transmute(";
    rust_builder += param_name;
    rust_builder += "_c) };\n";

  }

  rust_builder += "  ";
  if let Some(return_type_simple) = maybe_output_type {
    rust_builder += "let result_rs: ";
    rust_builder += &rustify_simple_type(&crates, &item_index, &return_type_simple, None);
    rust_builder += " = ";
  }
  rust_builder += &rustify_simple_type(&crates, &item_index, needle_type, Some(func));
  rust_builder += "(";
  for (param_name, param_type) in params {
    rust_builder += param_name;
    rust_builder += "_rs,";
  }
  rust_builder += ");\n";

  if let Some(return_type_simple) = &maybe_output_type {
    rust_builder += "  const_assert_eq!(std::mem::size_of::<";
    rust_builder += &rustify_simple_type(&crates, &item_index, &return_type_simple, None);
    rust_builder += ">(), std::mem::size_of::<";
    rust_builder += &crustify_simple_type(crates, item_index, aliases, &return_type_simple, true);
    rust_builder += ">());\n";

    rust_builder += "  let result_c: ";
    rust_builder += &crustify_simple_type(crates, item_index, aliases, &return_type_simple, true);
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

fn get_crate(rustc_sysroot_path: &str, cargo_path: &str, output_dir_path: &str, crate_name: &str) -> Result<Crate, anyhow::Error> {
  let json =
      if let Some(crate_json) = get_stdlib_json(&rustc_sysroot_path, crate_name)? {
        crate_json
      } else {
        get_dependency_json(cargo_path, output_dir_path, crate_name)?
      };
  let v: Crate = serde_json::from_str(&json)?;
  Ok(v)
}

fn print_function(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
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
                 Some(self_type) => mangle_simple_type(crates, item_index, &self_type, false) + "_"
               }) +
               &item.clone().name.unwrap_or("(none)".to_string()));

  for (name, input) in &func.decl.inputs {
    println!("  {:?}: {:?}", name, mangle_simple_type(
      crates,
      item_index,
      &simplify_type(
        crates, /*context_container,*/ &item_index, generics, crate_name, &input)?,
    true));
  }

  Ok(())
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
  primitive_name_to_uid: &ItemIndex,
  alias_to_type: &HashMap<String, SimpleValType>,
  context_container: Option<&SimpleValType>,
  original: &'a str
) -> anyhow::Result<(SimpleType, &'a str)> {
  let mut rest = original;

  // TODO: prevent parsing 'staticky or something
  if rest.starts_with("'static") {
    rest = &rest["'static".len()..].trim();
    return Ok(
      (
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype: SimpleValType {
            id: lifetime_id(),
            generic_args: Vec::new(),
            maybe_parent_concrete: None,
            maybe_parent_impl: None
          }
        },
        rest));
  }

  let has_mut_ref = rest.starts_with("&mut"); // TODO: maybe handle &
  if has_mut_ref {
    rest = &rest["&mut".len()..].trim();
  }
  let has_imm_ref = rest.starts_with("&"); // TODO: maybe handle &
  if has_imm_ref {
    rest = &rest[1..].trim();
  }

  let (path, new_rest) =
      parse_valtype(crates, primitive_name_to_uid, &alias_to_type, context_container, rest)?;
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

fn parse_valtype<'a>(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  alias_to_type: &HashMap<String, SimpleValType>,
  original_context_container: Option<&SimpleValType>,
  original: &'a str
) -> anyhow::Result<(SimpleValType, &'a str)> {
  let mut rest = original;
  let mut current =
      original_context_container.map(|x| x.clone());

  let re = Regex::new(r"(>|\)|$)").unwrap();
  let name_end =
      if let Some(generic_name_match) = re.find(&rest) {
        generic_name_match.start()
      } else {
        rest.len()
      };
  let full_name_preview = &rest[0..name_end];
  if let Some(uid) = item_index.primitive_name_to_uid.get(full_name_preview) {
    rest = &rest[full_name_preview.len()..].trim();

    return Ok(
      (
        SimpleValType {
          id: uid.clone(),
          generic_args: Vec::new(),
          maybe_parent_concrete: None,
          maybe_parent_impl: None
        },
        rest));
  }

  loop {
    let (new_steps, new_rest) =
        parse_type_path_step(crates, item_index, &alias_to_type, current.as_ref(), rest)?;
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
  item_index: &ItemIndex,
  alias_to_type: &HashMap<String, SimpleValType>,
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

  if let Some(aliased_type) = alias_to_type.get(name) {
    return Ok((aliased_type.clone(), rest));
  }

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
        let (generic_arg, new_rest) = parse_type(crates, item_index, &alias_to_type, None, rest)?;
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
      match resolve::extend_and_resolve(crates, item_index, context_container, &name, generic_args.clone()) {
        Ok(x) => x,
        Err(ResolveError::NotFound) => {
          resolve::extend_and_resolve(crates, item_index, context_container, &name, generic_args);
          let error = format!("Couldn't find {}", name);
          return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, error)));
          // unimplemented!()
        }
        Err(ResolveFatal(e)) => return Err(e)
      };

  Ok((new_path, rest))
}

fn simplify_generic_arg_inner(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
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
            maybe_parent_concrete: None,
            maybe_parent_impl: None
          }
        })
    },
    GenericArg::Type(type_) => Ok(simplify_type(crates, &item_index, generics, arg_crate_name, type_)?),
    GenericArg::Const(_) => unimplemented!(),
    GenericArg::Infer => unimplemented!(),
  }
}

fn mangle_simple_type(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  original_type: &SimpleType,
  // If this is the top one, then refs should be come pointers.
  // Otherwise, not.
  // For example, a param:
  //   &std::vec::Vec<&std::ffi::OsString>
  // should become:
  //   1std_vec_Vec__Ref__std_ffi_OsString*
  is_root: bool
) -> String {
  let mut type_ = original_type.clone();
  let has_pointer = type_.imm_ref || type_.mut_ref;
  let unpointered =
    if is_primitive(&item_index.primitive_uid_to_name, &original_type.valtype.id) {
      if is_special_primitive(&item_index.primitive_name_to_uid, &original_type.valtype.id) {
        if original_type.valtype.id == tuple_id(&item_index.primitive_name_to_uid, ) {
          mangle_simple_valtype(crates, item_index, &type_.valtype, is_root)
        } else if original_type.valtype.id == slice_id(&item_index.primitive_name_to_uid, ) {
          mangle_simple_valtype(crates, item_index, &type_.valtype, is_root)
        } else if original_type.valtype.id == str_id(&item_index.primitive_name_to_uid, ) {
          "str_ref".to_owned()
        } else if original_type.valtype.id == lifetime_id() {
          "L".to_owned()
        } else {
          unimplemented!();
        }
      } else {
        unimplemented!();
        // Then it's a primitive
        original_type.valtype.id.id.0.clone()
      }
    } else {
      mangle_simple_valtype(crates, item_index, &type_.valtype, is_root)
    };
  (if is_root { "VR_" } else { "" }).to_string() +
      &(if has_pointer {
        if is_root {
          unpointered + "*"
        } else {
          (if type_.imm_ref { "SRef__" } else { "URef__" }).to_string() + &unpointered
        }
      } else {
        unpointered
      })
}

fn crustify_simple_type(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  aliases: &HashMap<SimpleValType, String>,
  type_: &SimpleType,
  is_root: bool // TODO: see if you can take this out and have an outer function instead
) -> String {
  let valtype = &type_.valtype;
  let is_slice = is_dynamically_sized(&item_index.primitive_name_to_uid, &type_.valtype.id);
  (if is_slice {
    // Then it's going to be a struct, not a pointer.
    "".to_owned()
  } else {
    (if type_.imm_ref { "*const ".to_owned() }
    else if type_.mut_ref { "*mut ".to_owned() }
    else { "".to_owned() })
  }) +
      &(if let Some(alias) = aliases.get(&type_.valtype) {
        alias.to_owned()
      } else {
        if is_primitive(&item_index.primitive_uid_to_name, &type_.valtype.id) {
          // It's a primitive
          mangle_simple_valtype(crates, item_index, &valtype, is_root)
        } else {
          mangle_simple_valtype(crates, item_index, &valtype, is_root)
        }
      })
}

fn rustify_simple_valtype(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  valtype: &SimpleValType,
  maybe_func: Option<&Function>
) -> String {
  let prefix_and_name =
      if is_primitive(&item_index.primitive_uid_to_name, &valtype.id) { // Then it's a primitive
        if is_special_primitive(&item_index.primitive_name_to_uid, &valtype.id) {
          if valtype.id == tuple_id(&item_index.primitive_name_to_uid) || valtype.id == slice_id(&item_index.primitive_name_to_uid, ) {
            "".to_owned()
          } else if valtype.id == str_id(&item_index.primitive_name_to_uid) {
            "str".to_owned()
          } else if valtype.id == lifetime_id() {
            "'static".to_owned()
          } else {
            unimplemented!();
          }
        } else {
          return item_index.primitive_uid_to_name.get(&valtype.id).unwrap().to_string()
        }
      } else {
        let name = lookup_name(crates, &valtype);

        let key =
            match &valtype.maybe_parent_concrete {
              None => Normal(valtype.id.clone()),
              Some(parent) => {
                ImplOrMethod { struct_uid: parent.id.clone(), child_uid: valtype.id.clone() }
              }
            };
        let prefix =
            if let Some(parent_uid) = item_index.child_key_to_parent_uid.get(&key) {
              rustify_simple_valtype(
                crates,
                item_index,
                &SimpleValType {
                  id: parent_uid.clone(),
                  generic_args: Vec::new(),
                  maybe_parent_concrete: None,
                  maybe_parent_impl: None
                },
                maybe_func) + "::"
            } else {
              "".to_string()
            };

        prefix.to_string() + &name
      };

  let displayable_generic_args =
      if let Some(func) = maybe_func {
        // We don't want to display any impl args, like
        //   pub fn create(argv: &[impl AsRef<OsStr>], config: PopenConfig) -> Result<Popen>
        // from https://docs.rs/subprocess/latest/subprocess/struct.Popen.html#method.create
        // So lets filter them out.
        func.generics.params.iter()
            .zip(&valtype.generic_args)
            .filter(|(generic_param, generic_arg)| {
              // Yeah, this seems to be the best way to find them...
              !generic_param.name.starts_with("impl ")
            })
            .map(|(generic_param, generic_arg)| generic_arg.clone())
            .collect::<Vec<SimpleType>>()
      } else {
        valtype.generic_args.clone()
      };

  let generics_part =
      // If it's a tuple then we still want to print out the () even if
      // there's nothing inside it.
      if displayable_generic_args.len() > 0 || valtype.id == tuple_id(&item_index.primitive_name_to_uid) || valtype.id == slice_id(&item_index.primitive_name_to_uid) {
        let (start, end) =
            if valtype.id == tuple_id(&item_index.primitive_name_to_uid, ) { ("(", ")") }
            else if valtype.id == slice_id(&item_index.primitive_name_to_uid, ) { ("[", "]") }
            else if maybe_func.is_some() { ("::<", ">") }
            else { ("<", ">") };
        "".to_owned() +
            start +
            &displayable_generic_args
                .iter()
                .map(|x| {
                  rustify_simple_type(
                    crates, item_index, x, None)
                })
                .collect::<Vec<_>>()
                .join(", ") +
            end
      } else {
        "".to_owned()
      };
  prefix_and_name + &generics_part
}

fn lookup_name(crates: &HashMap<String, Crate>, valtype: &SimpleValType) -> String {
  let item = resolve_id::lookup_uid(crates, &valtype.id);
  item.name.as_ref().map(|x| &x[..]).unwrap_or("unnamed").to_string()
}

// TODO: It might be nice to get that is_func from the SimpleType...
//    not sure if that's possible though.
fn rustify_simple_type(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  type_: &SimpleType,
  maybe_func: Option<&Function>
) -> String {
  (if type_.imm_ref { "&".to_owned() }
  else if type_.mut_ref { "&mut ".to_owned() }
  else { "".to_owned() }) +
      &rustify_simple_valtype(
        &crates, &item_index,
        &type_.valtype, maybe_func)
}

fn mangle_simple_valtype(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  valtype: &SimpleValType,
  // For example if we're calling this on core::str::Chars then this would be true.
  // But when it recurses it'll be calling it on core::str and it would be false.
  is_root: bool
) -> String {
  let this_part_name =
      if is_primitive(&item_index.primitive_uid_to_name, &valtype.id) { // Then it's a primitive
        if is_special_primitive(&item_index.primitive_name_to_uid, &valtype.id) {
          if valtype.id == tuple_id(&item_index.primitive_name_to_uid) {
            "".to_owned()
          } else if valtype.id == slice_id(&item_index.primitive_name_to_uid, ) {
            "slice".to_owned()
          } else if &valtype.id == &str_id(&item_index.primitive_name_to_uid, ) {
            "str_ref".to_owned()
          } else {
            unimplemented!();
          }
        } else {
          return item_index.primitive_uid_to_name.get(&valtype.id).unwrap().to_string()
        }
      } else {
        let item =
            crates.get(&valtype.id.crate_name).unwrap()
                .index.get(&valtype.id.id).unwrap();
        item.name.as_ref().map(|x| &x[..]).unwrap_or("unnamed").to_string()
      };

  let key =
      match &valtype.maybe_parent_concrete {
        None => Normal(valtype.id.clone()),
        Some(parent) => {
          ImplOrMethod { struct_uid: parent.id.clone(), child_uid: valtype.id.clone() }
        }
      };
  let prefix =
      if let Some(parent_uid) = item_index.child_key_to_parent_uid.get(&key) {
        mangle_simple_valtype(
          crates,
          item_index,
          &SimpleValType {
            id: parent_uid.clone(),
            generic_args: Vec::new(),
            maybe_parent_concrete: None,
            maybe_parent_impl: None
          },
        false) + "_"
      } else {
        "".to_string()
      };

  let (generics_count_part, generics_part) =
      // If it's a tuple then we still want to print out the () even if
      // there's nothing inside it.
      if valtype.generic_args.len() > 0 || valtype.id == tuple_id(&item_index.primitive_name_to_uid) {
        (
            valtype.generic_args.len().to_string() + "_",
            "".to_owned() +
                &valtype.generic_args
                    .iter()
                    .map(|x| {
                      "__".to_string() +
                      &mangle_simple_type(
                        crates, item_index, x, false)//, false)
                    })
                    .collect::<Vec<_>>()
                    .join(""))
      } else {
        ("".to_string(), "".to_owned())
      };
  (if is_root { "VR_" } else { "" }).to_string() +
      &generics_count_part +
      &prefix +
      &this_part_name +
      &generics_part
}

fn simplify_type(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
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
                      simplify_generic_arg_inner(crates, &item_index, generics, type_crate_name, arg)?);
                  }
                  result
                }
                GenericArgs::Parenthesized { inputs, output } => {
                  let mut result = Vec::new();
                  for arg in inputs {
                    result.push(
                      simplify_type(crates, &item_index, generics, unimplemented!(), arg)?);
                  }
                  for output in output {
                    result.push(
                      simplify_type(crates, &item_index, generics, unimplemented!(), output)?);
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
        let result_uid =
            UId { crate_name: type_crate_name.to_string(), id: path.id.clone() };
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype:
          match resolve::resolve(&crates, item_index, None, None, &result_uid, generic_args) {
            Ok(x) => x,
            Err(ResolveError::NotFound) => {
              unimplemented!()
            }
            Err(ResolveFatal(e)) => return Err(e)
          }
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
            simplify_type(crates, &item_index, generics, type_crate_name, type_)?;
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
            id: primitive_id(&item_index.primitive_name_to_uid, name),
            generic_args: Vec::new(),
            maybe_parent_concrete: None,
            maybe_parent_impl: None
          }
        }
      },
      Type::FunctionPointer(_) => unimplemented!(),
      Type::Tuple(inners) => {
        let mut generic_args = Vec::new();
        for inner in inners {
          generic_args.push(
            simplify_type(crates, &item_index, generics, unimplemented!(), inner)?);
        }
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype: SimpleValType {
            id: tuple_id(&item_index.primitive_name_to_uid, ),
            generic_args: generic_args,
            maybe_parent_concrete: None,
            maybe_parent_impl: None
          }
        }
      }
      Type::Slice(inner) => {
        eprintln!("generics: {:?}", generics);
        eprintln!("slice inner: {:?}", inner);
        let inner_simple_type =
            simplify_type(crates, &item_index, generics, type_crate_name, inner)?;
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype: SimpleValType {
            id: slice_id(&item_index.primitive_name_to_uid, ),
            generic_args: vec![inner_simple_type],
            maybe_parent_concrete: None,
            maybe_parent_impl: None,
          }
        }
      },
      Type::Array { .. } => unimplemented!(),
      Type::Pat { .. } => unimplemented!(),
      Type::ImplTrait(generic_bounds) => {
        if generic_bounds.len() == 1 {
          let generic_bound = &generic_bounds[0];
          match generic_bound {
            GenericBound::TraitBound { trait_, generic_params, modifier } => {
              let needle_start = "impl ".to_owned() + &trait_.name;
              let matches =
                  generics.iter()
                      .filter(|(name, _)| name.starts_with(&needle_start))
                      .collect::<Vec<_>>();
              if matches.len() != 1 {
                unimplemented!();
              }
              let (_, match_) = matches[0];
              match_.clone()
            }
            GenericBound::Outlives(_) => unimplemented!(),
          }
        } else {
          unimplemented!();
        }
      }
      Type::Infer => unimplemented!(),
      Type::RawPointer { .. } => unimplemented!(),
      Type::QualifiedPath { .. } => unimplemented!(),
    })
}
