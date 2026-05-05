use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn mbrtoc32(
        __pc32: *mut char32_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn abort() -> !;
    fn hard_locale(category: libc::c_int) -> bool;
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
#[inline]
unsafe extern "C" fn mbszero(mut ps: *mut mbstate_t) {
    memset(
        ps as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
}
static mut internal_state: mbstate_t = mbstate_t {
    __count: 0,
    __value: C2RustUnnamed { __wch: 0 },
};
#[no_mangle]
pub unsafe extern "C" fn rpl_mbrtoc32(
    mut pwc: *mut char32_t,
    mut s: *const libc::c_char,
    mut n: size_t,
    mut ps: *mut mbstate_t,
) -> size_t {
    if s.is_null() {
        pwc = 0 as *mut char32_t;
        s = b"\0" as *const u8 as *const libc::c_char;
        n = 1 as libc::c_int as size_t;
    }
    if ps.is_null() {
        ps = &mut internal_state;
    }
    let mut ret: size_t = mbrtoc32(pwc, s, n, ps);
    if ret < -(3 as libc::c_int) as size_t && mbsinit(ps) == 0 {
        mbszero(ps);
    }
    if ret == -(3 as libc::c_int) as size_t {
        abort();
    }
    if -(2 as libc::c_int) as size_t <= ret && n != 0 as libc::c_int as libc::c_ulong
        && !hard_locale(0 as libc::c_int)
    {
        if !pwc.is_null() {
            *pwc = *s as libc::c_uchar as char32_t;
        }
        return 1 as libc::c_int as size_t;
    }
    return ret;
}
