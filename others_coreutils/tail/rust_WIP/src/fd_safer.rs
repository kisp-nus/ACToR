use libc::c_int;

use ::libc;
extern "C" {
    fn dup_safer(_: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub fn fd_safer(fd: i32) -> i32 {
    if (0..=2).contains(&fd) {
        let f = unsafe { dup_safer(fd) };
        unsafe { close(fd) };
        return f;
    }
    fd
}

