// The single payload handed to the backend's one compile entry (`backend_compile`).
//
// This file is the *contract* for the instantiator-to-backend boundary: it defines
// exactly what data can cross into the backend. Everything else in `src/backend_ffi/`
// is mechanics (building the metal cache, marshaling, the FFI call). To hand the backend
// a new piece of data, add a field here — which is what keeps that boundary under the
// core compiler's control rather than letting any subsystem smuggle data across it.

use std::ffi::c_void;
use std::os::raw::c_char;

use super::metal_cache::{MetalCache, Program};
use super::{BackendCompileOptions, BackendCompileOptionsFFIRaw};

// Compile mode selector for BackendInputsFFIRaw.mode; matches BACKEND_MODE_* in
// Backend/src/backend_options_ffi.h.
pub(crate) const BACKEND_MODE_STANDALONE: i32 = 0;
pub(crate) const BACKEND_MODE_INTEROP: i32 = 1;

/// C-repr mirror of InteropInputsFFI in Backend/src/backend_options_ffi.h. Read by the
/// backend only when `BackendInputsFFIRaw.mode == BACKEND_MODE_INTEROP`.
#[repr(C)]
pub(crate) struct InteropInputsFFIRaw {
    pub(crate) context: *mut c_void,
    pub(crate) module: *mut c_void,
    // The rustc-mangled entry symbol, or "" for the literal `__vale_main`.
    pub(crate) entry_symbol: *const c_char,
}

/// C-repr mirror of BackendInputsFFI in Backend/src/backend_options_ffi.h. Field order
/// and types must match. Built by `compile` and passed to `backend_compile`.
#[repr(C)]
pub(crate) struct BackendInputsFFIRaw {
    pub(crate) cache: *mut c_void,
    pub(crate) program: *mut c_void,
    pub(crate) options: BackendCompileOptionsFFIRaw,
    pub(crate) mode: i32,
    // Read only when mode == BACKEND_MODE_INTEROP.
    pub(crate) interop: InteropInputsFFIRaw,
}

/// Everything the backend needs for one compile. This is the sole value that crosses into
/// the backend.
pub struct BackendInputs<'a, 'c> {
    pub cache: &'a MetalCache,
    pub program: &'a Program<'c>,
    pub options: BackendCompileOptions,
    pub mode: BackendMode<'a>,
}

/// The two compile modes, one variant each so standalone-only data has a home symmetric
/// to the interop data.
pub enum BackendMode<'a> {
    Standalone(StandaloneInputs),
    Interop(InteropInputs<'a>),
}

/// Standalone-mode inputs. The output/target/optimize settings still ride in
/// `BackendInputs::options`; this variant exists so the two modes are symmetric and so
/// those settings can move here later.
pub struct StandaloneInputs {}

/// Borrowed-mode (rustc interop) inputs: rustc lends its LLVMContext + Module, and names
/// the mangled symbol to emit the entry under.
pub struct InteropInputs<'a> {
    /// rustc's borrowed LLVMContext. Must be a live handle for the whole `compile` call
    /// and must not be disposed (rustc owns its lifecycle).
    pub context: *mut c_void,
    /// rustc's borrowed LLVMModule. Same borrowing contract as `context`.
    pub module: *mut c_void,
    /// The rustc-mangled symbol to emit the entry under, or None for the literal
    /// `__vale_main` (also None for a library with no entry).
    pub entry_symbol: Option<&'a str>,
}
