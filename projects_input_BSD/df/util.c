/*
 * util.c - Utility functions for df program
 * Contains BSD compatibility functions copied from compat/
 */

#include "util.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <ctype.h>
#include <limits.h>
#include <sys/types.h>

/* === copied from compat/include/compat.h BEGIN (Line 54-56) === */
const char *__progname = "df";
/* === copied from compat/include/compat.h END (Line 54-56) === */

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

/* === copied from compat/common/fmt_scaled.c BEGIN (Line 49-278) === */
typedef enum {
	NONE = 0, KILO = 1, MEGA = 2, GIGA = 3, TERA = 4, PETA = 5, EXA = 6
} unit_type;

/* These three arrays MUST be in sync!  XXX make a struct */
static unit_type units[] = { NONE, KILO, MEGA, GIGA, TERA, PETA, EXA };
static char scale_chars[] = "BKMGTPE";
static long long scale_factors[] = {
	1LL,
	1024LL,
	1024LL*1024,
	1024LL*1024*1024,
	1024LL*1024*1024*1024,
	1024LL*1024*1024*1024*1024,
	1024LL*1024*1024*1024*1024*1024,
};
#define	SCALE_LENGTH (sizeof(units)/sizeof(units[0]))

#define MAX_DIGITS (SCALE_LENGTH * 3)	/* XXX strlen(sprintf("%lld", -1)? */

/* Convert the given input string "scaled" into numeric in "result".
 * Return 0 on success, -1 and errno set on error.
 */
int
scan_scaled(char *scaled, long long *result)
{
	char *p = scaled;
	int sign = 0;
	unsigned int i, ndigits = 0, fract_digits = 0;
	long long scale_fact = 1, whole = 0, fpart = 0;

	/* Skip leading whitespace */
	while (isascii((unsigned char)*p) && isspace((unsigned char)*p))
		++p;

	/* Then at most one leading + or - */
	while (*p == '-' || *p == '+') {
		if (*p == '-') {
			if (sign) {
				errno = EINVAL;
				return -1;
			}
			sign = -1;
			++p;
		} else if (*p == '+') {
			if (sign) {
				errno = EINVAL;
				return -1;
			}
			sign = +1;
			++p;
		}
	}

	/* Main loop: Scan digits, find decimal point, if present.
	 * We don't allow exponentials, so no scientific notation
	 * (but note that E for Exa might look like e to some!).
	 * Advance 'p' to end, to get scale factor.
	 */
	for (; isascii((unsigned char)*p) &&
	    (isdigit((unsigned char)*p) || *p=='.'); ++p) {
		if (*p == '.') {
			if (fract_digits > 0) {	/* oops, more than one '.' */
				errno = EINVAL;
				return -1;
			}
			fract_digits = 1;
			continue;
		}

		i = (*p) - '0';			/* whew! finally a digit we can use */
		if (fract_digits > 0) {
			if (fract_digits >= MAX_DIGITS-1)
				/* ignore extra fractional digits */
				continue;
			fract_digits++;		/* for later scaling */
			if (fpart > LLONG_MAX / 10) {
				errno = ERANGE;
				return -1;
			}
			fpart *= 10;
			if (i > LLONG_MAX - fpart) {
				errno = ERANGE;
				return -1;
			}
			fpart += i;
		} else {				/* normal digit */
			if (++ndigits >= MAX_DIGITS) {
				errno = ERANGE;
				return -1;
			}
			if (whole > LLONG_MAX / 10) {
				errno = ERANGE;
				return -1;
			}
			whole *= 10;
			if (i > LLONG_MAX - whole) {
				errno = ERANGE;
				return -1;
			}
			whole += i;
		}
	}

	if (sign) {
		whole *= sign;
		fpart *= sign;
	}

	/* If no scale factor given, we're done. fraction is discarded. */
	if (!*p) {
		*result = whole;
		return 0;
	}

	/* Validate scale factor, and scale whole and fraction by it. */
	for (i = 0; i < SCALE_LENGTH; i++) {

		/* Are we there yet? */
		if (*p == scale_chars[i] ||
			*p == tolower((unsigned char)scale_chars[i])) {

			/* If it ends with alphanumerics after the scale char, bad. */
			if (isalnum((unsigned char)*(p+1))) {
				errno = EINVAL;
				return -1;
			}
			scale_fact = scale_factors[i];

			/* check for overflow and underflow after scaling */
			if (whole > LLONG_MAX / scale_fact ||
			    whole < LLONG_MIN / scale_fact) {
				errno = ERANGE;
				return -1;
			}

			/* scale whole part */
			whole *= scale_fact;

			/* truncate fpart so it does't overflow.
			 * then scale fractional part.
			 */
			while (fpart >= LLONG_MAX / scale_fact) {
				fpart /= 10;
				fract_digits--;
			}
			fpart *= scale_fact;
			if (fract_digits > 0) {
				for (i = 0; i < fract_digits -1; i++)
					fpart /= 10;
			}
			whole += fpart;
			*result = whole;
			return 0;
		}
	}

	/* Invalid unit or character */
	errno = EINVAL;
	return -1;
}

