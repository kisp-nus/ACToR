use std::mem;

use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
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
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn mbszero(ps: &mut mbstate_t) {
    // Zero out the mbstate_t structure
    let zeroed: mbstate_t = unsafe { std::mem::zeroed() };
    *ps = zeroed;
}

