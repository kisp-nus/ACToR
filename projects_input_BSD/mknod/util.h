#ifndef UTIL_H
#define UTIL_H

#include <sys/types.h>
#include <sys/stat.h>

// === copied from CMakeLists.txt BEGIN (Line 29) ===
#ifndef DEFFILEMODE
#define DEFFILEMODE (S_IRUSR|S_IWUSR|S_IRGRP|S_IWGRP|S_IROTH|S_IWOTH)
#endif
// === copied from CMakeLists.txt END (Line 29) ===

// === copied from compat/darwin/reallocarray.c BEGIN (Line 32-41) ===
void *reallocarray(void *optr, size_t nmemb, size_t size);
// === copied from compat/darwin/reallocarray.c END (Line 32-41) ===

// === copied from compat/linux/setmode.c BEGIN (Line 77-78, 162-163) ===
mode_t getmode(const void *bbox, mode_t omode);
void *setmode(const char *p);
// === copied from compat/linux/setmode.c END (Line 77-78, 162-163) ===

#endif /* UTIL_H */