// === copied from compat/common/progname.c BEGIN (Line 1-116) ===

#include "util.h"
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/' || (c) == '\\')
#else
#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/')
#endif

const char *__progname = NULL;

const char*
getprogname(void)
{
    #if defined(HAVE_PROGRAM_INVOCATION_SHORT_NAME)
        if(__progname == NULL)
            __progname = program_invocation_short_name;
    #elif defined(HAVE_GETEXECNAME)
        if(__progname == NULL)
            setprogname(getexecname());
    #endif

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

// === copied from compat/common/progname.c END (Line 1-116) ===