#include "util_compat.h"
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <stdio.h>
#include <ctype.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <pwd.h>
#include <grp.h>
#include <unistd.h>
#include <err.h>
#include <limits.h>
#include <assert.h>

/* === copied from compat/common/progname.c BEGIN (Line 12-13) === */
#ifndef HAVE___PROGNAME
const char *__progname = NULL;
#endif
/* === copied from compat/common/progname.c END (Line 12-13) === */

/* === copied from compat/common/progname.c BEGIN (Line 6-9) === */
#ifdef _WIN32
#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/' || (c) == '\\')
#else
#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/')
#endif
/* === copied from compat/common/progname.c END (Line 6-9) === */

/* === copied from compat/common/progname.c BEGIN (Line 15-101) === */
const char*
getprogname(void)
{
        return __progname;
}
/* === copied from compat/common/progname.c END (Line 15-101) === */

/* === copied from compat/common/progname.c BEGIN (Line 103-115) === */
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
/* === copied from compat/common/progname.c END (Line 103-115) === */

/* === copied from compat/common/strmode.c BEGIN (Line 38-141) === */
void
strmode(int mode, char *p)
{
	 /* print type */
	switch (mode & S_IFMT) {
	case S_IFDIR:			/* directory */
		*p++ = 'd';
		break;
	case S_IFCHR:			/* character special */
		*p++ = 'c';
		break;
	case S_IFBLK:			/* block special */
		*p++ = 'b';
		break;
	case S_IFREG:			/* regular */
		*p++ = '-';
		break;
	case S_IFLNK:			/* symbolic link */
		*p++ = 'l';
		break;
	case S_IFSOCK:			/* socket */
		*p++ = 's';
		break;
#ifdef S_IFIFO
	case S_IFIFO:			/* fifo */
		*p++ = 'p';
		break;
#endif
	default:			/* unknown */
		*p++ = '?';
		break;
	}
	/* usr */
	if (mode & S_IRUSR)
		*p++ = 'r';
	else
		*p++ = '-';
	if (mode & S_IWUSR)
		*p++ = 'w';
	else
		*p++ = '-';
	switch (mode & (S_IXUSR | S_ISUID)) {
	case 0:
		*p++ = '-';
		break;
	case S_IXUSR:
		*p++ = 'x';
		break;
	case S_ISUID:
		*p++ = 'S';
		break;
	case S_IXUSR | S_ISUID:
		*p++ = 's';
		break;
	}
	/* group */
	if (mode & S_IRGRP)
		*p++ = 'r';
	else
		*p++ = '-';
	if (mode & S_IWGRP)
		*p++ = 'w';
	else
		*p++ = '-';
	switch (mode & (S_IXGRP | S_ISGID)) {
	case 0:
		*p++ = '-';
		break;
	case S_IXGRP:
		*p++ = 'x';
		break;
	case S_ISGID:
		*p++ = 'S';
		break;
	case S_IXGRP | S_ISGID:
		*p++ = 's';
		break;
	}
	/* other */
	if (mode & S_IROTH)
		*p++ = 'r';
	else
		*p++ = '-';
	if (mode & S_IWOTH)
		*p++ = 'w';
	else
		*p++ = '-';
	switch (mode & (S_IXOTH | S_ISVTX)) {
	case 0:
		*p++ = '-';
		break;
	case S_IXOTH:
		*p++ = 'x';
		break;
	case S_ISVTX:
		*p++ = 'T';
		break;
	case S_IXOTH | S_ISVTX:
		*p++ = 't';
		break;
	}
	*p++ = ' ';		/* will be a '+' if ACL's implemented */
	*p = '\0';
}
/* === copied from compat/common/strmode.c END (Line 38-141) === */

/* === copied from compat/common/pwcache.c BEGIN (Line 49-195, Line 204-448) === */
/*
 * Constants and data structures used to implement group and password file
 * caches.  Name lengths have been chosen to be as large as those supported
 * by the passwd and group files as well as the standard archive formats.
 * CACHE SIZES MUST BE PRIME
 */
