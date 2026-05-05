






use std::usize;

use std::num::Wrapping;

use std::convert::TryInto;

use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn rotr8(x: u8, n: i32) -> u8 {
    let n = n % 8; // Ensure n is within the range of 0-7
    (x >> n) | (x << (8 - n))
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn rotl8(x: u8, n: i32) -> u8 {
    let n = n % 8; // Ensure n is within the range of 0-7
    (x << n | x >> (8 - n)) & 0xFF
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn rotr16(x: u16, n: i32) -> u16 {
    let n = n % 16; // Ensure n is within the range of 0-15
    (x >> n) | (x << (16 - n))
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn rotl16(x: u16, n: i32) -> u16 {
    let n = n % 16; // Ensure n is within the range of 0-15
    (x << n | x >> (16 - n)) & 0xFFFF
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn rotr_sz(x: usize, n: i32) -> usize {
    let size_bits = 8 * std::mem::size_of::<usize>() as u32;
    let n = n as u32 % size_bits; // Ensure n is within the bounds of size_bits
    (x >> n) | (x << (size_bits - n))
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn rotl_sz(x: usize, n: i32) -> usize {
    let bits = (usize::BITS as usize) - n as usize;
    (x << n) | (x >> bits)
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn rotr32(x: u32, n: i32) -> u32 {
    let n = n as u32 % 32; // Ensure n is within the range of 0-31
    (x >> n) | (x << (32 - n))
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn rotl32(x: u32, n: i32) -> u32 {
    let n = n % 32; // Ensure n is within the range of 0-31
    (x << n) | (x >> (32 - n))
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn rotr64(x: u64, n: i32) -> u64 {
    let n = n % 64; // Ensure n is within the range of 0-63
    (x >> n) | (x << (64 - n))
}

#[no_mangle]
#[inline]
#[linkage = "external"]
pub fn rotl64(x: u64, n: i32) -> u64 {
    let n = n as u64 % 64; // Ensure n is within the range of 0-63
    (x << n) | (x >> (64 - n))
}

