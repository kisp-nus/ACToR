#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>
#include <grp.h>
#include <pwd.h>

// === copied from compat/common/strtonum.c BEGIN ===
long long strtonum(const char *numstr, long long minval, long long maxval, const char **errstrp);
// === copied from compat/common/strtonum.c END ===

// === copied from compat/unkown_brother/strlcpy.c BEGIN ===
size_t strlcpy(char *dst, const char *src, size_t dsize);
// === copied from compat/linux/strlcat.c BEGIN ===
size_t strlcat(char *dst, const char *src, size_t dsize);
// === copied from compat/linux/strlcat.c END ===

// === copied from compat/linux/setmode.c BEGIN ===
void *setmode(const char *p);
mode_t getmode(const void *bbox, mode_t omode);
// === copied from compat/linux/setmode.c END ===

// === copied from compat/common/pwcache.c BEGIN ===
int uid_from_user(const char *name, uid_t *uid);
int gid_from_group(const char *name, gid_t *gid);
// === copied from compat/common/pwcache.c END ===

#endif /* UTIL_H */
