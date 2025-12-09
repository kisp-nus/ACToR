#include "util.h"
#include <errno.h>
#include <limits.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <ctype.h>
#include <signal.h>
#include <unistd.h>
#include <grp.h>
#include <pwd.h>

// === copied from compat/common/strtonum.c BEGIN (Line 20-65) ===
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
// === copied from compat/common/strtonum.c END (Line 20-65) ===

// === copied from compat/unkown_brother/strlcpy.c BEGIN (Line 25-55) ===
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
// === copied from compat/unkown_brother/strlcpy.c END (Line 25-55) ===

// === copied from compat/linux/strlcat.c BEGIN (Line 25-60) ===
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
// === copied from compat/linux/strlcat.c END (Line 25-60) ===

// === copied from compat/linux/setmode.c BEGIN ===
#include <sys/types.h>
#include <sys/stat.h>

#include <ctype.h>
#include <errno.h>
#include <signal.h>
#include <stdlib.h>
#include <unistd.h>

#ifdef SETMODE_DEBUG
#include <stdio.h>
#endif

#define	SET_LEN	6		/* initial # of bitcmd struct to malloc */
#define	SET_LEN_INCR 4		/* # of bitcmd structs to add as needed */

typedef struct bitcmd {
	char	cmd;
	char	cmd2;
	mode_t	bits;
} BITCMD;

#define	CMD2_CLR	0x01
#define	CMD2_SET	0x02
#define	CMD2_GBITS	0x04
#define	CMD2_OBITS	0x08
#define	CMD2_UBITS	0x10

static BITCMD	*addcmd(BITCMD *, int, int, int, u_int);
static void	 compress_mode(BITCMD *);
#ifdef SETMODE_DEBUG
static void	 dumpmode(BITCMD *);
#endif

/*
 * Given the old mode and an array of bitcmd structures, apply the operations
 * described in the bitcmd structures to the old mode, and return the new mode.
 * Note that there is no '=' command; a strict assignment is just a '-' (clear
 * bits) followed by a '+' (set bits).
 */
mode_t
getmode(const void *bbox, mode_t omode)
{
	const BITCMD *set;
	mode_t clrval, newmode, value;

	set = (const BITCMD *)bbox;
	newmode = omode;
	for (value = 0;; set++)
		switch(set->cmd) {
		/*
		 * When copying the user, group or other bits around, we "know"
		 * where the bits are in the mode so that we can do shifts to
		 * copy them around.  If we don't use shifts, it gets real
		 * grundgy with lots of single bit checks and bit sets.
		 */
		case 'u':
			value = (newmode & S_IRWXU) >> 6;
			goto common;

		case 'g':
			value = (newmode & S_IRWXG) >> 3;
			goto common;

		case 'o':
			value = newmode & S_IRWXO;
common:			if (set->cmd2 & CMD2_CLR) {
				clrval =
				    (set->cmd2 & CMD2_SET) ?  S_IRWXO : value;
				if (set->cmd2 & CMD2_UBITS)
					newmode &= ~((clrval<<6) & set->bits);
				if (set->cmd2 & CMD2_GBITS)
					newmode &= ~((clrval<<3) & set->bits);
				if (set->cmd2 & CMD2_OBITS)
					newmode &= ~(clrval & set->bits);
			}
			if (set->cmd2 & CMD2_SET) {
				if (set->cmd2 & CMD2_UBITS)
					newmode |= (value<<6) & set->bits;
				if (set->cmd2 & CMD2_GBITS)
					newmode |= (value<<3) & set->bits;
				if (set->cmd2 & CMD2_OBITS)
					newmode |= value & set->bits;
			}
			break;

		case '+':
			newmode |= set->bits;
			break;

		case '-':
			newmode &= ~set->bits;
			break;

		case 'X':
			if (omode & (S_IFDIR|S_IXUSR|S_IXGRP|S_IXOTH))
				newmode |= set->bits;
			break;

		case '\0':
		default:
#ifdef SETMODE_DEBUG
			(void)printf("getmode:%04o -> %04o\n", omode, newmode);
#endif
			return (newmode);
		}
}

#define	ADDCMD(a, b, c, d)						\
	if (set >= endset) {						\
		BITCMD *newset;						\
		setlen += SET_LEN_INCR;					\
		newset = reallocarray(saveset, setlen, sizeof(BITCMD));	\
		if (newset == NULL) {					\
			free(saveset);					\
			return (NULL);					\
		}							\
		set = newset + (set - saveset);				\
		saveset = newset;					\
		endset = newset + (setlen - 2);				\
	}								\
	set = addcmd(set, (a), (b), (c), (d))

