# Typing Pass Files Still Needing Slice Pipeline

52 files in `src/typing/` that have not yet been sliced.

- [x] citizen/struct_compiler.rs
- [x] citizen/struct_compiler_core.rs
- [x] citizen/struct_compiler_generic_args_layer.rs
- [x] compiler.rs
- [x] compiler_error_humanizer.rs
- [x] compiler_outputs.rs
- [x] env/environment.rs
- [x] env/function_environment_t.rs
- [x] env/i_env_entry.rs
- [x] expression/block_compiler.rs
- [x] expression/call_compiler.rs
- [x] expression/expression_compiler.rs
- [x] expression/local_helper.rs
- [x] expression/pattern_compiler.rs
- [x] function/destructor_compiler.rs
- [x] function/function_body_compiler.rs
- [x] function/function_compiler.rs
- [x] function/function_compiler_closure_or_light_layer.rs
- [x] function/function_compiler_core.rs
- [x] function/function_compiler_middle_layer.rs
- [x] function/function_compiler_solving_layer.rs
- [x] function/virtual_compiler.rs
- [x] infer/compiler_solver.rs
- [x] infer_compiler.rs
- [x] macros/abstract_body_macro.rs
- [x] macros/anonymous_interface_macro.rs
- [x] macros/as_subtype_macro.rs
- [x] macros/citizen/interface_drop_macro.rs
- [x] macros/citizen/struct_drop_macro.rs
- [x] macros/functor_helper.rs
- [x] macros/lock_weak_macro.rs
- [x] macros/macros.rs
- [x] macros/rsa/rsa_drop_into_macro.rs
- [x] macros/rsa/rsa_immutable_new_macro.rs
- [x] macros/rsa/rsa_len_macro.rs
- [x] macros/rsa/rsa_mutable_capacity_macro.rs
- [x] macros/rsa/rsa_mutable_new_macro.rs
- [x] macros/rsa/rsa_mutable_pop_macro.rs
- [x] macros/rsa/rsa_mutable_push_macro.rs
- [x] macros/rsa_len_macro.rs
- [x] macros/same_instance_macro.rs
- [x] macros/ssa/ssa_drop_into_macro.rs
- [x] macros/ssa/ssa_len_macro.rs
- [x] macros/struct_constructor_macro.rs
- [x] names/name_translator.rs
- [x] names/names.rs
- [x] overload_resolver.rs
- [x] templata/conversions.rs
- [x] templata/templata.rs
- [x] templata/templata_utils.rs
- [x] templata_compiler.rs
- [x] types/types.rs

Please:

1. Pick the next one that doesn't have an `[x]`.
2. /slice-pipeline on the one you picked.
3. Tell the user about any complications that arose, or anything else they might be interested in.
