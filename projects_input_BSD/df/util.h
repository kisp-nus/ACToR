/*
 * util.h - Utility functions for df program
 * Contains BSD compatibility functions copied from compat/
 */

#ifndef UTIL_H
#define UTIL_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* === copied from compat/include/compat.h BEGIN (Line 54-56) === */
extern const char *__progname;
/* === copied from compat/include/compat.h END (Line 54-56) === */

/* === copied from compat/include/compat.h BEGIN (Line 175-180) === */
/* fmt_scaled.c */
int scan_scaled (char *, long long *);
int fmt_scaled (long long, char *);

/* getbsize.c */
char *getbsize (int *, long *);
/* === copied from compat/include/compat.h END (Line 175-180) === */

/* === copied from compat/include/compat.h BEGIN (Line 237-238) === */
/*
 * fmt_scaled(3) specific flags.
 * This comes from lib/libutil/util.h in the OpenBSD source.
 */
#define FMT_SCALED_STRSIZE 7 /* minus sign, 4 digits, suffix, null byte */
/* === copied from compat/include/compat.h END (Line 237-238) === */

/* === copied from compat/include/compat.h BEGIN (Line 207-214) === */
/* strlcpy.c */
#if defined __linux__|| defined __MINGW32__
size_t strlcpy (char *, const char *, size_t);
#endif
/* === copied from compat/include/compat.h END (Line 207-214) === */

#endif /* UTIL_H */