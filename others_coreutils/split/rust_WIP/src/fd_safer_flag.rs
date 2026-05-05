use std::os::unix::io::{AsRawFd, RawFd};
use std::fs::File;
use std::io::{self, ErrorKind};

use ::libc;
extern "C" {
    fn dup_safer_flag(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub fn fd_safer_flag(fd: i32, flag: i32) -> Result<i32, io::Error> {
    if (0..=2).contains(&fd) {
        let f = unsafe { dup_safer_flag(fd, flag) }; // Call to unsafe function
        let e = std::io::Error::last_os_error();
        unsafe { libc::close(fd) }; // Close the file descriptor safely
        return Ok(f);
    }
    Ok(fd)
}

