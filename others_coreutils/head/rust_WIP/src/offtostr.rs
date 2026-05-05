
use ::libc;
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
#[no_mangle]
pub fn offtostr(i: off_t, buf: &mut [libc::c_char]) -> &mut [libc::c_char] {
    let mut p = buf.len() as isize - 1;
    buf[p as usize] = 0;

    let mut num = i;
    if num < 0 {
        num = -num;
        loop {
            p -= 1;
            buf[p as usize] = ('0' as i32 - (num % 10) as i32) as libc::c_char;
            num /= 10;
            if num == 0 {
                break;
            }
        }
        p -= 1;
        buf[p as usize] = '-' as i32 as libc::c_char;
    } else {
        loop {
            p -= 1;
            buf[p as usize] = ('0' as i32 + (num % 10) as i32) as libc::c_char;
            num /= 10;
            if num == 0 {
                break;
            }
        }
    }
    &mut buf[p as usize..]
}

