
use std::usize;

use std::vec::Vec;

use std::alloc;
use std::ptr;
use std::slice;

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
    p: Option<&mut [u8]>,
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
        let new_size = nx.checked_mul(sx)?;
        let mut vec = match p {
            Some(slice) => {
                let mut new_vec = Vec::with_capacity(new_size);
                new_vec.extend_from_slice(slice);
                new_vec
            }
            None => Vec::with_capacity(new_size),
        };
        vec.resize(new_size, 0);
        Some(vec)
    } else {
        return None; // Handle memory allocation failure appropriately
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn icalloc(n: usize, s: usize) -> Option<Vec<u8>> {
    if n > usize::MAX / s {
        if s != 0 {
            return None; // _gl_alloc_nomem();
        }
        return Some(Vec::new()); // Return an empty Vec
    }
    if s > usize::MAX / n {
        if n != 0 {
            return None; // _gl_alloc_nomem();
        }
        return Some(Vec::new()); // Return an empty Vec
    }
    let total_size = n * s;
    let vec = vec![0u8; total_size];
    Some(vec)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn irealloc(p: Option<&mut [u8]>, s: usize) -> Option<Vec<u8>> {
    if s <= usize::MAX {
        let new_size = if s == 0 { 0 } else { s };
        let mut vec = match p {
            Some(slice) => {
                let mut new_vec = Vec::with_capacity(new_size);
                new_vec.extend_from_slice(slice);
                new_vec
            },
            None => Vec::with_capacity(new_size),
        };
        vec.resize(new_size, 0);
        Some(vec)
    } else {
        None // Assuming _gl_alloc_nomem() returns None in this context
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn imalloc(s: usize) -> Option<Box<[u8]>> {
    if s <= usize::MAX {
        let layout = std::alloc::Layout::from_size_align(s, 1).ok()?;
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, s)) })
        }
    } else {
        None // Assuming _gl_alloc_nomem() returns None in idiomatic Rust
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
