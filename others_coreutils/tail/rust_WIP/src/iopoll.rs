


use std::os::unix::io::AsRawFd;

use std::os::unix::io::RawFd;

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn isapipe(fd: libc::c_int) -> libc::c_int;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
fn iopoll_internal(
    fdin: i32,
    fdout: i32,
    block: bool,
    broken_output: bool,
) -> i32 {
    assert!(fdin != -1 || fdout != -1, "fdin != -1 || fdout != -1");

    let mut pfds: [libc::pollfd; 2] = [
        libc::pollfd {
            fd: fdin,
            events: (libc::POLLIN | libc::POLLOUT) as libc::c_short,
            revents: 0,
        },
        libc::pollfd {
            fd: fdout,
            events: libc::POLLOUT as libc::c_short,
            revents: 0,
        },
    ];

    let mut check_out_events = libc::POLLERR | libc::POLLHUP | libc::POLLNVAL;
    let mut ret: i32;

    if !broken_output {
        pfds[1].events = libc::POLLERR as libc::c_short;
        pfds[0].events = pfds[1].events;
        check_out_events = libc::POLLERR;
    }

    loop {
        ret = unsafe {
            libc::poll(
                pfds.as_mut_ptr(),
                pfds.len() as libc::c_ulong,
                if block { -1 } else { 0 },
            )
        };

        if ret < 0 {
            continue;
        }
        if ret == 0 && !block {
            return 0;
        }
        assert!(ret > 0, "0 < ret");

        if pfds[0].revents != 0 {
            return 0;
        }
        if (pfds[1].revents as i32) & (check_out_events as i32) != 0 {
            return if broken_output { -2 } else { 0 };
        }
    }
}

#[no_mangle]
pub fn iopoll(
    fdin: i32,
    fdout: i32,
    block: bool,
) -> i32 {
    iopoll_internal(fdin, fdout, block, true)
}

#[no_mangle]
pub fn iopoll_input_ok(fdin: libc::c_int) -> bool {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        __pad2: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 2],
    };

    // Use an unsafe block to call the unsafe function
    let always_ready = unsafe {
        fstat(fdin, &mut st) == 0 && 
        (st.st_mode & 0o170000 as libc::c_int as libc::c_uint == 0o100000 as libc::c_int as libc::c_uint ||
         st.st_mode & 0o170000 as libc::c_int as libc::c_uint == 0o60000 as libc::c_int as libc::c_uint)
    };

    !always_ready
}

#[no_mangle]
pub fn iopoll_output_ok(fdout: RawFd) -> bool {
    unsafe { isapipe(fdout) > 0 }
}

fn fwait_for_nonblocking_write(f: *mut FILE) -> bool {
    unsafe {
        if !(*__errno_location() == 11 || *__errno_location() == 11) {
            return false;
        }
        let fd: libc::c_int = fileno(f);
        if fd != -1 {
            if iopoll_internal(-1, fd, true, false) == 0 {
                clearerr_unlocked(f);
                return true;
            }
        }
        *__errno_location() = 11;
        return false;
    }
}

#[no_mangle]
pub unsafe extern "C" fn fclose_wait(mut f: *mut FILE) -> bool {
    while !(fflush_unlocked(f) == 0 as libc::c_int) {
        if !fwait_for_nonblocking_write(f) {
            break;
        }
    }
    return rpl_fclose(f) == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fwrite_wait(
    mut buf: *const libc::c_char,
    mut size: ssize_t,
    mut f: *mut FILE,
) -> bool {
    loop {
        let written: size_t = fwrite_unlocked(
            buf as *const libc::c_void,
            1 as libc::c_int as size_t,
            size as size_t,
            f,
        );
        size = (size as libc::c_ulong).wrapping_sub(written) as ssize_t as ssize_t;
        if size >= 0 as libc::c_int as libc::c_long {} else {
            __assert_fail(
                b"size >= 0\0" as *const u8 as *const libc::c_char,
                b"iopoll.c\0" as *const u8 as *const libc::c_char,
                230 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"_Bool fwrite_wait(const char *, ssize_t, FILE *)\0"))
                    .as_ptr(),
            );
        }
        'c_7276: {
            if size >= 0 as libc::c_int as libc::c_long {} else {
                __assert_fail(
                    b"size >= 0\0" as *const u8 as *const libc::c_char,
                    b"iopoll.c\0" as *const u8 as *const libc::c_char,
                    230 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 49],
                        &[libc::c_char; 49],
                    >(b"_Bool fwrite_wait(const char *, ssize_t, FILE *)\0"))
                        .as_ptr(),
                );
            }
        };
        if size <= 0 as libc::c_int as libc::c_long {
            return 1 as libc::c_int != 0;
        }
        if !fwait_for_nonblocking_write(f) {
            return 0 as libc::c_int != 0;
        }
        buf = buf.offset(written as isize);
    };
}
