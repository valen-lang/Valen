// From Frontend/SimplifyingPass/src/dev/vale/simplifying/FunctionHammer.scala
//
// Per typing-pass `Compiler` precedent, `FunctionHammer` is not a Rust
// struct. Methods become `impl Hammer { ... }` blocks colocated here.

use std::collections::{HashMap, HashSet};
use crate::final_ast::ast::FunctionRefH;
use crate::final_ast::ast::IFunctionAttributeH;
use crate::instantiating::ast::ast::{ExternI, FunctionDefinitionI, FunctionHeaderI, IFunctionAttributeI};
use crate::instantiating::ast::ast::{PrototypeI};
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::types::cI;
use crate::simplifying::hamuts::Hamuts;
use crate::simplifying::hammer::Hammer;
use crate::final_ast::ast::FunctionH;
use crate::final_ast::types::KindHT;
use crate::instantiating::ast::ast::PrototypeIValI;
use crate::instantiating::ast::expressions::ExpressionIE;
use crate::simplifying::hammer::Locals;

/*
package dev.vale.simplifying

import dev.vale.finalast.{FunctionH, Local, PureH, UserFunctionH, VariableIdH}
import dev.vale.{Keywords, vassert, vfail, vimpl, vwat, finalast => m}
import dev.vale.finalast._
import dev.vale.instantiating.ast._

class FunctionHammer(
    keywords: Keywords,
    typeHammer: TypeHammer,
    nameHammer: NameHammer,
    structHammer: StructHammer) {
  val expressionHammer =
    new ExpressionHammer(keywords, typeHammer, nameHammer, structHammer, this)
*/

