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

// === copied from compat/common/progname.c BEGIN (Line 5-116) ===
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
// === copied from compat/common/progname.c END (Line 5-116) ===