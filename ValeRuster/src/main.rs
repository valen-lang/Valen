#![allow(unused_imports)]

mod indexer;
mod resolve_id;
mod vast_types;
mod simplify;
mod resolve;

extern crate toml;

use std::collections::{HashMap, HashSet};
use std::{fs, io};
use std::cmp::max;
use std::fmt::{Debug, Pointer};
use clap::Arg;
use std::process::{Command, ExitStatus, Stdio};
use clap::ArgAction;
use std::fs::File;
use std::io::{BufRead, Read};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::ptr::replace;
use anyhow::{Context, Result};
use cargo_metadata::diagnostic::DiagnosticCode;
use cargo_metadata::Message;
use regex::Regex;
use rustdoc_types::{Crate, Function, GenericArg, GenericArgs, GenericParamDefKind, Generics, Id, Impl, Item, ItemEnum, Struct, Type};
use crate::indexer::ItemIndex;
use crate::ParsedType::ImplCast;
use crate::resolve_id::{extend_and_resolve_uid, generic_name, is_generic, is_primitive, is_special_primitive, lookup_uid, resolve_uid, str_id, tuple_id};
use crate::ResolveError::ResolveFatal;
use itertools::Itertools;
use crate::simplify::{SimpleType, SimpleValType, simplify_type, SimplifyError};
use crate::simplify::SimplifyError::Unsupported;

// Universal id.
// This can refer to anything, including type aliases, imports, etc.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct UId {
  crate_name: String,
  id: Id,
}

struct TypeInfo {
  canonical_type: ParsedFullType,
  public_type: ParsedFullType,
  c_name: String,
  size: usize,
  alignment: usize
}

