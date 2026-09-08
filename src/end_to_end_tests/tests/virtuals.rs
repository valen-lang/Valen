use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn interfaceimm() { assert_compile_and_run(&p("programs/virtuals/interfaceimm.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn interfacemut() { assert_compile_and_run(&p("programs/virtuals/interfacemut.vale"), 42); }
