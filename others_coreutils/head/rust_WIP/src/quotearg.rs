

























use std::os::raw::c_int;

use std::vec::Vec;

use std::option::Option;

use std::clone::Clone;

use std::mem;

use std::io;

use std::ffi::CStr;

use std::ffi::CString;
use std::os::raw::c_char;
use std::convert::TryInto;

use ::libc;
extern "C" {
    fn __ctype_get_mb_cur_max() -> size_t;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn xmemdup(p: *const libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xcharalloc(n: size_t) -> *mut libc::c_char;
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn locale_charset() -> *const libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_mbrtoc32(
        pc: *mut char32_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn iswprint(__wc: wint_t) -> libc::c_int;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint_least32_t = __uint32_t;
pub type ptrdiff_t = libc::c_long;
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
pub type quoting_flags = libc::c_uint;
pub const QA_SPLIT_TRIGRAPHS: quoting_flags = 4;
pub const QA_ELIDE_OUTER_QUOTES: quoting_flags = 2;
pub const QA_ELIDE_NULL_BYTES: quoting_flags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quoting_options {
    pub style: quoting_style,
    pub flags: libc::c_int,
    pub quote_these_too: [libc::c_uint; 8],
    pub left_quote: *const libc::c_char,
    pub right_quote: *const libc::c_char,
}
pub type __mbstate_t = mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type char32_t = __uint_least32_t;
pub type wint_t = libc::c_uint;
pub const _ISprint: C2RustUnnamed_0 = 16384;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slotvec {
    pub size: size_t,
    pub val: *mut libc::c_char,
}
pub type idx_t = ptrdiff_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
#[inline]
fn c32isprint(wc: wint_t) -> libc::c_int {
    unsafe { iswprint(wc) }
}

#[inline]
fn mbszero(ps: &mut mbstate_t) {
    *ps = mbstate_t {
        __count: 0,
        __value: unsafe { std::mem::zeroed() }, // Using zeroed to initialize __value
    };
}

#[no_mangle]
pub static mut quoting_style_args: [*const libc::c_char; 11] = [
    b"literal\0" as *const u8 as *const libc::c_char,
    b"shell\0" as *const u8 as *const libc::c_char,
    b"shell-always\0" as *const u8 as *const libc::c_char,
    b"shell-escape\0" as *const u8 as *const libc::c_char,
    b"shell-escape-always\0" as *const u8 as *const libc::c_char,
    b"c\0" as *const u8 as *const libc::c_char,
    b"c-maybe\0" as *const u8 as *const libc::c_char,
    b"escape\0" as *const u8 as *const libc::c_char,
    b"locale\0" as *const u8 as *const libc::c_char,
    b"clocale\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut quoting_style_vals: [quoting_style; 10] = [
    literal_quoting_style,
    shell_quoting_style,
    shell_always_quoting_style,
    shell_escape_quoting_style,
    shell_escape_always_quoting_style,
    c_quoting_style,
    c_maybe_quoting_style,
    escape_quoting_style,
    locale_quoting_style,
    clocale_quoting_style,
];
static mut default_quoting_options: quoting_options = quoting_options {
    style: literal_quoting_style,
    flags: 0,
    quote_these_too: [0; 8],
    left_quote: 0 as *const libc::c_char,
    right_quote: 0 as *const libc::c_char,
};
#[no_mangle]
pub fn clone_quoting_options(o: Option<&quoting_options>) -> quoting_options {
    let options = o.unwrap_or_else(|| unsafe { &default_quoting_options });
    options.clone()
}

#[no_mangle]
pub fn get_quoting_style(o: Option<&quoting_options>) -> quoting_style {
    let options = match o {
        Some(opts) => opts,
        None => unsafe { &default_quoting_options },
    };
    options.style
}

#[no_mangle]
pub fn set_quoting_style(
    o: Option<&mut quoting_options>,
    s: quoting_style,
) {
    let options = match o {
        Some(opts) => opts,
        None => unsafe { &mut default_quoting_options },
    };
    options.style = s;
}

#[no_mangle]
pub fn set_char_quoting(
    o: Option<&mut quoting_options>,
    c: char,
    i: i32,
) -> i32 {
    let uc: u8 = c as u8;
    let options = match o {
        Some(opts) => opts,
        None => unsafe { &mut default_quoting_options },
    };
    let index = (uc as usize) / (std::mem::size_of::<u32>() * 8);
    let shift = (uc as usize) % (std::mem::size_of::<u32>() * 8);
    
    let p = &mut options.quote_these_too[index];
    let r = (*p >> shift & 1) as i32;
    *p ^= ((i & 1 ^ r) << shift) as u32;
    
    r
}

#[no_mangle]
pub fn set_quoting_flags(
    mut o: Option<&mut quoting_options>,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let options = match o {
        Some(ref mut opt) => opt,
        None => unsafe { &mut default_quoting_options },
    };
    r = options.flags;
    options.flags = i;
    return r;
}

#[no_mangle]
pub fn set_custom_quoting(
    mut o: Option<&mut quoting_options>,
    left_quote: &str,
    right_quote: &str,
) {
    let o = o.get_or_insert_with(|| {
        // Create a mutable reference to the default quoting options
        unsafe { &mut default_quoting_options }
    });
    o.style = custom_quoting_style;

    if left_quote.is_empty() || right_quote.is_empty() {
        panic!("Quotes cannot be null");
    }

    o.left_quote = CString::new(left_quote).expect("CString::new failed").into_raw();
    o.right_quote = CString::new(right_quote).expect("CString::new failed").into_raw();
}

fn quoting_options_from_style(style: quoting_style) -> quoting_options {
    if style == custom_quoting_style {
        panic!("Custom quoting style is not allowed");
    }
    
    quoting_options {
        style,
        flags: 0,
        quote_these_too: [0; 8],
        left_quote: std::ptr::null(),
        right_quote: std::ptr::null(),
    }
}

fn gettext_quote(msgid: &str, s: quoting_style) -> String {
    let translation_ptr = unsafe { gettext(msgid.as_ptr() as *const libc::c_char) };
    let translation = unsafe { std::ffi::CStr::from_ptr(translation_ptr).to_string_lossy().into_owned() };
    let locale_code_ptr = unsafe { locale_charset() };
    let locale_code = unsafe { std::ffi::CStr::from_ptr(locale_code_ptr).to_string_lossy().into_owned() };

    if translation != msgid {
        return translation;
    }

    if locale_code.eq_ignore_ascii_case("UTF-8") {
        return if msgid.chars().next() == Some('`') {
            "\u{2018}".to_string() // Left single quotation mark
        } else {
            "\u{2019}".to_string() // Right single quotation mark
        };
    }

    if locale_code.eq_ignore_ascii_case("GB18030") {
        return if msgid.chars().next() == Some('`') {
            "\u{A1}e".to_string() // Specific character for GB18030
        } else {
            "\u{A1}AF".to_string() // Specific character for GB18030
        };
    }

    if s as u32 == clocale_quoting_style as u32 {
        return "\"".to_string(); // Double quote
    } else {
        return "'".to_string(); // Single quote
    }
}

unsafe extern "C" fn quotearg_buffer_restyled(
    mut buffer: *mut libc::c_char,
    mut buffersize: size_t,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
    mut quoting_style: quoting_style,
    mut flags: libc::c_int,
    mut quote_these_too: *const libc::c_uint,
    mut left_quote: *const libc::c_char,
    mut right_quote: *const libc::c_char,
) -> size_t {
    let mut pending_shell_escape_end: bool = false;
    let mut current_block: u64;
    let mut i: size_t = 0;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut orig_buffersize: size_t = 0 as libc::c_int as size_t;
    let mut quote_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut quote_string_len: size_t = 0 as libc::c_int as size_t;
    let mut backslash_escapes: bool = 0 as libc::c_int != 0;
    let mut unibyte_locale: bool = __ctype_get_mb_cur_max()
        == 1 as libc::c_int as libc::c_ulong;
    let mut elide_outer_quotes: bool = flags & QA_ELIDE_OUTER_QUOTES as libc::c_int
        != 0 as libc::c_int;
    let mut encountered_single_quote: bool = 0 as libc::c_int != 0;
    let mut all_c_and_shell_quote_compat: bool = 1 as libc::c_int != 0;
    's_25: loop {
        pending_shell_escape_end = 0 as libc::c_int != 0;
        let mut current_block_48: u64;
        match quoting_style as libc::c_uint {
            6 => {
                quoting_style = c_quoting_style;
                elide_outer_quotes = 1 as libc::c_int != 0;
                current_block_48 = 1283995450065962895;
            }
            5 => {
                current_block_48 = 1283995450065962895;
            }
            7 => {
                backslash_escapes = 1 as libc::c_int != 0;
                elide_outer_quotes = 0 as libc::c_int != 0;
                current_block_48 = 2989495919056355252;
            }
            8 | 9 | 10 => {
                if quoting_style as libc::c_uint
                    != custom_quoting_style as libc::c_int as libc::c_uint
                {
                    let left_quote = gettext_quote(
    "`",
    quoting_style,
);
                    let right_quote = gettext_quote(
    "'",
    quoting_style,
);
                }
                if !elide_outer_quotes {
                    quote_string = left_quote;
                    while *quote_string != 0 {
                        if len < buffersize {
                            *buffer.offset(len as isize) = *quote_string;
                        }
                        len = len.wrapping_add(1);
                        len;
                        quote_string = quote_string.offset(1);
                        quote_string;
                    }
                }
                backslash_escapes = 1 as libc::c_int != 0;
                quote_string = right_quote;
                quote_string_len = strlen(quote_string);
                current_block_48 = 2989495919056355252;
            }
            3 => {
                backslash_escapes = 1 as libc::c_int != 0;
                current_block_48 = 2694013637280298776;
            }
            1 => {
                current_block_48 = 2694013637280298776;
            }
            4 => {
                current_block_48 = 2266290538919981374;
            }
            2 => {
                current_block_48 = 14141891332124833771;
            }
            0 => {
                elide_outer_quotes = 0 as libc::c_int != 0;
                current_block_48 = 2989495919056355252;
            }
            _ => {
                abort();
            }
        }
        match current_block_48 {
            1283995450065962895 => {
                if !elide_outer_quotes {
                    if len < buffersize {
                        *buffer.offset(len as isize) = '"' as i32 as libc::c_char;
                    }
                    len = len.wrapping_add(1);
                    len;
                }
                backslash_escapes = 1 as libc::c_int != 0;
                quote_string = b"\"\0" as *const u8 as *const libc::c_char;
                quote_string_len = 1 as libc::c_int as size_t;
                current_block_48 = 2989495919056355252;
            }
            2694013637280298776 => {
                elide_outer_quotes = 1 as libc::c_int != 0;
                current_block_48 = 2266290538919981374;
            }
            _ => {}
        }
        match current_block_48 {
            2266290538919981374 => {
                if !elide_outer_quotes {
                    backslash_escapes = 1 as libc::c_int != 0;
                }
                current_block_48 = 14141891332124833771;
            }
            _ => {}
        }
        match current_block_48 {
            14141891332124833771 => {
                quoting_style = shell_always_quoting_style;
                if !elide_outer_quotes {
                    if len < buffersize {
                        *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                    }
                    len = len.wrapping_add(1);
                    len;
                }
                quote_string = b"'\0" as *const u8 as *const libc::c_char;
                quote_string_len = 1 as libc::c_int as size_t;
            }
            _ => {}
        }
        i = 0 as libc::c_int as size_t;
        while if argsize == 18446744073709551615 as libc::c_ulong {
            (*arg.offset(i as isize) as libc::c_int == '\0' as i32) as libc::c_int
        } else {
            (i == argsize) as libc::c_int
        } == 0
        {
            let mut c: libc::c_uchar = 0;
            let mut esc: libc::c_uchar = 0;
            let mut is_right_quote: bool = 0 as libc::c_int != 0;
            let mut escaping: bool = 0 as libc::c_int != 0;
            let mut c_and_shell_quote_compat: bool = 0 as libc::c_int != 0;
            if backslash_escapes as libc::c_int != 0
                && quoting_style as libc::c_uint
                    != shell_always_quoting_style as libc::c_int as libc::c_uint
                && quote_string_len != 0
                && i.wrapping_add(quote_string_len)
                    <= (if argsize == 18446744073709551615 as libc::c_ulong
                        && (1 as libc::c_int as libc::c_ulong) < quote_string_len
                    {
                        argsize = strlen(arg);
                        argsize
                    } else {
                        argsize
                    })
                && memcmp(
                    arg.offset(i as isize) as *const libc::c_void,
                    quote_string as *const libc::c_void,
                    quote_string_len,
                ) == 0 as libc::c_int
            {
                if elide_outer_quotes {
                    current_block = 7928555609993211441;
                    break 's_25;
                }
                is_right_quote = 1 as libc::c_int != 0;
            }
            c = *arg.offset(i as isize) as libc::c_uchar;
            match c as libc::c_int {
                0 => {
                    if backslash_escapes {
    if elide_outer_quotes {
        current_block = 7928555609993211441;
        break 's_25;
    }
    escaping = true;
    if quoting_style == shell_always_quoting_style && !pending_shell_escape_end {
        if len < buffersize {
            unsafe { *buffer.offset(len as isize) = '\'' as i8 };
        }
        len += 1;
        if len < buffersize {
            unsafe { *buffer.offset(len as isize) = '$' as i8 };
        }
        len += 1;
        if len < buffersize {
            unsafe { *buffer.offset(len as isize) = '\'' as i8 };
        }
        len += 1;
        pending_shell_escape_end = true;
    }
    if len < buffersize {
        unsafe { *buffer.offset(len as isize) = '\\' as i8 };
    }
    len += 1;
    if quoting_style != shell_always_quoting_style
        && i + 1 < argsize
        && ('0' as i8 <= unsafe { *arg.offset((i + 1) as isize) } && unsafe { *arg.offset((i + 1) as isize) } <= '9' as i8)
    {
        if len < buffersize {
            unsafe { *buffer.offset(len as isize) = '0' as i8 };
        }
        len += 1;
        if len < buffersize {
            unsafe { *buffer.offset(len as isize) = '0' as i8 };
        }
        len += 1;
    }
    c = '0' as u8;
    current_block = 253337042034819032;
} else if (flags & QA_ELIDE_NULL_BYTES as i32) != 0 {
    current_block = 13619784596304402172;
} else {
    current_block = 253337042034819032;
}

                }
                63 => {
                    match quoting_style as libc::c_uint {
                        2 => {
                            current_block = 17954593875197965021;
                            match current_block {
                                17954593875197965021 => {
                                    if elide_outer_quotes {
                                        current_block = 7928555609993211441;
                                        break 's_25;
                                    }
                                }
                                _ => {
                                    if flags & QA_SPLIT_TRIGRAPHS as libc::c_int != 0
                                        && i.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                            < argsize
                                        && *arg
                                            .offset(
                                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                            ) as libc::c_int == '?' as i32
                                    {
                                        match *arg
                                            .offset(
                                                i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                                            ) as libc::c_int
                                        {
                                            33 | 39 | 40 | 41 | 45 | 47 | 60 | 61 | 62 => {
                                                if elide_outer_quotes {
                                                    current_block = 7928555609993211441;
                                                    break 's_25;
                                                }
                                                c = *arg
                                                    .offset(
                                                        i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                                                    ) as libc::c_uchar;
                                                i = (i as libc::c_ulong)
                                                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                                                    as size_t;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            current_block = 253337042034819032;
                        }
                        5 => {
                            current_block = 14514781131754033399;
                            match current_block {
                                17954593875197965021 => {
                                    if elide_outer_quotes {
                                        current_block = 7928555609993211441;
                                        break 's_25;
                                    }
                                }
                                _ => {
                                    if flags & QA_SPLIT_TRIGRAPHS as libc::c_int != 0
                                        && i.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                            < argsize
                                        && *arg
                                            .offset(
                                                i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                            ) as libc::c_int == '?' as i32
                                    {
                                        match *arg
                                            .offset(
                                                i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                                            ) as libc::c_int
                                        {
                                            33 | 39 | 40 | 41 | 45 | 47 | 60 | 61 | 62 => {
                                                if elide_outer_quotes {
                                                    current_block = 7928555609993211441;
                                                    break 's_25;
                                                }
                                                c = *arg
                                                    .offset(
                                                        i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                                                    ) as libc::c_uchar;
                                                i = (i as libc::c_ulong)
                                                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                                                    as size_t;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '"' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                                if len < buffersize {
                                                    *buffer.offset(len as isize) = '?' as i32 as libc::c_char;
                                                }
                                                len = len.wrapping_add(1);
                                                len;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            current_block = 253337042034819032;
                        }
                        _ => {
                            current_block = 253337042034819032;
                        }
                    }
                }
                7 => {
                    esc = 'a' as i32 as libc::c_uchar;
                    current_block = 1190876092451756080;
                }
                8 => {
                    esc = 'b' as i32 as libc::c_uchar;
                    current_block = 1190876092451756080;
                }
                12 => {
                    esc = 'f' as i32 as libc::c_uchar;
                    current_block = 1190876092451756080;
                }
                10 => {
                    esc = 'n' as i32 as libc::c_uchar;
                    current_block = 9215498979640025612;
                }
                13 => {
                    esc = 'r' as i32 as libc::c_uchar;
                    current_block = 9215498979640025612;
                }
                9 => {
                    esc = 't' as i32 as libc::c_uchar;
                    current_block = 9215498979640025612;
                }
                11 => {
                    esc = 'v' as i32 as libc::c_uchar;
                    current_block = 1190876092451756080;
                }
                92 => {
                    esc = c;
if quoting_style == shell_always_quoting_style {
    if elide_outer_quotes {
        current_block = 7928555609993211441;
        break 's_25;
    }
    current_block = 4476262310586904498;
} else if backslash_escapes && elide_outer_quotes && quote_string_len != 0 {
    current_block = 4476262310586904498;
} else {
    current_block = 9215498979640025612;
}

                }
                123 | 125 => {
                    if if argsize == 18446744073709551615 as libc::c_ulong {
                        (*arg.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32) as libc::c_int
                    } else {
                        (argsize == 1 as libc::c_int as libc::c_ulong) as libc::c_int
                    } == 0
                    {
                        current_block = 253337042034819032;
                    } else {
                        current_block = 16442922512115311366;
                    }
                }
                35 | 126 => {
                    current_block = 16442922512115311366;
                }
                32 => {
                    current_block = 4634307283396172174;
                }
                33 => {
                    current_block = 7549413860336125482;
                }
                34 | 36 | 38 | 40 | 41 | 42 | 59 | 60 | 61 => {
                    current_block = 13059411171234995867;
                }
                62 | 91 | 94 => {
                    current_block = 16620298045565028098;
                }
                96 | 124 => {
                    current_block = 15155215915847730705;
                }
                39 => {
                    encountered_single_quote = true;
c_and_shell_quote_compat = true;

if quoting_style == shell_always_quoting_style {
    if elide_outer_quotes {
        current_block = 7928555609993211441;
        break 's_25;
    }
    if buffersize != 0 && orig_buffersize == 0 {
        orig_buffersize = buffersize;
        buffersize = 0;
    }
    if len < buffersize {
        let byte_buffer = unsafe { std::slice::from_raw_parts_mut(buffer, buffersize as usize) };
        byte_buffer[len as usize] = '\'' as i8;
    }
    len = len.wrapping_add(1);
    
    if len < buffersize {
        let byte_buffer = unsafe { std::slice::from_raw_parts_mut(buffer, buffersize as usize) };
        byte_buffer[len as usize] = '\\' as i8;
    }
    len = len.wrapping_add(1);
    
    if len < buffersize {
        let byte_buffer = unsafe { std::slice::from_raw_parts_mut(buffer, buffersize as usize) };
        byte_buffer[len as usize] = '\'' as i8;
    }
    len = len.wrapping_add(1);
    
    pending_shell_escape_end = false;
    current_block = 253337042034819032;
} else {
    current_block = 253337042034819032;
}

                }
                37 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56
                | 57 | 58 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76
                | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90
                | 93 | 95 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107
                | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119
                | 120 | 121 | 122 => {
                    c_and_shell_quote_compat = 1 as libc::c_int != 0;
                    current_block = 253337042034819032;
                }
                _ => {
                    let mut m: size_t = 0;
                    let mut printable: bool = false;
                    if unibyte_locale {
                        m = 1 as libc::c_int as size_t;
                        printable = *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                            as libc::c_int
                            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                            != 0 as libc::c_int;
                    } else {
                        let mut mbstate: mbstate_t = mbstate_t {
                            __count: 0,
                            __value: C2RustUnnamed { __wch: 0 },
                        };
                        mbszero(&mut mbstate);
                        m = 0 as libc::c_int as size_t;
                        printable = 1 as libc::c_int != 0;
                        if argsize == 18446744073709551615 as libc::c_ulong {
                            argsize = strlen(arg);
                        }
                        let mut w: char32_t = 0;
                        let mut bytes: size_t = rpl_mbrtoc32(
                            &mut w,
                            &*arg.offset(i.wrapping_add(m) as isize),
                            argsize.wrapping_sub(i.wrapping_add(m)),
                            &mut mbstate,
                        );
                        if !(bytes == 0 as libc::c_int as libc::c_ulong) {
                            if bytes == -(1 as libc::c_int) as size_t {
                                printable = 0 as libc::c_int != 0;
                            } else if bytes == -(2 as libc::c_int) as size_t {
                                printable = 0 as libc::c_int != 0;
                                while i.wrapping_add(m) < argsize
                                    && *arg.offset(i.wrapping_add(m) as isize) as libc::c_int
                                        != 0
                                {
                                    m = m.wrapping_add(1);
                                    m;
                                }
                            } else {
                                if '[' as i32 == 0x5b as libc::c_int
                                    && elide_outer_quotes as libc::c_int != 0
                                    && quoting_style as libc::c_uint
                                        == shell_always_quoting_style as libc::c_int as libc::c_uint
                                {
                                    let mut j: size_t = 0;
                                    j = 1 as libc::c_int as size_t;
                                    while j < bytes {
                                        match *arg
                                            .offset(i.wrapping_add(m).wrapping_add(j) as isize)
                                            as libc::c_int
                                        {
                                            91 | 92 | 94 | 96 | 124 => {
                                                current_block = 7928555609993211441;
                                                break 's_25;
                                            }
                                            _ => {}
                                        }
                                        j = j.wrapping_add(1);
                                        j;
                                    }
                                }
                                if c32isprint(w) == 0 {
                                    printable = 0 as libc::c_int != 0;
                                }
                                m = (m as libc::c_ulong).wrapping_add(bytes) as size_t
                                    as size_t;
                            }
                        }
                    }
                    c_and_shell_quote_compat = printable;
                    if m > 1 || (backslash_escapes && !printable) {
    let ilim = i.wrapping_add(m);
    let buffer_slice = unsafe { std::slice::from_raw_parts_mut(buffer, buffersize as usize) };
    loop {
        if backslash_escapes && !printable {
            if elide_outer_quotes {
                current_block = 7928555609993211441;
                break 's_25;
            }
            escaping = true;
            if quoting_style == shell_always_quoting_style && !pending_shell_escape_end {
                if len < buffersize {
                    buffer_slice[len as usize] = b'\'' as i8; // Add single quote
                }
                len += 1;
                if len < buffersize {
                    buffer_slice[len as usize] = b'$' as i8; // Add dollar sign
                }
                len += 1;
                if len < buffersize {
                    buffer_slice[len as usize] = b'\'' as i8; // Add single quote
                }
                len += 1;
                pending_shell_escape_end = true;
            }
            if len < buffersize {
                buffer_slice[len as usize] = b'\\' as i8; // Add backslash
            }
            len += 1;
            if len < buffersize {
                buffer_slice[len as usize] = ('0' as u8 + (c >> 6)) as i8; // Add first octet
            }
            len += 1;
            if len < buffersize {
                buffer_slice[len as usize] = ('0' as u8 + ((c >> 3) & 7)) as i8; // Add second octet
            }
            len += 1;
            c = ('0' as u8 + (c & 7)) as u8; // Update c to last octet
        } else if is_right_quote {
            if len < buffersize {
                buffer_slice[len as usize] = b'\\' as i8; // Add backslash
            }
            len += 1;
            is_right_quote = false;
        }
        if ilim <= i.wrapping_add(1) {
            break;
        }
        if pending_shell_escape_end && !escaping {
            if len < buffersize {
                buffer_slice[len as usize] = b'\'' as i8; // Add single quote
            }
            len += 1;
            if len < buffersize {
                buffer_slice[len as usize] = b'\'' as i8; // Add single quote
            }
            len += 1;
            pending_shell_escape_end = false;
        }
        if len < buffersize {
            buffer_slice[len as usize] = c as i8; // Add current character
        }
        len += 1;
        i += 1;
        c = unsafe { *arg.offset(i as isize) } as u8; // Update c to next character
    }
    current_block = 4476262310586904498;
} else {
    current_block = 253337042034819032;
}

                }
            }
            match current_block {
                16442922512115311366 => {
                    if i != 0 as libc::c_int as libc::c_ulong {
                        current_block = 253337042034819032;
                    } else {
                        current_block = 4634307283396172174;
                    }
                }
                9215498979640025612 => {
                    if quoting_style as libc::c_uint
                        == shell_always_quoting_style as libc::c_int as libc::c_uint
                        && elide_outer_quotes as libc::c_int != 0
                    {
                        current_block = 7928555609993211441;
                        break 's_25;
                    }
                    current_block = 1190876092451756080;
                }
                _ => {}
            }
            match current_block {
                1190876092451756080 => {
                    if backslash_escapes {
                        c = esc;
                        current_block = 3173013908131212616;
                    } else {
                        current_block = 253337042034819032;
                    }
                }
                4634307283396172174 => {
                    c_and_shell_quote_compat = 1 as libc::c_int != 0;
                    current_block = 7549413860336125482;
                }
                _ => {}
            }
            match current_block {
                7549413860336125482 => {
                    current_block = 13059411171234995867;
                }
                _ => {}
            }
            match current_block {
                13059411171234995867 => {
                    current_block = 16620298045565028098;
                }
                _ => {}
            }
            match current_block {
                16620298045565028098 => {
                    current_block = 15155215915847730705;
                }
                _ => {}
            }
            match current_block {
                15155215915847730705 => {
                    if quoting_style as libc::c_uint
                        == shell_always_quoting_style as libc::c_int as libc::c_uint
                        && elide_outer_quotes as libc::c_int != 0
                    {
                        current_block = 7928555609993211441;
                        break 's_25;
                    }
                    current_block = 253337042034819032;
                }
                _ => {}
            }
            match current_block {
                253337042034819032 => {
                    if !((backslash_escapes as libc::c_int != 0
                        && quoting_style as libc::c_uint
                            != shell_always_quoting_style as libc::c_int as libc::c_uint
                        || elide_outer_quotes as libc::c_int != 0)
                        && !quote_these_too.is_null()
                        && *quote_these_too
                            .offset(
                                (c as libc::c_ulong)
                                    .wrapping_div(
                                        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            )
                            >> (c as libc::c_ulong)
                                .wrapping_rem(
                                    (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                                ) & 1 as libc::c_int as libc::c_uint != 0)
                        && !is_right_quote
                    {
                        current_block = 4476262310586904498;
                    } else {
                        current_block = 3173013908131212616;
                    }
                }
                _ => {}
            }
            match current_block {
                3173013908131212616 => {
                    if elide_outer_quotes {
                        current_block = 7928555609993211441;
                        break 's_25;
                    }
                    escaping = 1 as libc::c_int != 0;
                    if quoting_style as libc::c_uint
                        == shell_always_quoting_style as libc::c_int as libc::c_uint
                        && !pending_shell_escape_end
                    {
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '$' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        pending_shell_escape_end = 1 as libc::c_int != 0;
                    }
                    if len < buffersize {
                        *buffer.offset(len as isize) = '\\' as i32 as libc::c_char;
                    }
                    len = len.wrapping_add(1);
                    len;
                    current_block = 4476262310586904498;
                }
                _ => {}
            }
            match current_block {
                4476262310586904498 => {
                    if pending_shell_escape_end as libc::c_int != 0 && !escaping {
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        if len < buffersize {
                            *buffer.offset(len as isize) = '\'' as i32 as libc::c_char;
                        }
                        len = len.wrapping_add(1);
                        len;
                        pending_shell_escape_end = 0 as libc::c_int != 0;
                    }
                    if len < buffersize {
                        *buffer.offset(len as isize) = c as libc::c_char;
                    }
                    len = len.wrapping_add(1);
                    len;
                    if !c_and_shell_quote_compat {
                        all_c_and_shell_quote_compat = 0 as libc::c_int != 0;
                    }
                }
                _ => {}
            }
            i = i.wrapping_add(1);
            i;
        }
        if len == 0 as libc::c_int as libc::c_ulong
            && quoting_style as libc::c_uint
                == shell_always_quoting_style as libc::c_int as libc::c_uint
            && elide_outer_quotes as libc::c_int != 0
        {
            current_block = 7928555609993211441;
            break;
        }
        if !(quoting_style as libc::c_uint
            == shell_always_quoting_style as libc::c_int as libc::c_uint
            && !elide_outer_quotes && encountered_single_quote as libc::c_int != 0)
        {
            current_block = 6412618891452676311;
            break;
        }
        if all_c_and_shell_quote_compat {
    return quotearg_buffer_restyled(
        buffer,
        orig_buffersize,
        arg,
        argsize,
        c_quoting_style,
        flags,
        quote_these_too,
        left_quote,
        right_quote,
    );
} else {
    if !(buffersize == 0 && orig_buffersize != 0) {
        current_block = 6412618891452676311;
        break;
    }
    buffersize = orig_buffersize;
    len = 0;
}

    }
    match current_block {
        7928555609993211441 => {
            if quoting_style as libc::c_uint
                == shell_always_quoting_style as libc::c_int as libc::c_uint
                && backslash_escapes as libc::c_int != 0
            {
                quoting_style = shell_escape_always_quoting_style;
            }
            return quotearg_buffer_restyled(
                buffer,
                buffersize,
                arg,
                argsize,
                quoting_style,
                flags & !(QA_ELIDE_OUTER_QUOTES as libc::c_int),
                0 as *const libc::c_uint,
                left_quote,
                right_quote,
            );
        }
        _ => {
            if !quote_string.is_null() && !elide_outer_quotes {
                while *quote_string != 0 {
                    if len < buffersize {
                        *buffer.offset(len as isize) = *quote_string;
                    }
                    len = len.wrapping_add(1);
                    len;
                    quote_string = quote_string.offset(1);
                    quote_string;
                }
            }
            if len < buffersize {
                *buffer.offset(len as isize) = '\0' as i32 as libc::c_char;
            }
            return len;
        }
    };
}
#[no_mangle]
pub fn quotearg_buffer(
    buffer: &mut [u8],
    arg: &[u8],
    o: Option<&quoting_options>,
) -> usize {
    let p: &quoting_options;
    if let Some(options) = o {
        p = options;
    } else {
        p = unsafe { &default_quoting_options };
    }
    
    let buffersize = buffer.len() as size_t;
    let argsize = arg.len() as size_t;

    let e = std::io::Error::last_os_error();
    
    let r = unsafe {
        quotearg_buffer_restyled(
            buffer.as_mut_ptr() as *mut libc::c_char,
            buffersize,
            arg.as_ptr() as *const libc::c_char,
            argsize,
            p.style,
            p.flags,
            p.quote_these_too.as_ptr(),
            p.left_quote,
            p.right_quote,
        )
    };
    
    std::mem::forget(e); // Restore the previous error state if needed
    r.try_into().unwrap() // Convert u64 to usize
}

#[no_mangle]
pub fn quotearg_alloc(
    arg: &CStr,
    argsize: usize,
    o: &quoting_options,
) -> CString {
    let mut size: size_t = 0; // Use size_t for compatibility with the C function
    let result_ptr: *mut libc::c_char;

    unsafe {
        result_ptr = quotearg_alloc_mem(arg.as_ptr(), argsize.try_into().unwrap(), &mut size, o);
    }

    unsafe { CString::from_raw(result_ptr) } // Convert the raw pointer to CString
}

#[no_mangle]
pub unsafe extern "C" fn quotearg_alloc_mem(
    mut arg: *const libc::c_char,
    mut argsize: size_t,
    mut size: *mut size_t,
    mut o: *const quoting_options,
) -> *mut libc::c_char {
    let mut p: *const quoting_options = if !o.is_null() {
        o
    } else {
        &mut default_quoting_options as *mut quoting_options as *const quoting_options
    };
    let mut e: libc::c_int = *__errno_location();
    let mut flags: libc::c_int = (*p).flags
        | (if !size.is_null() {
            0 as libc::c_int
        } else {
            QA_ELIDE_NULL_BYTES as libc::c_int
        });
    let mut bufsize: size_t = (quotearg_buffer_restyled(
        0 as *mut libc::c_char,
        0 as libc::c_int as size_t,
        arg,
        argsize,
        (*p).style,
        flags,
        ((*p).quote_these_too).as_ptr(),
        (*p).left_quote,
        (*p).right_quote,
    ))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut buf: *mut libc::c_char = xcharalloc(bufsize);
    quotearg_buffer_restyled(
        buf,
        bufsize,
        arg,
        argsize,
        (*p).style,
        flags,
        ((*p).quote_these_too).as_ptr(),
        (*p).left_quote,
        (*p).right_quote,
    );
    *__errno_location() = e;
    if !size.is_null() {
        *size = bufsize.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
    return buf;
}
static mut slot0: [libc::c_char; 256] = [0; 256];
static mut nslots: libc::c_int = 1 as libc::c_int;
static mut slotvec0: slotvec = unsafe {
    {
        let mut init = slotvec {
            size: ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            val: slot0.as_ptr() as *mut _,
        };
        init
    }
};
static mut slotvec: *mut slotvec = unsafe {
    &slotvec0 as *const slotvec as *mut slotvec
};
#[no_mangle]
pub fn quotearg_free() {
    unsafe {
        let sv = Vec::from_raw_parts(slotvec, nslots as usize, nslots as usize);
        for i in 1..nslots {
            let val = (*sv.get_unchecked(i as usize)).val;
            drop(Box::from_raw(val)); // Assuming val is a pointer to a type that needs to be dropped
        }
        if (*sv.get_unchecked(0)).val != slot0.as_mut_ptr() {
            let val = (*sv.get_unchecked(0)).val;
            drop(Box::from_raw(val)); // Assuming val is a pointer to a type that needs to be dropped
            slotvec0.size = std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong;
            slotvec0.val = slot0.as_mut_ptr();
        }
        if sv.as_ptr() != &slotvec0 as *const _ as *mut _ {
            drop(sv); // Assuming sv is a type that implements Drop
            slotvec = &mut slotvec0;
        }
        nslots = 1;
    }
}

unsafe extern "C" fn quotearg_n_options(
    mut n: libc::c_int,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
    mut options: *const quoting_options,
) -> *mut libc::c_char {
    let mut e: libc::c_int = *__errno_location();
    let mut sv: *mut slotvec = slotvec;
    let mut nslots_max: libc::c_int = (if (2147483647 as libc::c_int as libc::c_long)
        < 9223372036854775807 as libc::c_long
    {
        2147483647 as libc::c_int as libc::c_long
    } else {
        9223372036854775807 as libc::c_long
    }) as libc::c_int;
    if !(0 as libc::c_int <= n && n < nslots_max) {
        abort();
    }
    if nslots <= n {
        let mut preallocated: bool = sv == &mut slotvec0 as *mut slotvec;
        let mut new_nslots: idx_t = nslots as idx_t;
        sv = xpalloc(
            (if preallocated as libc::c_int != 0 { 0 as *mut slotvec } else { sv })
                as *mut libc::c_void,
            &mut new_nslots,
            (n - nslots + 1 as libc::c_int) as idx_t,
            nslots_max as ptrdiff_t,
            ::core::mem::size_of::<slotvec>() as libc::c_ulong as idx_t,
        ) as *mut slotvec;
        slotvec = sv;
        if preallocated {
            *sv = slotvec0;
        }
        memset(
            sv.offset(nslots as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((new_nslots - nslots as libc::c_long) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<slotvec>() as libc::c_ulong),
        );
        nslots = new_nslots as libc::c_int;
    }
    let mut size: size_t = (*sv.offset(n as isize)).size;
    let mut val: *mut libc::c_char = (*sv.offset(n as isize)).val;
    let mut flags: libc::c_int = (*options).flags | QA_ELIDE_NULL_BYTES as libc::c_int;
    let mut qsize: size_t = quotearg_buffer_restyled(
        val,
        size,
        arg,
        argsize,
        (*options).style,
        flags,
        ((*options).quote_these_too).as_ptr(),
        (*options).left_quote,
        (*options).right_quote,
    );
    if size <= qsize {
        size = qsize.wrapping_add(1 as libc::c_int as libc::c_ulong);
        (*sv.offset(n as isize)).size = size;
        if val != slot0.as_mut_ptr() {
            free(val as *mut libc::c_void);
        }
        val = xcharalloc(size);
        let ref mut fresh0 = (*sv.offset(n as isize)).val;
        *fresh0 = val;
        quotearg_buffer_restyled(
            val,
            size,
            arg,
            argsize,
            (*options).style,
            flags,
            ((*options).quote_these_too).as_ptr(),
            (*options).left_quote,
            (*options).right_quote,
        );
    }
    *__errno_location() = e;
    return val;
}
#[no_mangle]
pub fn quotearg_n(n: i32, arg: &CStr) -> *mut libc::c_char {
    unsafe {
        let options = &default_quoting_options;
        quotearg_n_options(
            n,
            arg.as_ptr(),
            u64::MAX,
            options,
        )
    }
}

#[no_mangle]
pub fn quotearg_n_mem(
    n: i32,
    arg: &str,
) -> *mut libc::c_char {
    let arg_cstring = CString::new(arg).expect("CString::new failed");
    let argsize = arg_cstring.as_bytes().len() as libc::c_ulong; // Convert to u64
    unsafe {
        quotearg_n_options(n, arg_cstring.as_ptr(), argsize, &default_quoting_options)
    }
}

#[no_mangle]
pub fn quotearg(arg: &CStr) -> CString {
    let result = quotearg_n(0, arg);
    unsafe { CString::from_raw(result) }
}

#[no_mangle]
pub fn quotearg_mem(arg: &CStr) -> CString {
    let arg_str = arg.to_str().expect("Failed to convert CStr to &str");
    let result = quotearg_n_mem(0, arg_str);
    unsafe { CString::from_raw(result) }
}

#[no_mangle]
pub fn quotearg_n_style(
    n: i32,
    s: quoting_style,
    arg: &str,
) -> *mut libc::c_char {
    let o: quoting_options = quoting_options_from_style(s);
    unsafe {
        quotearg_n_options(n, arg.as_ptr() as *const libc::c_char, u64::MAX, &o)
    }
}

#[no_mangle]
pub fn quotearg_n_style_mem(
    n: i32,
    s: quoting_style,
    arg: &str,
) -> *mut libc::c_char {
    let o: quoting_options = quoting_options_from_style(s);
    let arg_len = arg.len() as u64; // Convert to u64
    unsafe {
        quotearg_n_options(n, arg.as_ptr() as *const libc::c_char, arg_len, &o)
    }
}

#[no_mangle]
pub unsafe extern "C" fn quotearg_style(
    mut s: quoting_style,
    mut arg: *const libc::c_char,
) -> *mut libc::c_char {
    let result = quotearg_n_style(0, s, std::ffi::CStr::from_ptr(arg).to_str().unwrap());
return result;
}
#[no_mangle]
pub fn quotearg_style_mem(
    s: quoting_style,
    arg: &CStr,
) -> CString {
    let arg_str = arg.to_str().expect("Failed to convert CStr to &str");
    let result = quotearg_n_style_mem(0, s, arg_str);
    unsafe { CString::from_raw(result) }
}

#[no_mangle]
pub unsafe extern "C" fn quotearg_char_mem(
    mut arg: *const libc::c_char,
    mut argsize: size_t,
    mut ch: libc::c_char,
) -> *mut libc::c_char {
    let mut options: quoting_options = quoting_options {
        style: literal_quoting_style,
        flags: 0,
        quote_these_too: [0; 8],
        left_quote: 0 as *const libc::c_char,
        right_quote: 0 as *const libc::c_char,
    };
    options = default_quoting_options;
    set_char_quoting(Some(&mut options), ch as u8 as char, 1);
    return quotearg_n_options(0 as libc::c_int, arg, argsize, &mut options);
}
#[no_mangle]
pub fn quotearg_char(arg: &CStr, ch: char) -> *mut c_char {
    let ch_as_c_char = ch as u8 as c_char;
    unsafe { quotearg_char_mem(arg.as_ptr(), u64::MAX, ch_as_c_char) }
}

#[no_mangle]
pub fn quotearg_colon(arg: &CStr) -> CString {
    let result_ptr = quotearg_char(arg, ':' as char);
    unsafe { CString::from_raw(result_ptr) }
}

#[no_mangle]
pub fn quotearg_colon_mem(arg: &str) -> String {
    let argsize = arg.len() as u64;
    let ch = ':' as i8;
    let c_string = unsafe { quotearg_char_mem(arg.as_ptr() as *const libc::c_char, argsize, ch) };
    unsafe { CString::from_raw(c_string).into_string().unwrap() }
}

#[no_mangle]
pub fn quotearg_n_style_colon(
    n: i32,
    s: quoting_style,
    arg: &CStr,
) -> CString {
    let mut options: quoting_options = quoting_options {
        style: literal_quoting_style,
        flags: 0,
        quote_these_too: [0; 8],
        left_quote: std::ptr::null(),
        right_quote: std::ptr::null(),
    };
    options = quoting_options_from_style(s);
    set_char_quoting(Some(&mut options), ':' as char, 1);
    
    let result = unsafe {
        quotearg_n_options(
            n,
            arg.as_ptr(),
            u64::MAX,
            &mut options,
        )
    };
    
    unsafe { CString::from_raw(result) }
}

#[no_mangle]
pub unsafe extern "C" fn quotearg_n_custom(
    mut n: libc::c_int,
    mut left_quote: *const libc::c_char,
    mut right_quote: *const libc::c_char,
    mut arg: *const libc::c_char,
) -> *mut libc::c_char {
    return quotearg_n_custom_mem(
        n,
        left_quote,
        right_quote,
        arg,
        18446744073709551615 as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn quotearg_n_custom_mem(
    mut n: libc::c_int,
    mut left_quote: *const libc::c_char,
    mut right_quote: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut argsize: size_t,
) -> *mut libc::c_char {
    let mut o: quoting_options = default_quoting_options;
    let left_quote_str = unsafe { CStr::from_ptr(left_quote).to_string_lossy().into_owned() };
let right_quote_str = unsafe { CStr::from_ptr(right_quote).to_string_lossy().into_owned() };
set_custom_quoting(Some(&mut o), &left_quote_str, &right_quote_str);
    return quotearg_n_options(n, arg, argsize, &mut o);
}
#[no_mangle]
pub fn quotearg_custom(
    left_quote: &str,
    right_quote: &str,
    arg: &str,
) -> String {
    let left_quote_c = CString::new(left_quote).unwrap();
    let right_quote_c = CString::new(right_quote).unwrap();
    let arg_c = CString::new(arg).unwrap();
    
    let result = unsafe {
        quotearg_n_custom(0, left_quote_c.as_ptr(), right_quote_c.as_ptr(), arg_c.as_ptr())
    };
    
    unsafe { CStr::from_ptr(result).to_string_lossy().into_owned() }
}

#[no_mangle]
pub fn quotearg_custom_mem(
    left_quote: &CStr,
    right_quote: &CStr,
    arg: &CStr,
) -> CString {
    let argsize: usize = arg.to_bytes().len();
    let result_ptr: *mut libc::c_char = unsafe {
        quotearg_n_custom_mem(
            0,
            left_quote.as_ptr(),
            right_quote.as_ptr(),
            arg.as_ptr(),
            argsize.try_into().unwrap(),
        )
    };
    unsafe { CString::from_raw(result_ptr) }
}

#[no_mangle]
pub static mut quote_quoting_options: quoting_options = {
    let mut init = quoting_options {
        style: locale_quoting_style,
        flags: 0 as libc::c_int,
        quote_these_too: [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0, 0],
        left_quote: 0 as *const libc::c_char,
        right_quote: 0 as *const libc::c_char,
    };
    init
};
#[no_mangle]
pub fn quote_n_mem(
    n: i32,
    arg: &CStr,
) -> *const libc::c_char {
    let argsize = arg.to_bytes().len() as u64;
    let quoted = unsafe { quotearg_n_options(n, arg.as_ptr(), argsize, &quote_quoting_options) };
    quoted
}

#[no_mangle]
pub unsafe extern "C" fn quote_mem(
    mut arg: *const libc::c_char,
    mut argsize: size_t,
) -> *const libc::c_char {
    let arg_cstr = CStr::from_ptr(arg);
return quote_n_mem(0, arg_cstr);
}
#[no_mangle]
pub fn quote_n(n: i32, arg: &CStr) -> *const libc::c_char {
    quote_n_mem(n, arg)
}

#[no_mangle]
pub fn quote(arg: &str) -> String {
    let c_str = CString::new(arg).expect("CString::new failed");
    let result = quote_n(0, &c_str);
    unsafe { CStr::from_ptr(result).to_string_lossy().into_owned() }
}

