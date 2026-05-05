use std::ffi::CStr;

use ::libc;
extern "C" {
    fn setlocale_null_r(
        category: libc::c_int,
        buf: *mut libc::c_char,
        bufsize: size_t,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub fn hard_locale(category: libc::c_int) -> bool {
    let mut locale: [libc::c_char; 257] = [0; 257];
    unsafe {
        if setlocale_null_r(
            category,
            locale.as_mut_ptr(),
            locale.len() as libc::c_ulong,
        ) != 0 {
            return false;
        }
    }
    let locale_str = unsafe { CStr::from_ptr(locale.as_ptr()).to_string_lossy() };
    if locale_str == "C" || locale_str == "POSIX" {
        return false;
    }
    return true;
}

