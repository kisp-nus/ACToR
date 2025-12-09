#ifndef UTIL_H
#define UTIL_H

// === copied from compat/include/compat.h BEGIN (Line 12-12) ===
extern const char *__progname;
// === copied from compat/include/compat.h END (Line 12-12) ===

// === copied from compat/common/progname.c BEGIN (Line 15-116) ===
const char* getprogname(void);
void setprogname(const char *progname);
// === copied from compat/common/progname.c END (Line 15-116) ===

#endif