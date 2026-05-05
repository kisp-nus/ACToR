use std::alloc::Layout;

use std::option::Option;

use std::vec::Vec;

use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type idx_t = ptrdiff_t;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn ireallocarray(
    mut p: *mut libc::c_void,
    mut n: idx_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    if n as libc::c_ulong <= 18446744073709551615 as libc::c_ulong
        && s as libc::c_ulong <= 18446744073709551615 as libc::c_ulong
    {
        let mut nx: size_t = n as size_t;
        let mut sx: size_t = s as size_t;
        if n == 0 as libc::c_int as libc::c_long || s == 0 as libc::c_int as libc::c_long
        {
            sx = 1 as libc::c_int as size_t;
            nx = sx;
        }
        p = reallocarray(p, nx, sx);
        return p;
    } else {
        return _gl_alloc_nomem()
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn icalloc(n: usize, s: usize) -> Option<Vec<u8>> {
    if n > usize::MAX / s {
        if s != 0 {
            return None; // Equivalent to _gl_alloc_nomem()
        }
        return Some(Vec::new()); // Return an empty Vec
    }
    if s > usize::MAX / n {
        if n != 0 {
            return None; // Equivalent to _gl_alloc_nomem()
        }
        return Some(Vec::new()); // Return an empty Vec
    }
    let total_size = n * s;
    Some(vec![0u8; total_size]) // Allocate and initialize the Vec with zeroes
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn irealloc(p: Option<&mut [u8]>, s: usize) -> Option<Vec<u8>> {
    if s <= usize::MAX {
        let new_size = if s == 0 { 0 } else { s };
        let mut vec = p.map(|slice| Vec::from(slice)).unwrap_or_else(Vec::new);
        vec.resize(new_size, 0);
        Some(vec)
    } else {
        None
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn imalloc(s: idx_t) -> Option<Vec<u8>> {
    if s as usize <= usize::MAX {
        let vec = Vec::with_capacity(s as usize);
        Some(vec)
    } else {
        None
    }
}

#[no_mangle]
#[cold]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn _gl_alloc_nomem() -> *mut libc::c_void {
    *__errno_location() = 12 as libc::c_int;
    return 0 as *mut libc::c_void;
}
