
use std::usize;

use std::option::Option;

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
    p: Option<&mut Vec<u8>>,
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
        let mut vec = p.map_or_else(|| Vec::with_capacity(nx * sx), |v| {
            v.resize(nx * sx, 0);
            v.clone()
        });
        Some(vec)
    } else {
        return None; // Assuming _gl_alloc_nomem() returns None in this context
    }
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
    let vec = vec![0u8; total_size]; // Allocate and zero-initialize
    Some(vec)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn irealloc(p: Option<&mut [u8]>, s: usize) -> Option<Vec<u8>> {
    if s <= usize::MAX {
        let new_size = if s == 0 { 1 } else { s };
        let mut vec = Vec::with_capacity(new_size);
        if let Some(old) = p {
            vec.extend_from_slice(old);
        }
        Some(vec)
    } else {
        None
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn imalloc(s: idx_t) -> Option<Box<[u8]>> {
    if s as usize <= usize::MAX {
        let layout = std::alloc::Layout::from_size_align(s as usize, 1).ok()?;
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, s as usize)) })
        }
    } else {
        None // Return None instead of calling _gl_alloc_nomem()
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

