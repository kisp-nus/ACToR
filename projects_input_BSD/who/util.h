#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>
#include <stdint.h>

// === copied from compat/include/compat.h BEGIN (Line 55) ===
extern const char *__progname;
// === copied from compat/include/compat.h END (Line 55) ===

// === copied from compat/include/compat.h BEGIN (Line 61) ===
#define __UNCONST(a)	((void *)(unsigned long)(const void *)(a))
// === copied from compat/include/compat.h END (Line 61) ===

// === copied from compat/include/compat.h BEGIN (Line 264-273) ===
#ifndef timespeccmp
#define	timespeccmp(tsp, usp, cmp)					\
	(((tsp)->tv_sec == (usp)->tv_sec) ?				\
	    ((tsp)->tv_nsec cmp (usp)->tv_nsec) :			\
	    ((tsp)->tv_sec cmp (usp)->tv_sec))
#endif

#ifndef timespecclear
#define	timespecclear(tsp)		(tsp)->tv_sec = (tsp)->tv_nsec = 0
#endif
// === copied from compat/include/compat.h END (Line 264-273) ===

// Linux path definitions for utmp
#ifndef _PATH_UTMP
#define _PATH_UTMP "/var/run/utmp"
#endif

#ifndef _PATH_UTMPX
#define _PATH_UTMPX "/var/run/utmp"
#endif

// Linux lacks these paths by default, provide fallback
#ifdef __linux__
#ifndef _PATH_DEV
#define _PATH_DEV "/dev/"
#endif
#endif

#endif /* UTIL_H */