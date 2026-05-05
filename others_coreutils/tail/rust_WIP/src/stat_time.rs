





use std::fs::Metadata;

use libc::time_t;
use libc::c_long;

use std::os::unix::fs::MetadataExt;

use ::libc;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [libc::c_int; 2],
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn stat_time_normalize(result: i32, _st: &std::fs::Metadata) -> i32 {
    result
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn get_stat_birthtime(st: &libc::stat) -> libc::timespec {
    libc::timespec {
        tv_sec: -(1 as libc::c_int) as libc::time_t,
        tv_nsec: -(1 as libc::c_int) as libc::c_long,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn get_stat_mtime(st: &Metadata) -> libc::timespec {
    let mtime = st.mtime();
    libc::timespec {
        tv_sec: mtime as libc::time_t,
        tv_nsec: 0, // Assuming nanoseconds are not available, set to 0
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn get_stat_ctime(st: &Metadata) -> libc::timespec {
    let ctime = st.ctime();
    libc::timespec {
        tv_sec: ctime as libc::time_t,
        tv_nsec: 0, // Assuming nanoseconds are not needed, set to 0
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn get_stat_atime(st: &Metadata) -> libc::timespec {
    let atime = st.atime();
    libc::timespec {
        tv_sec: atime as libc::time_t,
        tv_nsec: 0, // Assuming nanoseconds are not needed, set to 0
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn get_stat_birthtime_ns(st: &Metadata) -> libc::c_long {
    st.ctime() as libc::c_long // Assuming ctime is the intended method for birthtime
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn get_stat_mtime_ns(st: &Metadata) -> libc::c_long {
    return st.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as libc::c_long;
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn get_stat_ctime_ns(st: &stat) -> libc::c_long {
    st.st_ctim.tv_nsec
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn get_stat_atime_ns(st: &Metadata) -> libc::c_long {
    return st.atime_nsec();
}

