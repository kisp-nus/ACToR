









use std::option::Option;

use std::string::String;

use std::slice;

use std::alloc::{self, Layout};

use std::mem;

use std::vec::Vec;

use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
    fn xalloc_die();
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type idx_t = ptrdiff_t;
pub const DEFAULT_MXFAST: C2RustUnnamed = 128;
pub type C2RustUnnamed = libc::c_uint;
pub const DEFAULT_MXFAST_0: C2RustUnnamed_0 = 128;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn irealloc(
    mut p: *mut libc::c_void,
    mut s: idx_t,
) -> *mut libc::c_void {
    if s as libc::c_ulong <= 18446744073709551615 as libc::c_ulong {
        p = realloc(p, (s | (s == 0) as libc::c_int as libc::c_long) as libc::c_ulong);
        return p;
    } else {
        return _gl_alloc_nomem()
    };
}
#[inline]
unsafe extern "C" fn icalloc(mut n: idx_t, mut s: idx_t) -> *mut libc::c_void {
    if (18446744073709551615 as libc::c_ulong) < n as libc::c_ulong {
        if s != 0 as libc::c_int as libc::c_long {
            return _gl_alloc_nomem();
        }
        n = 0 as libc::c_int as idx_t;
    }
    if (18446744073709551615 as libc::c_ulong) < s as libc::c_ulong {
        if n != 0 as libc::c_int as libc::c_long {
            return _gl_alloc_nomem();
        }
        s = 0 as libc::c_int as idx_t;
    }
    return calloc(n as libc::c_ulong, s as libc::c_ulong);
}
#[inline]
fn ireallocarray(n: usize, s: usize) -> *mut libc::c_void {
    if n <= usize::MAX && s <= usize::MAX {
        let mut nx = n;
        let mut sx = s;
        if n == 0 || s == 0 {
            sx = 1;
            nx = sx;
        }
        let p = unsafe { reallocarray(std::ptr::null_mut(), nx.try_into().unwrap(), sx.try_into().unwrap()) };
        return p;
    } else {
        return std::ptr::null_mut(); // Handle out of memory or invalid allocation
    }
}

#[cold]
#[inline]
unsafe extern "C" fn _gl_alloc_nomem() -> *mut libc::c_void {
    *__errno_location() = 12 as libc::c_int;
    return 0 as *mut libc::c_void;
}
#[inline]
fn imalloc(s: usize) -> *mut libc::c_void {
    if s <= usize::MAX {
        let layout = std::alloc::Layout::from_size_align(s, 1).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            std::ptr::null_mut()
        } else {
            ptr as *mut libc::c_void
        }
    } else {
        std::ptr::null_mut()
    }
}

fn check_nonnull(p: *mut libc::c_void) -> *mut libc::c_void {
    if p.is_null() {
        unsafe { xalloc_die() };
    }
    p
}

#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut s: size_t) -> *mut libc::c_void {
    return check_nonnull(malloc(s));
}
#[no_mangle]
pub unsafe extern "C" fn ximalloc(mut s: idx_t) -> *mut libc::c_void {
    let allocated_memory = imalloc(s.try_into().unwrap());
return check_nonnull(allocated_memory);
}
#[no_mangle]
pub unsafe extern "C" fn xcharalloc(mut n: size_t) -> *mut libc::c_char {
    return (if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
        == 1 as libc::c_int as libc::c_ulong
    {
        xmalloc(n)
    } else {
        xnmalloc(n, ::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
    }) as *mut libc::c_char;
}
#[no_mangle]
pub fn xrealloc(p: *mut libc::c_void, s: usize) -> *mut libc::c_void {
    if p.is_null() && s == 0 {
        return std::ptr::null_mut();
    }

    let mut vec = if p.is_null() {
        vec![0; s]
    } else {
        let slice = unsafe { std::slice::from_raw_parts_mut(p as *mut u8, s) };
        let mut v = Vec::from(slice);
        v.resize(s, 0);
        v
    };

    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // Prevent Vec from deallocating the memory
    ptr as *mut libc::c_void
}

#[no_mangle]
pub fn xirealloc(p: &mut Vec<u8>, s: usize) -> Option<Vec<u8>> {
    let mut new_vec = Vec::with_capacity(s);
    let len = p.len();
    new_vec.extend_from_slice(p);
    new_vec.resize(s, 0);
    Some(new_vec)
}

#[no_mangle]
pub unsafe extern "C" fn xreallocarray(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = reallocarray(p, n, s);
    if r.is_null() && (p.is_null() || n != 0 && s != 0) {
        xalloc_die();
    }
    return r;
}
#[no_mangle]
pub fn xireallocarray(n: usize, s: usize) -> Option<Vec<u8>> {
    let total_size = n.checked_mul(s)?;
    let mut vec = Vec::with_capacity(total_size);
    // Assuming that the original function's purpose is to reallocate an array,
    // we can return the vector as a byte array.
    Some(vec)
}

#[no_mangle]
pub unsafe extern "C" fn xnmalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    return xreallocarray(0 as *mut libc::c_void, n, s);
}
#[no_mangle]
pub fn xinmalloc(n: idx_t, s: idx_t) -> Vec<u8> {
    let size = n.checked_mul(s).expect("Overflow in allocation size");
    Vec::with_capacity(size as usize)
}

