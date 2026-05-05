
use std::char;

use ::libc;
#[inline]
fn c_tolower(c: i32) -> i32 {
    match c {
        65..=90 => c - 'A' as i32 + 'a' as i32,
        _ => c,
    }
}

#[no_mangle]
pub fn c_strcasecmp(s1: &str, s2: &str) -> libc::c_int {
    let c1: Vec<_> = s1.chars().map(|c| c_tolower(c as i32) as u8 as char).collect();
    let c2: Vec<_> = s2.chars().map(|c| c_tolower(c as i32) as u8 as char).collect();
    
    let len = c1.len().min(c2.len());
    
    for i in 0..len {
        if c1[i] != c2[i] {
            return c1[i] as libc::c_int - c2[i] as libc::c_int;
        }
    }
    
    c1.len() as libc::c_int - c2.len() as libc::c_int
}

