use std::ffi::CStr;

use ::libc;
extern "C" {
    fn copy_file_range(
        __infd: libc::c_int,
        __pinoff: *mut __off64_t,
        __outfd: libc::c_int,
        __poutoff: *mut __off64_t,
        __length: size_t,
        __flags: libc::c_uint,
    ) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
#[no_mangle]
pub fn rpl_copy_file_range(
    infd: libc::c_int,
    pinoff: &mut off_t,
    outfd: libc::c_int,
    poutoff: &mut off_t,
    length: size_t,
    flags: libc::c_uint,
) -> ssize_t {
    thread_local! {
        static OK: std::cell::RefCell<Option<libc::c_schar>> = std::cell::RefCell::new(None);
    }

    let ok = OK.with(|ok_ref| {
        let mut ok_value = ok_ref.borrow_mut();
        if ok_value.is_none() {
            let mut name: utsname = utsname {
                sysname: [0; 65],
                nodename: [0; 65],
                release: [0; 65],
                version: [0; 65],
                machine: [0; 65],
                domainname: [0; 65],
            };
            unsafe { uname(&mut name) };
            let release = unsafe { std::ffi::CStr::from_ptr(name.release.as_ptr()) };
            let release_str = release.to_str().unwrap_or("");

            let result = if release_str.len() > 1 && release_str.chars().nth(1) != Some('.')
                || release_str.chars().nth(0).unwrap_or('0') > '5'
                || (release_str.chars().nth(0) == Some('5') && (release_str.chars().nth(3) != Some('.') || release_str.chars().nth(2).unwrap_or('0') < '2'))
            {
                1
            } else {
                -1
            };
            *ok_value = Some(result as libc::c_schar);
            result
        } else {
            ok_value.unwrap() as libc::c_int
        }
    });

    if ok > 0 {
        return unsafe { copy_file_range(infd, pinoff, outfd, poutoff, length, flags) };
    }
    unsafe { *__errno_location() = 38 };
    return -1;
}

