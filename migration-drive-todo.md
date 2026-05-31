# Todo — CL bucket (closures-heavy) — 64 tests

## Phase 1 — foundations (21 tests)

- [ ] simple_lambda *(integration_tests_a.rs)*
- [ ] lambda_with_one_magic_arg *(integration_tests_a.rs)*
- [ ] lambda_with_a_type_specified_param *(integration_tests_a.rs)*
- [ ] test_closure_s_local_variables *(closure_tests.rs)*
- [ ] test_returning_a_nonmutable_closured_variable_from_the_closure *(closure_tests.rs)*
- [ ] read_from_inside_a_closure_inside_a_closure *(closure_tests.rs)*
- [ ] mutates_from_inside_a_closure *(closure_tests.rs)*
- [ ] mutates_from_inside_a_closure_inside_a_closure *(closure_tests.rs)*
- [ ] mutable_lambda *(closure_tests.rs)*
- [ ] addressibility *(closure_tests.rs)*
- [ ] lambda_can_call_sibling_lambda *(integration_tests_a.rs)*
- [ ] tests_lambda *(integration_tests_b.rs)*
- [ ] tests_double_closure *(integration_tests_b.rs)*
- [ ] simple_static_array_and_runtime_index_lookup *(array_tests.rs)*
- [ ] new_rsa *(array_tests.rs)*
- [ ] test_array_length *(array_tests.rs)*
- [ ] array_foreach *(array_tests.rs)*
- [ ] array_has *(array_tests.rs)*
- [ ] mutate_array *(array_tests.rs)*
- [ ] change_mutability *(array_tests.rs)*
- [ ] returning_static_array_from_function_and_dotting_it *(array_tests.rs)*

### → CHECKPOINT: stop here, wait for the other three worktrees to finish their Phase 1, 4-way merge, then resume.

## Phase 2 — rest (43 tests)

- [ ] captured_own_is_borrow *(closure_tests.rs)*
- [ ] simple_array_map_with_runtime_index_lookup *(array_tests.rs)*
- [ ] take_arraysequence_as_a_parameter *(array_tests.rs)*
- [ ] borrow_arraysequence_as_a_parameter *(array_tests.rs)*
- [ ] each_on_ssa *(array_tests.rs)*
- [ ] array_map_with_int *(array_tests.rs)*
- [ ] array_map_with_lambda *(array_tests.rs)*
- [ ] make_array_map_with_lambda *(array_tests.rs)*
- [ ] array_map_taking_a_closure_which_captures_something *(array_tests.rs)*
- [ ] map_using_array_construct *(array_tests.rs)*
- [ ] nested_array *(array_tests.rs)*
- [ ] nested_imm_arrays *(array_tests.rs)*
- [ ] two_dimensional_array *(array_tests.rs)*
- [ ] array_with_capture *(array_tests.rs)*
- [ ] swap_out_of_array *(array_tests.rs)*
- [ ] make_array_map_with_struct *(array_tests.rs)*
- [ ] array_map_with_interface *(array_tests.rs)*
- [ ] destroy_ssa_of_imms_into_function *(array_tests.rs)*
- [ ] destroy_rsa_of_imms_into_function *(array_tests.rs)*
- [ ] destroy_ssa_of_muts_into_function *(array_tests.rs)*
- [ ] destroy_rsa_of_muts_into_function *(array_tests.rs)*
- [ ] migrate_rsa *(array_tests.rs)*
- [ ] migrate_ssa *(array_tests.rs)*
- [ ] unspecified_mutability_static_array_from_lambda_defaults_to_mutable *(array_tests.rs)*
- [ ] immutable_static_array_from_lambda *(array_tests.rs)*
- [ ] mutable_static_array_from_lambda *(array_tests.rs)*
- [ ] immutable_static_array_from_values *(array_tests.rs)*
- [ ] mutable_static_array_from_values *(array_tests.rs)*
- [ ] unspecified_mutability_runtime_array_from_lambda_defaults_to_mutable *(array_tests.rs)*
- [ ] immutable_runtime_array_from_lambda *(array_tests.rs)*
- [ ] mutable_runtime_array_from_lambda *(array_tests.rs)*
- [ ] new_immutable_array *(array_tests.rs)*
- [ ] capture *(array_tests.rs)*
- [ ] capture_mutable_array *(array_tests.rs)*
- [ ] map_from_hardcoded_values *(array_tests.rs)*
- [ ] reports_when_making_new_imm_rsa_without_lambda *(array_tests.rs)*
- [ ] test_array_push_pop_len_capacity_drop *(integration_tests_b.rs)*
- [ ] tests_generic_with_a_lambda *(integration_tests_b.rs)*
- [ ] tests_generic_s_lambda_calling_parent_function_s_bound *(integration_tests_b.rs)*
- [ ] tests_generic_with_a_polymorphic_lambda *(integration_tests_b.rs)*
- [ ] tests_generic_with_a_polymorphic_lambda_invoked_twice *(integration_tests_b.rs)*
- [ ] tests_a_foreach_for_a_linked_list *(integration_tests_b.rs)*
- [ ] exporting_array *(integration_tests_c.rs)*
