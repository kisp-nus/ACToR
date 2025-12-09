#include "util.h"
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <sys/types.h>

// === copied from compat/common/progname.c BEGIN (Line 1-116) ===
#define LIBCOMPAT_IS_PATHNAME_SEPARATOR(c) ((c) == '/')

const char *__progname = NULL;

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
// === copied from compat/common/progname.c END (Line 1-116) ===

// === copied from compat/common/heapsort.c BEGIN (Line 1-191) ===
/*
 * Swap two areas of size number of bytes.  Although qsort(3) permits random
 * blocks of memory to be sorted, sorting pointers is almost certainly the
 * common case (and, were it not, could easily be made so).  Regardless, it
 * isn't worth optimizing; the SWAP's get sped up by the cache, and pointer
 * arithmetic gets lost in the time required for comparison function calls.
 */
#define SWAP(a, b, count, size, tmp)                                          \
  {                                                                           \
    count = size;                                                             \
    do                                                                        \
      {                                                                       \
        tmp = *a;                                                             \
        *a++ = *b;                                                            \
        *b++ = tmp;                                                           \
      }                                                                       \
    while (--count);                                                          \
  }

/* Copy one block of size size to another. */
#define COPY(a, b, count, size, tmp1, tmp2)                                   \
  {                                                                           \
    count = size;                                                             \
    tmp1 = a;                                                                 \
    tmp2 = b;                                                                 \
    do                                                                        \
      {                                                                       \
        *tmp1++ = *tmp2++;                                                    \
      }                                                                       \
    while (--count);                                                          \
  }

/*
 * Build the list into a heap, where a heap is defined such that for
 * the records K1 ... KN, Kj/2 >= Kj for 1 <= j/2 <= j <= N.
 *
 * There are two cases.  If j == nmemb, select largest of Ki and Kj.  If
 * j < nmemb, select largest of Ki, Kj and Kj+1.
 */
#define CREATE(initval, nmemb, par_i, child_i, par, child, size, count, tmp)  \
  {                                                                           \
    for (par_i = initval; (child_i = par_i * 2) <= nmemb; par_i = child_i)    \
      {                                                                       \
        child = base + child_i * size;                                        \
        if (child_i < nmemb && compar (child, child + size) < 0)              \
          {                                                                   \
            child += size;                                                    \
            ++child_i;                                                        \
          }                                                                   \
        par = base + par_i * size;                                            \
        if (compar (child, par) <= 0)                                         \
          break;                                                              \
        SWAP (par, child, count, size, tmp);                                  \
      }                                                                       \
  }

/*
 * Select the top of the heap and 'heapify'.  Since by far the most expensive
 * action is the call to the compar function, a considerable optimization
 * in the average case can be achieved due to the fact that k, the displaced
 * element, is usually quite small, so it would be preferable to first
 * heapify, always maintaining the invariant that the larger child is copied
 * over its parent's record.
 *
 * Then, starting from the *bottom* of the heap, finding k's correct place,
 * again maintaining the invariant.  As a result of the invariant no element
 * is 'lost' when k is assigned its correct place in the heap.
 *
 * The time savings from this optimization are on the order of 15-20% for the
 * average case. See Knuth, Vol. 3, page 158, problem 18.
 *
 * XXX Don't break the #define SELECT line, below.  Reiser cpp gets upset.
 */
#define SELECT(par_i, child_i, nmemb, par, child, size, k, count, tmp1, tmp2) \
  {                                                                           \
    for (par_i = 1; (child_i = par_i * 2) <= nmemb; par_i = child_i)          \
      {                                                                       \
        child = base + child_i * size;                                        \
        if (child_i < nmemb && compar (child, child + size) < 0)              \
          {                                                                   \
            child += size;                                                    \
            ++child_i;                                                        \
          }                                                                   \
        par = base + par_i * size;                                            \
        COPY (par, child, count, size, tmp1, tmp2);                           \
      }                                                                       \
    for (;;)                                                                  \
      {                                                                       \
        child_i = par_i;                                                      \
        par_i = child_i / 2;                                                  \
        child = base + child_i * size;                                        \
        par = base + par_i * size;                                            \
        if (child_i == 1 || compar (k, par) < 0)                              \
          {                                                                   \
            COPY (child, k, count, size, tmp1, tmp2);                         \
            break;                                                            \
          }                                                                   \
        COPY (child, par, count, size, tmp1, tmp2);                           \
      }                                                                       \
  }

