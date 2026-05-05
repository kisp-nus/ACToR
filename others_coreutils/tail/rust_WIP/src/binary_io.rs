
use std::os::unix::io::AsRawFd;

use ::libc;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn set_binary_mode(fd: i32, mode: i32) -> i32 {
    __gl_setmode(fd, mode)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn __gl_setmode(fd: i32, mode: i32) -> i32 {
    0
}

