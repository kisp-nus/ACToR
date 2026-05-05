







use std::os::raw::c_int;

use std::ops::RangeInclusive;

use std::convert::TryFrom;

use std::char;

use ::libc;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isalnum(c: i32) -> bool {
    (c >= 48 && c <= 57) || (c >= 65 && c <= 90) || (c >= 97 && c <= 122)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isalpha(c: libc::c_int) -> bool {
    c >= 65 && c <= 90 || c >= 97 && c <= 122
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isascii(c: i32) -> bool {
    (0..=127).contains(&c)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn c_isblank(mut c: libc::c_int) -> bool {
    return c == ' ' as i32 || c == '\t' as i32;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_iscntrl(c: i32) -> bool {
    match c {
        0..=31 | 127 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isdigit(c: i32) -> bool {
    c >= 48 && c <= 57
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isgraph(c: i32) -> bool {
    match c {
        48..=57 | 97..=122 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40
        | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58 | 59 | 60 | 61 | 62
        | 63 | 64 | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_islower(c: i32) -> bool {
    (c >= 97 && c <= 122)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isprint(c: i32) -> bool {
    char::from_u32(c as u32).map_or(false, |ch| ch.is_ascii() && (ch.is_alphanumeric() || ch.is_ascii_punctuation() || ch.is_whitespace()))
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_ispunct(c: i32) -> bool {
    match c {
        33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58
        | 59 | 60 | 61 | 62 | 63 | 64 | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125
        | 126 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isupper(c: i32) -> bool {
    matches!(c, 65..=90)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn c_isxdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 65 | 66 | 67 | 68 | 69 | 70 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_tolower(c: i32) -> i32 {
    if let Some(ch) = char::from_u32(c as u32) {
        if ch.is_ascii_uppercase() {
            return ch.to_ascii_lowercase() as i32;
        }
    }
    c
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_toupper(c: i32) -> i32 {
    if let Some(ch) = char::from_u32(c as u32) {
        if ch.is_ascii_lowercase() {
            return c - ('a' as i32 - 'A' as i32);
        }
    }
    c
}

