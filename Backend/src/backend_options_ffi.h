// POD C struct shared with FrontendRust across the backend FFI. Field
// order and types must stay in sync with the Rust mirror in
// FrontendRust/src/backend_ffi/mod.rs.

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
  uint8_t force_all_known_live;
  uint8_t print_mem_overhead;
  uint8_t enable_replaying;

  // Replay-whitelist entries as two parallel arrays of length
  // `replay_whitelist_count`. Each pair is (module_name, extern_name).
  size_t replay_whitelist_count;
  const char* const* replay_whitelist_modules;
  const char* const* replay_whitelist_functions;
} BackendCompileOptionsFFI;

#ifdef __cplusplus
} // extern "C"
#endif

#endif
