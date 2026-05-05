






use std::ffi::CStr;

use ::libc;
extern "C" {
    fn mbrtoc32(
        __pc32: *mut char32_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint_least32_t = __uint32_t;
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
pub type mbstate_t = __mbstate_t;
pub type char32_t = __uint_least32_t;
pub type wint_t = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MCEL_LEN_MAX: C2RustUnnamed_0 = 4;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MCEL_CHAR_MAX: C2RustUnnamed_1 = 1114111;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MCEL_ERR_MIN: C2RustUnnamed_2 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcel_t {
    pub ch: char32_t,
    pub err: libc::c_uchar,
    pub len: libc::c_uchar,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const MCEL_ERR_SHIFT: C2RustUnnamed_3 = 14;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_scanz(p: &str) -> mcel_t {
    return mcel_scant(p, '\0');
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_scant(p: &str, terminator: char) -> mcel_t {
    if mcel_isbasic(p.chars().next().unwrap() as i8) {
        return mcel_ch(p.chars().next().unwrap() as char32_t, 1);
    }
    
    let mut lim = &p[1..];
    let mut i = 0;

    while i < MCEL_LEN_MAX - 1 {
        if lim.is_empty() || lim.chars().next().unwrap() == terminator {
            break;
        }
        lim = &lim[1..];
        i += 1;
    }
    
    return mcel_scan(p, lim);
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_scan(p: &str, lim: &str) -> mcel_t {
    let c = p.chars().next().unwrap_or('\0') as i8;
    if mcel_isbasic(c) {
        return mcel_ch(c as char32_t, 1);
    }
    
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    
    let mut ch: char32_t = 0;
    let len: u64;
    
    unsafe {
        len = mbrtoc32(&mut ch, p.as_ptr() as *const i8, (lim.as_ptr().offset_from(p.as_ptr()) as isize) as u64, &mut mbs);
    }
    
    if len > (-(1 as isize) as u64).wrapping_div(2) {
        return mcel_err(c as u8);
    }
    
    return mcel_ch(ch, len as usize);
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_isbasic(c: i8) -> bool {
    (0 <= c && c < MCEL_ERR_MIN as i8)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_tocmp(
    to: Option<fn(wint_t) -> wint_t>,
    c1: mcel_t,
    c2: mcel_t,
) -> libc::c_int {
    let cmp: libc::c_int = mcel_cmp(c1, c2);
    if (c1.err as libc::c_int - c2.err as libc::c_int | (cmp == 0) as libc::c_int) != 0 {
        return cmp;
    }
    let ch1: libc::c_int = to.expect("non-null function pointer")(c1.ch).try_into().unwrap();
    let ch2: libc::c_int = to.expect("non-null function pointer")(c2.ch).try_into().unwrap();
    ch1 - ch2
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_cmp(c1: mcel_t, c2: mcel_t) -> i32 {
    let ch1: i32 = c1.ch as i32;
    let ch2: i32 = c2.ch as i32;
    (c1.err as i32 - c2.err as i32) * (1 << MCEL_ERR_SHIFT as i32) + (ch1 - ch2)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_err(err: u8) -> mcel_t {
    assert!(MCEL_ERR_MIN as i32 <= err as i32, "Error value out of range");
    
    mcel_t {
        ch: 0,
        err,
        len: 1,
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mcel_ch(ch: char32_t, len: usize) -> mcel_t {
    assert!(len > 0, "Length must be greater than 0");
    assert!(len <= MCEL_LEN_MAX as usize, "Length exceeds maximum allowed");
    assert!(ch <= MCEL_CHAR_MAX as u32, "Character exceeds maximum allowed");

    mcel_t {
        ch,
        err: 0,
        len: len as u8,
    }
}

