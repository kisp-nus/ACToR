
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
    let alignment = if alignment.is_power_of_two() && alignment > 0 { alignment } else { usize::MAX };
    let size = if size > 0 { size } else { usize::MAX };

    let ptr = unsafe { aligned_alloc(alignment.try_into().unwrap(), size.try_into().unwrap()) };
    if ptr.is_null() {
        std::ptr::null_mut()
    } else {
        ptr
    }
}

#[no_mangle]
pub unsafe extern "C" fn xalignalloc(
    mut alignment: idx_t,
    mut size: idx_t,
) -> *mut libc::c_void {
    let p: *mut libc::c_void = alignalloc(alignment.try_into().unwrap(), size.try_into().unwrap());
    if p.is_null() {
        xalloc_die();
    }
    return p;
}
