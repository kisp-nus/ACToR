#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>

// === copied from compat/include/compat.h BEGIN (Line 12-12) ===
extern const char *__progname;
// === copied from compat/include/compat.h END (Line 12-12) ===

// === copied from compat/common/progname.c BEGIN (Line 15-116) ===
const char* getprogname(void);
void setprogname(const char *progname);
// === copied from compat/common/progname.c END (Line 15-116) ===

// === copied from compat/common/strmode.c BEGIN (Line 38-141) ===
void strmode(int mode, char *p);
// === copied from compat/common/strmode.c END (Line 38-141) ===

// === copied from compat/common/pwcache.c BEGIN (Line 212-324) ===
const char *user_from_uid(uid_t uid, int noname);
const char *group_from_gid(gid_t gid, int noname);
int uid_from_user(const char *name, uid_t *uid);
int gid_from_group(const char *name, gid_t *gid);
// === copied from compat/common/pwcache.c END (Line 212-324) ===

// === copied from compat/unkown_brother/strlcpy.c BEGIN (Line 28-54) ===
size_t strlcpy(char *dst, const char *src, size_t dsize);
// === copied from compat/unkown_brother/strlcpy.c END (Line 28-54) ===

#endif