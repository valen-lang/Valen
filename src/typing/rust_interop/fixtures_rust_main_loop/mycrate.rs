// A Rust dependency crate for the main-loop capstone (slice 9).
//
// Rust owns a loop and calls the Valen callback once per iteration, each with a fresh scalar. This is
// the NobiliaV shape: winit/wgpu (Rust) owns the frame loop and calls Valen's `on_tick` every frame.
// It proves the callback survives repeated re-entry — one wrapper, emitted once, invoked N times.

pub trait Looper {
    fn on_tick(&self, i: i32) -> i32;
}

/// Rust owns the loop: it calls the Valen callback five times (i = 0..5) and sums the returns.
pub fn main_loop<C: Looper>(c: &C) -> i32 {
    let mut total = 0;
    for i in 0..5 {
        total += c.on_tick(i);
    }
    total
}
