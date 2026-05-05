use std::env;

use ::libc;
extern "C" {
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
}
#[no_mangle]
pub fn posix2_version() -> libc::c_int {
    let mut v: libc::c_long = 200809;
    if let Ok(s) = std::env::var("_POSIX2_VERSION") {
        if let Ok(i) = s.parse::<libc::c_long>() {
            v = i;
        }
    }
    return (if v < (-(2147483647) - 1) {
        (-(2147483647) - 1) as libc::c_long
    } else if v < 2147483647 {
        v
    } else {
        2147483647
    }) as libc::c_int;
}

