




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
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn skip_buf_matching<'a>(
    buf: &'a [libc::c_char],
    lim: &'a [libc::c_char],
    predicate: Option<fn(mcel_t) -> bool>,
    ok: bool,
) -> *mut libc::c_char {
    let mut s = buf.as_ptr();
    let lim_ptr = lim.as_ptr();
    let mut g: mcel_t = mcel_t { ch: 0, err: 0, len: 0 };

    while s < lim_ptr && {
        g = unsafe { mcel_scan(s, lim_ptr) };
        predicate.expect("non-null function pointer")(g) == ok
    } {
        unsafe {
            s = s.add(g.len as usize);
        }
    }
    s as *mut libc::c_char
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn skip_str_matching(
    mut str: *const libc::c_char,
    mut predicate: Option::<unsafe extern "C" fn(mcel_t) -> bool>,
    mut ok: bool,
) -> *mut libc::c_char {
    let mut s: *const libc::c_char = str;
    let mut g: mcel_t = mcel_t { ch: 0, err: 0, len: 0 };
    while *s as libc::c_int != 0
        && {
            g = mcel_scanz(s);
            predicate.expect("non-null function pointer")(g) as libc::c_int
                == ok as libc::c_int
        }
    {
        s = s.offset(g.len as libc::c_int as isize);
    }
    return s as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn mcel_scanz(mut p: *const libc::c_char) -> mcel_t {
    return mcel_scant(p, '\0' as i32 as libc::c_char);
}
#[inline]
unsafe extern "C" fn mcel_scant(
    mut p: *const libc::c_char,
    mut terminator: libc::c_char,
) -> mcel_t {
    if mcel_isbasic(*p) {
        return mcel_ch(*p as char32_t, 1usize);
    }
    let mut lim: *const libc::c_char = p.offset(1 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < MCEL_LEN_MAX as libc::c_int - 1 as libc::c_int {
        lim = lim
            .offset(
                (*lim as libc::c_int != terminator as libc::c_int) as libc::c_int
                    as isize,
            );
        i += 1;
        i;
    }
    return mcel_scan(p, lim);
}
#[inline]
fn mcel_scan(p: *const libc::c_char, lim: *const libc::c_char) -> mcel_t {
    let c = unsafe { *p };
    if unsafe { mcel_isbasic(c) } {
        return mcel_ch(c as char32_t, 1);
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
    
    if len > (-(1 as libc::c_int) as size_t).wrapping_div(2 as libc::c_int as libc::c_ulong) {
        return mcel_err(c as libc::c_uchar);
    }
    
    return mcel_ch(ch, len.try_into().unwrap());
}

#[inline]
fn mcel_isbasic(c: i8) -> bool {
    (0 <= c as i32 && (c as i32) < MCEL_ERR_MIN as i32)
}

#[inline]
fn mcel_err(err: u8) -> mcel_t {
    assert!(MCEL_ERR_MIN as i32 <= err as i32);
    
    mcel_t {
        ch: 0,
        err,
        len: 1u8,
    }
}

#[inline]
fn mcel_ch(ch: char32_t, len: usize) -> mcel_t {
    assert!(len > 0);
    assert!(len <= MCEL_LEN_MAX as usize);
    assert!(ch <= MCEL_CHAR_MAX as u32);

    mcel_t {
        ch,
        err: 0,
        len: len as u8,
    }
}

