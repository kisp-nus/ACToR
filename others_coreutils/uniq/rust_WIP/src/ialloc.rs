

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
pub fn ireallocarray(
    mut p: Option<Vec<u8>>,
    mut n: usize,
    mut s: usize,
) -> Option<Vec<u8>> {
    if n <= usize::MAX && s <= usize::MAX {
        let mut nx: usize = n;
        let mut sx: usize = s;
        if n == 0 || s == 0 {
            sx = 1;
            nx = sx;
        }
        let new_size = nx.checked_mul(sx)?;
        p = p.map(|mut vec| {
            vec.resize(new_size, 0);
            vec
        });
        return p;
    } else {
        return None; // Assuming _gl_alloc_nomem() returns a null pointer, we return None here.
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn icalloc(n: usize, s: usize) -> Option<Vec<u8>> {
    if n > usize::MAX {
        if s != 0 {
            return None; // Equivalent to _gl_alloc_nomem()
        }
        return Some(Vec::new()); // Return an empty vector
    }
    if s > usize::MAX {
        if n != 0 {
            return None; // Equivalent to _gl_alloc_nomem()
        }
        return Some(Vec::new()); // Return an empty vector
    }
    let total_size = n.checked_mul(s)?;
    Some(vec![0u8; total_size]) // Allocate a vector filled with zeros
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn irealloc(p: Option<Vec<u8>>, s: usize) -> Option<Vec<u8>> {
    if s <= usize::MAX {
        let new_size = if s == 0 { 1 } else { s };
        let mut vec = p.unwrap_or_else(|| Vec::with_capacity(new_size));
        vec.resize(new_size, 0);
        Some(vec)
    } else {
        None // Assuming _gl_alloc_nomem() returns None in this context
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn imalloc(s: idx_t) -> Option<Vec<u8>> {
    if s as usize <= usize::MAX {
        let vec = vec![0u8; s as usize];
        Some(vec)
    } else {
        None // Assuming _gl_alloc_nomem() indicates failure by returning None
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
