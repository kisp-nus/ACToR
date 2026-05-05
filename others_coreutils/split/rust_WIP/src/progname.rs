use std::ffi::CStr;
use std::process;

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut program_invocation_name: *mut libc::c_char;
    static mut program_invocation_short_name: *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn abort() -> !;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
#[no_mangle]
pub static mut program_name: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub fn set_program_name(argv0: *const libc::c_char) {
    if argv0.is_null() {
        eprintln!("A NULL argv[0] was passed through an exec system call.");
        std::process::abort();
    }

    let c_str = unsafe { std::ffi::CStr::from_ptr(argv0) };
    let argv0_str = c_str.to_string_lossy();
    
    let slash = argv0_str.rfind('/').map(|index| &argv0_str[index + 1..]).unwrap_or(&argv0_str);
    
    if slash.len() >= 7 && &slash[slash.len() - 7..] == "/.libs/" {
        let base = &slash[1..];
        if base.starts_with("lt-") {
            unsafe {
                program_invocation_short_name = base[3..].as_ptr() as *mut libc::c_char;
            }
        }
        unsafe {
            program_name = base.as_ptr() as *const libc::c_char;
            program_invocation_name = base.as_ptr() as *mut libc::c_char;
        }
    } else {
        unsafe {
            program_name = argv0;
            program_invocation_name = argv0 as *mut libc::c_char;
        }
    }
}

