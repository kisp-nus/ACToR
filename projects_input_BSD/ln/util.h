#ifndef UTIL_H
#define UTIL_H

/* === copied from compat/include/compat.h BEGIN (Line 54-56) === */
extern const char *__progname;
/* === copied from compat/include/compat.h END (Line 54-56) === */

/* === copied from compat/common/progname.c BEGIN (Line 15-16, 103-115) === */
const char* getprogname(void);
void setprogname(const char *progname);
/* === copied from compat/common/progname.c END (Line 15-16, 103-115) === */

#endif /* UTIL_H */