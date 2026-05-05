use std::os::unix::io::FromRawFd;

use ::libc;
extern "C" {
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
}
#[no_mangle]
pub fn dup_safer(fd: i32) -> i32 {
    use std::os::unix::io::AsRawFd;
    use std::os::unix::net::UnixStream;

    let stream = unsafe { UnixStream::from_raw_fd(fd) };
    let new_fd = stream.as_raw_fd();
    std::mem::forget(stream); // Prevent the stream from closing the fd
    return new_fd;
}

