use std::convert::TryInto;

use ::libc;
extern "C" {
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub const SYS_BUFSIZE_MAX: C2RustUnnamed = 2146435072;
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
pub fn safe_read(fd: libc::c_int, buf: &mut [u8]) -> usize {
    let mut count = buf.len() as u64;
    let mut total_read = 0;

    loop {
        let result = unsafe { read(fd, buf.as_mut_ptr() as *mut libc::c_void, count) };
        if result >= 0 {
            total_read += result as usize;
            return total_read;
        } else {
            let err = unsafe { *__errno_location() };
            if err == 4 { // Interrupted system call
                continue;
            }
            if err == 22 && (SYS_BUFSIZE_MAX as libc::c_ulong) < count {
                count = SYS_BUFSIZE_MAX as libc::c_int as u64;
            } else {
                return total_read;
            }
        }
    }
}

