#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>

// === copied from compat/include/compat.h BEGIN (Line 55-56) ===
extern const char *__progname;
// === copied from compat/include/compat.h END (Line 55-56) ===

// === copied from compat/include/compat.h BEGIN (Line 68-69) ===
void setprogname(const char *progname);
const char *getprogname(void);
// === copied from compat/include/compat.h END (Line 68-69) ===

// === copied from compat/include/compat.h BEGIN (Line 161) ===
void strmode (int, char *);
// === copied from compat/include/compat.h END (Line 161) ===

// === copied from compat/include/compat.h BEGIN (Line 168-173) ===
const char *user_from_uid (uid_t, int);
const char *group_from_gid (gid_t, int);
int uid_from_user (const char *, uid_t *);
int gid_from_group (const char *, gid_t *);
// === copied from compat/include/compat.h END (Line 168-173) ===

// === copied from compat/include/compat.h BEGIN (Line 213) ===
size_t strlcpy (char *, const char *, size_t);
// === copied from compat/include/compat.h END (Line 213) ===

// === copied from compat/include/compat.h BEGIN (Line 229-230) ===
#define MAXBSIZE (64 * 1024)
// === copied from compat/include/compat.h END (Line 229-230) ===

// === copied from compat/include/compat.h BEGIN (Line 254-255) ===
#define S_ISTXT S_ISVTX
// === copied from compat/include/compat.h END (Line 254-255) ===

#endif /* UTIL_H */