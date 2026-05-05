use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
#[no_mangle]
pub unsafe extern "C" fn fd_reopen(
    mut desired_fd: libc::c_int,
    mut file: *const libc::c_char,
    mut flags: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    let mut fd: libc::c_int = open(file, flags, mode);
    if fd == desired_fd || fd < 0 as libc::c_int {
        return fd
    } else {
        let mut fd2: libc::c_int = dup2(fd, desired_fd);
        let mut saved_errno: libc::c_int = *__errno_location();
        close(fd);
        *__errno_location() = saved_errno;
        return fd2;
    };
}
