#ifndef UTIL_H
#define UTIL_H

#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <limits.h>

// === copied from compat/common/strtonum.c BEGIN (Line 29-66) ===
long long strtonum(const char *numstr, long long minval, long long maxval,
    const char **errstrp);
// === copied from compat/common/strtonum.c END (Line 29-66) ===

// === copied from compat/include/compat.h BEGIN (Line 46) ===
extern const char *__progname;
// === copied from compat/include/compat.h END (Line 46) ===

#endif /* UTIL_H */