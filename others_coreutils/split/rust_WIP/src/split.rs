#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types, label_break_value)]
























use std::option::Option;

use std::ffi::CString;

use std::ptr;
use std::process;

use std::io::Read;

use std::ffi::CStr;
use std::io::Write;

use ::rust::*;
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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn fpurge(gl_stream: *mut FILE) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn getpagesize() -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    static mut Version: *const libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xinmalloc(n: idx_t, s: idx_t) -> *mut libc::c_void;
    fn xicalloc(n: idx_t, s: idx_t) -> *mut libc::c_void;
    fn xirealloc(p: *mut libc::c_void, s: idx_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn close_stdout();
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn proper_name_lite(
        name_ascii: *const libc::c_char,
        name_utf8: *const libc::c_char,
    ) -> *const libc::c_char;
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xalignalloc(_: idx_t, _: idx_t) -> *mut libc::c_void;
    fn fdadvise(fd: libc::c_int, offset: off_t, len: off_t, advice: fadvice_t);
    fn fd_reopen(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> size_t;
    fn quote_mem(arg: *const libc::c_char, argsize: size_t) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn sig2str(_: libc::c_int, _: *mut libc::c_char) -> libc::c_int;
    fn temp_stream(fp: *mut *mut FILE, file_name: *mut *mut libc::c_char) -> bool;
    fn xdectoimax(
        n_str: *const libc::c_char,
        min: intmax_t,
        max: intmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> intmax_t;
    fn xdectoumax(
        n_str: *const libc::c_char,
        min: uintmax_t,
        max: uintmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> uintmax_t;
    fn xstrtoimax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut intmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
}
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
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
pub type ssize_t = __ssize_t;
pub type ptrdiff_t = libc::c_long;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type idx_t = ptrdiff_t;
pub type C2RustUnnamed_0 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_0 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_0 = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
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
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IO_BUFSIZE: C2RustUnnamed_1 = 262144;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const SYS_BUFSIZE_MAX: C2RustUnnamed_2 = 2146435072;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type Split_type = libc::c_uint;
pub const type_rr: Split_type = 7;
pub const type_chunk_lines: Split_type = 6;
pub const type_chunk_bytes: Split_type = 5;
pub const type_digits: Split_type = 4;
pub const type_lines: Split_type = 3;
pub const type_byteslines: Split_type = 2;
pub const type_bytes: Split_type = 1;
pub const type_undef: Split_type = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const ADDITIONAL_SUFFIX_OPTION: C2RustUnnamed_3 = 259;
pub const IO_BLKSIZE_OPTION: C2RustUnnamed_3 = 258;
pub const FILTER_OPTION: C2RustUnnamed_3 = 257;
pub const VERBOSE_OPTION: C2RustUnnamed_3 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct of_info {
    pub of_name: *mut libc::c_char,
    pub ofd: libc::c_int,
    pub ofile: *mut FILE,
    pub opid: pid_t,
}
pub type of_t = of_info;
pub type C2RustUnnamed_4 = libc::c_int;
pub const OFD_APPEND: C2RustUnnamed_4 = -2;
pub const OFD_NEW: C2RustUnnamed_4 = -1;
#[inline]
fn to_uchar(ch: i8) -> u8 {
    ch as u8
}

#[inline]
fn psame_inode(a: &stat, b: &stat) -> bool {
    (*a).st_dev == (*b).st_dev && (*a).st_ino == (*b).st_ino
}

#[inline]
fn emit_stdin_note() {
    let message = "\nWith no FILE, or when FILE is -, read standard input.\n";
    let translated_message = unsafe { gettext(message.as_ptr() as *const libc::c_char) };
    let stdout_handle = std::io::stdout();
    let mut handle = stdout_handle.lock();
    let cstr = unsafe { CStr::from_ptr(translated_message) };
    if let Err(e) = handle.write_all(cstr.to_bytes()) {
        eprintln!("Error writing to stdout: {}", e);
    }
}

#[inline]
fn emit_mandatory_arg_note() {
    let message = "\nMandatory arguments to long options are mandatory for short options too.\n";
    let stdout_handle = std::io::stdout();
    let mut handle = stdout_handle.lock();
    handle.write_all(message.as_bytes()).expect("Failed to write to stdout");
}

#[inline]
fn emit_size_note() {
    let message = "\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y,R,Q (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n";
    let stdout_handle = std::io::stdout();
    let mut handle = stdout_handle.lock();
    let _ = handle.write_all(message.as_bytes());
}

#[inline]
fn emit_ancillary_info(program: &CStr) {
    let infomap_0: [(&CStr, &CStr); 7] = [
        (CStr::from_bytes_with_nul(b"[\0").unwrap(), CStr::from_bytes_with_nul(b"test invocation\0").unwrap()),
        (CStr::from_bytes_with_nul(b"coreutils\0").unwrap(), CStr::from_bytes_with_nul(b"Multi-call invocation\0").unwrap()),
        (CStr::from_bytes_with_nul(b"sha224sum\0").unwrap(), CStr::from_bytes_with_nul(b"sha2 utilities\0").unwrap()),
        (CStr::from_bytes_with_nul(b"sha256sum\0").unwrap(), CStr::from_bytes_with_nul(b"sha2 utilities\0").unwrap()),
        (CStr::from_bytes_with_nul(b"sha384sum\0").unwrap(), CStr::from_bytes_with_nul(b"sha2 utilities\0").unwrap()),
        (CStr::from_bytes_with_nul(b"sha512sum\0").unwrap(), CStr::from_bytes_with_nul(b"sha2 utilities\0").unwrap()),
        (CStr::from_bytes_with_nul(b"\0").unwrap(), CStr::from_bytes_with_nul(b"\0").unwrap()),
    ];
    
    let mut node = program.as_ptr();
    let mut map_prog = infomap_0.iter();

    while let Some(&(prog, ref n)) = map_prog.next() {
        if prog.as_ptr().is_null() || program.as_ptr() == prog.as_ptr() {
            node = n.as_ptr();
            break;
        }
    }

    let help_url = CStr::from_bytes_with_nul(b"https://www.gnu.org/software/coreutils/\0").unwrap();
    println!(
        "{} online help: <{}>",
        CStr::from_bytes_with_nul(b"GNU coreutils\0").unwrap().to_str().unwrap(),
        help_url.to_str().unwrap()
    );

    let lc_messages: *const libc::c_char;
    unsafe {
        lc_messages = setlocale(5, std::ptr::null());
    }
    
    if !lc_messages.is_null() && unsafe { CStr::from_ptr(lc_messages).to_str().unwrap().starts_with("en_") } {
        writeln!(
            std::io::stdout(),
            "{}",
            CStr::from_bytes_with_nul(b"Report any translation bugs to <https://translationproject.org/team/>\n\0").unwrap().to_str().unwrap()
        ).unwrap();
    }

    let url_program = if program.to_bytes() == b"[\0" {
        CStr::from_bytes_with_nul(b"test\0").unwrap()
    } else {
        program
    };

    println!(
        "Full documentation <{}{}>",
        help_url.to_str().unwrap(),
        url_program.to_str().unwrap()
    );

    println!(
        "or available locally via: info '(coreutils) {}{}'",
        unsafe { CStr::from_ptr(node).to_str().unwrap() },
        if node == program.as_ptr() {
            " invocation"
        } else {
            ""
        }
    );
}

#[inline]
fn usable_st_size(sb: *const stat) -> bool {
    let sb = unsafe { &*sb };
    (sb.st_mode & 0o170000) == 0o100000
        || (sb.st_mode & 0o170000) == 0o120000
        || (sb.st_mode.wrapping_sub(sb.st_mode) != 0) || (0 != 0)
}

#[inline]
fn write_error() {
    let saved_errno = std::io::Error::last_os_error();
    let _ = std::io::stdout().flush();
    let _ = std::io::stdout().lock().flush();
    
    let error_message = std::ffi::CString::new("write error").unwrap();
    
    if false {
        unsafe {
            error(
                1,
                saved_errno.raw_os_error().unwrap_or(0),
                error_message.as_ptr(),
            );
        }
        if true {
            unreachable!();
        }
    } else {
        {
            let __errstatus = 1;
            unsafe {
                error(
                    __errstatus,
                    saved_errno.raw_os_error().unwrap_or(0),
                    error_message.as_ptr(),
                );
            }
            if __errstatus != 0 {
                unreachable!();
            }
        }
        {
            let __errstatus = 1;
            unsafe {
                error(
                    __errstatus,
                    saved_errno.raw_os_error().unwrap_or(0),
                    error_message.as_ptr(),
                );
            }
            if __errstatus != 0 {
                unreachable!();
            }
        }
    }
}

#[inline]
fn __gl_stdbit_clzll(n: u64) -> i32 {
    if n != 0 {
        n.leading_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u64>() as u32) as i32
    }
}

#[inline]
unsafe extern "C" fn stdc_leading_zeros_ull(mut n: libc::c_ulonglong) -> libc::c_uint {
    let result: u32 = __gl_stdbit_clzll(n) as u32;
return result;
}
#[inline]
unsafe extern "C" fn io_blksize(mut st: *const stat) -> idx_t {
    let mut blocksize: idx_t = (if (if (0 as libc::c_int) < (*st).st_blksize
        && (*st).st_blksize as libc::c_ulong
            <= (-(1 as libc::c_int) as size_t)
                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        (*st).st_blksize
    } else {
        512 as libc::c_int
    }) <= 0 as libc::c_int
    {
        IO_BUFSIZE as libc::c_int
    } else if (0 as libc::c_int) < (*st).st_blksize
        && (*st).st_blksize as libc::c_ulong
            <= (-(1 as libc::c_int) as size_t)
                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        (*st).st_blksize
    } else {
        512 as libc::c_int
    }) as idx_t;
    blocksize
        += (IO_BUFSIZE as libc::c_int - 1 as libc::c_int) as libc::c_long
            - (IO_BUFSIZE as libc::c_int - 1 as libc::c_int) as libc::c_long % blocksize;
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        && blocksize & blocksize - 1 as libc::c_int as libc::c_long != 0
    {
        let mut leading_zeros: libc::c_int = stdc_leading_zeros_ull(
            blocksize as libc::c_ulonglong,
        ) as libc::c_int;
        if (9223372036854775807 as libc::c_long as libc::c_ulonglong)
            < (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_ulonglong) || leading_zeros != 0
        {
            let mut power: libc::c_ulonglong = (1 as libc::c_ulonglong)
                << 64 as libc::c_int - leading_zeros;
            if power <= 9223372036854775807 as libc::c_long as libc::c_ulonglong {
                blocksize = power as idx_t;
            }
        }
    }
    return (if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
        < 18446744073709551615 as libc::c_ulong
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        18446744073709551615 as libc::c_ulong
    })
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) < blocksize as libc::c_ulong
    {
        (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        })
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        blocksize as libc::c_ulong
    }) as idx_t;
}
#[inline]
fn __gl_setmode(fd: i32, mode: i32) -> i32 {
    return 0;
}

