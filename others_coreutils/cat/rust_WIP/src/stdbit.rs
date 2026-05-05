


use std::u32;

use std::u64;

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
pub fn __gl_stdbit_clzll(n: u64) -> i32 {
    if n != 0 {
        n.leading_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u64>() as u64) as i32
    }
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
pub fn __gl_stdbit_ctzll(n: u64) -> i32 {
    if n != 0 {
        n.trailing_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u64>() as u32) as i32
    }
}

