//! A dependency-free stub of NobiliaV's `nobiliav` crate, holding only the public API that
//! `driver_check.valen` imports. Every body is `unimplemented!()` — the crate exists to reproduce a
//! typing-pass crash in the Valen interop (see ../README.md), not to render anything. The real crate's
//! headless core, renderer, and winit/wgpu window shell are all gone with its dependencies.

mod window;
pub use window::{MainLoopCallback, NobiliaWindow};

/// One frame's input. A Valen driver reads it through these `&self` accessors, since it cannot read an
/// opaque imported struct's fields directly. The fields are kept (plain primitives) so the type is a
/// faithful stand-in; the accessor bodies are stubbed.
pub struct FrameInput {
    pub quit: bool,
    pub key: i32,
    pub mouse_x: i32,
    pub mouse_y: i32,
}

impl FrameInput {
    /// Whether this frame asks the loop to quit (window close / Esc).
    pub fn quit(&self) -> bool {
        return self.quit;
    }

    /// The key pressed this frame by its code, or `-1` for none.
    pub fn key(&self) -> i32 {
        return self.key;
    }

    /// The x of a click this frame, or `-1` for none.
    pub fn mouse_x(&self) -> i32 {
        return self.mouse_x;
    }

    /// The y of a click this frame, or `-1` for none.
    pub fn mouse_y(&self) -> i32 {
        return self.mouse_y;
    }
}

// The `i32` arrow-key codes a Valen driver compares against `FrameInput::key()`.

/// The key code for the left arrow.
pub fn key_arrow_left() -> i32 {
    return 1;
}

/// The key code for the right arrow.
pub fn key_arrow_right() -> i32 {
    return 2;
}

/// The key code for the up arrow.
pub fn key_arrow_up() -> i32 {
    return 3;
}

/// The key code for the down arrow.
pub fn key_arrow_down() -> i32 {
    return 4;
}
