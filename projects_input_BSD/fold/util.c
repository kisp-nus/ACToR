#include "util.h"
#include <errno.h>
#include <limits.h>
#include <stdlib.h>
#include <stdint.h>

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