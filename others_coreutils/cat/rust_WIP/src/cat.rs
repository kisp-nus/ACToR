#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_imports
)]
#![feature(extern_types)]

use libc;

use std::io;

use std::ptr;

use std::io::Write;

use rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fpurge(gl_stream: *mut FILE) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn rpl_copy_file_range(
        ifd: libc::c_int,
        ipos: *mut off_t,
        ofd: libc::c_int,
        opos: *mut off_t,
        len: size_t,
        flags: libc::c_uint,
    ) -> ssize_t;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xalloc_die();
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
    fn error(__status: libc::c_int, __errnum: libc::c_int, __format: *const libc::c_char, _: ...);
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn xalignalloc(_: idx_t, _: idx_t) -> *mut libc::c_void;
    fn fdadvise(fd: libc::c_int, offset: off_t, len: off_t, advice: fadvice_t);
    fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> size_t;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
}
pub type size_t = libc::c_ulong;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IO_BUFSIZE: C2RustUnnamed_0 = 262144;
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
#[inline]
fn is_ENOTSUP(err: i32) -> bool {
    err == 95
}

#[inline]
fn emit_stdin_note() {
    let message = "\nWith no FILE, or when FILE is -, read standard input.\n";
    let _ = std::io::stdout().write_all(message.as_bytes());
}

#[inline]
fn emit_ancillary_info(program: &str) {
    let infomap_0: [(&str, &str); 7] = [
        ("[", "test invocation"),
        ("coreutils", "Multi-call invocation"),
        ("sha224sum", "sha2 utilities"),
        ("sha256sum", "sha2 utilities"),
        ("sha384sum", "sha2 utilities"),
        ("sha512sum", "sha2 utilities"),
        ("", ""),
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
        "GNU coreutils", "https://www.gnu.org/software/coreutils/"
    );

    let lc_messages = unsafe { setlocale(5, std::ptr::null()) };
    if !lc_messages.is_null() {
        let lc_messages_str = unsafe { std::ffi::CStr::from_ptr(lc_messages).to_string_lossy() };
        if !lc_messages_str.starts_with("en_") {
            eprint!(
                "{}",
                "Report any translation bugs to <https://translationproject.org/team/>"
            );
        }
    }

    let url_program = if program == "[" { "test" } else { program };

    println!(
        "Full documentation <{}{}>",
        "https://www.gnu.org/software/coreutils/", url_program
    );

    println!(
        "or available locally via: info '(coreutils) {}{}'",
        node,
        if node == program { " invocation" } else { "" }
    );
}

#[inline]
fn write_error() {
    let saved_errno: i32 = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
    let _ = std::io::stdout().flush();

    if false {
        unsafe {
            error(
                1,
                saved_errno,
                gettext(b"write error\0".as_ptr() as *const libc::c_char),
            );
        }
        if true {
            unreachable!();
        }
    } else {
        {
            let errstatus: i32 = 1;
            unsafe {
                error(
                    errstatus,
                    saved_errno,
                    gettext(b"write error\0".as_ptr() as *const libc::c_char),
                );
            }
            if errstatus != 0 {
                unreachable!();
            }
        }
        {
            let errstatus: i32 = 1;
            unsafe {
                error(
                    errstatus,
                    saved_errno,
                    gettext(b"write error\0".as_ptr() as *const libc::c_char),
                );
            }
            if errstatus != 0 {
                unreachable!();
            }
        }
    }
}

