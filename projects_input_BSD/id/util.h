#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>

// === copied from compat/common/progname.c BEGIN (Line 1-116) ===
extern const char *__progname;
const char *getprogname(void);
void setprogname(const char *progname);
// === copied from compat/common/progname.c END (Line 1-116) ===

// === copied from compat/common/strtonum.c BEGIN (Line 29-66) ===
long long strtonum(const char *numstr, long long minval, long long maxval,
    const char **errstrp);
// === copied from compat/common/strtonum.c END (Line 29-66) ===

#endif