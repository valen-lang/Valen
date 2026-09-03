// Borrow-checker tests, re-enabled against the rewritten checker; their diagnostics now point at the
// use site. `util` holds the shared harness helpers.
mod attack_tests;
mod ellipsis_tests;
mod held_register_tests;
mod joint_argument_move_tests;
mod joint_argument_tests;
mod robustness_tests;
mod same_group_aliasing_tests;
mod use_after_churn_tests;
mod walk_completeness_tests;
mod util;
