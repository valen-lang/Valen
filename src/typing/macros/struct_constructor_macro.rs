use crate::interner::StrI;
use crate::postparsing::ast::FunctionS;
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::ast::*;
use crate::postparsing::ast::{GeneratedBodyS, IBodyS, IStructMemberS, ParameterS};
use crate::postparsing::itemplatatype::{
  FunctionTemplataType, ITemplataType, KindTemplataType, TemplateTemplataType,
};
use crate::postparsing::names::CodeNameS;
use crate::scout_arena::ScoutArena;
use crate::postparsing::names::CodeVarNameS;
use crate::postparsing::names::IFunctionDeclarationNameS;
use crate::postparsing::names::{
  ConstructorNameS, ICitizenDeclarationNameS, IImpreciseNameS, INameS, INameValS, IRuneValS,
  IStructDeclarationNameS, IVarDeclarationNameS, ReturnRuneS, StructNameRuneS,
};
use crate::postparsing::patterns::patterns::{AtomSP, CaptureS};
use crate::postparsing::rules::rules::{CallSR, IRulexSR, LookupSR, RuneUsage};
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::macros::macros::GeneratedAhtDenizen;
use crate::typing::names::names::IdValT;
use crate::typing::names::names::*;
use crate::typing::templata::templata::*;
use crate::typing::templata_compiler::{peel_all_references, IBoundArgumentsSource};
use crate::typing::types::types::*;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::utils::range::RangeS;

