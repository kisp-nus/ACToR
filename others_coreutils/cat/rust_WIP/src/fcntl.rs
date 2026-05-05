

use std::ffi::VaList;

use ::libc;
extern "C" {
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
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
pub type va_list = __builtin_va_list;
#[no_mangle]
pub fn rpl_fcntl(
    fd: libc::c_int,
    action: libc::c_int,
    args: &mut std::ffi::VaList,
) -> libc::c_int {
    let mut result: libc::c_int = -1;

    match action {
        0 => {
            let target: libc::c_int = unsafe { args.arg::<libc::c_int>() };
            result = unsafe { rpl_fcntl_DUPFD(fd, target) };
        }
        1030 => {
            let target: libc::c_int = unsafe { args.arg::<libc::c_int>() };
            result = unsafe { rpl_fcntl_DUPFD_CLOEXEC(fd, target) };
        }
        _ => {
            match action {
                1 | 3 => {
                    result = unsafe { fcntl(fd, action) };
                }
                1025 => {
                    let p: *mut libc::c_void = unsafe { args.arg::<*mut libc::c_void>() };
                    result = unsafe { fcntl(fd, action, p) };
                }
                9 => {
                    result = unsafe { fcntl(fd, action) };
                }
                1032 => {
                    result = unsafe { fcntl(fd, action) };
                }
                1034 => {
                    result = unsafe { fcntl(fd, action) };
                }
                11 => {
                    result = unsafe { fcntl(fd, action) };
                }
                1033 => {
                    result = unsafe { fcntl(fd, action) };
                }
                2 => {
                    let x: libc::c_int = unsafe { args.arg::<libc::c_int>() };
                    result = unsafe { fcntl(fd, action, x) };
                }
                4 => {
                    let x: libc::c_int = unsafe { args.arg::<libc::c_int>() };
                    result = unsafe { fcntl(fd, action, x) };
                }
                8 | 1031 => {
                    result = unsafe { fcntl(fd, action) };
                }
                1024 | 10 => {
                    let x: libc::c_int = unsafe { args.arg::<libc::c_int>() };
                    result = unsafe { fcntl(fd, action, x) };
                }
                _ => {
                    let p: *mut libc::c_void = unsafe { args.arg::<*mut libc::c_void>() };
                    result = unsafe { fcntl(fd, action, p) };
                }
            }
        }
    }
    result
}

fn rpl_fcntl_DUPFD(fd: i32, target: i32) -> i32 {
    unsafe {
        fcntl(fd, 0, target)
    }
}

static mut have_dupfd_cloexec: libc::c_int = 0;
unsafe extern "C" fn rpl_fcntl_DUPFD_CLOEXEC(
    mut fd: libc::c_int,
    mut target: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if 0 as libc::c_int <= have_dupfd_cloexec {
        result = fcntl(fd, 1030 as libc::c_int, target);
        if 0 as libc::c_int <= result || *__errno_location() != 22 as libc::c_int {
            have_dupfd_cloexec = 1 as libc::c_int;
        } else {
            let result = rpl_fcntl_DUPFD(fd, target);
            if result >= 0 as libc::c_int {
                have_dupfd_cloexec = -(1 as libc::c_int);
            }
        }
    } else {
        let result = rpl_fcntl_DUPFD(fd, target);
    }
    if 0 as libc::c_int <= result && have_dupfd_cloexec == -(1 as libc::c_int) {
        let mut flags: libc::c_int = fcntl(result, 1 as libc::c_int);
        if flags < 0 as libc::c_int
            || fcntl(result, 2 as libc::c_int, flags | 1 as libc::c_int)
                == -(1 as libc::c_int)
        {
            let mut saved_errno: libc::c_int = *__errno_location();
            close(result);
            *__errno_location() = saved_errno;
            result = -(1 as libc::c_int);
        }
    }
    return result;
}
unsafe extern "C" fn run_static_initializers() {
    have_dupfd_cloexec = if 0 != 0 {
        -(1)
    } else {
        0
    };
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
