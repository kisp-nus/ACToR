use ::libc;
extern "C" {
    fn c_strtod(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
    fn __errno_location() -> *mut libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn cl_strtod(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
) -> libc::c_double {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: libc::c_double = strtod(nptr, &mut end);
    if *end != 0 {
        let mut strtod_errno: libc::c_int = *__errno_location();
        let mut c_end: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut c: libc::c_double = c_strtod(nptr, &mut c_end);
        if end < c_end {
            d = c;
            end = c_end;
        } else {
            *__errno_location() = strtod_errno;
        }
    }
    if !endptr.is_null() {
        *endptr = end;
    }
    return d;
}
