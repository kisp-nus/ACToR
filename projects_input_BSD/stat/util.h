#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>

// === copied from compat/common/progname.c BEGIN (Line 12, 15-16, 103-104) ===
extern const char *__progname;
const char* getprogname(void);
void setprogname(const char *progname);
// === copied from compat/common/progname.c END (Line 12, 15-16, 103-104) ===

// === copied from compat/common/strmode.c BEGIN (Line 38-39) ===
void strmode(int mode, char *p);
// === copied from compat/common/strmode.c END (Line 38-39) ===

// === copied from compat/common/pwcache.c BEGIN (Line 212-213, 273-274, 333-334, 394-395) ===
const char *user_from_uid(uid_t uid, int noname);
const char *group_from_gid(gid_t gid, int noname);
int uid_from_user(const char *name, uid_t *uid);
int gid_from_group(const char *name, gid_t *gid);
// === copied from compat/common/pwcache.c END (Line 212-213, 273-274, 333-334, 394-395) ===

// === copied from compat/unix/devname.c BEGIN (Line 49-50) ===
char *devname(dev_t dev, mode_t type);
// === copied from compat/unix/devname.c END (Line 49-50) ===

// === copied from compat/unkown_brother/strlcpy.c BEGIN (Line 28-29) ===
size_t strlcpy(char *dst, const char *src, size_t dsize);
// === copied from compat/unkown_brother/strlcpy.c END (Line 28-29) ===

// === copied from compat/linux/strlcat.c BEGIN (Line 30-31) ===
size_t strlcat(char *dst, const char *src, size_t dsize);
// === copied from compat/linux/strlcat.c END (Line 30-31) ===

#endif