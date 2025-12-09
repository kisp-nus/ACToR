#include "util.h"
#include <errno.h>
#include <limits.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

// === copied from compat/common/strtonum.c BEGIN (Line 25-66) ===
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
// === copied from compat/common/strtonum.c END (Line 25-66) ===

// === copied from compat/unkown_brother/strlcpy.c BEGIN (Line 28-54) ===
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
// === copied from compat/unkown_brother/strlcpy.c END (Line 28-54) ===

// === copied from compat/linux/strlcat.c BEGIN (Line 30-58) ===
/*
 * Appends src to string dst of size dsize (unlike strncat, dsize is the
 * full size of dst, not space left).  At most dsize-1 characters
 * will be copied.  Always NUL terminates (unless dsize <= strlen(dst)).
 * Returns strlen(src) + MIN(dsize, strlen(initial dst)).
 * If retval >= dsize, truncation occurred.
 */
size_t
strlcat (char *dst, const char *src, size_t dsize)
{
  const char *odst = dst;
  const char *osrc = src;
  size_t n = dsize;
  size_t dlen;

  /* Find the end of dst and adjust bytes left but don't go past end. */
  while (n-- != 0 && *dst != '\0')
    dst++;
  dlen = dst - odst;
  n = dsize - dlen;

  if (n-- == 0)
    return (dlen + strlen (src));
  while (*src != '\0')
    {
      if (n != 0)
        {
          *dst++ = *src;
          n--;
        }
      src++;
    }
  *dst = '\0';

  return (dlen + (src - osrc)); /* count does not include NUL */
}
// === copied from compat/linux/strlcat.c END (Line 30-58) ===