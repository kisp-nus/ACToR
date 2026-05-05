use std::any::Any;

use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn aligned_alloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type idx_t = ptrdiff_t;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn alignalloc(
    mut alignment: idx_t,
    mut size: idx_t,
) -> *mut libc::c_void {
    if (-(1 as libc::c_int) as size_t) < alignment as libc::c_ulong {
        alignment = -(1 as libc::c_int) as size_t as idx_t;
    }
    if (-(1 as libc::c_int) as size_t) < size as libc::c_ulong {
        size = -(1 as libc::c_int) as size_t as idx_t;
    }
    return aligned_alloc(alignment as libc::c_ulong, size as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn alignfree(ptr: Option<Box<dyn std::any::Any>>) {
    if let Some(boxed) = ptr {
        drop(boxed);
    }
}