#define UNMLEN		32	/* >= user name found in any protocol */
#define GNMLEN		32	/* >= group name found in any protocol */
#define UID_SZ		317	/* size of uid to user_name cache */
#define UNM_SZ		317	/* size of user_name to uid cache */
#define GID_SZ		251	/* size of gid to group_name cache */
#define GNM_SZ		251	/* size of group_name to gid cache */
#define VALID		1	/* entry and name are valid */
#define INVALID		2	/* entry valid, name NOT valid */

/*
 * Node structures used in the user, group, uid, and gid caches.
 */
 
typedef struct uidc {
	int valid;		/* is this a valid or a miss entry */
	char name[UNMLEN];	/* uid name */
	uid_t uid;		/* cached uid */
} UIDC;

typedef struct gidc {
	int valid;		/* is this a valid or a miss entry */
	char name[GNMLEN];	/* gid name */
	gid_t gid;		/* cached gid */
} GIDC;

/*
 * Routines that control user, group, uid and gid caches.
 * Traditional passwd/group cache routines perform quite poorly with
 * archives. The chances of hitting a valid lookup with an archive is quite a
 * bit worse than with files already resident on the file system. These misses
 * create a MAJOR performance cost. To adress this problem, these routines
 * cache both hits and misses.
 */

static UIDC **uidtb;	/* uid to name cache */
static GIDC **gidtb;	/* gid to name cache */
static UIDC **usrtb;	/* user name to uid cache */
static GIDC **grptb;	/* group name to gid cache */

static u_int
st_hash(const char *name, size_t len, int tabsz)
{
	u_int key = 0;

	assert(name != NULL);

	while (len--) {
		key += *name++;
		key = (key << 8) | (key >> 24);
	}

	return key % tabsz;
}

/*
 * uidtb_start
 *	creates an an empty uidtb
 * Return:
 *	0 if ok, -1 otherwise
 */
static int
uidtb_start(void)
{
	static int fail = 0;

	if (uidtb != NULL)
		return 0;
	if (fail)
		return -1;
	if ((uidtb = calloc(UID_SZ, sizeof(UIDC *))) == NULL) {
		++fail;
		return -1;
	}
	return 0;
}

/*
 * gidtb_start
 *	creates an an empty gidtb
 * Return:
 *	0 if ok, -1 otherwise
 */
static int
gidtb_start(void)
{
	static int fail = 0;

	if (gidtb != NULL)
		return 0;
	if (fail)
		return -1;
	if ((gidtb = calloc(GID_SZ, sizeof(GIDC *))) == NULL) {
		++fail;
		return -1;
	}
	return 0;
}

/*
 * usrtb_start
 *	creates an an empty usrtb
 * Return:
 *	0 if ok, -1 otherwise
 */
static int
usrtb_start(void)
{
	static int fail = 0;

	if (usrtb != NULL)
		return 0;
	if (fail)
		return -1;
	if ((usrtb = calloc(UNM_SZ, sizeof(UIDC *))) == NULL) {
		++fail;
		return -1;
	}
	return 0;
}

/*
 * grptb_start
 *	creates an an empty grptb
 * Return:
 *	0 if ok, -1 otherwise
 */
static int
grptb_start(void)
{
	static int fail = 0;

	if (grptb != NULL)
		return 0;
	if (fail)
		return -1;
	if ((grptb = calloc(GNM_SZ, sizeof(GIDC *))) == NULL) {
		++fail;
		return -1;
	}
	return 0;
}

/*
 * user_from_uid()
 *	caches the name (if any) for the uid. If noname clear, we always
 *	return the stored name (if valid or invalid match).
 *	We use a simple hash table.
 * Return:
 *	Pointer to stored name (or a empty string)
 */