#define	STANDARD_BITS	(S_ISUID|S_ISGID|S_IRWXU|S_IRWXG|S_IRWXO)

void *
setmode(const char *p)
{
	char op, *ep;
	BITCMD *set, *saveset, *endset;
	sigset_t sigset, sigoset;
	mode_t mask, perm, permXbits, who;
	int equalopdone, setlen;
	u_long perml;

	if (!*p) {
		errno = EINVAL;
		return (NULL);
	}

	/*
	 * Get a copy of the mask for the permissions that are mask relative.
	 * Flip the bits, we want what's not set.  Since it's possible that
	 * the caller is opening files inside a signal handler, protect them
	 * as best we can.
	 */
	sigfillset(&sigset);
	(void)sigprocmask(SIG_BLOCK, &sigset, &sigoset);
	(void)umask(mask = umask(0));
	mask = ~mask;
	(void)sigprocmask(SIG_SETMASK, &sigoset, NULL);

	setlen = SET_LEN + 2;
	
	if ((set = calloc((u_int)sizeof(BITCMD), setlen)) == NULL)
		return (NULL);
	saveset = set;
	endset = set + (setlen - 2);

	/*
	 * If an absolute number, get it and return; disallow non-octal digits
	 * or illegal bits.
	 */
	if (isdigit((unsigned char)*p)) {
		perml = strtoul(p, &ep, 8);
		/* The test on perml will also catch overflow. */
		if (*ep != '\0' || (perml & ~(STANDARD_BITS|S_ISVTX))) {
			free(saveset);
			errno = ERANGE;
			return (NULL);
		}
		perm = (mode_t)perml;
		ADDCMD('=', (STANDARD_BITS|S_ISVTX), perm, mask);
		set->cmd = 0;
		return (saveset);
	}

	/*
	 * Build list of structures to set/clear/copy bits as described by
	 * each clause of the symbolic mode.
	 */
	for (;;) {
		/* First, find out which bits might be modified. */
		for (who = 0;; ++p) {
			switch (*p) {
			case 'a':
				who |= STANDARD_BITS;
				break;
			case 'u':
				who |= S_ISUID|S_IRWXU;
				break;
			case 'g':
				who |= S_ISGID|S_IRWXG;
				break;
			case 'o':
				who |= S_IRWXO;
				break;
			default:
				goto getop;
			}
		}

getop:		if ((op = *p++) != '+' && op != '-' && op != '=') {
			free(saveset);
			errno = EINVAL;
			return (NULL);
		}
		if (op == '=')
			equalopdone = 0;

		who &= ~S_ISVTX;
		for (perm = 0, permXbits = 0;; ++p) {
			switch (*p) {
			case 'r':
				perm |= S_IRUSR|S_IRGRP|S_IROTH;
				break;
			case 's':
				/*
				 * If specific bits where requested and
				 * only "other" bits ignore set-id.
				 */
				if (who == 0 || (who & ~S_IRWXO))
					perm |= S_ISUID|S_ISGID;
				break;
			case 't':
				/*
				 * If specific bits where requested and
				 * only "other" bits ignore sticky.
				 */
				if (who == 0 || (who & ~S_IRWXO)) {
					who |= S_ISVTX;
					perm |= S_ISVTX;
				}
				break;
			case 'w':
				perm |= S_IWUSR|S_IWGRP|S_IWOTH;
				break;
			case 'X':
				permXbits = S_IXUSR|S_IXGRP|S_IXOTH;
				break;
			case 'x':
				perm |= S_IXUSR|S_IXGRP|S_IXOTH;
				break;
			case 'u':
			case 'g':
			case 'o':
				/*
				 * When ever we hit 'u', 'g', or 'o', we have
				 * to flush out any partial mode that we have,
				 * and then do the copying of the mode bits.
				 */
				if (perm) {
					ADDCMD(op, who, perm, mask);
					perm = 0;
				}
				if (op == '=')
					equalopdone = 1;
				if (op == '+' && permXbits) {
					ADDCMD('X', who, permXbits, mask);
					permXbits = 0;
				}
				ADDCMD(*p, who, op, mask);
				break;

			default:
				/*
				 * Add any permissions that we haven't already
				 * done.
				 */
				if (perm || (op == '=' && !equalopdone)) {
					if (op == '=')
						equalopdone = 1;
					ADDCMD(op, who, perm, mask);
					perm = 0;
				}
				if (permXbits) {
					ADDCMD('X', who, permXbits, mask);
					permXbits = 0;
				}
				goto apply;
			}
		}

apply:		if (!*p)
			break;
		if (*p != ',')
			goto getop;
		++p;
	}
	set->cmd = 0;
#ifdef SETMODE_DEBUG
	(void)printf("Before compress_mode()\n");
	dumpmode(saveset);
#endif
	compress_mode(saveset);
#ifdef SETMODE_DEBUG
	(void)printf("After compress_mode()\n");
	dumpmode(saveset);
#endif
	return (saveset);
}

