




use std::u32;

use std::mem;

use std::convert::TryInto;

use ::libc;
#[inline]
fn __gl_stdbit_clz(n: u32) -> i32 {
    if n != 0 {
        n.leading_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u32>() as i32)
    }
}

#[inline]
fn __gl_stdbit_clzl(n: u64) -> i32 {
    if n != 0 {
        n.leading_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u64>() as u32) as i32
    }
}

#[inline]
fn __gl_stdbit_clzll(n: u64) -> i32 {
    if n != 0 {
        n.leading_zeros() as i32
    } else {
        (8 * std::mem::size_of::<u64>() as u32) as i32
    }
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn stdc_leading_zeros_ui(n: u32) -> u32 {
    let leading_zeros = n.leading_zeros();
    leading_zeros
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn stdc_leading_zeros_uc(n: u8) -> u32 {
    let leading_zeros = stdc_leading_zeros_ui(n as u32);
    let size_difference = (std::mem::size_of::<u32>() as u32).wrapping_sub(std::mem::size_of::<u8>() as u32);
    (leading_zeros as u64).wrapping_sub((8u32 as u64).wrapping_mul(size_difference as u64)) as u32
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn stdc_leading_zeros_us(n: u16) -> u32 {
    let leading_zeros = stdc_leading_zeros_ui(n as u32);
    let size_diff = (std::mem::size_of::<u32>() - std::mem::size_of::<u16>()) * 8;
    leading_zeros.wrapping_sub(size_diff as u32)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn stdc_leading_zeros_ul(n: u64) -> u32 {
    return __gl_stdbit_clzl(n) as u32;
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn stdc_leading_zeros_ull(n: u64) -> u32 {
    n.leading_zeros()
}

