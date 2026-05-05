#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types)]













use std::ffi::CString;
use std::ptr;
use std::process;

use std::ffi::CStr;
use std::io::Write;

use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut Version: *const libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn exit(_: libc::c_int) -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn xdectoimax(
        n_str: *const libc::c_char,
        min: intmax_t,
        max: intmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> intmax_t;
}
pub type __intmax_t = libc::c_long;
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
pub type ptrdiff_t = libc::c_long;
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
pub type intmax_t = __intmax_t;
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
pub type rel_mode_t = libc::c_uint;
pub const rm_rup: rel_mode_t = 5;
pub const rm_rdn: rel_mode_t = 4;
pub const rm_max: rel_mode_t = 3;
pub const rm_min: rel_mode_t = 2;
pub const rm_rel: rel_mode_t = 1;
pub const rm_abs: rel_mode_t = 0;
#[inline]
fn usable_st_size(sb: &stat) -> bool {
    (sb.st_mode & 0o170000 as libc::c_uint == 0o100000 as libc::c_uint)
        || (sb.st_mode & 0o170000 as libc::c_uint == 0o120000 as libc::c_uint)
        || (sb.st_mode.wrapping_sub(sb.st_mode) != 0 || 0 != 0)
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
        CStr::from_bytes_with_nul(b"GNU coreutils\0").unwrap().to_string_lossy(),
        help_url.to_string_lossy()
    );

    let lc_messages: *const libc::c_char;
    unsafe {
        lc_messages = setlocale(5, std::ptr::null());
    }
    
    if !lc_messages.is_null() {
        let lc_messages_str = unsafe { CStr::from_ptr(lc_messages) };
        if !lc_messages_str.to_string_lossy().starts_with("en_") {
            writeln!(
                std::io::stdout(),
                "{}",
                CStr::from_bytes_with_nul(b"Report any translation bugs to <https://translationproject.org/team/>\n\0").unwrap().to_string_lossy()
            ).unwrap();
        }
    }

    let url_program = if program.to_bytes() == b"[\0" {
        CStr::from_bytes_with_nul(b"test\0").unwrap()
    } else {
        program
    };

    println!(
        "Full documentation <{}{}>",
        help_url.to_string_lossy(),
        url_program.to_string_lossy()
    );

    let invocation_suffix = if node == program.as_ptr() {
        CStr::from_bytes_with_nul(b" invocation\0").unwrap().to_string_lossy()
    } else {
        "".into()
    };

    println!(
        "or available locally via: info '(coreutils) {}{}'",
        unsafe { CStr::from_ptr(node) }.to_string_lossy(),
        invocation_suffix
    );
}

#[inline]
fn to_uchar(ch: i8) -> u8 {
    ch as u8
}

#[inline]
fn emit_mandatory_arg_note() {
    let message = "\nMandatory arguments to long options are mandatory for short options too.\n";
    print!("{}", message);
}

#[inline]
fn emit_size_note() {
    let message = "\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y,R,Q (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n";
    let translated_message = unsafe { gettext(message.as_ptr() as *const libc::c_char) };
    let _ = std::io::stdout().write_all(unsafe { std::ffi::CStr::from_ptr(translated_message).to_bytes() });
}

