#include <stdint.h>

extern int64_t __main_num_args;
extern char** __main_args;
int64_t __vale_numMainArgs() {
  return __main_num_args;
}

int32_t __vale_rt_get_main_arg_len(int64_t i) {
  const char* arg = __main_args[i];
  int32_t len = 0;
  while (arg[len] != '\0') len++;
  return len;
}

const char* __vale_rt_get_main_arg_ptr(int64_t i) {
  return __main_args[i];
}
