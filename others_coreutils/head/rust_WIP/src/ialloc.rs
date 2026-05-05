use std::option::Option;

use std::mem;

use std::vec::Vec;

use std::alloc;
use std::ptr;

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
    p: &mut Vec<u8>,
    n: idx_t,
    s: idx_t,
) -> Option<&mut Vec<u8>> {
    if n as usize <= usize::MAX && s as usize <= usize::MAX {
        let mut nx: usize = n as usize;
        let mut sx: usize = s as usize;
        if n == 0 || s == 0 {
            sx = 1;
            nx = sx;
        }
        p.resize(nx * sx, 0);
        Some(p)
    } else {
        unsafe {
            _gl_alloc_nomem();
        }
        None
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
        return Some(Vec::new());
    }
    if s > usize::MAX / n {
        if n != 0 {
            return None; // _gl_alloc_nomem();
        }
        return Some(Vec::new());
    }
    let total_size = n * s;
    let mut vec = Vec::with_capacity(total_size);
    unsafe {
        // This is safe because we are allocating a Vec, which guarantees the memory is valid.
        let ptr = vec.as_mut_ptr();
        std::mem::forget(vec); // Prevent Vec from deallocating the memory
        Some(Vec::from_raw_parts(ptr, total_size, total_size))
    }
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
        None // Assuming _gl_alloc_nomem() returns a null pointer, we return None here.
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
