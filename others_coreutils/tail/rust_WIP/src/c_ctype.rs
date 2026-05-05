











use std::convert::TryFrom;

use std::char;

use std::ops::RangeInclusive;

use ::libc;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isalnum(c: i32) -> bool {
    match c {
        48..=57 | 65..=90 | 97..=122 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isalpha(c: i32) -> bool {
    matches!(c, 97..=122 | 65..=90)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isascii(c: i32) -> bool {
    match c {
        32 | 7 | 8 | 12 | 10 | 13 | 9 | 11 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 14 | 15 | 16
        | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 127
        | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101
        | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114
        | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 33 | 34 | 35 | 36 | 37 | 38
        | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58 | 59 | 60 | 61 | 62 | 63 | 64
        | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126 | 65 | 66 | 67 | 68 | 69
        | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85
        | 86 | 87 | 88 | 89 | 90 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isblank(c: char) -> bool {
    c == ' ' || c == '\t'
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_iscntrl(c: i32) -> bool {
    matches!(c, 0..=31 | 127)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isdigit(c: i32) -> bool {
    matches!(c, 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isgraph(c: i32) -> bool {
    matches!(c, 
        48..=57 | 97..=122 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 |
        58 | 59 | 60 | 61 | 62 | 63 | 64 | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126 | 
        65..=90
    )
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_islower(c: char) -> bool {
    c.is_lowercase()
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isprint(c: i32) -> bool {
    match c {
        32 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101
        | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114
        | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 33 | 34 | 35 | 36 | 37 | 38
        | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58 | 59 | 60 | 61 | 62 | 63 | 64
        | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126 | 65 | 66 | 67 | 68 | 69
        | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85
        | 86 | 87 | 88 | 89 | 90 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_ispunct(c: i32) -> bool {
    match c as u8 {
        33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58
        | 59 | 60 | 61 | 62 | 63 | 64 | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125
        | 126 => true,
        _ => false,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_isspace(c: i32) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => true,
        _ => false,
    }
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
pub fn c_isxdigit(c: i32) -> bool {
    matches!(c, 48..=57 | 97..=102 | 65..=70)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_tolower(c: i32) -> i32 {
    if (65..=90).contains(&c) {
        return c - ('A' as i32 - 'a' as i32);
    }
    c
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn c_toupper(c: i32) -> i32 {
    if let Some(ch) = char::from_u32(c as u32) {
        if ch.is_ascii_lowercase() {
            return ch.to_ascii_uppercase() as i32;
        }
    }
    c
}