const char *
user_from_uid(uid_t uid, int noname)
{
	struct passwd pwstore, *pw = NULL;
	char pwbuf[_PW_BUF_LEN];
	UIDC **pptr, *ptr = NULL;

	if ((uidtb != NULL) || (uidtb_start() == 0)) {
		/*
		* see if we have this uid cached
		*/
		pptr = uidtb + (uid % UID_SZ);
		ptr = *pptr;

		if ((ptr != NULL) && (ptr->valid > 0) && (ptr->uid == uid)) {
			/*
			* have an entry for this uid
			*/
			if (!noname || (ptr->valid == VALID))
				return ptr->name;
			return NULL;
		}

		if (ptr == NULL)
			*pptr = ptr = malloc(sizeof(UIDC));
	}

	getpwuid_r(uid, &pwstore, pwbuf, sizeof(pwbuf), &pw);
	if (pw == NULL) {
		/*
		* no match for this uid in the local password file
		* a string that is the uid in numeric format
		*/
		if (ptr == NULL)
			return NULL;
		ptr->uid = uid;
		(void)snprintf(ptr->name, UNMLEN, "%u", uid);
		ptr->valid = INVALID;
		if (noname)
			return NULL;
	} else {
		/*
		* there is an entry for this uid in the password file
		*/
		if (ptr == NULL)
			return pw->pw_name;
		ptr->uid = uid;
		(void)strlcpy(ptr->name, pw->pw_name, sizeof(ptr->name));
		ptr->valid = VALID;
	}
	return ptr->name;
}

/*
* group_from_gid()
*	caches the name (if any) for the gid. If noname clear, we always
*	return the stored name (if valid or invalid match).
*	We use a simple hash table.
* Return:
*	Pointer to stored name (or a empty string)
*/
const char *
group_from_gid(gid_t gid, int noname)
{
	struct group grstore, *gr = NULL;
	char grbuf[_GR_BUF_LEN];
	GIDC **pptr, *ptr = NULL;

	if ((gidtb != NULL) || (gidtb_start() == 0)) {
		/*
		* see if we have this gid cached
		*/
		pptr = gidtb + (gid % GID_SZ);
		ptr = *pptr;

		if ((ptr != NULL) && (ptr->valid > 0) && (ptr->gid == gid)) {
			/*
			* have an entry for this gid
			*/
			if (!noname || (ptr->valid == VALID))
				return ptr->name;
			return NULL;
		}

		if (ptr == NULL)
			*pptr = ptr = malloc(sizeof(GIDC));
	}

	getgrgid_r(gid, &grstore, grbuf, sizeof(grbuf), &gr);
	if (gr == NULL) {
		/*
		* no match for this gid in the local group file, put in
		* a string that is the gid in numeric format
		*/
		if (ptr == NULL)
			return NULL;
		ptr->gid = gid;
		(void)snprintf(ptr->name, GNMLEN, "%u", gid);
		ptr->valid = INVALID;
		if (noname)
			return NULL;
	} else {
		/*
		* there is an entry for this group in the group file
		*/
		if (ptr == NULL)
			return gr->gr_name;
		ptr->gid = gid;
		(void)strlcpy(ptr->name, gr->gr_name, sizeof(ptr->name));
		ptr->valid = VALID;
	}
	return ptr->name;
}
 
/*
* uid_from_user()
*	caches the uid for a given user name. We use a simple hash table.
* Return:
*	0 if the user name is found (filling in uid), -1 otherwise
*/
int
uid_from_user(const char *name, uid_t *uid)
{
	struct passwd pwstore, *pw = NULL;
	char pwbuf[_PW_BUF_LEN];
	UIDC **pptr, *ptr = NULL;
	size_t namelen;

	/*
	* return -1 for mangled names
	*/
	if (name == NULL || ((namelen = strlen(name)) == 0))
		return -1;

	if ((usrtb != NULL) || (usrtb_start() == 0)) {
		/*
		* look up in hash table, if found and valid return the uid,
		* if found and invalid, return a -1
		*/
		pptr = usrtb + st_hash(name, namelen, UNM_SZ);
		ptr = *pptr;

		if ((ptr != NULL) && (ptr->valid > 0) &&
			strcmp(name, ptr->name) == 0) {
			if (ptr->valid == INVALID)
				return -1;
			*uid = ptr->uid;
			return 0;
		}

		if (ptr == NULL)
			*pptr = ptr = malloc(sizeof(UIDC));
	}

	/*
	* no match, look it up, if no match store it as an invalid entry,
	* or store the matching uid
	*/
	getpwnam_r(name, &pwstore, pwbuf, sizeof(pwbuf), &pw);
	if (ptr == NULL) {
		if (pw == NULL)
			return -1;
		*uid = pw->pw_uid;
		return 0;
	}
	(void)strlcpy(ptr->name, name, sizeof(ptr->name));
	if (pw == NULL) {
		ptr->valid = INVALID;
		return -1;
	}
	ptr->valid = VALID;
	*uid = ptr->uid = pw->pw_uid;
	return 0;
}

