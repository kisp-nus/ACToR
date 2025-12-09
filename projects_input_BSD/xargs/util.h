#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>
#include <stdint.h>

// === copied from compat/include/compat.h BEGIN (Line 158) ===
long long strtonum (const char *, long long, long long, const char **);
// === copied from compat/include/compat.h END (Line 158) ===

// === copied from compat/include/compat.h BEGIN (Line 208, 213) ===
size_t strlcat (char *, const char *, size_t);
size_t strlcpy (char *, const char *, size_t);
// === copied from compat/include/compat.h END (Line 208, 213) ===

#endif /* UTIL_H */