#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>

void *reallocarray(void *ptr, size_t nmemb, size_t size);
size_t strlcpy(char *dst, const char *src, size_t dsize);
void *recallocarray(void *ptr, size_t oldnmemb, size_t newnmemb, size_t size);
void explicit_bzero(void *buf, size_t len);

#endif