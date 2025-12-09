/* util.c - Utility functions for tsort */

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <limits.h>
#include "util.h"

/* === copied from compat/common/progname.c BEGIN (Line 12-116) === */
#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/')

#ifndef HAVE___PROGNAME
const char *__progname = NULL;
#endif

const char*
getprogname(void)
{
    #if defined(HAVE_PROGRAM_INVOCATION_SHORT_NAME)
        if(__progname == NULL)
            __progname = program_invocation_short_name;
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
/* === copied from compat/common/progname.c END (Line 12-116) === */

/* === copied from compat/darwin/reallocarray.c BEGIN (Line 25-41) === */
/*
 * This is sqrt(SIZE_MAX+1), as s1*s2 <= SIZE_MAX
 * if both s1 < MUL_NO_OVERFLOW and s2 < MUL_NO_OVERFLOW
 */
#define MUL_NO_OVERFLOW ((size_t)1 << (sizeof (size_t) * 4))

void *
reallocarray (void *optr, size_t nmemb, size_t size)
{
  if ((nmemb >= MUL_NO_OVERFLOW || size >= MUL_NO_OVERFLOW) && nmemb > 0
      && SIZE_MAX / nmemb < size)
    {
      errno = ENOMEM;
      return NULL;
    }
  return realloc (optr, size * nmemb);
}
/* === copied from compat/darwin/reallocarray.c END (Line 25-41) === */

/* === copied from compat/common/ohash.c BEGIN (Line 26-329) === */
struct _ohash_record {
	uint32_t	hv;
	const char	*p;
};

#define DELETED		((const char *)h)
#define NONE		(h->size)

/* Don't bother changing the hash table if the change is small enough.  */
#define MINSIZE		(1UL << 4)
#define MINDELETED	4

static void ohash_resize(struct ohash *);

/* This handles the common case of variable length keys, where the
 * key is stored at the end of the record.
 */
void *
ohash_create_entry(struct ohash_info *i, const char *start, const char **end)
{
	char *p;

	if (!*end)
		*end = start + strlen(start);
	p = (i->alloc)(i->key_offset + (*end - start) + 1, i->data);
	if (p) {
		memcpy(p+i->key_offset, start, *end-start);
		p[i->key_offset + (*end - start)] = '\0';
	}
	return (void *)p;
}

/* hash_delete only frees the hash structure. Use hash_first/hash_next
 * to free entries as well.  */
void
ohash_delete(struct ohash *h)
{
	(h->info.free)(h->t, h->info.data);
#ifndef NDEBUG
	h->t = NULL;
#endif
}

static void
ohash_resize(struct ohash *h)
{
	struct _ohash_record *n;
	size_t ns;
	unsigned int	j;
	unsigned int	i, incr;

	if (4 * h->deleted < h->total) {
		if (h->size >= (UINT_MAX >> 1U))
			ns = UINT_MAX;
		else
			ns = h->size << 1U;
	} else if (3 * h->deleted > 2 * h->total)
		ns = h->size >> 1U;
	else
		ns = h->size;
	if (ns < MINSIZE)
		ns = MINSIZE;

	n = (h->info.calloc)(ns, sizeof(struct _ohash_record), h->info.data);
	if (!n)
		return;

	for (j = 0; j < h->size; j++) {
		if (h->t[j].p != NULL && h->t[j].p != DELETED) {
			i = h->t[j].hv % ns;
			incr = ((h->t[j].hv % (ns - 2)) & ~1) + 1;
			while (n[i].p != NULL) {
				i += incr;
				if (i >= ns)
					i -= ns;
			}
			n[i].hv = h->t[j].hv;
			n[i].p = h->t[j].p;
		}
	}
	(h->info.free)(h->t, h->info.data);
	h->t = n;
	h->size = ns;
	h->total -= h->deleted;
	h->deleted = 0;
}

void *
ohash_remove(struct ohash *h, unsigned int i)
{
	void		*result = (void *)h->t[i].p;

	if (result == NULL || result == DELETED)
		return NULL;

	h->t[i].p = DELETED;
	h->deleted++;
	if (h->deleted >= MINDELETED && 4 * h->deleted > h->total)
		ohash_resize(h);
	return result;
}

void *
ohash_find(struct ohash *h, unsigned int i)
{
	if (h->t[i].p == DELETED)
		return NULL;
	else
		return (void *)h->t[i].p;
}

void *
ohash_insert(struct ohash *h, unsigned int i, void *p)
{
	if (h->t[i].p == DELETED) {
		h->deleted--;
		h->t[i].p = p;
	} else {
		h->t[i].p = p;
		/* Arbitrary resize boundary.  Tweak if not efficient enough.  */
		if (++h->total * 4 > h->size * 3)
			ohash_resize(h);
	}
	return p;
}

unsigned int
ohash_entries(struct ohash *h)
{
	return h->total - h->deleted;
}

void *
ohash_first(struct ohash *h, unsigned int *pos)
{
	*pos = 0;
	return ohash_next(h, pos);
}

void *
ohash_next(struct ohash *h, unsigned int *pos)
{
	for (; *pos < h->size; (*pos)++)
		if (h->t[*pos].p != DELETED && h->t[*pos].p != NULL)
			return (void *)h->t[(*pos)++].p;
	return NULL;
}

void
ohash_init(struct ohash *h, unsigned int size, struct ohash_info *info)
{
	h->size = 1UL << size;
	if (h->size < MINSIZE)
		h->size = MINSIZE;
	/* Copy info so that caller may free it.  */
	h->info.key_offset = info->key_offset;
	h->info.calloc = info->calloc;
	h->info.free = info->free;
	h->info.alloc = info->alloc;
	h->info.data = info->data;
	h->t = (h->info.calloc)(h->size, sizeof(struct _ohash_record),
		    h->info.data);
	h->total = h->deleted = 0;
}

uint32_t
ohash_interval(const char *s, const char **e)
{
	uint32_t k;

	if (!*e)
		*e = s + strlen(s);
	if (s == *e)
		k = 0;
	else
		k = *s++;
	while (s != *e)
		k =  ((k << 2) | (k >> 30)) ^ *s++;
	return k;
}

unsigned int
ohash_lookup_interval(struct ohash *h, const char *start, const char *end,
    uint32_t hv)
{
	unsigned int	i, incr;
	unsigned int	empty;

	empty = NONE;
	i = hv % h->size;
	incr = ((hv % (h->size-2)) & ~1) + 1;
	while (h->t[i].p != NULL) {
		if (h->t[i].p == DELETED) {
			if (empty == NONE)
				empty = i;
		} else if (h->t[i].hv == hv &&
		    strncmp(h->t[i].p+h->info.key_offset, start,
			end - start) == 0 &&
		    (h->t[i].p+h->info.key_offset)[end-start] == '\0') {
			if (empty != NONE) {
				h->t[empty].hv = hv;
				h->t[empty].p = h->t[i].p;
				h->t[i].p = DELETED;
				return empty;
			} else {
				return i;
			}
		}
		i += incr;
		if (i >= h->size)
			i -= h->size;
	}

	/* Found an empty position.  */
	if (empty != NONE)
		i = empty;
	h->t[i].hv = hv;
	return i;
}

unsigned int
ohash_lookup_memory(struct ohash *h, const char *k, size_t size, uint32_t hv)
{
	unsigned int	i, incr;
	unsigned int	empty;

	empty = NONE;
	i = hv % h->size;
	incr = ((hv % (h->size-2)) & ~1) + 1;
	while (h->t[i].p != NULL) {
		if (h->t[i].p == DELETED) {
			if (empty == NONE)
				empty = i;
		} else if (h->t[i].hv == hv &&
		    memcmp(h->t[i].p+h->info.key_offset, k, size) == 0) {
			if (empty != NONE) {
				h->t[empty].hv = hv;
				h->t[empty].p = h->t[i].p;
				h->t[i].p = DELETED;
				return empty;
			} else {
				return i;
			}
		}
		i += incr;
		if (i >= h->size)
			i -= h->size;
	}

	/* Found an empty position.  */
	if (empty != NONE)
		i = empty;
	h->t[i].hv = hv;
	return i;
}

unsigned int
ohash_qlookup(struct ohash *h, const char *s)
{
	const char *e = NULL;
	return ohash_qlookupi(h, s, &e);
}

unsigned int
ohash_qlookupi(struct ohash *h, const char *s, const char **e)
{
	uint32_t hv;

	hv = ohash_interval(s, e);
	return ohash_lookup_interval(h, s, *e, hv);
}
/* === copied from compat/common/ohash.c END (Line 26-329) === */