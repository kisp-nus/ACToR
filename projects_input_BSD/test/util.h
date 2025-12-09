#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>

extern const char *__progname;

size_t strlcpy(char *dst, const char *src, size_t dsize);
long long strtonum(const char *numstr, long long minval, long long maxval, const char **errstrp);

#endif