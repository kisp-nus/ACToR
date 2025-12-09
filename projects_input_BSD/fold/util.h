#ifndef UTIL_H
#define UTIL_H

#include <stddef.h>

// === copied from compat/common/strtonum.c BEGIN (Line 29-31) ===
long long strtonum(const char *numstr, long long minval, long long maxval,
    const char **errstrp);
// === copied from compat/common/strtonum.c END (Line 29-31) ===

// === copied from compat/darwin/reallocarray.c BEGIN (Line 31-32) ===
void *reallocarray(void *ptr, size_t nmemb, size_t size);
// === copied from compat/darwin/reallocarray.c END (Line 31-32) ===

#endif /* UTIL_H */