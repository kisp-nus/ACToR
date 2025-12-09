#include "util.h"
#include <errno.h>
#include <limits.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

/* === copied from compat/common/strtonum.c BEGIN (Line 25-66) === */
#define	INVALID		1
#define	TOOSMALL	2
#define	TOOLARGE	3

long long
strtonum(const char *numstr, long long minval, long long maxval,
    const char **errstrp)
{
	long long ll = 0;
	int error = 0;
	char *ep;
	struct errval {
		const char *errstr;
		int err;
	} ev[4] = {
		{ NULL,		0 },
		{ "invalid",	EINVAL },
		{ "too small",	ERANGE },
		{ "too large",	ERANGE },
	};

	ev[0].err = errno;
	errno = 0;
	if (minval > maxval) {
		error = INVALID;
	} else {
		ll = strtoll(numstr, &ep, 10);
		if (numstr == ep || *ep != '\0')
			error = INVALID;
		else if ((ll == LLONG_MIN && errno == ERANGE) || ll < minval)
			error = TOOSMALL;
		else if ((ll == LLONG_MAX && errno == ERANGE) || ll > maxval)
			error = TOOLARGE;
	}
	if (errstrp != NULL)
		*errstrp = ev[error].errstr;
	errno = ev[error].err;
	if (error)
		ll = 0;

	return (ll);
}
/* === copied from compat/common/strtonum.c END (Line 25-66) === */

/* === copied from compat/common/progname.c BEGIN (Line 12, 15-101, 103-115) === */
const char *__progname = NULL;

const char*
getprogname(void)
{
    #if defined(__GLIBC__)
        if(__progname == NULL)
            __progname = program_invocation_short_name;
    #endif
    return __progname;
}

void
setprogname(const char *progname)
{
    size_t i;

    for (i = strlen(progname); i > 0; i--) {
        if (progname[i - 1] == '/') {
            __progname = progname + i;
            return;
        }
    }

    __progname = progname;
}
/* === copied from compat/common/progname.c END (Line 12, 15-101, 103-115) === */

/* === copied from compat/unkown_brother/strlcpy.c BEGIN (Line 28-54) === */
/*
 * Copy string src to buffer dst of size dsize.  At most dsize-1
 * chars will be copied.  Always NUL terminates (unless dsize == 0).
 * Returns strlen(src); if retval >= dsize, truncation occurred.
 */
size_t
strlcpy (char *dst, const char *src, size_t dsize)
{
  const char *osrc = src;
  size_t nleft = dsize;

  /* Copy as many bytes as will fit. */
  if (nleft != 0)
    {
      while (--nleft != 0)
        {
          if ((*dst++ = *src++) == '\0')
            break;
        }
    }

  /* Not enough room in dst, add NUL and traverse rest of src. */
  if (nleft == 0)
    {
      if (dsize != 0)
        *dst = '\0'; /* NUL-terminate dst */
      while (*src++)
        ;
    }

  return (src - osrc - 1); /* count does not include NUL */
}
/* === copied from compat/unkown_brother/strlcpy.c END (Line 28-54) === */