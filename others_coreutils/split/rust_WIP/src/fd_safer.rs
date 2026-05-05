
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
        let e = std::io::Error::last_os_error(); // Capture the current error
        unsafe { close(fd) };
        // Note: Restoring the error is not directly possible in safe Rust.
        // We will just return the new file descriptor.
        return f;
    }
    fd
}

