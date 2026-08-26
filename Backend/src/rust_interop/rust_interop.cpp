// Rust-interop (valec-rs) backend entry points — the C++ side of rustc's
// `fill_extra_modules` hook. rustc lends its LLVMContext + Module (not the
// TargetMachine, by Sky's design); Vale emits its IR into that module and
// returns, leaving optimization and object emission to rustc.

#include <cstdint>
#include <string>

#include <llvm-c/Core.h>
#include <llvm-c/Target.h>

#include "globalstate.h"
#include "valeopts.h"
#include "error.h"
#include "backend_options_ffi.h"
#include "metal/metal_cache_ffi.h"

class MetalCache;
class Program;

// Defined in vale.cpp; the whole-program codegen pieces the standalone path also uses.
// compileValeCode returns the Vale main's prototype when the program exports a `main`
// (binary), or nullptr in library mode.
Prototype* compileValeCode(GlobalState* globalState, MetalCache* metalCache, Program* program);
void finalizeCompile(GlobalState* globalState);
// Defined in mainFunction.cpp; emits an entry symbol that calls the Vale main.
LLVMValueRef makeEntryFunction(
    GlobalState* globalState, Prototype* valeMainPrototype,
    const std::string& entryName, bool emitLibcShim);

// Defined in metal_cache_ffi.cpp; reaches into the opaque CacheOwner wrapper.
extern "C" MetalCache* metal_cache_ffi_inner(MetalCacheHandle*);

// Borrowed-mode compile: rustc lends only its LLVMContext + Module (as opaque handles),
// Vale emits its IR into that module, and control returns to rustc. rustc owns
// optimization and object emission — they run after this — so unlike compileStandalone we
// do NOT optimize, generateOutput, or dispose any of the borrowed handles. We also skip
// the libc `main` entry and main setup/cleanup: rustc's libstd provides `main` (arch 5.6).
// With no machine, GlobalState reads its data layout off the (rustc-preset) module.
int32_t compileIntoModuleFromRustc(
    MetalCache* metalCache, Program* program,
    const BackendCompileOptionsFFI* ffi_opts,
    void* context, void* mod, const char* entrySymbol) {
  ValeOptions valeOptions;
  int ok = loadFromFfi(&valeOptions, ffi_opts);
  if (ok <= 0) {
    return ok == 0 ? 0 : (int32_t)ExitCode::BadOpts;
  }
  auto modRef = reinterpret_cast<LLVMModuleRef>(mod);
  // rustc already set the module's data layout from the target; read it off as-is (no
  // machine to derive it from, and re-deriving could drift from rustc's).
  LLVMTargetDataRef dataLayout = LLVMGetModuleDataLayout(modRef);
  GlobalState globalState(
      &valeOptions,
      reinterpret_cast<LLVMContextRef>(context),
      modRef,
      /*machine=*/nullptr,
      dataLayout);
  // A `main` export makes this a Vale binary — emit `__vale_main` for the stub's Rust
  // `fn main` to call (rustc's libstd owns the real libc `main`, so no libc shim). No
  // `main` is a Vale library: exported functions only, no entry.
  Prototype* valeMainPrototype = compileValeCode(&globalState, metalCache, program);
  if (valeMainPrototype != nullptr) {
    // Single-symbol (arch §5.2): emit the entry under rustc's mangled __vale_main symbol when the
    // driver supplied one, so the stub's `fn main` (which calls the Rust name __vale_main) links to
    // Vale's real body rather than rustc's unreachable!() placeholder (removed by the partition
    // filter). Empty = the literal __vale_main (standalone, or no explicit symbol).
    std::string entryName =
        (entrySymbol != nullptr && entrySymbol[0] != '\0') ? std::string(entrySymbol) : "__vale_main";
    makeEntryFunction(&globalState, valeMainPrototype, entryName, /*emitLibcShim=*/false);
  }
  // generateExports (the C-ABI export headers/sources) is deliberately NOT called here: it
  // is the standalone-valec C-FFI boundary, it requires an outputDir and writes files to
  // disk, and interop exports go through single-symbol instead (Vale bodies emitted under
  // rustc-mangled names). So the interop path emits IR into the module only.
  finalizeCompile(&globalState);
  return 0;
}

// FFI entry for the `fill_extra_modules` hook: rustc lends its LLVMContext + Module as
// opaque `void*` (not the TargetMachine). Caller retains ownership of everything (nothing
// is freed here).
extern "C" __attribute__((visibility("default")))
int32_t backend_compile_program_into(
    MetalCacheHandle* cacheH, ProgramHandle* programH,
    const BackendCompileOptionsFFI* ffi_opts,
    void* context, void* mod, const char* entrySymbol) {
  return compileIntoModuleFromRustc(
      metal_cache_ffi_inner(cacheH),
      reinterpret_cast<Program*>(programH),
      ffi_opts, context, mod, entrySymbol);
}
