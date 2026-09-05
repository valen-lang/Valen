// Repro for the reverse-callback panic Pearl hit: an imported Rust trait whose method takes TWO
// imported-type borrow params and returns void. Compiling the synthesized interface's abstract method
// header panics in get_inner_env_for_type (None) for one of the imported param types. Minimal shape:
// two distinct opaque imported structs, both as `&` params, void return.

pub struct Alpha {}
pub struct Beta {}

impl Alpha {
    pub fn new() -> Alpha {
        Alpha {}
    }
    pub fn touch(&self) {}
    /// The generic caller is a **method** (like NobiliaWindow::main_loop<C>), not a free fn — this is
    /// the shape Pearl's program uses, and the difference from the passing free-fn repro.
    pub fn run_cb<C: Cb>(&self, c: &C) -> i32 {
        let b = Beta::new();
        c.go(self, &b);
        7
    }
}

impl Beta {
    pub fn new() -> Beta {
        Beta {}
    }
}

pub trait Cb {
    fn go(&self, x: &Alpha, y: &Beta);
}
