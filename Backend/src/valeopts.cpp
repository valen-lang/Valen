
#include "valeopts.h"

#include <cassert>
#include <iostream>

int loadFromFfi(ValeOptions *opt, const BackendCompileOptionsFFI *ffi) {
  assert(ffi != nullptr);
  assert(ffi->output_dir != nullptr);
  assert(ffi->triple != nullptr);
  assert(ffi->cpu != nullptr);

  opt->outputDir = ffi->output_dir;
  opt->triple = ffi->triple;
  opt->cpu = ffi->cpu;

  switch (ffi->opt_level) {
    case BACKEND_OPT_LEVEL_O0:  opt->optLevel = ValeOptimizationLevel::O0;  break;
    case BACKEND_OPT_LEVEL_O1:  opt->optLevel = ValeOptimizationLevel::O1;  break;
    case BACKEND_OPT_LEVEL_O2:  opt->optLevel = ValeOptimizationLevel::O2;  break;
    case BACKEND_OPT_LEVEL_O2i: opt->optLevel = ValeOptimizationLevel::O2i; break;
    case BACKEND_OPT_LEVEL_O3:  opt->optLevel = ValeOptimizationLevel::O3;  break;
    default:
      std::cerr << "Bad opt_level: " << ffi->opt_level << std::endl;
      return -1;
  }

  opt->pic = ffi->pic != 0;
  opt->verify = ffi->verify != 0;
  opt->print_asm = ffi->print_asm != 0;
  opt->print_llvmir = ffi->print_llvmir != 0;
  opt->census = ffi->census != 0;
  opt->flares = ffi->flares != 0;
  opt->includeBoundsChecks = ffi->include_bounds_checks != 0;
  opt->useAtomicRc = ffi->use_atomic_rc != 0;
  opt->forceAllKnownLive = ffi->force_all_known_live != 0;
  opt->printMemOverhead = ffi->print_mem_overhead != 0;
  opt->enableReplaying = ffi->enable_replaying != 0;

  for (size_t i = 0; i < ffi->replay_whitelist_count; i++) {
    assert(ffi->replay_whitelist_modules[i] != nullptr);
    assert(ffi->replay_whitelist_functions[i] != nullptr);
    std::string moduleName = ffi->replay_whitelist_modules[i];
    std::string functionName = ffi->replay_whitelist_functions[i];
    opt->projectNameToReplayWhitelistedExterns[moduleName].insert(functionName);
  }

  return 1;
}
