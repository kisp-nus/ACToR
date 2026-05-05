use std::convert::TryInto;

use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> size_t;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub fn full_write(
    fd: libc::c_int,
    buf: &[u8],
) -> usize {
    let mut total: usize = 0;
    let mut count = buf.len();
    let mut ptr = buf.as_ptr();

    while count > 0 {
        let n_rw: u64;
        unsafe {
            n_rw = safe_write(fd, ptr as *const libc::c_void, count.try_into().unwrap());
        }
        if n_rw == !(0 as libc::c_int) as u64 {
            break;
        }
        if n_rw == 0 {
            unsafe { *__errno_location() = 28 as libc::c_int };
            break;
        } else {
            total += n_rw as usize;
            ptr = unsafe { ptr.add(n_rw as usize) };
            count -= n_rw as usize;
        }
    }
    total
}

