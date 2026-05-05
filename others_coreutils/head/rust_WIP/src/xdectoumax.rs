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
    return xnumtoumax(n_str, 10 as libc::c_int, min, max, suffixes, err, err_exit);
}
#[no_mangle]
pub unsafe extern "C" fn xnumtoumax(
    mut n_str: *const libc::c_char,
    mut base: libc::c_int,
    mut min: uintmax_t,
    mut max: uintmax_t,
    mut suffixes: *const libc::c_char,
    mut err: *const libc::c_char,
    mut err_exit: libc::c_int,
) -> uintmax_t {
    let mut s_err: strtol_error = LONGINT_OK;
    let mut tnum: uintmax_t = 0;
    s_err = xstrtoumax(n_str, 0 as *mut *mut libc::c_char, base, &mut tnum, suffixes);
    if s_err as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint {
        if tnum < min || max < tnum {
            s_err = LONGINT_OVERFLOW;
            if tnum > (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_ulong {
                *__errno_location() = 75 as libc::c_int;
            } else {
                *__errno_location() = 34 as libc::c_int;
            }
        }
    } else if s_err as libc::c_uint == LONGINT_OVERFLOW as libc::c_int as libc::c_uint {
        *__errno_location() = 75 as libc::c_int;
    } else if s_err as libc::c_uint
        == LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW as libc::c_int as libc::c_uint
    {
        *__errno_location() = 0 as libc::c_int;
    }
    if s_err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
        if 0 != 0 {
            error(
                if err_exit != 0 { err_exit } else { 1 as libc::c_int },
                if *__errno_location() == 22 as libc::c_int {
                    0 as libc::c_int
                } else {
                    *__errno_location()
                },
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                err,
                quote(n_str),
            );
            if (if err_exit != 0 { err_exit } else { 1 as libc::c_int })
                != 0 as libc::c_int
            {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = if err_exit != 0 {
                    err_exit
                } else {
                    1 as libc::c_int
                };
                error(
                    __errstatus,
                    if *__errno_location() == 22 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        *__errno_location()
                    },
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    err,
                    quote(n_str),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = if err_exit != 0 {
                    err_exit
                } else {
                    1 as libc::c_int
                };
                error(
                    __errstatus,
                    if *__errno_location() == 22 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        *__errno_location()
                    },
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    err,
                    quote(n_str),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        unreachable!();
    }
    return tnum;
}
