use std::io::Write;
use std::marker::PhantomData;
use crate::interner::StrI;
use crate::final_ast::ast::ProgramH;
use crate::testvm::values::{CallIdV, PrimitiveKindV, ReferenceV};
use crate::testvm::heap::HeapV;
use crate::von::ast::IVonData;

type PrintStream = std::io::Stdout;

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
#[derive(PartialEq, Eq, Hash)]
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
#[derive(PartialEq, Eq, Hash)]
pub struct ConstraintViolatedExceptionV {
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

object Vivem {
*/
// mig: fn execute_with_primitive_args
pub fn execute_with_primitive_args<'v, 'h, 's>(program_h: &'h ProgramH<'s, 'h>, external_argument_kinds: &'v [PrimitiveKindV<'v, 'h, 's>], vivem_dout: &PrintStream, stdin: &dyn Fn() -> StrI<'s>, stdout: &dyn Fn(StrI<'s>)) -> IVonData {
    panic!("Unimplemented: execute_with_primitive_args")
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
pub fn execute_with_heap<'v, 'h, 's>(program_h: &'h ProgramH<'s, 'h>, input_heap: &HeapV<'v, 's, 'h>, input_argument_references: &'v [ReferenceV<'s, 'h>], vivem_dout: &PrintStream, stdin: &dyn Fn() -> StrI<'s>, stdout: &dyn Fn(StrI<'s>)) -> IVonData {
    panic!("Unimplemented: execute_with_heap")
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
    panic!("Unimplemented: regular_stdout")
}
/*
  def regularStdout(str: String) = {
    print(str)
  }
*/
// mig: fn stdin_from_list
pub fn stdin_from_list<'v, 'h, 's>(stdin_list: &'v [StrI<'s>]) -> Box<dyn Fn() -> StrI<'s>> {
    panic!("Unimplemented: stdin_from_list")
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
pub fn stdout_collector<'v, 'h, 's>() -> (String, Box<dyn Fn(StrI<'s>)>) {
    panic!("Unimplemented: stdout_collector")
}
/*
  def stdoutCollector(): (StringBuilder, String => Unit) = {
    val stdoutput = new StringBuilder()
    val func = (str: String) => { print(str); stdoutput.append(str); }: Unit
    (stdoutput, func)
  }
*/
// mig: fn inner_execute
pub fn inner_execute<'v, 'h, 's>(program_h: &'h ProgramH<'s, 'h>, argument_references: &'v [ReferenceV<'s, 'h>], heap: &HeapV<'v, 's, 'h>, vivem_dout: &PrintStream, stdin: &dyn Fn() -> StrI<'s>, stdout: &dyn Fn(StrI<'s>)) -> IVonData {
    panic!("Unimplemented: inner_execute")
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
