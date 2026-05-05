#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types)]












use std::os::raw::c_char;

use std::ffi::CString;
use std::mem;

use std::ffi::CStr;

use std::io::Write;

use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut optarg: *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn fpurge(gl_stream: *mut FILE) -> libc::c_int;
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
    fn initbuffer(linebuffer: *mut linebuffer);
    fn readlinebuffer_delim(
        linebuffer: *mut linebuffer,
        stream: *mut FILE,
        delimiter: libc::c_char,
    ) -> *mut linebuffer;
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn iswblank(__wc: wint_t) -> libc::c_int;
    fn mbrtoc32(
        __pc32: *mut char32_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn posix2_version() -> libc::c_int;
    fn freopen_safer(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut FILE,
    ) -> *mut FILE;
    fn xstrtoimax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut intmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn memcasecmp(
        vs1: *const libc::c_void,
        vs2: *const libc::c_void,
        n: size_t,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __uint32_t = libc::c_uint;
pub type __uint_least32_t = __uint32_t;
pub type __intmax_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type intmax_t = __intmax_t;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
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
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linebuffer {
    pub size: idx_t,
    pub length: idx_t,
    pub buffer: *mut libc::c_char,
}
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
pub type mbstate_t = __mbstate_t;
pub type char32_t = __uint_least32_t;
pub type wint_t = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MCEL_LEN_MAX: C2RustUnnamed_1 = 4;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MCEL_CHAR_MAX: C2RustUnnamed_2 = 1114111;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const MCEL_ERR_MIN: C2RustUnnamed_3 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcel_t {
    pub ch: char32_t,
    pub err: libc::c_uchar,
    pub len: libc::c_uchar,
}
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type delimit_method = libc::c_uint;
pub const DM_SEPARATE: delimit_method = 2;
pub const DM_PREPEND: delimit_method = 1;
pub const DM_NONE: delimit_method = 0;
pub type grouping_method = libc::c_uint;
pub const GM_BOTH: grouping_method = 4;
pub const GM_SEPARATE: grouping_method = 3;
pub const GM_APPEND: grouping_method = 2;
pub const GM_PREPEND: grouping_method = 1;
pub const GM_NONE: grouping_method = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const GROUP_OPTION: C2RustUnnamed_4 = 256;
pub type Skip_field_option_type = libc::c_uint;
pub const SFO_NEW: Skip_field_option_type = 2;
pub const SFO_OBSOLETE: Skip_field_option_type = 1;
pub const SFO_NONE: Skip_field_option_type = 0;
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

    let help_message = unsafe { CStr::from_ptr(gettext(b"GNU coreutils\0".as_ptr() as *const i8)).to_string_lossy() };
    let help_url = "https://www.gnu.org/software/coreutils/";
    println!("{} online help: <{}>", help_message, help_url);

    let lc_messages = unsafe { setlocale(5, std::ptr::null()) };
    if !lc_messages.is_null() {
        let lc_str = unsafe { CStr::from_ptr(lc_messages).to_str().unwrap_or("") };
        if !lc_str.starts_with("en_") {
            let report_message = unsafe { CStr::from_ptr(gettext(b"Report any translation bugs to <https://translationproject.org/team/>\0".as_ptr() as *const i8)).to_string_lossy() };
            writeln!(std::io::stdout(), "{}", report_message).unwrap();
        }
    }

    let url_program = if program == "[" { "test" } else { program };

    println!(
        "Full documentation <{}{}>",
        help_url,
        url_program
    );

    println!(
        "or available locally via: info '(coreutils) {}{}'",
        node,
        if node == program { " invocation" } else { "" }
    );
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
            let errstatus = 1;
            unsafe {
                error(
                    errstatus,
                    saved_errno.raw_os_error().unwrap_or(0),
                    gettext(b"write error\0".as_ptr() as *const libc::c_char),
                );
            }
            if errstatus != 0 {
                unreachable!();
            }
        }
        {
            let errstatus = 1;
            unsafe {
                error(
                    errstatus,
                    saved_errno.raw_os_error().unwrap_or(0),
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
fn c32isblank(wc: char) -> bool {
    wc.is_whitespace() && !wc.is_ascii_graphic()
}

#[inline]
fn mcel_ch(ch: char32_t, len: size_t) -> mcel_t {
    assert!(len > 0, "Length must be greater than 0");
    assert!(len <= MCEL_LEN_MAX as libc::c_int as libc::c_ulong, "Length exceeds maximum allowed");
    assert!(ch <= MCEL_CHAR_MAX as libc::c_int as libc::c_uint, "Character exceeds maximum allowed");

    mcel_t {
        ch,
        err: 0,
        len: len as libc::c_uchar,
    }
}

#[inline]
fn mcel_err(err: u8) -> mcel_t {
    assert!(MCEL_ERR_MIN as i32 <= err as i32, "Error value out of range");
    
    mcel_t {
        ch: 0,
        err,
        len: 1u8,
    }
}

#[inline]
fn mcel_isbasic(c: i8) -> bool {
    let c_int = c as i32;
    c_int >= 0 && c_int < MCEL_ERR_MIN as i32
}

#[inline]
fn mcel_scan(p: *const libc::c_char, lim: *const libc::c_char) -> mcel_t {
    let c = unsafe { *p };
    if mcel_isbasic(c) {
        return mcel_ch(c as char32_t, 1 as libc::c_int as size_t);
    }
    
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    
    mbs.__count = 0;
    let mut ch: char32_t = 0;
    let mut len: size_t = unsafe {
        mbrtoc32(
            &mut ch,
            p,
            lim.offset_from(p) as libc::c_long as size_t,
            &mut mbs,
        )
    };
    
    if ((-(1 as libc::c_int) as size_t).wrapping_div(2 as libc::c_int as libc::c_ulong) < len) as libc::c_int as libc::c_long != 0 {
        return mcel_err(c as libc::c_uchar);
    }
    
    return mcel_ch(ch, len);
}

#[inline]
unsafe extern "C" fn skip_buf_matching(
    mut buf: *const libc::c_char,
    mut lim: *const libc::c_char,
    mut predicate: Option::<unsafe extern "C" fn(mcel_t) -> bool>,
    mut ok: bool,
) -> *mut libc::c_char {
    let mut s: *const libc::c_char = buf;
    let mut g: mcel_t = mcel_t { ch: 0, err: 0, len: 0 };
    while s < lim
        && {
            g = mcel_scan(s, lim);
            predicate.expect("non-null function pointer")(g) as libc::c_int
                == ok as libc::c_int
        }
    {
        s = s.offset(g.len as libc::c_int as isize);
    }
    return s as *mut libc::c_char;
}
fn swap_lines(a: &mut *mut linebuffer, b: &mut *mut linebuffer) {
    std::mem::swap(a, b);
}

static mut skip_fields: idx_t = 0 as libc::c_int as idx_t;
static mut skip_chars: idx_t = 0 as libc::c_int as idx_t;
static mut check_chars: idx_t = 9223372036854775807 as libc::c_long;
static mut count_occurrences: bool = 0 as libc::c_int != 0;
static mut output_unique: bool = 1 as libc::c_int != 0;
static mut output_first_repeated: bool = 1 as libc::c_int != 0;
static mut output_later_repeated: bool = 0 as libc::c_int != 0;
static mut ignore_case: bool = false;
static mut delimit_method_string: [*const libc::c_char; 4] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"prepend\0" as *const u8 as *const libc::c_char,
    b"separate\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut delimit_method_map: [delimit_method; 3] = [DM_NONE, DM_PREPEND, DM_SEPARATE];
static mut delimit_groups: delimit_method = DM_NONE;
static mut grouping_method_string: [*const libc::c_char; 5] = [
    b"prepend\0" as *const u8 as *const libc::c_char,
    b"append\0" as *const u8 as *const libc::c_char,
    b"separate\0" as *const u8 as *const libc::c_char,
    b"both\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut grouping_method_map: [grouping_method; 4] = [
    GM_PREPEND,
    GM_APPEND,
    GM_SEPARATE,
    GM_BOTH,
];
static mut grouping: grouping_method = GM_NONE;
static mut longopts: [option; 13] = [
    {
        let mut init = option {
            name: b"count\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"repeated\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"all-repeated\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"group\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GROUP_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-case\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"unique\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"skip-fields\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"skip-chars\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"check-chars\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
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
                b"Usage: %s [OPTION]... [INPUT [OUTPUT]]\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Filter adjacent matching lines from INPUT (or standard input),\nwriting to OUTPUT (or standard output).\n\nWith no options, matching lines are merged to the first occurrence.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -c, --count           prefix lines by the number of occurrences\n  -d, --repeated        only print duplicate lines, one for each group\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -D                    print all duplicate lines\n      --all-repeated[=METHOD]  like -D, but allow separating groups\n                                 with an empty line;\n                                 METHOD={none(default),prepend,separate}\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -f, --skip-fields=N   avoid comparing the first N fields\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --group[=METHOD]  show all items, separating groups with an empty line;\n                          METHOD={separate(default),prepend,append,both}\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -i, --ignore-case     ignore differences in case when comparing\n  -s, --skip-chars=N    avoid comparing the first N characters\n  -u, --unique          only print unique lines\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -z, --zero-terminated     line delimiter is NUL, not newline\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -w, --check-chars=N   compare no more than N characters in lines\n\0"
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
        fputs_unlocked(
            gettext(
                b"\nA field is a run of blanks (usually spaces and/or TABs), then non-blank\ncharacters.  Fields are skipped before chars.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"\n'uniq' does not detect repeated lines unless they are adjacent.\nYou may want to sort the input first, or use 'sort -u' without 'uniq'.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info("uniq");
    }
    exit(status);
}
fn strict_posix2() -> bool {
    unsafe {
        let posix_ver = posix2_version();
        200112 <= posix_ver && posix_ver < 200809
    }
}

fn size_opt(opt: &CStr, msgid: &CStr) -> idx_t {
    let mut size: i64 = 0;
    let opt_str = opt.to_str().unwrap(); // Convert CStr to &str

    if let Err(_) = opt_str.parse::<i64>().map(|s| {
        if s < 0 {
            return Err(());
        }
        size = s;
        Ok(())
    }) {
        let error_msg = format!("{}: {}", opt_str, msgid.to_string_lossy());
        // Use a safe block to call the unsafe function
        unsafe {
            error(1, 0, error_msg.as_ptr() as *const i8); // Pass raw pointer to error function
        }
    }

    if size < 9223372036854775807 {
        size
    } else {
        9223372036854775807
    }
}

unsafe extern "C" fn newline_or_blank(mut g: mcel_t) -> bool {
    let is_newline = g.ch == '\n' as u32;
let is_blank = match char::from_u32(g.ch) {
    Some(c) => c32isblank(c),
    None => false,
};
return is_newline || is_blank;
}
unsafe extern "C" fn find_field(
    mut line: *const linebuffer,
    mut plen: *mut idx_t,
) -> *mut libc::c_char {
    let mut lp: *mut libc::c_char = (*line).buffer;
    let mut lim: *const libc::c_char = lp
        .offset((*line).length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut i: idx_t = skip_fields;
    while (0 as libc::c_int as libc::c_long) < i && lp < lim as *mut libc::c_char {
        lp = skip_buf_matching(
            lp,
            lim,
            Some(newline_or_blank as unsafe extern "C" fn(mcel_t) -> bool),
            1 as libc::c_int != 0,
        );
        lp = skip_buf_matching(
            lp,
            lim,
            Some(newline_or_blank as unsafe extern "C" fn(mcel_t) -> bool),
            0 as libc::c_int != 0,
        );
        i -= 1;
        i;
    }
    let mut i_0: idx_t = skip_chars;
    while (0 as libc::c_int as libc::c_long) < i_0 && lp < lim as *mut libc::c_char {
        lp = lp.offset((mcel_scan(lp, lim)).len as libc::c_int as isize);
        i_0 -= 1;
        i_0;
    }
    let mut len: idx_t = 0;
    if lim.offset_from(lp) as libc::c_long <= check_chars {
        len = lim.offset_from(lp) as libc::c_long;
    } else if __ctype_get_mb_cur_max() <= 1 as libc::c_int as libc::c_ulong {
        len = check_chars;
    } else {
        let mut ep: *mut libc::c_char = lp;
        let mut i_1: idx_t = check_chars;
        while (0 as libc::c_int as libc::c_long) < i_1 && lp < lim as *mut libc::c_char {
            ep = ep.offset((mcel_scan(lp, lim)).len as libc::c_int as isize);
            i_1 -= 1;
            i_1;
        }
        len = ep.offset_from(lp) as libc::c_long;
    }
    *plen = len;
    return lp;
}
unsafe extern "C" fn different(
    mut old: *mut libc::c_char,
    mut new: *mut libc::c_char,
    mut oldlen: idx_t,
    mut newlen: idx_t,
) -> bool {
    if ignore_case {
        return oldlen != newlen
            || memcasecmp(
                old as *const libc::c_void,
                new as *const libc::c_void,
                oldlen as size_t,
            ) != 0
    } else {
        return oldlen != newlen
            || memcmp(
                old as *const libc::c_void,
                new as *const libc::c_void,
                oldlen as libc::c_ulong,
            ) != 0
    };
}
unsafe extern "C" fn writeline(
    mut line: *const linebuffer,
    mut match_0: bool,
    mut linecount: intmax_t,
) {
    if if linecount == 0 as libc::c_int as libc::c_long {
        output_unique as libc::c_int
    } else if !match_0 {
        output_first_repeated as libc::c_int
    } else {
        output_later_repeated as libc::c_int
    } == 0
    {
        return;
    }
    if count_occurrences {
        printf(
            b"%7jd \0" as *const u8 as *const libc::c_char,
            linecount + 1 as libc::c_int as libc::c_long,
        );
    }
    if fwrite_unlocked(
        (*line).buffer as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        (*line).length as size_t,
        stdout,
    ) != (*line).length as libc::c_ulong
    {
        write_error();
    }
}
unsafe extern "C" fn check_file(
    mut infile: *const libc::c_char,
    mut outfile: *const libc::c_char,
    mut delimiter: libc::c_char,
) {
    let mut current_block: u64;
    let mut lb1: linebuffer = linebuffer {
        size: 0,
        length: 0,
        buffer: 0 as *mut libc::c_char,
    };
    let mut lb2: linebuffer = linebuffer {
        size: 0,
        length: 0,
        buffer: 0 as *mut libc::c_char,
    };
    let mut thisline: *mut linebuffer = 0 as *mut linebuffer;
    let mut prevline: *mut linebuffer = 0 as *mut linebuffer;
    if !(strcmp(infile, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || !(freopen_safer(infile, b"r\0" as *const u8 as *const libc::c_char, stdin))
            .is_null())
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
    if !(strcmp(outfile, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || !(freopen_safer(outfile, b"w\0" as *const u8 as *const libc::c_char, stdout))
            .is_null())
    {
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
    fadvise(stdin, FADVISE_SEQUENTIAL);
    thisline = &mut lb1;
    prevline = &mut lb2;
    initbuffer(thisline);
    initbuffer(prevline);
    if output_unique as libc::c_int != 0 && output_first_repeated as libc::c_int != 0
        && !count_occurrences
    {
        let mut prevfield: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut prevlen: idx_t = 0;
        let mut first_group_printed: bool = 0 as libc::c_int != 0;
        while feof_unlocked(stdin) == 0
            && !(readlinebuffer_delim(thisline, stdin, delimiter)).is_null()
        {
            let mut thislen: idx_t = 0;
            let mut thisfield: *mut libc::c_char = find_field(thisline, &mut thislen);
            let mut new_group: bool = prevfield.is_null()
                || different(thisfield, prevfield, thislen, prevlen) as libc::c_int != 0;
            if new_group as libc::c_int != 0
                && grouping as libc::c_uint != GM_NONE as libc::c_int as libc::c_uint
                && (grouping as libc::c_uint == GM_PREPEND as libc::c_int as libc::c_uint
                    || grouping as libc::c_uint == GM_BOTH as libc::c_int as libc::c_uint
                    || first_group_printed as libc::c_int != 0
                        && (grouping as libc::c_uint
                            == GM_APPEND as libc::c_int as libc::c_uint
                            || grouping as libc::c_uint
                                == GM_SEPARATE as libc::c_int as libc::c_uint))
            {
                putchar_unlocked(delimiter as libc::c_int);
            }
            if new_group as libc::c_int != 0
                || grouping as libc::c_uint != GM_NONE as libc::c_int as libc::c_uint
            {
                if fwrite_unlocked(
                    (*thisline).buffer as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    (*thisline).length as size_t,
                    stdout,
                ) != (*thisline).length as libc::c_ulong
                {
                    write_error();
                }
                swap_lines(&mut prevline, &mut thisline);
                prevfield = thisfield;
                prevlen = thislen;
                first_group_printed = 1 as libc::c_int != 0;
            }
        }
        if (grouping as libc::c_uint == GM_BOTH as libc::c_int as libc::c_uint
            || grouping as libc::c_uint == GM_APPEND as libc::c_int as libc::c_uint)
            && first_group_printed as libc::c_int != 0
        {
            putchar_unlocked(delimiter as libc::c_int);
        }
    } else if !(readlinebuffer_delim(prevline, stdin, delimiter)).is_null() {
        let mut prevlen_0: idx_t = 0;
        let mut prevfield_0: *mut libc::c_char = find_field(prevline, &mut prevlen_0);
        let mut match_count: intmax_t = 0 as libc::c_int as intmax_t;
        let mut first_delimiter: bool = 1 as libc::c_int != 0;
        loop {
            if !(feof_unlocked(stdin) == 0) {
                current_block = 6476622998065200121;
                break;
            }
            if (readlinebuffer_delim(thisline, stdin, delimiter)).is_null() {
                if ferror_unlocked(stdin) != 0 {
                    current_block = 6469497169255231809;
                    break;
                } else {
                    current_block = 6476622998065200121;
                    break;
                }
            } else {
                let mut thislen_0: idx_t = 0;
                let mut thisfield_0: *mut libc::c_char = find_field(
                    thisline,
                    &mut thislen_0,
                );
                let mut match_0: bool = !different(
                    thisfield_0,
                    prevfield_0,
                    thislen_0,
                    prevlen_0,
                );
                match_count += match_0 as libc::c_long;
                if match_count == 9223372036854775807 as libc::c_long {
                    if count_occurrences {
                        if 0 != 0 {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"too many repeated lines\0" as *const u8
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
                                        b"too many repeated lines\0" as *const u8
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
                                        b"too many repeated lines\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                    }
                    match_count -= 1;
                    match_count;
                }
                if delimit_groups as libc::c_uint
                    != DM_NONE as libc::c_int as libc::c_uint
                {
                    if !match_0 {
                        if match_count != 0 {
                            first_delimiter = 0 as libc::c_int != 0;
                        }
                    } else if match_count == 1 as libc::c_int as libc::c_long {
                        if delimit_groups as libc::c_uint
                            == DM_PREPEND as libc::c_int as libc::c_uint
                            || delimit_groups as libc::c_uint
                                == DM_SEPARATE as libc::c_int as libc::c_uint
                                && !first_delimiter
                        {
                            putchar_unlocked(delimiter as libc::c_int);
                        }
                    }
                }
                if !match_0 || output_later_repeated as libc::c_int != 0 {
                    writeline(prevline, match_0, match_count);
                    swap_lines(&mut prevline, &mut thisline);
                    prevfield_0 = thisfield_0;
                    prevlen_0 = thislen_0;
                    if !match_0 {
                        match_count = 0 as libc::c_int as intmax_t;
                    }
                }
            }
        }
        match current_block {
            6469497169255231809 => {}
            _ => {
                writeline(prevline, 0 as libc::c_int != 0, match_count);
            }
        }
    }
    if ferror_unlocked(stdin) != 0 || rpl_fclose(stdin) != 0 {
    let errstatus = 1;
    error(
        errstatus,
        *__errno_location(),
        gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
        quotearg_style(shell_escape_always_quoting_style, infile),
    );
    unreachable!();
}

drop(lb1);
drop(lb2);

}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0 as libc::c_int;
    let mut posixly_correct: bool = !(getenv(
        b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
    ))
        .is_null();
    let mut skip_field_option_type: Skip_field_option_type = SFO_NONE;
    let mut nfiles: libc::c_int = 0 as libc::c_int;
    let mut file: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut delimiter: libc::c_char = '\n' as i32 as libc::c_char;
    let mut output_option_used: bool = 0 as libc::c_int != 0;
    file[1 as libc::c_int as usize] = b"-\0" as *const u8 as *const libc::c_char;
    file[0 as libc::c_int as usize] = file[1 as libc::c_int as usize];
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        if optc == -(1 as libc::c_int)
            || posixly_correct as libc::c_int != 0 && nfiles != 0 as libc::c_int
            || {
                optc = getopt_long(
                    argc,
                    argv,
                    b"-0123456789Dcdf:is:uw:z\0" as *const u8 as *const libc::c_char,
                    longopts.as_ptr(),
                    0 as *mut libc::c_int,
                );
                optc == -(1 as libc::c_int)
            }
        {
            if argc <= optind {
    break;
}
if nfiles == 2 {
    let extra_operand = quote(unsafe { *argv.offset(optind as isize) });
    error(0, 0, gettext(b"extra operand %s\0" as *const u8 as *const libc::c_char), extra_operand);
    usage(1);
}
let fresh0 = optind;
optind += 1;
let fresh1 = nfiles;
nfiles += 1;
file[fresh1 as usize] = unsafe { *argv.offset(fresh0 as isize) };

        } else {
            match optc {
    1 => {
         let mut size: intmax_t = 0;
                    if *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                        == '+' as i32 && !strict_posix2()
                        && xstrtoimax(
                            optarg,
                            0 as *mut *mut libc::c_char,
                            10 as libc::c_int,
                            &mut size,
                            b"\0" as *const u8 as *const libc::c_char,
                        ) as libc::c_uint
                            <= LONGINT_OVERFLOW as libc::c_int as libc::c_uint
                    {
                        skip_chars = if size < 9223372036854775807 as libc::c_long {
                            size
                        } else {
                            9223372036854775807 as libc::c_long
                        };
                    } else if nfiles == 2 as libc::c_int {
                        if 0 != 0 {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"extra operand %s\0" as *const u8 as *const libc::c_char,
                                ),
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
                                    gettext(
                                        b"extra operand %s\0" as *const u8 as *const libc::c_char,
                                    ),
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
                                    gettext(
                                        b"extra operand %s\0" as *const u8 as *const libc::c_char,
                                    ),
                                    quote(optarg),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                        usage(1 as libc::c_int);
                    } else {
                        let fresh2 = nfiles;
                        nfiles = nfiles + 1;
                        file[fresh2 as usize] = optarg;
                    }

    }
    48..=57 => {
         if skip_field_option_type as libc::c_uint
                        == SFO_NEW as libc::c_int as libc::c_uint
                    {
                        skip_fields = 0 as libc::c_int as idx_t;
                    }
                    if !(!((if (0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t
                        && ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            skip_fields
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        && ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            10 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        && (if (10 as libc::c_int) < 0 as libc::c_int {
                            if skip_fields < 0 as libc::c_int as libc::c_long {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        -(1 as libc::c_int) as idx_t
                                    }) + 10 as libc::c_int as libc::c_long
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (skip_fields
                                        < -(1 as libc::c_int) as idx_t
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
                                        }) as libc::c_long + -(1 as libc::c_int) as idx_t
                                            >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        -(1 as libc::c_int) as idx_t
                                            / -(10 as libc::c_int) as libc::c_long
                                    }) <= -(1 as libc::c_int) as libc::c_long - skip_fields)
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
                                    }) as libc::c_long + 0 as libc::c_int as idx_t
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
                                        }) as libc::c_long + 0 as libc::c_int as idx_t
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
                                        }) as libc::c_long + 0 as libc::c_int as idx_t
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) as libc::c_long + 0 as libc::c_int as idx_t)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) as libc::c_long + 0 as libc::c_int as idx_t
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
                                                }) as libc::c_long + 0 as libc::c_int as idx_t
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
                                                }) as libc::c_long + 0 as libc::c_int as idx_t
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) as libc::c_long + 0 as libc::c_int as idx_t)
                                        as libc::c_int
                                }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        skip_fields
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < skip_fields + 0 as libc::c_int as idx_t) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long) < skip_fields
                                            && (-(1 as libc::c_int) as libc::c_long
                                                - 0 as libc::c_int as idx_t)
                                                < skip_fields - 1 as libc::c_int as libc::c_long)
                                            as libc::c_int
                                    }
                                } else {
                                    ((0 as libc::c_int as idx_t
                                        / 10 as libc::c_int as libc::c_long) < skip_fields)
                                        as libc::c_int
                                }
                            }
                        } else {
                            if 10 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if skip_fields < 0 as libc::c_int as libc::c_long {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            skip_fields
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
                                                skip_fields
                                            }) + 0 as libc::c_int as idx_t
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
                                                skip_fields
                                            }) + 0 as libc::c_int as idx_t
                                        }) + 0 as libc::c_int as libc::c_long
                                    }) < 0 as libc::c_int as libc::c_long
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            skip_fields
                                        }) + 0 as libc::c_int as idx_t)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    skip_fields
                                                }) + 0 as libc::c_int as idx_t
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        skip_fields
                                                    }) + 0 as libc::c_int as idx_t
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
                                                        skip_fields
                                                    }) + 0 as libc::c_int as idx_t
                                                }) - 1 as libc::c_int as libc::c_long
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int as libc::c_long)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                skip_fields
                                            }) + 0 as libc::c_int as idx_t) as libc::c_int
                                    }) != 0
                                        && skip_fields == -(1 as libc::c_int) as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int as libc::c_long)
                                                < 10 as libc::c_int as libc::c_long
                                                    + 0 as libc::c_int as idx_t) as libc::c_int
                                        } else {
                                            ((-(1 as libc::c_int) as libc::c_long
                                                - 0 as libc::c_int as idx_t)
                                                < (10 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                                                as libc::c_int
                                        }
                                    } else {
                                        (0 as libc::c_int as idx_t / skip_fields
                                            < 10 as libc::c_int as libc::c_long) as libc::c_int
                                    }
                                } else {
                                    ((-(1 as libc::c_int) as idx_t
                                        / 10 as libc::c_int as libc::c_long) < skip_fields)
                                        as libc::c_int
                                }
                            }
                        }) != 0
                    {
                        let (fresh7, _fresh8) = skip_fields
                            .overflowing_mul((10 as libc::c_int).into());
                        *(&mut skip_fields as *mut idx_t) = fresh7;
                        1 as libc::c_int
                    } else {
                        let (fresh9, fresh10) = skip_fields
                            .overflowing_mul((10 as libc::c_int).into());
                        *(&mut skip_fields as *mut idx_t) = fresh9;
                        fresh10 as libc::c_int
                    }) != 0)
                        && {
                            let (fresh11, fresh12) = skip_fields
                                .overflowing_add((optc - '0' as i32).into());
                            *(&mut skip_fields as *mut idx_t) = fresh11;
                            !fresh12
                        })
                    {
                        skip_fields = 9223372036854775807 as libc::c_long;
                    }
                    skip_field_option_type = SFO_OBSOLETE;

    }
    99 => {
        count_occurrences = true;
        output_option_used = true;
    }
    100 => {
        output_unique = false;
        output_option_used = true;
    }
    68 => {
         output_unique = false;
output_later_repeated = true;

if optarg.is_null() {
    delimit_groups = DM_NONE;
} else {
    let optarg_str = unsafe { std::ffi::CStr::from_ptr(optarg) };
    delimit_groups = delimit_method_map[__xargmatch_internal(
        std::ffi::CString::new("--all-repeated").unwrap().as_ptr(),
        optarg_str.as_ptr(),
        delimit_method_string.as_ptr(),
        delimit_method_map.as_ptr() as *const libc::c_void,
        std::mem::size_of::<delimit_method>() as u64,
        argmatch_die,
        true,
    ) as usize];
}

output_option_used = true;


    }
    256 => {
         if optarg.is_null() {
    grouping = GM_SEPARATE;
} else {
    let optarg_str = unsafe { std::ffi::CStr::from_ptr(optarg) };
    grouping = grouping_method_map[__xargmatch_internal(
        std::ffi::CString::new("--group").unwrap().as_ptr(),
        optarg_str.as_ptr(),
        grouping_method_string.as_ptr(),
        grouping_method_map.as_ptr() as *const libc::c_void,
        std::mem::size_of::<grouping_method>() as u64,
        argmatch_die,
        true,
    ) as usize];
}


    }
    102 => {
        skip_field_option_type = SFO_NEW;
        skip_fields = size_opt(
            unsafe { CStr::from_ptr(optarg) },
            unsafe { CStr::from_bytes_with_nul_unchecked(b"invalid number of fields to skip\0") },
        );
    }
    105 => {
        ignore_case = true;
    }
    115 => {
        skip_chars = size_opt(
            unsafe { CStr::from_ptr(optarg) },
            unsafe { CStr::from_bytes_with_nul_unchecked(b"invalid number of bytes to skip\0") },
        );
    }
    117 => {
        output_first_repeated = false;
        output_option_used = true;
    }
    119 => {
        check_chars = size_opt(
            unsafe { CStr::from_ptr(optarg) },
            unsafe { CStr::from_bytes_with_nul_unchecked(b"invalid number of bytes to compare\0") },
        );
    }
    122 => {
        delimiter = b'\0' as i8;
    }
    -2 => {
        usage(0);
    }
    -3 => {
         version_etc(
                        stdout,
                        b"uniq\0" as *const u8 as *const libc::c_char,
                        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                        Version,
                        proper_name_lite(
                            b"Richard M. Stallman\0" as *const u8 as *const libc::c_char,
                            b"Richard M. Stallman\0" as *const u8 as *const libc::c_char,
                        ),
                        proper_name_lite(
                            b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                            b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                        ),
                        0 as *mut libc::c_void as *mut libc::c_char,
                    );
                    exit(0 as libc::c_int);

    }
    _ => {
        usage(1);
    }
}
/*
The variables live at this point are:
(mut optc: i32, mut skip_field_option_type: u32, mut nfiles: i32, mut file: [*const i8; 2], mut delimiter: i8, mut output_option_used: bool)
*/

        }
    }
    if grouping as libc::c_uint != GM_NONE as libc::c_int as libc::c_uint
        && output_option_used as libc::c_int != 0
    {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"--group is mutually exclusive with -c/-d/-D/-u\0" as *const u8
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
                        b"--group is mutually exclusive with -c/-d/-D/-u\0" as *const u8
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
                        b"--group is mutually exclusive with -c/-d/-D/-u\0" as *const u8
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
    if grouping as libc::c_uint != GM_NONE as libc::c_int as libc::c_uint
        && count_occurrences as libc::c_int != 0
    {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"grouping and printing repeat counts is meaningless\0" as *const u8
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
                        b"grouping and printing repeat counts is meaningless\0"
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
                        b"grouping and printing repeat counts is meaningless\0"
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
    if count_occurrences as libc::c_int != 0 && output_later_repeated as libc::c_int != 0
    {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"printing all duplicated lines and repeat counts is meaningless\0"
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
                        b"printing all duplicated lines and repeat counts is meaningless\0"
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
                        b"printing all duplicated lines and repeat counts is meaningless\0"
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
    check_file(
        file[0 as libc::c_int as usize],
        file[1 as libc::c_int as usize],
        delimiter,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
