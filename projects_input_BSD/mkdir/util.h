#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>

// === copied from compat/common/progname.c BEGIN (Line 12-13) ===
extern const char *__progname;
// === copied from compat/common/progname.c END (Line 12-13) ===

// === copied from compat/common/progname.c BEGIN (Line 15-16) ===
const char* getprogname(void);
// === copied from compat/common/progname.c END (Line 15-16) ===

// === copied from compat/common/progname.c BEGIN (Line 103-104) ===
void setprogname(const char *progname);
// === copied from compat/common/progname.c END (Line 103-104) ===

// === copied from compat/linux/setmode.c BEGIN (Line 77-78) ===
mode_t getmode(const void *bbox, mode_t omode);
// === copied from compat/linux/setmode.c END (Line 77-78) ===

// === copied from compat/linux/setmode.c BEGIN (Line 162-163) ===
void *setmode(const char *p);
// === copied from compat/linux/setmode.c END (Line 162-163) ===

#endif