static mut no_create: bool = false;
static mut block_mode: bool = false;
static mut ref_file: *const libc::c_char = 0 as *const libc::c_char;
static mut longopts: [option; 7] = [
    {
        let mut init = option {
            name: b"no-create\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"io-blocks\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"reference\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"size\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
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
            unsafe { CStr::from_ptr(gettext(format!("Try '{} --help' for more information.\n", unsafe { CStr::from_ptr(program_name).to_string_lossy() }).as_ptr() as *const i8)).to_string_lossy() }
        );
    } else {
        println!(
            "{}", 
            unsafe { CStr::from_ptr(gettext(format!("Usage: {} OPTION... FILE...\n", unsafe { CStr::from_ptr(program_name).to_string_lossy() }).as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { CStr::from_ptr(gettext("Shrink or extend the size of each FILE to the specified size\n\nA FILE argument that does not exist is created.\n\nIf a FILE is larger than the specified size, the extra data is lost.\nIf a FILE is shorter, it is extended and the sparse extended part (hole)\nreads as zero bytes.\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        emit_mandatory_arg_note();
        print!(
            "{}",
            unsafe { CStr::from_ptr(gettext("  -c, --no-create        do not create any files\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { CStr::from_ptr(gettext("  -o, --io-blocks        treat SIZE as number of IO blocks instead of bytes\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { CStr::from_ptr(gettext("  -r, --reference=RFILE  base size on RFILE\n  -s, --size=SIZE        set or adjust the file size by SIZE bytes\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { CStr::from_ptr(gettext("      --help        display this help and exit\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        print!(
            "{}",
            unsafe { CStr::from_ptr(gettext("      --version     output version information and exit\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        emit_size_note();
        print!(
            "{}",
            unsafe { CStr::from_ptr(gettext("\nSIZE may also be prefixed by one of the following modifying characters:\n'+' extend by, '-' reduce by, '<' at most, '>' at least,\n'/' round down to multiple of, '%' round up to multiple of.\n".as_ptr() as *const i8)).to_string_lossy() }
        );
        emit_ancillary_info(CStr::from_bytes_with_nul(b"truncate\0").unwrap());
    }
    std::process::exit(status);
}

unsafe extern "C" fn do_ftruncate(
    mut fd: libc::c_int,
    mut fname: *const libc::c_char,
    mut ssize: off_t,
    mut rsize: off_t,
    mut rel_mode: rel_mode_t,
) -> bool {
    let mut sb: stat = stat {
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
    let mut nsize: off_t = 0;
    if (block_mode as libc::c_int != 0
        || rel_mode as libc::c_uint != 0 && rsize < 0 as libc::c_int as libc::c_long)
        && fstat(fd, &mut sb) != 0 as libc::c_int
    {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, fname),
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
                    quotearg_style(shell_escape_always_quoting_style, fname),
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
                    quotearg_style(shell_escape_always_quoting_style, fname),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        return 0 as libc::c_int != 0;
    }
    if block_mode {
        let mut blksize: ptrdiff_t = (if (0 as libc::c_int) < sb.st_blksize
            && sb.st_blksize as libc::c_ulong
                <= (-(1 as libc::c_int) as size_t)
                    .wrapping_div(8 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            sb.st_blksize
        } else {
            512 as libc::c_int
        }) as ptrdiff_t;
        let mut ssize0: intmax_t = ssize;
        if if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t
            && ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                ssize
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            && ((if 1 as libc::c_int != 0 {
                0 as libc::c_int as libc::c_long
            } else {
                blksize
            }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            && (if blksize < 0 as libc::c_int as libc::c_long {
                if ssize < 0 as libc::c_int as libc::c_long {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            -(1 as libc::c_int) as off_t
                        }) + blksize
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (ssize < -(1 as libc::c_int) as off_t / blksize) as libc::c_int
                    } else {
                        ((if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (blksize
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < blksize) as libc::c_int
                        }) != 0
                        {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + -(1 as libc::c_int) as off_t
                                >> (::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            -(1 as libc::c_int) as off_t / -blksize
                        }) <= -(1 as libc::c_int) as libc::c_long - ssize) as libc::c_int
                    }
                } else {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as off_t
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as off_t
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
                                blksize
                            }) + 0 as libc::c_int as off_t
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            blksize
                        }) + 0 as libc::c_int as off_t)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    blksize
                                }) + 0 as libc::c_int as off_t
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        blksize
                                    }) + 0 as libc::c_int as off_t
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
                                        blksize
                                    }) + 0 as libc::c_int as off_t
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) + 0 as libc::c_int as off_t) as libc::c_int
                    }) != 0 && blksize == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            ssize
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < ssize + 0 as libc::c_int as off_t) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < ssize
                                && (-(1 as libc::c_int) as libc::c_long
                                    - 0 as libc::c_int as off_t)
                                    < ssize - 1 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        (0 as libc::c_int as off_t / blksize < ssize) as libc::c_int
                    }
                }
            } else {
                if blksize == 0 as libc::c_int as libc::c_long {
                    0 as libc::c_int
                } else {
                    if ssize < 0 as libc::c_int as libc::c_long {
                        if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            }) + 0 as libc::c_int as off_t
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) + 0 as libc::c_int as off_t
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
                                    ssize
                                }) + 0 as libc::c_int as off_t
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                ssize
                            }) + 0 as libc::c_int as off_t)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        ssize
                                    }) + 0 as libc::c_int as off_t
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            ssize
                                        }) + 0 as libc::c_int as off_t
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
                                            ssize
                                        }) + 0 as libc::c_int as off_t
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    ssize
                                }) + 0 as libc::c_int as off_t) as libc::c_int
                        }) != 0 && ssize == -(1 as libc::c_int) as libc::c_long
                        {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                blksize
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((0 as libc::c_int as libc::c_long)
                                    < blksize + 0 as libc::c_int as off_t) as libc::c_int
                            } else {
                                ((-(1 as libc::c_int) as libc::c_long
                                    - 0 as libc::c_int as off_t)
                                    < blksize - 1 as libc::c_int as libc::c_long) as libc::c_int
                            }
                        } else {
                            (0 as libc::c_int as off_t / ssize < blksize) as libc::c_int
                        }
                    } else {
                        (-(1 as libc::c_int) as off_t / blksize < ssize) as libc::c_int
                    }
                }
            }) != 0
        {
            let (fresh4, _fresh5) = ssize.overflowing_mul(blksize);
            *(&mut ssize as *mut off_t) = fresh4;
            1 as libc::c_int
        } else {
            let (fresh6, fresh7) = ssize.overflowing_mul(blksize);
            *(&mut ssize as *mut off_t) = fresh6;
            fresh7 as libc::c_int
        } != 0
        {
            if false {
    error(
        0,
        0,
        gettext(b"overflow in %jd * %td byte blocks for file %s\0" as *const u8 as *const libc::c_char),
        ssize0,
        blksize,
        quotearg_style(shell_escape_always_quoting_style, fname),
    );
    unreachable!();
} else {
    let __errstatus: i32 = 0;
    error(
        __errstatus,
        0,
        gettext(b"overflow in %jd * %td byte blocks for file %s\0" as *const u8 as *const libc::c_char),
        ssize0,
        blksize,
        quotearg_style(shell_escape_always_quoting_style, fname),
    );
    unreachable!();
    
    let __errstatus: i32 = 0;
    error(
        __errstatus,
        0,
        gettext(b"overflow in %jd * %td byte blocks for file %s\0" as *const u8 as *const libc::c_char),
        ssize0,
        blksize,
        quotearg_style(shell_escape_always_quoting_style, fname),
    );
    unreachable!();
}
return false;

        }
    }
    if rel_mode as u64 != 0 {
        let mut fsize: i64 = 0;
if rsize >= 0 {
    fsize = rsize;
} else if usable_st_size(&sb) {
    fsize = sb.st_size;
    if fsize < 0 {
        error(
            0,
            0,
            gettext(b"%s has unusable, apparently negative size\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, fname),
        );
        return false;
    }
} else {
    fsize = lseek(fd, 0, 2);
    if fsize < 0 {
        error(
            0,
            *__errno_location(),
            gettext(b"cannot get the size of %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, fname),
        );
        return false;
    }
}

        if rel_mode == rm_min {
    nsize = fsize.max(ssize);
} else if rel_mode == rm_max {
    nsize = fsize.min(ssize);
} else if rel_mode == rm_rdn {
    nsize = fsize - fsize % ssize;
} else {
    if rel_mode == rm_rup {
        let r: i64 = fsize % ssize;
        ssize = if r == 0 { 0 } else { ssize - r };
    }
    let (new_nsize, overflowed) = fsize.overflowing_add(ssize);
    nsize = new_nsize;
    if overflowed {
        let __errstatus: i32 = 0;
        error(
            __errstatus,
            0,
            gettext(b"overflow extending size of file %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, fname),
        );
        return false as i32 != 0;
    }
}

    } else {
        nsize = ssize;
    }
    if nsize < 0 as libc::c_int as libc::c_long {
        nsize = 0 as libc::c_int as off_t;
    }
    if ftruncate(fd, nsize) != 0 as libc::c_int {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"failed to truncate %s at %jd bytes\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, fname),
                nsize,
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
                        b"failed to truncate %s at %jd bytes\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, fname),
                    nsize,
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
                        b"failed to truncate %s at %jd bytes\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, fname),
                    nsize,
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut got_size: bool = 0 as libc::c_int != 0;
    let mut size: off_t = 0 as libc::c_int as off_t;
    let mut rsize: off_t = -(1 as libc::c_int) as off_t;
    let mut rel_mode: rel_mode_t = rm_abs;
    let mut c: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        c = getopt_long(
            argc,
            argv,
            b"cor:s:\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            99 => {
                no_create = 1 as libc::c_int != 0;
            }
            111 => {
                block_mode = 1 as libc::c_int != 0;
            }
            114 => {
                ref_file = optarg;
            }
            115 => {
                while *(*__ctype_b_loc())
                    .offset(to_uchar(*optarg) as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    optarg = optarg.offset(1);
                    optarg;
                }
                match *optarg as libc::c_int {
                    60 => {
                        rel_mode = rm_max;
                        optarg = optarg.offset(1);
                        optarg;
                    }
                    62 => {
                        rel_mode = rm_min;
                        optarg = optarg.offset(1);
                        optarg;
                    }
                    47 => {
                        rel_mode = rm_rdn;
                        optarg = optarg.offset(1);
                        optarg;
                    }
                    37 => {
                        rel_mode = rm_rup;
                        optarg = optarg.offset(1);
                        optarg;
                    }
                    _ => {}
                }
                while *(*__ctype_b_loc())
                    .offset(to_uchar(*optarg) as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    optarg = optarg.offset(1);
                    optarg;
                }
                if *optarg as libc::c_int == '+' as i32
                    || *optarg as libc::c_int == '-' as i32
                {
                    if rel_mode as u64 != 0 {
                        if 0 != 0 {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"multiple relative modifiers specified\0" as *const u8
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
                                        b"multiple relative modifiers specified\0" as *const u8
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
                                        b"multiple relative modifiers specified\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                        usage(1);
                    }
                    rel_mode = rm_rel;
                }
                size = xdectoimax(
                    optarg,
                    !if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                        -(1 as libc::c_int) as off_t
                    } else {
                        (((1 as libc::c_int as off_t)
                            << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    },
                    if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                        -(1 as libc::c_int) as off_t
                    } else {
                        (((1 as libc::c_int as off_t)
                            << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    },
                    b"EgGkKmMPQRtTYZ0\0" as *const u8 as *const libc::c_char,
                    gettext(b"Invalid number\0" as *const u8 as *const libc::c_char),
                    0 as libc::c_int,
                );
                if (rel_mode == rm_rup || rel_mode == rm_rdn) && size == 0 {
    if 0 != 0 {
        error(
            1,
            0,
            gettext(b"division by zero\0".as_ptr() as *const libc::c_char),
        );
        if true {
            unreachable!();
        }
    } else {
        let __errstatus: i32 = 1;
        error(
            __errstatus,
            0,
            gettext(b"division by zero\0".as_ptr() as *const libc::c_char),
        );
        if __errstatus != 0 {
            unreachable!();
        }

        let __errstatus: i32 = 1;
        error(
            __errstatus,
            0,
            gettext(b"division by zero\0".as_ptr() as *const libc::c_char),
        );
        if __errstatus != 0 {
            unreachable!();
        }
    }
}
got_size = true;

            }
            -2 => {
                usage(0);
            }
            -3 => {
                version_etc(
    stdout,
    CString::new("truncate").unwrap().as_ptr(),
    CString::new("GNU coreutils").unwrap().as_ptr(),
    Version,
    proper_name_lite(
        CString::new("Padraig Brady").unwrap().as_ptr(),
        CString::new("Pádraig Brady").unwrap().as_ptr(),
    ),
    std::ptr::null_mut::<libc::c_char>(),
);
std::process::exit(0);

            }
            _ => {
                usage(1);
            }
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if ref_file.is_null() && !got_size {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"you must specify either %s or %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quote_n(
                    0 as libc::c_int,
                    b"--size\0" as *const u8 as *const libc::c_char,
                ),
                quote_n(
                    1 as libc::c_int,
                    b"--reference\0" as *const u8 as *const libc::c_char,
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
                        b"you must specify either %s or %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote_n(
                        0 as libc::c_int,
                        b"--size\0" as *const u8 as *const libc::c_char,
                    ),
                    quote_n(
                        1 as libc::c_int,
                        b"--reference\0" as *const u8 as *const libc::c_char,
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
                        b"you must specify either %s or %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote_n(
                        0 as libc::c_int,
                        b"--size\0" as *const u8 as *const libc::c_char,
                    ),
                    quote_n(
                        1 as libc::c_int,
                        b"--reference\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        usage(1);
    }
    if !ref_file.is_null() && got_size as libc::c_int != 0 && rel_mode as u64 == 0 {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"you must specify a relative %s with %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quote_n(
                    0 as libc::c_int,
                    b"--size\0" as *const u8 as *const libc::c_char,
                ),
                quote_n(
                    1 as libc::c_int,
                    b"--reference\0" as *const u8 as *const libc::c_char,
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
                        b"you must specify a relative %s with %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote_n(
                        0 as libc::c_int,
                        b"--size\0" as *const u8 as *const libc::c_char,
                    ),
                    quote_n(
                        1 as libc::c_int,
                        b"--reference\0" as *const u8 as *const libc::c_char,
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
                        b"you must specify a relative %s with %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote_n(
                        0 as libc::c_int,
                        b"--size\0" as *const u8 as *const libc::c_char,
                    ),
                    quote_n(
                        1 as libc::c_int,
                        b"--reference\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        usage(1);
    }
    if block_mode as libc::c_int != 0 && !got_size {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s was specified but %s was not\0" as *const u8
                        as *const libc::c_char,
                ),
                quote_n(
                    0 as libc::c_int,
                    b"--io-blocks\0" as *const u8 as *const libc::c_char,
                ),
                quote_n(
                    1 as libc::c_int,
                    b"--size\0" as *const u8 as *const libc::c_char,
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
                        b"%s was specified but %s was not\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote_n(
                        0 as libc::c_int,
                        b"--io-blocks\0" as *const u8 as *const libc::c_char,
                    ),
                    quote_n(
                        1 as libc::c_int,
                        b"--size\0" as *const u8 as *const libc::c_char,
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
                        b"%s was specified but %s was not\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote_n(
                        0 as libc::c_int,
                        b"--io-blocks\0" as *const u8 as *const libc::c_char,
                    ),
                    quote_n(
                        1 as libc::c_int,
                        b"--size\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        usage(1);
    }
    if argc < 1 {
    let errstatus = 0;
    error(errstatus, 0, gettext(b"missing file operand\0".as_ptr() as *const libc::c_char));
    if errstatus != 0 {
        unreachable!();
    }
    usage(1);
}

    if !ref_file.is_null() {
    let ref_file_str = unsafe { std::ffi::CStr::from_ptr(ref_file) };
    let ref_file_path = ref_file_str.to_str().unwrap();

    let sb = std::fs::metadata(ref_file_path).map_err(|e| {
        error(
            1,
            e.raw_os_error().unwrap_or(0),
            gettext(b"cannot stat %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, ref_file),
        );
    });

    let mut file_size: i64 = -1;
    if let Ok(metadata) = sb {
        file_size = metadata.len() as i64;
    } else {
        let ref_fd = open(ref_file, 0);
        if ref_fd >= 0 {
            let file_end = unsafe { lseek(ref_fd, 0, 2) };
            let saved_errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
            unsafe { close(ref_fd) };
            if file_end >= 0 {
                file_size = file_end;
            } else {
                // Note: We cannot set errno directly in safe Rust.
                // This is a workaround to handle the error appropriately.
                std::process::exit(saved_errno);
            }
        }
    }

    if file_size < 0 {
        error(
            1,
            std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
            gettext(b"cannot get the size of %s\0" as *const u8 as *const libc::c_char),
            quotearg_style(shell_escape_always_quoting_style, ref_file),
        );
    }

    if !got_size {
        size = file_size;
    } else {
        rsize = file_size;
    }
}

let oflags = 0o1 | if no_create { 0 } else { 0o100 } | 0o4000;
let mut errors = false;
let mut fname: *const libc::c_char = std::ptr::null();

    loop {
    let fname = match unsafe { *argv } {
        ptr if ptr.is_null() => break,
        ptr => ptr,
    };

    let fd = open(fname, oflags, 0o400 | 0o200 | 0o400 >> 3 | 0o200 >> 3 | 0o400 >> 6 | 0o200 >> 6);
    
    if fd < 0 {
        if !(no_create && unsafe { *__errno_location() } == 2) {
            let error_message = gettext(b"cannot open %s for writing\0" as *const u8 as *const libc::c_char);
            error(0, unsafe { *__errno_location() }, error_message, quotearg_style(shell_escape_always_quoting_style, fname));
            errors = true;
        }
    } else {
        errors |= !do_ftruncate(fd, fname, size, rsize, rel_mode);
        if close(fd) != 0 {
            let error_message = gettext(b"failed to close %s\0" as *const u8 as *const libc::c_char);
            error(0, unsafe { *__errno_location() }, error_message, quotearg_style(shell_escape_always_quoting_style, fname));
            errors = true;
        }
    }
    argv = argv.add(1);
}
return if errors { 1 } else { 0 };

}
pub fn main() {
    let args: Vec<CString> = ::std::env::args()
        .map(|arg| CString::new(arg).expect("Failed to convert argument into CString."))
        .collect();
    
    let argc = args.len() as libc::c_int;
    let argv: Vec<*mut libc::c_char> = args.iter()
        .map(|cstr| cstr.as_ptr() as *mut libc::c_char)
        .chain(std::iter::once(std::ptr::null_mut()))
        .collect();
    
    let status = unsafe { main_0(argc, argv.as_ptr() as *mut *mut libc::c_char) };
    ::std::process::exit(status as i32);
}

