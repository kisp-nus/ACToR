use std::ffi::CString;

use std::process;
use std::io;

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn _exit(_: libc::c_int) -> !;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn close_stream(stream: *mut FILE) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
}
pub type FILE = _IO_FILE;
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub const SANITIZE_ADDRESS: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
static mut file_name: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub fn close_stdout_set_file_name(file: &str) {
    let c_string = std::ffi::CString::new(file).expect("CString::new failed");
    unsafe {
        file_name = c_string.as_ptr();
    }
}

static mut ignore_EPIPE: bool = false;
#[no_mangle]
pub unsafe extern "C" fn close_stdout_set_ignore_EPIPE(mut ignore: bool) {
    ignore_EPIPE = ignore;
}
#[no_mangle]
pub fn close_stdout() {
    unsafe {
        if close_stream(stdout) != 0 
            && !(ignore_EPIPE && std::io::Error::last_os_error().kind() == std::io::ErrorKind::BrokenPipe)
        {
            let write_error = gettext(b"write error\0".as_ptr() as *const libc::c_char);
            if !file_name.is_null() {
                if false {
                    error(
                        0,
                        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                        b"%s: %s\0".as_ptr() as *const libc::c_char,
                        quotearg_colon(file_name),
                        write_error,
                    );
                    unreachable!();
                } else {
                    let __errstatus = 0;
                    error(
                        __errstatus,
                        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                        b"%s: %s\0".as_ptr() as *const libc::c_char,
                        quotearg_colon(file_name),
                        write_error,
                    );
                    if __errstatus != 0 {
                        unreachable!();
                    }
                    error(
                        __errstatus,
                        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                        b"%s: %s\0".as_ptr() as *const libc::c_char,
                        quotearg_colon(file_name),
                        write_error,
                    );
                    if __errstatus != 0 {
                        unreachable!();
                    }
                }
            } else {
                if false {
                    error(
                        0,
                        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                        b"%s\0".as_ptr() as *const libc::c_char,
                        write_error,
                    );
                    unreachable!();
                } else {
                    let __errstatus = 0;
                    error(
                        __errstatus,
                        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                        b"%s\0".as_ptr() as *const libc::c_char,
                        write_error,
                    );
                    if __errstatus != 0 {
                        unreachable!();
                    }
                    error(
                        __errstatus,
                        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                        b"%s\0".as_ptr() as *const libc::c_char,
                        write_error,
                    );
                    if __errstatus != 0 {
                        unreachable!();
                    }
                }
            }
            std::process::exit(exit_failure);
        }
        if SANITIZE_ADDRESS == 0 && close_stream(stderr) != 0 {
            std::process::exit(exit_failure);
        }
    }
}

