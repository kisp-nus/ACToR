#ifndef SORT_UTIL_H
#define SORT_UTIL_H

#include <stddef.h>

// === copied from compat/include/compat.h BEGIN (Line 1-50) ===
// Program name functions
extern const char *__progname;
const char *getprogname(void);
void setprogname(const char *progname);
// === copied from compat/include/compat.h END (Line 1-50) ===

// === copied from compat/common/heapsort.c and compat/common/merge.c BEGIN ===
// BSD sorting functions
int heapsort(void *vbase, size_t nmemb, size_t size, int (*compar)(const void *, const void *));
int mergesort(void *base, size_t nmemb, size_t size, int (*cmp)(const void *, const void *));
// === copied from compat/common/heapsort.c and compat/common/merge.c END ===

#endif /* SORT_UTIL_H */