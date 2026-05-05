#include <stdint.h>
#include "coordinator/IsDarwin.h"

int8_t coordinator_IsDarwin(void) {
#ifdef __APPLE__
  return 1;
#else
  return 0;
#endif
}
