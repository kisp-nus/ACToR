#ifndef UTIL_COMPAT_H
#define UTIL_COMPAT_H

#include <stddef.h>
#include <sys/types.h>
#include <unistd.h>

/* === copied from compat/include/compat.h BEGIN (Line 54-56) === */
extern const char *__progname;
/* === copied from compat/include/compat.h END (Line 54-56) === */

/* === copied from compat/common/progname.c BEGIN (Line 15-16, 103-115) === */
const char* getprogname(void);
void setprogname(const char *progname);
/* === copied from compat/common/progname.c END (Line 15-16, 103-115) === */

/* === copied from compat/include/compat.h BEGIN (Line 160-161) === */
void strmode(int, char *);
/* === copied from compat/include/compat.h END (Line 160-161) === */

/* === copied from compat/include/compat.h BEGIN (Line 167-173) === */
#ifndef __APPLE__
const char *user_from_uid(uid_t, int);
const char *group_from_gid(gid_t, int);
#endif

int uid_from_user(const char *, uid_t *);
int gid_from_group(const char *, gid_t *);
/* === copied from compat/include/compat.h END (Line 167-173) === */

/* === copied from compat/include/compat.h BEGIN (Line 175-177) === */
int scan_scaled(char *, long long *);
int fmt_scaled(long long, char *);
/* === copied from compat/include/compat.h END (Line 175-177) === */

/* === copied from compat/include/compat.h BEGIN (Line 179-180) === */
char *getbsize(int *, long *);
/* === copied from compat/include/compat.h END (Line 179-180) === */

/* === copied from compat/include/compat.h BEGIN (Line 237) === */
#define FMT_SCALED_STRSIZE 7 /* minus sign, 4 digits, suffix, null byte */
/* === copied from compat/include/compat.h END (Line 237) === */

/* === copied from compat/include/compat.h BEGIN (Line 241-242) === */
#define _PW_BUF_LEN sysconf (_SC_GETPW_R_SIZE_MAX)
#define _GR_BUF_LEN sysconf (_SC_GETGR_R_SIZE_MAX)
/* === copied from compat/include/compat.h END (Line 241-242) === */

/* === copied from compat/include/compat.h BEGIN (Line 158) === */
long long strtonum(const char *, long long, long long, const char **);
/* === copied from compat/include/compat.h END (Line 158) === */

#endif /* UTIL_COMPAT_H */