#[inline]
unsafe extern "C" fn alignfree(mut ptr: *mut libc::c_void) {
    free(ptr);
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
fn stdc_leading_zeros_ull(n: u64) -> u32 {
    n.leading_zeros()
}

#[inline]
fn io_blksize(st: &stat) -> idx_t {
    let blocksize: idx_t = if st.st_blksize > 0 && st.st_blksize as usize <= (usize::MAX / 8) {
        st.st_blksize as idx_t
    } else {
        512
    };

    let mut adjusted_blocksize =
        blocksize + (IO_BUFSIZE as idx_t - 1) - (IO_BUFSIZE as idx_t - 1) % blocksize;

    if (st.st_mode & 0o170000) == 0o100000 && adjusted_blocksize & (adjusted_blocksize - 1) != 0 {
        let leading_zeros = stdc_leading_zeros_ull(adjusted_blocksize as u64) as i32;
        if leading_zeros != 0 {
            let power = 1u64 << (64 - leading_zeros);
            if power <= i64::MAX as u64 {
                adjusted_blocksize = power as idx_t;
            }
        }
    }

    let max_value = if (i64::MAX as u64) < u64::MAX {
        i64::MAX as u64
    } else {
        u64::MAX
    };

    if max_value.wrapping_div(2) + 1 < adjusted_blocksize as u64 {
        (max_value.wrapping_div(2) + 1) as idx_t
    } else {
        adjusted_blocksize as u64 as idx_t
    }
}

#[inline]
fn __gl_setmode(fd: i32, mode: i32) -> i32 {
    0
}

#[inline]
unsafe extern "C" fn set_binary_mode(mut fd: libc::c_int, mut mode: libc::c_int) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
#[inline]
fn xset_binary_mode_error() {
    // Implement the functionality that was previously unsafe or C API related.
    // Since the original function does not have any implementation, we will assume
    // it is meant to set some binary mode. Here we can use Rust's standard library
    // features to achieve similar functionality if needed.
    // For now, we will leave it empty, as the original function does nothing.
}

#[inline]
fn xset_binary_mode(fd: i32, mode: i32) {
    unsafe {
        if set_binary_mode(fd, mode) < 0 {
            xset_binary_mode_error();
        }
    }
}

static mut infile: *const libc::c_char = 0 as *const libc::c_char;
static mut input_desc: libc::c_int = 0;
static mut line_buf: [libc::c_char; 20] = [
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '\t' as i32 as libc::c_char,
    '\0' as i32 as libc::c_char,
];
static mut line_num_print: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut line_num_start: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut line_num_end: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut newlines2: libc::c_int = 0 as libc::c_int;
static mut pending_cr: bool = 0 as libc::c_int != 0;
#[no_mangle]
pub unsafe extern "C" fn usage(mut status: libc::c_int) {
    if status != 0 as libc::c_int {
        fprintf(
            stderr,
            gettext(
                b"Try '%s --help' for more information.\n\0" as *const u8 as *const libc::c_char,
            ),
            program_name,
        );
    } else {
        printf(
            gettext(b"Usage: %s [OPTION]... [FILE]...\n\0" as *const u8 as *const libc::c_char),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Concatenate FILE(s) to standard output.\n\0" as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_stdin_note();
        fputs_unlocked(
            gettext(
                b"\n  -A, --show-all           equivalent to -vET\n  -b, --number-nonblank    number nonempty output lines, overrides -n\n  -e                       equivalent to -vE\n  -E, --show-ends          display $ at end of each line\n  -n, --number             number all output lines\n  -s, --squeeze-blank      suppress repeated empty output lines\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -t                       equivalent to -vT\n  -T, --show-tabs          display TAB characters as ^I\n  -u                       (ignored)\n  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB\n\0"
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
                b"      --version     output version information and exit\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        printf(
            gettext(
                b"\nExamples:\n  %s f - g  Output f's contents, then standard input, then g's contents.\n  %s        Copy standard input to standard output.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            program_name,
            program_name,
        );
        let program_str = std::ffi::CStr::from_ptr(b"cat\0".as_ptr() as *const libc::c_char)
            .to_string_lossy()
            .into_owned();
        emit_ancillary_info(&program_str);
    }
    exit(status);
}
fn next_line_num() {
    unsafe {
        let mut endp = line_num_end;
        loop {
            let fresh0 = *endp;
            *endp = (*endp as i32 + 1) as libc::c_char;
            if (fresh0 as i32) < ('9' as i32) {
                return;
            }
            let fresh1 = endp;
            endp = endp.offset(-1);
            *fresh1 = '0' as i32 as libc::c_char;
            if endp < line_num_start {
                break;
            }
        }
        if line_num_start > line_buf.as_mut_ptr() {
            line_num_start = line_num_start.offset(-1);
            *line_num_start = '1' as i32 as libc::c_char;
        } else {
            *line_buf.as_mut_ptr() = '>' as i32 as libc::c_char;
        }
        if line_num_start < line_num_print {
            line_num_print = line_num_print.offset(-1);
        }
    }
}

fn simple_cat(buf: &mut [u8]) -> bool {
    loop {
        let n_read = unsafe {
            safe_read(
                input_desc,
                buf.as_mut_ptr() as *mut libc::c_void,
                buf.len() as size_t,
            )
        };
        if n_read == !(0 as libc::c_int) as size_t {
            let errstatus: libc::c_int = 0;
            unsafe {
                error(
                    errstatus,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(0 as libc::c_int, shell_escape_quoting_style, infile),
                );
            }
            return false;
        }
        if n_read == 0 {
            return true;
        }
        unsafe {
            if full_write(
                1 as libc::c_int,
                buf.as_ptr() as *const libc::c_void,
                n_read,
            ) != n_read
            {
                write_error();
            }
        }
    }
}

#[inline]
unsafe extern "C" fn write_pending(
    mut outbuf: *mut libc::c_char,
    mut bpout: *mut *mut libc::c_char,
) {
    let mut n_write: idx_t = (*bpout).offset_from(outbuf) as libc::c_long;
    if (0 as libc::c_int as libc::c_long) < n_write {
        if full_write(
            1 as libc::c_int,
            outbuf as *const libc::c_void,
            n_write as size_t,
        ) != n_write as libc::c_ulong
        {
            write_error();
        }
        *bpout = outbuf;
    }
}
unsafe extern "C" fn cat(
    mut inbuf: *mut libc::c_char,
    mut insize: idx_t,
    mut outbuf: *mut libc::c_char,
    mut outsize: idx_t,
    mut show_nonprinting: bool,
    mut show_tabs: bool,
    mut number: bool,
    mut number_nonblank: bool,
    mut show_ends: bool,
    mut squeeze_blank: bool,
) -> bool {
    let mut ch: libc::c_uchar = 0;
    let mut newlines: libc::c_int = newlines2;
    let mut use_fionread: bool = 1 as libc::c_int != 0;
    let mut eob: *mut libc::c_char = inbuf;
    let mut bpin: *mut libc::c_char = eob.offset(1 as libc::c_int as isize);
    let mut bpout: *mut libc::c_char = outbuf;
    loop {
        let mut current_block_52: u64;
        loop {
            if (bpout as isize) < (outsize as isize) {
                let mut wp: *mut i8 = outbuf;
                let mut remaining_bytes: i64 = 0;
                loop {
                    if full_write(1, wp as *const libc::c_void, outsize as size_t)
                        != outsize as libc::c_ulong
                    {
                        write_error();
                    }
                    wp = wp.offset(outsize as isize);
                    remaining_bytes = bpout.offset_from(wp) as i64;
                    if !(outsize as i64 <= remaining_bytes) {
                        break;
                    }
                }
                unsafe {
                    std::ptr::copy(wp, outbuf, remaining_bytes as usize);
                }
                bpout = outbuf.offset(remaining_bytes as isize);
            }
            if bpin > eob {
                let mut input_pending: bool = false;
                let mut n_to_read: i32 = 0;

                if use_fionread && ioctl(input_desc, 0x541b, &mut n_to_read) < 0 {
                    match *__errno_location() {
                        95 | 25 | 22 | 19 | 38 => {
                            use_fionread = false;
                        }
                        _ => {
                            error(
                                0,
                                *__errno_location(),
                                gettext(
                                    b"cannot do ioctl on %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, infile),
                            );
                            newlines2 = newlines;
                            return false;
                        }
                    }
                }

                if n_to_read != 0 {
                    input_pending = true;
                }

                if !input_pending {
                    write_pending(outbuf, &mut bpout);
                }

                let n_read: u64 = safe_read(
                    input_desc,
                    inbuf as *mut libc::c_void,
                    insize.try_into().unwrap(),
                );
                if n_read == u64::MAX {
                    error(
                        0,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(0, shell_escape_quoting_style, infile),
                    );
                    write_pending(outbuf, &mut bpout);
                    newlines2 = newlines;
                    return false;
                }

                if n_read == 0 {
                    write_pending(outbuf, &mut bpout);
                    newlines2 = newlines;
                    return true;
                }

                bpin = inbuf;
                eob = bpin.add(n_read as usize);
                *eob = '\n' as i32 as i8;
                current_block_52 = 6476622998065200121;
            } else {
                newlines += 1;
                if newlines > 0 {
                    if newlines >= 2 {
                        newlines = 2;
                        if squeeze_blank {
                            ch = unsafe { *bpin } as u8; // Dereference the raw pointer and cast to u8
                            bpin = bpin.add(1); // Move the pointer forward
                            current_block_52 = 16658872821858055392;
                        } else {
                            current_block_52 = 15597372965620363352;
                        }
                    } else {
                        current_block_52 = 15597372965620363352;
                    }
                    match current_block_52 {
                        16658872821858055392 => {}
                        _ => {
                            if number && !number_nonblank {
                                next_line_num();
                                bpout = stpcpy(bpout, line_num_print);
                            }
                            current_block_52 = 17784502470059252271;
                        }
                    }
                } else {
                    current_block_52 = 17784502470059252271;
                }
                match current_block_52 {
                    16658872821858055392 => {}
                    _ => {
                        if show_ends {
                            if pending_cr {
                                unsafe {
                                    *bpout = '^' as i8;
                                    *(bpout.add(1)) = 'M' as i8;
                                }
                                bpout = bpout.add(2);
                                pending_cr = false;
                            }
                            unsafe {
                                *bpout = '$' as i8;
                            }
                            bpout = bpout.add(1);
                        }
                        unsafe {
                            *bpout = '\n' as i8;
                        }
                        bpout = bpout.add(1);
                        current_block_52 = 6476622998065200121;
                    }
                }
            }
            match current_block_52 {
                6476622998065200121 => {
                    let fresh7 = bpin;
                    bpin = bpin.offset(1);
                    ch = *fresh7 as libc::c_uchar;
                }
                _ => {}
            }
            if !(ch as libc::c_int == '\n' as i32) {
                break;
            }
        }
        if pending_cr {
            *bpout = '\r' as i8;
            bpout = bpout.add(1);
            pending_cr = false;
        }
        if newlines >= 0 && number {
            next_line_num();
            bpout = stpcpy(bpout, line_num_print);
        }
        if show_nonprinting {
            loop {
                if ch >= 32 {
                    if ch < 127 {
                        *bpout = ch as i8;
                        bpout = bpout.add(1);
                    } else if ch == 127 {
                        *bpout = '^' as i8;
                        bpout = bpout.add(1);
                        *bpout = '?' as i8;
                        bpout = bpout.add(1);
                    } else {
                        *bpout = 'M' as i8;
                        bpout = bpout.add(1);
                        *bpout = '-' as i8;
                        if ch >= 128 + 32 {
                            if ch < 128 + 127 {
                                *bpout = (ch - 128) as i8;
                                bpout = bpout.add(1);
                            } else {
                                *bpout = '^' as i8;
                                bpout = bpout.add(1);
                                *bpout = '?' as i8;
                                bpout = bpout.add(1);
                            }
                        } else {
                            *bpout = '^' as i8;
                            bpout = bpout.add(1);
                            *bpout = (ch + 64) as i8;
                            bpout = bpout.add(1);
                        }
                    }
                } else if ch == b'\t' && !show_tabs {
                    *bpout = ch as i8;
                    bpout = bpout.add(1);
                } else if ch == b'\n' {
                    newlines = -1;
                    break;
                } else {
                    *bpout = '^' as i8;
                    bpout = bpout.add(1);
                    *bpout = (ch + 64) as i8;
                    bpout = bpout.add(1);
                }
                ch = *bpin as u8;
                bpin = bpin.add(1);
            }
        } else {
            loop {
                if ch == b'\t' && show_tabs {
                    *bpout = '^' as i8;
                    bpout = bpout.add(1);
                    *bpout = (ch + 64) as i8;
                    bpout = bpout.add(1);
                } else if ch != b'\n' {
                    if ch == b'\r' && *bpin as i8 == b'\n' as i8 && show_ends {
                        if bpin == eob {
                            pending_cr = true;
                        } else {
                            *bpout = '^' as i8;
                            bpout = bpout.add(1);
                            *bpout = 'M' as i8;
                            bpout = bpout.add(1);
                        }
                    } else {
                        *bpout = ch as i8;
                        bpout = bpout.add(1);
                    }
                } else {
                    newlines = -1;
                    break;
                }
                ch = *bpin as u8;
                bpin = bpin.add(1);
            }
        }
        /*
        The variables live at this point are:
        (mut inbuf: *mut i8, mut insize: i64, mut outbuf: *mut i8, mut outsize: i64, mut show_nonprinting: bool, mut show_tabs: bool, mut number: bool, mut number_nonblank: bool, mut show_ends: bool, mut squeeze_blank: bool, mut ch: u8, mut newlines: i32, mut use_fionread: bool, mut eob: *mut i8, mut bpin: *mut i8, mut bpout: *mut i8, mut current_block_52: u64)
        */
    }
}
unsafe extern "C" fn copy_cat() -> libc::c_int {
    let mut copy_max: ssize_t = (((if (9223372036854775807 as libc::c_long as libc::c_ulong)
        < 18446744073709551615 as libc::c_ulong
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        18446744073709551615 as libc::c_ulong
    }) >> 30 as libc::c_int)
        << 30 as libc::c_int) as ssize_t;
    let mut some_copied: bool = 0 as libc::c_int != 0;
    loop {
        match rpl_copy_file_range(
            input_desc,
            0 as *mut off_t,
            1 as libc::c_int,
            0 as *mut off_t,
            copy_max as size_t,
            0 as libc::c_int as libc::c_uint,
        ) {
            0 => return some_copied as libc::c_int,
            -1 => {
                if *__errno_location() == 38 as libc::c_int
                    || is_ENOTSUP(*__errno_location()) as libc::c_int != 0
                    || *__errno_location() == 22 as libc::c_int
                    || *__errno_location() == 9 as libc::c_int
                    || *__errno_location() == 18 as libc::c_int
                    || *__errno_location() == 26 as libc::c_int
                    || *__errno_location() == 1 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if 0 as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {
                    };
                } else {
                    ({
                        let __errstatus: libc::c_int = 0 as libc::c_int;
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
                        } else {
                        };
                    });
                    ({
                        let __errstatus: libc::c_int = 0 as libc::c_int;
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
                        } else {
                        };
                    });
                };
                return -(1 as libc::c_int);
            }
            _ => {}
        }
        some_copied = 1 as libc::c_int != 0;
    }
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut insize: idx_t = 0;
    let mut inbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut have_read_stdin: bool = 0 as libc::c_int != 0;
    let mut stat_buf: stat = stat {
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 2],
    };
    let mut number: bool = 0 as libc::c_int != 0;
    let mut number_nonblank: bool = 0 as libc::c_int != 0;
    let mut squeeze_blank: bool = 0 as libc::c_int != 0;
    let mut show_ends: bool = 0 as libc::c_int != 0;
    let mut show_nonprinting: bool = 0 as libc::c_int != 0;
    let mut show_tabs: bool = 0 as libc::c_int != 0;
    let mut file_open_mode: libc::c_int = 0 as libc::c_int;
    static mut long_options: [option; 10] = [
        {
            let mut init = option {
                name: b"number-nonblank\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"number\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"squeeze-blank\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-nonprinting\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-ends\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'E' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-tabs\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'T' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-all\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'A' as i32,
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
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    let mut c: libc::c_int = 0;
    loop {
        c = getopt_long(
            argc,
            argv,
            b"benstuvAET\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            98 => {
                number = 1 as libc::c_int != 0;
                number_nonblank = 1 as libc::c_int != 0;
            }
            101 => {
                show_ends = 1 as libc::c_int != 0;
                show_nonprinting = 1 as libc::c_int != 0;
            }
            110 => {
                number = 1 as libc::c_int != 0;
            }
            115 => {
                squeeze_blank = 1 as libc::c_int != 0;
            }
            116 => {
                show_tabs = 1 as libc::c_int != 0;
                show_nonprinting = 1 as libc::c_int != 0;
            }
            117 => {}
            118 => {
                show_nonprinting = 1 as libc::c_int != 0;
            }
            65 => {
                show_nonprinting = 1 as libc::c_int != 0;
                show_ends = 1 as libc::c_int != 0;
                show_tabs = 1 as libc::c_int != 0;
            }
            69 => {
                show_ends = 1 as libc::c_int != 0;
            }
            84 => {
                show_tabs = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"cat\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name_lite(
                        b"Torbjorn Granlund\0" as *const u8 as *const libc::c_char,
                        b"Torbj\xC3\xB6rn Granlund\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"Richard M. Stallman\0" as *const u8 as *const libc::c_char,
                        b"Richard M. Stallman\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if fstat(1 as libc::c_int, &mut stat_buf) < 0 as libc::c_int {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(b"standard output\0" as *const u8 as *const libc::c_char),
            );
            if 1 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {
            };
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
                } else {
                };
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
                } else {
                };
            });
        };
    }
    let mut outsize: idx_t = io_blksize(&stat_buf);
    let mut out_dev: dev_t = stat_buf.st_dev;
    let mut out_ino: ino_t = stat_buf.st_ino;
    let mut out_flags: libc::c_int = -(2 as libc::c_int);
    let mut out_isreg: bool = (stat_buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int
        != 0 as libc::c_int;
    if !(number as libc::c_int != 0
        || show_ends as libc::c_int != 0
        || squeeze_blank as libc::c_int != 0)
    {
        file_open_mode |= 0 as libc::c_int;
        xset_binary_mode(1, 0);
    }
    infile = b"-\0" as *const u8 as *const libc::c_char;
    let mut argind: libc::c_int = optind;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut page_size: idx_t = getpagesize() as idx_t;
    loop {
        if argind < argc {
            infile = *argv.offset(argind as isize);
        }
        let mut reading_stdin: bool =
            strcmp(infile, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int;
        if reading_stdin {
            have_read_stdin = 1 as libc::c_int != 0;
            input_desc = 0 as libc::c_int;
            if file_open_mode & 0 as libc::c_int != 0 {
                xset_binary_mode(0, 0);
            }
            current_block = 13321564401369230990;
        } else {
            input_desc = open(infile, file_open_mode);
            if input_desc < 0 as libc::c_int {
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            infile,
                        ),
                    );
                    if 0 as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {
                    };
                } else {
                    ({
                        let __errstatus: libc::c_int = 0 as libc::c_int;
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
                        } else {
                        };
                    });
                    ({
                        let __errstatus: libc::c_int = 0 as libc::c_int;
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
                        } else {
                        };
                    });
                };
                ok = 0 as libc::c_int != 0;
                current_block = 4567019141635105728;
            } else {
                current_block = 13321564401369230990;
            }
        }
        match current_block {
            13321564401369230990 => {
                if fstat(input_desc, &mut stat_buf) < 0 as libc::c_int {
                    if 0 != 0 {
                        error(
                            0,
                            std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                            b"%s\0".as_ptr() as *const i8,
                            quotearg_n_style_colon(0, shell_escape_quoting_style, infile),
                        );
                        if 0 != 0 {
                            unreachable!();
                        }
                    } else {
                        {
                            let errstatus: i32 = 0;
                            error(
                                errstatus,
                                std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                                b"%s\0".as_ptr() as *const i8,
                                quotearg_n_style_colon(0, shell_escape_quoting_style, infile),
                            );
                            if errstatus != 0 {
                                unreachable!();
                            }
                        }
                        {
                            let errstatus: i32 = 0;
                            error(
                                errstatus,
                                std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
                                b"%s\0".as_ptr() as *const i8,
                                quotearg_n_style_colon(0, shell_escape_quoting_style, infile),
                            );
                            if errstatus != 0 {
                                unreachable!();
                            }
                        }
                    }
                    ok = false;
                } else {
                    insize = io_blksize(&stat_buf);
                    fdadvise(input_desc, 0, 0, FADVISE_SEQUENTIAL);
                    if stat_buf.st_dev == out_dev && stat_buf.st_ino == out_ino {
                        if out_flags < -1 {
                            out_flags = rpl_fcntl(1, 3);
                        }
                        let mut exhausting = out_flags >= 0 && (out_flags & 0o2000) != 0;
                        if !exhausting {
                            let in_pos = lseek(input_desc, 0, 1);
                            if in_pos >= 0 {
                                exhausting = in_pos < lseek(1, 0, 1);
                            }
                        }
                        if exhausting {
                            error(
                                0,
                                0,
                                gettext(
                                    b"%s: input file is output file\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quotearg_n_style_colon(0, shell_escape_quoting_style, infile),
                            );
                            ok = false;
                            current_block = 7239751344758050955;
                        } else {
                            current_block = 5372832139739605200;
                        }
                    } else {
                        current_block = 5372832139739605200;
                    }
                    match current_block {
                        7239751344758050955 => {}
                        _ => {
                            let mut inbuf: *mut i8 = std::ptr::null_mut(); // Using a raw pointer initialized to null
                            if !(number
                                || show_ends
                                || show_nonprinting
                                || show_tabs
                                || squeeze_blank)
                            {
                                let copy_cat_status =
                                    if out_isreg && (stat_buf.st_mode & 0o170000) == 0o100000 {
                                        copy_cat()
                                    } else {
                                        0
                                    };

                                if copy_cat_status != 0 {
                                    inbuf = std::ptr::null_mut();
                                    ok &= (0 < copy_cat_status);
                                } else {
                                    insize = insize.max(outsize);
                                    let aligned_buf = xalignalloc(page_size, insize) as *mut u8;
                                    let buf_slice = unsafe {
                                        std::slice::from_raw_parts_mut(aligned_buf, insize as usize)
                                    };
                                    ok &= simple_cat(buf_slice);
                                }
                            } else {
                                inbuf = xalignalloc(
                                    page_size,
                                    insize + 1 as libc::c_int as libc::c_long,
                                ) as *mut libc::c_char;
                                let mut bufsize: idx_t = 0;
                                if (if (0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t
                                    && ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        insize
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    && ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        4 as libc::c_int
                                    }) - 1 as libc::c_int)
                                        < 0 as libc::c_int
                                    && (if (4 as libc::c_int) < 0 as libc::c_int {
                                        if insize < 0 as libc::c_int as libc::c_long {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    -(1 as libc::c_int) as idx_t
                                                }) + 4 as libc::c_int as libc::c_long
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                (insize
                                                    < -(1 as libc::c_int) as idx_t
                                                        / 4 as libc::c_int as libc::c_long)
                                                    as libc::c_int
                                            } else {
                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    4 as libc::c_int
                                                }) - 1 as libc::c_int)
                                                    < 0 as libc::c_int
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        4 as libc::c_int
                                                    }) + 1 as libc::c_int)
                                                        << (::core::mem::size_of::<libc::c_int>()
                                                            as libc::c_ulong)
                                                            .wrapping_mul(
                                                                8 as libc::c_int as libc::c_ulong,
                                                            )
                                                            .wrapping_sub(
                                                                2 as libc::c_int as libc::c_ulong,
                                                            ))
                                                        - 1 as libc::c_int)
                                                        * 2 as libc::c_int
                                                        + 1 as libc::c_int)
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        4 as libc::c_int
                                                    }) + 0 as libc::c_int
                                                }) < 0 as libc::c_int
                                                {
                                                    ((4 as libc::c_int)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int
                                                        } else {
                                                            4 as libc::c_int
                                                        }) - 1 as libc::c_int)
                                                            < 0 as libc::c_int
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                4 as libc::c_int
                                                            }) + 1 as libc::c_int)
                                                                << (::core::mem::size_of::<
                                                                    libc::c_int,
                                                                >(
                                                                )
                                                                    as libc::c_ulong)
                                                                    .wrapping_mul(
                                                                        8 as libc::c_int
                                                                            as libc::c_ulong,
                                                                    )
                                                                    .wrapping_sub(
                                                                        2 as libc::c_int
                                                                            as libc::c_ulong,
                                                                    ))
                                                                - 1 as libc::c_int)
                                                                * 2 as libc::c_int
                                                                + 1 as libc::c_int
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int
                                                            } else {
                                                                4 as libc::c_int
                                                            }) - 1 as libc::c_int
                                                        }))
                                                        as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int) < 4 as libc::c_int)
                                                        as libc::c_int
                                                }) != 0
                                                {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        4 as libc::c_int
                                                    })
                                                        as libc::c_long
                                                        + -(1 as libc::c_int) as idx_t
                                                        >> (::core::mem::size_of::<libc::c_int>()
                                                            as libc::c_ulong)
                                                            .wrapping_mul(
                                                                8 as libc::c_int as libc::c_ulong,
                                                            )
                                                            .wrapping_sub(
                                                                1 as libc::c_int as libc::c_ulong,
                                                            )
                                                } else {
                                                    -(1 as libc::c_int) as idx_t
                                                        / -(4 as libc::c_int) as libc::c_long
                                                }) <= -(1 as libc::c_int) as libc::c_long - insize)
                                                    as libc::c_int
                                            }
                                        } else {
                                            if {
                                                let condition1 = if 1 != 0 {
                                                    0
                                                } else {
                                                    (if 1 != 0 { 0 } else { 4 }) + 0
                                                } - 1
                                                    < 0;

                                                if condition1 {
                                                    let inner_value = (if 1 != 0 {
                                                        0
                                                    } else {
                                                        (if 1 != 0 { 0 } else { 4 }) + 0
                                                    } + 1)
                                                        << (std::mem::size_of::<i64>() * 8 - 2) - 1;
                                                    !((inner_value * 2) + 1) < 0
                                                } else {
                                                    let outer_value = if 1 != 0 {
                                                        0
                                                    } else {
                                                        (if 1 != 0 { 0 } else { 4 }) + 0
                                                    };
                                                    outer_value < 0
                                                }
                                            } {
                                                let condition2 = {
                                                    let inner_value =
                                                        if 1 != 0 { 0 } else { 4 + 0 };
                                                    inner_value < 0
                                                        && (if 1 != 0 {
                                                            0
                                                        } else {
                                                            (if 1 != 0 { 0 } else { 4 }) + 0
                                                        } - 1
                                                            < 0)
                                                };

                                                if condition2 {
                                                    let inner_value = (if 1 != 0 {
                                                        0
                                                    } else {
                                                        (if 1 != 0 { 0 } else { 4 }) + 0
                                                    } + 1)
                                                        << (std::mem::size_of::<i64>() * 8 - 2) - 1;
                                                    inner_value * 2 + 1
                                                } else {
                                                    (if 1 != 0 {
                                                        0
                                                    } else {
                                                        (if 1 != 0 { 0 } else { 4 }) + 0
                                                    } - 1)
                                                }
                                            } else {
                                                if (0 < (if 1 != 0 { 0 } else { 4 } + 0))
                                                    && (4 == -1)
                                                {
                                                    if (if 1 != 0 { 0 } else { insize } - 1) < 0 {
                                                        (0 < insize + 0) as i32
                                                    } else {
                                                        (0 < insize && (-1 - 0) < insize - 1) as i32
                                                    }
                                                } else {
                                                    (0 / 4 < insize) as i32
                                                }
                                            }
                                        }
                                    } else {
                                        if 4 as libc::c_int == 0 as libc::c_int {
                                            0 as libc::c_int
                                        } else {
                                            if insize < 0 as libc::c_int as libc::c_long {
                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        insize
                                                    }) + 0 as libc::c_int as idx_t
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    !(((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            insize
                                                        }) + 0 as libc::c_int as idx_t
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::core::mem::size_of::<libc::c_long>()
                                                            as libc::c_ulong)
                                                            .wrapping_mul(
                                                                8 as libc::c_int as libc::c_ulong,
                                                            )
                                                            .wrapping_sub(
                                                                2 as libc::c_int as libc::c_ulong,
                                                            ))
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
                                                            insize
                                                        }) + 0 as libc::c_int as idx_t
                                                    }) + 0 as libc::c_int as libc::c_long
                                                }) < 0 as libc::c_int as libc::c_long
                                                {
                                                    (((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        insize
                                                    }) + 0 as libc::c_int as idx_t)
                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                insize
                                                            }) + 0 as libc::c_int as idx_t
                                                        }) - 1 as libc::c_int
                                                            as libc::c_long)
                                                            < 0 as libc::c_int as libc::c_long
                                                        {
                                                            ((((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    insize
                                                                }) + 0 as libc::c_int as idx_t
                                                            }) + 1 as libc::c_int
                                                                as libc::c_long)
                                                                << (::core::mem::size_of::<
                                                                    libc::c_long,
                                                                >(
                                                                )
                                                                    as libc::c_ulong)
                                                                    .wrapping_mul(
                                                                        8 as libc::c_int
                                                                            as libc::c_ulong,
                                                                    )
                                                                    .wrapping_sub(
                                                                        2 as libc::c_int
                                                                            as libc::c_ulong,
                                                                    ))
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
                                                                    insize
                                                                }) + 0 as libc::c_int as idx_t
                                                            }) - 1 as libc::c_int as libc::c_long
                                                        }))
                                                        as libc::c_int
                                                } else {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            insize
                                                        }) + 0 as libc::c_int as idx_t)
                                                        as libc::c_int
                                                }) != 0
                                                    && insize == -(1 as libc::c_int) as libc::c_long
                                                {
                                                    if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        4 as libc::c_int
                                                    }) - 1 as libc::c_int)
                                                        < 0 as libc::c_int
                                                    {
                                                        ((0 as libc::c_int as libc::c_long)
                                                            < 4 as libc::c_int as libc::c_long
                                                                + 0 as libc::c_int as idx_t)
                                                            as libc::c_int
                                                    } else {
                                                        ((-(1 as libc::c_int) as libc::c_long
                                                            - 0 as libc::c_int as idx_t)
                                                            < (4 as libc::c_int - 1 as libc::c_int)
                                                                as libc::c_long)
                                                            as libc::c_int
                                                    }
                                                } else {
                                                    (0 as libc::c_int as idx_t / insize
                                                        < 4 as libc::c_int as libc::c_long)
                                                        as libc::c_int
                                                }
                                            } else {
                                                ((-(1 as libc::c_int) as idx_t
                                                    / 4 as libc::c_int as libc::c_long)
                                                    < insize)
                                                    as libc::c_int
                                            }
                                        }
                                    }) != 0
                                {
                                    let (fresh33, _fresh34) =
                                        insize.overflowing_mul((4 as libc::c_int).into());
                                    *(&mut bufsize as *mut idx_t) = fresh33;
                                    1 as libc::c_int
                                } else {
                                    let (fresh35, fresh36) =
                                        insize.overflowing_mul((4 as libc::c_int).into());
                                    *(&mut bufsize as *mut idx_t) = fresh35;
                                    fresh36 as libc::c_int
                                }) != 0
                                    || {
                                        let (fresh37, fresh38) = bufsize.overflowing_add(outsize);
                                        *(&mut bufsize as *mut idx_t) = fresh37;
                                        fresh38 as libc::c_int != 0
                                    }
                                    || {
                                        let (fresh39, fresh40) = bufsize.overflowing_add(
                                            (20 as libc::c_int - 1 as libc::c_int).into(),
                                        );
                                        *(&mut bufsize as *mut idx_t) = fresh39;
                                        fresh40 as libc::c_int != 0
                                    }
                                {
                                    xalloc_die();
                                }
                                let mut outbuf: *mut libc::c_char =
                                    xalignalloc(page_size, bufsize) as *mut libc::c_char;
                                ok = (ok as libc::c_int
                                    & cat(
                                        inbuf,
                                        insize,
                                        outbuf,
                                        outsize,
                                        show_nonprinting,
                                        show_tabs,
                                        number,
                                        number_nonblank,
                                        show_ends,
                                        squeeze_blank,
                                    ) as libc::c_int)
                                    != 0;
                                alignfree(outbuf as *mut libc::c_void);
                            }
                            alignfree(inbuf as *mut libc::c_void); // Assuming alignfree can accept a pointer from Vec
                        }
                    }
                }
                if !reading_stdin && close(input_desc) < 0 as libc::c_int {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                infile,
                            ),
                        );
                        if 0 as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {
                        };
                    } else {
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
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
                            } else {
                            };
                        });
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
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
                            } else {
                            };
                        });
                    };
                    ok = 0 as libc::c_int != 0;
                }
            }
            _ => {}
        }
        argind += 1;
        if !(argind < argc) {
            break;
        }
    }
    if pending_cr {
        if full_write(
            1 as libc::c_int,
            b"\r\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            write_error();
        }
    }
    if have_read_stdin && close(0) < 0 {
        if true {
            error(
                1,
                *__errno_location(),
                gettext(b"closing standard input\0" as *const u8 as *const libc::c_char),
            );
            unreachable!();
        } else {
            let errstatus = 1;
            error(
                errstatus,
                *__errno_location(),
                gettext(b"closing standard input\0" as *const u8 as *const libc::c_char),
            );
            if errstatus != 0 {
                unreachable!();
            }

            let errstatus = 1;
            error(
                errstatus,
                *__errno_location(),
                gettext(b"closing standard input\0" as *const u8 as *const libc::c_char),
            );
            if errstatus != 0 {
                unreachable!();
            }
        }
    }
    return if ok { 0 } else { 1 };
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
unsafe extern "C" fn run_static_initializers() {
    let line_buf_slice = &mut line_buf[..];
    line_num_print = line_buf_slice.as_mut_ptr().offset(12); // 20 - 8
    line_num_start = line_buf_slice.as_mut_ptr().offset(17); // 20 - 3
    line_num_end = line_buf_slice.as_mut_ptr().offset(17); // 20 - 3
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