static BITCMD *
addcmd(BITCMD *set, int op, int who, int oparg, u_int mask)
{
	switch (op) {
	case '=':
		set->cmd = '-';
		set->bits = who ? who : STANDARD_BITS;
		set++;

		op = '+';
		/* FALLTHROUGH */
	case '+':
	case '-':
	case 'X':
		set->cmd = op;
		set->bits = (who ? who : mask) & oparg;
		break;

	case 'u':
	case 'g':
	case 'o':
		set->cmd = op;
		if (who) {
			set->cmd2 = ((who & S_IRUSR) ? CMD2_UBITS : 0) |
				    ((who & S_IRGRP) ? CMD2_GBITS : 0) |
				    ((who & S_IROTH) ? CMD2_OBITS : 0);
			set->bits = (mode_t)~0;
		} else {
			set->cmd2 = CMD2_UBITS | CMD2_GBITS | CMD2_OBITS;
			set->bits = mask;
		}
	
		if (oparg == '+')
			set->cmd2 |= CMD2_SET;
		else if (oparg == '-')
			set->cmd2 |= CMD2_CLR;
		else if (oparg == '=')
			set->cmd2 |= CMD2_SET|CMD2_CLR;
		break;
	}
	return (set + 1);
}

#ifdef SETMODE_DEBUG
static void
dumpmode(BITCMD *set)
{
	for (; set->cmd; ++set)
		(void)printf("cmd: '%c' bits %04o%s%s%s%s%s%s\n",
		    set->cmd, set->bits, set->cmd2 ? " cmd2:" : "",
		    set->cmd2 & CMD2_CLR ? " CLR" : "",
		    set->cmd2 & CMD2_SET ? " SET" : "",
		    set->cmd2 & CMD2_UBITS ? " UBITS" : "",
		    set->cmd2 & CMD2_GBITS ? " GBITS" : "",
		    set->cmd2 & CMD2_OBITS ? " OBITS" : "");
}
#endif

/*
 * Given an array of bitcmd structures, compress by compacting consecutive
 * '+', '-' and 'X' commands into at most 3 commands, one of each.  The 'u',
 * 'g' and 'o' commands continue to be separate.  They could probably be
 * compacted, but it's not worth the effort.
 */
static void
compress_mode(BITCMD *set)
{
	BITCMD *nset;
	int setbits, clrbits, Xbits, op;

	for (nset = set;;) {
		/* Copy over any 'u', 'g' and 'o' commands. */
		while ((op = nset->cmd) != '+' && op != '-' && op != 'X') {
			*set++ = *nset++;
			if (!op)
				return;
		}

		for (setbits = clrbits = Xbits = 0;; nset++) {
			if ((op = nset->cmd) == '-') {
				clrbits |= nset->bits;
				setbits &= ~nset->bits;
				Xbits &= ~nset->bits;
			} else if (op == '+') {
				setbits |= nset->bits;
				clrbits &= ~nset->bits;
				Xbits &= ~nset->bits;
			} else if (op == 'X')
				Xbits |= nset->bits & ~setbits;
			else
				break;
		}
		if (clrbits) {
			set->cmd = '-';
			set->cmd2 = 0;
			set->bits = clrbits;
			set++;
		}
		if (setbits) {
			set->cmd = '+';
			set->cmd2 = 0;
			set->bits = setbits;
			set++;
		}
		if (Xbits) {
			set->cmd = 'X';
			set->cmd2 = 0;
			set->bits = Xbits;
			set++;
		}
	}
}
// === copied from compat/linux/setmode.c END ===

// === copied from compat/common/pwcache.c BEGIN ===
#include <sys/types.h>

#include <assert.h>
#ifndef __MINGW32__
#include <grp.h>
#include <pwd.h>
#endif
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

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
#undef INVALID
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
 * uid_from_user()
 *	caches the uid for a given user name. We use a simple hash table.
 * Return:
 *	0 if the user name is found (filling in uid), -1 otherwise
 */
int
uid_from_user(const char *name, uid_t *uid)
{
	struct passwd pwstore, *pw = NULL;
	char pwbuf[1024];
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
	char grbuf[1024];
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
// === copied from compat/common/pwcache.c END ===