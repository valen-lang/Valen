//! The window shell, stubbed: the public `MainLoopCallback` trait and `NobiliaWindow` methods
//! `driver_check.valen` drives. Every body is `unimplemented!()`; the real winit/wgpu event loop is
//! gone with the dependencies.

use crate::FrameInput;

/// A per-frame callback the library invokes each frame. A Valen struct implements this via the
/// rust-interop trait mechanism, and compiling its synthesized interface header — this method's two
/// imported-type borrow params — is what trips the typing crash.
pub trait MainLoopCallback {
    /// React to one frame: read `input`, and call methods on `w`.
    fn on_tick(&self, w: &NobiliaWindow, input: &FrameInput);
}

/// A window a Valen driver builds and hands a callback. Opaque to the interop; the two fields are a
/// plain stand-in for the real (interior-mutable, GPU-backed) state.
pub struct NobiliaWindow {
    pub width: i32,
    pub height: i32,
}

impl NobiliaWindow {
    /// A window of this logical size.
    pub fn new(width: i32, height: i32) -> NobiliaWindow {
        return NobiliaWindow { width, height };
    }

    /// How many frames have run so far.
    pub fn frame_index(&self) -> i32 {
        return 0;
    }

    /// Load the checked-in reference map.
    pub fn load_terrain(&self) {
        
    }

    /// Frame the whole current level at this viewpoint (whole `i32` degrees).
    pub fn fit_camera(&self, _az: i32, _el: i32) {
        
    }

    /// Nudge the camera by whole degrees of azimuth/elevation.
    pub fn rotate_camera(&self, _d_az: i32, _d_el: i32) {
        
    }

    /// Print what a click landed on.
    pub fn report_pick(&self, _x: i32, _y: i32) {
        
    }

    /// Ask the loop to exit after this frame.
    pub fn request_exit(&self) {
        
    }

    /// Run the loop, calling `cb.on_tick(self, &input)` each frame. A generic *method* caller — the
    /// shape that also trips the sibling codegen bug once typing is fixed.
    pub fn main_loop<C: MainLoopCallback>(&self, cb: &C) {
        let input = FrameInput { quit: false, key: -1, mouse_x: -1, mouse_y: -1 };
        cb.on_tick(self, &input);
    }
}
