use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use rustdoc_types::{Crate, GenericArg, GenericArgs, GenericBound, Type};
use crate::indexer::ItemIndex;
use crate::{ResolveError, UId};
use crate::resolve::{lookup_name, resolve};
use crate::resolve_id::{item_has_name, primitive_id, tuple_id};
use crate::ResolveError::ResolveFatal;
use crate::simplify::SimplifyError::Unsupported;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct SimpleType {
  pub imm_ref: bool,
  pub mut_ref: bool,
  pub valtype: SimpleValType,
}
// It's guaranteed (well, in progress) that this thing's ID will not be
// pointing at a type alias or an import.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct SimpleValType {
  // This is the underlying actual ID, after we've resolved any
  // imports or type aliases or whatnot.
  // This wouldn't be std::io::Result's ID, since that's an import.
  // That import imports an alias. This ID isn't for that type alias.
  // This is the ID of the final core::result::Result.
  pub id: UId,
  // These are the generic args for the underlying thing, e.g.
  // core::result::Result not std::io::Result.
  pub generic_args: Vec<SimpleType>,

  // If this is a method, then the parent will be the struct.
  pub maybe_parent_concrete: Option<Box<SimpleValType>>,
  // We'll need this in case we import like:
  //   #pragma rsuse std::ffi::OsString::From<&str>::from as RustOsStringFrom
  // because when we figure out from's argument's we'll need to know what the impl's <T> is.
  pub maybe_parent_impl: Option<Box<SimpleValType>>,
}


#[derive(Debug)]
pub(crate) enum SimplifyError {
  Unsupported(String),
  SimplifyFatal(anyhow::Error)
}

