
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
    if suffixes.is_null() { None } else { Some(std::ffi::CStr::from_ptr(suffixes).to_str().unwrap()) },
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
    let suffix_ptr = suffixes.map_or(std::ptr::null(), |s| s.as_ptr() as *const libc::c_char);

    unsafe {
        let s_err = xstrtoumax(n_str.as_ptr() as *const libc::c_char, &mut end_ptr, base, &mut tnum, suffix_ptr);

        if s_err as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint {
            if tnum < min || tnum > max {
                let overflow_code = if tnum > (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_ulong {
                    75
                } else {
                    34
                };
                *__errno_location() = overflow_code;
                panic!("Value out of range: {}", tnum);
            }
        } else if s_err as libc::c_uint == LONGINT_OVERFLOW as libc::c_int as libc::c_uint {
            *__errno_location() = 75;
        } else if s_err as libc::c_uint == LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW as libc::c_int as libc::c_uint {
            *__errno_location() = 0;
        }

        if s_err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
            let exit_code = if err_exit != 0 { err_exit } else { 1 };
            let errno_value = *__errno_location();
            error(exit_code, if errno_value == 22 { 0 } else { errno_value }, b"%s: %s\0" as *const u8 as *const libc::c_char, err.as_ptr() as *const libc::c_char, quote(n_str.as_ptr() as *const libc::c_char));
            if exit_code != 0 {
                unreachable!();
            }
        }
    }

    tnum
}

