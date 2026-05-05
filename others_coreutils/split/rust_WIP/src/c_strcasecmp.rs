
use ::libc;
#[inline]
unsafe extern "C" fn c_tolower(mut c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
#[no_mangle]
pub fn c_strcasecmp(s1: &str, s2: &str) -> libc::c_int {
    let mut iter1 = s1.chars().map(|c| c.to_ascii_lowercase());
    let mut iter2 = s2.chars().map(|c| c.to_ascii_lowercase());

    loop {
        let c1 = iter1.next().unwrap_or('\0');
        let c2 = iter2.next().unwrap_or('\0');

        if c1 == '\0' {
            return 0;
        }

        if c1 != c2 {
            return c1 as libc::c_int - c2 as libc::c_int;
        }
    }
}