/* Format the given "number" into human-readable form in "result".
 * Result must point to an allocated buffer of length FMT_SCALED_STRSIZE.
 * Return 0 on success, -1 and errno set if error.
 */
int
fmt_scaled(long long number, char *result)
{
	long long abval, fract = 0;
	unsigned int i;
	unit_type unit = NONE;

	/* Not every negative long long has a positive representation. */
	if (number == LLONG_MIN) {
		errno = ERANGE;
		return -1;
	}

	abval = llabs(number);

	/* Also check for numbers that are just too darned big to format. */
	if (abval / 1024 >= scale_factors[SCALE_LENGTH-1]) {
		errno = ERANGE;
		return -1;
	}

	/* scale whole part; get unscaled fraction */
	for (i = 0; i < SCALE_LENGTH; i++) {
		if (abval/1024 < scale_factors[i]) {
			unit = units[i];
			fract = (i == 0) ? 0 : abval % scale_factors[i];
			number /= scale_factors[i];
			if (i > 0)
				fract /= scale_factors[i - 1];
			break;
		}
	}

	fract = (10 * fract + 512) / 1024;
	/* if the result would be >= 10, round main number */
	if (fract >= 10) {
		if (number >= 0)
			number++;
		else
			number--;
		fract = 0;
	} else if (fract < 0) {
		/* shouldn't happen */
		fract = 0;
	}

	if (number == 0)
		strlcpy(result, "0B", FMT_SCALED_STRSIZE);
	else if (unit == NONE || number >= 100 || number <= -100) {
		if (fract >= 5) {
			if (number >= 0)
				number++;
			else
				number--;
		}
		(void)snprintf(result, FMT_SCALED_STRSIZE, "%lld%c",
			number, scale_chars[unit]);
	} else
		(void)snprintf(result, FMT_SCALED_STRSIZE, "%lld.%1lld%c",
			number, fract, scale_chars[unit]);

	return 0;
}
/* === copied from compat/common/fmt_scaled.c END (Line 49-278) === */

/* === copied from compat/common/getbsize.c BEGIN (Line 36-102) === */
char *
getbsize(int *headerlenp, long *blocksizep)
{
	static char header[20];
	long n, max, mul, blocksize;
	char *ep, *p, *form;

#define	KB	(1024)
#define	MB	(1024 * 1024)
#define	GB	(1024 * 1024 * 1024)
#define	MAXB	GB		/* No tera, peta, nor exa. */
	form = "";
	if ((p = getenv("BLOCKSIZE")) != NULL && *p != '\0') {
		if ((n = strtol(p, &ep, 10)) < 0)
			goto underflow;
		if (n == 0)
			n = 1;
		if (*ep && ep[1])
			goto fmterr;
		switch (*ep) {
		case 'G': case 'g':
			form = "G";
			max = MAXB / GB;
			mul = GB;
			break;
		case 'K': case 'k':
			form = "K";
			max = MAXB / KB;
			mul = KB;
			break;
		case 'M': case 'm':
			form = "M";
			max = MAXB / MB;
			mul = MB;
			break;
		case '\0':
			max = MAXB;
			mul = 1;
			break;
		default:
fmterr:			fprintf(stderr, "%s: unknown blocksize", p);
			n = 512;
			max = MAXB;
			mul = 1;
			break;
		}
		if (n > max) {
			fprintf(stderr, "maximum blocksize is %dG", MAXB / GB);
			n = max;
		}
		if ((blocksize = n * mul) < 512) {
underflow:		fprintf(stderr, "%s: minimum blocksize is 512", p);
			form = "";
			blocksize = n = 512;
		}
	} else
		blocksize = n = 512;

	*headerlenp = snprintf(header, sizeof(header), "%ld%s-blocks", n, form);
	if (*headerlenp < 0)
		*headerlenp = 0;
	else if (*headerlenp >= sizeof(header))
		*headerlenp = sizeof(header) - 1;
	*blocksizep = blocksize;
	return (header);
}
/* === copied from compat/common/getbsize.c END (Line 36-102) === */