#include "util.h"
#include <sys/types.h>
#include <sys/stat.h>
#include <dirent.h>
#include <fcntl.h>
#include <limits.h>
#include <paths.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <grp.h>
#include <pwd.h>
#include <assert.h>

// Linux buffer size definitions for password and group functions
#ifndef _PW_BUF_LEN
#define _PW_BUF_LEN 1024
#endif
#ifndef _GR_BUF_LEN
#define _GR_BUF_LEN 1024
#endif

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

// === copied from compat/common/strmode.c BEGIN (Line 38-141) ===
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
// === copied from compat/common/strmode.c END (Line 38-141) ===

// === copied from compat/unix/devname.c BEGIN (Line 49-72) ===
char *
devname(dev_t dev, mode_t type)
{
	static char buf[NAME_MAX + 1];
	char *name = NULL;
	struct dirent *dp;
	struct stat sb;
	DIR *dirp;

	if ((dirp = opendir(_PATH_DEV)) == NULL)
		return (NULL);
	while ((dp = readdir(dirp)) != NULL) {
		if (dp->d_type != DT_UNKNOWN && DTTOIF(dp->d_type) != type)
			continue;
		if (fstatat(dirfd(dirp), dp->d_name, &sb, AT_SYMLINK_NOFOLLOW)
		    || sb.st_rdev != dev || (sb.st_mode & S_IFMT) != type)
			continue;
		strlcpy(buf, dp->d_name, sizeof(buf));
		name = buf;
		break;
	}
	closedir(dirp);
	return (name);
}
// === copied from compat/unix/devname.c END (Line 49-72) ===

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

// === copied from compat/common/pwcache.c BEGIN (Line 55-449) ===
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
// === copied from compat/common/pwcache.c END (Line 55-449) ===