// mig: fn translate_functions
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_functions(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        functions2: &[&'i FunctionDefinitionI<'s, 'i>],
    ) -> Vec<FunctionRefH<'s, 'h>>
    {
        let mut previous_functions_h: Vec<FunctionRefH<'s, 'h>> = Vec::new();
        for function2 in functions2.iter() {
            let function_h = self.translate_function(hinputs, hamuts, function2);
            let mut new_vec = vec![function_h];
            new_vec.append(&mut previous_functions_h);
            previous_functions_h = new_vec;
        }
        previous_functions_h
    }
/*
  def translateFunctions(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    functions2: Vector[FunctionDefinitionI]):
  (Vector[FunctionRefH]) = {
    functions2.foldLeft((Vector[FunctionRefH]()))({
      case ((previousFunctionsH), function2) => {
        val (functionH) = translateFunction(hinputs, hamuts, function2)
        Vector(functionH) ++ previousFunctionsH
      }
    })
  }
*/

// mig: fn translate_function
    pub fn translate_function(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        function2: &'i FunctionDefinitionI<'s, 'i>,
    ) -> FunctionRefH<'s, 'h>
    {
        let header_prototype = self.instantiating_interner.intern_prototype_ci(PrototypeIValI { id: function2.header.id, return_type: function2.header.return_type });
        if let Some(function_ref_h) = hamuts.function_refs.get(header_prototype) {
            return *function_ref_h;
        }
        let header = &function2.header;
        let attrs2 = header.attributes;
        let body = &function2.body;
        let prototype_h = self.translate_prototype(hinputs, hamuts, header_prototype);
        let temporary_function_ref_h = FunctionRefH { prototype: prototype_h };
        hamuts.forward_declare_function(header_prototype, temporary_function_ref_h);
        let mut locals = Locals {
            typing_pass_locals: HashMap::new(),
            unstackified_vars: HashSet::new(),
            locals: HashMap::new(),
            next_local_id_number: 1,
        };
        let (body_h, deferreds) = self.translate_expression(hinputs, hamuts, header, &mut locals, ExpressionIE::Reference(*body));
        assert!(deferreds.is_empty());
        assert_eq!(locals.unstackified_vars.len(), locals.locals.len());
        let result_coord = body_h.result_type();
        if result_coord != prototype_h.return_type {
            match result_coord.kind {
                KindHT::NeverHT(_) => {}
                _ => panic!("Result of body's instructions didnt match return type!\nReturn type:   {:?}\nBody's result: {:?}", prototype_h.return_type, result_coord),
            }
        }
        let is_abstract = header.get_abstract_interface().is_some();
        let is_extern = header.attributes.iter().any(|a| matches!(a, IFunctionAttributeI::ExternI(_)));
        let attrs_h: Vec<_> = attrs2.iter().filter(|a| !matches!(a, IFunctionAttributeI::ExternI(_))).cloned().collect();
        let attrs_h_translated = self.translate_function_attributes(&attrs_h);
        let function_h = FunctionH { prototype: prototype_h, is_abstract, is_extern, attributes: self.interner.alloc_slice_from_vec(attrs_h_translated), body: body_h };
        hamuts.add_function(header_prototype, function_h);
        temporary_function_ref_h
    }
/*
  def translateFunction(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    function2: FunctionDefinitionI):
  (FunctionRefH) = {
//    opts.debugOut("Translating function " + function2.header.fullName)
    hamuts.functionRefs.get(function2.header.toPrototype) match {
      case Some(functionRefH) => functionRefH
      case None => {
        val FunctionDefinitionI(
            header @ FunctionHeaderI(humanName, attrs2, params2, returnType2),
            _,
            _,
            body) = function2;

        val (prototypeH) = typeHammer.translatePrototype(hinputs, hamuts, header.toPrototype);
        val temporaryFunctionRefH = FunctionRefH(prototypeH);
        hamuts.forwardDeclareFunction(header.toPrototype, temporaryFunctionRefH)

        val locals =
          LocalsBox(
            Locals(
              Map[IVarNameI[cI], VariableIdH](),
              Set[VariableIdH](),
              Map[VariableIdH,Local](),
              1));
        val (bodyH, Vector()) =
          expressionHammer.translate(hinputs, hamuts, header, locals, body)
        vassert(locals.unstackifiedVars.size == locals.locals.size)
        val resultCoord = bodyH.resultType
        if (resultCoord != prototypeH.returnType) {
          resultCoord.kind match {
            case NeverHT(_) => // meh its fine
            case _ => {
              vfail(
                "Result of body's instructions didnt match return type!\n" +
                  "Return type:   " + prototypeH.returnType + "\n" +
                  "Body's result: " + resultCoord)
            }
          }
        }

        val isAbstract = header.getAbstractInterface.nonEmpty
        val isExtern = header.attributes.exists({ case ExternI(packageCoord) => true case _ => false })
        val attrsH = translateFunctionAttributes(attrs2.filter(a => !a.isInstanceOf[ExternI]))
        val functionH = FunctionH(prototypeH, isAbstract, isExtern, attrsH, bodyH);
        hamuts.addFunction(header.toPrototype, functionH)

        (temporaryFunctionRefH)
      }
    }
  }
*/

// mig: fn translate_function_attributes
    pub fn translate_function_attributes(
        &self,
        attributes: &[IFunctionAttributeI<'s>],
    ) -> Vec<IFunctionAttributeH> {
        attributes.iter().map(|a| match a {
            IFunctionAttributeI::UserFunctionI => IFunctionAttributeH::UserFunctionH,
            IFunctionAttributeI::PureI => IFunctionAttributeH::PureH,
            IFunctionAttributeI::ExternI(_) => panic!("translate_function_attributes: ExternI vwat (should have been filtered)"),
            #[allow(unreachable_patterns)] // mirrors Scala's `case x => vimpl(x.toString)` catch-all
            other => panic!("translate_function_attributes: unimplemented {:?}", other),
        }).collect()
    }
/*
  def translateFunctionAttributes(attributes: Vector[IFunctionAttributeI]) = {
    attributes.map({
      case UserFunctionI => UserFunctionH
      case PureI => PureH
      case ExternI(_) => vwat() // Should have been filtered out, hammer cares about extern directly
      case x => vimpl(x.toString)
    })
  }
*/

// mig: fn translate_function_ref
    pub fn translate_function_ref(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        prototype2: &'i PrototypeI<'s, 'i, cI>,
    ) -> FunctionRefH<'s, 'h>
    {
        let prototype_h = self.translate_prototype(hinputs, hamuts, prototype2);
        FunctionRefH { prototype: prototype_h }
    }
}
/*
  def translateFunctionRef(
      hinputs: HinputsI,
      hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
      prototype2: PrototypeI[cI]):
  (FunctionRefH) = {
    val (prototypeH) = typeHammer.translatePrototype(hinputs, hamuts, prototype2);
    val functionRefH = FunctionRefH(prototypeH);
    (functionRefH)
  }
}
*/
