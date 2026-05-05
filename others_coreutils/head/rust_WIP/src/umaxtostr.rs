use std::string::String;

use ::libc;
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
#[no_mangle]
pub fn umaxtostr(i: uintmax_t, buf: &mut [libc::c_char]) -> String {
    let mut p = buf.len() as isize - 1;
    buf[p as usize] = 0;

    if i < 0 {
        let mut num = i;
        loop {
            p -= 1;
            buf[p as usize] = ('0' as i32 as uintmax_t).wrapping_sub(num % 10) as libc::c_char;
            num /= 10;
            if num == 0 {
                break;
            }
        }
        p -= 1;
        buf[p as usize] = '-' as i32 as libc::c_char;
    } else {
        let mut num = i;
        loop {
            p -= 1;
            buf[p as usize] = ('0' as i32 as uintmax_t).wrapping_add(num % 10) as libc::c_char;
            num /= 10;
            if num == 0 {
                break;
            }
        }
    }
    let len = buf.len() - p as usize - 1;
    let slice = &buf[p as usize..p as usize + len];
    String::from_utf8_lossy(&slice.iter().map(|&c| c as u8).collect::<Vec<u8>>()).into_owned()
}

