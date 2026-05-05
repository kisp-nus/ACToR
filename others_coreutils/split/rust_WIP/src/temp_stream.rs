

use std::ffi;
use std::io;

use std::ffi::CStr;
use std::fs;

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fseeko(fp: *mut FILE, offset: off_t, whence: libc::c_int) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn mkstemp_safer(_: *mut libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn __errno_location() -> *mut libc::c_int;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn path_search(
        tmpl: *mut libc::c_char,
        tmpl_len: size_t,
        dir: *const libc::c_char,
        pfx: *const libc::c_char,
        try_tmpdir: bool,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type off_t = __off_t;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
fn record_or_unlink_tempfile(fn_0: &str) {
    std::fs::remove_file(fn_0).ok();
}

#[no_mangle]
pub unsafe extern "C" fn temp_stream(
    mut fp: *mut *mut FILE,
    mut file_name: *mut *mut libc::c_char,
) -> bool {
    use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::ptr;

static mut tempfile: *mut libc::c_char = ptr::null_mut();
static mut tmp_fp: *mut FILE = ptr::null_mut();

if tempfile.is_null() {
    's_103: {
         let mut tempbuf: Vec<i8> = vec![0; 128];
let mut tempbuf_len: usize = tempbuf.len();
loop {
    tempbuf.resize(tempbuf_len, 0);
    if tempbuf.is_empty() {
        error(
            0,
            *__errno_location(),
            gettext(b"failed to make temporary file name\0" as *const u8 as *const libc::c_char),
        );
        return false;
    }
    if path_search(
        tempbuf.as_mut_ptr(),
        tempbuf_len.try_into().unwrap(),
        std::ptr::null(),
        b"cutmp\0" as *const u8 as *const libc::c_char,
        true,
    ) == 0
    {
        break;
    }
    if *__errno_location() != 22 || (4096 / 2) < tempbuf_len {
        error(
            0,
            if *__errno_location() == 22 { 36 } else { *__errno_location() },
            gettext(b"failed to make temporary file name\0" as *const u8 as *const libc::c_char),
        );
        return false;
    }
    tempbuf_len *= 2;
}
let tempfile_ptr = tempbuf.as_mut_ptr();
let fd: libc::c_int = mkstemp_safer(tempfile_ptr);

            if fd < 0 {
    if false {
        error(
            0,
            std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
            gettext("failed to create temporary file %s\0".as_ptr() as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, tempfile),
        );
        unreachable!();
    } else {
        let __errstatus: i32 = 0;
        error(
            __errstatus,
            std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
            gettext("failed to create temporary file %s\0".as_ptr() as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, tempfile),
        );
        if __errstatus != 0 {
            unreachable!();
        }

        let __errstatus: i32 = 0;
        error(
            __errstatus,
            std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
            gettext("failed to create temporary file %s\0".as_ptr() as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, tempfile),
        );
        if __errstatus != 0 {
            unreachable!();
        }
    }
} else {
    let mode = if false { "w+b\0" } else { "w+\0" };
    let file_pointer = fdopen(fd, mode.as_ptr() as *const libc::c_char);
    if file_pointer.is_null() {
        if false {
            error(
                0,
                std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                gettext("failed to open %s for writing\0".as_ptr() as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, tempfile),
            );
            unreachable!();
        } else {
            let __errstatus: i32 = 0;
            error(
                __errstatus,
                std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                gettext("failed to open %s for writing\0".as_ptr() as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, tempfile),
            );
            if __errstatus != 0 {
                unreachable!();
            }

            let __errstatus: i32 = 0;
            error(
                __errstatus,
                std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                gettext("failed to open %s for writing\0".as_ptr() as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, tempfile),
            );
            if __errstatus != 0 {
                unreachable!();
            }
        }
        close(fd);
        unlink(tempfile);
    } else {
        record_or_unlink_tempfile(unsafe { std::ffi::CStr::from_ptr(tempfile).to_str().unwrap() });
        break 's_103;
    }
}
free(tempfile as *mut libc::c_void);
tempfile = std::ptr::null_mut();
return false;

        }
} else {
    clearerr_unlocked(tmp_fp);
        if rpl_fseeko(tmp_fp, 0 as libc::c_int as off_t, 0 as libc::c_int)
            < 0 as libc::c_int
            || ftruncate(fileno(tmp_fp), 0 as libc::c_int as __off_t) < 0 as libc::c_int
        {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to rewind stream for %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, tempfile),
                );
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to rewind stream for %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, tempfile),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to rewind stream for %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, tempfile),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            return 0 as libc::c_int != 0;
        }
    
}

*fp = tmp_fp;
if !file_name.is_null() {
    *file_name = tempfile;
}
return 1 != 0;

}