// A constructor param mirrors a user-written param: it carries the member's @PFVSZ split verbatim, so
// downstream (e.g. §2A's expected-value-type-template scan) sees a constructor param exactly as it would
// a hand-written one. Pure: reads only the member's fields and calls the sealed ParameterS::new.
// VCOORD: inline?
fn parameter_from_normal_member<'s>(scout_arena: &ScoutArena<'s>, member: &NormalStructMemberS<'s>, lid: LocationInDenizen<'s>) -> ParameterS<'s> {
  ParameterS::new(
    member.range,
    None,
    false,
    IVarDeclarationNameS::CodeVarName(CodeVarNameS { imprecise_name: scout_arena.intern_code_name(member.name), lid }),
    member.tyype,
    member.type_rune,
    member.value_type_rune,
    member.type_outer_ref_rules,
    member.value_type_rules,
  )
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn get_struct_sibling_entries_struct_constructor(
    &self,
    struct_name: IdT<'s, 't>,
    struct_a: &'s StructS<'s>,
  ) -> Vec<GeneratedAhtDenizen<'s, 't>> {
    if struct_a.members.iter().any(|m| matches!(m, IStructMemberS::VariadicStructMember(_))) {
      // Dont generate constructors for variadic structs, not supported yet.
      // Only one we have right now is tuple, which has its own special syntax for constructing.
      return vec![];
    }
    let mut rules: Vec<IRulexSR<'s>> = Vec::new();

    // We dont need these, they really just contain bounds and stuff, which we'd inherit from our parameters anyway.
    // However, if we leave it out, then this (from an IRAGP test):
    //   struct Bork<T, Y> where T = Y { t T; y Y; }
    // thing's constructor would be:
    //   func Bork<T, Y>(t T, y Y) Bork<T, Y> { ... }
    // and it fails to resolve that return type there because it doesn't meet the struct's conditions, because it didn't
    // repeat the rules from the struct's header, specifically the T = Y rule.
    // So, we just include all the rules from the constructor's header.
    // If we ever need to drop that functionality (the T = Y nonsense) then we can probably take out the inheriting of
    // the header rules.
    for r in struct_a.header_rules.iter() {
      rules.push(*r);
    }

    let struct_name_range = struct_a.name.range();
    let ret_rune_s = self.scout_arena.intern_rune(IRuneValS::ReturnRune(ReturnRuneS {}));
    let ret_rune = RuneUsage { range: struct_name_range, rune: ret_rune_s };

    let struct_name_as_citizen: ICitizenDeclarationNameS<'s> = struct_a.name.into();
    let struct_generic_rune_s =
      self.scout_arena.intern_rune(IRuneValS::StructNameRune(StructNameRuneS {
        struct_name: struct_name_as_citizen,
      }));
    let struct_generic_rune = RuneUsage { range: struct_name_range, rune: struct_generic_rune_s };

    let struct_imprecise_name = struct_a.name.get_imprecise_name(self.scout_arena);
    rules.push(IRulexSR::Lookup(LookupSR {
      range: struct_name_range,
      rune: struct_generic_rune,
      parts: self.scout_arena.alloc_slice_copy(&[struct_imprecise_name]),
    }));

    // Instantiate the struct template; the resulting kind is the constructor's return type,
    // since an owned value is a bare kind.
    let generic_param_runes: Vec<_> = struct_a.generic_params.iter().map(|p| p.rune).collect();
    let generic_param_runes_slice = self.scout_arena.alloc_slice_copy(&generic_param_runes);
    rules.push(IRulexSR::Call(CallSR {
      range: struct_name_range,
      result_rune: ret_rune,
      template_rune: struct_generic_rune,
      args: generic_param_runes_slice,
    }));

    // Each param is a declaration in the constructor denizen (root LID `[]`), so it gets its own
    // child LID `[1]`, `[2]`, ... — LIDs start at 1 and are never 0. Without a distinct LID every
    // param would collapse to the same life and collide (see typing-design.md).
    let params: Vec<ParameterS<'s>> = struct_a
      .members
      .iter()
      .enumerate()
      .flat_map(|(index, m)| match m {
        IStructMemberS::NormalStructMember(member) => {
          let lid = LocationInDenizen {
            path: self.scout_arena.alloc_slice_copy(&[(index + 1) as i32]),
          };
          vec![parameter_from_normal_member(self.scout_arena, member, lid)]
        }
        IStructMemberS::VariadicStructMember(_) => vec![],
      })
      .collect();

    let params_slice = self.scout_arena.alloc_slice_from_vec(params);
    let rules_slice = self.scout_arena.alloc_slice_copy(&rules);
    // A constructor's imprecise name is the citizen's spelling (a `MyStruct(...)` call resolves as
    // `CodeName{"MyStruct"}`); its lid is the synthesized denizen root seed. Built directly. A
    // function declaration name is identity, not interned (@WVSBIZ).
    let constructor_imprecise_name = match struct_imprecise_name {
      IImpreciseNameS::CodeName(cn) => cn,
      _ => panic!("struct constructor macro: expected a CodeName struct imprecise name"),
    };
    let constructor_name_s = IFunctionDeclarationNameS::ConstructorName(self.scout_arena.alloc(
      ConstructorNameS {
        tlcd: struct_name_as_citizen,
        imprecise_name: constructor_imprecise_name,
        lid: LocationInDenizen { path: &[] },
      },
    ));
    let function_a = self.scout_arena.alloc(FunctionS::new(
      struct_a.range,
      constructor_name_s,
      &[],
      struct_a.generic_params,
      TemplateTemplataType {
        param_types: struct_a.tyype.param_types,
        return_type: self
          .scout_arena
          .alloc(ITemplataType::FunctionTemplataType(FunctionTemplataType {})),
      },
      params_slice,
      Some(ret_rune),
      // A synthesized constructor carries no effect clause.
      &[],
      rules_slice,
      &[],
      &[],
      self.scout_arena.alloc(IBodyS::GeneratedBody(GeneratedBodyS {
        generator_id: self.keywords.struct_constructor_generator,
      })),
    ));
    let function_name_s = INameS::FunctionDeclaration(
      self.scout_arena.alloc_function_declaration_name(constructor_name_s),
    );
    let translated_local_name = self.translate_name_step(function_name_s);
    let result_template_id_ref = self.typing_interner.intern_id(IdValT {
      package_coord: struct_name.package_coord,
      init_steps: struct_name.init_steps,
      local_name: translated_local_name,
    });
    vec![GeneratedAhtDenizen::Function(result_template_id_ref, function_a)]
  }

  pub fn generate_function_body_struct_constructor(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    env: &'t FunctionEnvironmentT<'s, 't>,
    generator_id: StrI<'s>,
    loct: LocT<'t>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    origin_function: Option<&FunctionS<'s>>,
    param_coords: &[ParameterT<'s, 't>],
    maybe_ret_coord: Option<KindT<'s, 't>>,
  ) -> (FunctionHeaderT<'s, 't>, ExpressionTE<'s, 't>) {
    let ret_coord = maybe_ret_coord.expect("vassertSome: maybeRetCoord");
    // The return coord arrives ShareRef-wrapped for a share citizen (see the share-wrap in
    // function_compiler_core); peel to the struct kind to construct it. The return type is
    // re-wrapped from sharedness below.
    // VCOORD: revisit this
    let struct_tt = match peel_all_references(ret_coord) {
      KindT::Struct(s) => s,
      _ => panic!("Expected struct kind in generate_function_body_struct_constructor"),
    };
    let definition = coutputs.lookup_struct(*struct_tt.id, self);
    let instantiation_bound_params = definition.instantiation_bound_params;
    let instantiation_bounds = coutputs
      .get_instantiation_bounds(self.typing_interner, *struct_tt.id)
      .expect("vassertSome: getInstantiationBounds");
    let bound_arguments_source = IBoundArgumentsSource::UseBoundsFromContainer {
      instantiation_bound_params,
      instantiation_bound_arguments: instantiation_bounds,
    };
    let members: Vec<(IVarNameT<'s, 't>, KindT<'s, 't>)> = {
      let placeholder_substituter = self.get_placeholder_substituter(
        false, // sanity_check
        &env.template_id,
        struct_tt.id,
        bound_arguments_source,
      );
      definition
        .members
        .iter()
        .map(|member| {
          (IVarNameT::Member(member.name), placeholder_substituter.substitute_for_kind(coutputs, member.tyype))
        })
        .collect()
    };

    let constructor_id = env.id;
    assert!(
      constructor_id.local_name.parameters().len() == members.len(),
      "vassert: constructorId.localName.parameters.size == members.size"
    );

    let constructor_params: Vec<ParameterT<'s, 't>> = members
      .iter()
      .map(|(name, coord)| ParameterT {
        name: *name,
        virtuality: None,
        pre_checked: false,
        tyype: *coord,
      })
      .collect();

    let bound_arguments_source2 = IBoundArgumentsSource::UseBoundsFromContainer {
      instantiation_bound_params,
      instantiation_bound_arguments: instantiation_bounds,
    };
    let mutability = self.struct_compiler_get_sharedness(
      false, // sanity_check
      coutputs,
      env.template_id,
      RegionT::Default,
      *struct_tt,
      bound_arguments_source2,
    );
    // A share citizen is only ever held ShareRef-wrapped; a single one is held bare.
    let constructor_return_type = match mutability {
      SharednessT::Single => KindT::Struct(struct_tt),
      SharednessT::Shared => {
        KindT::ShareRef(self.typing_interner.alloc(ShareRefT { inner: KindT::Struct(struct_tt) }))
      }
    };

    let constructor_params_slice = self.typing_interner.alloc_slice_from_vec(constructor_params);
    let header = FunctionHeaderT {
      id: constructor_id,
      attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
      params: constructor_params_slice,
      return_type: constructor_return_type,
      maybe_origin_function_templata: Some(env.templata()),
    };

    let args: Vec<ExpressionTE<'s, 't>> = constructor_params_slice
      .iter()
      .enumerate()
      .map(|(index, p)| {
        ExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE::new(index as i32, p.tyype)))
      })
      .collect();
    let args_slice = self.typing_interner.alloc_slice_from_vec(args);
    let struct_tt_ref = self.typing_interner.alloc(struct_tt);
    let construct_expr = ExpressionTE::Construct(self.typing_interner.alloc(ConstructTE::new(
      struct_tt_ref,
      constructor_return_type,
      args_slice,
    )));
    let return_expr =
      ExpressionTE::Return(self.typing_interner.alloc(ReturnTE::new(construct_expr)));
    let body = ExpressionTE::Block(self.typing_interner.alloc(BlockTE::new(return_expr)));
    (header, body)
  }
}
