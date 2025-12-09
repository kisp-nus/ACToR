#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>

/* === copied from compat/darwin/reallocarray.c BEGIN (Line 31-32) === */
void *reallocarray(void *optr, size_t nmemb, size_t size);
/* === copied from compat/darwin/reallocarray.c END (Line 31-32) === */

/* Linux-specific definitions for utmp access */
#define _PATH_UTMP "/var/run/utmp"

#endif /* UTIL_H */