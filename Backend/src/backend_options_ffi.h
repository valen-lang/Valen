// POD C struct shared with FrontendRust across the backend FFI. Field
// order and types must stay in sync with the Rust mirror in
// src/backend_ffi/mod.rs.

#ifndef backend_options_ffi_h
#define backend_options_ffi_h

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Optimization level, matching ValeOptimizationLevel in valeopts.h.
#define BACKEND_OPT_LEVEL_O0  0
#define BACKEND_OPT_LEVEL_O1  1
#define BACKEND_OPT_LEVEL_O2  2
#define BACKEND_OPT_LEVEL_O2i 3
#define BACKEND_OPT_LEVEL_O3  4

typedef struct BackendCompileOptionsFFI {
  // NUL-terminated UTF-8 strings owned by the caller. Empty ("") means
  // "use the LLVM/default value" for triple and cpu.
  const char* output_dir;
  const char* triple;
  const char* cpu;

  int32_t opt_level;
  uint8_t pic;
  uint8_t verify;
  uint8_t print_asm;
  uint8_t print_llvmir;
  uint8_t census;
  uint8_t flares;
  uint8_t include_bounds_checks;
  uint8_t use_atomic_rc;
  uint8_t print_mem_overhead;
  // Emit DWARF debug info (`--debug`). Drives per-function/statement/local DWARF
  // emission in codegen. See docs/architecture/debugging-architecture.md.
  uint8_t debug;
} BackendCompileOptionsFFI;

// Compile mode selector for BackendInputsFFI.mode.
#define BACKEND_MODE_STANDALONE 0
#define BACKEND_MODE_INTEROP    1

// One Rust→Vale callback wrapper the interop backend must emit: `symbol` is the
// rustc-mangled name to define the wrapper under (so Rust's static call resolves to
// Vale's body — single-symbol), `vale_name` the humanized name of the internal Vale
// body to forward to (also its key in the program's extern-ABI map). Field order and
// types must stay in sync with CallbackFFIRaw in src/backend_ffi/backend_inputs.rs.
typedef struct CallbackFFI {
  const char* symbol;
  const char* vale_name;
} CallbackFFI;

// Interop-only inputs, read by the backend only when
// BackendInputsFFI.mode == BACKEND_MODE_INTEROP: rustc's borrowed LLVMContext +
// Module, the rustc-mangled symbol to emit the entry under ("" or null → the
// literal __vale_main), and the Rust→Vale callback wrappers to emit.
typedef struct InteropInputsFFI {
  void* context;
  void* module;
  const char* entry_symbol;
  const CallbackFFI* callbacks;   // num_callbacks-long array, or null when none
  size_t num_callbacks;
} InteropInputsFFI;

// One source file's `basename` (as it rides on a metal SourceLocation.filePath) paired with its
// absolute on-disk `abspath`, so the DWARF emitter can record a resolvable DW_AT_comp_dir instead of
// ".". Field order and types must stay in sync with SourceFilePathFFIRaw in
// src/backend_ffi/backend_inputs.rs.
typedef struct SourceFilePathFFI {
  const char* basename;
  const char* abspath;
} SourceFilePathFFI;

// The single unified backend entry payload. `mode` selects which fields are read:
// standalone reads only cache/program/options; interop additionally reads `interop`.
// Field order and types must stay in sync with the Rust mirror BackendInputsFFIRaw
// in src/backend_ffi/backend_inputs.rs.
typedef struct BackendInputsFFI {
  void* cache;                     // MetalCacheHandle*
  void* program;                   // ProgramHandle*
  BackendCompileOptionsFFI options;
  int32_t mode;                    // BACKEND_MODE_*
  InteropInputsFFI interop;        // read only when mode == BACKEND_MODE_INTEROP
  // (basename, abspath) for each source file, or null/0 when none. Used only for DWARF comp_dir.
  const SourceFilePathFFI* source_paths;
  size_t num_source_paths;
} BackendInputsFFI;

#ifdef __cplusplus
} // extern "C"
#endif

#endif
