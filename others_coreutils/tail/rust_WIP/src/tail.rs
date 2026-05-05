#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types, label_break_value)]
















































use std::cmp::Ordering;

use std::convert::TryInto;
use std::slice;

use std::os::unix::io::AsRawFd;

use std::os::raw::c_int;

use std::ptr;
use std::process;

use std::ptr::null_mut;

use std::ffi::CString;
use std::os::raw::c_void;
use std::io;

use std::ffi::CStr;

use std::io::Write;

use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fpurge(gl_stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut Version: *const libc::c_char;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
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
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn dir_len(file: *const libc::c_char) -> size_t;
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
    fn offtostr(_: off_t, _: *mut libc::c_char) -> *mut libc::c_char;
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
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    static mut argmatch_die: argmatch_exit_fn;
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn cl_strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn iopoll(fdin: libc::c_int, fdout: libc::c_int, block: bool) -> libc::c_int;
    fn isapipe(fd: libc::c_int) -> libc::c_int;
    fn posix2_version() -> libc::c_int;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn xdectoumax(
        n_str: *const libc::c_char,
        min: uintmax_t,
        max: uintmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> uintmax_t;
    fn xnanosleep(_: libc::c_double) -> libc::c_int;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn xstrtod(
        str: *const libc::c_char,
        ptr: *mut *const libc::c_char,
        result: *mut libc::c_double,
        convert: Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *mut *mut libc::c_char,
            ) -> libc::c_double,
        >,
    ) -> bool;
    fn hash_get_n_entries(table: *const Hash_table) -> size_t;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_free(table: *mut Hash_table);
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_remove(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn inotify_init() -> libc::c_int;
    fn inotify_add_watch(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __mask: uint32_t,
    ) -> libc::c_int;
    fn inotify_rm_watch(__fd: libc::c_int, __wd: libc::c_int) -> libc::c_int;
    fn fstatfs(__fildes: libc::c_int, __buf: *mut statfs) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [libc::c_int; 2],
}
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type ptrdiff_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type blksize_t = __blksize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
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
pub type uint32_t = __uint32_t;
pub type uintmax_t = __uintmax_t;
pub type idx_t = ptrdiff_t;
pub type C2RustUnnamed = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed = -2;
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
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
pub type Hash_table = hash_table;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inotify_event {
    pub wd: libc::c_int,
    pub mask: uint32_t,
    pub cookie: uint32_t,
    pub len: uint32_t,
    pub name: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
pub type Follow_mode = libc::c_uint;
pub const Follow_descriptor: Follow_mode = 2;
pub const Follow_name: Follow_mode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct File_spec {
    pub name: *mut libc::c_char,
    pub size: off_t,
    pub mtime: timespec,
    pub dev: dev_t,
    pub ino: ino_t,
    pub mode: mode_t,
    pub ignore: bool,
    pub remote: bool,
    pub tailable: bool,
    pub fd: libc::c_int,
    pub errnum: libc::c_int,
    pub blocking: libc::c_int,
    pub wd: libc::c_int,
    pub parent_wd: libc::c_int,
    pub basename_start: size_t,
    pub n_unchanged_stats: uintmax_t,
}
pub type header_mode = libc::c_uint;
pub const never: header_mode = 2;
pub const always: header_mode = 1;
pub const multiple_files: header_mode = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DISABLE_INOTIFY_OPTION: C2RustUnnamed_0 = 261;
pub const LONG_FOLLOW_OPTION: C2RustUnnamed_0 = 260;
pub const PRESUME_INPUT_PIPE_OPTION: C2RustUnnamed_0 = 259;
pub const PID_OPTION: C2RustUnnamed_0 = 258;
pub const MAX_UNCHANGED_STATS_OPTION: C2RustUnnamed_0 = 257;
pub const RETRY_OPTION: C2RustUnnamed_0 = 256;
pub type LBUFFER = linebuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linebuffer {
    pub buffer: [libc::c_char; 8192],
    pub nbytes: size_t,
    pub nlines: size_t,
    pub next: *mut linebuffer,
}
pub type CBUFFER = charbuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charbuffer {
    pub buffer: [libc::c_char; 8192],
    pub nbytes: size_t,
    pub next: *mut charbuffer,
}
#[inline]
fn timespec_cmp(a: timespec, b: timespec) -> i32 {
    let sec_cmp = a.tv_sec.cmp(&b.tv_sec);
    if sec_cmp != std::cmp::Ordering::Equal {
        return match sec_cmp {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            _ => unreachable!(),
        };
    }
    let nsec_cmp = a.tv_nsec.cmp(&b.tv_nsec);
    match nsec_cmp {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

#[inline]
fn emit_stdin_note() {
    let message = "\nWith no FILE, or when FILE is -, read standard input.\n";
    let stdout_handle = std::io::stdout();
    let mut handle = stdout_handle.lock();
    handle.write_all(message.as_bytes()).expect("Failed to write to stdout");
}

#[inline]
fn emit_mandatory_arg_note() {
    let message = "\nMandatory arguments to long options are mandatory for short options too.\n";
    let stdout_handle = std::io::stdout();
    let mut handle = stdout_handle.lock();
    handle.write_all(message.as_bytes()).expect("Failed to write to stdout");
}

#[inline]
fn emit_ancillary_info(program: &str) {
    let infomap_0: [( &str, &str); 7] = [
        ( "[", "test invocation" ),
        ( "coreutils", "Multi-call invocation" ),
        ( "sha224sum", "sha2 utilities" ),
        ( "sha256sum", "sha2 utilities" ),
        ( "sha384sum", "sha2 utilities" ),
        ( "sha512sum", "sha2 utilities" ),
        ( "", "" ),
    ];

    let mut node = program;
    let mut map_prog = infomap_0.iter();

    while let Some(&(prog, n)) = map_prog.next() {
        if prog.is_empty() || program == prog {
            node = n;
            break;
        }
    }

    println!(
        "{} online help: <{}>",
        "GNU coreutils",
        "https://www.gnu.org/software/coreutils/"
    );

    let lc_messages = unsafe { setlocale(5, std::ptr::null()) };
    if !lc_messages.is_null() {
        let lc_messages_str = unsafe { std::ffi::CStr::from_ptr(lc_messages) };
        if !lc_messages_str.to_string_lossy().starts_with("en_") {
            eprint!(
                "{}",
                "Report any translation bugs to <https://translationproject.org/team/>"
            );
        }
    }

    let url_program = if program == "[" { "test" } else { program };

    println!(
        "Full documentation <{}{}>",
        "https://www.gnu.org/software/coreutils/",
        url_program
    );

    println!(
        "or available locally via: info '(coreutils) {}{}'",
        node,
        if node == program { " invocation" } else { "" }
    );
}

#[inline]
unsafe extern "C" fn usable_st_size(mut sb: *const stat) -> bool {
    return (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        || ((*sb).st_mode).wrapping_sub((*sb).st_mode) != 0 || 0 as libc::c_int != 0;
}
#[inline]
fn write_error() {
    let saved_errno = std::io::Error::last_os_error();
    let _ = std::io::stdout().flush();
    
    if false {
        unsafe {
            error(
                1,
                saved_errno.raw_os_error().unwrap_or(0),
                gettext(b"write error\0".as_ptr() as *const libc::c_char),
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
                    gettext(b"write error\0".as_ptr() as *const libc::c_char),
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
                    gettext(b"write error\0".as_ptr() as *const libc::c_char),
                );
            }
            if __errstatus != 0 {
                unreachable!();
            }
        }
    }
}

#[inline]
fn get_stat_mtime(st: &stat) -> timespec {
    st.st_mtim
}

#[inline]
fn xset_binary_mode_error() {
    // Implement the functionality that was previously unsafe or C API related.
    // For example, if this function was meant to set binary mode for a stream,
    // we can use Rust's standard library features to achieve that.
    
    // Assuming we want to set binary mode for standard output:
    let stdout_handle = std::io::stdout();
    let mut handle = stdout_handle.lock();
    
    // Set the output to binary mode if necessary.
    // This is a placeholder for the actual implementation.
    // In Rust, we typically don't have a direct equivalent to binary mode,
    // but we can ensure that we write bytes directly.
    handle.write_all(b""); // Example operation, replace with actual logic as needed.
}

#[inline]
fn xset_binary_mode(fd: i32, mode: i32) {
    unsafe {
        if set_binary_mode(fd, mode) < 0 {
            xset_binary_mode_error();
        }
    }
}

#[inline]
fn __gl_setmode(fd: i32, mode: i32) -> i32 {
    0
}

#[inline]
fn set_binary_mode(fd: libc::c_int, mode: libc::c_int) -> libc::c_int {
    __gl_setmode(fd, mode)
}

#[inline]
fn is_local_fs_type(magic: u64) -> i32 {
    match magic {
        1513908720 => 1,
        1633904243 => 0,
        44533 => 1,
        44543 => 1,
        1397113167 => 0,
        151263540 => 1,
        1635083891 => 0,
        391 => 1,
        325456742 => 1,
        3393526350 => 1,
        1111905073 => 1,
        1650746742 => 1,
        464386766 => 1,
        1819242352 => 1,
        3405662737 => 1,
        1112100429 => 1,
        2435016766 => 1,
        1936880249 => 1,
        12805120 => 0,
        2613483 => 1,
        1667723888 => 1,
        4283649346 => 0,
        1937076805 => 0,
        19920823 => 1,
        1650812272 => 1,
        684539205 => 1,
        1161678120 => 1,
        1684300152 => 1,
        1684170528 => 1,
        4979 => 1,
        1162691661 => 1,
        7377 => 1,
        1145913666 => 1,
        61791 => 1,
        3730735588 => 1,
        4278867 => 1,
        3774210530 => 1,
        538032816 => 1,
        1163413075 => 1,
        24053 => 1,
        4989 => 1,
        61267 => 1,
        61265 => 1,
        4076150800 => 1,
        16390 => 1,
        428016422 => 0,
        1702057286 => 0,
        1702057283 => 0,
        195894762 => 1,
        18225520 => 0,
        1196443219 => 0,
        16964 => 1,
        18475 => 1,
        18520 => 1,
        12648430 => 1,
        4187351113 => 1,
        2508478710 => 1,
        288389204 => 1,
        19993000 => 0,
        732765674 => 1,
        38496 => 1,
        16388 => 1,
        16384 => 1,
        1984 => 1,
        29366 => 1,
        827541066 => 1,
        1799439955 => 0,
        3380511080 => 1,
        198183888 => 0,
        1397109069 => 1,
        4991 => 1,
        5007 => 1,
        9320 => 1,
        9336 => 1,
        19802 => 1,
        427819522 => 1,
        19780 => 1,
        22092 => 0,
        26985 => 0,
        1852207972 => 0,
        13364 => 1,
        1853056627 => 1,
        1397118030 => 1,
        40865 => 1,
        1952539503 => 0,
        2035054128 => 0,
        2866260714 => 0,
        1346978886 => 1,
        1346981957 => 0,
        3344373136 => 1,
        2088527475 => 0,
        40864 => 1,
        1634035564 => 1,
        47 => 1,
        1746473250 => 1,
        2240043254 => 1,
        124082209 => 1,
        1382369651 => 1,
        29301 => 1,
        1733912937 => 1,
        1573531125 => 1,
        1397048141 => 1,
        1935894131 => 1,
        4185718668 => 1,
        1128357203 => 1,
        20859 => 0,
        4266872130 => 0,
        3203391149 => 0,
        1397703499 => 1,
        1936814952 => 1,
        1650812274 => 1,
        19920822 => 1,
        19920821 => 1,
        16914836 => 1,
        1953653091 => 1,
        604313861 => 1,
        352400198 => 1,
        72020 => 1,
        1410924800 => 1,
        40866 => 1,
        16914839 => 1,
        2020557398 => 0,
        3133910204 => 0,
        2768370933 => 0,
        1448756819 => 1,
        1397114950 => 1,
        2881100148 => 1,
        19920820 => 1,
        1481003842 => 1,
        19911021 => 1,
        51 => 1,
        801189825 => 1,
        1515144787 => 1,
        1479104553 => 1,
        _ => -1,
    }
}

static mut follow_mode_string: [*const libc::c_char; 3] = [
    b"descriptor\0" as *const u8 as *const libc::c_char,
    b"name\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut follow_mode_map: [Follow_mode; 2] = [Follow_descriptor, Follow_name];
static mut reopen_inaccessible_files: bool = false;
static mut count_lines: bool = false;
static mut follow_mode: Follow_mode = Follow_descriptor;
static mut forever: bool = false;
static mut monitor_output: bool = false;
static mut from_start: bool = false;
static mut print_headers: bool = false;
static mut line_end: libc::c_char = 0;
static mut max_n_unchanged_stats_between_opens: uintmax_t = 5 as libc::c_int
    as uintmax_t;
static mut nbpids: libc::c_int = 0 as libc::c_int;
static mut pids: *mut pid_t = 0 as *const pid_t as *mut pid_t;
static mut pids_alloc: idx_t = 0;
static mut page_size: idx_t = 0;
static mut have_read_stdin: bool = false;
static mut presume_input_pipe: bool = false;
static mut disable_inotify: bool = false;
static mut long_options: [option; 16] = [
    {
        let mut init = option {
            name: b"bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"follow\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: LONG_FOLLOW_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"lines\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-unchanged-stats\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: MAX_UNCHANGED_STATS_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"-disable-inotify\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: DISABLE_INOTIFY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"pid\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PID_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"-presume-input-pipe\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESUME_INPUT_PIPE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"retry\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: RETRY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"silent\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"sleep-interval\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"zero-terminated\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'z' as i32,
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
#[no_mangle]
pub fn usage(status: i32) {
    if status != 0 {
        eprintln!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext(format!("Try '{} --help' for more information.\n", unsafe { std::ffi::CStr::from_ptr(program_name).to_string_lossy() }).as_ptr() as *const i8)).to_string_lossy() }
        );
    } else {
        println!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext(format!("Usage: {} [OPTION]... [FILE]...\n", unsafe { std::ffi::CStr::from_ptr(program_name).to_string_lossy() }).as_ptr() as *const i8)).to_string_lossy() }
        );
        println!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext(format!("Print the last {} lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n", 10).as_ptr() as *const i8)).to_string_lossy() }
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        print!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext("  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext("  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means 'descriptor'\n  -F                       same as --follow=name --retry\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        println!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext(format!("  -n, --lines=[+]NUM       output the last NUM lines, instead of the last {};\n                             or use -n +NUM to skip NUM-1 lines at the start\n", 10).as_ptr() as *const i8)).to_string_lossy() }
        );
        println!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext(format!("      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default {}) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n", 5).as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext("      --pid=PID            with -f, terminate after process ID, PID dies;\n                             can be repeated to watch multiple processes\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext("  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext("  -z, --zero-terminated    line delimiter is NUL, not newline\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext("      --help        display this help and exit\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext("      --version     output version information and exit\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext("\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y, R, Q.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(gettext("With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        emit_ancillary_info("tail");
    }
    std::process::exit(status);
}

fn die_pipe() {
    std::process::exit(1);
}

fn check_output_alive() {
    if !unsafe { monitor_output } {
        return;
    }
    let result = unsafe { iopoll(-1, 1, false) };
    if result == -2 {
        die_pipe();
    }
}

unsafe extern "C" fn valid_file_spec(mut f: *const File_spec) -> bool {
    return ((*f).fd == -(1 as libc::c_int)) as libc::c_int
        ^ ((*f).errnum == 0 as libc::c_int) as libc::c_int != 0;
}
unsafe extern "C" fn pretty_name(mut f: *const File_spec) -> *const libc::c_char {
    return if strcmp((*f).name, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        gettext(b"standard input\0" as *const u8 as *const libc::c_char)
    } else {
        (*f).name
    };
}
fn record_open_fd(
    f: &mut File_spec,
    fd: libc::c_int,
    size: off_t,
    st: &stat,
    blocking: libc::c_int,
) {
    f.fd = fd;
    f.size = size;
    f.mtime = get_stat_mtime(st);
    f.dev = st.st_dev;
    f.ino = st.st_ino;
    f.mode = st.st_mode;
    f.blocking = blocking;
    f.n_unchanged_stats = 0;
    f.ignore = false;
}

unsafe extern "C" fn close_fd(mut fd: libc::c_int, mut filename: *const libc::c_char) {
    if fd != -(1 as libc::c_int) && fd != 0 as libc::c_int && close(fd) != 0 {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"closing %s (fd=%d)\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, filename),
                fd,
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
                    gettext(b"closing %s (fd=%d)\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                    fd,
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
                    gettext(b"closing %s (fd=%d)\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                    fd,
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
}
unsafe extern "C" fn write_header(mut pretty_filename: *const libc::c_char) {
    static mut first_file: bool = 1 as libc::c_int != 0;
    printf(
        b"%s==> %s <==\n\0" as *const u8 as *const libc::c_char,
        if first_file as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"\n\0" as *const u8 as *const libc::c_char
        },
        pretty_filename,
    );
    first_file = 0 as libc::c_int != 0;
}
unsafe extern "C" fn xwrite_stdout(
    mut buffer: *const libc::c_char,
    mut n_bytes: size_t,
) {
    if n_bytes > 0 as libc::c_int as libc::c_ulong
        && fwrite_unlocked(
            buffer as *const libc::c_void,
            1 as libc::c_int as size_t,
            n_bytes,
            stdout,
        ) < n_bytes
    {
        clearerr_unlocked(stdout);
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(b"error writing %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(
                    shell_escape_always_quoting_style,
                    b"standard output\0" as *const u8 as *const libc::c_char,
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
                    gettext(b"error writing %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"standard output\0" as *const u8 as *const libc::c_char,
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
                    gettext(b"error writing %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"standard output\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
}
unsafe extern "C" fn dump_remainder(
    mut want_header: bool,
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_bytes: uintmax_t,
) -> uintmax_t {
    let mut n_written: uintmax_t = 0;
    let mut n_remaining: uintmax_t = n_bytes;
    n_written = 0 as libc::c_int as uintmax_t;
    loop {
        let mut buffer: [libc::c_char; 8192] = [0; 8192];
        let mut n: size_t = if n_remaining < 8192 as libc::c_int as libc::c_ulong {
            n_remaining
        } else {
            8192 as libc::c_int as libc::c_ulong
        };
        let mut bytes_read: size_t = safe_read(
            fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            n,
        );
        if bytes_read == -(1 as libc::c_int) as size_t {
            if *__errno_location() != 11 as libc::c_int {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_filename,
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
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_filename,
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
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_filename,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            break;
        } else {
            if bytes_read == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            if want_header {
                write_header(pretty_filename);
                want_header = 0 as libc::c_int != 0;
            }
            xwrite_stdout(buffer.as_mut_ptr(), bytes_read);
            n_written = (n_written as libc::c_ulong).wrapping_add(bytes_read)
                as uintmax_t as uintmax_t;
            if !(n_bytes != 18446744073709551615 as libc::c_ulong) {
                continue;
            }
            n_remaining = (n_remaining as libc::c_ulong).wrapping_sub(bytes_read)
                as uintmax_t as uintmax_t;
            if n_remaining == 0 as libc::c_int as libc::c_ulong
                || n_bytes
                    == (18446744073709551615 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                break;
            }
        }
    }
    return n_written;
}
unsafe extern "C" fn xlseek(
    mut fd: libc::c_int,
    mut offset: off_t,
    mut whence: libc::c_int,
    mut filename: *const libc::c_char,
) -> off_t {
    let mut new_offset: off_t = lseek(fd, offset, whence);
    let mut buf: [libc::c_char; 21] = [0; 21];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 as libc::c_int as libc::c_long <= new_offset {
        return new_offset;
    }
    s = offtostr(offset, buf.as_mut_ptr());
    match whence {
        0 => {
            if false {
    error(
        1,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        gettext(b"%s: cannot seek to offset %s\0" as *const u8 as *const libc::c_char),
        quotearg_n_style_colon(0, shell_escape_quoting_style, filename),
        s,
    );
    unreachable!();
} else {
    let __errstatus: i32 = 1;
    error(
        __errstatus,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        gettext(b"%s: cannot seek to offset %s\0" as *const u8 as *const libc::c_char),
        quotearg_n_style_colon(0, shell_escape_quoting_style, filename),
        s,
    );
    unreachable!();
    
    let __errstatus: i32 = 1;
    error(
        __errstatus,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        gettext(b"%s: cannot seek to offset %s\0" as *const u8 as *const libc::c_char),
        quotearg_n_style_colon(0, shell_escape_quoting_style, filename),
        s,
    );
    unreachable!();
}

        }
        1 => {
            if false {
    error(
        1,
        *__errno_location(),
        gettext(b"%s: cannot seek to relative offset %s\0" as *const u8 as *const libc::c_char),
        quotearg_n_style_colon(0, shell_escape_quoting_style, filename),
        s,
    );
    unreachable!();
} else {
    let __errstatus: i32 = 1;
    error(
        __errstatus,
        *__errno_location(),
        gettext(b"%s: cannot seek to relative offset %s\0" as *const u8 as *const libc::c_char),
        quotearg_n_style_colon(0, shell_escape_quoting_style, filename),
        s,
    );
    unreachable!();
    
    let __errstatus: i32 = 1;
    error(
        __errstatus,
        *__errno_location(),
        gettext(b"%s: cannot seek to relative offset %s\0" as *const u8 as *const libc::c_char),
        quotearg_n_style_colon(0, shell_escape_quoting_style, filename),
        s,
    );
    unreachable!();
}

        }
        2 => {
            if false {
    error(
        1,
        *__errno_location(),
        gettext(b"%s: cannot seek to end-relative offset %s\0" as *const u8 as *const libc::c_char),
        quotearg_n_style_colon(0, shell_escape_quoting_style, filename),
        s,
    );
    unreachable!();
} else {
    let errstatus: i32 = 1;
    error(
        errstatus,
        *__errno_location(),
        gettext(b"%s: cannot seek to end-relative offset %s\0" as *const u8 as *const libc::c_char),
        quotearg_n_style_colon(0, shell_escape_quoting_style, filename),
        s,
    );
    unreachable!();
    
    let errstatus: i32 = 1;
    error(
        errstatus,
        *__errno_location(),
        gettext(b"%s: cannot seek to end-relative offset %s\0" as *const u8 as *const libc::c_char),
        quotearg_n_style_colon(0, shell_escape_quoting_style, filename),
        s,
    );
    unreachable!();
}

        }
        _ => {
            unreachable!();
        }
    }
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn file_lines(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut sb: *const stat,
    mut n_lines: uintmax_t,
    mut start_pos: off_t,
    mut end_pos: off_t,
    mut read_pos: *mut uintmax_t,
) -> bool {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bytes_read: size_t = 0;
    let mut bufsize: blksize_t = 8192 as libc::c_int;
    let mut pos: off_t = end_pos;
    let mut ok: bool = 1 as libc::c_int != 0;
    if n_lines == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    if (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"((((sb->st_mode)) & 0170000) == (0100000))\0" as *const u8
                as *const libc::c_char,
            b"tail.c\0" as *const u8 as *const libc::c_char,
            543 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"_Bool file_lines(const char *, int, const struct stat *, uintmax_t, off_t, off_t, uintmax_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10359: {
        if (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"((((sb->st_mode)) & 0170000) == (0100000))\0" as *const u8
                    as *const libc::c_char,
                b"tail.c\0" as *const u8 as *const libc::c_char,
                543 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"_Bool file_lines(const char *, int, const struct stat *, uintmax_t, off_t, off_t, uintmax_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*sb).st_size % page_size == 0 as libc::c_int as libc::c_long {
        bufsize = (if 8192 as libc::c_int as libc::c_long > page_size {
            8192 as libc::c_int as libc::c_long
        } else {
            page_size
        }) as blksize_t;
    }
    buffer = xmalloc(bufsize as size_t) as *mut libc::c_char;
    bytes_read = ((pos - start_pos) % bufsize as libc::c_long) as size_t;
    if bytes_read == 0 as libc::c_int as libc::c_ulong {
        bytes_read = bufsize as size_t;
    }
    pos = (pos as libc::c_ulong).wrapping_sub(bytes_read) as off_t as off_t;
    xlseek(fd, pos, 0 as libc::c_int, pretty_filename);
    bytes_read = safe_read(fd, buffer as *mut libc::c_void, bytes_read);
    if bytes_read == -(1 as libc::c_int) as size_t {
        if false {
    error(
        0,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        gettext(b"error reading %s\0".as_ptr() as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_filename),
    );
    if false {
        unreachable!();
    }
} else {
    let __errstatus: i32 = 0;
    error(
        __errstatus,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        gettext(b"error reading %s\0".as_ptr() as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_filename),
    );
    if __errstatus != 0 {
        unreachable!();
    }

    let __errstatus: i32 = 0;
    error(
        __errstatus,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        gettext(b"error reading %s\0".as_ptr() as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_filename),
    );
    if __errstatus != 0 {
        unreachable!();
    }
}
ok = false;

    } else {
        *read_pos = (pos as libc::c_ulong).wrapping_add(bytes_read);
        if bytes_read != 0
            && *buffer
                .offset(
                    bytes_read.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int != line_end as libc::c_int
        {
            n_lines = n_lines.wrapping_sub(1);
            n_lines;
        }
        's_79: loop {
            let mut n: size_t = bytes_read;
            while n != 0 {
                let mut nl: *const libc::c_char = 0 as *const libc::c_char;
                nl = memrchr(buffer as *const libc::c_void, line_end as libc::c_int, n)
                    as *const libc::c_char;
                if nl.is_null() {
                    break;
                }
                n = nl.offset_from(buffer) as libc::c_long as size_t;
                let fresh0 = n_lines;
                n_lines = n_lines.wrapping_sub(1);
                if !(fresh0 == 0 as libc::c_int as libc::c_ulong) {
                    continue;
                }
                xwrite_stdout(
                    nl.offset(1 as libc::c_int as isize),
                    bytes_read
                        .wrapping_sub(n.wrapping_add(1 as libc::c_int as libc::c_ulong)),
                );
                *read_pos = (*read_pos as libc::c_ulong)
                    .wrapping_add(
                        dump_remainder(
                            0 as libc::c_int != 0,
                            pretty_filename,
                            fd,
                            (end_pos as libc::c_ulong)
                                .wrapping_sub(
                                    (pos as libc::c_ulong).wrapping_add(bytes_read),
                                ),
                        ),
                    ) as uintmax_t as uintmax_t;
                break 's_79;
            }
            if pos == start_pos {
                xlseek(fd, start_pos, 0 as libc::c_int, pretty_filename);
                *read_pos = (start_pos as libc::c_ulong)
                    .wrapping_add(
                        dump_remainder(
                            0 as libc::c_int != 0,
                            pretty_filename,
                            fd,
                            end_pos as uintmax_t,
                        ),
                    );
                break;
            } else {
                pos -= bufsize as libc::c_long;
                xlseek(fd, pos, 0 as libc::c_int, pretty_filename);
                bytes_read = safe_read(
                    fd,
                    buffer as *mut libc::c_void,
                    bufsize as size_t,
                );
                if bytes_read == -(1 as libc::c_int) as size_t {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_filename,
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
                                *__errno_location(),
                                gettext(
                                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    pretty_filename,
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
                                *__errno_location(),
                                gettext(
                                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    pretty_filename,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    ok = 0 as libc::c_int != 0;
                    break;
                } else {
                    *read_pos = (pos as libc::c_ulong).wrapping_add(bytes_read);
                    if !(bytes_read > 0 as libc::c_int as libc::c_ulong) {
                        break;
                    }
                }
            }
        }
    }
    free(buffer as *mut libc::c_void);
    return ok;
}
fn pipe_lines(
    pretty_filename: *const libc::c_char,
    fd: i32,
    n_lines: u64,
    read_pos: *mut u64,
) -> bool {
    unsafe {
        // Convert pretty_filename to a C-compatible string
        let c_filename = std::ffi::CStr::from_ptr(pretty_filename);

         let mut first: *mut LBUFFER = 0 as *mut LBUFFER;
    let mut last: *mut LBUFFER = 0 as *mut LBUFFER;
    let mut tmp: *mut LBUFFER = 0 as *mut LBUFFER;
    let mut total_lines: size_t = 0 as libc::c_int as size_t;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut n_read: size_t = 0;
    last = xmalloc(::core::mem::size_of::<LBUFFER>() as libc::c_ulong) as *mut LBUFFER;
    first = last;
    (*first).nlines = 0 as libc::c_int as size_t;
    (*first).nbytes = (*first).nlines;
    (*first).next = 0 as *mut linebuffer;
    tmp = xmalloc(::core::mem::size_of::<LBUFFER>() as libc::c_ulong) as *mut LBUFFER;
    loop {
        n_read = safe_read(
            fd,
            ((*tmp).buffer).as_mut_ptr() as *mut libc::c_void,
            8192 as libc::c_int as size_t,
        );
        if n_read == 0 as libc::c_int as libc::c_ulong
            || n_read == -(1 as libc::c_int) as size_t
        {
            break;
        }
        (*tmp).nbytes = n_read;
        *read_pos = (*read_pos as libc::c_ulong).wrapping_add(n_read) as uintmax_t
            as uintmax_t;
        (*tmp).nlines = 0 as libc::c_int as size_t;
        (*tmp).next = 0 as *mut linebuffer;
        let mut buffer_end: *const libc::c_char = ((*tmp).buffer)
            .as_mut_ptr()
            .offset(n_read as isize);
        let mut p: *const libc::c_char = ((*tmp).buffer).as_mut_ptr();
        loop {
            p = memchr(
                p as *const libc::c_void,
                line_end as libc::c_int,
                buffer_end.offset_from(p) as libc::c_long as libc::c_ulong,
            ) as *const libc::c_char;
            if p.is_null() {
                break;
            }
            p = p.offset(1);
            p;
            (*tmp).nlines = ((*tmp).nlines).wrapping_add(1);
            (*tmp).nlines;
        }
        total_lines = (total_lines as libc::c_ulong).wrapping_add((*tmp).nlines)
            as size_t as size_t;
        if ((*tmp).nbytes).wrapping_add((*last).nbytes)
            < 8192 as libc::c_int as libc::c_ulong
        {
            memcpy(
                &mut *((*last).buffer).as_mut_ptr().offset((*last).nbytes as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                ((*tmp).buffer).as_mut_ptr() as *const libc::c_void,
                (*tmp).nbytes,
            );
            (*last)
                .nbytes = ((*last).nbytes as libc::c_ulong).wrapping_add((*tmp).nbytes)
                as size_t as size_t;
            (*last)
                .nlines = ((*last).nlines as libc::c_ulong).wrapping_add((*tmp).nlines)
                as size_t as size_t;
        } else {
            (*last).next = tmp;
            last = (*last).next;
            if total_lines.wrapping_sub((*first).nlines) > n_lines {
                tmp = first;
                total_lines = (total_lines as libc::c_ulong)
                    .wrapping_sub((*first).nlines) as size_t as size_t;
                first = (*first).next;
            } else {
                tmp = xmalloc(::core::mem::size_of::<LBUFFER>() as libc::c_ulong)
                    as *mut LBUFFER;
            }
        }
    }
    free(tmp as *mut libc::c_void);
   
         if n_read == -(1 as libc::c_int) as size_t {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        ok = 0 as libc::c_int != 0;
    } else if !((*last).nbytes == 0 as libc::c_int as libc::c_ulong) {
        if !(n_lines == 0 as libc::c_int as libc::c_ulong) {
            if (*last)
                .buffer[((*last).nbytes).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int != line_end as libc::c_int
            {
                (*last).nlines = ((*last).nlines).wrapping_add(1);
                (*last).nlines;
                total_lines = total_lines.wrapping_add(1);
                total_lines;
            }
            tmp = first;
            while total_lines.wrapping_sub((*tmp).nlines) > n_lines {
                total_lines = (total_lines as libc::c_ulong).wrapping_sub((*tmp).nlines)
                    as size_t as size_t;
                tmp = (*tmp).next;
            }
            let mut beg: *const libc::c_char = ((*tmp).buffer).as_mut_ptr();
            let mut buffer_end_0: *const libc::c_char = ((*tmp).buffer)
                .as_mut_ptr()
                .offset((*tmp).nbytes as isize);
            if total_lines > n_lines {
                let mut j: size_t = 0;
                j = total_lines.wrapping_sub(n_lines);
                while j != 0 {
                    beg = rawmemchr(beg as *const libc::c_void, line_end as libc::c_int)
                        as *const libc::c_char;
                    beg = beg.offset(1);
                    beg;
                    j = j.wrapping_sub(1);
                    j;
                }
            }
            xwrite_stdout(beg, buffer_end_0.offset_from(beg) as libc::c_long as size_t);
            tmp = (*tmp).next;
            while !tmp.is_null() {
                xwrite_stdout(((*tmp).buffer).as_mut_ptr(), (*tmp).nbytes);
                tmp = (*tmp).next;
            }
        }
    }
    while !first.is_null() {
        tmp = (*first).next;
        free(first as *mut libc::c_void);
        first = tmp;
    }
    return ok;

    }
}

unsafe extern "C" fn pipe_bytes(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_bytes: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> bool {
    let mut first: *mut CBUFFER = 0 as *mut CBUFFER;
    let mut last: *mut CBUFFER = 0 as *mut CBUFFER;
    let mut tmp: *mut CBUFFER = 0 as *mut CBUFFER;
    let mut i: size_t = 0;
    let mut total_bytes: size_t = 0 as libc::c_int as size_t;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut n_read: size_t = 0;
    last = xmalloc(::core::mem::size_of::<CBUFFER>() as libc::c_ulong) as *mut CBUFFER;
    first = last;
    (*first).nbytes = 0 as libc::c_int as size_t;
    (*first).next = 0 as *mut charbuffer;
    tmp = xmalloc(::core::mem::size_of::<CBUFFER>() as libc::c_ulong) as *mut CBUFFER;
    loop {
        n_read = safe_read(
            fd,
            ((*tmp).buffer).as_mut_ptr() as *mut libc::c_void,
            8192 as libc::c_int as size_t,
        );
        if n_read == 0 as libc::c_int as libc::c_ulong
            || n_read == -(1 as libc::c_int) as size_t
        {
            break;
        }
        *read_pos = (*read_pos as libc::c_ulong).wrapping_add(n_read) as uintmax_t
            as uintmax_t;
        (*tmp).nbytes = n_read;
        (*tmp).next = 0 as *mut charbuffer;
        total_bytes = (total_bytes as libc::c_ulong).wrapping_add((*tmp).nbytes)
            as size_t as size_t;
        if ((*tmp).nbytes).wrapping_add((*last).nbytes)
            < 8192 as libc::c_int as libc::c_ulong
        {
            memcpy(
                &mut *((*last).buffer).as_mut_ptr().offset((*last).nbytes as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                ((*tmp).buffer).as_mut_ptr() as *const libc::c_void,
                (*tmp).nbytes,
            );
            (*last)
                .nbytes = ((*last).nbytes as libc::c_ulong).wrapping_add((*tmp).nbytes)
                as size_t as size_t;
        } else {
            (*last).next = tmp;
            last = (*last).next;
            if total_bytes.wrapping_sub((*first).nbytes) > n_bytes {
                tmp = first;
                total_bytes = (total_bytes as libc::c_ulong)
                    .wrapping_sub((*first).nbytes) as size_t as size_t;
                first = (*first).next;
            } else {
                tmp = xmalloc(::core::mem::size_of::<CBUFFER>() as libc::c_ulong)
                    as *mut CBUFFER;
            }
        }
    }
    free(tmp as *mut libc::c_void);
    if n_read == -(1 as libc::c_int) as size_t {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        ok = 0 as libc::c_int != 0;
    } else {
        tmp = first;
        while total_bytes.wrapping_sub((*tmp).nbytes) > n_bytes {
            total_bytes = (total_bytes as libc::c_ulong).wrapping_sub((*tmp).nbytes)
                as size_t as size_t;
            tmp = (*tmp).next;
        }
        if total_bytes > n_bytes {
            i = total_bytes.wrapping_sub(n_bytes);
        } else {
            i = 0 as libc::c_int as size_t;
        }
        xwrite_stdout(
            &mut *((*tmp).buffer).as_mut_ptr().offset(i as isize),
            ((*tmp).nbytes).wrapping_sub(i),
        );
        tmp = (*tmp).next;
        while !tmp.is_null() {
            xwrite_stdout(((*tmp).buffer).as_mut_ptr(), (*tmp).nbytes);
            tmp = (*tmp).next;
        }
    }
    while !first.is_null() {
        tmp = (*first).next;
        free(first as *mut libc::c_void);
        first = tmp;
    }
    return ok;
}
unsafe extern "C" fn start_bytes(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_bytes: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> libc::c_int {
    let mut buffer: [libc::c_char; 8192] = [0; 8192];
    while (0 as libc::c_int as libc::c_ulong) < n_bytes {
        let mut bytes_read: size_t = safe_read(
            fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            8192 as libc::c_int as size_t,
        );
        if bytes_read == 0 as libc::c_int as libc::c_ulong {
            return -(1 as libc::c_int);
        }
        if bytes_read == -(1 as libc::c_int) as size_t {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_filename,
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
                        *__errno_location(),
                        gettext(
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_filename,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            return 1 as libc::c_int;
        }
        *read_pos = (*read_pos as libc::c_ulong).wrapping_add(bytes_read) as uintmax_t
            as uintmax_t;
        if bytes_read <= n_bytes {
            n_bytes = (n_bytes as libc::c_ulong).wrapping_sub(bytes_read) as uintmax_t
                as uintmax_t;
        } else {
            let mut n_remaining: size_t = bytes_read.wrapping_sub(n_bytes);
            xwrite_stdout(
                &mut *buffer.as_mut_ptr().offset(n_bytes as isize),
                n_remaining,
            );
            break;
        }
    }
    return 0 as libc::c_int;
}
fn start_lines(
    pretty_filename: *const libc::c_char,
    fd: libc::c_int,
    mut n_lines: u64,
    read_pos: *mut u64,
) -> libc::c_int {
    if n_lines == 0 {
        return 0;
    }
    
    let mut buffer = vec![0; 8192];
    
    loop {
        let bytes_read = unsafe { safe_read(fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len() as u64) };
        
        if bytes_read == 0 {
            return -1;
        }
        
        if bytes_read == u64::MAX {
            unsafe {
                error(
                    0,
                    *__errno_location(),
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
                );
            }
            return 1;
        }
        
        let buffer_end = bytes_read as usize;
        unsafe {
            *read_pos += bytes_read;
        }
        
        let mut p = 0;
        while p < buffer_end {
            if unsafe { buffer[p] } == unsafe { line_end } {
                p += 1;
                n_lines = n_lines.wrapping_sub(1);
                if n_lines == 0 {
                    if p < buffer_end {
                        let remaining = &buffer[p..buffer_end];
                        let remaining_u8: &[u8] = unsafe { std::slice::from_raw_parts(remaining.as_ptr() as *const u8, remaining.len()) };
                        std::io::stdout().write_all(remaining_u8).unwrap();
                    }
                    return 0;
                }
            } else {
                p += 1;
            }
        }
    }
}

unsafe extern "C" fn fremote(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
) -> bool {
    let mut remote: bool = 1 as libc::c_int != 0;
    let mut buf: statfs = statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: __fsid_t { __val: [0; 2] },
        f_namelen: 0,
        f_frsize: 0,
        f_flags: 0,
        f_spare: [0; 4],
    };
    let mut err: libc::c_int = fstatfs(fd, &mut buf);
    if err != 0 as libc::c_int {
        if *__errno_location() != 38 as libc::c_int {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"cannot determine location of %s. reverting to polling\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, name),
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
                            b"cannot determine location of %s. reverting to polling\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, name),
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
                            b"cannot determine location of %s. reverting to polling\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, name),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    } else {
        let remote = is_local_fs_type(buf.f_type as u64) <= 0;
    }
    return remote;
}
unsafe extern "C" fn recheck(mut f: *mut File_spec, mut blocking: bool) {
    let mut new_stats: stat = stat {
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
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut is_stdin: bool = strcmp(
        (*f).name,
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int;
    let mut was_tailable: bool = (*f).tailable;
    let mut prev_errnum: libc::c_int = (*f).errnum;
    let mut new_file: bool = false;
    let mut fd: libc::c_int = if is_stdin as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        open_safer(
            (*f).name,
            0 as libc::c_int
                | (if blocking as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    0o4000 as libc::c_int
                }),
        )
    };
    if valid_file_spec(f) {} else {
        __assert_fail(
            b"valid_file_spec (f)\0" as *const u8 as *const libc::c_char,
            b"tail.c\0" as *const u8 as *const libc::c_char,
            987 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void recheck(struct File_spec *, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_13353: {
        if valid_file_spec(f) {} else {
            __assert_fail(
                b"valid_file_spec (f)\0" as *const u8 as *const libc::c_char,
                b"tail.c\0" as *const u8 as *const libc::c_char,
                987 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void recheck(struct File_spec *, _Bool)\0"))
                    .as_ptr(),
            );
        }
    };
    (*f)
        .tailable = !(reopen_inaccessible_files as libc::c_int != 0
        && fd == -(1 as libc::c_int));
    if !disable_inotify && lstat((*f).name, &mut new_stats) == 0
        && new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
    {
        let mut ok = false;
(*f).errnum = -1;
(*f).ignore = true;

if false {
    error(
        0,
        0,
        gettext(b"%s has been replaced with an untailable symbolic link\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    unreachable!();
} else {
    let __errstatus = 0;
    error(
        __errstatus,
        0,
        gettext(b"%s has been replaced with an untailable symbolic link\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if __errstatus != 0 {
        unreachable!();
    }

    let __errstatus = 0;
    error(
        __errstatus,
        0,
        gettext(b"%s has been replaced with an untailable symbolic link\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if __errstatus != 0 {
        unreachable!();
    }
}

    } else if fd == -(1 as libc::c_int) || fstat(fd, &mut new_stats) < 0 as libc::c_int {
        let ok = false; // Assuming this is the intended logic since 0 as libc::c_int != 0 is always false
let errnum = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
let f_ref = unsafe { &mut *f }; // Dereference the raw pointer

f_ref.errnum = errnum;

if !f_ref.tailable {
    if was_tailable {
        if false { // This condition is always false
            error(
                0,
                f_ref.errnum,
                gettext(b"%s has become inaccessible\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
            );
            unreachable!();
        } else {
            let __errstatus = 0;
            error(
                __errstatus,
                f_ref.errnum,
                gettext(b"%s has become inaccessible\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
            );
            if __errstatus != 0 {
                unreachable!();
            }

            let __errstatus = 0;
            error(
                __errstatus,
                f_ref.errnum,
                gettext(b"%s has become inaccessible\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
            );
            if __errstatus != 0 {
                unreachable!();
            }
        }
    }
} else if prev_errnum != errnum {
    if false { // This condition is always false
        error(
            0,
            errnum,
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style_colon(0, shell_escape_quoting_style, pretty_name(f)),
        );
        unreachable!();
    } else {
        let __errstatus = 0;
        error(
            __errstatus,
            errnum,
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style_colon(0, shell_escape_quoting_style, pretty_name(f)),
        );
        if __errstatus != 0 {
            unreachable!();
        }

        let __errstatus = 0;
        error(
            __errstatus,
            errnum,
            b"%s\0" as *const u8 as *const libc::c_char,
            quotearg_n_style_colon(0, shell_escape_quoting_style, pretty_name(f)),
        );
        if __errstatus != 0 {
            unreachable!();
        }
    }
}

    } else if !(new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint
        || new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o140000 as libc::c_int as libc::c_uint
        || new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint)
    {
        ok = 0 as libc::c_int != 0;
        (*f).errnum = -(1 as libc::c_int);
        (*f).tailable = 0 as libc::c_int != 0;
        (*f)
            .ignore = !(reopen_inaccessible_files as libc::c_int != 0
            && follow_mode as libc::c_uint
                == Follow_name as libc::c_int as libc::c_uint);
        if was_tailable as libc::c_int != 0 || prev_errnum != (*f).errnum {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s has been replaced with an untailable file%s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
                    if (*f).ignore as libc::c_int != 0 {
                        gettext(
                            b"; giving up on this name\0" as *const u8
                                as *const libc::c_char,
                        ) as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
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
                            b"%s has been replaced with an untailable file%s\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_name(f),
                        ),
                        if (*f).ignore as libc::c_int != 0 {
                            gettext(
                                b"; giving up on this name\0" as *const u8
                                    as *const libc::c_char,
                            ) as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
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
                            b"%s has been replaced with an untailable file%s\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_name(f),
                        ),
                        if (*f).ignore as libc::c_int != 0 {
                            gettext(
                                b"; giving up on this name\0" as *const u8
                                    as *const libc::c_char,
                            ) as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    } else {
        let remote = fremote(fd, pretty_name(f));
if remote && !disable_inotify {
    ok = false;
    (*f).errnum = -1;
    error(
        0,
        0,
        gettext(
            b"%s has been replaced with an untailable remote file\0" as *const u8 as *const libc::c_char,
        ),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if false {
        unreachable!();
    }
    (*f).ignore = true;
    (*f).remote = true;
} else {
    (*f).errnum = 0;
}

    }
    new_file = 0 as libc::c_int != 0;
    if !ok {
        close_fd(fd, pretty_name(f));
        close_fd((*f).fd, pretty_name(f));
        (*f).fd = -(1 as libc::c_int);
    } else if prev_errnum != 0 && prev_errnum != 2 as libc::c_int {
        new_file = 1 as libc::c_int != 0;
        if (*f).fd == -(1 as libc::c_int) {} else {
            __assert_fail(
                b"f->fd == -1\0" as *const u8 as *const libc::c_char,
                b"tail.c\0" as *const u8 as *const libc::c_char,
                1064 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void recheck(struct File_spec *, _Bool)\0"))
                    .as_ptr(),
            );
        }
        'c_12481: {
            if (*f).fd == -(1 as libc::c_int) {} else {
                __assert_fail(
                    b"f->fd == -1\0" as *const u8 as *const libc::c_char,
                    b"tail.c\0" as *const u8 as *const libc::c_char,
                    1064 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 40],
                        &[libc::c_char; 40],
                    >(b"void recheck(struct File_spec *, _Bool)\0"))
                        .as_ptr(),
                );
            }
        };
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s has become accessible\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"%s has become accessible\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"%s has become accessible\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    } else if (*f).fd == -(1 as libc::c_int) {
        let new_file = true;

if false {
    error(
        0,
        0,
        gettext(b"%s has appeared;  following new file\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    unreachable!();
} else {
    let errstatus = 0;
    error(
        errstatus,
        0,
        gettext(b"%s has appeared;  following new file\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if errstatus != 0 {
        unreachable!();
    }

    let errstatus = 0;
    error(
        errstatus,
        0,
        gettext(b"%s has appeared;  following new file\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if errstatus != 0 {
        unreachable!();
    }
}

    } else if (*f).ino != new_stats.st_ino || (*f).dev != new_stats.st_dev {
        let new_file = true;

if false {
    error(
        0,
        0,
        gettext(b"%s has been replaced; following new file\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if false {
        unreachable!();
    }
} else {
    let errstatus = 0;
    error(
        errstatus,
        0,
        gettext(b"%s has been replaced; following new file\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if errstatus != 0 {
        unreachable!();
    }

    let errstatus = 0;
    error(
        errstatus,
        0,
        gettext(b"%s has been replaced; following new file\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if errstatus != 0 {
        unreachable!();
    }
}

close_fd(unsafe { (*f).fd }, pretty_name(f));

    } else {
        close_fd(fd, pretty_name(f));
    }
    if new_file {
        let blocking_value = if is_stdin as libc::c_int != 0 {
    -(1 as libc::c_int)
} else {
    blocking as libc::c_int
};
record_open_fd(
    &mut *f,
    fd,
    0,
    &new_stats,
    blocking_value,
);
        if new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        {
            xlseek(fd, 0 as libc::c_int as off_t, 0 as libc::c_int, pretty_name(f));
        }
    }
}
unsafe extern "C" fn any_live_files(
    mut f: *const File_spec,
    mut n_files: size_t,
) -> bool {
    if reopen_inaccessible_files as libc::c_int != 0
        && follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_files {
        if 0 as libc::c_int <= (*f.offset(i as isize)).fd {
            return 1 as libc::c_int != 0
        } else if !(*f.offset(i as isize)).ignore
            && reopen_inaccessible_files as libc::c_int != 0
        {
            return 1 as libc::c_int != 0
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
fn writers_are_dead() -> bool {
    unsafe {
        if nbpids == 0 {
            return false;
        }
        for i in 0..nbpids {
            let pid = *pids.offset(i as isize); // Accessing the raw pointer safely
            if std::process::Command::new("kill").arg("-0").arg(pid.to_string()).status().is_err() {
                return false;
            }
        }
    }
    true
}

unsafe extern "C" fn tail_forever(
    mut f: *mut File_spec,
    mut n_files: size_t,
    mut sleep_interval: libc::c_double,
) {
    let blocking: bool = nbpids == 0
    && follow_mode == Follow_descriptor
    && n_files == 1
    && unsafe { (*f).fd } != -1
    && (unsafe { (*f).mode & 0o170000 } != 0o100000);

let mut last: u64 = n_files.wrapping_sub(1);
let mut writers_dead: bool = false;

loop {
     /*
The variables live at this point are:
(mut f: *mut File_spec, mut n_files: u64, mut sleep_interval: f64, mut blocking: bool, mut last: u64, mut writers_dead: bool)
*/
let mut i: u64 = 0;
let mut any_input: bool = false;
let mut current_block_47: u64;
i = 0;
while i < n_files {
     let mut fd: i32 = 0;
let mut name: *const libc::c_char = std::ptr::null();
let mut mode: u32 = 0;
let mut stats: stat = unsafe { std::mem::zeroed() }; // Initialize to zero
let mut bytes_read: u64 = 0;

if !unsafe { (*f.offset(i as isize)).ignore } {
    if unsafe { (*f.offset(i as isize)).fd } < 0 {
        recheck(&mut *f.offset(i as isize), blocking);
    } else {
         fd = (*f.offset(i as isize)).fd;
                    name = pretty_name(&mut *f.offset(i as isize));
                    mode = (*f.offset(i as isize)).mode;
                    if (*f.offset(i as isize)).blocking != blocking as libc::c_int {
                        let mut old_flags: libc::c_int = rpl_fcntl(fd, 3 as libc::c_int);
                        let mut new_flags: libc::c_int = old_flags
                            | (if blocking as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                0o4000 as libc::c_int
                            });
                        if old_flags < 0 as libc::c_int
                            || new_flags != old_flags
                                && rpl_fcntl(fd, 4 as libc::c_int, new_flags)
                                    == -(1 as libc::c_int)
                        {
                            if !((*f.offset(i as isize)).mode
                                & 0o170000 as libc::c_int as libc::c_uint
                                == 0o100000 as libc::c_int as libc::c_uint
                                && *__errno_location() == 1 as libc::c_int)
                            {
                                if 0 != 0 {
                                    error(
                                        1 as libc::c_int,
                                        *__errno_location(),
                                        gettext(
                                            b"%s: cannot change nonblocking mode\0" as *const u8
                                                as *const libc::c_char,
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
                                                b"%s: cannot change nonblocking mode\0" as *const u8
                                                    as *const libc::c_char,
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
                                                b"%s: cannot change nonblocking mode\0" as *const u8
                                                    as *const libc::c_char,
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
                        } else {
                            (*f.offset(i as isize)).blocking = blocking as libc::c_int;
                        }
                    }
                    let mut read_unchanged: bool = 0 as libc::c_int != 0;
                    if (*f.offset(i as isize)).blocking == 0 {
                        if fstat(fd, &mut stats) != 0 as libc::c_int {
                            let file_spec = &mut *f.offset(i as isize);
file_spec.fd = -1;
file_spec.errnum = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);

if false {
    error(
        0,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        b"%s\0".as_ptr() as *const i8,
        quotearg_n_style_colon(0, shell_escape_quoting_style, name),
    );
    if false {
        unreachable!();
    }
} else {
    let errstatus = 0;
    error(
        errstatus,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        b"%s\0".as_ptr() as *const i8,
        quotearg_n_style_colon(0, shell_escape_quoting_style, name),
    );
    if errstatus != 0 {
        unreachable!();
    }

    let errstatus = 0;
    error(
        errstatus,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        b"%s\0".as_ptr() as *const i8,
        quotearg_n_style_colon(0, shell_escape_quoting_style, name),
    );
    if errstatus != 0 {
        unreachable!();
    }
}

close(fd);
current_block_47 = 17778012151635330486;

                        } else {
                            let file_spec = unsafe { &mut *f.offset(i as isize) };
if file_spec.mode == stats.st_mode
    && (!(stats.st_mode & 0o170000 == 0o100000)
        || file_spec.size == stats.st_size)
    && timespec_cmp(file_spec.mtime, get_stat_mtime(&stats)) == 0
{
    let fresh1 = &mut file_spec.n_unchanged_stats;
    let fresh2 = *fresh1;
    *fresh1 = fresh1.wrapping_add(1);
    
    if max_n_unchanged_stats_between_opens <= fresh2
        && follow_mode == Follow_name
    {
        recheck(file_spec, file_spec.blocking != 0);
        file_spec.n_unchanged_stats = 0;
    }
    
    if fd != file_spec.fd
        || stats.st_mode & 0o170000 == 0o100000
        || 1 < n_files
    {
        current_block_47 = 17778012151635330486;
    } else {
        read_unchanged = true;
        current_block_47 = 8693738493027456495;
    }
} else {
    current_block_47 = 8693738493027456495;
}

                            match current_block_47 {
    17778012151635330486 => {}
    _ => {
        let file = &mut *f.offset(i as isize);
        if fd != file.fd {
            panic!("fd == f[i].fd");
        }

        file.mtime = get_stat_mtime(&mut stats);
        file.mode = stats.st_mode;

        if !read_unchanged {
            file.n_unchanged_stats = 0;
        }

        if (mode & 0o170000) == 0o100000 && stats.st_size < file.size {
            error(
                0,
                0,
                gettext(b"%s: file truncated\0" as *const u8 as *const libc::c_char),
                quotearg_n_style_colon(0, shell_escape_quoting_style, name),
            );

            xlseek(fd, 0, 0, name);
            file.size = 0;
        }

        if i != last {
            if print_headers {
                write_header(name);
            }
            last = i;
        }
        current_block_47 = 3222590281903869779;
    }
}

                        }
                    } else {
                        current_block_47 = 3222590281903869779;
                    }
                    match current_block_47 {
                        17778012151635330486 => {}
                        _ => {
                            let mut bytes_to_read: uintmax_t = 0;
                            if (*f.offset(i as isize)).blocking != 0 {
                                bytes_to_read = (18446744073709551615 as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                            } else if mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o100000 as libc::c_int as libc::c_uint
                                && (*f.offset(i as isize)).remote as libc::c_int != 0
                            {
                                bytes_to_read = (stats.st_size
                                    - (*f.offset(i as isize)).size) as uintmax_t;
                            } else {
                                bytes_to_read = 18446744073709551615 as libc::c_ulong;
                            }
                            bytes_read = dump_remainder(
                                0 as libc::c_int != 0,
                                name,
                                fd,
                                bytes_to_read,
                            );
                            if read_unchanged as libc::c_int != 0 && bytes_read != 0 {
                                (*f.offset(i as isize))
                                    .n_unchanged_stats = 0 as libc::c_int as uintmax_t;
                            }
                            any_input = (any_input as libc::c_int
                                | (bytes_read != 0 as libc::c_int as libc::c_ulong)
                                    as libc::c_int) != 0;
                            let ref mut fresh3 = (*f.offset(i as isize)).size;
                            *fresh3 = (*fresh3 as libc::c_ulong).wrapping_add(bytes_read)
                                as off_t as off_t;
                        }
                    }

    }
}
i = i.wrapping_add(1);
i;


}
if !any_live_files(f, n_files) {
    let errstatus: libc::c_int = 0;
    error(
        errstatus,
        0,
        gettext(b"no files remaining\0" as *const u8 as *const libc::c_char),
    );
    if errstatus != 0 {
        unreachable!();
    }
    break;
} else {
    if (!any_input || blocking) && fflush_unlocked(stdout) != 0 {
        write_error();
    }
    check_output_alive();
    if any_input {
        continue;
    }
    if writers_dead {
        break;
    }
    writers_dead = writers_are_dead();
    if !writers_dead && xnanosleep(sleep_interval) != 0 {
        let errstatus: libc::c_int = 1;
        error(
            errstatus,
            *__errno_location(),
            gettext(b"cannot read realtime clock\0" as *const u8 as *const libc::c_char),
        );
        if errstatus != 0 {
            unreachable!();
        }
    }
}
/*
The variables live at this point are:
(mut f: *mut File_spec, mut n_files: u64, mut sleep_interval: f64, mut blocking: bool, mut last: u64, mut writers_dead: bool, mut i: u64, mut any_input: bool, mut current_block_47: u64, writers_dead: bool)
*/


};
/*
The variables live at this point are:
(mut f: *mut File_spec, mut n_files: u64, mut sleep_interval: f64)
*/

}
fn any_remote_file(f: *const File_spec, n_files: u64) -> bool {
    let n_files_usize: usize = n_files.try_into().expect("Conversion to usize failed");
    let slice = unsafe { std::slice::from_raw_parts(f, n_files_usize) };
    for file in slice.iter() {
        if file.fd >= 0 && file.remote {
            return true;
        }
    }
    false
}

fn any_non_remote_file(f: *const File_spec, n_files: u64) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(f, n_files as usize) };
    for file in slice.iter() {
        if file.fd >= 0 && !file.remote {
            return true;
        }
    }
    false
}

unsafe extern "C" fn any_symlinks(mut f: *const File_spec, mut n_files: size_t) -> bool {
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
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_files {
        if lstat((*f.offset(i as isize)).name, &mut st) == 0 as libc::c_int
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
fn any_non_regular_fifo(f: *const File_spec, n_files: u64) -> bool {
    let n_files_usize: usize = n_files.try_into().unwrap();
    let slice = unsafe { std::slice::from_raw_parts(f, n_files_usize) };
    for file in slice.iter() {
        if file.fd >= 0
            && !(file.mode & 0o170000 == 0o100000)
            && !(file.mode & 0o170000 == 0o10000)
        {
            return true;
        }
    }
    false
}

unsafe extern "C" fn tailable_stdin(
    mut f: *const File_spec,
    mut n_files: size_t,
) -> bool {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_files {
        if !(*f.offset(i as isize)).ignore
            && strcmp(
                (*f.offset(i as isize)).name,
                b"-\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn wd_hasher(
    mut entry: *const libc::c_void,
    mut tabsize: size_t,
) -> size_t {
    let mut spec: *const File_spec = entry as *const File_spec;
    return ((*spec).wd as libc::c_ulong).wrapping_rem(tabsize);
}
unsafe extern "C" fn wd_comparator(
    e1: *const libc::c_void,
    e2: *const libc::c_void,
) -> bool {
    let spec1 = &*(e1 as *const File_spec);
    let spec2 = &*(e2 as *const File_spec);
    spec1.wd == spec2.wd
}

unsafe extern "C" fn check_fspec(
    mut fspec: *mut File_spec,
    mut prev_fspec: *mut *mut File_spec,
) {
    let mut stats: stat = stat {
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
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if (*fspec).fd == -(1 as libc::c_int) {
        return;
    }
    name = pretty_name(fspec);
    if fstat((*fspec).fd, &mut stats) != 0 as libc::c_int {
        (*fspec).errnum = *__errno_location();
        close_fd((*fspec).fd, name);
        (*fspec).fd = -(1 as libc::c_int);
        return;
    }
    if (*fspec).mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint && stats.st_size < (*fspec).size
    {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"%s: file truncated\0" as *const u8 as *const libc::c_char),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    name,
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
                    gettext(b"%s: file truncated\0" as *const u8 as *const libc::c_char),
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
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(b"%s: file truncated\0" as *const u8 as *const libc::c_char),
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
        xlseek((*fspec).fd, 0 as libc::c_int as off_t, 0 as libc::c_int, name);
        (*fspec).size = 0 as libc::c_int as off_t;
    } else if (*fspec).mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint && stats.st_size == (*fspec).size
        && timespec_cmp((*fspec).mtime, get_stat_mtime(&mut stats)) == 0 as libc::c_int
    {
        return
    }
    let mut want_header: bool = print_headers as libc::c_int != 0
        && fspec != *prev_fspec;
    let mut bytes_read: uintmax_t = dump_remainder(
        want_header,
        name,
        (*fspec).fd,
        18446744073709551615 as libc::c_ulong,
    );
    (*fspec)
        .size = ((*fspec).size as libc::c_ulong).wrapping_add(bytes_read) as off_t
        as off_t;
    if bytes_read != 0 {
        *prev_fspec = fspec;
        if fflush_unlocked(stdout) != 0 as libc::c_int {
            write_error();
        }
    }
}
unsafe extern "C" fn tail_forever_inotify(
    mut wd: libc::c_int,
    mut f: *mut File_spec,
    mut n_files: size_t,
    mut sleep_interval: libc::c_double,
    mut wd_to_namep: *mut *mut Hash_table,
) {
    let mut max_realloc: libc::c_uint = 3 as libc::c_int as libc::c_uint;
    let mut wd_to_name: *mut Hash_table = 0 as *mut Hash_table;
    let mut found_watchable_file: bool = 0 as libc::c_int != 0;
    let mut tailed_but_unwatchable: bool = 0 as libc::c_int != 0;
    let mut found_unwatchable_dir: bool = 0 as libc::c_int != 0;
    let mut no_inotify_resources: bool = 0 as libc::c_int != 0;
    let mut writers_dead: bool = 0 as libc::c_int != 0;
    let mut prev_fspec: *mut File_spec = 0 as *mut File_spec;
    let mut evlen: size_t = 0 as libc::c_int as size_t;
    let mut evbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut evbuf_off: size_t = 0 as libc::c_int as size_t;
    let mut len: size_t = 0 as libc::c_int as size_t;
    wd_to_name = hash_initialize(
        n_files,
        0 as *const Hash_tuning,
        Some(wd_hasher as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t),
        Some(
            wd_comparator
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        None,
    );
    if wd_to_name.is_null() {
        xalloc_die();
    }
    *wd_to_namep = wd_to_name;
    let mut inotify_wd_mask: uint32_t = 0x2 as libc::c_int as uint32_t;
    if follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint {
        inotify_wd_mask
            |= (0x4 as libc::c_int | 0x400 as libc::c_int | 0x800 as libc::c_int)
                as libc::c_uint;
    }
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n_files {
        if !(*f.offset(i as isize)).ignore {
            let fnlen: usize = unsafe { CStr::from_ptr((*f.offset(i as isize)).name).to_bytes().len() };
if evlen < fnlen as u64 {
    evlen = fnlen as u64;
}
unsafe { (*f.offset(i as isize)).wd = -1 };
if follow_mode == Follow_name {
    let name_ptr = unsafe { (*f.offset(i as isize)).name };
    let dirlen: usize = dir_len(name_ptr).try_into().unwrap();
    let prev: i8 = unsafe { *name_ptr.offset(dirlen as isize) };
    unsafe {
        (*f.offset(i as isize)).basename_start = last_component(name_ptr)
            .offset_from(name_ptr) as u64;
        *name_ptr.offset(dirlen as isize) = 0;
    }
    unsafe {
        (*f.offset(i as isize)).parent_wd = inotify_add_watch(
            wd,
            if dirlen != 0 {
                name_ptr
            } else {
                b".\0" as *const u8 as *const libc::c_char
            },
            (0x100 | 0x200 | 0x80 | 0x4 | 0x400) as u32,
        );
        *name_ptr.offset(dirlen as isize) = prev;
    }
    if unsafe { (*f.offset(i as isize)).parent_wd } < 0 {
        if *__errno_location() != 28 {
            error(
                0,
                *__errno_location(),
                gettext(b"cannot watch parent directory of %s\0" as *const u8 as *const libc::c_char),
            );
            if 0 != 0 {
                unreachable!();
            }
        } else {
            error(
                0,
                0,
                gettext(b"inotify resources exhausted\0" as *const u8 as *const libc::c_char),
            );
            if 0 != 0 {
                unreachable!();
            }
        }
        found_unwatchable_dir = true;
        break;
    }
}
unsafe {
    (*f.offset(i as isize)).wd = inotify_add_watch(
        wd,
        (*f.offset(i as isize)).name,
        inotify_wd_mask,
    );
}

            if unsafe { (*f.offset(i as isize)).wd } < 0 {
    if unsafe { (*f.offset(i as isize)).fd } != -1 {
        tailed_but_unwatchable = true;
    }
    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
    if errno == 28 || errno == 12 {
        no_inotify_resources = true;
        error(
            0,
            0,
            gettext(b"inotify resources exhausted\0" as *const u8 as *const libc::c_char),
        );
        break;
    } else if errno != unsafe { (*f.offset(i as isize)).errnum } {
        error(
            0,
            errno,
            gettext(b"cannot watch %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, unsafe { (*f.offset(i as isize)).name }),
        );
    }
} else {
    if hash_insert(wd_to_name, unsafe { &*f.offset(i as isize) as *const File_spec as *const libc::c_void }).is_null() {
        xalloc_die();
    }
    found_watchable_file = true;
}

        }
        i = i.wrapping_add(1);
        i;
    }
    if no_inotify_resources as libc::c_int != 0
        || found_unwatchable_dir as libc::c_int != 0
        || follow_mode as libc::c_uint
            == Follow_descriptor as libc::c_int as libc::c_uint
            && tailed_but_unwatchable as libc::c_int != 0
    {
        return;
    }
    if follow_mode as libc::c_uint == Follow_descriptor as libc::c_int as libc::c_uint
        && !found_watchable_file
    {
        exit(1 as libc::c_int);
    }
    prev_fspec = &mut *f
        .offset(n_files.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as *mut File_spec;
    i = 0 as libc::c_int as size_t;
    while i < n_files {
        if !(*f.offset(i as isize)).ignore {
            if follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint
            {
                recheck(&mut *f.offset(i as isize), 0 as libc::c_int != 0);
            } else if (*f.offset(i as isize)).fd != -(1 as libc::c_int) {
                let mut stats: stat = stat {
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
                if stat((*f.offset(i as isize)).name, &mut stats) == 0 as libc::c_int
                    && ((*f.offset(i as isize)).dev != stats.st_dev
                        || (*f.offset(i as isize)).ino != stats.st_ino)
                {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"%s was replaced\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_name(&mut *f.offset(i as isize)),
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
                                *__errno_location(),
                                gettext(
                                    b"%s was replaced\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    pretty_name(&mut *f.offset(i as isize)),
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
                                *__errno_location(),
                                gettext(
                                    b"%s was replaced\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    pretty_name(&mut *f.offset(i as isize)),
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    return;
                }
            }
            check_fspec(&mut *f.offset(i as isize), &mut prev_fspec);
        }
        i = i.wrapping_add(1);
        i;
    }
    evlen = (evlen as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<inotify_event>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    evbuf = xmalloc(evlen) as *mut libc::c_char;
    loop {
        let mut fspec: *mut File_spec = std::ptr::null_mut();
let mut ev: *mut inotify_event = std::ptr::null_mut();

if follow_mode == Follow_name && !reopen_inaccessible_files && hash_get_n_entries(wd_to_name) == 0 {
    if false {
        error(1, 0, gettext(b"no files remaining\0" as *const u8 as *const libc::c_char));
        unreachable!();
    } else {
        let __errstatus: i32 = 1;
        error(__errstatus, 0, gettext(b"no files remaining\0" as *const u8 as *const libc::c_char));
        if __errstatus != 0 {
            unreachable!();
        }
    }
}

if len <= evbuf_off {
     let mut file_change: libc::c_int = 0;
            let mut pfd: [pollfd; 2] = [pollfd {
                fd: 0,
                events: 0,
                revents: 0,
            }; 2];
            loop {
                let mut delay: libc::c_int = -(1 as libc::c_int);
                if nbpids != 0 {
                    if writers_dead {
                        exit(0 as libc::c_int);
                    }
                    let writers_dead = writers_are_dead();
                    if writers_dead as libc::c_int != 0
                        || sleep_interval <= 0 as libc::c_int as libc::c_double
                    {
                        delay = 0 as libc::c_int;
                    } else if sleep_interval
                        < (2147483647 as libc::c_int / 1000 as libc::c_int
                            - 1 as libc::c_int) as libc::c_double
                    {
                        let mut ddelay: libc::c_double = sleep_interval
                            * 1000 as libc::c_int as libc::c_double;
                        delay = ddelay as libc::c_int;
                        delay += ((delay as libc::c_double) < ddelay) as libc::c_int;
                    }
                }
                pfd[0 as libc::c_int as usize].fd = wd;
                pfd[0 as libc::c_int as usize]
                    .events = 0x1 as libc::c_int as libc::c_short;
                pfd[1 as libc::c_int as usize].fd = 1 as libc::c_int;
                pfd[1 as libc::c_int as usize]
                    .revents = 0 as libc::c_int as libc::c_short;
                pfd[1 as libc::c_int as usize]
                    .events = pfd[1 as libc::c_int as usize].revents;
                file_change = poll(
                    pfd.as_mut_ptr(),
                    (monitor_output as libc::c_int + 1 as libc::c_int) as nfds_t,
                    delay,
                );
                if !(file_change == 0 as libc::c_int) {
                    break;
                }
            }
            if file_change < 0 as libc::c_int {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"error waiting for inotify and output events\0" as *const u8
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
                                b"error waiting for inotify and output events\0"
                                    as *const u8 as *const libc::c_char,
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
                                b"error waiting for inotify and output events\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            if pfd[1 as libc::c_int as usize].revents != 0 {
                die_pipe();
            }
            len = safe_read(wd, evbuf as *mut libc::c_void, evlen);
            evbuf_off = 0 as libc::c_int as size_t;

    
     if (len == 0 || len == u64::MAX && std::io::Error::last_os_error().raw_os_error() == Some(22)) && {
    let fresh4 = max_realloc;
    max_realloc = max_realloc.wrapping_sub(1);
    fresh4 != 0
} {
    len = 0;
    evlen = evlen.wrapping_mul(2);
    evbuf = xrealloc(evbuf as *mut libc::c_void, evlen) as *mut i8;
    continue;
} else if len == 0 || len == u64::MAX {
    let __errstatus: i32 = 1;
    error(__errstatus, std::io::Error::last_os_error().raw_os_error().unwrap_or(0), CString::new("error reading inotify event").unwrap().as_ptr());
    if __errstatus != 0 {
        unreachable!();
    }
    
    error(__errstatus, std::io::Error::last_os_error().raw_os_error().unwrap_or(0), CString::new("error reading inotify event").unwrap().as_ptr());
    if __errstatus != 0 {
        unreachable!();
    }
}


}

let void_ev = unsafe { evbuf.add(evbuf_off as usize) } as *mut libc::c_void;
ev = void_ev as *mut inotify_event;
evbuf_off += (std::mem::size_of::<inotify_event>() as u64 + (*ev).len as u64);

if (*ev).mask & 0x400 != 0 && (*ev).len == 0 {
    let mut i: usize = 0;
    while i < n_files as usize {
        if (*ev).wd == unsafe { (*f.offset(i as isize)).parent_wd } {
            if false {
                error(0, 0, gettext(b"directory containing watched file was removed\0" as *const u8 as *const libc::c_char));
                unreachable!();
            } else {
                let __errstatus: i32 = 0;
                error(__errstatus, 0, gettext(b"directory containing watched file was removed\0" as *const u8 as *const libc::c_char));
                if __errstatus != 0 {
                    unreachable!();
                }
            }
            return;
        }
        i += 1;
    }
}

if (*ev).len != 0 {
     let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < n_files {
                if (*f.offset(j as isize)).parent_wd == (*ev).wd
                    && strcmp(
                        ((*ev).name).as_mut_ptr(),
                        ((*f.offset(j as isize)).name)
                            .offset((*f.offset(j as isize)).basename_start as isize),
                    ) == 0 as libc::c_int
                {
                    break;
                }
                j = j.wrapping_add(1);
                j;
            }
            if j == n_files {
                continue;
            }
            fspec = &mut *f.offset(j as isize) as *mut File_spec;
            let mut new_wd: libc::c_int = -(1 as libc::c_int);
            let mut deleting: bool = (*ev).mask & 0x200 as libc::c_int as libc::c_uint
                != 0;
            if !deleting {
                new_wd = inotify_add_watch(
                    wd,
                    (*f.offset(j as isize)).name,
                    inotify_wd_mask,
                );
            }
            if !deleting && new_wd < 0 as libc::c_int {
                if *__errno_location() == 28 as libc::c_int
                    || *__errno_location() == 12 as libc::c_int
                {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"inotify resources exhausted\0" as *const u8
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
                                    b"inotify resources exhausted\0" as *const u8
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
                                    b"inotify resources exhausted\0" as *const u8
                                        as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    return;
                } else {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"cannot watch %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                (*f.offset(j as isize)).name,
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
                                *__errno_location(),
                                gettext(
                                    b"cannot watch %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    (*f.offset(j as isize)).name,
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
                                *__errno_location(),
                                gettext(
                                    b"cannot watch %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    (*f.offset(j as isize)).name,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
            }
            let mut new_watch: bool = false;
            new_watch = !deleting
                && ((*fspec).wd < 0 as libc::c_int || new_wd != (*fspec).wd);

    
     if new_watch {
    if unsafe { (*fspec).wd } >= 0 {
        inotify_rm_watch(wd, unsafe { (*fspec).wd });
        hash_remove(wd_to_name, fspec as *const libc::c_void);
    }
    unsafe { (*fspec).wd = new_wd };
    if new_wd == -1 {
        continue;
    }
    let prev = hash_remove(wd_to_name, fspec as *const libc::c_void) as *mut File_spec;
    if !prev.is_null() && prev != fspec {
        if follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint {
            recheck(prev, false);
        }
        unsafe {
            (*prev).wd = -1;
            close_fd((*prev).fd, pretty_name(prev));
        }
    }
    if hash_insert(wd_to_name, fspec as *const libc::c_void).is_null() {
        xalloc_die();
    }
}
if follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint {
    recheck(fspec, false);
}


} else {
     let mut key = File_spec {
    name: std::ptr::null_mut(),
    size: 0,
    mtime: timespec { tv_sec: 0, tv_nsec: 0 },
    dev: 0,
    ino: 0,
    mode: 0,
    ignore: false,
    remote: false,
    tailable: false,
    fd: 0,
    errnum: 0,
    blocking: 0,
    wd: 0,
    parent_wd: 0,
    basename_start: 0,
    n_unchanged_stats: 0,
};

key.wd = unsafe { (*ev).wd };

fspec = hash_lookup(wd_to_name, &key as *const File_spec as *const libc::c_void) as *mut File_spec;


}

if fspec.is_null() {
    continue;
}

if (*ev).mask & (0x4 | 0x200 | 0x400 | 0x800) != 0 {
    if (*ev).mask & 0x400 != 0 {
        inotify_rm_watch(wd, (*fspec).wd);
        hash_remove(wd_to_name, fspec as *const libc::c_void);
    }
    recheck(fspec, false);
} else {
    check_fspec(fspec, &mut prev_fspec);
}

/*
The variables live at this point are:
(mut wd: i32, mut f: *mut File_spec, mut n_files: u64, mut sleep_interval: f64, mut max_realloc: u32, mut wd_to_name: *mut hash_table, mut writers_dead: bool, mut prev_fspec: *mut File_spec, mut evlen: u64, mut evbuf: *mut i8, mut evbuf_off: u64, mut len: u64, mut inotify_wd_mask: u32, mut i: u64, mut fspec: *mut File_spec, mut ev: *mut inotify_event, mut void_ev: *mut libc::c_void)
*/

    };
}
unsafe extern "C" fn tail_bytes(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_bytes: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> bool {
    use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::io;

let mut stats: stat = unsafe { std::mem::zeroed() }; // Create a zeroed stat struct

if unsafe { fstat(fd, &mut stats as *mut _ as *mut stat) } != 0 {
    let err_code = unsafe { *__errno_location() };
    error(
        0,
        err_code,
        gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_filename),
    );
    return false; // Return false instead of 0
}

/*
The variables live at this point are:
(pretty_filename: *const i8, fd: i32, stats: stat)
*/

    if from_start {
        if !presume_input_pipe
            && n_bytes
                <= (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                    -(1 as libc::c_int) as off_t
                } else {
                    (((1 as libc::c_int as off_t)
                        << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as libc::c_ulong
            && (stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
                && xlseek(fd, n_bytes as off_t, 1 as libc::c_int, pretty_filename)
                    >= 0 as libc::c_int as libc::c_long
                || lseek(fd, n_bytes as __off_t, 1 as libc::c_int)
                    != -(1 as libc::c_int) as libc::c_long)
        {
            *read_pos = (*read_pos as libc::c_ulong).wrapping_add(n_bytes) as uintmax_t
                as uintmax_t;
        } else {
            let mut t: libc::c_int = start_bytes(pretty_filename, fd, n_bytes, read_pos);
            if t != 0 {
                return t < 0 as libc::c_int;
            }
        }
        n_bytes = 18446744073709551615 as libc::c_ulong;
    } else {
        let mut end_pos: off_t = -(1 as libc::c_int) as off_t;
        let mut current_pos: off_t = -(1 as libc::c_int) as off_t;
        let mut copy_from_current_pos: bool = 0 as libc::c_int != 0;
        if !presume_input_pipe
            && n_bytes
                <= (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                    -(1 as libc::c_int) as off_t
                } else {
                    (((1 as libc::c_int as off_t)
                        << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as libc::c_ulong
        {
            if usable_st_size(&mut stats) {
                end_pos = stats.st_size;
                let mut smallish_size: off_t = (if (0 as libc::c_int) < stats.st_blksize
                    && stats.st_blksize as libc::c_ulong
                        <= (-(1 as libc::c_int) as size_t)
                            .wrapping_div(8 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    stats.st_blksize
                } else {
                    512 as libc::c_int
                }) as off_t;
                copy_from_current_pos = smallish_size < end_pos;
            } else {
                current_pos = lseek(
                    fd,
                    n_bytes.wrapping_neg() as __off_t,
                    2 as libc::c_int,
                );
                copy_from_current_pos = current_pos
                    != -(1 as libc::c_int) as libc::c_long;
                if copy_from_current_pos {
                    end_pos = (current_pos as libc::c_ulong).wrapping_add(n_bytes)
                        as off_t;
                }
            }
        }
        if !copy_from_current_pos {
            return pipe_bytes(pretty_filename, fd, n_bytes, read_pos);
        }
        if current_pos == -(1 as libc::c_int) as libc::c_long {
            current_pos = xlseek(
                fd,
                0 as libc::c_int as off_t,
                1 as libc::c_int,
                pretty_filename,
            );
        }
        if current_pos < end_pos {
            let mut bytes_remaining: off_t = end_pos - current_pos;
            if n_bytes < bytes_remaining as libc::c_ulong {
                current_pos = (end_pos as libc::c_ulong).wrapping_sub(n_bytes) as off_t;
                xlseek(fd, current_pos, 0 as libc::c_int, pretty_filename);
            }
        }
        *read_pos = current_pos as uintmax_t;
    }
    *read_pos = (*read_pos as libc::c_ulong)
        .wrapping_add(
            dump_remainder(0 as libc::c_int != 0, pretty_filename, fd, n_bytes),
        ) as uintmax_t as uintmax_t;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tail_lines(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_lines: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> bool {
    let mut stats: stat = stat {
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
    if fstat(fd, &mut stats) != 0 {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        return 0 as libc::c_int != 0;
    }
    if from_start {
        let mut t: libc::c_int = start_lines(pretty_filename, fd, n_lines.try_into().unwrap(), read_pos);
        if t != 0 {
            return t < 0 as libc::c_int;
        }
        *read_pos = (*read_pos as libc::c_ulong)
            .wrapping_add(
                dump_remainder(
                    0 as libc::c_int != 0,
                    pretty_filename,
                    fd,
                    18446744073709551615 as libc::c_ulong,
                ),
            ) as uintmax_t as uintmax_t;
    } else {
        let mut start_pos: off_t = -(1 as libc::c_int) as off_t;
        let mut end_pos: off_t = 0;
        if !presume_input_pipe
            && stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
            && {
                start_pos = lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int);
                start_pos != -(1 as libc::c_int) as libc::c_long
            }
            && {
                end_pos = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
                start_pos < end_pos
            }
        {
            *read_pos = end_pos as uintmax_t;
            if end_pos != 0 as libc::c_int as libc::c_long
                && !file_lines(
                    pretty_filename,
                    fd,
                    &mut stats,
                    n_lines,
                    start_pos,
                    end_pos,
                    read_pos,
                )
            {
                return 0 as libc::c_int != 0;
            }
        } else {
            if start_pos != -(1 as libc::c_int) as libc::c_long {
                xlseek(fd, start_pos, 0 as libc::c_int, pretty_filename);
            }
            let result = pipe_lines(pretty_filename, fd, n_lines, read_pos);
return result;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tail(
    mut filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_units: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> bool {
    *read_pos = 0 as libc::c_int as uintmax_t;
    if count_lines {
        return tail_lines(filename, fd, n_units, read_pos)
    } else {
        return tail_bytes(filename, fd, n_units, read_pos)
    };
}
unsafe extern "C" fn tail_file(mut f: *mut File_spec, mut n_units: uintmax_t) -> bool {
    let mut fd: i32 = 0;
let mut ok: bool = false;
let is_stdin = unsafe { CStr::from_ptr((*f).name) } == CStr::from_bytes_with_nul(b"-\0").unwrap();
if is_stdin {
    have_read_stdin = true;
    fd = 0;
    xset_binary_mode(0, 0);
} else {
    fd = open_safer(unsafe { CStr::from_ptr((*f).name).as_ptr() }, 0);
}
(*f).tailable = !(reopen_inaccessible_files && fd == -1);
if fd == -1 {
     if forever {
    unsafe {
        (*f).fd = -1;
        (*f).errnum = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
        (*f).ignore = !reopen_inaccessible_files;
        (*f).ino = 0;
        (*f).dev = 0;
    }
}

if false {
    error(
        0,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        gettext(b"cannot open %s for reading\0".as_ptr() as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if false {
        unreachable!();
    }
} else {
    let __errstatus: i32 = 0;
    error(
        __errstatus,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        gettext(b"cannot open %s for reading\0".as_ptr() as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if __errstatus != 0 {
        unreachable!();
    }

    let __errstatus: i32 = 0;
    error(
        __errstatus,
        std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        gettext(b"cannot open %s for reading\0".as_ptr() as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if __errstatus != 0 {
        unreachable!();
    }
}

ok = false;

    
} else {
    let mut read_pos: u64 = 0;
    if print_headers {
        write_header(pretty_name(f));
    }
    ok = tail(pretty_name(f), fd, n_units, &mut read_pos);
    if forever {
         let mut stats = stat {
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

if let Some(file_spec) = unsafe { f.as_mut() } {
    file_spec.errnum = if ok { 0 } else { -1 };
}

        
         if fstat(fd, &mut stats) < 0 as libc::c_int {
                ok = 0 as libc::c_int != 0;
                (*f).errnum = *__errno_location();
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_name(f),
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
                            *__errno_location(),
                            gettext(
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_name(f),
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            } else if !(stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
                || stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o10000 as libc::c_int as libc::c_uint
                || stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o140000 as libc::c_int as libc::c_uint
                || stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o20000 as libc::c_int as libc::c_uint)
            {
                ok = 0 as libc::c_int != 0;
                (*f).errnum = -(1 as libc::c_int);
                (*f).tailable = 0 as libc::c_int != 0;
                (*f).ignore = !reopen_inaccessible_files;
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"%s: cannot follow end of this type of file%s\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            pretty_name(f),
                        ),
                        if (*f).ignore as libc::c_int != 0 {
                            gettext(
                                b"; giving up on this name\0" as *const u8
                                    as *const libc::c_char,
                            ) as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
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
                                b"%s: cannot follow end of this type of file%s\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                pretty_name(f),
                            ),
                            if (*f).ignore as libc::c_int != 0 {
                                gettext(
                                    b"; giving up on this name\0" as *const u8
                                        as *const libc::c_char,
                                ) as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
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
                                b"%s: cannot follow end of this type of file%s\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                pretty_name(f),
                            ),
                            if (*f).ignore as libc::c_int != 0 {
                                gettext(
                                    b"; giving up on this name\0" as *const u8
                                        as *const libc::c_char,
                                ) as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
        
         if !ok {
    (*f).ignore = !reopen_inaccessible_files;
    close_fd(fd, pretty_name(f));
    (*f).fd = -1;
} else {
    record_open_fd(
        &mut *f,
        fd,
        read_pos as i64,
        &mut stats,
        if is_stdin { -1 } else { 1 },
    );
    (*f).remote = fremote(fd, pretty_name(f));
}

        
    } else if !is_stdin && close(fd) != 0 {
         if false {
    error(
        0,
        *__errno_location(),
        gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if false {
        unreachable!();
    }
} else {
    let __errstatus: i32 = 0;
    error(
        __errstatus,
        *__errno_location(),
        gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if __errstatus != 0 {
        unreachable!();
    }

    let __errstatus: i32 = 0;
    error(
        __errstatus,
        *__errno_location(),
        gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
    );
    if __errstatus != 0 {
        unreachable!();
    }
}
ok = false;

        
    }
}
ok

}
unsafe extern "C" fn parse_obsolete_option(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut n_units: *mut uintmax_t,
) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut n_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut n_string_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut default_count: libc::c_int = 10 as libc::c_int;
    let mut t_from_start: bool = false;
    let mut t_count_lines: bool = 1 as libc::c_int != 0;
    let mut t_forever: bool = 0 as libc::c_int != 0;
    if !(argc == 2 as libc::c_int
        || argc == 3 as libc::c_int
            && !(*(*argv.offset(2 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *(*argv.offset(2 as libc::c_int as isize))
                    .offset(1 as libc::c_int as isize) as libc::c_int != 0)
        || 3 as libc::c_int <= argc && argc <= 4 as libc::c_int
            && strcmp(
                *argv.offset(2 as libc::c_int as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
    {
        return 0 as libc::c_int != 0;
    }
    let mut posix_ver: libc::c_int = posix2_version();
    let mut obsolete_usage: bool = posix_ver < 200112 as libc::c_int;
    let mut traditional_usage: bool = obsolete_usage as libc::c_int != 0
        || 200809 as libc::c_int <= posix_ver;
    p = *argv.offset(1 as libc::c_int as isize);
    let fresh5 = p;
    p = p.offset(1);
    match *fresh5 as libc::c_int {
        43 => {
            if !traditional_usage {
                return 0 as libc::c_int != 0;
            }
            t_from_start = 1 as libc::c_int != 0;
        }
        45 => {
            if !obsolete_usage
                && *p
                    .offset(
                        (*p.offset(0 as libc::c_int as isize) as libc::c_int
                            == 'c' as i32) as libc::c_int as isize,
                    ) == 0
            {
                return 0 as libc::c_int != 0;
            }
            t_from_start = 0 as libc::c_int != 0;
        }
        _ => return 0 as libc::c_int != 0,
    }
    n_string = p;
    while (*p as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        p = p.offset(1);
        p;
    }
    n_string_end = p;
    let mut current_block_19: u64;
    match *p as libc::c_int {
        98 => {
            default_count *= 512 as libc::c_int;
            current_block_19 = 12856559154846489347;
        }
        99 => {
            current_block_19 = 12856559154846489347;
        }
        108 => {
            current_block_19 = 7044594549367080378;
        }
        _ => {
            current_block_19 = 5783071609795492627;
        }
    }
    match current_block_19 {
        12856559154846489347 => {
            t_count_lines = 0 as libc::c_int != 0;
            current_block_19 = 7044594549367080378;
        }
        _ => {}
    }
    match current_block_19 {
        7044594549367080378 => {
            p = p.offset(1);
            p;
        }
        _ => {}
    }
    if *p as libc::c_int == 'f' as i32 {
        t_forever = 1 as libc::c_int != 0;
        p = p.offset(1);
        p;
    }
    if *p != 0 {
        return 0 as libc::c_int != 0;
    }
    if n_string == n_string_end {
        *n_units = default_count as uintmax_t;
    } else if xstrtoumax(
        n_string,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
        n_units,
        b"b\0" as *const u8 as *const libc::c_char,
    ) as libc::c_uint & !(LONGINT_INVALID_SUFFIX_CHAR as libc::c_int) as libc::c_uint
        != LONGINT_OK as libc::c_int as libc::c_uint
    {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                gettext(b"invalid number\0" as *const u8 as *const libc::c_char),
                quote(*argv.offset(1 as libc::c_int as isize)),
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
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    gettext(b"invalid number\0" as *const u8 as *const libc::c_char),
                    quote(*argv.offset(1 as libc::c_int as isize)),
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
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    gettext(b"invalid number\0" as *const u8 as *const libc::c_char),
                    quote(*argv.offset(1 as libc::c_int as isize)),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    from_start = t_from_start;
    count_lines = t_count_lines;
    forever = t_forever;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut n_units: *mut uintmax_t,
    mut header_mode: *mut header_mode,
    mut sleep_interval: *mut libc::c_double,
) {
    /*
The variables live at this point are:
(mut argc: i32, mut argv: *mut *mut i8, mut n_units: *mut u64, mut header_mode: *mut u32, mut sleep_interval: *mut f64)
*/
let mut c: i32 = 0;
loop {
     use std::ffi::CString;
use std::os::raw::c_char;

let mut c: i32;

c = getopt_long(
    argc,
    argv,
    CString::new("c:n:fFqs:vz0123456789").unwrap().as_ptr() as *const c_char,
    long_options.as_ptr(),
    std::ptr::null_mut(),
);

if c == -1 {
    break;
}

let mut current_block_33: u64;
match c {
    70 => {
        forever = true;
        follow_mode = Follow_name; // Assuming Follow_name is defined elsewhere
        reopen_inaccessible_files = true;
        current_block_33 = 4567019141635105728;
    }
    99 | 110 => {
         let is_count_lines = c == 'n' as i32;
let optarg_str = unsafe { std::ffi::CStr::from_ptr(optarg) }.to_string_lossy();

let is_from_start = match optarg_str.chars().next() {
    Some('+') => true,
    Some('-') => {
        optarg = optarg_str[1..].as_ptr() as *mut i8; // Move the pointer forward by one character
        false
    },
    _ => false,
};

*n_units = xdectoumax(
    optarg_str.as_ptr() as *const i8,
    0,
    u64::MAX,
    b"bkKmMGTPEZYRQ0\0" as *const u8 as *const libc::c_char,
    if is_count_lines {
        gettext(b"invalid number of lines\0" as *const u8 as *const libc::c_char)
    } else {
        gettext(b"invalid number of bytes\0" as *const u8 as *const libc::c_char)
    },
    0,
);
current_block_33 = 4567019141635105728;


    }
    102 | 260 => {
         let is_forever = true;
if optarg.is_null() {
    follow_mode = Follow_descriptor;
} else {
    follow_mode = follow_mode_map[__xargmatch_internal(
        b"--follow\0".as_ptr() as *const libc::c_char,
        optarg,
        follow_mode_string.as_ptr(),
        follow_mode_map.as_ptr() as *const libc::c_void,
        std::mem::size_of::<Follow_mode>() as libc::c_ulong,
        argmatch_die,
        true,
    ) as usize];
}
current_block_33 = 4567019141635105728;


    }
    256 => {
        reopen_inaccessible_files = true;
        current_block_33 = 4567019141635105728;
    }
    257 => {
         max_n_unchanged_stats_between_opens = xdectoumax(
    optarg,
    0u64,
    u64::MAX,
    std::ptr::null(),
    gettext(std::ffi::CString::new("invalid maximum number of unchanged stats between opens").unwrap().as_ptr()),
    0,
);
current_block_33 = 4567019141635105728;


    }
    261 => {
        disable_inotify = true;
        current_block_33 = 4567019141635105728;
    }
    258 => {
         if nbpids as libc::c_long == pids_alloc {
                    pids = xpalloc(
                        pids as *mut libc::c_void,
                        &mut pids_alloc,
                        1 as libc::c_int as idx_t,
                        if (2147483647 as libc::c_int as libc::c_long)
                            < 9223372036854775807 as libc::c_long
                        {
                            2147483647 as libc::c_int as libc::c_long
                        } else {
                            9223372036854775807 as libc::c_long
                        },
                        ::core::mem::size_of::<pid_t>() as libc::c_ulong as idx_t,
                    ) as *mut pid_t;
                }
                let fresh6 = nbpids;
                nbpids = nbpids + 1;
                *pids
                    .offset(
                        fresh6 as isize,
                    ) = xdectoumax(
                    optarg,
                    0 as libc::c_int as uintmax_t,
                    (if (0 as libc::c_int) < -(1 as libc::c_int) {
                        -(1 as libc::c_int)
                    } else {
                        (((1 as libc::c_int)
                            << (::core::mem::size_of::<pid_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                    }) as uintmax_t,
                    b"\0" as *const u8 as *const libc::c_char,
                    gettext(b"invalid PID\0" as *const u8 as *const libc::c_char),
                    0 as libc::c_int,
                ) as pid_t;
                current_block_33 = 4567019141635105728;

    }
    259 => {
        presume_input_pipe = true;
        current_block_33 = 4567019141635105728;
    }
    113 => {
        *header_mode = never; // Assuming never is defined elsewhere
        current_block_33 = 4567019141635105728;
    }
    115 => {
         let mut s: libc::c_double = 0.;
                if !(xstrtod(
                    optarg,
                    0 as *mut *const libc::c_char,
                    &mut s,
                    Some(
                        cl_strtod
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                *mut *mut libc::c_char,
                            ) -> libc::c_double,
                    ),
                ) as libc::c_int != 0 && 0 as libc::c_int as libc::c_double <= s)
                {
                    if 0 != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"invalid number of seconds: %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quote(optarg),
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
                                    b"invalid number of seconds: %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quote(optarg),
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
                                    b"invalid number of seconds: %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quote(optarg),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
                *sleep_interval = s;
                current_block_33 = 4567019141635105728;

    }
    118 => {
        *header_mode = always; // Assuming always is defined elsewhere
        current_block_33 = 4567019141635105728;
    }
    122 => {
        line_end = '\0' as libc::c_char;
        current_block_33 = 4567019141635105728;
    }
    -2 => {
        usage(0); // Assuming usage is defined elsewhere
        current_block_33 = 4567019141635105728;
    }
    -3 => {
         version_etc(
    stdout,
    "tail\0".as_ptr() as *const libc::c_char,
    "GNU coreutils\0".as_ptr() as *const libc::c_char,
    Version,
    proper_name_lite(
        "Paul Rubin\0".as_ptr() as *const libc::c_char,
        "Paul Rubin\0".as_ptr() as *const libc::c_char,
    ),
    proper_name_lite(
        "David MacKenzie\0".as_ptr() as *const libc::c_char,
        "David MacKenzie\0".as_ptr() as *const libc::c_char,
    ),
    proper_name_lite(
        "Ian Lance Taylor\0".as_ptr() as *const libc::c_char,
        "Ian Lance Taylor\0".as_ptr() as *const libc::c_char,
    ),
    proper_name_lite(
        "Jim Meyering\0".as_ptr() as *const libc::c_char,
        "Jim Meyering\0".as_ptr() as *const libc::c_char,
    ),
    std::ptr::null_mut::<libc::c_char>(),
);
std::process::exit(0);


    }
    48..=57 => {
         if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"option used in invalid context -- %c\0" as *const u8
                                as *const libc::c_char,
                        ),
                        c,
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
                                b"option used in invalid context -- %c\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            c,
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
                                b"option used in invalid context -- %c\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            c,
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
                current_block_33 = 17156019370052222107;

    }
    _ => {
        current_block_33 = 17156019370052222107;
    }
}

match current_block_33 {
    17156019370052222107 => {
        usage(1); // Assuming usage is defined elsewhere
    }
    _ => {}
}

/*
The variables live at this point are:
(mut argc: i32, mut argv: *mut *mut i8, mut n_units: *mut u64, mut header_mode: *mut u32, mut sleep_interval: *mut f64, mut c: i32, mut current_block_33: u64)
*/

    
}
if reopen_inaccessible_files {
    if !forever {
        reopen_inaccessible_files = false;
        if false {
            error(
                0,
                0,
                gettext(
                    b"warning: --retry ignored; --retry is useful only when following\0" as *const u8 as *const libc::c_char,
                ),
            );
            if false {
                unreachable!();
            }
        } else {
            let __errstatus: i32 = 0;
            error(
                __errstatus,
                0,
                gettext(
                    b"warning: --retry ignored; --retry is useful only when following\0" as *const u8 as *const libc::c_char,
                ),
            );
            if __errstatus != 0 {
                unreachable!();
            }
            let __errstatus: i32 = 0;
            error(
                __errstatus,
                0,
                gettext(
                    b"warning: --retry ignored; --retry is useful only when following\0" as *const u8 as *const libc::c_char,
                ),
            );
            if __errstatus != 0 {
                unreachable!();
            }
        }
    } else if follow_mode == Follow_descriptor {
        if false {
            error(
                0,
                0,
                gettext(
                    b"warning: --retry only effective for the initial open\0" as *const u8 as *const libc::c_char,
                ),
            );
            if false {
                unreachable!();
            }
        } else {
            let __errstatus: i32 = 0;
            error(
                __errstatus,
                0,
                gettext(
                    b"warning: --retry only effective for the initial open\0" as *const u8 as *const libc::c_char,
                ),
            );
            if __errstatus != 0 {
                unreachable!();
            }
            let __errstatus: i32 = 0;
            error(
                __errstatus,
                0,
                gettext(
                    b"warning: --retry only effective for the initial open\0" as *const u8 as *const libc::c_char,
                ),
            );
            if __errstatus != 0 {
                unreachable!();
            }
        }
    }
}
/*
The variables live at this point are:
(mut argc: i32, mut argv: *mut *mut i8, mut n_units: *mut u64, mut header_mode: *mut u32, mut sleep_interval: *mut f64, mut c: i32)
*/

    if nbpids != 0 && !forever {
    if 0 != 0 {
        error(
            0,
            0,
            gettext(b"warning: PID ignored; --pid=PID is useful only when following\0" as *const u8 as *const libc::c_char),
        );
        if 0 != 0 {
            unreachable!();
        }
    } else {
        let __errstatus: i32 = 0;
        error(
            __errstatus,
            0,
            gettext(b"warning: PID ignored; --pid=PID is useful only when following\0" as *const u8 as *const libc::c_char),
        );
        if __errstatus != 0 {
            unreachable!();
        }

        let __errstatus: i32 = 0;
        error(
            __errstatus,
            0,
            gettext(b"warning: PID ignored; --pid=PID is useful only when following\0" as *const u8 as *const libc::c_char),
        );
        if __errstatus != 0 {
            unreachable!();
        }
    }
} else if nbpids != 0 && kill(*pids.offset(0), 0) != 0 && *__errno_location() == 38 {
    if 0 != 0 {
        error(
            0,
            0,
            gettext(b"warning: --pid=PID is not supported on this system\0" as *const u8 as *const libc::c_char),
        );
        if 0 != 0 {
            unreachable!();
        }
    } else {
        let __errstatus: i32 = 0;
        error(
            __errstatus,
            0,
            gettext(b"warning: --pid=PID is not supported on this system\0" as *const u8 as *const libc::c_char),
        );
        if __errstatus != 0 {
            unreachable!();
        }

        let __errstatus: i32 = 0;
        error(
            __errstatus,
            0,
            gettext(b"warning: --pid=PID is not supported on this system\0" as *const u8 as *const libc::c_char),
        );
        if __errstatus != 0 {
            unreachable!();
        }
    }
    nbpids = 0;
    unsafe { free(pids as *mut libc::c_void) }; // Assuming pids is a raw pointer, replace with appropriate freeing logic if needed
}

}
unsafe extern "C" fn ignore_fifo_and_pipe(
    mut f: *mut File_spec,
    mut n_files: size_t,
) -> size_t {
    let mut n_viable: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_files {
        let mut is_a_fifo_or_pipe: bool = strcmp(
            (*f.offset(i as isize)).name,
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int && !(*f.offset(i as isize)).ignore
            && 0 as libc::c_int <= (*f.offset(i as isize)).fd
            && ((*f.offset(i as isize)).mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o10000 as libc::c_int as libc::c_uint
                || 1 as libc::c_int != 1 as libc::c_int
                    && isapipe((*f.offset(i as isize)).fd) != 0);
        if is_a_fifo_or_pipe {
            (*f.offset(i as isize)).fd = -(1 as libc::c_int);
            (*f.offset(i as isize)).ignore = 1 as libc::c_int != 0;
        } else {
            n_viable = n_viable.wrapping_add(1);
            n_viable;
        }
        i = i.wrapping_add(1);
        i;
    }
    return n_viable;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut header_mode: header_mode = multiple_files;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut n_units: uintmax_t = 10 as libc::c_int as uintmax_t;
    let mut n_files: size_t = 0;
    let mut file: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut F: *mut File_spec = 0 as *mut File_spec;
    let mut i: size_t = 0;
    let mut obsolete_option: bool = false;
    let mut sleep_interval: libc::c_double = 1.0f64;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    page_size = getpagesize() as idx_t;
    have_read_stdin = 0 as libc::c_int != 0;
    count_lines = 1 as libc::c_int != 0;
    print_headers = 0 as libc::c_int != 0;
    from_start = print_headers;
    forever = from_start;
    line_end = '\n' as i32 as libc::c_char;
    obsolete_option = parse_obsolete_option(argc, argv, &mut n_units);
    argc -= obsolete_option as libc::c_int;
    argv = argv.offset(obsolete_option as libc::c_int as isize);
    parse_options(argc, argv, &mut n_units, &mut header_mode, &mut sleep_interval);
    if from_start {
        if n_units != 0 {
            n_units = n_units.wrapping_sub(1);
            n_units;
        }
    }
    if optind < argc {
        n_files = (argc - optind) as size_t;
        file = argv.offset(optind as isize);
    } else {
        static mut dummy_stdin: *mut libc::c_char = b"-\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        n_files = 1 as libc::c_int as size_t;
        file = &mut dummy_stdin;
    }
    let mut found_hyphen: bool = 0 as libc::c_int != 0;
    i = 0 as libc::c_int as size_t;
    while i < n_files {
        if strcmp(*file.offset(i as isize), b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            found_hyphen = 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    if found_hyphen as libc::c_int != 0
        && follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint
    {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"cannot follow %s by name\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(
                    shell_escape_always_quoting_style,
                    b"-\0" as *const u8 as *const libc::c_char,
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
                        b"cannot follow %s by name\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"-\0" as *const u8 as *const libc::c_char,
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
                        b"cannot follow %s by name\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"-\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    if forever as libc::c_int != 0 && found_hyphen as libc::c_int != 0 {
        let mut in_stat: stat = stat {
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
        let mut blocking_stdin: bool = false;
        blocking_stdin = nbpids == 0
            && follow_mode as libc::c_uint
                == Follow_descriptor as libc::c_int as libc::c_uint
            && n_files == 1 as libc::c_int as libc::c_ulong
            && fstat(0 as libc::c_int, &mut in_stat) == 0
            && !(in_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint);
        if !blocking_stdin && isatty(0 as libc::c_int) != 0 {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"warning: following standard input indefinitely is ineffective\0"
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
                            b"warning: following standard input indefinitely is ineffective\0"
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
                            b"warning: following standard input indefinitely is ineffective\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    }
    if n_units == 0 && !forever && !from_start {
        return 0 as libc::c_int;
    }
    F = xnmalloc(n_files, ::core::mem::size_of::<File_spec>() as libc::c_ulong)
        as *mut File_spec;
    i = 0 as libc::c_int as size_t;
    while i < n_files {
        let ref mut fresh7 = (*F.offset(i as isize)).name;
        *fresh7 = *file.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    if header_mode as libc::c_uint == always as libc::c_int as libc::c_uint
        || header_mode as libc::c_uint == multiple_files as libc::c_int as libc::c_uint
            && n_files > 1 as libc::c_int as libc::c_ulong
    {
        print_headers = 1 as libc::c_int != 0;
    }
    let fd1: i32 = 1;
let mode1: i32 = 0;
xset_binary_mode(fd1, mode1);
    i = 0 as libc::c_int as size_t;
    while i < n_files {
        ok = (ok as libc::c_int
            & tail_file(&mut *F.offset(i as isize), n_units) as libc::c_int) != 0;
        i = i.wrapping_add(1);
        i;
    }
    if forever as libc::c_int != 0 && ignore_fifo_and_pipe(F, n_files) != 0 {
        let mut out_stat: stat = stat {
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
        if fstat(1 as libc::c_int, &mut out_stat) < 0 as libc::c_int {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"standard output\0" as *const u8 as *const libc::c_char),
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
                        gettext(b"standard output\0" as *const u8 as *const libc::c_char),
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
                        gettext(b"standard output\0" as *const u8 as *const libc::c_char),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        monitor_output = out_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint
            || 1 as libc::c_int != 1 as libc::c_int && isapipe(1 as libc::c_int) != 0;
        if !disable_inotify
            && (tailable_stdin(F, n_files) as libc::c_int != 0
                || any_remote_file(F, n_files) as libc::c_int != 0
                || !any_non_remote_file(F, n_files)
                || any_symlinks(F, n_files) as libc::c_int != 0
                || any_non_regular_fifo(F, n_files) as libc::c_int != 0
                || !ok
                    && follow_mode as libc::c_uint
                        == Follow_descriptor as libc::c_int as libc::c_uint)
        {
            disable_inotify = 1 as libc::c_int != 0;
        }
        if !disable_inotify {
            let mut wd: libc::c_int = inotify_init();
            if 0 as libc::c_int <= wd {
                if fflush_unlocked(stdout) != 0 as libc::c_int {
                    write_error();
                }
                let mut ht: *mut Hash_table = 0 as *mut Hash_table;
                tail_forever_inotify(wd, F, n_files, sleep_interval, &mut ht);
                hash_free(ht);
                close(wd);
                *__errno_location() = 0 as libc::c_int;
            }
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"inotify cannot be used, reverting to polling\0" as *const u8
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
                        *__errno_location(),
                        gettext(
                            b"inotify cannot be used, reverting to polling\0"
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
                        *__errno_location(),
                        gettext(
                            b"inotify cannot be used, reverting to polling\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        disable_inotify = 1 as libc::c_int != 0;
        tail_forever(F, n_files, sleep_interval);
    }
    if have_read_stdin as libc::c_int != 0 && close(0 as libc::c_int) < 0 as libc::c_int
    {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"-\0" as *const u8 as *const libc::c_char,
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
                    b"-\0" as *const u8 as *const libc::c_char,
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
                    b"-\0" as *const u8 as *const libc::c_char,
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    exit(if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int });
}
pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let argc = args.len() as libc::c_int;
    let argv: Vec<CString> = args.iter()
        .map(|arg| CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();
    
    let mut argv_ptr: Vec<*mut libc::c_char> = argv.iter()
        .map(|cstr| cstr.as_ptr() as *mut libc::c_char)
        .collect();
    argv_ptr.push(std::ptr::null_mut());

    let exit_code = unsafe {
        main_0(argc, argv_ptr.as_mut_ptr()) as i32
    };
    ::std::process::exit(exit_code);
}

