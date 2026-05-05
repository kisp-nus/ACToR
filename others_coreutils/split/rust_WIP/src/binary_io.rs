use ::libc;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn set_binary_mode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn __gl_setmode(
    mut _fd: libc::c_int,
    mut _mode: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
