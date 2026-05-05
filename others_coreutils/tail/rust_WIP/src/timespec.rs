

use std::time::Duration;
use std::time::SystemTime;

use std::time;

use ::libc;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn make_timespec(s: i64, ns: i64) -> std::time::SystemTime {
    let duration = std::time::Duration::new(s as u64, ns as u32);
    std::time::UNIX_EPOCH + duration
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn timespec_cmp(a: timespec, b: timespec) -> i32 {
    if a.tv_sec > b.tv_sec {
        1
    } else if a.tv_sec < b.tv_sec {
        -1
    } else {
        if a.tv_nsec > b.tv_nsec {
            1
        } else if a.tv_nsec < b.tv_nsec {
            -1
        } else {
            0
        }
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn timespec_sign(a: timespec) -> i32 {
    if a.tv_sec > 0 {
        1
    } else if a.tv_sec < 0 {
        -1
    } else {
        if a.tv_nsec != 0 {
            1
        } else {
            0
        }
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn timespectod(a: std::time::SystemTime) -> f64 {
    let duration = a.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1e9
}

