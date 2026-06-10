use std::io::Write;
use std::marker::PhantomData;
use crate::interner::StrI;
use crate::final_ast::ast::ProgramH;
use crate::testvm::values::{CallIdV, PrimitiveKindV, ReferenceV};
use crate::testvm::heap::HeapV;
use crate::von::ast::IVonData;
use crate::final_ast::ast::FunctionH;
use crate::final_ast::types::LocationH;
use crate::final_ast::types::OwnershipH;
use crate::scout_arena::ScoutArena;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::testvm::expression_vivem::discard;
use crate::testvm::function_vivem::execute_function;
use crate::testvm::values::KindV;
use std::cell::RefCell;
use std::rc::Rc;

pub type PrintStream = dyn Write;

/*
package dev.vale.testvm

import dev.vale.finalast._
import dev.vale.{Result, vassert, vassertSome, vcurious, vfail, vpass}

import java.io.PrintStream
import dev.vale.finalast.ProgramH
import dev.vale.von.IVonData

import scala.collection.immutable.List
*/
// mig: struct PanicExceptionV
/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct PanicExceptionV;
/*
case class PanicException() extends Throwable {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for PanicExceptionV` below.)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for PanicExceptionV` below.)
/*
override def equals(obj: Any): Boolean = vcurious();
  vpass()
}
*/
// mig: struct ConstraintViolatedExceptionV
/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ConstraintViolatedExceptionV<'s>
{
    pub msg: StrI<'s>,
}
/*
case class ConstraintViolatedException(msg: String) extends Throwable {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ConstraintViolatedExceptionV` below.)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ConstraintViolatedExceptionV` below.)
/*
override def equals(obj: Any): Boolean = vcurious();
  vpass()
}
*/
// (no scala counterpart — Rust adaptation: enum wrapping the testvm exception structs so vm errors bubble as Result instead of Scala's throw/catch. Carries only the 's lifetime — the vm/heap lifetimes from the original phantoms were vacuous and prevent the Result from outliving the eval_for_* boundary.)
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum VmRuntimeErrorV<'s>
{
    PanicException(PanicExceptionV),
    ConstraintViolatedException(ConstraintViolatedExceptionV<'s>),
}
/* */
/*

object Vivem {
*/
// mig: fn execute_with_primitive_args
pub fn execute_with_primitive_args<'v, 'h, 's>(program_h: &'h ProgramH<'s, 'h>, interner: &HammerInterner<'s, 'h>, scout_arena: &ScoutArena<'s>, external_argument_kinds: &'v [PrimitiveKindV<'v, 'h, 's>], vivem_dout: &'v mut PrintStream, vivem_bump: &'v bumpalo::Bump, stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>)) -> Result<IVonData, VmRuntimeErrorV<'s>> {
    let mut heap = HeapV::new(interner, vivem_dout, vivem_bump);
    let arg_references: &'v [ReferenceV<'v, 'h, 's>] =
        vivem_bump.alloc_slice_fill_iter(
            external_argument_kinds.iter().map(|arg_kind| {
                heap.add(interner, OwnershipH::MutableShareH, LocationH::InlineH, KindV::from(*arg_kind))
            }));
    inner_execute(program_h, interner, scout_arena, arg_references, &mut heap, stdin, stdout)
}
/*
  def executeWithPrimitiveArgs(
      programH: ProgramH,
      externalArgumentKinds: Vector[PrimitiveKindV],
      vivemDout: PrintStream,
      stdin: () => String,
      stdout: String => Unit): IVonData = {
    val heap = new Heap(vivemDout)
    val argReferences =
      externalArgumentKinds.map(argKind => {
        heap.add(MutableShareH, InlineH, argKind);
      });
    innerExecute(programH, argReferences, heap, vivemDout, stdin, stdout)
  }
*/
// mig: fn execute_with_heap
pub fn execute_with_heap<'v, 'h, 's>(program_h: &'h ProgramH<'s, 'h>, interner: &HammerInterner<'s, 'h>, scout_arena: &ScoutArena<'s>, input_heap: &mut HeapV<'v, 'h, 's>, input_argument_references: &'v [ReferenceV<'v, 'h, 's>], stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>)) -> Result<IVonData, VmRuntimeErrorV<'s>> {
    assert_eq!(input_heap.count_unreachable_allocations(interner, input_argument_references), 0);
    inner_execute(program_h, interner, scout_arena, input_argument_references, input_heap, stdin, stdout)
}
/*
  def executeWithHeap(
      programH: ProgramH,
      inputHeap: Heap,
      inputArgumentReferences: Vector[ReferenceV],
      vivemDout: PrintStream,
      stdin: () => String,
      stdout: String => Unit):
  IVonData = {
    vassert(inputHeap.countUnreachableAllocations(inputArgumentReferences) == 0)
    innerExecute(programH, inputArgumentReferences, inputHeap, vivemDout, stdin, stdout)
  }
*/
// mig: fn empty_stdin
pub fn empty_stdin<'v, 'h, 's>() -> StrI<'s> {
    panic!("Unimplemented: empty_stdin")
}
/*
  def emptyStdin() = {
    vfail("Empty stdin!")
  }
*/
// mig: fn null_stdout
pub fn null_stdout<'v, 'h, 's>(str: StrI<'s>) {
    panic!("Unimplemented: null_stdout")
}
/*
  def nullStdout(str: String) = {
  }
*/
// mig: fn regular_stdout
pub fn regular_stdout<'v, 'h, 's>(str: StrI<'s>) {
    print!("{}", str.0);
}
/*
  def regularStdout(str: String) = {
    print(str)
  }
*/
// mig: fn stdin_from_list
pub fn stdin_from_list<'s>(stdin_list: &[StrI<'s>]) -> Box<dyn Fn() -> StrI<'s> + 's> {
    let remaining_stdin = RefCell::new(stdin_list.to_vec());
    let stdin: Box<dyn Fn() -> StrI<'s> + 's> = Box::new(move || {
        let mut r = remaining_stdin.borrow_mut();
        assert!(!r.is_empty());
        let result = r[0];
        r.remove(0);
        result
    });
    stdin
}
/*
  def stdinFromList(stdinList: Vector[String]) = {
    var remainingStdin = stdinList
    val stdin = (() => {
      vassert(remainingStdin.nonEmpty)
      val result = remainingStdin.head
      remainingStdin = remainingStdin.tail
      result
    })
    stdin
  }
*/
// mig: fn stdout_collector
pub fn stdout_collector<'s>() -> (Rc<RefCell<String>>, Box<dyn Fn(StrI<'s>)>) {
    let stdoutput = Rc::new(RefCell::new(String::new()));
    let stdoutput_clone = stdoutput.clone();
    let func: Box<dyn Fn(StrI<'s>)> = Box::new(move |s: StrI<'s>| {
        print!("{}", s.0);
        stdoutput_clone.borrow_mut().push_str(s.0);
    });
    (stdoutput, func)
}
/*
  def stdoutCollector(): (StringBuilder, String => Unit) = {
    val stdoutput = new StringBuilder()
    val func = (str: String) => { print(str); stdoutput.append(str); }: Unit
    (stdoutput, func)
  }
*/
// mig: fn inner_execute
pub fn inner_execute<'v, 'h, 's>(program_h: &'h ProgramH<'s, 'h>, interner: &HammerInterner<'s, 'h>, scout_arena: &ScoutArena<'s>, argument_references: &'v [ReferenceV<'v, 'h, 's>], heap: &mut HeapV<'v, 'h, 's>, stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>)) -> Result<IVonData, VmRuntimeErrorV<'s>> {
    let mains: Vec<&'h FunctionH<'s, 'h>> =
        program_h.packages.package_coord_to_contents.iter().flat_map(|(_package_coord, paackage)| {
            paackage.export_name_to_function.iter()
                .find(|(name, _prototype)| name.0 == "main")
                .map(|(_name, prototype)| {
                    paackage.functions.iter().find(|f| f.prototype == *prototype)
                        .expect("main prototype not found in functions")
                })
                .into_iter()
                .collect::<Vec<_>>()
        }).collect();
    let main = match mains.as_slice() {
        [] => panic!("No main func!"),
        [m] => *m,
        _ => panic!("inner_execute: multiple mains"),
    };
    let _call_id = CallIdV { call_depth: 0, function: main.prototype, _phantom: PhantomData };

    {
        write!(heap.vivem_dout, "Making stack frame").unwrap();
        writeln!(heap.vivem_dout).unwrap();
    }

    let (callee_call_id, retuurn) =
        execute_function(program_h, interner, scout_arena, stdin, stdout, heap, argument_references, main)?;
    let return_ref = retuurn.return_ref;

    {
        write!(heap.vivem_dout, "Ending program").unwrap();
    }

    let von = heap.to_von(return_ref);
    discard(program_h, interner, scout_arena, heap, stdout, stdin, callee_call_id, main.prototype.return_type, return_ref)?;
    {
        writeln!(heap.vivem_dout).unwrap();
    }
    println!("Checking for leaks");
    heap.check_for_leaks();
    {
        writeln!(heap.vivem_dout).unwrap();
    }
    Ok(von)
}
/*
  def innerExecute(
      programH: ProgramH,
      argumentReferences: Vector[ReferenceV],
      heap: Heap,
      vivemDout: PrintStream,
      stdin: () => String,
      stdout: String => Unit): IVonData = {
    val main =
      programH.packages.flatMap({ case (packageCoord, paackage) =>
        paackage.exportNameToFunction.find(_._1.str == "main")
          .map({ case (name, prototype) =>
            vassertSome(paackage.functions.find(_.prototype == prototype))
          }).toVector
      }).flatten.toVector match {
        case Vector() => vfail("No main func!")
        case Vector(m) => m
        case other => vfail(other)
      }

    val callId = CallId(0, main.prototype)

    vivemDout.print("Making stack frame")
    vivemDout.println()

    val (calleeCallId, retuurn) =
      FunctionVivem.executeFunction(programH, stdin, stdout, heap, argumentReferences, main)
    val returnRef = retuurn.returnRef

    vivemDout.print("Ending program")

    val von = heap.toVon(returnRef)
    ExpressionVivem.discard(programH, heap, stdout, stdin, calleeCallId, main.prototype.returnType, returnRef)
    vivemDout.println()
    println("Checking for leaks")
    heap.checkForLeaks()
    vivemDout.println()
    von
  }
}

*/
