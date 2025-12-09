/* util.h - Utility functions for tsort */

#ifndef UTIL_H
#define UTIL_H

#include <stddef.h>
#include <stdint.h>

/* === copied from compat/common/progname.c BEGIN (Line 48-51) === */
extern const char *__progname;
const char *getprogname(void);
void setprogname(const char *progname);
/* === copied from compat/common/progname.c END (Line 48-51) === */

/* === copied from compat/darwin/reallocarray.c BEGIN (Line 26-27) === */
void *reallocarray(void *ptr, size_t nmemb, size_t size);
/* === copied from compat/darwin/reallocarray.c END (Line 26-27) === */

/* === copied from compat/include/ohash.h BEGIN (Line 26-80) === */
/* Open hashing support. 
 * Open hashing was chosen because it is much lighter than other hash
 * techniques, and more efficient in most cases.
 */

/* user-visible data structure */
struct ohash_info {
	ptrdiff_t key_offset;
	void *data;	/* user data */
	void *(*calloc)(size_t, size_t, void *);
	void (*free)(void *, void *);
	void *(*alloc)(size_t, void *);
};

struct _ohash_record;

/* private structure. It's there just so you can do a sizeof */
struct ohash {
	struct _ohash_record 	*t;
	struct ohash_info 	info;
	unsigned int 		size;
	unsigned int 		total;
	unsigned int 		deleted;
};

/* For this to be tweakable, we use small primitives, and leave part of the
 * logic to the client application.  e.g., hashing is left to the client
 * application.  We also provide a simple table entry lookup that yields
 * a hashing table index (opaque) to be used in find/insert/remove.
 * The keys are stored at a known position in the client data.
 */
#ifdef __cplusplus
extern "C" {
#endif

	void ohash_init(struct ohash *, unsigned, struct ohash_info *);
	void ohash_delete(struct ohash *);

	unsigned int ohash_lookup_interval(struct ohash *, const char *,
			const char *, uint32_t);
	unsigned int ohash_lookup_memory(struct ohash *, const char *,
			size_t, uint32_t);
	void *ohash_find(struct ohash *, unsigned int);
	void *ohash_remove(struct ohash *, unsigned int);
	void *ohash_insert(struct ohash *, unsigned int, void *);
	void *ohash_first(struct ohash *, unsigned int *);
	void *ohash_next(struct ohash *, unsigned int *);
	unsigned int ohash_entries(struct ohash *);

	void *ohash_create_entry(struct ohash_info *, const char *, const char **);
	uint32_t ohash_interval(const char *, const char **);

	unsigned int ohash_qlookupi(struct ohash *, const char *, const char **);
	unsigned int ohash_qlookup(struct ohash *, const char *);

#ifdef __cplusplus
}
#endif
/* === copied from compat/include/ohash.h END (Line 26-80) === */

#endif /* UTIL_H */