/*
* gid_from_group()
*	caches the gid for a given group name. We use a simple hash table.
* Return:
*	0 if the group name is found (filling in gid), -1 otherwise
*/
int
gid_from_group(const char *name, gid_t *gid)
{
	struct group grstore, *gr = NULL;
	char grbuf[_GR_BUF_LEN];
	GIDC **pptr, *ptr = NULL;
	size_t namelen;

	/*
	* return -1 for mangled names
	*/
	if (name == NULL || ((namelen = strlen(name)) == 0))
		return -1;

	if ((grptb != NULL) || (grptb_start() == 0)) {
		/*
		* look up in hash table, if found and valid return the uid,
		* if found and invalid, return a -1
		*/
		pptr = grptb + st_hash(name, namelen, GID_SZ);
		ptr = *pptr;

		if ((ptr != NULL) && (ptr->valid > 0) &&
			strcmp(name, ptr->name) == 0) {
			if (ptr->valid == INVALID)
				return -1;
			*gid = ptr->gid;
			return 0;
		}

		if (ptr == NULL)
			*pptr = ptr = malloc(sizeof(GIDC));
	}

	/*
	* no match, look it up, if no match store it as an invalid entry,
	* or store the matching gid
	*/
	getgrnam_r(name, &grstore, grbuf, sizeof(grbuf), &gr);
	if (ptr == NULL) {
		if (gr == NULL)
			return -1;
		*gid = gr->gr_gid;
		return 0;
	}

	(void)strlcpy(ptr->name, name, sizeof(ptr->name));
	if (gr == NULL) {
		ptr->valid = INVALID;
		return -1;
	}
	ptr->valid = VALID;
	*gid = ptr->gid = gr->gr_gid;
	return 0;
}
/* === copied from compat/common/pwcache.c END (Line 204-448) === */

/* === copied from compat/common/fmt_scaled.c BEGIN (Line 49-277) === */
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
/* === copied from compat/common/fmt_scaled.c END (Line 49-277) === */

/* === copied from compat/common/getbsize.c BEGIN (Line 36-89) === */
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
		if (*ep == '\0') {
			blocksize = n;
			goto done;
		}
		if (n > MAXB / 1024)
			goto overflow;
		mul = 1;
		switch (*ep++) {
		case 'G':
		case 'g':
			mul *= 1024;
			/* FALLTHROUGH */
		case 'M':
		case 'm':
			mul *= 1024;
			/* FALLTHROUGH */
		case 'K':
		case 'k':
			mul *= 1024;
			if (*ep != '\0')
				goto fmterr;
			n *= mul;
			if (n > MAXB)
				goto overflow;
			blocksize = n;
			goto done;
		default:
fmterr:			warnx("unknown blocksize format \"%s\"", p);
			break;
		}
	}
	blocksize = 512;
done:
	*blocksizep = blocksize;
	max = MAXB / blocksize;
	if (max < 999950) {
		*headerlenp = snprintf(header, sizeof(header),
		    "%s%ld%s", form, blocksize, form[0] != '\0' ? ")" : "");
	} else {
		*headerlenp = snprintf(header, sizeof(header), "%s%ld%s", 
		    form, blocksize / 1024, form[0] != '\0' ? "K)" : "K");
	}
	return header;

underflow:
	warnx("blocksize too small");
	blocksize = 512;
	goto done;
overflow:
	warnx("blocksize too large");
	blocksize = MAXB;
	goto done;
}
/* === copied from compat/common/getbsize.c END (Line 36-89) === */

/* === copied from compat/common/strtonum.c BEGIN (Line 25-66) === */
#undef INVALID
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