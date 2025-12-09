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