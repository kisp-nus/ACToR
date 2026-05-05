use std::mem;

use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
}
pub type __mode_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __syscall_slong_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __gid_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type nlink_t = __nlink_t;
#[no_mangle]
pub fn isapipe(fd: libc::c_int) -> libc::c_int {
    let mut pipe_link_count_max: nlink_t = 1;
    let mut check_for_fifo: bool = true;
    let mut st: stat = unsafe { std::mem::zeroed() }; // Using zeroed to initialize stat

    let fstat_result = unsafe { fstat(fd, &mut st) };
    if fstat_result != 0 {
        return fstat_result;
    }

    if !((true || true) && 1u32 != !0) 
        && (st.st_mode & 0o170000 == 0o10000 || st.st_mode & 0o170000 == 0o14000) {
        
        let mut fd_pair: [libc::c_int; 2] = [0; 2];
        let pipe_result = unsafe { pipe(fd_pair.as_mut_ptr()) };
        if pipe_result != 0 {
            return pipe_result;
        }

        let mut pipe_st: stat = unsafe { std::mem::zeroed() };
        let fstat_pipe_result = unsafe { fstat(fd_pair[0], &mut pipe_st) };
        let fstat_pipe_errno = unsafe { *__errno_location() };
        unsafe {
            close(fd_pair[0]);
            close(fd_pair[1]);
        }

        if fstat_pipe_result != 0 {
            unsafe {
                *__errno_location() = fstat_pipe_errno;
            }
            return fstat_pipe_result;
        }

        check_for_fifo = (pipe_st.st_mode & 0o170000 == 0o10000);
        pipe_link_count_max = pipe_st.st_nlink;
    }

    return (st.st_nlink <= pipe_link_count_max
        && if check_for_fifo {
            st.st_mode & 0o170000 == 0o10000
        } else {
            st.st_mode & 0o170000 == 0o14000
        }) as libc::c_int;
}

