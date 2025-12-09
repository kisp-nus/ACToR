#include "util.h"
#include <stdlib.h>
#include <string.h>

// === copied from compat/common/progname.c BEGIN (Line 7-9) ===
#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/')
// === copied from compat/common/progname.c END (Line 7-9) ===

// === copied from compat/common/progname.c BEGIN (Line 11-13) ===
const char *__progname = NULL;
// === copied from compat/common/progname.c END (Line 11-13) ===

// === copied from compat/common/progname.c BEGIN (Line 15-101) ===
const char*
getprogname(void)
{
    return __progname;
}
// === copied from compat/common/progname.c END (Line 15-101) ===

// === copied from compat/common/progname.c BEGIN (Line 103-116) ===
void
setprogname(const char *progname)
{
    size_t i;

    for (i = strlen(progname); i > 0; i--) {
        if (LIBCOMPAT_IS_PATHNAME_SEPARATOR(progname[i - 1])) {
            __progname = progname + i;
            return;
        }
    }

    __progname = progname;
}
// === copied from compat/common/progname.c END (Line 103-116) ===