/*
 * Heapsort -- Knuth, Vol. 3, page 145.  Runs in O (N lg N), both average
 * and worst.  While heapsort is faster than the worst case of quicksort,
 * the BSD quicksort does median selection so that the chance of finding
 * a data set that will trigger the worst case is nonexistent.  Heapsort's
 * only advantage over quicksort is that it requires little additional memory.
 */
int
heapsort (void *vbase, size_t nmemb, size_t size,
          int (*compar) (const void *, const void *))
{
  size_t cnt, i, j, l;
  char tmp, *tmp1, *tmp2;
  char *base, *k, *p, *t;

  if (nmemb <= 1)
    return (0);

  if (!size)
    {
      errno = EINVAL;
      return (-1);
    }

  if ((k = malloc (size)) == NULL)
    return (-1);

  /*
   * Items are numbered from 1 to nmemb, so offset from size bytes
   * below the starting address.
   */
  base = (char *)vbase - size;

  for (l = nmemb / 2 + 1; --l;)
    CREATE (l, nmemb, i, j, t, p, size, cnt, tmp);

  /*
   * For each element of the heap, save the largest element into its
   * final slot, save the displaced element (k), then recreate the
   * heap.
   */
  while (nmemb > 1)
    {
      COPY (k, base + nmemb * size, cnt, size, tmp1, tmp2);
      COPY (base + nmemb * size, base + size, cnt, size, tmp1, tmp2);
      --nmemb;
      SELECT (i, j, nmemb, t, p, size, k, cnt, tmp1, tmp2);
    }
  free (k);
  return (0);
}
// === copied from compat/common/heapsort.c END (Line 1-191) ===

// === copied from compat/common/merge.c BEGIN (Line 1-338) ===
#define NATURAL
#define THRESHOLD 16	/* Best choice for natural merge cut-off. */

static void setup(unsigned char *, unsigned char *, size_t, size_t, int (*)());
static void insertionsort(unsigned char *, size_t, size_t, int (*)());

#define ISIZE sizeof(int)
#define PSIZE sizeof(unsigned char *)
#define ICOPY_LIST(src, dst, last)				\
	do							\
	*(int*)dst = *(int*)src, src += ISIZE, dst += ISIZE;	\
	while(src < last)
#define ICOPY_ELT(src, dst, i)					\
	do							\
	*(int*) dst = *(int*) src, src += ISIZE, dst += ISIZE;	\
	while (i -= ISIZE)

#define CCOPY_LIST(src, dst, last)		\
	do					\
		*dst++ = *src++;		\
	while (src < last)
#define CCOPY_ELT(src, dst, i)			\
	do					\
		*dst++ = *src++;		\
	while (i -= 1)
		
/*
 * Find the next possible pointer head.  (Trickery for forcing an array
 * to do double duty as a linked list when objects do not align with word
 * boundaries.
 */
/* Assumption: PSIZE is a power of 2. */
#define EVAL(p) (unsigned char **)						\
	((unsigned char *)0 +							\
	    (((unsigned char *)p + PSIZE - 1 - (unsigned char *) 0) & ~(PSIZE - 1)))

/*
 * Arguments are as for qsort.
 */
