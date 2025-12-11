#include "util.h"
#include <stdlib.h>
#include <string.h>

#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/')

// === copied from compat/common/progname.c BEGIN (Line 12-116) ===
const char *__progname = NULL;

const char*
getprogname(void)
{
    return __progname;
}

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
// === copied from compat/common/progname.c END (Line 12-116) ===