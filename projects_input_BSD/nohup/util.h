#ifndef UTIL_H
#define UTIL_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

// === copied from compat/unkown_brother/strlcpy.c BEGIN (Line 28-54) ===
size_t strlcpy(char *dst, const char *src, size_t dsize);
// === copied from compat/unkown_brother/strlcpy.c END (Line 28-54) ===

// === copied from compat/linux/strlcat.c BEGIN (Line 30-58) ===
size_t strlcat(char *dst, const char *src, size_t dsize);
// === copied from compat/linux/strlcat.c END (Line 30-58) ===

#endif /* UTIL_H */