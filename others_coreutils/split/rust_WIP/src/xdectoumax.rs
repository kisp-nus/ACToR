use std::ffi::CStr;
use std::io::Write;

use ::libc;
extern "C" {
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
}
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
pub const LONGINT_OK: strtol_error = 0;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
#[no_mangle]
pub unsafe extern "C" fn xdectoumax(
    mut n_str: *const libc::c_char,
    mut min: uintmax_t,
    mut max: uintmax_t,
    mut suffixes: *const libc::c_char,
    mut err: *const libc::c_char,
    mut err_exit: libc::c_int,
) -> uintmax_t {
    let result = xnumtoumax(
    std::ffi::CStr::from_ptr(n_str).to_str().unwrap(),
    10,
    min,
    max,
    suffixes.as_ref().map(|s| std::ffi::CStr::from_ptr(s).to_str().unwrap()),
    std::ffi::CStr::from_ptr(err).to_str().unwrap(),
    err_exit,
);
return result;
}
#[no_mangle]
pub fn xnumtoumax(
    n_str: &str,
    base: i32,
    min: u64,
    max: u64,
    suffixes: Option<&str>,
    err: &str,
    err_exit: i32,
) -> u64 {
    let mut tnum: u64 = 0;
    let mut end_ptr: *mut libc::c_char = std::ptr::null_mut();

    // Call to unsafe function requires an unsafe block
    let s_err = unsafe {
        xstrtoumax(
            n_str.as_ptr() as *const libc::c_char,
            &mut end_ptr,
            base,
            &mut tnum,
            suffixes.map_or(std::ptr::null(), |s| s.as_ptr() as *const libc::c_char),
        )
    };

    match s_err {
        LONGINT_OK => {
            if tnum < min || tnum > max {
                eprintln!("Value out of range");
                std::process::exit(err_exit);
            }
        }
        LONGINT_OVERFLOW => {
            eprintln!("Overflow occurred");
            std::process::exit(err_exit);
        }
        LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW => {
            eprintln!("Invalid suffix character with overflow");
            std::process::exit(err_exit);
        }
        _ => {
            eprintln!("Unknown error");
            std::process::exit(err_exit);
        }
    }

    tnum
}

