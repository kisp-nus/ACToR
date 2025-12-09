#ifndef UTIL_H
#define UTIL_H

#include <stddef.h>
#include <sys/types.h>

/* === copied from compat/include/compat.h BEGIN (Line 86-88) === */
#ifndef u_long
typedef unsigned long u_long;
#endif
/* === copied from compat/include/compat.h END (Line 86-88) === */

/* === copied from compat/include/compat.h BEGIN (Line 54-56) === */
extern const char *__progname;
/* === copied from compat/include/compat.h END (Line 54-56) === */

/* === copied from compat/common/progname.c BEGIN (Line 15-16, 103-115) === */
const char* getprogname(void);
void setprogname(const char *progname);
/* === copied from compat/common/progname.c END (Line 15-16, 103-115) === */

/* === copied from compat/darwin/reallocarray.c BEGIN (Line 31-32) === */
void *reallocarray(void *optr, size_t nmemb, size_t size);
/* === copied from compat/darwin/reallocarray.c END (Line 31-32) === */

#endif /* UTIL_H */