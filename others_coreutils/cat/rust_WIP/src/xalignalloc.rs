
use std::alloc::{alloc, Layout};

use ::libc;
extern "C" {
    fn aligned_alloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn xalloc_die();
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type idx_t = ptrdiff_t;
#[inline]
fn alignalloc(alignment: usize, size: usize) -> *mut libc::c_void {
    let alignment = if alignment.is_power_of_two() && alignment != 0 {
        alignment
    } else {
        usize::MAX
    };
    
    let size = if size != 0 {
        size
    } else {
        usize::MAX
    };

    let layout = std::alloc::Layout::from_size_align(size, alignment).ok();
    if let Some(layout) = layout {
        unsafe {
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                std::ptr::null_mut()
            } else {
                ptr as *mut libc::c_void
            }
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub fn xalignalloc(alignment: usize, size: usize) -> Box<[u8]> {
    let layout = Layout::from_size_align(size, alignment).expect("Invalid layout");
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        unsafe { xalloc_die() };
    }
    let slice = unsafe { std::slice::from_raw_parts_mut(ptr, size) };
    let box_slice = unsafe { Box::from_raw(slice) };
    box_slice
}