#[inline]
fn set_binary_mode(fd: i32, mode: i32) -> i32 {
    // Safety: We need to ensure that the file descriptor is valid and that the mode is appropriate.
    // This is a direct call to a function that may have undefined behavior if misused.
    unsafe { __gl_setmode(fd, mode) }
}

#[inline]
unsafe extern "C" fn xset_binary_mode_error() {}
#[inline]
fn xset_binary_mode(fd: i32, mode: i32) {
    if set_binary_mode(fd, mode) < 0 {
        unsafe {
            xset_binary_mode_error();
        }
    }
}

static mut filter_command: *const libc::c_char = 0 as *const libc::c_char;
static mut filter_pid: pid_t = 0;
static mut open_pipes: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut open_pipes_alloc: idx_t = 0;
static mut n_open_pipes: libc::c_int = 0;
static mut default_SIGPIPE: bool = false;
static mut outbase: *const libc::c_char = 0 as *const libc::c_char;
static mut outfile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut outfile_mid: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut suffix_auto: bool = 1 as libc::c_int != 0;
static mut suffix_length: idx_t = 0;
static mut suffix_alphabet: *const libc::c_char = b"abcdefghijklmnopqrstuvwxyz\0"
    as *const u8 as *const libc::c_char;
static mut numeric_suffix_start: *const libc::c_char = 0 as *const libc::c_char;
static mut additional_suffix: *const libc::c_char = 0 as *const libc::c_char;
static mut infile: *const libc::c_char = 0 as *const libc::c_char;
static mut in_stat_buf: stat = stat {
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
static mut output_desc: libc::c_int = -(1 as libc::c_int);
static mut verbose: bool = false;
static mut elide_empty_files: bool = false;
static mut unbuffered: bool = false;
static mut eolchar: libc::c_int = -(1 as libc::c_int);
static mut longopts: [option; 17] = [
    {
        let mut init = option {
            name: b"bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"lines\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"line-bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"number\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"elide-empty-files\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"unbuffered\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"suffix-length\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"additional-suffix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ADDITIONAL_SUFFIX_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"numeric-suffixes\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"hex-suffixes\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'x' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"filter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FILTER_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: VERBOSE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"separator\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"-io-blksize\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: IO_BLKSIZE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_HELP_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_VERSION_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[inline]
fn ignorable(err: i32) -> bool {
    let command_ref = unsafe { filter_command.as_ref() };
    err == 32 && command_ref.is_some()
}

unsafe extern "C" fn set_suffix_length(
    mut n_units: intmax_t,
    mut split_type: Split_type,
) {
    let mut suffix_length_needed: libc::c_int = 0 as libc::c_int;
    if !numeric_suffix_start.is_null() {
        suffix_auto = 0 as libc::c_int != 0;
    }
    if split_type as libc::c_uint == type_chunk_bytes as libc::c_int as libc::c_uint
        || split_type as libc::c_uint == type_chunk_lines as libc::c_int as libc::c_uint
        || split_type as libc::c_uint == type_rr as libc::c_int as libc::c_uint
    {
        let mut n_units_end: intmax_t = n_units - 1 as libc::c_int as libc::c_long;
        if !numeric_suffix_start.is_null() {
            let mut n_start: intmax_t = 0;
            let mut e: strtol_error = xstrtoimax(
                numeric_suffix_start,
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
                &mut n_start,
                b"\0" as *const u8 as *const libc::c_char,
            );
            if e as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
                && n_start < n_units
            {
                let (fresh0, fresh1) = n_units_end.overflowing_add(n_start);
                *(&mut n_units_end as *mut intmax_t) = fresh0;
                if fresh1 {
                    n_units_end = 9223372036854775807 as libc::c_long;
                }
            }
        }
        let mut alphabet_len: idx_t = strlen(suffix_alphabet) as idx_t;
        loop {
            suffix_length_needed += 1;
            suffix_length_needed;
            n_units_end /= alphabet_len;
            if !(n_units_end != 0) {
                break;
            }
        }
        suffix_auto = 0 as libc::c_int != 0;
    }
    if suffix_length != 0 {
        if suffix_length < suffix_length_needed as libc::c_long {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"the suffix length needs to be at least %d\0" as *const u8
                            as *const libc::c_char,
                    ),
                    suffix_length_needed,
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"the suffix length needs to be at least %d\0" as *const u8
                                as *const libc::c_char,
                        ),
                        suffix_length_needed,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"the suffix length needs to be at least %d\0" as *const u8
                                as *const libc::c_char,
                        ),
                        suffix_length_needed,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        suffix_auto = 0 as libc::c_int != 0;
        return;
    } else {
        suffix_length = (if 2 as libc::c_int > suffix_length_needed {
            2 as libc::c_int
        } else {
            suffix_length_needed
        }) as idx_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut status: libc::c_int) {
    if status != 0 as libc::c_int {
        fprintf(
            stderr,
            gettext(
                b"Try '%s --help' for more information.\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
    } else {
        printf(
            gettext(
                b"Usage: %s [OPTION]... [FILE [PREFIX]]\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Output pieces of FILE to PREFIXaa, PREFIXab, ...;\ndefault size is 1000 lines, and default PREFIX is 'x'.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fprintf(
            stdout,
            gettext(
                b"  -a, --suffix-length=N   generate suffixes of length N (default %d)\n      --additional-suffix=SUFFIX  append an additional SUFFIX to file names\n  -b, --bytes=SIZE        put SIZE bytes per output file\n  -C, --line-bytes=SIZE   put at most SIZE bytes of records per output file\n  -d                      use numeric suffixes starting at 0, not alphabetic\n      --numeric-suffixes[=FROM]  same as -d, but allow setting the start value\n  -x                      use hex suffixes starting at 0, not alphabetic\n      --hex-suffixes[=FROM]  same as -x, but allow setting the start value\n  -e, --elide-empty-files  do not generate empty output files with '-n'\n      --filter=COMMAND    write to shell COMMAND; file name is $FILE\n  -l, --lines=NUMBER      put NUMBER lines/records per output file\n  -n, --number=CHUNKS     generate CHUNKS output files; see explanation below\n  -t, --separator=SEP     use SEP instead of newline as the record separator;\n                            '\\0' (zero) specifies the NUL character\n  -u, --unbuffered        immediately copy input to output with '-n r/...'\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            2 as libc::c_int,
        );
        fputs_unlocked(
            gettext(
                b"      --verbose           print a diagnostic just before each\n                            output file is opened\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --help        display this help and exit\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --version     output version information and exit\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_size_note();
        fputs_unlocked(
            gettext(
                b"\nCHUNKS may be:\n  N       split into N files based on size of input\n  K/N     output Kth of N to stdout\n  l/N     split into N files without splitting lines/records\n  l/K/N   output Kth of N to stdout without splitting lines/records\n  r/N     like 'l' but use round robin distribution\n  r/K/N   likewise but only output Kth of N to stdout\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(CStr::from_bytes_with_nul(b"split\0").unwrap());
    }
    exit(status);
}
unsafe extern "C" fn copy_to_tmpfile(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: idx_t,
) -> off_t {
    let mut tmp: *mut FILE = 0 as *mut FILE;
    if !temp_stream(&mut tmp, 0 as *mut *mut libc::c_char) {
        return -(1 as libc::c_int) as off_t;
    }
    let mut copied: off_t = 0 as libc::c_int as off_t;
    let mut r: off_t = 0;
    loop {
        r = read(fd, buf as *mut libc::c_void, bufsize as size_t);
        if !((0 as libc::c_int as libc::c_long) < r) {
            break;
        }
        if fwrite_unlocked(
            buf as *const libc::c_void,
            1 as libc::c_int as size_t,
            r as size_t,
            tmp,
        ) != r as libc::c_ulong
        {
            return -(1 as libc::c_int) as off_t;
        }
        let (fresh2, fresh3) = copied.overflowing_add(r);
        *(&mut copied as *mut off_t) = fresh2;
        if fresh3 {
            *__errno_location() = 75 as libc::c_int;
            return -(1 as libc::c_int) as off_t;
        }
    }
    if r < 0 as libc::c_int as libc::c_long {
        return r;
    }
    r = dup2(fileno(tmp), fd) as off_t;
    if r < 0 as libc::c_int as libc::c_long {
        return r;
    }
    if rpl_fclose(tmp) < 0 as libc::c_int {
        return -(1 as libc::c_int) as off_t;
    }
    return copied;
}
unsafe extern "C" fn input_file_size(
    mut fd: libc::c_int,
    mut st: *const stat,
    mut buf: *mut libc::c_char,
    mut bufsize: idx_t,
) -> off_t {
    let mut size: off_t = 0 as libc::c_int as off_t;
    loop {
        let mut n_read: ssize_t = read(
            fd,
            buf.offset(size as isize) as *mut libc::c_void,
            (bufsize - size) as size_t,
        );
        if n_read <= 0 as libc::c_int as libc::c_long {
            return if n_read < 0 as libc::c_int as libc::c_long { n_read } else { size };
        }
        size += n_read;
        if !(size < bufsize) {
            break;
        }
    }
    let mut cur: off_t = 0;
    let mut end: off_t = 0;
    if usable_st_size(st) as libc::c_int != 0 && (*st).st_size < size
        || {
            cur = lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int);
            cur < 0 as libc::c_int as libc::c_long
        } || cur < size
        || {
            end = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
            end < 0 as libc::c_int as libc::c_long
        }
    {
        let mut tmpbuf: *mut libc::c_char = xmalloc(bufsize as size_t)
            as *mut libc::c_char;
        end = copy_to_tmpfile(fd, tmpbuf, bufsize);
        free(tmpbuf as *mut libc::c_void);
        if end < 0 as libc::c_int as libc::c_long {
            return end;
        }
        cur = 0 as libc::c_int as off_t;
    }
    if end
        == (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        })
        || cur < end
            && {
                let (fresh4, fresh5) = size.overflowing_add(end - cur);
                *(&mut size as *mut off_t) = fresh4;
                fresh5 as libc::c_int != 0
            }
    {
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int) as off_t;
    }
    if cur < end {
        let mut r: off_t = lseek(fd, cur, 0 as libc::c_int);
        if r < 0 as libc::c_int as libc::c_long {
            return r;
        }
    }
    return size;
}
unsafe extern "C" fn next_file_name() {
    static mut sufindex: *mut idx_t = 0 as *const idx_t as *mut idx_t;
    static mut outbase_length: idx_t = 0;
    static mut outfile_length: idx_t = 0;
    static mut addsuf_length: idx_t = 0;
    let mut overflow: bool = false;
    let mut widen: bool = false;
    let mut current_block_37: u64;
    if outfile.is_null() {
        overflow = false;
        widen = false;
        current_block_37 = 9769017674192345533;
    } else {
        let mut i_0: idx_t = suffix_length;
        loop {
            let fresh13 = i_0;
            i_0 = i_0 - 1;
            if !(fresh13 != 0 as libc::c_int as libc::c_long) {
                current_block_37 = 14434620278749266018;
                break;
            }
            let ref mut fresh14 = *sufindex.offset(i_0 as isize);
            *fresh14 += 1;
            let _ = *fresh14;
            if suffix_auto as libc::c_int != 0 && i_0 == 0 as libc::c_int as libc::c_long
                && *suffix_alphabet
                    .offset(
                        (*sufindex.offset(0 as libc::c_int as isize)
                            + 1 as libc::c_int as libc::c_long) as isize,
                    ) == 0
            {
                current_block_37 = 9769017674192345533;
                break;
            }
            *outfile_mid
                .offset(
                    i_0 as isize,
                ) = *suffix_alphabet.offset(*sufindex.offset(i_0 as isize) as isize);
            if *outfile_mid.offset(i_0 as isize) != 0 {
                return;
            }
            *sufindex.offset(i_0 as isize) = 0 as libc::c_int as idx_t;
            *outfile_mid
                .offset(
                    i_0 as isize,
                ) = *suffix_alphabet.offset(*sufindex.offset(i_0 as isize) as isize);
        }
        match current_block_37 {
            9769017674192345533 => {}
            _ => {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"output file suffixes exhausted\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if 1 as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"output file suffixes exhausted\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"output file suffixes exhausted\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
                current_block_37 = 2873832966593178012;
            }
        }
    }
    match current_block_37 {
        9769017674192345533 => {
            widen = outfile_length != 0;
            if !widen {
                outbase_length = strlen(outbase) as idx_t;
                addsuf_length = (if !additional_suffix.is_null() {
                    strlen(additional_suffix)
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as idx_t;
                let (fresh6, fresh7) = (outbase_length + addsuf_length)
                    .overflowing_add(suffix_length);
                *(&mut outfile_length as *mut idx_t) = fresh6;
                overflow = fresh7;
            } else {
                let (fresh8, fresh9) = outfile_length.overflowing_add((2 as libc::c_int).into());
                *(&mut outfile_length as *mut idx_t) = fresh8;
                overflow = fresh9;
                suffix_length += 1;
                suffix_length;
            }
            let mut outfile_size: idx_t = 0;
            let (fresh10, fresh11) = outfile_length.overflowing_add((1 as libc::c_int).into());
            *(&mut outfile_size as *mut idx_t) = fresh10;
            overflow = (overflow as libc::c_int | fresh11 as libc::c_int) != 0;
            if overflow {
                xalloc_die();
            }
            outfile = xirealloc(outfile as *mut libc::c_void, outfile_size)
                as *mut libc::c_char;
            if !widen {
                memcpy(
                    outfile as *mut libc::c_void,
                    outbase as *const libc::c_void,
                    outbase_length as libc::c_ulong,
                );
            } else {
                *outfile
                    .offset(
                        outbase_length as isize,
                    ) = *suffix_alphabet
                    .offset(*sufindex.offset(0 as libc::c_int as isize) as isize);
                outbase_length += 1;
                outbase_length;
            }
            outfile_mid = outfile.offset(outbase_length as isize);
            memset(
                outfile_mid as *mut libc::c_void,
                *suffix_alphabet.offset(0 as libc::c_int as isize) as libc::c_int,
                suffix_length as libc::c_ulong,
            );
            if !additional_suffix.is_null() {
                memcpy(
                    outfile_mid.offset(suffix_length as isize) as *mut libc::c_void,
                    additional_suffix as *const libc::c_void,
                    addsuf_length as libc::c_ulong,
                );
            }
            *outfile.offset(outfile_length as isize) = 0 as libc::c_int as libc::c_char;
            free(sufindex as *mut libc::c_void);
            sufindex = xicalloc(
                suffix_length,
                ::core::mem::size_of::<idx_t>() as libc::c_ulong as idx_t,
            ) as *mut idx_t;
            if !numeric_suffix_start.is_null() {
                if !widen {} else {
                    __assert_fail(
                        b"! widen\0" as *const u8 as *const libc::c_char,
                        b"split.c\0" as *const u8 as *const libc::c_char,
                        423 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 26],
                            &[libc::c_char; 26],
                        >(b"void next_file_name(void)\0"))
                            .as_ptr(),
                    );
                }
                'c_9037: {
                    if !widen {} else {
                        __assert_fail(
                            b"! widen\0" as *const u8 as *const libc::c_char,
                            b"split.c\0" as *const u8 as *const libc::c_char,
                            423 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 26],
                                &[libc::c_char; 26],
                            >(b"void next_file_name(void)\0"))
                                .as_ptr(),
                        );
                    }
                };
                let mut i: idx_t = strlen(numeric_suffix_start) as idx_t;
                memcpy(
                    outfile_mid.offset(suffix_length as isize).offset(-(i as isize))
                        as *mut libc::c_void,
                    numeric_suffix_start as *const libc::c_void,
                    i as libc::c_ulong,
                );
                let mut sufindex_end: *mut idx_t = sufindex
                    .offset(suffix_length as isize);
                loop {
                    let fresh12 = i;
                    i = i - 1;
                    if !(fresh12 != 0 as libc::c_int as libc::c_long) {
                        break;
                    }
                    sufindex_end = sufindex_end.offset(-1);
                    *sufindex_end = (*numeric_suffix_start.offset(i as isize)
                        as libc::c_int - '0' as i32) as idx_t;
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn create(mut name: *const libc::c_char) -> libc::c_int {
    if filter_command.is_null() {
        if verbose {
            fprintf(
                stdout,
                gettext(b"creating file %s\n\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, name),
            );
        }
        let mut oflags: libc::c_int = 0o1 as libc::c_int | 0o100 as libc::c_int
            | 0 as libc::c_int;
        let mut fd: libc::c_int = open_safer(
            name,
            oflags | 0o200 as libc::c_int,
            0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        if 0 as libc::c_int <= fd || *__errno_location() != 17 as libc::c_int {
            return fd;
        }
        fd = open_safer(
            name,
            oflags,
            0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        if fd < 0 as libc::c_int {
            return fd;
        }
        let mut out_stat_buf: stat = stat {
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
        if fstat(fd, &mut out_stat_buf) != 0 as libc::c_int {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"failed to stat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, name),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to stat %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, name),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to stat %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, name),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        if psame_inode(&mut in_stat_buf, &mut out_stat_buf) {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s would overwrite input; aborting\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, name),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"%s would overwrite input; aborting\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, name),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"%s would overwrite input; aborting\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, name),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        if ftruncate(fd, 0 as libc::c_int as __off_t) < 0 as libc::c_int
            && (out_stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
                || (out_stat_buf.st_mode).wrapping_sub(out_stat_buf.st_mode) != 0)
        {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"%s: error truncating\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        name,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"%s: error truncating\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            name,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"%s: error truncating\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            name,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        return fd;
    } else {
        let mut fd_pair: [libc::c_int; 2] = [0; 2];
        let mut child_pid: pid_t = 0;
        let mut shell_prog: *const libc::c_char = getenv(
            b"SHELL\0" as *const u8 as *const libc::c_char,
        );
        if shell_prog.is_null() {
            shell_prog = b"/bin/sh\0" as *const u8 as *const libc::c_char;
        }
        if setenv(b"FILE\0" as *const u8 as *const libc::c_char, name, 1 as libc::c_int)
            != 0 as libc::c_int
        {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to set FILE environment variable\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to set FILE environment variable\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to set FILE environment variable\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        if verbose {
            fprintf(
                stdout,
                gettext(
                    b"executing with FILE=%s\n\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    name,
                ),
            );
        }
        if pipe(fd_pair.as_mut_ptr()) != 0 as libc::c_int {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to create pipe\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to create pipe\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to create pipe\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        child_pid = fork();
        if child_pid == 0 as libc::c_int {
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while j < n_open_pipes {
                if close(*open_pipes.offset(j as isize)) != 0 as libc::c_int {
                    if 0 != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"closing prior pipe\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                        if 1 as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = 1 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                gettext(
                                    b"closing prior pipe\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = 1 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                gettext(
                                    b"closing prior pipe\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
                j += 1;
                j;
            }
            if close(fd_pair[1 as libc::c_int as usize]) != 0 {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"closing output pipe\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    if 1 as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            gettext(
                                b"closing output pipe\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            gettext(
                                b"closing output pipe\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            if fd_pair[0 as libc::c_int as usize] != 0 as libc::c_int {
                if dup2(fd_pair[0 as libc::c_int as usize], 0 as libc::c_int)
                    != 0 as libc::c_int
                {
                    if 0 != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"moving input pipe\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                        if 1 as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = 1 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                gettext(
                                    b"moving input pipe\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = 1 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                gettext(
                                    b"moving input pipe\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
                if close(fd_pair[0 as libc::c_int as usize]) != 0 as libc::c_int {
                    if 0 != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"closing input pipe\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                        if 1 as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = 1 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                gettext(
                                    b"closing input pipe\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = 1 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                gettext(
                                    b"closing input pipe\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
            }
            if default_SIGPIPE {
                signal(13 as libc::c_int, None);
            }
            execl(
                shell_prog,
                last_component(shell_prog),
                b"-c\0" as *const u8 as *const libc::c_char,
                filter_command,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to run command: \"%s -c %s\"\0" as *const u8
                            as *const libc::c_char,
                    ),
                    shell_prog,
                    filter_command,
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to run command: \"%s -c %s\"\0" as *const u8
                                as *const libc::c_char,
                        ),
                        shell_prog,
                        filter_command,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to run command: \"%s -c %s\"\0" as *const u8
                                as *const libc::c_char,
                        ),
                        shell_prog,
                        filter_command,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        if child_pid < 0 as libc::c_int {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"fork system call failed\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"fork system call failed\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"fork system call failed\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        if close(fd_pair[0 as libc::c_int as usize]) != 0 as libc::c_int {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to close input pipe\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to close input pipe\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"failed to close input pipe\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        filter_pid = child_pid;
        if n_open_pipes as libc::c_long == open_pipes_alloc {
            open_pipes = xpalloc(
                open_pipes as *mut libc::c_void,
                &mut open_pipes_alloc,
                1 as libc::c_int as idx_t,
                if (2147483647 as libc::c_int as libc::c_long)
                    < 9223372036854775807 as libc::c_long
                {
                    2147483647 as libc::c_int as libc::c_long
                } else {
                    9223372036854775807 as libc::c_long
                },
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as idx_t,
            ) as *mut libc::c_int;
        }
        let fresh15 = n_open_pipes;
        n_open_pipes = n_open_pipes + 1;
        *open_pipes.offset(fresh15 as isize) = fd_pair[1 as libc::c_int as usize];
        return fd_pair[1 as libc::c_int as usize];
    };
}
unsafe extern "C" fn closeout(
    mut fp: *mut FILE,
    mut fd: libc::c_int,
    mut pid: pid_t,
    mut name: *const libc::c_char,
) {
    if !fp.is_null() && rpl_fclose(fp) != 0 as libc::c_int
        && !ignorable(*__errno_location())
    {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    name,
                ),
            );
            if 1 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        name,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        name,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    if fd >= 0 as libc::c_int {
        if fp.is_null() && close(fd) < 0 as libc::c_int {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        name,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            name,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            name,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < n_open_pipes {
            if *open_pipes.offset(j as isize) == fd {
                n_open_pipes -= 1;
                *open_pipes
                    .offset(j as isize) = *open_pipes.offset(n_open_pipes as isize);
                break;
            } else {
                j += 1;
                j;
            }
        }
    }
    if pid > 0 as libc::c_int {
        let mut wstatus: libc::c_int = 0;
        if waitpid(pid, &mut wstatus, 0 as libc::c_int) < 0 as libc::c_int {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"waiting for child process\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"waiting for child process\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"waiting for child process\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        } else if ((wstatus & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
            as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
            let mut sig: libc::c_int = wstatus & 0x7f as libc::c_int;
            if sig != 13 as libc::c_int {
                let mut signame: [libc::c_char; 19] = [0; 19];
                if sig2str(sig, signame.as_mut_ptr()) != 0 as libc::c_int {
                    sprintf(
                        signame.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        sig,
                    );
                }
                if 0 != 0 {
                    error(
                        sig + 128 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"with FILE=%s, signal %s from command: %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            name,
                        ),
                        signame.as_mut_ptr(),
                        filter_command,
                    );
                    if sig + 128 as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = sig + 128 as libc::c_int;
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"with FILE=%s, signal %s from command: %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                name,
                            ),
                            signame.as_mut_ptr(),
                            filter_command,
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = sig + 128 as libc::c_int;
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"with FILE=%s, signal %s from command: %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                name,
                            ),
                            signame.as_mut_ptr(),
                            filter_command,
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
        } else if wstatus & 0x7f as libc::c_int == 0 as libc::c_int {
            let mut ex: libc::c_int = (wstatus & 0xff00 as libc::c_int)
                >> 8 as libc::c_int;
            if ex != 0 as libc::c_int {
                if 0 != 0 {
                    error(
                        ex,
                        0 as libc::c_int,
                        gettext(
                            b"with FILE=%s, exit %d from command: %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            name,
                        ),
                        ex,
                        filter_command,
                    );
                    if ex != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = ex;
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"with FILE=%s, exit %d from command: %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                name,
                            ),
                            ex,
                            filter_command,
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = ex;
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"with FILE=%s, exit %d from command: %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                name,
                            ),
                            ex,
                            filter_command,
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
        } else {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"unknown status from command (0x%X)\0" as *const u8
                            as *const libc::c_char,
                    ),
                    (wstatus as libc::c_uint).wrapping_add(0 as libc::c_uint),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"unknown status from command (0x%X)\0" as *const u8
                                as *const libc::c_char,
                        ),
                        (wstatus as libc::c_uint).wrapping_add(0 as libc::c_uint),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"unknown status from command (0x%X)\0" as *const u8
                                as *const libc::c_char,
                        ),
                        (wstatus as libc::c_uint).wrapping_add(0 as libc::c_uint),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    }
}
unsafe extern "C" fn cwrite(
    mut new_file_flag: bool,
    mut bp: *const libc::c_char,
    mut bytes: idx_t,
) -> bool {
    if new_file_flag {
        if bp.is_null() && bytes == 0 as libc::c_int as libc::c_long
            && elide_empty_files as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
        closeout(0 as *mut FILE, output_desc, filter_pid, outfile);
        next_file_name();
        output_desc = create(outfile);
        if output_desc < 0 as libc::c_int {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        outfile,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            outfile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            outfile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    }
    if full_write(output_desc, bp as *const libc::c_void, bytes as size_t)
        == bytes as libc::c_ulong
    {
        return 1 as libc::c_int != 0
    } else {
        if !ignorable(*__errno_location()) {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        outfile,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            outfile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            outfile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        return 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn bytes_split(
    mut n_bytes: intmax_t,
    mut rem_bytes: intmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: idx_t,
    mut initial_read: ssize_t,
    mut max_files: intmax_t,
) {
    let mut new_file_flag: bool = 1 as libc::c_int != 0;
    let mut filter_ok: bool = 1 as libc::c_int != 0;
    let mut opened: intmax_t = 0 as libc::c_int as intmax_t;
    let mut to_write: intmax_t = n_bytes
        + ((0 as libc::c_int as libc::c_long) < rem_bytes) as libc::c_int
            as libc::c_long;
    let mut eof: bool = to_write == 0;
    while !eof {
        let mut n_read: ssize_t = 0;
        if 0 as libc::c_int as libc::c_long <= initial_read {
            n_read = initial_read;
            initial_read = -(1 as libc::c_int) as ssize_t;
            eof = n_read < bufsize;
        } else {
            if !filter_ok
                && 0 as libc::c_int as libc::c_long
                    <= lseek(0 as libc::c_int, to_write, 1 as libc::c_int)
            {
                to_write = n_bytes
                    + ((opened + 1 as libc::c_int as libc::c_long) < rem_bytes)
                        as libc::c_int as libc::c_long;
                new_file_flag = 1 as libc::c_int != 0;
            }
            n_read = read(0 as libc::c_int, buf as *mut libc::c_void, bufsize as size_t);
            if n_read < 0 as libc::c_int as libc::c_long {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if 1 as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                infile,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                infile,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            eof = n_read == 0 as libc::c_int as libc::c_long;
        }
        let mut bp_out: *mut libc::c_char = buf;
        while (0 as libc::c_int as libc::c_long) < to_write && to_write <= n_read {
            if filter_ok as libc::c_int != 0 || new_file_flag as libc::c_int != 0 {
                filter_ok = cwrite(new_file_flag, bp_out, to_write);
            }
            opened += new_file_flag as libc::c_long;
            new_file_flag = max_files == 0 || opened < max_files;
            if !filter_ok && !new_file_flag {
                n_read = 0 as libc::c_int as ssize_t;
                eof = 1 as libc::c_int != 0;
                break;
            } else {
                bp_out = bp_out.offset(to_write as isize);
                n_read -= to_write;
                to_write = n_bytes + (opened < rem_bytes) as libc::c_int as libc::c_long;
            }
        }
        if !((0 as libc::c_int as libc::c_long) < n_read) {
            continue;
        }
        if filter_ok as libc::c_int != 0 || new_file_flag as libc::c_int != 0 {
            filter_ok = cwrite(new_file_flag, bp_out, n_read);
        }
        opened += new_file_flag as libc::c_long;
        new_file_flag = 0 as libc::c_int != 0;
        if !filter_ok && opened == max_files {
            break;
        }
        to_write -= n_read;
    }
    loop {
        let fresh16 = opened;
        opened = opened + 1;
        if !(fresh16 < max_files) {
            break;
        }
        cwrite(
            1 as libc::c_int != 0,
            0 as *const libc::c_char,
            0 as libc::c_int as idx_t,
        );
    };
}
unsafe extern "C" fn lines_split(
    mut n_lines: intmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: idx_t,
) {
    let mut n_read: ssize_t = 0;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bp_out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eob: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_file_flag: bool = 1 as libc::c_int != 0;
    let mut n: intmax_t = 0 as libc::c_int as intmax_t;
    loop {
        n_read = read(0 as libc::c_int, buf as *mut libc::c_void, bufsize as size_t);
        if n_read < 0 as libc::c_int as libc::c_long {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        bp_out = buf;
        bp = bp_out;
        eob = bp.offset(n_read as isize);
        *eob = eolchar as libc::c_char;
        loop {
            bp = rawmemchr(bp as *const libc::c_void, eolchar) as *mut libc::c_char;
            if bp == eob {
                if eob != bp_out {
                    let mut len: idx_t = eob.offset_from(bp_out) as libc::c_long;
                    cwrite(new_file_flag, bp_out, len);
                    new_file_flag = 0 as libc::c_int != 0;
                }
                break;
            } else {
                bp = bp.offset(1);
                bp;
                n += 1;
                if n >= n_lines {
                    cwrite(
                        new_file_flag,
                        bp_out,
                        bp.offset_from(bp_out) as libc::c_long,
                    );
                    bp_out = bp;
                    new_file_flag = 1 as libc::c_int != 0;
                    n = 0 as libc::c_int as intmax_t;
                }
            }
        }
        if !(n_read != 0) {
            break;
        }
    };
}
unsafe extern "C" fn line_bytes_split(
    mut n_bytes: intmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: idx_t,
) {
    let mut n_read: ssize_t = 0;
    let mut n_out: intmax_t = 0 as libc::c_int as intmax_t;
    let mut n_hold: idx_t = 0 as libc::c_int as idx_t;
    let mut hold: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hold_size: idx_t = 0 as libc::c_int as idx_t;
    let mut split_line: bool = 0 as libc::c_int != 0;
    loop {
        n_read = read(0 as libc::c_int, buf as *mut libc::c_void, bufsize as size_t);
        if n_read < 0 as libc::c_int as libc::c_long {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        let mut n_left: idx_t = n_read;
        let mut sob: *mut libc::c_char = buf;
        while n_left != 0 {
            let mut split_rest: idx_t = 0 as libc::c_int as idx_t;
            let mut eoc: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut eol: *mut libc::c_char = 0 as *mut libc::c_char;
            if n_bytes - n_out - n_hold <= n_left {
                split_rest = n_bytes - n_out - n_hold;
                eoc = sob
                    .offset(split_rest as isize)
                    .offset(-(1 as libc::c_int as isize));
                eol = memrchr(sob as *const libc::c_void, eolchar, split_rest as size_t)
                    as *mut libc::c_char;
            } else {
                eol = memrchr(sob as *const libc::c_void, eolchar, n_left as size_t)
                    as *mut libc::c_char;
            }
            if n_hold != 0 && !(eol.is_null() && n_out != 0) {
                cwrite(n_out == 0 as libc::c_int as libc::c_long, hold, n_hold);
                n_out += n_hold;
                n_hold = 0 as libc::c_int as idx_t;
            }
            if !eol.is_null() {
                split_line = 1 as libc::c_int != 0;
                let mut n_write: idx_t = eol.offset_from(sob) as libc::c_long
                    + 1 as libc::c_int as libc::c_long;
                cwrite(n_out == 0 as libc::c_int as libc::c_long, sob, n_write);
                n_out += n_write;
                n_left -= n_write;
                sob = sob.offset(n_write as isize);
                if !eoc.is_null() {
                    split_rest -= n_write;
                }
            }
            if n_left != 0 && !split_line {
                let mut n_write_0: idx_t = if !eoc.is_null() {
                    split_rest
                } else {
                    n_left
                };
                cwrite(n_out == 0 as libc::c_int as libc::c_long, sob, n_write_0);
                n_out += n_write_0;
                n_left -= n_write_0;
                sob = sob.offset(n_write_0 as isize);
                if !eoc.is_null() {
                    split_rest -= n_write_0;
                }
            }
            if !eoc.is_null() && split_rest != 0 || eoc.is_null() && n_left != 0 {
                let mut n_buf: idx_t = if !eoc.is_null() { split_rest } else { n_left };
                if hold_size - n_hold < n_buf {
                    hold = xpalloc(
                        hold as *mut libc::c_void,
                        &mut hold_size,
                        n_buf - (hold_size - n_hold),
                        -(1 as libc::c_int) as ptrdiff_t,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong as idx_t,
                    ) as *mut libc::c_char;
                }
                memcpy(
                    hold.offset(n_hold as isize) as *mut libc::c_void,
                    sob as *const libc::c_void,
                    n_buf as libc::c_ulong,
                );
                n_hold += n_buf;
                n_left -= n_buf;
                sob = sob.offset(n_buf as isize);
            }
            if !eoc.is_null() {
                n_out = 0 as libc::c_int as intmax_t;
                split_line = 0 as libc::c_int != 0;
            }
        }
        if !(n_read != 0) {
            break;
        }
    }
    if n_hold != 0 {
        cwrite(n_out == 0 as libc::c_int as libc::c_long, hold, n_hold);
    }
    free(hold as *mut libc::c_void);
}
unsafe extern "C" fn lines_chunk_split(
    mut k: intmax_t,
    mut n: intmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: idx_t,
    mut initial_read: ssize_t,
    mut file_size: off_t,
) {
    if n != 0 && k <= n {
    // Do nothing, condition is satisfied
} else {
    panic!("n && k <= n");
}

let rem_bytes: i64 = file_size % n;
let chunk_size: i64 = file_size / n;
let mut chunk_no: i64 = 1;
let mut chunk_end: i64 = chunk_size + if rem_bytes > 0 { 1 } else { 0 };
let mut n_written: i64 = 0;
let mut new_file_flag: bool = true;
let mut chunk_truncated: bool = false;

if k > 1 && file_size > 0 {
    let start: i64 = (k - 1) * chunk_size + if (k - 1) < rem_bytes { k - 1 } else { rem_bytes } - 1;

    if start < initial_read {
        let buf_slice = unsafe { std::slice::from_raw_parts_mut(buf, bufsize as usize) };
        buf_slice.copy_within(start as usize..initial_read as usize, 0);
        initial_read -= start;
    } else {
        if initial_read < start {
            let seek_result = lseek(0, start - initial_read, 1);
            if seek_result < 0 {
                let err = unsafe { *libc::__errno_location() };
                error(1, err, b"Error seeking file: %s\0".as_ptr() as *const i8, quotearg_n_style_colon(0, shell_escape_quoting_style, infile));
                unreachable!();
            } else {
                initial_read = -1;
            }
        }
        n_written = start;
        chunk_no = k - 1;
        chunk_end = start + 1;
    }
}

while n_written < file_size {
     let mut bp: *mut libc::c_char = buf;
        let mut eob: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut n_read: ssize_t = 0;
        if 0 as libc::c_int as libc::c_long <= initial_read {
            n_read = initial_read;
            initial_read = -(1 as libc::c_int) as ssize_t;
        } else {
            n_read = read(
                0 as libc::c_int,
                buf as *mut libc::c_void,
                (if bufsize < file_size - n_written {
                    bufsize
                } else {
                    file_size - n_written
                }) as size_t,
            );
            if n_read < 0 as libc::c_int as libc::c_long {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if 1 as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                infile,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                infile,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
        }
        if n_read == 0 as libc::c_int as libc::c_long {
            break;
        }
        chunk_truncated = 0 as libc::c_int != 0;
        eob = buf.offset(n_read as isize);
        while bp != eob {
            let mut to_write: idx_t = 0;
            let mut next: bool = 0 as libc::c_int != 0;
            let mut skip: off_t = if n_read
                < (if 0 as libc::c_int as libc::c_long
                    > chunk_end - 1 as libc::c_int as libc::c_long - n_written
                {
                    0 as libc::c_int as libc::c_long
                } else {
                    chunk_end - 1 as libc::c_int as libc::c_long - n_written
                })
            {
                n_read
            } else if 0 as libc::c_int as libc::c_long
                > chunk_end - 1 as libc::c_int as libc::c_long - n_written
            {
                0 as libc::c_int as libc::c_long
            } else {
                chunk_end - 1 as libc::c_int as libc::c_long - n_written
            };
            let mut bp_out: *mut libc::c_char = memchr(
                bp.offset(skip as isize) as *const libc::c_void,
                eolchar,
                (n_read - skip) as libc::c_ulong,
            ) as *mut libc::c_char;
            if !bp_out.is_null() {
                bp_out = bp_out.offset(1);
                bp_out;
                next = 1 as libc::c_int != 0;
            } else {
                bp_out = eob;
            }
            to_write = bp_out.offset_from(bp) as libc::c_long;
            if k == chunk_no {
                if full_write(
                    1 as libc::c_int,
                    bp as *const libc::c_void,
                    to_write as size_t,
                ) != to_write as libc::c_ulong
                {
                    write_error();
                }
            } else if k == 0 {
                cwrite(new_file_flag, bp, to_write);
            }
            n_written += to_write;
            bp = bp.offset(to_write as isize);
            n_read -= to_write;
            new_file_flag = next;
            while next as libc::c_int != 0 || chunk_end <= n_written {
                if !next && bp == eob {
                    chunk_truncated = 1 as libc::c_int != 0;
                    break;
                } else {
                    if k == chunk_no {
                        return;
                    }
                    chunk_end
                        += chunk_size
                            + (chunk_no < rem_bytes) as libc::c_int as libc::c_long;
                    chunk_no += 1;
                    chunk_no;
                    if chunk_end <= n_written {
                        if k == 0 {
                            cwrite(
                                1 as libc::c_int != 0,
                                0 as *const libc::c_char,
                                0 as libc::c_int as idx_t,
                            );
                        }
                    } else {
                        next = 0 as libc::c_int != 0;
                    }
                }
            }
        }

}

if chunk_truncated {
    chunk_no += 1;
}

if k == 0 {
    loop {
        let fresh17 = chunk_no;
        chunk_no += 1;
        if fresh17 > n {
            break;
        }
        cwrite(new_file_flag as i32 != 0, 0 as *const i8, 0);
    }
}

}
unsafe extern "C" fn bytes_chunk_extract(
    mut k: intmax_t,
    mut n: intmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: idx_t,
    mut initial_read: ssize_t,
    mut file_size: off_t,
) {
    let mut start: off_t = 0;
    let mut end: off_t = 0;
    if (0 as libc::c_int as libc::c_long) < k && k <= n {} else {
        __assert_fail(
            b"0 < k && k <= n\0" as *const u8 as *const libc::c_char,
            b"split.c\0" as *const u8 as *const libc::c_char,
            1009 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void bytes_chunk_extract(intmax_t, intmax_t, char *, idx_t, ssize_t, off_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_14132: {
        if (0 as libc::c_int as libc::c_long) < k && k <= n {} else {
            __assert_fail(
                b"0 < k && k <= n\0" as *const u8 as *const libc::c_char,
                b"split.c\0" as *const u8 as *const libc::c_char,
                1009 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void bytes_chunk_extract(intmax_t, intmax_t, char *, idx_t, ssize_t, off_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    start = (k - 1 as libc::c_int as libc::c_long) * (file_size / n)
        + (if (k - 1 as libc::c_int as libc::c_long) < file_size % n {
            k - 1 as libc::c_int as libc::c_long
        } else {
            file_size % n
        });
    end = if k == n {
        file_size
    } else {
        k * (file_size / n) + (if k < file_size % n { k } else { file_size % n })
    };
    if start < initial_read {
        memmove(
            buf as *mut libc::c_void,
            buf.offset(start as isize) as *const libc::c_void,
            (initial_read - start) as libc::c_ulong,
        );
        initial_read -= start;
    } else {
        if initial_read < start
            && lseek(0 as libc::c_int, start - initial_read, 1 as libc::c_int)
                < 0 as libc::c_int as libc::c_long
        {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        initial_read = -(1 as libc::c_int) as ssize_t;
    }
    while start < end {
    let mut n_read: i64 = 0;
    if initial_read >= 0 {
        n_read = initial_read;
        initial_read = -1;
    } else {
        let buffer = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, bufsize as usize) };
        n_read = match std::io::stdin().read(&mut buffer[start as usize..end as usize]) {
            Ok(bytes_read) => bytes_read as i64,
            Err(e) => {
                error(
                    1,
                    e.raw_os_error().unwrap_or(0),
                    b"%s\0" as *const u8 as *const i8,
                    quotearg_n_style_colon(0, shell_escape_quoting_style, infile),
                );
                return; // Handle error appropriately
            }
        };
    }

    if n_read == 0 {
        break;
    }

    n_read = n_read.min(end - start);
    let buffer = unsafe { std::slice::from_raw_parts(buf as *mut u8, bufsize as usize) };
    let written = std::io::stdout().write(&buffer[start as usize..(start + n_read) as usize] as &[u8]);
    if written.is_err() {
        error(
            1,
            written.err().unwrap().raw_os_error().unwrap_or(0),
            b"%s\0" as *const u8 as *const i8,
            quotearg_n_style_colon(0, shell_escape_quoting_style, b"-\0" as *const u8 as *const i8),
        );
        return; // Handle error appropriately
    }

    start += n_read;
}

}
unsafe extern "C" fn ofile_open(
    mut files: *mut of_t,
    mut i_check: idx_t,
    mut nfiles: idx_t,
) -> bool {
    let mut file_limit: bool = 0 as libc::c_int != 0;
    if (*files.offset(i_check as isize)).ofd <= OFD_NEW as libc::c_int {
        let mut fd: libc::c_int = 0;
        let mut i_reopen: idx_t = if i_check != 0 {
            i_check - 1 as libc::c_int as libc::c_long
        } else {
            nfiles - 1 as libc::c_int as libc::c_long
        };
        loop {
            if (*files.offset(i_check as isize)).ofd == OFD_NEW {
    fd = create((*files.offset(i_check as isize)).of_name);
} else {
    fd = open_safer(
        (*files.offset(i_check as isize)).of_name,
        libc::O_RDONLY | libc::O_CREAT | libc::O_EXCL | libc::O_TRUNC,
    );
}

if fd >= 0 {
    break;
}

if !(*__errno_location() == 24 || *__errno_location() == 23) {
    let errstatus = 1;
    error(
        errstatus,
        *__errno_location(),
        "%s\0".as_ptr() as *const libc::c_char,
        quotearg_n_style_colon(
            0,
            shell_escape_quoting_style,
            (*files.offset(i_check as isize)).of_name,
        ),
    );
    unreachable!();
}

file_limit = true;

while (*files.offset(i_reopen as isize)).ofd < 0 {
    i_reopen = if i_reopen != 0 {
        i_reopen - 1
    } else {
        nfiles - 1
    };

    if i_reopen == i_check {
        let errstatus = 1;
        error(
            errstatus,
            *__errno_location(),
            "%s\0".as_ptr() as *const libc::c_char,
            quotearg_n_style_colon(
                0,
                shell_escape_quoting_style,
                (*files.offset(i_check as isize)).of_name,
            ),
        );
        unreachable!();
    }
}

            if rpl_fclose((*files.offset(i_reopen as isize)).ofile) != 0 as libc::c_int {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            (*files.offset(i_reopen as isize)).of_name,
                        ),
                    );
                    if 1 as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                (*files.offset(i_reopen as isize)).of_name,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                (*files.offset(i_reopen as isize)).of_name,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            let ref mut fresh18 = (*files.offset(i_reopen as isize)).ofile;
            *fresh18 = 0 as *mut FILE;
            (*files.offset(i_reopen as isize)).ofd = OFD_APPEND as libc::c_int;
        }
        (*files.offset(i_check as isize)).ofd = fd;
        let mut ofile: *mut FILE = fdopen(
            fd,
            b"a\0" as *const u8 as *const libc::c_char,
        );
        if ofile.is_null() {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        (*files.offset(i_check as isize)).of_name,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            (*files.offset(i_check as isize)).of_name,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            (*files.offset(i_check as isize)).of_name,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        let ref mut fresh19 = (*files.offset(i_check as isize)).ofile;
        *fresh19 = ofile;
        (*files.offset(i_check as isize)).opid = filter_pid;
        filter_pid = 0 as libc::c_int;
    }
    return file_limit;
}
unsafe extern "C" fn lines_rr(
    mut k: intmax_t,
    mut n: intmax_t,
    mut buf: *mut libc::c_char,
    mut bufsize: idx_t,
    mut filesp: *mut *mut of_t,
) {
    let mut wrapped: bool = 0 as libc::c_int != 0;
    let mut wrote: bool = 0 as libc::c_int != 0;
    let mut file_limit: bool = false;
    let mut i_file: idx_t = 0;
    let mut files: *mut of_t = 0 as *mut of_t;
    let mut line_no: intmax_t = 0;
    if k != 0 {
        line_no = 1 as libc::c_int as intmax_t;
    } else {
        if (9223372036854775807 as libc::c_long) < n {
            xalloc_die();
        }
        *filesp = xinmalloc(n, ::core::mem::size_of::<of_t>() as libc::c_ulong as idx_t)
            as *mut of_t;
        files = *filesp;
        i_file = 0 as libc::c_int as idx_t;
        while i_file < n {
            next_file_name();
            let ref mut fresh20 = (*files.offset(i_file as isize)).of_name;
            *fresh20 = xstrdup(outfile);
            (*files.offset(i_file as isize)).ofd = OFD_NEW as libc::c_int;
            let ref mut fresh21 = (*files.offset(i_file as isize)).ofile;
            *fresh21 = 0 as *mut FILE;
            (*files.offset(i_file as isize)).opid = 0 as libc::c_int;
            i_file += 1;
            i_file;
        }
        i_file = 0 as libc::c_int as idx_t;
        file_limit = 0 as libc::c_int != 0;
    }
    's_76: loop {
        let mut bp: *mut libc::c_char = buf;
        let mut eob: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut n_read: ssize_t = read(
            0 as libc::c_int,
            buf as *mut libc::c_void,
            bufsize as size_t,
        );
        if n_read < 0 as libc::c_int as libc::c_long {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        } else if n_read == 0 as libc::c_int as libc::c_long {
            break;
        }
        eob = buf.offset(n_read as isize);
        while bp != eob {
            let mut to_write: idx_t = 0;
            let mut next: bool = 0 as libc::c_int != 0;
            let mut bp_out: *mut libc::c_char = memchr(
                bp as *const libc::c_void,
                eolchar,
                eob.offset_from(bp) as libc::c_long as libc::c_ulong,
            ) as *mut libc::c_char;
            if !bp_out.is_null() {
                bp_out = bp_out.offset(1);
                bp_out;
                next = 1 as libc::c_int != 0;
            } else {
                bp_out = eob;
            }
            to_write = bp_out.offset_from(bp) as libc::c_long;
            if k != 0 {
                if line_no == k && unbuffered as libc::c_int != 0 {
                    if full_write(
                        1 as libc::c_int,
                        bp as *const libc::c_void,
                        to_write as size_t,
                    ) != to_write as libc::c_ulong
                    {
                        write_error();
                    }
                } else if line_no == k
                    && fwrite_unlocked(
                        bp as *const libc::c_void,
                        to_write as size_t,
                        1 as libc::c_int as size_t,
                        stdout,
                    ) != 1 as libc::c_int as libc::c_ulong
                {
                    write_error();
                }
                if next {
                    line_no = if line_no == n {
                        1 as libc::c_int as libc::c_long
                    } else {
                        line_no + 1 as libc::c_int as libc::c_long
                    };
                }
            } else {
                file_limit = (file_limit as libc::c_int
                    | ofile_open(files, i_file, n) as libc::c_int) != 0;
                if unbuffered {
                    if full_write(
                        (*files.offset(i_file as isize)).ofd,
                        bp as *const libc::c_void,
                        to_write as size_t,
                    ) != to_write as libc::c_ulong && !ignorable(*__errno_location())
                    {
                        if 0 != 0 {
                            error(
                                1 as libc::c_int,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    (*files.offset(i_file as isize)).of_name,
                                ),
                            );
                            if 1 as libc::c_int != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                        } else {
                            ({
                                let __errstatus: libc::c_int = 1 as libc::c_int;
                                error(
                                    __errstatus,
                                    *__errno_location(),
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    quotearg_n_style_colon(
                                        0 as libc::c_int,
                                        shell_escape_quoting_style,
                                        (*files.offset(i_file as isize)).of_name,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                            ({
                                let __errstatus: libc::c_int = 1 as libc::c_int;
                                error(
                                    __errstatus,
                                    *__errno_location(),
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    quotearg_n_style_colon(
                                        0 as libc::c_int,
                                        shell_escape_quoting_style,
                                        (*files.offset(i_file as isize)).of_name,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                    }
                } else if fwrite_unlocked(
                    bp as *const libc::c_void,
                    to_write as size_t,
                    1 as libc::c_int as size_t,
                    (*files.offset(i_file as isize)).ofile,
                ) != 1 as libc::c_int as libc::c_ulong && !ignorable(*__errno_location())
                {
                    if 0 != 0 {
                        error(
                            1 as libc::c_int,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                (*files.offset(i_file as isize)).of_name,
                            ),
                        );
                        if 1 as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = 1 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    (*files.offset(i_file as isize)).of_name,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = 1 as libc::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                b"%s\0" as *const u8 as *const libc::c_char,
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    (*files.offset(i_file as isize)).of_name,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
                if !ignorable(*__errno_location()) {
                    wrote = 1 as libc::c_int != 0;
                }
                if file_limit {
    if rpl_fclose((*files.offset(i_file as isize)).ofile) != 0 {
        let errstatus = 1;
        error(
            errstatus,
            std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
            b"%s\0".as_ptr() as *const i8,
            quotearg_n_style_colon(0, shell_escape_quoting_style, (*files.offset(i_file as isize)).of_name),
        );
        if errstatus != 0 {
            unreachable!();
        }
    }
    (*files.offset(i_file as isize)).ofile = std::ptr::null_mut();
    (*files.offset(i_file as isize)).ofd = OFD_APPEND;
}

if next && {
    i_file += 1;
    i_file == n
} {
    wrapped = true;
    if !wrote {
        break 's_76;
    }
    wrote = false;
    i_file = 0;
}

            }
            bp = bp_out;
        }
    }
    if k == 0 {
        let mut ceiling: idx_t = if wrapped as libc::c_int != 0 { n } else { i_file };
        i_file = 0 as libc::c_int as idx_t;
        while i_file < n {
            if i_file >= ceiling && !elide_empty_files {
                file_limit = (file_limit as libc::c_int
                    | ofile_open(files, i_file, n) as libc::c_int) != 0;
            }
            if (*files.offset(i_file as isize)).ofd >= 0 as libc::c_int {
                closeout(
                    (*files.offset(i_file as isize)).ofile,
                    (*files.offset(i_file as isize)).ofd,
                    (*files.offset(i_file as isize)).opid,
                    (*files.offset(i_file as isize)).of_name,
                );
            }
            (*files.offset(i_file as isize)).ofd = OFD_APPEND as libc::c_int;
            i_file += 1;
            i_file;
        }
    }
}
fn strtoint_die(msgid: *const libc::c_char, arg: *const libc::c_char) {
    let err_status = 1;
    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
    
    let msgid_str = unsafe { std::ffi::CStr::from_ptr(msgid).to_string_lossy() };
    let arg_str = unsafe { std::ffi::CStr::from_ptr(arg).to_string_lossy() };

    unsafe {
        error(
            err_status,
            if errno == 22 { 0 } else { errno },
            b"%s: %s\0".as_ptr() as *const libc::c_char,
            msgid_str.as_ptr() as *const libc::c_char,
            arg_str.as_ptr() as *const libc::c_char,
        );
    }

    if err_status != 0 {
        std::process::abort(); // Use abort instead of unreachable to avoid hanging
    }
}

unsafe extern "C" fn parse_n_units(
    mut arg: *const libc::c_char,
    mut multipliers: *const libc::c_char,
    mut msgid: *const libc::c_char,
) -> intmax_t {
    let mut n: intmax_t = 0;
    if (LONGINT_OVERFLOW as libc::c_int as libc::c_uint)
        < xstrtoimax(
            arg,
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
            &mut n,
            multipliers,
        ) as libc::c_uint || n < 1 as libc::c_int as libc::c_long
    {
        strtoint_die(msgid, arg);
    }
    return n;
}
unsafe extern "C" fn parse_chunk(
    mut k_units: *mut intmax_t,
    mut n_units: *mut intmax_t,
    mut arg: *const libc::c_char,
) {
    let mut argend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: strtol_error = xstrtoimax(
        arg,
        &mut argend,
        10 as libc::c_int,
        n_units,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if e as libc::c_uint == LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint
        && *argend as libc::c_int == '/' as i32
    {
        *k_units = *n_units;
        *n_units = parse_n_units(
            argend.offset(1 as libc::c_int as isize),
            b"\0" as *const u8 as *const libc::c_char,
            b"invalid number of chunks\0" as *const u8 as *const libc::c_char,
        );
        if !((0 as libc::c_int as libc::c_long) < *k_units && *k_units <= *n_units) {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid chunk number\0" as *const u8 as *const libc::c_char,
                    ),
                    quote_mem(arg, argend.offset_from(arg) as libc::c_long as size_t),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        gettext(
                            b"invalid chunk number\0" as *const u8 as *const libc::c_char,
                        ),
                        quote_mem(arg, argend.offset_from(arg) as libc::c_long as size_t),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        gettext(
                            b"invalid chunk number\0" as *const u8 as *const libc::c_char,
                        ),
                        quote_mem(arg, argend.offset_from(arg) as libc::c_long as size_t),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    } else if !(e as libc::c_uint <= LONGINT_OVERFLOW as libc::c_int as libc::c_uint
        && (0 as libc::c_int as libc::c_long) < *n_units)
    {
        strtoint_die(
    b"invalid number of chunks\0" as *const u8 as *const libc::c_char,
    arg,
);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut split_type: Split_type = type_undef;
    let mut in_blk_size: idx_t = 0 as libc::c_int as idx_t;
    let mut page_size: idx_t = getpagesize() as idx_t;
    let mut k_units: intmax_t = 0 as libc::c_int as intmax_t;
    let mut n_units: intmax_t = 0 as libc::c_int as intmax_t;
    static mut multipliers: [libc::c_char; 15] = unsafe {
        *::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"bEGKkMmPQRTYZ0\0")
    };
    let mut c: libc::c_int = 0;
    let mut digits_optind: libc::c_int = 0 as libc::c_int;
    let mut file_size: off_t = if (0 as libc::c_int as off_t)
        < -(1 as libc::c_int) as off_t
    {
        -(1 as libc::c_int) as off_t
    } else {
        (((1 as libc::c_int as off_t)
            << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    };
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    infile = b"-\0" as *const u8 as *const libc::c_char;
    outbase = b"x\0" as *const u8 as *const libc::c_char;
    loop {
        let this_optind: i32 = if optind != 0 {
    optind
} else {
    1
};

c = getopt_long(
    argc,
    argv,
    b"0123456789C:a:b:del:n:t:ux\0" as *const u8 as *const i8,
    longopts.as_ptr(),
    std::ptr::null_mut(),
);

if c == -1 {
    break;
}

match c {
    97 => {
         suffix_length = xdectoimax(
                    optarg,
                    0,
                    i64::MAX,
                    std::ptr::null(),
                    gettext(std::ffi::CString::new("invalid suffix length").unwrap().as_ptr()),
                    0,
                );


    }
    259 => {
         let suffix_len = unsafe { CStr::from_ptr(optarg).to_bytes().len() as i32 };
let optarg_str = unsafe { CStr::from_ptr(optarg).to_str().unwrap() };
if last_component(optarg) != optarg
    || (suffix_len != 0 && optarg_str.ends_with('/')) {
    let errstatus = 0;
    error(
        errstatus,
        0,
        gettext(b"invalid suffix %s, contains directory separator\0".as_ptr() as *const libc::c_char),
        quote(optarg),
    );
    usage(1);
}
additional_suffix = optarg;


    }
    98 => {
         if split_type != type_undef {
    if false {
        error(
            0,
            0,
            unsafe { gettext(b"cannot split in more than one way\0".as_ptr() as *const libc::c_char) },
        );
        if false {
            unreachable!();
        }
    } else {
        let errstatus = 0;
        error(
            errstatus,
            0,
            unsafe { gettext(b"cannot split in more than one way\0".as_ptr() as *const libc::c_char) },
        );
        if errstatus != 0 {
            unreachable!();
        }

        let errstatus = 0;
        error(
            errstatus,
            0,
            unsafe { gettext(b"cannot split in more than one way\0".as_ptr() as *const libc::c_char) },
        );
        if errstatus != 0 {
            unreachable!();
        }
    }
    usage(1);
}
split_type = type_bytes;
n_units = unsafe {
    parse_n_units(
        optarg,
        multipliers.as_ptr(),
        b"invalid number of bytes\0".as_ptr() as *const libc::c_char,
    )
};


    }
    108 => {
         if split_type != type_undef {
    if false {
        error(0, 0, gettext(b"cannot split in more than one way\0".as_ptr() as *const libc::c_char));
        unreachable!();
    } else {
        let errstatus = 0;
        error(errstatus, 0, gettext(b"cannot split in more than one way\0".as_ptr() as *const libc::c_char));
        if errstatus != 0 {
            unreachable!();
        }
        
        let errstatus = 0;
        error(errstatus, 0, gettext(b"cannot split in more than one way\0".as_ptr() as *const libc::c_char));
        if errstatus != 0 {
            unreachable!();
        }
    }
    usage(1);
}
split_type = type_lines;
n_units = parse_n_units(
    optarg,
    b"\0".as_ptr() as *const libc::c_char,
    b"invalid number of lines\0".as_ptr() as *const libc::c_char,
);


    }
    67 => {
         if split_type != type_undef {
    if false {
        error(
            0,
            0,
            gettext(b"cannot split in more than one way\0".as_ptr() as *const libc::c_char),
        );
        unreachable!();
    } else {
        let errstatus = 0;
        error(
            errstatus,
            0,
            gettext(b"cannot split in more than one way\0".as_ptr() as *const libc::c_char),
        );
        if errstatus != 0 {
            unreachable!();
        }
    }
    usage(1);
}
split_type = type_byteslines;
n_units = parse_n_units(
    optarg,
    multipliers.as_ptr(),
    b"invalid number of lines\0".as_ptr() as *const libc::c_char,
);


    }
    110 => {
         if split_type != type_undef {
    if false {
        error(0, 0, gettext("cannot split in more than one way\0".as_ptr() as *const libc::c_char));
        unreachable!();
    } else {
        let errstatus = 0;
        error(errstatus, 0, gettext("cannot split in more than one way\0".as_ptr() as *const libc::c_char));
        if errstatus != 0 {
            unreachable!();
        }
    }
    usage(1);
}

let mut local_optarg = unsafe { std::ffi::CStr::from_ptr(optarg).to_string_lossy() };
let mut trimmed_optarg = local_optarg.as_ref();

while trimmed_optarg.chars().next().map_or(false, |c| c.is_whitespace()) {
    trimmed_optarg = &trimmed_optarg[1..];
}

if trimmed_optarg.starts_with("r/") {
    split_type = type_rr;
    trimmed_optarg = &trimmed_optarg[2..];
} else if trimmed_optarg.starts_with("l/") {
    split_type = type_chunk_lines;
    trimmed_optarg = &trimmed_optarg[2..];
} else {
    split_type = type_chunk_bytes;
}

parse_chunk(&mut k_units, &mut n_units, trimmed_optarg.as_ptr() as *const libc::c_char);


    }
    117 => {
        unbuffered = true;
    }
    116 => {
         let optarg_str = unsafe { std::ffi::CStr::from_ptr(optarg) }.to_string_lossy();
let mut neweol = optarg_str.chars().next().unwrap_or('\0') as i8;

if neweol == 0 {
    let errstatus = 1;
    error(
        errstatus,
        0,
        gettext("empty record separator\0".as_ptr() as *const libc::c_char),
    );
    unreachable!();
}

if optarg_str.len() > 1 {
    if optarg_str == "\\0" {
        neweol = '\0' as i32 as i8;
    } else {
        let errstatus = 1;
        error(
            errstatus,
            0,
            gettext("multi-character separator %s\0".as_ptr() as *const libc::c_char),
            quote(optarg_str.as_ptr() as *const libc::c_char),
        );
        unreachable!();
    }
}

if 0 <= eolchar && neweol as i32 != eolchar {
    let errstatus = 1;
    error(
        errstatus,
        0,
        gettext("multiple separator characters specified\0".as_ptr() as *const libc::c_char),
    );
    unreachable!();
}

eolchar = neweol as i32;


    }
    48..=57 => {
         if split_type as libc::c_uint
                    == type_undef as libc::c_int as libc::c_uint
                {
                    split_type = type_digits;
                    n_units = 0 as libc::c_int as intmax_t;
                }
                if split_type as libc::c_uint
                    != type_undef as libc::c_int as libc::c_uint
                    && split_type as libc::c_uint
                        != type_digits as libc::c_int as libc::c_uint
                {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"cannot split in more than one way\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if 0 as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                0 as libc::c_int,
                                gettext(
                                    b"cannot split in more than one way\0" as *const u8
                                        as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                0 as libc::c_int,
                                gettext(
                                    b"cannot split in more than one way\0" as *const u8
                                        as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    usage(1 as libc::c_int);
                }
                if digits_optind != 0 as libc::c_int && digits_optind != this_optind {
                    n_units = 0 as libc::c_int as intmax_t;
                }
                digits_optind = this_optind;
                if (if (0 as libc::c_int as intmax_t) < -(1 as libc::c_int) as intmax_t
                    && ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n_units
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    && ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        10 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    && (if (10 as libc::c_int) < 0 as libc::c_int {
                        if n_units < 0 as libc::c_int as libc::c_long {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    -(1 as libc::c_int) as intmax_t
                                }) + 10 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                (n_units
                                    < -(1 as libc::c_int) as intmax_t
                                        / 10 as libc::c_int as libc::c_long) as libc::c_int
                            } else {
                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    ((10 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                }) != 0
                                {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) as libc::c_long + -(1 as libc::c_int) as intmax_t
                                        >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    -(1 as libc::c_int) as intmax_t
                                        / -(10 as libc::c_int) as libc::c_long
                                }) <= -(1 as libc::c_int) as libc::c_long - n_units)
                                    as libc::c_int
                            }
                        } else {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) as libc::c_long + 0 as libc::c_int as intmax_t
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) as libc::c_long + 0 as libc::c_int as intmax_t
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) as libc::c_long + 0 as libc::c_int as intmax_t
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) as libc::c_long + 0 as libc::c_int as intmax_t)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) as libc::c_long + 0 as libc::c_int as intmax_t
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) as libc::c_long + 0 as libc::c_int as intmax_t
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) as libc::c_long + 0 as libc::c_int as intmax_t
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) as libc::c_long + 0 as libc::c_int as intmax_t)
                                    as libc::c_int
                            }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n_units
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((0 as libc::c_int as libc::c_long)
                                        < n_units + 0 as libc::c_int as intmax_t) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < n_units
                                        && (-(1 as libc::c_int) as libc::c_long
                                            - 0 as libc::c_int as intmax_t)
                                            < n_units - 1 as libc::c_int as libc::c_long) as libc::c_int
                                }
                            } else {
                                ((0 as libc::c_int as intmax_t
                                    / 10 as libc::c_int as libc::c_long) < n_units)
                                    as libc::c_int
                            }
                        }
                    } else {
                        if 10 as libc::c_int == 0 as libc::c_int {
                            0 as libc::c_int
                        } else {
                            if n_units < 0 as libc::c_int as libc::c_long {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n_units
                                    }) + 0 as libc::c_int as intmax_t
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n_units
                                        }) + 0 as libc::c_int as intmax_t
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n_units
                                        }) + 0 as libc::c_int as intmax_t
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n_units
                                    }) + 0 as libc::c_int as intmax_t)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n_units
                                            }) + 0 as libc::c_int as intmax_t
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n_units
                                                }) + 0 as libc::c_int as intmax_t
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n_units
                                                }) + 0 as libc::c_int as intmax_t
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n_units
                                        }) + 0 as libc::c_int as intmax_t) as libc::c_int
                                }) != 0 && n_units == -(1 as libc::c_int) as libc::c_long
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < 10 as libc::c_int as libc::c_long
                                                + 0 as libc::c_int as intmax_t) as libc::c_int
                                    } else {
                                        ((-(1 as libc::c_int) as libc::c_long
                                            - 0 as libc::c_int as intmax_t)
                                            < (10 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                                            as libc::c_int
                                    }
                                } else {
                                    (0 as libc::c_int as intmax_t / n_units
                                        < 10 as libc::c_int as libc::c_long) as libc::c_int
                                }
                            } else {
                                ((-(1 as libc::c_int) as intmax_t
                                    / 10 as libc::c_int as libc::c_long) < n_units)
                                    as libc::c_int
                            }
                        }
                    }) != 0
                {
                    let (fresh27, _fresh28) = n_units.overflowing_mul((10 as libc::c_int).into());
                    *(&mut n_units as *mut intmax_t) = fresh27;
                    1 as libc::c_int
                } else {
                    let (fresh29, fresh30) = n_units.overflowing_mul((10 as libc::c_int).into());
                    *(&mut n_units as *mut intmax_t) = fresh29;
                    fresh30 as libc::c_int
                }) != 0
                    || {
                        let (fresh31, fresh32) = n_units.overflowing_add((c - '0' as i32).into());
                        *(&mut n_units as *mut intmax_t) = fresh31;
                        fresh32 as libc::c_int != 0
                    }
                {
                    n_units = 9223372036854775807 as libc::c_long;
                }

    }
    100 | 120 => {
         if c == 'd' as i32 {
                    suffix_alphabet = b"0123456789\0" as *const u8
                        as *const libc::c_char;
                } else {
                    suffix_alphabet = b"0123456789abcdef\0" as *const u8
                        as *const libc::c_char;
                }
                if !optarg.is_null() {
                    if strlen(optarg) != strspn(optarg, suffix_alphabet) {
                        if 0 != 0 {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                if c == 'd' as i32 {
                                    gettext(
                                        b"%s: invalid start value for numerical suffix\0"
                                            as *const u8 as *const libc::c_char,
                                    )
                                } else {
                                    gettext(
                                        b"%s: invalid start value for hexadecimal suffix\0"
                                            as *const u8 as *const libc::c_char,
                                    )
                                },
                                quote(optarg),
                            );
                            if 0 as libc::c_int != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                        } else {
                            ({
                                let __errstatus: libc::c_int = 0 as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    if c == 'd' as i32 {
                                        gettext(
                                            b"%s: invalid start value for numerical suffix\0"
                                                as *const u8 as *const libc::c_char,
                                        )
                                    } else {
                                        gettext(
                                            b"%s: invalid start value for hexadecimal suffix\0"
                                                as *const u8 as *const libc::c_char,
                                        )
                                    },
                                    quote(optarg),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                            ({
                                let __errstatus: libc::c_int = 0 as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    if c == 'd' as i32 {
                                        gettext(
                                            b"%s: invalid start value for numerical suffix\0"
                                                as *const u8 as *const libc::c_char,
                                        )
                                    } else {
                                        gettext(
                                            b"%s: invalid start value for hexadecimal suffix\0"
                                                as *const u8 as *const libc::c_char,
                                        )
                                    },
                                    quote(optarg),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                        usage(1 as libc::c_int);
                    } else {
                        while *optarg as libc::c_int == '0' as i32
                            && *optarg.offset(1 as libc::c_int as isize) as libc::c_int
                                != '\0' as i32
                        {
                            optarg = optarg.offset(1);
                            optarg;
                        }
                        numeric_suffix_start = optarg;
                    }
                }

    }
    101 => {
        elide_empty_files = true;
    }
    257 => {
        filter_command = optarg;
    }
    258 => {
         in_blk_size = xdectoumax(
                    optarg,
                    1 as libc::c_int as uintmax_t,
                    if (SYS_BUFSIZE_MAX as libc::c_int as libc::c_ulong)
                        < (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                            < 18446744073709551615 as libc::c_ulong
                        {
                            9223372036854775807 as libc::c_long as libc::c_ulong
                        } else {
                            18446744073709551615 as libc::c_ulong
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        SYS_BUFSIZE_MAX as libc::c_int as libc::c_ulong
                    } else {
                        (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                            < 18446744073709551615 as libc::c_ulong
                        {
                            9223372036854775807 as libc::c_long as libc::c_ulong
                        } else {
                            18446744073709551615 as libc::c_ulong
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    },
                    multipliers.as_ptr(),
                    gettext(
                        b"invalid IO block size\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                ) as idx_t;

    }
    256 => {
        verbose = true;
    }
    -2 => {
        usage(0);
    }
    -3 => {
         version_etc(
    unsafe { stdout },
    b"split\0".as_ptr() as *const libc::c_char,
    b"GNU coreutils\0".as_ptr() as *const libc::c_char,
    Version,
    proper_name_lite(
        b"Torbjorn Granlund\0".as_ptr() as *const libc::c_char,
        b"Torbj\xF6rn Granlund\0".as_ptr() as *const libc::c_char,
    ),
    proper_name_lite(
        b"Richard M. Stallman\0".as_ptr() as *const libc::c_char,
        b"Richard M. Stallman\0".as_ptr() as *const libc::c_char,
    ),
    std::ptr::null_mut::<libc::c_char>(),
);
std::process::exit(0);


    }
    _ => {
        usage(1);
    }
}

/*
The variables live at this point are:
(mut argc: i32, mut argv: *mut *mut i8, mut split_type: u32, mut in_blk_size: i64, mut k_units: i64, mut n_units: i64, mut c: i32, mut digits_optind: i32, mut this_optind: i32)
*/

    }
    if k_units != 0 as libc::c_int as libc::c_long && !filter_command.is_null() {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"--filter does not process a chunk extracted to stdout\0"
                        as *const u8 as *const libc::c_char,
                ),
            );
            if 0 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"--filter does not process a chunk extracted to stdout\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"--filter does not process a chunk extracted to stdout\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        usage(1 as libc::c_int);
    }
    if split_type as libc::c_uint == type_undef as libc::c_int as libc::c_uint {
        split_type = type_lines;
        n_units = 1000 as libc::c_int as intmax_t;
    }
    if n_units == 0 as libc::c_int as libc::c_long {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"invalid number of lines: %s\0" as *const u8 as *const libc::c_char,
                ),
                quote(b"0\0" as *const u8 as *const libc::c_char),
            );
            if 0 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"invalid number of lines: %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote(b"0\0" as *const u8 as *const libc::c_char),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"invalid number of lines: %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote(b"0\0" as *const u8 as *const libc::c_char),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        usage(1 as libc::c_int);
    }
    if eolchar < 0 as libc::c_int {
        eolchar = '\n' as i32;
    }
    set_suffix_length(n_units, split_type);
    if optind < argc {
        let fresh33 = optind;
        optind = optind + 1;
        infile = *argv.offset(fresh33 as isize);
    }
    if optind < argc {
        let fresh34 = optind;
        optind = optind + 1;
        outbase = *argv.offset(fresh34 as isize);
    }
    if optind < argc {
    let extra_operand_error = format!("extra operand {}", unsafe { CStr::from_ptr(*argv.offset(optind as isize)).to_string_lossy() });
    error(0, 0, gettext(extra_operand_error.as_ptr() as *const libc::c_char));
    usage(1);
}

if !numeric_suffix_start.is_null() && strlen(numeric_suffix_start) > suffix_length as libc::c_ulong {
    let suffix_error = "numerical suffix start value is too large for the suffix length";
    error(0, 0, gettext(suffix_error.as_ptr() as *const libc::c_char));
    usage(1);
}

if unsafe { strcmp(infile, b"-\0" as *const u8 as *const libc::c_char) } != 0 {
    if fd_reopen(0, infile, 0, 0) < 0 {
        let open_error = format!("cannot open {}", unsafe { CStr::from_ptr(infile).to_string_lossy() });
        error(1, *__errno_location(), gettext(open_error.as_ptr() as *const libc::c_char));
    }
}

xset_binary_mode(0, 0);
;
    fdadvise(
        0 as libc::c_int,
        0 as libc::c_int as off_t,
        0 as libc::c_int as off_t,
        FADVISE_SEQUENTIAL,
    );
    if fstat(0 as libc::c_int, &mut in_stat_buf) != 0 as libc::c_int {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    infile,
                ),
            );
            if 1 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    if in_blk_size == 0 as libc::c_int as libc::c_long {
        in_blk_size = io_blksize(&mut in_stat_buf);
        if (SYS_BUFSIZE_MAX as libc::c_int as libc::c_long) < in_blk_size {
            in_blk_size = SYS_BUFSIZE_MAX as libc::c_int as idx_t;
        }
    }
    let mut buf: *mut libc::c_char = xalignalloc(
        page_size,
        in_blk_size + 1 as libc::c_int as libc::c_long,
    ) as *mut libc::c_char;
    let mut initial_read: ssize_t = -(1 as libc::c_int) as ssize_t;
    if split_type as libc::c_uint == type_chunk_bytes as libc::c_int as libc::c_uint
        || split_type as libc::c_uint == type_chunk_lines as libc::c_int as libc::c_uint
    {
        file_size = input_file_size(
            0 as libc::c_int,
            &mut in_stat_buf,
            buf,
            in_blk_size,
        );
        if file_size < 0 as libc::c_int as libc::c_long {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"%s: cannot determine file size\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"%s: cannot determine file size\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"%s: cannot determine file size\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        initial_read = if file_size < in_blk_size { file_size } else { in_blk_size };
    }
    if !filter_command.is_null() {
        default_SIGPIPE = (signal(
            13 as libc::c_int,
            ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        ))
            .is_none();
    }
    match split_type as libc::c_uint {
        4 | 3 => {
            lines_split(n_units, buf, in_blk_size);
        }
        1 => {
            bytes_split(
                n_units,
                0 as libc::c_int as intmax_t,
                buf,
                in_blk_size,
                -(1 as libc::c_int) as ssize_t,
                0 as libc::c_int as intmax_t,
            );
        }
        2 => {
            line_bytes_split(n_units, buf, in_blk_size);
        }
        5 => {
            if k_units == 0 as libc::c_int as libc::c_long {
                bytes_split(
                    file_size / n_units,
                    file_size % n_units,
                    buf,
                    in_blk_size,
                    initial_read,
                    n_units,
                );
            } else {
                bytes_chunk_extract(
                    k_units,
                    n_units,
                    buf,
                    in_blk_size,
                    initial_read,
                    file_size,
                );
            }
        }
        6 => {
            lines_chunk_split(
                k_units,
                n_units,
                buf,
                in_blk_size,
                initial_read,
                file_size,
            );
        }
        7 => {
            let mut files: *mut of_t = 0 as *mut of_t;
            lines_rr(k_units, n_units, buf, in_blk_size, &mut files);
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"split.c\0" as *const u8 as *const libc::c_char,
                1686 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
            'c_16295: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"split.c\0" as *const u8 as *const libc::c_char,
                    1686 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            };
        }
    }
    if close(0 as libc::c_int) != 0 as libc::c_int {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    infile,
                ),
            );
            if 1 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 1 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        infile,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    closeout(0 as *mut FILE, output_desc, filter_pid, outfile);
    exit(0 as libc::c_int);
}
pub fn main() {
    let args: Vec<CString> = ::std::env::args()
        .map(|arg| CString::new(arg).expect("Failed to convert argument into CString."))
        .collect();
    
    let mut argv: Vec<*mut libc::c_char> = args.iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    
    argv.push(std::ptr::null_mut());

    unsafe {
        ::std::process::exit(main_0((argv.len() - 1) as libc::c_int, argv.as_mut_ptr()) as i32);
    }
}

