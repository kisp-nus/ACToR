
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
    let c1 = s1.chars().map(|c| c_tolower(c as i32));
    let c2 = s2.chars().map(|c| c_tolower(c as i32));
    
    for (ch1, ch2) in c1.zip(c2) {
        if ch1 != ch2 {
            return ch1 - ch2;
        }
    }
    
    // If one string is a prefix of the other, we need to compare their lengths
    if s1.len() != s2.len() {
        return s1.len() as libc::c_int - s2.len() as libc::c_int;
    }
    
    0
}

