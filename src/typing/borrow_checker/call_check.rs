use crate::interner::StrI;
use crate::postparsing::ast::{FunctionS, ParameterS};
use crate::postparsing::names::{CodeRuneS, IRuneS};
use crate::postparsing::rules::rules::RuneUsage;
use crate::postparsing::rules::types::{BorrowRefST, EffectS, GroupS, ITypeST, RegionS};
use crate::typing::ast::expressions::FunctionCallTE;
use crate::typing::borrow_checker::borrow_error::BorrowErrorKind;
use crate::typing::borrow_checker::place_path::{moved_root, place_path};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::names::names::IdValT;
use crate::utils::range::RangeS;

/// A parameter's declared group, as its group-rune name, when the parameter is `&T in <rune>` with a
/// declared group rune. `None` for a non-group / non-borrow parameter, or a group that resolved to a
/// local rather than a declared rune.
pub(super) fn param_group_name<'s>(param: &ParameterS<'s>) -> Option<StrI<'s>> {
  match &param.tyype {
    ITypeST::BorrowRef(BorrowRefST {
      region:
        RegionS::Group(GroupS::Rune(RuneUsage { rune: IRuneS::CodeRune(CodeRuneS { name }), .. })),
      ..
    }) => Some(*name),
    _ => None,
  }
}

/// Whether the callee declares `mut(<group>)` over the group named `group`.
pub(super) fn is_mut_target<'s>(effects: &[EffectS<'s>], group: StrI<'s>) -> bool {
  effects.iter().any(|effect| match effect {
    EffectS::Mut(GroupS::Rune(RuneUsage { rune: IRuneS::CodeRune(CodeRuneS { name }), .. })) => {
      *name == group
    }
    _ => false,
  })
}

/// The scout `FunctionS` of a call's callee — its declared parameters, groups, and effects. `None`
/// when the callee has no postparsed function on record (e.g. a builtin the borrow checker skips).
pub(super) fn resolve_callee<'s, 'ctx, 't>(
  call: &FunctionCallTE<'s, 't>,
  coutputs: &CompilerOutputs<'s, 't>,
  compiler: &Compiler<'s, 'ctx, 't>,
) -> Option<&'s FunctionS<'s>> {
  let template_id_val = Compiler::get_function_template(compiler.typing_interner, call.callable.id);
  let template_id = compiler.typing_interner.intern_id(IdValT {
    package_coord: template_id_val.package_coord,
    init_steps: template_id_val.init_steps,
    local_name: template_id_val.local_name,
  });
  coutputs.peek_postparsed_function(template_id)
}

/// Check one call's arguments jointly:
/// - a pair bound to parameters in *distinct* named groups, at least one of which the callee
///   mutates, must not alias — the callee is entitled to treat those groups as disjoint;
/// - a borrow argument must not be rooted in a local another argument moves, or it would dangle.
pub fn check_call<'s, 'ctx, 't>(
  call: &FunctionCallTE<'s, 't>,
  caller_range: RangeS<'s>,
  coutputs: &CompilerOutputs<'s, 't>,
  compiler: &Compiler<'s, 'ctx, 't>,
) -> Result<(), ICompileErrorT<'s, 't>> {
  let callee = match resolve_callee(call, coutputs, compiler) {
    Some(callee) => callee,
    None => return Ok(()),
  };

  let arg_count = call.args.len().min(callee.params.len());
  for i in 0..arg_count {
    for j in (i + 1)..arg_count {
      let (Some(group_i), Some(group_j)) =
        (param_group_name(&callee.params[i]), param_group_name(&callee.params[j]))
      else {
        continue;
      };
      if group_i == group_j {
        continue; // same group: aliasing into one group is allowed
      }
      if !is_mut_target(callee.effects, group_i) && !is_mut_target(callee.effects, group_j) {
        continue; // neither group mutated: free immutable aliasing
      }
      let (Some(path_i), Some(path_j)) = (place_path(&call.args[i]), place_path(&call.args[j]))
      else {
        continue;
      };
      if path_i.aliases(&path_j) {
        return Err(BorrowErrorKind::AliasingIntoDisjointMutGroups {
          local: path_i.root,
          arg_a: i,
          arg_b: j,
          group_a: group_i,
          group_b: group_j,
        }
        .at(compiler, caller_range));
      }
    }
  }

  for moved_index in 0..arg_count {
    let Some(moved) = moved_root(&call.args[moved_index]) else {
      continue;
    };
    for borrow_index in 0..arg_count {
      if borrow_index == moved_index {
        continue;
      }
      if let Some(path) = place_path(&call.args[borrow_index]) {
        if path.root == moved {
          return Err(BorrowErrorKind::BorrowIntoMovedArgument {
            local: moved,
            borrow_arg: borrow_index,
            move_arg: moved_index,
          }
          .at(compiler, caller_range));
        }
      }
    }
  }
  Ok(())
}
