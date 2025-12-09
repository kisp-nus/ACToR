#include "util.h"
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <stdint.h>

// === copied from compat/common/progname.c BEGIN (Line 12-116) ===
#ifndef HAVE___PROGNAME
const char *__progname = NULL;
#endif

#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/')

const char*
getprogname(void)
{
    return __progname;
}

void
setprogname(const char *progname)
{
    size_t i;

    for (i = strlen(progname); i > 0; i--) {
        if (LIBCOMPAT_IS_PATHNAME_SEPARATOR(progname[i - 1])) {
            __progname = progname + i;
            return;
        }
    }

    __progname = progname;
}
// === copied from compat/common/progname.c END (Line 12-116) ===

// === copied from compat/darwin/reallocarray.c BEGIN (Line 25-42) ===
/*
 * This is sqrt(SIZE_MAX+1), as s1*s2 <= SIZE_MAX
 * if both s1 < MUL_NO_OVERFLOW and s2 < MUL_NO_OVERFLOW
 */
#define MUL_NO_OVERFLOW ((size_t)1 << (sizeof (size_t) * 4))

void *
reallocarray (void *optr, size_t nmemb, size_t size)
{
  if ((nmemb >= MUL_NO_OVERFLOW || size >= MUL_NO_OVERFLOW) && nmemb > 0
      && SIZE_MAX / nmemb < size)
    {
      errno = ENOMEM;
      return NULL;
    }
  return realloc (optr, size * nmemb);
}
// === copied from compat/darwin/reallocarray.c END (Line 25-42) ===