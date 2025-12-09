#include "util.h"
#include <sys/stat.h>
#include <sys/types.h>
#include <assert.h>
#include <grp.h>
#include <pwd.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/')

// === copied from compat/common/progname.c BEGIN (Line 12-12) ===
const char *__progname = NULL;
// === copied from compat/common/progname.c END (Line 12-12) ===

// === copied from compat/common/progname.c BEGIN (Line 15-101) ===
const char*
getprogname(void)
{
    return __progname;
}
// === copied from compat/common/progname.c END (Line 15-101) ===

// === copied from compat/common/progname.c BEGIN (Line 103-116) ===
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
// === copied from compat/common/progname.c END (Line 103-116) ===

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

// === copied from compat/unkown_brother/strlcpy.c BEGIN (Line 28-54) ===
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

/*
 * pwcache.c functions - constants and data structures
 */
#define UNMLEN		32	/* >= user name found in any protocol */
#define GNMLEN		32	/* >= group name found in any protocol */
#define UID_SZ		317	/* size of uid to user_name cache */
#define UNM_SZ		317	/* size of user_name to uid cache */
#define GID_SZ		251	/* size of gid to group_name cache */
#define GNM_SZ		251	/* size of group_name to gid cache */
#define VALID		1	/* entry and name are valid */
#define INVALID		2	/* entry valid, name NOT valid */

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

// === copied from compat/common/pwcache.c BEGIN (Line 212-324) ===
const char *
user_from_uid(uid_t uid, int noname)
{
	struct passwd pwstore, *pw = NULL;
	char pwbuf[1024];  /* _PW_BUF_LEN equivalent */
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

const char *
group_from_gid(gid_t gid, int noname)
{
	struct group grstore, *gr = NULL;
	char grbuf[1024];  /* _GR_BUF_LEN equivalent */
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
// === copied from compat/common/pwcache.c END (Line 212-324) ===

// === copied from compat/common/pwcache.c BEGIN (Line 333-386) ===
int
uid_from_user(const char *name, uid_t *uid)
{
	struct passwd pwstore, *pw = NULL;
	char pwbuf[1024];  /* _PW_BUF_LEN equivalent */
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
// === copied from compat/common/pwcache.c END (Line 333-386) ===

// === copied from compat/common/pwcache.c BEGIN (Line 394-448) ===
int
gid_from_group(const char *name, gid_t *gid)
{
	struct group grstore, *gr = NULL;
	char grbuf[1024];  /* _GR_BUF_LEN equivalent */
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
// === copied from compat/common/pwcache.c END (Line 394-448) ===