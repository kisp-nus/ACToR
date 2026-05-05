use std::convert::TryInto;

use ::libc;
extern "C" {
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub fn full_read(
    fd: libc::c_int,
    buf: &mut [u8],
) -> usize {
    let mut total: usize = 0;
    let mut count = buf.len();
    let mut ptr = buf.as_mut_ptr();

    while count > 0 {
        let n_rw: u64;
        unsafe {
            n_rw = safe_read(fd, ptr as *mut libc::c_void, count.try_into().unwrap());
        }
        if n_rw == !(0 as libc::c_int) as u64 {
            break;
        }
        if n_rw == 0 {
            unsafe { *__errno_location() = 0; }
            break;
        } else {
            total += n_rw as usize;
            ptr = unsafe { ptr.add(n_rw as usize) };
            count -= n_rw as usize;
        }
    }
    total
}

