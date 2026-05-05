

use std::u32;

use std::mem;

use ::libc;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn __gl_stdbit_clz(n: u32) -> i32 {
    if n != 0 {
        n.leading_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u32>() as i32)
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn __gl_stdbit_clzl(n: u64) -> i32 {
    if n != 0 {
        n.leading_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u64>() as u64) as i32
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn __gl_stdbit_clzll(mut n: libc::c_ulonglong) -> libc::c_int {
    return (if n != 0 {
        n.leading_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
    }) as libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn __gl_stdbit_ctz(n: u32) -> i32 {
    if n != 0 {
        n.trailing_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u32>() as i32)
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn __gl_stdbit_ctzl(n: u64) -> i32 {
    if n != 0 {
        n.trailing_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u64>() as u64) as i32
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn __gl_stdbit_ctzll(mut n: libc::c_ulonglong) -> libc::c_int {
    return (if n != 0 {
        n.trailing_zeros() as i32 as libc::c_ulong
    } else {
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
    }) as libc::c_int;
}
