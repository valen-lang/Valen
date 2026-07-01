
#ifndef valeopts_h
#define valeopts_h

#include <string>
#include <stdint.h>
#include <stddef.h>
#include <unordered_map>
#include <unordered_set>

#include "backend_options_ffi.h"

enum class ValeOptimizationLevel {
    O0,
    O1,
    O2,
    O2i,
    O3
};


// Compiler options. Populated from BackendCompileOptionsFFI via loadFromFfi.
struct ValeOptions {
    std::string outputDir;

    std::string triple;
    std::string cpu;

    ValeOptimizationLevel optLevel = ValeOptimizationLevel::O2i;
    bool pic = false;
    bool verify = false;
    bool print_asm = false;
    bool print_llvmir = false;
    bool census = false;
    bool flares = false;
    bool includeBoundsChecks = true;
    bool useAtomicRc = false;
    bool forceAllKnownLive = false;
    bool printMemOverhead = false;
    bool enableReplaying = false;
    std::unordered_map<std::string, std::unordered_set<std::string>> projectNameToReplayWhitelistedExterns;
};

// Copy fields out of the FFI POD into a ValeOptions. Returns 1 on success,
// 0 or negative on malformed input (only opt_level is validated; all other
// fields are trusted).
int loadFromFfi(ValeOptions *opt, const BackendCompileOptionsFFI *ffi);

#endif