int
mergesort(void *base, size_t nmemb, size_t size,
    int (*cmp)(const void *, const void *))
{
	int i, sense;
	int big, iflag;
	unsigned char *f1, *f2, *t, *b, *tp2, *q, *l1, *l2;
	unsigned char *list2, *list1, *p2, *p, *last, **p1;

	if (size < PSIZE / 2) {		/* Pointers must fit into 2 * size. */
		errno = EINVAL;
		return (-1);
	}

	if (nmemb == 0)
		return (0);

	/*
	 * XXX
	 * Stupid subtraction for the Cray.
	 */
	iflag = 0;
	if (!(size % ISIZE) && !(((char *)base - (char *)0) % ISIZE))
		iflag = 1;

	if ((list2 = malloc(nmemb * size + PSIZE)) == NULL)
		return (-1);

	list1 = base;
	setup(list1, list2, nmemb, size, cmp);
	last = list2 + nmemb * size;
	i = big = 0;
	while (*EVAL(list2) != last) {
	    l2 = list1;
	    p1 = EVAL(list1);
	    for (tp2 = p2 = list2; p2 != last; p1 = EVAL(l2)) {
	    	p2 = *EVAL(p2);
	    	f1 = l2;
	    	f2 = l1 = list1 + (p2 - list2);
	    	if (p2 != last)
	    		p2 = *EVAL(p2);
	    	l2 = list1 + (p2 - list2);
	    	while (f1 < l1 && f2 < l2) {
	    		if ((*cmp)(f1, f2) <= 0) {
	    			q = f2;
	    			b = f1, t = l1;
	    			sense = -1;
	    		} else {
	    			q = f1;
	    			b = f2, t = l2;
	    			sense = 0;
	    		}
	    		if (!big) {	/* here i = 0 */
	    			while ((b += size) < t && cmp(q, b) >sense)
	    				if (++i == 6) {
	    					big = 1;
	    					goto EXPONENTIAL;
	    				}
	    		} else {
EXPONENTIAL:	    		for (i = size; ; i <<= 1)
	    				if ((p = (b + i)) >= t) {
	    					if ((p = t - size) > b &&
						    (*cmp)(q, p) <= sense)
	    						t = p;
	    					else
	    						b = p;
	    					break;
	    				} else if ((*cmp)(q, p) <= sense) {
	    					t = p;
	    					if (i == size)
	    						big = 0; 
	    					goto FASTCASE;
	    				} else
	    					b = p;
		    		while (t > b+size) {
	    				i = (((t - b) / size) >> 1) * size;
	    				if ((*cmp)(q, p = b + i) <= sense)
	    					t = p;
	    				else
	    					b = p;
	    			}
	    			goto COPY;
FASTCASE:	    		while (i > size)
	    				if ((*cmp)(q,
	    					p = b + (i >>= 1)) <= sense)
	    					t = p;
	    				else
	    					b = p;
COPY:	    			b = t;
	    		}
	    		i = size;
	    		if (q == f1) {
	    			if (iflag) {
	    				ICOPY_LIST(f2, tp2, b);
	    				ICOPY_ELT(f1, tp2, i);
	    			} else {
	    				CCOPY_LIST(f2, tp2, b);
	    				CCOPY_ELT(f1, tp2, i);
	    			}
	    		} else {
	    			if (iflag) {
	    				ICOPY_LIST(f1, tp2, b);
	    				ICOPY_ELT(f2, tp2, i);
	    			} else {
	    				CCOPY_LIST(f1, tp2, b);
	    				CCOPY_ELT(f2, tp2, i);
	    			}
	    		}
	    	}
	    	if (f2 < l2) {
	    		if (iflag)
	    			ICOPY_LIST(f2, tp2, l2);
	    		else
	    			CCOPY_LIST(f2, tp2, l2);
	    	} else if (f1 < l1) {
	    		if (iflag)
	    			ICOPY_LIST(f1, tp2, l1);
	    		else
	    			CCOPY_LIST(f1, tp2, l1);
	    	}
	    	*p1 = l2;
	    }
	    tp2 = list1;	/* swap list1, list2 */
	    list1 = list2;
	    list2 = tp2;
	    last = list2 + nmemb*size;
	}
	if (base == list2) {
		memmove(list2, list1, nmemb*size);
		list2 = list1;
	}
	free(list2);
	return (0);
}

