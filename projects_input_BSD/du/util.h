// === copied from compat/include/compat.h BEGIN (Line 158-159, 175-180, 207-214) ===
long long strtonum (const char *, long long, long long, const char **);
int scan_scaled (char *, long long *);
int fmt_scaled (long long, char *);
char *getbsize (int *, long *);
#if defined __linux__|| defined __MINGW32__
size_t strlcpy (char *, const char *, size_t);
#endif
// === copied from compat/include/compat.h END (Line 158-159, 175-180, 207-214) ===

// === copied from compat/include/compat.h BEGIN (Line 237) ===
#define FMT_SCALED_STRSIZE 7 /* minus sign, 4 digits, suffix, null byte */
// === copied from compat/include/compat.h END (Line 237) ===