#[no_mangle]
pub fn x2realloc(
    p: &[u8],
    ps: &mut usize,
) -> Vec<u8> {
    let new_size = *ps + 1; // Assuming we want to allocate one more byte
    let mut new_vec = Vec::with_capacity(new_size);
    new_vec.extend_from_slice(p);
    *ps = new_size; // Update the size
    new_vec
}

#[no_mangle]
pub unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as libc::c_int as libc::c_ulong).wrapping_div(s);
            n = (n as libc::c_ulong)
                .wrapping_add((n == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
    } else {
        let (fresh0, fresh1) = n
            .overflowing_add(
                (n >> 1 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        *(&mut n as *mut size_t) = fresh0;
        if fresh1 {
            xalloc_die();
        }
    }
    p = xreallocarray(p, n, s);
    *pn = n;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn xpalloc(
    mut pa: *mut libc::c_void,
    mut pn: *mut idx_t,
    mut n_incr_min: idx_t,
    mut n_max: ptrdiff_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    let mut n0: idx_t = *pn;
    let mut n: idx_t = 0;
    let (fresh2, fresh3) = n0.overflowing_add(n0 >> 1 as libc::c_int);
    *(&mut n as *mut idx_t) = fresh2;
    if fresh3 {
        n = 9223372036854775807 as libc::c_long;
    }
    if 0 as libc::c_int as libc::c_long <= n_max && n_max < n {
        n = n_max;
    }
    let mut nbytes: idx_t = 0;
    let mut adjusted_nbytes: idx_t = (if if (0 as libc::c_int as idx_t)
        < -(1 as libc::c_int) as idx_t
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { n })
            - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { s })
            - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        && (if s < 0 as libc::c_int as libc::c_long {
            if n < 0 as libc::c_int as libc::c_long {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        -(1 as libc::c_int) as idx_t
                    }) + s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    (n < -(1 as libc::c_int) as idx_t / s) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (s
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + -(1 as libc::c_int) as idx_t
                            >> (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        -(1 as libc::c_int) as idx_t / -s
                    }) <= -(1 as libc::c_int) as libc::c_long - n) as libc::c_int
                }
            } else {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as idx_t
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as idx_t
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as idx_t
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as idx_t)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as idx_t
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as idx_t) as libc::c_int
                }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < n + 0 as libc::c_int as idx_t) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < n
                            && (-(1 as libc::c_int) as libc::c_long
                                - 0 as libc::c_int as idx_t)
                                < n - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (0 as libc::c_int as idx_t / s < n) as libc::c_int
                }
            }
        } else {
            if s == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else {
                if n < 0 as libc::c_int as libc::c_long {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as idx_t
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as idx_t
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as idx_t
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as idx_t)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                }) + 0 as libc::c_int as idx_t
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as idx_t) as libc::c_int
                    }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < s + 0 as libc::c_int as idx_t) as libc::c_int
                        } else {
                            ((-(1 as libc::c_int) as libc::c_long
                                - 0 as libc::c_int as idx_t)
                                < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        (0 as libc::c_int as idx_t / n < s) as libc::c_int
                    }
                } else {
                    (-(1 as libc::c_int) as idx_t / s < n) as libc::c_int
                }
            }
        }) != 0
    {
        let (fresh8, _fresh9) = n.overflowing_mul(s);
        *(&mut nbytes as *mut idx_t) = fresh8;
        1 as libc::c_int
    } else {
        let (fresh10, fresh11) = n.overflowing_mul(s);
        *(&mut nbytes as *mut idx_t) = fresh10;
        fresh11 as libc::c_int
    } != 0
    {
        if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        }
    } else {
        (if nbytes < DEFAULT_MXFAST_0 as libc::c_int as libc::c_long {
            DEFAULT_MXFAST_0 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_ulong
    }) as idx_t;
    if adjusted_nbytes != 0 {
        n = adjusted_nbytes / s;
        nbytes = adjusted_nbytes - adjusted_nbytes % s;
    }
    if pa.is_null() {
        *pn = 0 as libc::c_int as idx_t;
    }
    if n - n0 < n_incr_min
        && {
            let (fresh12, fresh13) = n0.overflowing_add(n_incr_min);
            *(&mut n as *mut idx_t) = fresh12;
            fresh13 as libc::c_int != 0
                || 0 as libc::c_int as libc::c_long <= n_max && n_max < n
                || (if (0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t
                    && ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    && ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    && (if s < 0 as libc::c_int as libc::c_long {
                        if n < 0 as libc::c_int as libc::c_long {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    -(1 as libc::c_int) as idx_t
                                }) + s
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                (n < -(1 as libc::c_int) as idx_t / s) as libc::c_int
                            } else {
                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (s
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                }) != 0
                                {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + -(1 as libc::c_int) as idx_t
                                        >> (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    -(1 as libc::c_int) as idx_t / -s
                                }) <= -(1 as libc::c_int) as libc::c_long - n)
                                    as libc::c_int
                            }
                        } else {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as idx_t
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as idx_t
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 0 as libc::c_int as idx_t
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as idx_t
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as idx_t
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as idx_t) as libc::c_int
                            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((0 as libc::c_int as libc::c_long)
                                        < n + 0 as libc::c_int as idx_t) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < n
                                        && (-(1 as libc::c_int) as libc::c_long
                                            - 0 as libc::c_int as idx_t)
                                            < n - 1 as libc::c_int as libc::c_long) as libc::c_int
                                }
                            } else {
                                (0 as libc::c_int as idx_t / s < n) as libc::c_int
                            }
                        }
                    } else {
                        if s == 0 as libc::c_int as libc::c_long {
                            0 as libc::c_int
                        } else {
                            if n < 0 as libc::c_int as libc::c_long {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) + 0 as libc::c_int as idx_t
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) + 0 as libc::c_int as idx_t
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            }) + 0 as libc::c_int as idx_t
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as idx_t
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as idx_t
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) + 0 as libc::c_int as idx_t) as libc::c_int
                                }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < s + 0 as libc::c_int as idx_t) as libc::c_int
                                    } else {
                                        ((-(1 as libc::c_int) as libc::c_long
                                            - 0 as libc::c_int as idx_t)
                                            < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                                    }
                                } else {
                                    (0 as libc::c_int as idx_t / n < s) as libc::c_int
                                }
                            } else {
                                (-(1 as libc::c_int) as idx_t / s < n) as libc::c_int
                            }
                        }
                    }) != 0
                {
                    let (fresh18, _fresh19) = n.overflowing_mul(s);
                    *(&mut nbytes as *mut idx_t) = fresh18;
                    1 as libc::c_int
                } else {
                    let (fresh20, fresh21) = n.overflowing_mul(s);
                    *(&mut nbytes as *mut idx_t) = fresh20;
                    fresh21 as libc::c_int
                }) != 0
        }
    {
        xalloc_die();
    }
    pa = xrealloc(pa as *mut libc::c_void, nbytes as usize);
    *pn = n;
    return pa;
}
#[no_mangle]
pub fn xzalloc(s: usize) -> Vec<u8> {
    let size = s * std::mem::size_of::<u8>();
    Vec::with_capacity(size)
}

