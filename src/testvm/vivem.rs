use std::io::Write;
use crate::interner::StrI;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::testvm::values::{PrimitiveKindV, ReferenceV};
use crate::testvm::heap::HeapV;
use crate::testvm::von::IVonData;
use crate::instantiating::ast::ast::FunctionDefinitionI;
use crate::testvm::values::OwnershipV;
use crate::scout_arena::ScoutArena;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::testvm::expression_vivem::discard;
use crate::testvm::function_vivem::execute_function;
use crate::testvm::values::KindV;
use std::cell::RefCell;
use std::rc::Rc;

pub type PrintStream = dyn Write;



/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct PanicExceptionV;


// (Realized by `impl Hash for PanicExceptionV` below.)


// (Realized by `impl PartialEq for PanicExceptionV` below.)


/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ConstraintViolatedExceptionV<'s>
{
    pub msg: StrI<'s>,
}


// (Realized by `impl Hash for ConstraintViolatedExceptionV` below.)


// (Realized by `impl PartialEq for ConstraintViolatedExceptionV` below.)

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum VmRuntimeErrorV<'s>
{
    PanicException(PanicExceptionV),
    ConstraintViolatedException(ConstraintViolatedExceptionV<'s>),
}



pub fn execute_with_primitive_args<'v, 'i, 's>(program_h: &'i HinputsI<'s, 'i>, interner: &InstantiatingInterner<'s, 'i>, scout_arena: &ScoutArena<'s>, external_argument_kinds: &'v [PrimitiveKindV<'v, 'i, 's>], vivem_dout: &'v mut PrintStream, vivem_bump: &'v bumpalo::Bump, stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>)) -> Result<IVonData, VmRuntimeErrorV<'s>> {
    let mut heap = HeapV::new(interner, vivem_dout, vivem_bump);
    let arg_references: &'v [ReferenceV<'v, 'i, 's>] =
        vivem_bump.alloc_slice_fill_iter(
            external_argument_kinds.iter().map(|arg_kind| {
                heap.add(interner, OwnershipV::Own, KindV::from(*arg_kind))
            }));
    inner_execute(program_h, interner, scout_arena, arg_references, &mut heap, stdin, stdout)
}


pub fn execute_with_heap<'v, 'i, 's>(program_h: &'i HinputsI<'s, 'i>, interner: &InstantiatingInterner<'s, 'i>, scout_arena: &ScoutArena<'s>, input_heap: &mut HeapV<'v, 'i, 's>, input_argument_references: &'v [ReferenceV<'v, 'i, 's>], stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>)) -> Result<IVonData, VmRuntimeErrorV<'s>> {
    assert_eq!(input_heap.count_unreachable_allocations(interner, input_argument_references), 0);
    inner_execute(program_h, interner, scout_arena, input_argument_references, input_heap, stdin, stdout)
}


pub fn empty_stdin<'v, 'i, 's>() -> StrI<'s> {
    panic!("Unimplemented: empty_stdin")
}


pub fn null_stdout<'v, 'i, 's>(str: StrI<'s>) {
    panic!("Unimplemented: null_stdout")
}


pub fn regular_stdout<'v, 'i, 's>(str: StrI<'s>) {
    print!("{}", str.0);
}


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


pub fn stdout_collector<'s>() -> (Rc<RefCell<String>>, Box<dyn Fn(StrI<'s>)>) {
    let stdoutput = Rc::new(RefCell::new(String::new()));
    let stdoutput_clone = stdoutput.clone();
    let func: Box<dyn Fn(StrI<'s>)> = Box::new(move |s: StrI<'s>| {
        print!("{}", s.0);
        stdoutput_clone.borrow_mut().push_str(s.0);
    });
    (stdoutput, func)
}


pub fn inner_execute<'v, 'i, 's>(program_h: &'i HinputsI<'s, 'i>, interner: &InstantiatingInterner<'s, 'i>, scout_arena: &ScoutArena<'s>, argument_references: &'v [ReferenceV<'v, 'i, 's>], heap: &mut HeapV<'v, 'i, 's>, stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>)) -> Result<IVonData, VmRuntimeErrorV<'s>> {
    let mains: Vec<&'i FunctionDefinitionI<'s, 'i>> =
        program_h.function_exports.iter()
            .filter(|export| export.exported_name.0 == "main")
            .map(|export| {
                *program_h.functions.iter().find(|f| f.header.id == export.prototype.id)
                    .expect("main prototype not found in functions")
            })
            .collect();
    let main = match mains.as_slice() {
        [] => panic!("No main func!"),
        [m] => *m,
        _ => panic!("inner_execute: multiple mains"),
    };

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
    discard(program_h, interner, scout_arena, heap, stdout, stdin, callee_call_id, main.header.return_type, return_ref)?;
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

