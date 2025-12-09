// === copied from compat/common/progname.c BEGIN (Line 1-116) ===
#include "util.h"
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/' || (c) == '\\')
#else
#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/')
#endif

#ifndef HAVE___PROGNAME
const char *__progname = NULL;
#endif

const char*
getprogname(void)
{
    #if defined(HAVE_PROGRAM_INVOCATION_SHORT_NAME)
        if(__progname == NULL)
            __progname = program_invocation_short_name;
    #elif defined(HAVE_GETEXECNAME) // Solaris
        if(__progname == NULL)
            setprogname(getexecname());
    #elif defined(IS_USING_WHEREAMI_LIBRARY) // Musl/Linux, GNU/Linux...
        if(__progname == NULL) {
            // This code logic leaves progname as NULL btw...
            char *path = NULL;
            int lenght, dirname_lenght;

            lenght = wai_getExecutablePath(NULL, 0, &dirname_lenght);
            if (lenght > 0) {
                path = (char*)malloc(lenght + 1);
                wai_getExecutablePath(path, lenght, &dirname_lenght);
                path[lenght] = '\0';
                setprogname(path); // Must be transformed into relative path
            }
        }
    #elif defined(__UTOPIA__)
        setprogname(c_proc_getexename());
    #elif defined(_WIN32) // I hate the Windows API, I hate the Windows API
        if (__progname == NULL) {
            WCHAR *wpath = NULL;
            WCHAR *wname = NULL;
            WCHAR *wext = NULL;
            DWORD wpathsiz = MAX_PATH / 2;
            DWORD len, i;
            char *mbname = NULL;
            int mbnamesiz;

            do {
                WCHAR *wpathnew;

                wpathsiz *= 2;
                wpathsiz = MIN(wpathsiz, UNICODE_STRING_MAX_CHARS);
                wpathnew = reallocarray(wpath, wpathsiz, sizeof(*wpath));
                if (wpathnew == NULL) {
                    goto done;
                }
                wpath = wpathnew;

                len = GetModuleFileNameW(NULL, wpath, wpathsiz);
                if (wpathsiz == UNICODE_STRING_MAX_CHARS)
                    goto done;
            } while (wpathsiz == len);

            if (len == 0)
                goto done;
            
            wname = wpath;
            for (i = len; i > 0; i--) {
                if (LIBCOMPAT_IS_PATHNAME_SEPARATOR(wpath[i - 1])) {
                    wname = wpath + i;
                    break;
                }
            }

            wext = PathFindExtensionW(wname);
            wext[0] = '\0';

            mbnamesiz = WideCharToMultiByte(CP_UTF8, 0, wname, -1, NULL, 0, NULL, NULL);

            if (mbnamesiz == 0)
                goto done;
            mbname = malloc(mbnamesiz);
            if (mbname == NULL)
                goto done;
            
            mbnamesiz = WideCharToMultiByte(CP_UTF8, 0, wname, -1, mbname, mbnamesiz, NULL, NULL);

            if (mbnamesiz == 0)
                goto done;
            __progname = mbname;
            mbname = NULL;
done:
            free(wpath);
            free(mbname);
        }
        #endif

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
// === copied from compat/common/progname.c END (Line 1-116) ===

// === copied from compat/linux/setmode.c BEGIN (Line 1-451) ===
#include <sys/types.h>
#include <sys/stat.h>

#include <ctype.h>
#include <errno.h>
#include <signal.h>
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
// === copied from compat/linux/setmode.c END (Line 1-451) ===