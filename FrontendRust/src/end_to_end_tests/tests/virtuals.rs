use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] fn interfaceimm() { assert_compile_and_run(&p("programs/virtuals/interfaceimm.vale"), 42); }
#[test] fn interfacemut() { assert_compile_and_run(&p("programs/virtuals/interfacemut.vale"), 42); }