#[no_mangle]
pub fn xizalloc(s: idx_t) -> Vec<u8> {
    let layout = Layout::from_size_align(s as usize, 1).expect("Invalid layout");
    let ptr = unsafe { alloc::alloc(layout) };
    if ptr.is_null() {
        panic!("Memory allocation failed");
    }
    let vec = unsafe { Vec::from_raw_parts(ptr, s as usize, s as usize) };
    vec
}

#[no_mangle]
pub fn xcalloc(n: usize, s: usize) -> *mut libc::c_void {
    let total_size = n.checked_mul(s).expect("Overflow in multiplication");
    let vec = vec![0u8; total_size];
    let boxed_slice = vec.into_boxed_slice();
    Box::into_raw(boxed_slice) as *mut libc::c_void
}

#[no_mangle]
pub fn xicalloc(n: usize, s: usize) -> *mut libc::c_void {
    let total_size = n.checked_mul(s).expect("Overflow in allocation size");
    let allocation = vec![0u8; total_size].into_boxed_slice();
    Box::into_raw(allocation) as *mut libc::c_void
}

#[no_mangle]
pub fn xmemdup(p: &[u8]) -> Vec<u8> {
    let mut result = vec![0; p.len()];
    result.copy_from_slice(p);
    result
}

#[no_mangle]
pub fn ximemdup(p: &[u8]) -> Vec<u8> {
    let mut result = vec![0; p.len()];
    result.copy_from_slice(p);
    result
}

#[no_mangle]
pub fn ximemdup0(p: &[u8]) -> Vec<u8> {
    let mut result = vec![0; p.len() + 1];
    result[..p.len()].copy_from_slice(p);
    result[p.len()] = 0; // Null-terminate the string
    result
}

#[no_mangle]
pub fn xstrdup(string: &str) -> String {
    let length = string.len();
    let mut vec = Vec::with_capacity(length + 1);
    vec.extend_from_slice(string.as_bytes());
    vec.push(0); // Null-terminate the string
    String::from_utf8(vec).expect("Failed to convert to String")
}

