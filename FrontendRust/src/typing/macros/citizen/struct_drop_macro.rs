use crate::interner::StrI;
use crate::utils::range::RangeS;
use crate::postparsing::names::*;
use crate::higher_typing::ast::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compiler::Compiler;
use crate::typing::templata::templata::*;
use crate::postparsing::ast::LocationInDenizen;

/*
package dev.vale.typing.macros.citizen

import dev.vale.highertyping._
import dev.vale.postparsing.patterns._
import dev.vale.postparsing.rules._
import dev.vale._
import dev.vale.postparsing._
import dev.vale.typing.ast._
import dev.vale.typing.env.{FunctionEnvEntry, FunctionEnvironmentT, FunctionEnvironmentBoxT, ReferenceLocalVariableT}
import dev.vale.typing._
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.function.DestructorCompiler
import dev.vale.typing.macros.{IFunctionBodyMacro, IOnStructDefinedMacro}
import dev.vale.typing.names._
import dev.vale.typing.types._
import dev.vale.typing.ast._
import dev.vale.typing.macros.IOnStructDefinedMacro
import dev.vale.typing.names.INameT
import dev.vale.typing.types._
import dev.vale.typing.templata._

import scala.collection.mutable
*/
// (Scala `class StructDropMacro(opts, interner, keywords, nameTranslator, destructorCompiler)`
//  absorbed onto `Compiler`; the three method bodies live at
//  `Compiler::get_struct_sibling_entries_struct_drop`,
//  `Compiler::make_implicit_drop_function_struct_drop`, and
//  `Compiler::generate_function_body_struct_drop` below.)
/*
class StructDropMacro(
  opts: TypingPassOptions,
  interner: Interner,
  keywords: Keywords,
  nameTranslator: NameTranslator,
  destructorCompiler: DestructorCompiler
) extends IOnStructDefinedMacro with IFunctionBodyMacro {

  val macroName: StrI = keywords.DeriveStructDrop

  val dropGeneratorId: StrI = keywords.dropGenerator
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_struct_sibling_entries_struct_drop(
        &self,
        struct_name: IdT<'s, 't>,
        struct_a: &'s StructA<'s>,
    ) -> Vec<(IdT<'s, 't>, IEnvEntryT<'s, 't>)> {
        panic!("Unimplemented: get_struct_sibling_entries_struct_drop");
    }
/*
  override def getStructSiblingEntries(
    structName: IdT[INameT], structA: StructA):
  Vector[(IdT[INameT], FunctionEnvEntry)] = {
    def range(n: Int) = RangeS.internal(interner, n)
    def use(n: Int, rune: IRuneS) = RuneUsage(range(n), rune)

    val rules = new Accumulator[IRulexSR]()
    // Use the same rules as the original struct, see MDSFONARFO.
    structA.headerRules.foreach(r => rules.add(r))
    val runeToType = mutable.HashMap[IRuneS, ITemplataType]()
    // Use the same runes as the original struct, see MDSFONARFO.
    structA.headerRuneToType.foreach(runeToType += _)

    val voidKindRune = MacroVoidKindRuneS()
    runeToType.put(voidKindRune, KindTemplataType())
    rules.add(LookupSR(range(-1672147),use(-64002, voidKindRune),interner.intern(CodeNameS(keywords.void))))
    val voidCoordRune = MacroVoidCoordRuneS()
    runeToType.put(voidCoordRune, CoordTemplataType())
    rules.add(CoerceToCoordSR(range(-1672147),use(-64002, voidCoordRune),use(-64002, voidKindRune)))

    val selfKindTemplateRune = SelfKindTemplateRuneS(structA.range.begin)
    runeToType += (selfKindTemplateRune -> structA.tyype)
    rules.add(
      LookupSR(
        structA.name.range,
        RuneUsage(structA.name.range, selfKindTemplateRune),
        structA.name.getImpreciseName(interner)))

    val selfKindRune = SelfKindRuneS()
    runeToType += (selfKindRune -> KindTemplataType())
    rules.add(
      CallSR(
        structA.name.range,
        use(-64002, selfKindRune),
        RuneUsage(structA.name.range, selfKindTemplateRune),
        structA.genericParameters.map(_.rune).toVector))

    val selfCoordRune = SelfCoordRuneS()
    runeToType += (selfCoordRune -> CoordTemplataType())
    rules.add(
      CoerceToCoordSR(
        structA.name.range,
        RuneUsage(structA.name.range, selfCoordRune),
        RuneUsage(structA.name.range, selfKindRune)))


    // Use the same generic parameters as the struct
    val functionGenericParameters = structA.genericParameters

    val functionTemplataType =
      TemplateTemplataType(
        functionGenericParameters.map(_.rune.rune).map(runeToType),
        FunctionTemplataType())

    val nameS = interner.intern(FunctionNameS(keywords.drop, structA.range.begin))
    val dropFunctionA =
      FunctionA(
        structA.range,
        nameS,
        Vector(),
        functionTemplataType,
        functionGenericParameters,
        runeToType.toMap,
        Vector(
          ParameterS(
            range(-1340),
            None,
            false,
            AtomSP(
              range(-1340),
              Some(CaptureS(interner.intern(CodeVarNameS(keywords.thiss)), false)),
              Some(use(-64002, selfCoordRune)), None))),
        Some(use(-64002, voidCoordRune)),
        rules.buildArray().toVector,
        GeneratedBodyS(dropGeneratorId))

    val dropNameT = structName.addStep(nameTranslator.translateGenericFunctionName(dropFunctionA.name))
    Vector((dropNameT, FunctionEnvEntry(dropFunctionA)))
  }

  // Implicit drop is one made for closures, arrays, or anything else that's not explicitly
  // defined by the user.
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_implicit_drop_function_struct_drop(
        &self,
        drop_or_free_function_name_s: IFunctionDeclarationNameS<'s>,
        struct_range: RangeS<'s>,
    ) -> FunctionA<'s> {
        use crate::postparsing::ast::{ParameterS, GeneratedBodyS, IBodyS};
        use crate::postparsing::patterns::patterns::{CaptureS, AtomSP};
        use crate::postparsing::rules::rules::{RuneUsage, IRulexSR, LookupSR, CoerceToCoordSR};
        use crate::postparsing::itemplatatype::*;
        use crate::utils::range::CodeLocationS;

        let internal_range = |n: i32| {
            let loc = CodeLocationS::internal(self.scout_arena, n);
            RangeS::new(loc, loc)
        };

        let drop_p1_rune = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: self.keywords.drop_p1 }));
        let drop_p1k_rune = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: self.keywords.drop_p1k }));
        let drop_vk_rune = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: self.keywords.drop_vk }));
        let drop_v_rune = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: self.keywords.drop_v }));

        let rune_to_type = self.scout_arena.alloc_index_map_from_iter(vec![
            (drop_p1_rune, ITemplataType::CoordTemplataType(CoordTemplataType {})),
            (drop_p1k_rune, ITemplataType::KindTemplataType(KindTemplataType {})),
            (drop_vk_rune, ITemplataType::KindTemplataType(KindTemplataType {})),
            (drop_v_rune, ITemplataType::CoordTemplataType(CoordTemplataType {})),
        ]);

        let params = self.scout_arena.alloc_slice_from_vec(vec![
            ParameterS::new(
                internal_range(-1342),
                None,
                false,
                AtomSP {
                    range: internal_range(-1342),
                    name: Some(CaptureS { name: IVarNameS::CodeVarName(self.keywords.x), mutate: false }),
                    coord_rune: Some(RuneUsage { range: internal_range(-64002), rune: drop_p1_rune }),
                    destructure: None,
                }),
        ]);

        let maybe_ret_coord_rune = Some(RuneUsage { range: internal_range(-64002), rune: drop_v_rune });

        let self_name_s = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::SelfName(SelfNameS {}));
        let void_name_s = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: self.keywords.void }));

        let rules = self.scout_arena.alloc_slice_from_vec(vec![
            IRulexSR::Lookup(LookupSR {
                range: internal_range(-1672161),
                rune: RuneUsage { range: internal_range(-64002), rune: drop_p1k_rune },
                name: self_name_s,
            }),
            IRulexSR::Lookup(LookupSR {
                range: internal_range(-1672162),
                rune: RuneUsage { range: internal_range(-64002), rune: drop_vk_rune },
                name: void_name_s,
            }),
            IRulexSR::CoerceToCoord(CoerceToCoordSR {
                range: internal_range(-1672162),
                coord_rune: RuneUsage { range: internal_range(-64002), rune: drop_v_rune },
                kind_rune: RuneUsage { range: internal_range(-64002), rune: drop_vk_rune },
            }),
            IRulexSR::CoerceToCoord(CoerceToCoordSR {
                range: internal_range(-1672162),
                coord_rune: RuneUsage { range: internal_range(-64002), rune: drop_p1_rune },
                kind_rune: RuneUsage { range: internal_range(-64002), rune: drop_p1k_rune },
            }),
        ]);

        FunctionA::new(
            struct_range,
            drop_or_free_function_name_s,
            self.scout_arena.alloc_slice_from_vec(vec![]),
            TemplateTemplataType {
                param_types: self.scout_arena.alloc_slice_from_vec(vec![]),
                return_type: self.scout_arena.alloc(ITemplataType::FunctionTemplataType(FunctionTemplataType {})),
            },
            self.scout_arena.alloc_slice_from_vec(vec![]),
            rune_to_type,
            params,
            maybe_ret_coord_rune,
            rules,
            IBodyS::GeneratedBody(GeneratedBodyS { generator_id: self.keywords.drop_generator }),
        )
    }
/*
  def makeImplicitDropFunction(
    dropOrFreeFunctionNameS: IFunctionDeclarationNameS,
    structRange: RangeS):
  FunctionA = {
    FunctionA(
      structRange,
      dropOrFreeFunctionNameS,
      Vector(),
      TemplateTemplataType(Vector(), FunctionTemplataType()),
      Vector(),
      Map(
        CodeRuneS(keywords.DropP1) -> CoordTemplataType(),
        CodeRuneS(keywords.DropP1K) -> KindTemplataType(),
        CodeRuneS(keywords.DropVK) -> KindTemplataType(),
        CodeRuneS(keywords.DropV) -> CoordTemplataType()),
      Vector(
        ParameterS(
          RangeS.internal(interner, -1342),
          None,
          false,
          AtomSP(
            RangeS.internal(interner, -1342),
            Some(CaptureS(interner.intern(CodeVarNameS(keywords.x)), false)),
            Some(RuneUsage(RangeS.internal(interner, -64002), CodeRuneS(keywords.DropP1))),
            None))),
      Some(RuneUsage(RangeS.internal(interner, -64002), CodeRuneS(keywords.DropV))),
      Vector(
        LookupSR(
          RangeS.internal(interner, -1672161),
          RuneUsage(RangeS.internal(interner, -64002), CodeRuneS(keywords.DropP1K)),
          interner.intern(SelfNameS())),
        LookupSR(RangeS.internal(interner, -1672162), RuneUsage(RangeS.internal(interner, -64002), CodeRuneS(keywords.DropVK)), interner.intern(CodeNameS(keywords.void))),
        CoerceToCoordSR(RangeS.internal(interner, -1672162), RuneUsage(RangeS.internal(interner, -64002), CodeRuneS(keywords.DropV)), RuneUsage(RangeS.internal(interner, -64002), CodeRuneS(keywords.DropVK))),
        CoerceToCoordSR(RangeS.internal(interner, -1672162), RuneUsage(RangeS.internal(interner, -64002), CodeRuneS(keywords.DropP1)), RuneUsage(RangeS.internal(interner, -64002), CodeRuneS(keywords.DropP1K)))),
      GeneratedBodyS(dropGeneratorId))
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn generate_function_body_struct_drop(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'t FunctionEnvironmentT<'s, 't>,
        generator_id: StrI<'s>,
        life: LocationInFunctionEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        origin_function1: Option<&'s FunctionA<'s>>,
        params2: &[ParameterT<'s, 't>],
        maybe_ret_coord: Option<CoordT<'s, 't>>,
    ) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
        let body_env = self.typing_interner.alloc(IInDenizenEnvironmentT::Function(env));

        let struct_tt = match params2[0].tyype.kind {
            KindT::Struct(s) => s,
            _ => panic!("struct drop: first param is not a struct"),
        };
        let struct_def = coutputs.lookup_struct(struct_tt.id, self);
        let struct_ownership = match struct_def.mutability {
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => OwnershipT::Own,
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => OwnershipT::Share,
            ITemplataT::Placeholder(_) => OwnershipT::Own,
            _ => panic!("struct drop: unexpected mutability"),
        };
        let struct_type = CoordT { ownership: struct_ownership, region: RegionT {}, kind: KindT::Struct(struct_tt) };

        let ret = CoordT { ownership: OwnershipT::Share, region: RegionT {}, kind: KindT::Void(VoidT {}) };
        let params_arena: &'t [ParameterT<'s, 't>] = self.typing_interner.alloc_slice_from_vec(params2.to_vec());
        let header = FunctionHeaderT {
            id: env.id,
            attributes: &[],
            params: params_arena,
            return_type: ret,
            maybe_origin_function_templata: Some(env.templata()),
        };

        coutputs.declare_function_return_type(
            self.typing_interner.alloc(header.to_signature()), header.return_type);

        let body_expr = match struct_def.mutability {
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => {
                ReferenceExpressionTE::Discard(DiscardTE {
                    expr: self.typing_interner.alloc(ReferenceExpressionTE::ArgLookup(ArgLookupTE { param_index: 0, coord: struct_type })),
                })
            }
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) |
            ITemplataT::Placeholder(_) => {
                panic!("implement: generate_function_body_struct_drop mutable/placeholder case");
            }
            _ => panic!("struct drop: unexpected mutability"),
        };

        let return_expr = self.typing_interner.alloc(
            ReferenceExpressionTE::Return(ReturnTE {
                source_expr: self.typing_interner.alloc(
                    ReferenceExpressionTE::VoidLiteral(VoidLiteralTE { region: RegionT {}, _phantom: std::marker::PhantomData })),
            }));
        let body_expr_ref = self.typing_interner.alloc(body_expr);
        let body = ReferenceExpressionTE::Block(BlockTE {
            inner: self.consecutive(&[body_expr_ref, return_expr]),
        });

        (header, body)
    }
/*
  override def generateFunctionBody(
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    generatorId: StrI,
    life: LocationInFunctionEnvironmentT,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    originFunction1: Option[FunctionA],
    params2: Vector[ParameterT],
    maybeRetCoord: Option[CoordT]):
  (FunctionHeaderT, ReferenceExpressionTE) = {
    val bodyEnv = FunctionEnvironmentBoxT(env)

    val structTT =
      params2.head.tyype.kind match {
        case structTT @ StructTT(_) => structTT
        case other => vwat(other)
      }
    val structDef = coutputs.lookupStruct(structTT.id)
    val structOwnership =
      structDef.mutability match {
        case MutabilityTemplataT(MutableT) => OwnT
        case MutabilityTemplataT(ImmutableT) => ShareT
        case PlaceholderTemplataT(idT, MutabilityTemplataType()) => OwnT
      }
    val structType = CoordT(structOwnership, RegionT(), structTT)

    val ret = CoordT(ShareT, RegionT(), VoidT())
    val header = ast.FunctionHeaderT(env.id, Vector.empty, params2, ret, Some(env.templata))

    coutputs.declareFunctionReturnType(header.toSignature, header.returnType)

    val body =
      BlockTE(
        Compiler.consecutive(
          Vector(
            structDef.mutability match {
              case MutabilityTemplataT(ImmutableT) => DiscardTE(ArgLookupTE(0, structType))
              case MutabilityTemplataT(MutableT) | PlaceholderTemplataT(_, _) => {
                val memberLocalVariables =
                  structDef.members.flatMap({
                    case NormalStructMemberT(name, _, ReferenceMemberTypeT(unsubstitutedReference)) => {
                      val substituter =
                        TemplataCompiler.getPlaceholderSubstituter(
                          opts.globalOptions.sanityCheck,
                          interner, keywords,
                          env.denizenTemplateId,
                          structTT.id,
                          // We received an instance of this type, so we can use the bounds from it.
                          InheritBoundsFromTypeItself)
                      val reference = substituter.substituteForCoord(coutputs, unsubstitutedReference)
                      Vector(ReferenceLocalVariableT(name, FinalT, reference))
                    }
                    case NormalStructMemberT(_, _, AddressMemberTypeT(_)) => {
                      // See Destructure2 and its handling of addressible members for why
                      // we don't include these in the destination variables.
                      Vector.empty
                    }
                    case VariadicStructMemberT(name, tyype) => vimpl()
                  })

                Compiler.consecutive(
                  Vector(DestroyTE(ArgLookupTE(0, structType), structTT, memberLocalVariables)) ++
                    memberLocalVariables.map(v => {
                      destructorCompiler.drop(
                        bodyEnv,
                        coutputs,
                        originFunction1.map(_.range).toList ++ callRange,
                        callLocation,
                        RegionT(),
                        UnletTE(v))
                    }))
              }
            },
            ReturnTE(VoidLiteralTE(RegionT())))))
    (header, body)
  }
}
*/
}
