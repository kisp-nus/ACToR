
use std::option::Option;
use std::slice;
use std::process;

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
fn mbszero(ps: &mut mbstate_t) {
    let size = std::mem::size_of::<mbstate_t>();
    let bytes: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ps as *mut _ as *mut u8, size) };
    bytes.fill(0);
}

static mut internal_state: mbstate_t = mbstate_t {
    __count: 0,
    __value: C2RustUnnamed { __wch: 0 },
};
#[no_mangle]
pub fn rpl_mbrtoc32(
    pwc: &mut Option<char32_t>,
    s: Option<&[u8]>,
    n: usize,
    ps: &mut Option<mbstate_t>,
) -> usize {
    let mut local_s = s.unwrap_or_else(|| &[0]);
    
    let local_ps = ps.get_or_insert_with(|| {
        // Create a new instance of internal_state
        unsafe { internal_state }
    });

    if local_s.is_empty() {
        pwc.take(); // Set pwc to None
        local_s = &[0]; // Set s to a null byte
    }

    let ret: usize;
    unsafe {
        ret = mbrtoc32(
            pwc.as_mut().map(|p| p as *mut char32_t).unwrap_or(std::ptr::null_mut()),
            local_s.as_ptr() as *const libc::c_char,
            n as libc::c_ulong,
            local_ps,
        ) as usize;
    }

    if ret < !(3 as libc::c_int) as usize && unsafe { mbsinit(local_ps) } == 0 {
        unsafe { mbszero(local_ps) };
    }
    if ret == !(3 as libc::c_int) as usize {
        std::process::abort();
    }
    if !(2 as libc::c_int) as usize <= ret && n != 0 && !unsafe { hard_locale(0 as libc::c_int) } {
        if let Some(p) = pwc {
            *p = *local_s.get(0).unwrap_or(&0) as libc::c_uchar as char32_t;
        }
        return 1;
    }
    ret
}

