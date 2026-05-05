use std::os::unix::io::FromRawFd;

use ::libc;
extern "C" {
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
}
#[no_mangle]
pub fn dup_safer(fd: i32) -> i32 {
    use std::os::unix::io::AsRawFd;
    use std::os::unix::io::FromRawFd;

    let new_fd = unsafe { rpl_fcntl(fd, 0, libc::F_DUPFD + 1) };
    new_fd
}

