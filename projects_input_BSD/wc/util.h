#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>
#include <stdint.h>

// === copied from compat/include/compat.h BEGIN (Line 230) ===
#define MAXBSIZE (64 * 1024)
// === copied from compat/include/compat.h END (Line 230) ===

// === copied from compat/include/compat.h BEGIN (Line 237) ===
#define FMT_SCALED_STRSIZE 7 /* minus sign, 4 digits, suffix, null byte */
// === copied from compat/include/compat.h END (Line 237) ===

// === copied from compat/include/compat.h BEGIN (Line 55) ===
extern const char *__progname;
// === copied from compat/include/compat.h END (Line 55) ===

// === copied from compat/include/compat.h BEGIN (Line 176-177) ===
int scan_scaled (char *, long long *);
int fmt_scaled (long long, char *);
// === copied from compat/include/compat.h END (Line 176-177) ===

// === copied from compat/include/compat.h BEGIN (Line 213) ===
size_t strlcpy (char *, const char *, size_t);
// === copied from compat/include/compat.h END (Line 213) ===

#endif /* UTIL_H */