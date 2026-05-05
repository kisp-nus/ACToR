

use std::alloc::{self, Layout};

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
    p: Option<Vec<u8>>,
    n: idx_t,
    s: idx_t,
) -> Option<Vec<u8>> {
    if n as usize <= usize::MAX && s as usize <= usize::MAX {
        let mut nx: usize = n as usize;
        let mut sx: usize = s as usize;
        if n == 0 || s == 0 {
            sx = 1;
            nx = sx;
        }
        let mut vec = p.unwrap_or_else(|| Vec::with_capacity(nx * sx));
        vec.resize(nx * sx, 0);
        return Some(vec);
    } else {
        return None; // Assuming _gl_alloc_nomem() returns None or similar
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
    Some(vec![0u8; total_size]) // Allocate and initialize the vector with zeros
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn irealloc(p: Option<&mut Vec<u8>>, s: usize) -> Option<Vec<u8>> {
    if s <= usize::MAX {
        let mut vec = p.map_or_else(Vec::new, |v| v.clone());
        vec.resize(s, 0);
        Some(vec)
    } else {
        None // Assuming _gl_alloc_nomem() returns None in the idiomatic Rust version
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
pub fn _gl_alloc_nomem() -> *mut libc::c_void {
    // Simulating allocation failure by setting errno and returning a null pointer
    unsafe {
        *__errno_location() = 12; // Set errno to ENOMEM
    }
    std::ptr::null_mut() // Return a null pointer
}