pub(crate) fn simplify_type(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  // context_container: Option<&SimpleValType>,
  defaulted_generic_runes: &HashSet<String>, // DO NOT SUBMIT explain that this is a blacklist, and maybe wrap it in an option
  maybe_whitelist_type_uids: Option<&HashSet<UId>>,
  generics: &HashMap<String, SimpleType>,
  type_crate_name: &str,
  type_: &Type
) -> anyhow::Result<SimpleType, SimplifyError> {
  let result =
    match type_ {
      Type::ResolvedPath(path) => {
        let generic_args =
            if let Some(outer_args) = &path.args {
              match outer_args.deref() {
                GenericArgs::AngleBracketed { args, .. } => {
                  let mut result = Vec::new();
                  for arg in args {
                    result.push(
                      simplify_generic_arg_inner(crates, &item_index, defaulted_generic_runes, maybe_whitelist_type_uids, generics, type_crate_name, arg)?);
                  }
                  result
                }
                GenericArgs::Parenthesized { inputs: _, output: _ } => {
                  unimplemented!();
                }
                GenericArgs::ReturnTypeNotation => {
                  unimplemented!();
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
        let result =
          SimpleType {
            imm_ref: false,
            mut_ref: false,
            valtype:
            match resolve(&crates, item_index, None, None, &result_uid, generic_args) {
              Ok(x) => x,
              Err(ResolveError::NotFound) => {
                unimplemented!()
              }
              Err(ResolveError::Unsupported(reason)) => return Err(SimplifyError::Unsupported(reason)),
              Err(ResolveFatal(e)) => return Err(SimplifyError::SimplifyFatal(e))
            }
          };
        if let Some(whitelist_type_uids) = maybe_whitelist_type_uids {
          if !whitelist_type_uids.contains(&result.valtype.id) {
            return Err(Unsupported(format!("Didn't contain whitelisted type: {}", lookup_name(crates, &result.valtype))));
          }
        }
        result
      }
      Type::DynTrait(_dynTrait) => {
        println!("what");
        unimplemented!();
      }
      Type::Generic(name) => {
        if defaulted_generic_runes.contains(name) {
          return Err(Unsupported("Encountered defaulted rune".to_owned()));
        } else {
          let result =
            generics.get(name)
                .expect(&("Unknown generic: ".to_owned() + &name))
                .clone();
          if let Some(whitelist_type_uids) = maybe_whitelist_type_uids {
            // A generic should always be whitelisted, I'd imagine
            assert!(whitelist_type_uids.contains(&result.valtype.id));
          }
          result
        }
      }
      Type::BorrowedRef { is_mutable, type_, .. } => {
        let mut thing =
            simplify_type(crates, &item_index, defaulted_generic_runes, maybe_whitelist_type_uids, generics, type_crate_name, type_)?;
        if *is_mutable {
          thing.mut_ref = true;
        } else {
          thing.imm_ref = true;
        }
        if let Some(whitelist_type_uids) = maybe_whitelist_type_uids {
          // valtype should have been checked already
          assert!(whitelist_type_uids.contains(&thing.valtype.id));
        }
        thing
      }
      Type::Primitive(name) => {
        let result =
          SimpleType {
            imm_ref: false,
            mut_ref: false,
            valtype: SimpleValType {
              id: primitive_id(&item_index.primitive_name_to_uid, name),
              generic_args: Vec::new(),
              maybe_parent_concrete: None,
              maybe_parent_impl: None
            }
          };
        if let Some(whitelist_type_uids) = maybe_whitelist_type_uids {
          if !whitelist_type_uids.contains(&result.valtype.id) {
            return Err(Unsupported(format!("Didn't contain whitelisted type: {}", name)));
          }
        }
        result
      },
      Type::FunctionPointer(_) => unimplemented!(),
      Type::Tuple(inners) => {
        let mut generic_args = Vec::new();
        for inner in inners {
          generic_args.push(
            simplify_type(crates, &item_index, defaulted_generic_runes, maybe_whitelist_type_uids, generics, type_crate_name, inner)?);
        }
        let result =
          SimpleType {
            imm_ref: false,
            mut_ref: false,
            valtype: SimpleValType {
              id: tuple_id(&item_index.primitive_name_to_uid, ),
              generic_args: generic_args,
              maybe_parent_concrete: None,
              maybe_parent_impl: None
            }
          };
        if let Some(whitelist_type_uids) = maybe_whitelist_type_uids {
          if !whitelist_type_uids.contains(&result.valtype.id) {
            return Err(Unsupported(format!("Didn't contain whitelisted tuple")));
          }
        }
        result
      }
      Type::Slice(inner) => {
        return Err(SimplifyError::Unsupported("Encountered slice type".to_owned()));
        println!("generics: {:?}", generics);
        println!("slice inner: {:?}", inner);
        let inner_simple_type =
            simplify_type(crates, &item_index, defaulted_generic_runes, maybe_whitelist_type_uids, generics, type_crate_name, inner)?;
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype: SimpleValType {
            id: unimplemented!(),// slice_id(&item_index.primitive_name_to_uid, ),
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
              let needle_start = "impl ".to_owned() + &trait_.path;
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
            GenericBound::Use(_) => unimplemented!(),
          }
        } else {
          unimplemented!();
        }
      }
      Type::Infer => unimplemented!(),
      Type::RawPointer { .. } => {
        return Err(SimplifyError::Unsupported("Encountered raw pointer type".to_owned()));
      }
      Type::QualifiedPath { .. } => {
        return Err(SimplifyError::Unsupported("Encountered QualifiedPath".to_owned()));
      }
    };
  if let Some(whitelist_type_uids) = maybe_whitelist_type_uids {
    assert!(whitelist_type_uids.contains(&result.valtype.id));
  }
  Ok(result)
}

fn simplify_generic_arg_inner(
  crates: &HashMap<String, Crate>,
  item_index: &ItemIndex,
  // context_container: Option<&SimpleValType>,
  defaulted_generic_runes: &HashSet<String>,
  maybe_whitelist_type_uids: Option<&HashSet<UId>>,
  generics: &HashMap<String, SimpleType>,
  arg_crate_name: &str,
  arg: &GenericArg
) -> anyhow::Result<SimpleType, SimplifyError> {
  match arg {
    GenericArg::Lifetime(_) => {
      return Err(SimplifyError::Unsupported("Encountered lifetime generic arg".to_owned()));
      Ok(
        SimpleType {
          imm_ref: false,
          mut_ref: false,
          valtype: SimpleValType {
            id: unimplemented!(),// lifetime_id(),
            generic_args: Vec::new(),
            maybe_parent_concrete: None,
            maybe_parent_impl: None
          }
        })
    },
    GenericArg::Type(type_) => Ok(simplify_type(crates, &item_index, defaulted_generic_runes, maybe_whitelist_type_uids, generics, arg_crate_name, type_)?),
    GenericArg::Const(_) => unimplemented!(),
    GenericArg::Infer => unimplemented!(),
  }
}
