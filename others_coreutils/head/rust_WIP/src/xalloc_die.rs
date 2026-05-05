use std::process;

use ::libc;
extern "C" {
    fn abort() -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
}
#[no_mangle]
pub fn xalloc_die() {
    if 0 != 0 {
        unsafe {
            error(
                exit_failure,
                0,
                b"%s\0" as *const u8 as *const libc::c_char,
                gettext(b"memory exhausted\0" as *const u8 as *const libc::c_char),
            );
        }
        if unsafe { exit_failure } != 0 {
            unreachable!();
        }
    } else {
        let errstatus = unsafe { exit_failure };
        unsafe {
            error(
                errstatus,
                0,
                b"%s\0" as *const u8 as *const libc::c_char,
                gettext(b"memory exhausted\0" as *const u8 as *const libc::c_char),
            );
        }
        if errstatus != 0 {
            unreachable!();
        }

        let errstatus = unsafe { exit_failure };
        unsafe {
            error(
                errstatus,
                0,
                b"%s\0" as *const u8 as *const libc::c_char,
                gettext(b"memory exhausted\0" as *const u8 as *const libc::c_char),
            );
        }
        if errstatus != 0 {
            unreachable!();
        }
    }
    std::process::abort();
}

