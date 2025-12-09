#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>

// === copied from compat/common/progname.c BEGIN (Line 1-116) ===
extern const char *__progname;
const char *getprogname(void);
void setprogname(const char *progname);
// === copied from compat/common/progname.c END (Line 1-116) ===

// === copied from unkown_brother/strlcpy.c BEGIN (Line 18-54) ===
size_t strlcpy (char *dst, const char *src, size_t dsize);
// === copied from unkown_brother/strlcpy.c END (Line 18-54) ===

#endif