# Migration-drive targets

- [x] simplifying::test::hammer_test::local_ids_unique
- [x] integration_tests::tests::hammer_tests::simple_main
- [x] integration_tests::tests::hammer_tests::tests_export_function
- [x] integration_tests::tests::hammer_tests::tests_export_struct
- [x] integration_tests::tests::hammer_tests::tests_export_interface
- [x] integration_tests::tests::hammer_tests::two_templated_structs_make_it_into_hamuts
- [x] integration_tests::tests::hammer_tests::tests_stripping_things_after_panic
- [~] integration_tests::tests::hammer_tests::panic_in_expr — BLOCKED on typing-pass solver fix. Designed and verified end-to-end (Scala 1104/1104 ✓, Rust resolves solver blocker, then panics downstream at instantiator.rs:5188 cascade). See `investigations/coord_send_some_branch_fix.md`. Fix is reverted pending coordinated landing with Scala upstream.
- [x] integration_tests::tests::hammer_tests::tests_exports_from_two_modules_different_names
- [ ] integration_tests::tests::hammer_tests::top_level_extern_functions_wire_format_simple_id_has_flat_shape