#define	swap(a, b) {					\
		s = b;					\
		i = size;				\
		do {					\
			tmp = *a; *a++ = *s; *s++ = tmp; \
		} while (--i);				\
		a -= size;				\
	}
#define reverse(bot, top) {				\
	s = top;					\
	do {						\
		i = size;				\
		do {					\
			tmp = *bot; *bot++ = *s; *s++ = tmp; \
		} while (--i);				\
		s -= size2;				\
	} while(bot < s);				\
}

/*
 * Optional hybrid natural/pairwise first pass.  Eats up list1 in runs of
 * increasing order, list2 in a corresponding linked list.  Checks for runs
 * when THRESHOLD/2 pairs compare with same sense.  (Only used when NATURAL
 * is defined.  Otherwise simple pairwise merging is used.)
 */
void
setup(unsigned char *list1, unsigned char *list2, size_t n, size_t size,
    int (*cmp)(const void *, const void *))
{
	int i, length, size2, sense;
	unsigned char tmp, *f1, *f2, *s, *l2, *last, *p2;

	size2 = size*2;
	if (n <= 5) {
		insertionsort(list1, n, size, cmp);
		*EVAL(list2) = (unsigned char*) list2 + n*size;
		return;
	}
	/*
	 * Avoid running pointers out of bounds; limit n to evens
	 * for simplicity.
	 */
	i = 4 + (n & 1);
	insertionsort(list1 + (n - i) * size, i, size, cmp);
	last = list1 + size * (n - i);
	*EVAL(list2 + (last - list1)) = list2 + n * size;

#ifdef NATURAL
	p2 = list2;
	f1 = list1;
	sense = (cmp(f1, f1 + size) > 0);
	for (; f1 < last; sense = !sense) {
		length = 2;
					/* Find pairs with same sense. */
		for (f2 = f1 + size2; f2 < last; f2 += size2) {
			if ((cmp(f2, f2+ size) > 0) != sense)
				break;
			length += 2;
		}
		if (length < THRESHOLD) {		/* Pairwise merge */
			do {
				p2 = *EVAL(p2) = f1 + size2 - list1 + list2;
				if (sense > 0)
					swap (f1, f1 + size);
			} while ((f1 += size2) < f2);
		} else {				/* Natural merge */
			l2 = f2;
			for (f2 = f1 + size2; f2 < l2; f2 += size2) {
				if ((cmp(f2-size, f2) > 0) != sense) {
					p2 = *EVAL(p2) = f2 - list1 + list2;
					if (sense > 0)
						reverse(f1, f2-size);
					f1 = f2;
				}
			}
			if (sense > 0)
				reverse (f1, f2-size);
			f1 = f2;
			if (f2 < last || cmp(f2 - size, f2) > 0)
				p2 = *EVAL(p2) = f2 - list1 + list2;
			else
				p2 = *EVAL(p2) = list2 + n*size;
		}
	}
#else		/* pairwise merge only. */
	for (f1 = list1, p2 = list2; f1 < last; f1 += size2) {
		p2 = *EVAL(p2) = p2 + size2;
		if (cmp (f1, f1 + size) > 0)
			swap(f1, f1 + size);
	}
#endif /* NATURAL */
}

/*
 * This is to avoid out-of-bounds addresses in sorting the
 * last 4 elements.
 */
static void
insertionsort(unsigned char *a, size_t n, size_t size,
    int (*cmp)(const void *, const void *))
{
	unsigned char *ai, *s, *t, *u, tmp;
	int i;

	for (ai = a+size; --n >= 1; ai += size)
		for (t = ai; t > a; t -= size) {
			u = t - size;
			if (cmp(u, t) <= 0)
				break;
			swap(u, t);
		}
}
// === copied from compat/common/merge.c END (Line 1-338) ===