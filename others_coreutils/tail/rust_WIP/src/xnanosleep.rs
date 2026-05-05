use std::mem;

use ::libc;
extern "C" {
    fn dtotimespec(_: libc::c_double) -> timespec;
    fn rpl_nanosleep(__rqtp: *const timespec, __rmtp: *mut timespec) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn pause() -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __syscall_slong_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
#[no_mangle]
pub fn xnanosleep(seconds: f64) -> i32 {
    if 1.0 + (if (0 as i32 as time_t) < -(1 as i32) as time_t {
        -(1 as i32) as f64
    } else {
        (((1 as i32 as time_t)
            << (std::mem::size_of::<time_t>() as usize * 8 - 2) - 1) * 2 + 1) as f64
    }) <= seconds {
        loop {
            unsafe { pause() };
            if unsafe { *__errno_location() } != 4 {
                break;
            }
        }
    }
    let mut ts_sleep: timespec;
    unsafe {
        ts_sleep = dtotimespec(seconds);
    }
    loop {
        unsafe { *__errno_location() = 0 };
        if unsafe { rpl_nanosleep(&ts_sleep as *const timespec, &mut ts_sleep) } == 0 {
            break;
        }
        if unsafe { *__errno_location() } != 4 && unsafe { *__errno_location() } != 0 {
            return -1;
        }
    }
    return 0;
}

