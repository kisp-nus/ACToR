#ifndef UTIL_H
#define UTIL_H

#include <stddef.h>

// === copied from compat/include/compat.h BEGIN (Line 1-50) ===
extern const char *__progname;
const char *getprogname(void);
void setprogname(const char *progname);
// === copied from compat/include/compat.h END (Line 1-50) ===

// === copied from compat/common/reallocarray.c BEGIN (Line 1-50) ===
void *reallocarray(void *ptr, size_t nmemb, size_t size);
// === copied from compat/common/reallocarray.c END (Line 1-50) ===

#endif /* UTIL_H */