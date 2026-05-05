use std::cmp::Ordering;

use ::libc;
extern "C" {
    fn toupper(_: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub fn memcasecmp(vs1: &[u8], vs2: &[u8]) -> libc::c_int {
    let n = vs1.len().min(vs2.len());
    for i in 0..n {
        let u1 = vs1[i];
        let u2 = vs2[i];
        let U1 = u1.to_ascii_uppercase();
        let U2 = u2.to_ascii_uppercase();
        let diff = U1.cmp(&U2);
        if diff != std::cmp::Ordering::Equal {
            return diff as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}

