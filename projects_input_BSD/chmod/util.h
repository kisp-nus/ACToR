#include <sys/types.h>

// === copied from compat/include/compat.h BEGIN (Line 52-56) ===
#ifndef HAVE___PROGNAME
extern const char *__progname;
#else
extern const char *__progname;
#endif
// === copied from compat/include/compat.h END (Line 52-56) ===

// === copied from compat/common/progname.c BEGIN (Line 15-16, 103-104) ===
const char* getprogname(void);
void setprogname(const char *progname);
// === copied from compat/common/progname.c END (Line 15-16, 103-104) ===

// === copied from compat/include/compat.h BEGIN (Line 157-158, 171-172, 207-213) ===
long long strtonum (const char *, long long, long long, const char **);
int uid_from_user (const char *, uid_t *);
int gid_from_group (const char *, gid_t *);
size_t strlcpy (char *, const char *, size_t);
mode_t getmode (const void *, mode_t);
void *setmode (const char *);
// === copied from compat/include/compat.h END (Line 157-158, 171-172, 207-213) ===