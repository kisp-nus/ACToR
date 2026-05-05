
use std::ffi::CString;

use ::libc;
extern "C" {
    fn mkostemp(__template: *mut libc::c_char, __flags: libc::c_int) -> libc::c_int;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn fd_safer(_: libc::c_int) -> libc::c_int;
    fn fd_safer_flag(_: libc::c_int, _: libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub fn mkstemp_safer(templ: &mut String) -> libc::c_int {
    let c_str = std::ffi::CString::new(templ.clone()).expect("CString::new failed");
    let fd = unsafe { mkstemp(c_str.as_ptr() as *mut libc::c_char) };
    if fd == -1 {
        return -1; // Handle error appropriately
    }
    unsafe { fd_safer(fd) }
}

#[no_mangle]
pub fn mkostemp_safer(templ: &mut CString, flags: libc::c_int) -> libc::c_int {
    let templ_ptr = templ.as_ptr() as *mut libc::c_char;
    unsafe {
        return fd_safer_flag(mkostemp(templ_ptr, flags), flags);
    }
}

