#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>

// === copied from compat/common/strtonum.c BEGIN (Line 29-31) ===
long long strtonum(const char *numstr, long long minval, long long maxval,
    const char **errstrp);
// === copied from compat/common/strtonum.c END (Line 29-31) ===

// === copied from compat/unkown_brother/strlcpy.c BEGIN (Line 28-29) ===
size_t strlcpy(char *dst, const char *src, size_t dsize);
// === copied from compat/unkown_brother/strlcpy.c END (Line 28-29) ===

// === copied from compat/common/progname.c BEGIN (Line 12, 15-16, 103-104) ===
extern const char *__progname;
const char* getprogname(void);
void setprogname(const char *progname);
// === copied from compat/common/progname.c END (Line 12, 15-16, 103-104) ===

#endif