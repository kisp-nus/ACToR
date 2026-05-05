use ::libc;
extern "C" {
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fd_safer(_: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut libc::c_void,
    pub __gr_top: *mut libc::c_void,
    pub __vr_top: *mut libc::c_void,
    pub __gr_offs: libc::c_int,
    pub __vr_offs: libc::c_int,
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn open_safer(
    mut file: *const libc::c_char,
    mut flags: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut mode: mode_t = 0 as libc::c_int as mode_t;
    if flags & 0o100 as libc::c_int != 0 {
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        mode = ap.arg::<mode_t>();
    }
    return fd_safer(open(file, flags, mode));
}