struct FuncInfo {
  canonical_type: ParsedFullType,
  public_type: ParsedFullType,
  c_name: String,
  ret_type_rust_str: String,
  param_types_rust_strs: Vec<String>
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct ParsedFullType {
  steps: Vec<ParsedType>
}
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum ParsedType {
  Ref{ mutable: bool, inner: ParsedFullType },
  Value{ name: String, generic_args: Vec<ParsedFullType>, params: Vec<ParsedFullType> },
  Alias(String),
  Tuple{ generic_args: Vec<ParsedFullType> },
  Slice{ inner: ParsedFullType },
  ImplCast{ struct_: ParsedFullType, impl_: ParsedFullType },
  Primitive(String),
  Lifetime,
  Wildcard,
}

fn main() -> Result<(), anyhow::Error> {
  // _rust_2_std_collections_HashMap__i32__String
  // _rust_2_std_collections_HashMap__i32__2_std_collections_Vec__String

  let root_matches =
      clap::Command::new("ValeRuster")
          .version("1.0")
          .author("Evan Ovadia")
          .about("Creates bindings for arbitrary Rust libraries")
          .subcommand(
            clap::Command::new("list")
                .about("List all generic structs and functions"))
          .subcommand(
            clap::Command::new("instantiate")
                .about("Instantiate either a function or a struct.")
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
          .arg(Arg::new("vale_bindings_dir")
              .long("vale_bindings_dir")
              .help("Directory to output vale bindings to.")
              .action(ArgAction::Set)
              .required(true))
          .arg(Arg::new("output_dir")
              .long("output_dir")
              .help("Directory to output to.")
              .action(ArgAction::Set)
              .required(true))
          .arg(Arg::new("input_file")
              .long("input_file")
              .help("File to read from, intead of stdin.")
              .action(ArgAction::Set))
          .arg(Arg::new("type")
              .long("type")
              .help("A single dotted type path to generate bindings for (e.g. std.vec.Vec). When set, overrides --input_file and stdin.")
              .action(ArgAction::Set))
          .arg(Arg::new("output_sizes")
              .long("output_sizes")
              .help("File to output size information to.")
              .action(ArgAction::Set))
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

  let vale_bindings_dir_path = root_matches.get_one::<String>("vale_bindings_dir").unwrap();
  let output_dir_path = root_matches.get_one::<String>("output_dir").unwrap();
  let maybe_output_sizes_path = root_matches.get_one::<String>("output_sizes");

  let maybe_input_file_path = root_matches.get_one::<String>("input_file");
  let maybe_single_type = root_matches.get_one::<String>("type");

  let mut input_lines: Vec<String> = Vec::new();
  if let Some(single_type) = maybe_single_type {
    // Synthesize a single import line so the list subcommand's existing regex picks it up.
    input_lines.push(format!("import rust.{}", single_type));
  } else if let Some(input_file_path) = maybe_input_file_path {
    let file = File::open(input_file_path)?;
    let reader = io::BufReader::new(file);
    for line_res in reader.lines() {
      let line = line_res?;
      input_lines.push(line);
    }
  } else {
    for line in io::stdin().lock().lines() {
      input_lines.push(line?);
    }
  }

  // This should be done before read_toml, so it can read the generated docs from it.
  setup_output_dir(&cargo_toml_path, &output_dir_path)?;

  match root_matches.subcommand() {
    Some(("list", list_matches)) => {
      let output_dir_path = root_matches.get_one::<String>("output_dir").unwrap();

      let mut crates = HashMap::new();
      crates.insert("std".to_string(), indexer::get_crate(&rustc_sysroot_path, &cargo_path, &output_dir_path, "std")?);
      crates.insert("alloc".to_string(), indexer::get_crate(&rustc_sysroot_path, &cargo_path, &output_dir_path, "alloc")?);
      crates.insert("core".to_string(), indexer::get_crate(&rustc_sysroot_path, &cargo_path, &output_dir_path, "core")?);
      indexer::get_dependency_crates(&rustc_sysroot_path, &cargo_path, &output_dir_path, &cargo_toml_path, &mut crates)?;

      let item_index = indexer::genealogize(&crates)?;

      let mut undeduped_types_to_find: Vec<String> = Vec::new();

      if maybe_single_type.is_none() {
        if let Some(input_file_path) = maybe_input_file_path {
          if !input_file_path.ends_with(".vale") {
            panic!("Input file doesn't end with .vale!");
          }
        }
      }
      for line in input_lines {
        let line = line.trim().to_string();

        let regex = Regex::new(r#"^import\s+rust\s*\.([\w\.]+)"#).unwrap();
        if let Some(captures) = regex.captures(&line) {
          let type_str =
              captures.get(1)
                  .expect("Bad rsuse/rsfn line")
                  .as_str().to_string();
          undeduped_types_to_find.push(type_str);
        }
      }

      let types_to_find = undeduped_types_to_find.iter().unique().collect::<Vec<_>>();

      let mut desired_types_steps_and_uids = Vec::new();
      for type_to_find in types_to_find {
        let type_step_strs =
            type_to_find.split(".").into_iter().map(|x| x.to_owned()).collect::<Vec<_>>();
        let mut current: Option<UId> = None;
        for step_str in &type_step_strs {
          current = Some(
            match extend_and_resolve_uid(
              &crates, &item_index.primitive_name_to_uid, current.as_ref(), step_str) {
              Ok(x) => x,
              Err(ResolveError::NotFound) => {
                return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Not found: {}", step_str)))); // DO NOT SUBMIT
              }
              Err(ResolveError::Unsupported(reason)) => {
                return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Unsupported top-level type '{}': {}", step_str, reason))));
              }
              Err(ResolveError::ResolveFatal(fatal)) => return Err(fatal),
            });
        }
        if let Some(uid) = current {
          desired_types_steps_and_uids.push((type_step_strs, uid));
        } else {
          unimplemented!();
        }
      }

      let mut available_types_uids: HashSet<UId> =
          desired_types_steps_and_uids.iter().map(|x| x.1.clone()).collect();
      available_types_uids.insert(item_index.primitive_name_to_uid.get("usize").unwrap().clone());
      available_types_uids.insert(item_index.primitive_name_to_uid.get("i64").unwrap().clone());
      available_types_uids.insert(item_index.primitive_name_to_uid.get("i32").unwrap().clone());
      available_types_uids.insert(item_index.primitive_name_to_uid.get("bool").unwrap().clone());
      available_types_uids.insert(tuple_id(&item_index.primitive_name_to_uid));

      for (type_steps_strs, uid) in desired_types_steps_and_uids {
        let mut dest_file_path_str = type_steps_strs.join("/");
        dest_file_path_str = vale_bindings_dir_path.to_string() + "/" + &dest_file_path_str + ".vale";
        let dest_file_path = Path::new(&dest_file_path_str);
        let dest_dir_path = dest_file_path.parent().unwrap();
        fs::create_dir_all(&dest_dir_path).unwrap(); // DO NOT SUBMIT

        let item = lookup_uid(&crates, &uid);

        match &item.inner {
          ItemEnum::Struct(struct_) => {
            let s =
                match valify_concrete(&crates, &item_index, &available_types_uids, uid, item, &struct_.generics, &struct_.impls) {
                  Ok(x) => x,
                  Err(SimplifyError::Unsupported(reason)) => unimplemented!(),
                  Err(SimplifyError::SimplifyFatal(e)) => Err(e).unwrap() // DO NOT SUBMIT
                };
            println!("Writing to {:?}", dest_file_path);
            fs::write(dest_file_path, s).unwrap();
          }
          ItemEnum::Enum(enum_) => {
            let s =
                match valify_concrete(&crates, &item_index, &available_types_uids, uid, item, &enum_.generics, &enum_.impls) {
                  Ok(x) => x,
                  Err(SimplifyError::Unsupported(reason)) => unimplemented!(),
                  Err(SimplifyError::SimplifyFatal(e)) => Err(e).unwrap() // DO NOT SUBMIT
                };
            println!("Writing to {:?}", dest_file_path);
            fs::write(dest_file_path, s).unwrap();
          }
          ItemEnum::Module(_) => unimplemented!(),
          ItemEnum::ExternCrate { .. } => unimplemented!(),
          ItemEnum::Use(_) => unimplemented!(),
          ItemEnum::Union(_) => unimplemented!(),
          ItemEnum::StructField(_) => unimplemented!(),
          ItemEnum::Variant(_) => unimplemented!(),
          ItemEnum::Function(_) => unimplemented!(),
          ItemEnum::Trait(_) => unimplemented!(),
          ItemEnum::TraitAlias(_) => unimplemented!(),
          ItemEnum::Impl(_) => unimplemented!(),
          ItemEnum::TypeAlias(_) => unimplemented!(),
          ItemEnum::Constant { .. } => unimplemented!(),
          ItemEnum::Static(_) => unimplemented!(),
          ItemEnum::ExternType => unimplemented!(),
          ItemEnum::Macro(_) => unimplemented!(),
          ItemEnum::ProcMacro(_) => unimplemented!(),
          ItemEnum::Primitive(_) => unimplemented!(),
          ItemEnum::AssocConst { .. } => unimplemented!(),
          ItemEnum::AssocType { .. } => unimplemented!(),
        }

        // #!DeriveStructConstructor
        // extern struct Vec<T> imm {
        //   extern func with_capacity(capacity i64) Vec<T>;
        //   extern func capacity(self Vec<T>) i64;
        // }
      }
    }
    _ => {
      unimplemented!();
    }
  }

  // println!("{:?}", v);

  return Ok(());
}

fn valify_concrete(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  available_type_uids: &HashSet<UId>,
  struct_uid: UId,
  struct_item: &Item,
  concrete_generics: &Generics,
  concrete_impls: &Vec<Id>
) -> Result<String, SimplifyError> {
  let mut result =
      "#!DeriveStructConstructor extern struct ".to_owned() +
          &struct_item.name.as_ref().expect("Struct has no name?");
  if concrete_generics.params.len() > 0 {
    result += "<";
    for gen in &concrete_generics.params {
      match &gen.kind {
        GenericParamDefKind::Lifetime { .. } => unimplemented!(),
        GenericParamDefKind::Const { .. } => unimplemented!(),
        GenericParamDefKind::Type { bounds, default, is_synthetic } => {
          if bounds.len() > 0 {
            continue; // DO NOT SUBMIT
          }
          result += &gen.name;
        }
      }
    }
    result += ">";
  }
  result += " imm {\n"; // DO NOT SUBMIT imm

  let mut method_name_to_count: HashMap<String, usize> = HashMap::new();

  for impl_id in concrete_impls {
    let impl_uid_unresolved = UId { crate_name: struct_uid.crate_name.clone(), id: impl_id.clone() };
    let impl_uid = resolve_uid(crates, &item_index.primitive_name_to_uid, &impl_uid_unresolved).unwrap(); // DO NOT SUBMIT
    let impl_item = lookup_uid(crates, &impl_uid);
    let impl_ =
        match &impl_item.inner {
          ItemEnum::Impl(i) => i,
          _ => panic!("Impl item isn't impl!"),
        };
    for method_id in &impl_.items {
      let method_uid_unresolved =
          UId { crate_name: struct_uid.crate_name.clone(), id: method_id.clone() };
      let method_uid = resolve_uid(crates, &item_index.primitive_name_to_uid, &method_uid_unresolved).unwrap(); // DO NOT SUBMIT
      let method_item = lookup_uid(crates, &method_uid);
      match &method_item.inner {
        ItemEnum::Function(method) => {
          if let Some(name) = &method_item.name {
            *method_name_to_count.entry(name.clone()).or_default() += 1;
          }
        }
        _ => {}
      }
    }
  }
  for impl_id in concrete_impls {
    let impl_uid_unresolved = UId { crate_name: struct_uid.crate_name.clone(), id: impl_id.clone() };
    let impl_uid = resolve_uid(crates, &item_index.primitive_name_to_uid, &impl_uid_unresolved).unwrap(); // DO NOT SUBMIT
    let impl_item = lookup_uid(crates, &impl_uid);
    let impl_ =
        match &impl_item.inner {
          ItemEnum::Impl(i) => i,
          _ => panic!("Impl item isn't impl!"),
        };

    // if impl_.generics.params.len() > 0 {
    //   eprintln!(
    //     "Skipping {} impl {}: {}",
    //     struct_item.name.as_ref().unwrap_or(&"".to_owned()),
    //     impl_.trait_.as_ref().map(|x| &x.path).unwrap_or(&"".to_owned()),
    //     "Impl has generics");
    //   continue;
    // }

    match &valify_impl(crates, &item_index, &available_type_uids, &struct_uid, struct_item, concrete_generics, concrete_impls, &method_name_to_count, &impl_) {
      Ok(x) => {
        result += x;
      }
      Err(Unsupported(reason)) => {
        result +=
            &format!(
              "  // Skipping {} impl: {}\n",
              // struct_item.name.as_ref().unwrap_or(&"".to_owned()),
              impl_.trait_.as_ref().map(|x| &x.path).unwrap_or(&"".to_owned()),
              reason);
      }
      Err(SimplifyError::SimplifyFatal(err)) => Err(err).unwrap() // DO NOT SUBMIT
    }
  }
  result += "}\n";
  Ok(result)
}

fn valify_impl(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  outer_whitelist_type_uids: &HashSet<UId>,
  struct_uid: &UId,
  struct_item: &Item,
  concrete_generics: &Generics,
  concrete_impls: &Vec<Id>,
  method_name_to_count: &HashMap<String, usize>,
  impl_: &Impl
) -> Result<String, SimplifyError> {
  let mut result = "".to_owned();

  let struct_name =
    match &impl_.for_ {
      Type::ResolvedPath(path) => &path.path,
      Type::BorrowedRef { .. } => {
        return Err(Unsupported("Impl for borrowed struct unsupported".to_string()));
      },
      Type::DynTrait(_) => unimplemented!(),
      Type::Generic(_) => unimplemented!(),
      Type::Primitive(_) => unimplemented!(),
      Type::FunctionPointer(_) => unimplemented!(),
      Type::Tuple(_) => unimplemented!(),
      Type::Slice(_) => {
        return Err(Unsupported("Impl for slice unsupported".to_string()));
      }
      Type::Array { .. } => {
        return Err(Unsupported("Impl for array unsupported".to_string()));
      }
      Type::Pat { .. } => unimplemented!(),
      Type::ImplTrait(_) => unimplemented!(),
      Type::Infer => unimplemented!(),
      Type::RawPointer { .. } => unimplemented!(),
      Type::QualifiedPath { .. } => unimplemented!(),
    };
  let trait_name =
    match &impl_.trait_ {
      None => "(self)",
      Some(trait_) => &trait_.path,
    };
  eprintln!("Considering {} impl {}...", struct_name, trait_name);

  if impl_.blanket_impl.is_some() {
    return Err(Unsupported("Blanket impls unsupported".to_owned()));
  }

  // DO NOT SUBMIT take out, simplify
  // let names_and_generic_simple_types =
  let impl_struct_arg_impl_runes =
      match &impl_.for_ {
        Type::ResolvedPath(path) => {
          if path.id != struct_uid.id {
            return Err(Unsupported("Impl for non-struct types unsupported".to_owned()));
          }
          let empty = Vec::new();
          let generic_args =
              path.args.as_ref()
                  .map(|x| {
                    match &**x {
                      GenericArgs::AngleBracketed { args, .. } => args,
                      GenericArgs::Parenthesized { .. } => unimplemented!(),
                      GenericArgs::ReturnTypeNotation => unimplemented!(),
                    }
                  })
                  .unwrap_or(&empty);
          let mut result = vec![];
          for generic_arg in generic_args {
            result.push(
              match generic_arg {
                GenericArg::Lifetime(_) => unimplemented!(),
                GenericArg::Const(_) => unimplemented!(),
                GenericArg::Infer => unimplemented!(),
                GenericArg::Type(type_) => {
                  match type_ {
                    Type::Generic(rune) => {
                      match rune.as_str() {
                        "$" | "" => unimplemented!(),
                        _ => rune.clone()
                      }
                    },
                    Type::Array { .. } => {
                      return Err(Unsupported("Encountered generic arg Array".to_owned()))
                    }
                    Type::ResolvedPath(path) => {
                      return Err(Unsupported(format!("Encountered generic arg nested ResolvedPath {}", path.path)))
                    }
                    Type::DynTrait(_) => unimplemented!(),
                    Type::Primitive(_) => {
                      return Err(Unsupported("Encountered const generic".to_owned()))
                    },
                    Type::FunctionPointer(_) => unimplemented!(),
                    Type::Tuple(_) => {
                      return Err(Unsupported("Encountered generic arg tuple".to_owned()))
                    }
                    Type::Slice(_) => unimplemented!(),
                    Type::Pat { .. } => unimplemented!(),
                    Type::ImplTrait(_) => unimplemented!(),
                    Type::Infer => unimplemented!(),
                    Type::RawPointer { .. } => unimplemented!(),
                    Type::BorrowedRef { .. } => {
                      return Err(Unsupported("Encountered generic arg borrow ref".to_owned()))
                    }
                    Type::QualifiedPath { .. } => unimplemented!(),
                  }
                },
              });
          }
          result
        }
        _ => unimplemented!(),
      };

  // DO NOT SUBMIT take out, simplify
  let mut defaulted_generic_runes: HashSet<String> = HashSet::new();
  let mut names_and_generic_simple_types = vec![];
  for (struct_generic_param, impl_struct_arg_impl_rune) in
      concrete_generics.params.iter().zip(&impl_struct_arg_impl_runes) {
    match struct_generic_param.name.as_str() {
      "$" | "" => unimplemented!(),
      _ => {} // proceed
    }
    match impl_struct_arg_impl_rune.as_str() {
      "$" | "" => unimplemented!(),
      _ => {} // proceed
    }

    let has_default =
      match &struct_generic_param.kind {
        GenericParamDefKind::Lifetime { .. } => unimplemented!(),
        GenericParamDefKind::Const { .. } => unimplemented!(),
        GenericParamDefKind::Type { bounds, default, is_synthetic } => {
          default.is_some()
        }
      };
    if has_default {
      defaulted_generic_runes.insert(impl_struct_arg_impl_rune.to_owned());
    } else {
      names_and_generic_simple_types.push((
        impl_struct_arg_impl_rune,
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype: SimpleValType {
            id: generic_id(struct_generic_param.name.clone()),
            generic_args: Vec::new(),
            maybe_parent_impl: None,
            maybe_parent_concrete: None
          }
        }));
    }
  }
  if impl_struct_arg_impl_runes.len() < impl_.generics.params.len() {
    return Err(Unsupported("Impl has more args than used in struct".to_owned()))
  }

  let mut generics: HashMap<String, SimpleType> = HashMap::new();
  for (a, b) in &names_and_generic_simple_types {
    generics.insert((*a).to_owned(), b.clone());
  }
  let self_valtype =
      SimpleValType {
        id: struct_uid.clone(),
        generic_args: names_and_generic_simple_types.into_iter().map(|(a, b)| b).collect(),
        maybe_parent_concrete: None,
        maybe_parent_impl: None
      };
  generics.insert(
    "Self".to_owned(),
    SimpleType {
      imm_ref: false,
      mut_ref: false,
      valtype: self_valtype
    });
  let mut whitelist_type_ids = outer_whitelist_type_uids.clone();
  for (_, generic) in &generics {
    whitelist_type_ids.insert(generic.valtype.id.clone());
  }

  for method_id in &impl_.items {
    let method_uid_unresolved =
        UId { crate_name: struct_uid.crate_name.clone(), id: method_id.clone() };
    let method_uid = resolve_uid(crates, &item_index.primitive_name_to_uid, &method_uid_unresolved).unwrap(); // DO NOT SUBMIT
    let method_item = lookup_uid(crates, &method_uid);
    match &method_item.inner {
      ItemEnum::Function(method) => {
        match valify_method(crates, item_index, &method_name_to_count, &defaulted_generic_runes, &whitelist_type_ids, &generics, method_uid, method_item, method) {
          Ok(x) => {
            result += &x
          },
          Err(SimplifyError::SimplifyFatal(e)) => return Err(SimplifyError::SimplifyFatal(e)),
          Err(SimplifyError::Unsupported(reason)) => {
            result +=
              &format!(
                "  // Skipping {} method {}: {}\n",
                // struct_item.name.as_ref().unwrap_or(&"".to_owned()),
                trait_name,
                method_item.name.as_ref().unwrap_or(&"".to_owned()),
                &reason);
          }
        }
      }
      _ => {} // Ignore anything else
    }
  }

  Ok(result)
}

fn generic_id(
  name: String,
) -> UId {
  use std::collections::hash_map::DefaultHasher;
  use std::hash::{Hash, Hasher};
  let mut hasher = DefaultHasher::new();
  name.hash(&mut hasher);
  UId {
    // Store the generic param name after "$" so we can extract it later
    crate_name: format!("${}", name),
    id: Id(hasher.finish() as u32),
  }
}

fn valify_method(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  method_name_to_count: &HashMap<String, usize>,
  defaulted_generic_runes: &HashSet<String>,
  outer_whitelist_type_uids: &HashSet<UId>,
  outer_generics: &HashMap<String, SimpleType>,
  method_uid: UId,
  method_item: &Item,
  method: &Function
) -> Result<String, SimplifyError> {
  let mut generics = outer_generics.clone();
  let mut whitelist_type_uids = outer_whitelist_type_uids.clone();
  // for generic in &method.generics.params {
  //   generics.insert(
  //     generic.name.to_owned(),
  //     SimpleType {
  //       imm_ref: false,
  //       mut_ref: false,
  //       valtype: SimpleValType {
  //         id: generic_id(generic.name.to_owned()),
  //         generic_args: Vec::new(),
  //         maybe_parent_impl: None,
  //         maybe_parent_concrete: None
  //       }
  //     });
  // }

  if let Some(name) = &method_item.name {
    if *method_name_to_count.get(name).unwrap_or(&0) > 1 {
      return Err(Unsupported("Method is overloaded".to_owned()));
    } else {
      eprintln!("Doing func {} which has {} overloads", name, *method_name_to_count.get(name).unwrap());
    }
  } else {
    return Err(Unsupported("Encountered no-name method".to_owned()));
  }

  if method.generics.params.len() > 0 {
    return Err(Unsupported("Method generics unsupported".to_owned()));
  }

  let mut result: String =
      "  extern func ".to_owned() +
      // DO NOT SUBMIT
      method_item.name.as_ref().expect("Struct method has no name!") +
      "(";

  let mut params_inner_str = "".to_owned();
  for (param_name, param_type) in &method.sig.inputs {
    if params_inner_str.len() > 0 {
      params_inner_str += ", ";
    }
    let param_simplified =
        simplify_type(crates, item_index, &defaulted_generic_runes, Some(&whitelist_type_uids), &generics, &method_uid.crate_name, param_type)?; // DO NOT SUBMIT
    params_inner_str += param_name;
    params_inner_str += " ";
    params_inner_str += &valify_simple_type(crates, item_index, &param_simplified);
  }
  result += &params_inner_str;

  result += ")";

  if let Some(output) = &method.sig.output {
    let output_simplified =
        simplify_type(crates, item_index, defaulted_generic_runes, Some(&whitelist_type_uids), &generics, &method_uid.crate_name, output)?;
    result += " ";
    result += &valify_simple_type(crates, item_index, &output_simplified);
  }

  result += ";\n";

  Ok(result)
}

fn valify_simple_type(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  type_: &SimpleType
) -> String {
  "".to_owned() +
  (if type_.mut_ref || type_.imm_ref {
    // Skip, the ABI will do this for us for now DO NOT SUBMIT
    // From later: wat? i think we need it now DO NOT SUBMIT
    "&"
  } else {
    ""
  }) +
      &valify_simple_valtype(crates, item_index, &type_.valtype)
}

fn valify_simple_valtype(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  valtype: &SimpleValType
) -> String {
  let uid = &valtype.id;
  if is_primitive(&item_index.primitive_uid_to_name, uid) {
    if is_special_primitive(&item_index.primitive_name_to_uid, &uid) {
      if is_generic(uid) {
        return generic_name(uid).to_string()
      } else if uid == &tuple_id(&item_index.primitive_name_to_uid) {
        let mut inner_str = "".to_owned();
        for x in &valtype.generic_args {
          inner_str += &valify_simple_type(crates, item_index, x);
          inner_str += ", ";
        }
        "(".to_owned() + &inner_str + ")"
      } else if valtype.id == str_id(&item_index.primitive_name_to_uid) {
        "str".to_owned()
      } else {
        unimplemented!()
      }
    } else {
      if let Some(thing) = item_index.primitive_uid_to_name.get(uid) {
        if thing == "usize" {
          return "i64".to_owned(); // DO NOT SUBMIT
        } else {
          return thing.to_owned()
        }
      } else {
        unimplemented!()
      }
    }
  } else {
    let item = lookup_uid(crates, uid);
    let mut result =
        match &item.name {
          Some(x) => x.to_owned(),
          None => unimplemented!(),
        };
    if valtype.generic_args.len() > 0 {
      let trimmed_num_generic_args =
        match &item.inner {
          ItemEnum::Trait(trait_) => filter_defaulted_generics(&trait_.generics),
          ItemEnum::Struct(struct_) => filter_defaulted_generics(&struct_.generics),
          ItemEnum::Enum(enum_) => filter_defaulted_generics(&enum_.generics),
          _ => unimplemented!(),
        };

      let mut generics_str = "".to_owned();
      for x in &valtype.generic_args.iter().take(trimmed_num_generic_args).into_iter().collect::<Vec<_>>() {
        if generics_str.len() > 0 {
          generics_str += ", ";
        }
        generics_str += &valify_simple_type(crates, item_index, x);
      }

      result += "<";
      result += &generics_str;
      result += ">";
    }
    result
  }
}

fn filter_defaulted_generics(generics: &Generics) -> usize {
  generics.params.iter().filter(|x| {
    match &x.kind {
      GenericParamDefKind::Type { bounds, default, is_synthetic } => {
        default.is_none()
      }
      GenericParamDefKind::Lifetime { .. } => unimplemented!(),
      GenericParamDefKind::Const { .. } => unimplemented!(),
    }
  }).collect::<Vec<_>>().len()
}

fn turbofishify(rust_type_str_unturboed: &str) -> String {
  // Strings start with < if they're talking about impls, hence not doing the replace on that first one.
  rust_type_str_unturboed[0..1].to_string() +
      &rust_type_str_unturboed[1..].replace("<", "::<").replace("::::<", "::<")
}

fn full_type_get_init(parsed_type: &ParsedFullType) -> ParsedFullType {
  ParsedFullType { steps: parsed_type.steps[0..parsed_type.steps.len() - 1].into_iter().map(|x| x.clone()).collect::<Vec<_>>() }
}

fn determine_public_type(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  canonical_type: &ParsedFullType
) -> anyhow::Result<ParsedFullType> {
  let s = sizify_full_type(canonical_type);
  eprintln!("Doing type {:?}", s);
  let mut result_type = ParsedFullType { steps: Vec::new() };
  let mut previous: Option<UId> = None;
  for (step_i, step) in canonical_type.steps.iter().enumerate() {
    // let uid =
    match step {
      ParsedType::Alias(name) => unimplemented!(),
      ParsedType::Primitive(_) => {
        result_type.steps.push(step.clone());
      },
      ParsedType::Ref { mutable, inner } => {
        result_type.steps.push(
          ParsedType::Ref {
            mutable: *mutable,
            inner: determine_public_type(crates, item_index, inner)?
          });
      },
      ParsedType::Tuple { generic_args } => {
        let mut new_generic_args = Vec::new();
        for generic_arg in generic_args {
          new_generic_args.push(
            determine_public_type(crates, item_index, generic_arg)?);
        }
        result_type.steps.push(ParsedType::Tuple { generic_args: new_generic_args });
      },
      ParsedType::Slice { inner } => {
        result_type.steps.push(
          ParsedType::Slice {
            inner: determine_public_type(crates, item_index, inner)?
          });
      },
      ParsedType::Value { name, generic_args, params } => {
        match extend_and_resolve_uid(crates, &item_index.primitive_name_to_uid, previous.as_ref(), name) {
          Ok(uid) => {
            previous = Some(uid.clone());

            // sanity check
            match &lookup_uid(crates, &uid).inner {
              ItemEnum::Module(m) => {
                if m.is_stripped {
                  // Don't add this step to the final type, since its members were re-exported.
                  unimplemented!(); // curious
                }
              }
              _ => {}
            }

            let mut new_generic_args = Vec::new();
            for generic_arg in generic_args {
              new_generic_args.push(
                determine_public_type(crates, item_index, generic_arg)?);
            }
            let mut new_params = Vec::new();
            for param in params {
              new_params.push(
                determine_public_type(crates, item_index, param)?);
            }

            result_type.steps.push(ParsedType::Value {
              name: name.clone(),
              generic_args: new_generic_args,
              params: new_params
            });
          }
          Err(ResolveError::NotFound) => {
            // Sometimes when we get one of these path steps, it's actually a private module
            // such as the unwind_safe in core::panic::unwind_safe::RefUnwindSafe.
            // The RefUnwindSafe was actually pub use'd in core::panic.
            // If this isn't the last step, let's skip it and see if things work.
            // TODO: document better or something? seems sketchy.
            // Also, this seems like it has something to do with Module's is_stripped
            // field, but there's not really a good way to use that to help, AFAICT.
            continue;
          }
          Err(ResolveError::Unsupported(reason)) => {
            return Err(anyhow::anyhow!("Unsupported type: {}", reason));
          }
          Err(ResolveError::ResolveFatal(fatal)) => return Err(fatal)
        }
      }
      ImplCast { struct_, impl_ } => {
        // All impl methods are public, so we don't actually need to do anything here.
        // We can just use the struct's name.
        // Though, it does mean that things are ambiguous now, for example, we're turning:
        //   <std::ffi::os_str::OsString as core::convert::From<&str>>::from
        // into:
        //   std::ffi::os_str::OsString::from
        let mut struct_canonical_type = determine_public_type(crates, item_index, struct_)?;
        result_type.steps.append(&mut struct_canonical_type.steps);
        result_type.steps.append(&mut canonical_type.steps[(step_i + 1)..].iter().map(|x| x.clone()).collect::<Vec<ParsedType>>());
        eprintln!("Determined canonical: {:?}", sizify_full_type(&result_type));
        return Ok(result_type);
      }
      ParsedType::Lifetime => unimplemented!(),
      ParsedType::Wildcard => unimplemented!(),
    }
  }
  eprintln!("Determined canonical: {:?}", sizify_full_type(&result_type));
  return Ok(result_type);
}

fn parse_func_info_line(info_line: &str) -> Result<(usize, String)> {
  if let Some(first_space_pos) = info_line.find(" ") {
    let arity_str = &info_line[0..first_space_pos];
    match arity_str.parse::<usize>() {
      Ok(arity) => {
        let func_str = &info_line[first_space_pos..].trim();
        return Ok((arity, turbofishify(func_str)));
      },
      Err(e) => {}
    }
  }
  return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Bad func info line: {}", info_line))));
}

fn parse_type_info_line(info_line: &str) -> Result<(usize, usize, String)> {
  if let Some(first_space_pos) = info_line.find(" ") {
    let size_str = &info_line[0..first_space_pos];
    match size_str.parse::<usize>() {
      Ok(size) => {
        let line_after_size = info_line[first_space_pos..].trim();
        if let Some(second_space_pos_in_line_after_size) = line_after_size.find(" ") {
          let alignment_str = &line_after_size[0..second_space_pos_in_line_after_size];
          match alignment_str.parse::<usize>() {
            Ok(alignment) => {
              let type_str = &line_after_size[second_space_pos_in_line_after_size..].trim();
              return Ok((size, alignment, turbofishify(type_str)));
            }
            Err(E) => {}
          }
        }
      },
      Err(e) => {}
    }
  }
  return Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Bad type info line: {}", info_line))));
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

fn sizer_preamble() -> String {
  common_preamble() +
  r#"
fn print_type<T>() {
    println!("  {} {} {}", std::mem::size_of::<T>(), std::mem::align_of::<T>(), std::any::type_name::<T>());
}

trait PrintFn<Args> {
    fn print_fn();
}

fn print_fn<T: PrintFn<Args>, Args>(_: T) {
    T::print_fn()
}

impl <F, R> PrintFn<(R, )> for F where F: Fn() -> R {
    fn print_fn() {
        println!("  0 {}", std::any::type_name::<Self>());
        print_type::<R>();
    }
}

impl <F, R, P1> PrintFn<(R, P1, )> for F where F: Fn(P1) -> R {
    fn print_fn() {
        println!("  1 {}", std::any::type_name::<Self>());
        print_type::<R>();
        print_type::<P1>();
    }
}

impl <F, R, P1, P2> PrintFn<(R, P1, P2, )> for F where F: Fn(P1, P2) -> R {
    fn print_fn() {
        println!("  2 {}", std::any::type_name::<Self>());
        print_type::<R>();
        print_type::<P1>();
        print_type::<P2>();
    }
}

fn select_overload_printable_1<R, P1>(thing: impl Fn(P1) -> R + PrintFn<(R, P1, )>) -> impl Fn(P1) -> R + PrintFn<(R, P1,)> {
    thing
}
fn select_overload_printable_2<R, P1, P2>(thing: impl Fn(P1, P2) -> R + PrintFn<(R, P1, P2, )>) -> impl Fn(P1, P2) -> R + PrintFn<(R, P1, P2,)> {
    thing
}
"#
}

fn common_preamble() -> String {
  r#"
#![feature(os_str_display)]

use static_assertions::const_assert_eq;
use std::mem;
extern crate alloc;
use core;
use core::ffi::c_char;
"#.to_owned()
}

fn instantiations_preamble(str_ref_alias: &str) -> String {
  r#"
#[no_mangle]
pub extern "C" fn rust_StrFromCStr(char_ptr: *const c_char) -> rust_str_ref {
  let c_str = unsafe { core::ffi::CStr::from_ptr(char_ptr) };
  if let Ok(rust_str) = c_str.to_str() {
    let s_rs: rust_str_ref = unsafe { mem::transmute(rust_str) };
    return s_rs;
  } else {
    panic!("Error: c_str.to_str() failed.");
  }
}

// TODO: Is it okay to use u8 here instead of c_char?
#[no_mangle]
pub extern "C" fn rust_StrNew(length: usize, char_ptr: *const u8) -> rust_str_ref {
  let c_str = unsafe { std::slice::from_raw_parts(char_ptr, length) };
  if let Ok(rust_str) = core::ffi::CStr::from_bytes_with_nul(c_str) {
    if let Ok(rust_str) = rust_str.to_str() {
      let s_rs: rust_str_ref = unsafe { mem::transmute(rust_str) };
      return s_rs;
    } else {
      panic!("Error: c_str.to_str() failed.");
    }
  } else {
    panic!("Error: CStr::from_bytes_with_nul() failed.");
  }
}

#[no_mangle]
pub extern "C" fn rust_StrToCStr(str_c: rust_str_ref) -> *const c_char {
  let str_rs: &str = unsafe { mem::transmute(str_c) };
  let ptr = str_rs.as_ptr() as *const c_char;
  return ptr;
}

#[no_mangle]
pub extern "C" fn rust_StrLen(str_c: rust_str_ref) -> usize {
  let str_rs: &str = unsafe { mem::transmute(str_c) };
  let len: usize = str_rs.len();
  return len;
}

"#.replace("rust_str_ref", str_ref_alias)
}

#[derive(Debug)]
enum ResolveError {
  NotFound,
  Unsupported(String),
  ResolveFatal(anyhow::Error)
}

fn sizify_type(type_: &ParsedType) -> String {
  match type_ {
    ParsedType::Alias(name) => unimplemented!(),
    ParsedType::ImplCast { struct_, impl_ } => {
      sizify_full_type(struct_)
    }
    ParsedType::Ref { mutable, inner } => {
      "&".to_string() + (if *mutable { "mut " } else { "" }) + &sizify_full_type(inner)
    }
    ParsedType::Tuple { generic_args} => {
      "(".to_owned() +
          &generic_args.into_iter().map(|x| sizify_full_type(x)).collect::<Vec<_>>().join(", ") +
          ")"
    }
    ParsedType::Slice { inner} => {
      "[".to_owned() + &sizify_full_type(inner) + "]"
    }
    ParsedType::Value { name, generic_args, params } => {
      name.to_owned() +
      &(if generic_args.len() > 0 {

        "::<".to_owned() +
        &generic_args
            .iter()
            .filter(|x| {
              // Can't specify lifetimes explicitly, for example:
              //   error[E0794]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
              //      --> src/main.rs:69:87
              //       |
              //   69  |   println!("fn {}", "Regex::captures::<'static>");  print_fn(regex::Regex::captures::<'static>);
              //       |                                                                                       ^^^^^^^
              // so we just take them out.
              **x != (ParsedFullType { steps: vec![ParsedType::Lifetime] })
            })
            .map(sizify_full_type)
            .collect::<Vec<_>>()
            .join(", ") +
        ">"
      } else {
        "".to_owned()
      })
    }
    ParsedType::Primitive(name) => name.to_owned(),
    ParsedType::Lifetime => "'static".to_owned(),
    ParsedType::Wildcard => "_".to_owned(),
  }
}

fn sizify_full_type(full_type: &ParsedFullType) -> String {
  let steps = &full_type.steps;
  steps.into_iter().map(sizify_type).collect::<Vec<_>>().join("::")
}

fn sizify_func(full_type: &ParsedFullType) -> String {
  let inner = sizify_full_type(full_type);
  let last_step = full_type.steps.last().unwrap();
  match last_step {
    ParsedType::Alias(name) => unimplemented!(),
    ParsedType::ImplCast { .. } => panic!("wat"),
    ParsedType::Ref { .. } => panic!("Func last step is a ref?"),
    ParsedType::Primitive(_) => panic!("Func last step is a primitive?"),
    ParsedType::Lifetime => panic!("Func last step is a lifetime?"),
    ParsedType::Wildcard => panic!("Func last step is a wildcard?"),
    ParsedType::Slice { .. } => panic!("Func last step is a slice?"),
    ParsedType::Tuple { .. } => panic!("Func last step is a tuple?"),
    ParsedType::Value { name, generic_args, params } => {
      if params.len() > 0 {
        "select_overload_printable_".to_owned() +
        &params.len().to_string() +
        "::<_, " +
        &params.into_iter().map(sizify_full_type).collect::<Vec<_>>().join(", ") +
        ">(" +
        &inner +
        ")"
      } else {
        inner
      }
    }
  }
}

fn get_type_sizer_string(
  // This isn't necessarily the same as sizified_type.
  // For example, the user might have supplied std::vec::Vec<std::ffi::OsString>
  // and the sizified_type might be std::vec::Vec::<std::ffi::OsString>.
  original_str: &str,
  full_type: &ParsedFullType
) -> String {
  let sizified_type = sizify_full_type(full_type);
  let mut rust_src = String::with_capacity(1000);
  rust_src += "  println!(\"type {}\", \"";
  rust_src += original_str;
  rust_src += "\");";
  rust_src += "  print_type::<";
  rust_src += &sizified_type;
  rust_src += ">();";
  return rust_src;
}

fn get_func_scouting_string(
  // This isn't necessarily the same as sizified_type.
  // For example, the user might have supplied std::vec::Vec<std::ffi::OsString>::push
  // and the sizified_type might be std::vec::Vec::<std::ffi::OsString>::push.
  original_func_str: &str,
  full_type: &ParsedFullType
) -> String {
  let sizified_type = sizify_full_type(full_type);
  let sizified_func = sizify_func(full_type);
  let mut rust_src = String::with_capacity(1000);
  rust_src += "  println!(\"fn {}\", \"";
  rust_src += original_func_str;
  rust_src += "\");";
  rust_src += "  print_fn(";
  rust_src += &sizified_func;
  rust_src += ");";
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
  info: &TypeInfo
) -> anyhow::Result<String> {
  let mut builder = String::with_capacity(1000);
  builder += "#[repr(C, align(";
  builder += &info.alignment.to_string();
  builder += "))]\n";
  // builder += "#[repr(C)]\n";
  builder += "pub struct ";
  builder += &info.c_name;
  builder += " (std::mem::MaybeUninit<[u8; ";
  builder += &info.size.to_string();
  builder += "]>);\n";
  builder += "const_assert_eq!(std::mem::size_of::<";
  builder += &info.c_name;
  builder += ">(), ";
  builder += &info.size.to_string();
  builder += ");\n";

  return Ok(builder);
}

fn instantiate_func(
  type_rust_str_to_info: &HashMap<String, TypeInfo>,
  info: &FuncInfo
) -> anyhow::Result<String> {
  let mut param_type_infos =
      info.param_types_rust_strs
          .iter()
          .map(|x| type_rust_str_to_info.get(x).unwrap())
          .collect::<Vec<&TypeInfo>>();
  // Per @SMLRZ, drop identified by last path step. Path has type args on struct step (e.g. Vec<i32>::drop).
  let is_drop =
    match info.canonical_type.steps.last().unwrap() {
      ParsedType::Value { name, .. } => name == "drop",
      _ => false
    };
  if is_drop {
    let type_rust_str = sizify_full_type(&full_type_get_init(&info.canonical_type));
    param_type_infos[0] = type_rust_str_to_info.get(&type_rust_str).unwrap();
  }

  let ret_type_info = type_rust_str_to_info.get(&info.ret_type_rust_str).unwrap();

  let mut rust_builder = String::with_capacity(1000);
  rust_builder += "#[no_mangle]\n";
  rust_builder += "pub extern \"C\" fn ";
  rust_builder += &info.c_name;
  rust_builder += "(\n";
  for (param_type_rust_str, param_i) in info.param_types_rust_strs.iter().zip(0..info.param_types_rust_strs.len()) {
    let param_name = "  param_".to_owned() + &param_i.to_string() + &"_c";
    let param_c_type_str = &type_rust_str_to_info.get(param_type_rust_str).unwrap().c_name;
    rust_builder += &format!("  {}: {},\n", param_name, param_c_type_str);
  }
  rust_builder += ")";
  if !is_drop { // DO NOT SUBMIT can we instead check if its void
    // if let Some(return_type_simple) = &maybe_output_type {
    rust_builder += " -> ";
    rust_builder += &ret_type_info.c_name;
    // }
  }
  rust_builder += " {\n";
  for (param_type_str, param_i) in info.param_types_rust_strs.iter().zip(0..info.param_types_rust_strs.len()) {
    let c_param_name = "param_".to_owned() + &param_i.to_string() + &"_c";
    let rs_param_name = "param_".to_owned() + &param_i.to_string() + &"_rs";
    let param_type_info = type_rust_str_to_info.get(param_type_str).unwrap();

    rust_builder += "  const_assert_eq!(std::mem::size_of::<";
    rust_builder += &str_for_full_type(&param_type_info.public_type);
    rust_builder += ">(), std::mem::size_of::<";
    rust_builder += &param_type_info.c_name;
    rust_builder += ">());\n";

    rust_builder += "  let ";
    rust_builder += &rs_param_name;
    rust_builder += ": ";
    rust_builder += &str_for_full_type(&param_type_info.public_type);
    rust_builder += " = unsafe { mem::transmute(";
    rust_builder += &c_param_name;
    rust_builder += ") };\n";
  }

  if !is_drop {
    rust_builder += "  ";
    // if let Some(return_type_simple) = maybe_output_type {
      rust_builder += "let result_rs: ";
      rust_builder += &str_for_full_type(&ret_type_info.public_type);
      rust_builder += " = ";
    // }

    rust_builder += &caller_str_for_full_type(&info.public_type);
    rust_builder += "(";
    for (param_type_str, param_i) in info.param_types_rust_strs.iter().zip(0..info.param_types_rust_strs.len()) {
      let c_param_name = "param_".to_owned() + &param_i.to_string() + &"_c";
      let rs_param_name = "param_".to_owned() + &param_i.to_string() + &"_rs";
      let param_type_info = type_rust_str_to_info.get(param_type_str).unwrap();

      rust_builder += &rs_param_name;
      rust_builder += ",";
    }
    rust_builder += ");\n";

    // if let Some(return_type_simple) = &maybe_output_type {
      rust_builder += "  const_assert_eq!(std::mem::size_of::<";
      rust_builder += &str_for_full_type(&ret_type_info.public_type);
      rust_builder += ">(), std::mem::size_of::<";
      rust_builder += &ret_type_info.c_name;
      rust_builder += ">());\n";

      rust_builder += "  let result_c: ";
      rust_builder += &ret_type_info.c_name;
      rust_builder += " = unsafe { mem::transmute(result_rs) };\n";
      rust_builder += "  return result_c;\n";
    // }
  }
  rust_builder += "}\n";

  return Ok(rust_builder);
}

fn parse_type<'a>(
  concrete_primitives: &HashSet<String>,
  replacements: &HashMap<String, ParsedFullType>,
  parsed_type_to_alias: &HashMap<ParsedFullType, String>,
  original: &'a str
) -> anyhow::Result<(ParsedType, ParsedType, &'a str)> {
  let mut rest = original;

  // TODO: prevent parsing 'staticky or something
  if rest.starts_with("'static") {
    rest = &rest["'static".len()..].trim();
    return Ok((ParsedType::Lifetime, ParsedType::Lifetime, rest));
  }

  let has_mut_ref = rest.starts_with("&mut"); // TODO: maybe handle &
  if has_mut_ref {
    rest = &rest["&mut".len()..].trim();
  }
  let has_imm_ref = rest.starts_with("&"); // TODO: maybe handle &
  if has_imm_ref {
    rest = &rest[1..].trim();
  }
  if has_imm_ref || has_mut_ref {
    let (inner_canonical, inner_aliasing, new_rest) =
        parse_full_type(&concrete_primitives, &replacements, &parsed_type_to_alias, rest)?;
    rest = new_rest;
    let result_canonical = ParsedType::Ref { mutable: has_mut_ref, inner: inner_canonical };
    let result_aliasing = ParsedType::Ref { mutable: has_mut_ref, inner: inner_aliasing };
    return Ok((result_canonical, result_aliasing, rest));
  }

  if rest.starts_with("<") {
    unimplemented!();
  }

  match parse_group(&concrete_primitives, &replacements, &parsed_type_to_alias, rest, false, true, false)? {
    (Some((tuple_members_canonical, tuple_members_aliasing)), new_rest) => {
      let result_canonical =
          ParsedType::Tuple { generic_args: tuple_members_canonical };
      let result_aliasing =
          ParsedType::Tuple { generic_args: tuple_members_aliasing };
      return Ok((result_canonical, result_aliasing, new_rest));
    }
    (None, _) => {} // continue
  }

  match parse_group(&concrete_primitives, &replacements, &parsed_type_to_alias, rest, false, false, true)? {
    (Some((tuple_members_canonical, tuple_members_aliasing)), new_rest) => {
      if tuple_members_canonical.len() != 1 {
        parse_group(&concrete_primitives, &replacements, &parsed_type_to_alias, rest, false, false, true);
        panic!("Bad slice!"); // DO NOT SUBMIT
      }
      let inner_canonical = tuple_members_canonical.first().unwrap();
      let result_canonical = ParsedType::Slice { inner: inner_canonical.clone() };
      let inner_aliasing = tuple_members_canonical.first().unwrap();
      let result_aliasing = ParsedType::Slice { inner: inner_aliasing.clone() };
      return Ok((result_canonical, result_aliasing, new_rest));
    }
    (None, _) => {} // continue
  }

  let re = Regex::new(r"( |,|::<|::|<|>|\[|\]|\(|\)|$)").unwrap();
  let name_end =
      if let Some(generic_name_match) = re.find(&rest) {
        generic_name_match.start()
      } else {
        rest.len()
      };
  let name = &rest[0..name_end];
  rest = &rest[name.len()..].trim();

  if name == "_" {
    return Ok((ParsedType::Wildcard, ParsedType::Wildcard, rest));
  }

  if concrete_primitives.contains(name) {
    return Ok((ParsedType::Primitive(name.to_string()), ParsedType::Primitive(name.to_string()), rest));
  }

  let (maybe_generic_args, new_rest) =
      parse_group(&concrete_primitives, replacements, parsed_type_to_alias, rest, true, false, false)?;
  rest = new_rest;
  let (generic_args_canonical, generic_args_aliasing) = maybe_generic_args.unwrap_or((Vec::new(), Vec::new()));

  let (maybe_params, new_rest) =
      parse_group(&concrete_primitives, replacements, parsed_type_to_alias, rest, false, true, false)?;
  rest = new_rest;
  let (params_canonical, params_aliasing) = maybe_params.unwrap_or((Vec::new(), Vec::new()));

  assert!(!name.contains("["));

  let result_canonical =
    ParsedType::Value {
      name: name.to_owned(),
      generic_args: generic_args_canonical,
      params: params_canonical
    };
  let result_aliasing =
      ParsedType::Value {
        name: name.to_owned(),
        generic_args: generic_args_aliasing,
        params: params_aliasing
      };
  Ok((result_canonical, result_aliasing, rest))
}

fn parse_group<'a>(
  concrete_primitives: &HashSet<String>,
  replacements: &HashMap<String, ParsedFullType>,
  parsed_type_to_alias: &HashMap<ParsedFullType, String>,
  original: &'a str,
  allow_angles: bool,
  allow_parens: bool,
  allow_squares: bool,
) -> Result<(Option<(Vec<ParsedFullType>, Vec<ParsedFullType>)>, &'a str)> {
  let mut rest = original;
  if (allow_angles && (rest.starts_with("::<") || rest.starts_with("<"))) ||
      (allow_parens && rest.starts_with("(")) ||
      (allow_squares && rest.starts_with("[")) {
    let mut generic_args_canonical = Vec::new();
    let mut generic_args_aliasing = Vec::new();

    if rest.starts_with("::<") {
      rest = &rest["::<".len()..].trim();
    } else if rest.starts_with("<") {
      rest = &rest["<".len()..].trim();
    } else if rest.starts_with("(") {
      rest = &rest["(".len()..].trim();
    } else if rest.starts_with("[") {
      rest = &rest["[".len()..].trim();
    } else {
      panic!("wat");
    }

    if rest.starts_with(">") {
      // Do nothing
    } else if rest.starts_with(")") {
      // Do nothing
    } else if rest.starts_with("]") {
      // Do nothing
    } else {
      loop {
        let (generic_arg_canonical, generic_arg_aliasing, new_rest) =
            parse_full_type(&concrete_primitives, &replacements, &parsed_type_to_alias, rest)?;
        rest = new_rest;
        generic_args_canonical.push(generic_arg_canonical);
        generic_args_aliasing.push(generic_arg_aliasing);
        if rest.starts_with(",") {
          rest = &rest[",".len()..].trim();
          // continue
        } else if rest.starts_with(">") || rest.starts_with(")") || rest.starts_with("]") {
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
    } else if rest.starts_with("]") {
      rest = &rest["]".len()..].trim();
    } else {
      panic!("Wat");
    }

    Ok((Some((generic_args_canonical, generic_args_aliasing)), rest))
  } else {
    Ok((None, rest))
  }
}

// Returns:
// - Full type
// - rest of the string that wasnt parsed
fn parse_full_type<'a>(
  concrete_primitives: &HashSet<String>,
  replacements: &HashMap<String, ParsedFullType>,
  parsed_type_to_alias: &HashMap<ParsedFullType, String>,
  original: &'a str
) -> anyhow::Result<(ParsedFullType, ParsedFullType, &'a str)> {
  let (full_type_canonical, full_type_with_aliases, rest) =
      parse_full_type_inner(concrete_primitives, replacements, parsed_type_to_alias, original)?;

  if let Some(alias_name) = parsed_type_to_alias.get(&full_type_canonical) {
    let alias_result =
        ParsedFullType {
          steps: vec![
            ParsedType::Alias(alias_name.clone())
          ]
        };
    return Ok((full_type_canonical, alias_result, rest));
  } else {
    return Ok((full_type_canonical, full_type_with_aliases, rest));
  }
}

// Per @SMLRZ, parses Rust paths with type args on type steps (e.g. Vec<i32>::capacity).
fn parse_full_type_inner<'a>(
  concrete_primitives: &HashSet<String>,
  replacements: &HashMap<String, ParsedFullType>,
  parsed_type_to_alias: &HashMap<ParsedFullType, String>,
  original: &'a str
) -> anyhow::Result<(ParsedFullType, ParsedFullType, &'a str)> {
  let mut rest = original;

  let re = Regex::new(r"(,|>|\)|$)").unwrap();
  let name_end =
      if let Some(generic_name_match) = re.find(&rest) {
        generic_name_match.start()
      } else {
        rest.len()
      };
  let full_name_preview = &rest[0..name_end];
  // if let Some(uid) = item_index.primitive_name_to_uid.get(full_name_preview) {
  //   rest = &rest[full_name_preview.len()..].trim();
  //
  //   return Ok(
  //     (
  //       SimpleValType {
  //         id: uid.clone(),
  //         generic_args: Vec::new(),
  //         maybe_parent_concrete: None,
  //         maybe_parent_impl: None
  //       },
  //       rest));
  // }

  let mut steps_canonical = Vec::new();
  let mut steps_aliasing = Vec::new();

  if rest.starts_with("<") {
    rest = &rest["<".len()..].trim();

    let (struct_full_type_canonical, struct_full_type_aliasing, new_rest) =
        parse_full_type(&concrete_primitives, &replacements, &parsed_type_to_alias, rest)?;
    rest = new_rest;

    if !rest.starts_with("as ") {
      panic!("wat");
    }
    rest = &rest["as".len()..].trim();

    let (impl_full_type_canonical, impl_full_type_aliasing, new_rest) =
        parse_full_type(&concrete_primitives, &replacements, &parsed_type_to_alias, rest)?;
    rest = new_rest;


    if !rest.starts_with(">") {
      panic!("wat");
    }
    rest = &rest[">".len()..].trim();

    if !rest.starts_with("::") {
      panic!("wat");
    }
    rest = &rest["::".len()..].trim();

    steps_canonical.push(
      ImplCast {
        struct_: struct_full_type_canonical,
        impl_: impl_full_type_canonical });
    steps_aliasing.push(
      ImplCast {
        struct_: struct_full_type_aliasing,
        impl_: impl_full_type_aliasing });
  }

  loop {
    let (new_step_canonical, new_step_aliasing, new_rest) =
        parse_type(&concrete_primitives, &replacements, &parsed_type_to_alias, rest)?;
    rest = new_rest;

    let maybe_replacement =
      if steps_canonical.len() == 0 {
        match &new_step_canonical {
          ParsedType::Value { name, generic_args, params } => {
            if generic_args.len() == 0 && params.len() == 0 {
              if let Some(replacement) = replacements.get(name) {
                Some(replacement)
              } else {
                None
              }
            } else {
              None
            }
          }
          _ => None
        }
      } else {
        None
      };
    if let Some(replacement) = maybe_replacement {
      steps_canonical.append(&mut replacement.steps.clone());
      steps_aliasing.append(&mut replacement.steps.clone());
    } else {
      steps_canonical.push(new_step_canonical);
      steps_aliasing.push(new_step_aliasing);
    }
    if rest.starts_with("::") {
      rest = &rest["::".len()..].trim();
      // continue
    } else {
      break;
    }
  }

  let result_canonical = ParsedFullType{ steps: steps_canonical };
  let result_aliasing = ParsedFullType{ steps: steps_aliasing };
  Ok((result_canonical, result_aliasing, rest))
}

fn mangle_generic_args(generic_args: &Vec<ParsedFullType>, more_after: bool) -> String {
  // If it's a tuple then we still want to print out the () even if
  // there's nothing inside it.
  if generic_args.len() > 0 {
    "_".to_owned() +
    &generic_args.len().to_string() +
    "__" +
    &generic_args
        .iter()
        .map(|x| {
          mangle_full_type(x)
        })
        .collect::<Vec<_>>()
        .join("__") +
        (if more_after { "__" } else { "" })
  } else {
    "".to_string()
    //("".to_string(), "".to_owned())
  }
}

fn mangle_type(
  valtype: &ParsedType,
  more: bool
) -> String {
  match valtype {
    ParsedType::Alias(name) => name.clone(),
    ParsedType::ImplCast { struct_, impl_ } => {
      mangle_full_type(struct_) +
          "__as_1__" +
          &mangle_full_type(impl_)
    }
    ParsedType::Ref { mutable, inner } => {
      (if *mutable { "mref_1__" } else { "iref_1__" }).to_owned() +
      &mangle_full_type(inner)
    }
    ParsedType::Value { name, generic_args, params } => {
      name.to_owned() +
          &mangle_generic_args(generic_args, more)
    },
    ParsedType::Tuple { generic_args } => {
      "tuple_".to_owned() +
          &mangle_generic_args(generic_args, more)
    }
    ParsedType::Slice { inner } => {
      "slice_".to_owned() +
          &mangle_generic_args(&vec![inner.clone()], more)
    },
    ParsedType::Primitive(name) => {
      if name == "str" {
        "str_ref".to_owned()
      } else {
        name.to_owned()
      }
    }
    ParsedType::Lifetime => "life".to_owned(),
    ParsedType::Wildcard => panic!("Wat wildcard"), // I dont think we can get these from rust
  }
}

fn mangle_full_type(type_: &ParsedFullType) -> String {
  let steps = &type_.steps;
  let mut result = "".to_string();
  for step_i in 0..steps.len() {
    if step_i > 0 {
      result += "_";
    }
    result += &mangle_type(&steps[step_i], step_i < steps.len() - 1);
  }
  result
}

fn get_pointered_prefixed_mangled_type(
  type_: &ParsedFullType
) -> String {
  match type_.steps.last().unwrap() {
    ImplCast { .. } => unimplemented!(),
    ParsedType::Lifetime => unimplemented!(),
    ParsedType::Wildcard => unimplemented!(),
    ParsedType::Ref { mutable, inner: referend } => {
      match referend.steps.last().unwrap() {
        ParsedType::Slice { inner: _IGNORED } => {
          // Something that's a reference to a slice should be a struct to C
          let thing = mangle_full_type(referend);
          thing
        }
        _ => {
          "*".to_owned() +
              (if *mutable { "mut " } else { "const " }) +
              &get_prefixed_mangled_type(referend)
        }
      }
    }
    _ => get_prefixed_mangled_type(type_)
  }
}

fn get_prefixed_mangled_type(
  type_: &ParsedFullType
) -> String {
  match type_.steps.last().unwrap() {
    ImplCast { .. } => unimplemented!(),
    ParsedType::Lifetime => unimplemented!(),
    ParsedType::Wildcard => unimplemented!(),
    ParsedType::Ref { mutable, inner: referend } => {
      unimplemented!(); // If we get here, we're probably in a double pointer
    }
    ParsedType::Value { .. } => {
      "rust_".to_owned() + &mangle_full_type(type_)
    }
    ParsedType::Alias(name) => name.clone(),
    ParsedType::Primitive(name) => name.clone(),
    ParsedType::Tuple { .. } => {
      "rust_".to_owned() + &mangle_full_type(type_)
    }
    ParsedType::Slice { .. } => {
      "rust_".to_owned() + &mangle_full_type(type_)
    }
  }
}

fn str_for_type(type_: &ParsedType) -> String {
  match type_ {
    ParsedType::Alias(name) => "$".to_string() + name,
    ParsedType::ImplCast { struct_, impl_ } => {
      str_for_full_type(struct_)
    }
    ParsedType::Ref { mutable, inner } => {
      "&".to_string() + (if *mutable { "mut " } else { "" }) + &str_for_full_type(inner)
    }
    ParsedType::Tuple { generic_args} => {
      "(".to_owned() +
          &generic_args.into_iter().map(|x| str_for_full_type(x)).collect::<Vec<_>>().join(", ") +
          ")"
    }
    ParsedType::Slice { inner} => {
      "[".to_owned() + &str_for_full_type(inner) + "]"
    }
    ParsedType::Value { name, generic_args, params } => {
      name.to_owned() +
          &(if generic_args.len() > 0 {
            "::<".to_owned() +
                &generic_args.into_iter().map(str_for_full_type).collect::<Vec<_>>().join(", ") +
                ">"
          } else {
            "".to_owned()
          })
    }
    ParsedType::Primitive(name) => name.to_owned(),
    ParsedType::Lifetime => "'static".to_owned(),
    ParsedType::Wildcard => "_".to_owned(),
  }
}

fn str_for_full_type(full_type: &ParsedFullType) -> String {
  full_type.steps.iter().map(str_for_type).collect::<Vec<_>>().join("::")
}

fn caller_str_for_full_type(full_type: &ParsedFullType) -> String {
  let init_steps: Vec<ParsedType> =
      full_type.steps[0..full_type.steps.len() - 1].to_vec();
  let last_step =
    match full_type.steps.last().unwrap().clone() {
      ParsedType::Alias(name) => unimplemented!(),
      ParsedType::Ref { .. } => panic!("wat"),
      ParsedType::Tuple { .. } => panic!("wat"),
      ParsedType::Slice { .. } => panic!("wat"),
      ImplCast { .. } => panic!("wat"),
      ParsedType::Primitive(_) => panic!("wat"),
      ParsedType::Lifetime => panic!("wat"),
      ParsedType::Wildcard => panic!("wat"),
      ParsedType::Value { name, generic_args, params } => {
        ParsedType::Value { name, generic_args: Vec::new(), params: Vec::new() }
      }
    };
  let mut steps = init_steps;
  steps.push(last_step);
  steps.iter().map(str_for_type).collect::<Vec<_>>().join("::")
}
