use std::slice;

use ::libc;
extern "C" {
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn setlocale_null_unlocked(
    mut category: libc::c_int,
) -> *const libc::c_char {
    let mut result: *const libc::c_char = setlocale(category, 0 as *const libc::c_char);
    return result;
}
#[no_mangle]
pub fn setlocale_null_r_unlocked(
    category: libc::c_int,
    buf: &mut Vec<u8>,
) -> libc::c_int {
    let result: *const libc::c_char;
    unsafe {
        result = setlocale_null_unlocked(category);
    }

    if result.is_null() {
        if !buf.is_empty() {
            buf[0] = 0; // Null-terminate the buffer
        }
        return 22; // EINVAL
    } else {
        let length: usize = unsafe { strlen(result) } as usize;
        if length < buf.len() {
            buf[..length].copy_from_slice(unsafe { std::slice::from_raw_parts(result as *const u8, length) });
            buf[length] = 0; // Null-terminate the buffer
            return 0; // Success
        } else {
            if !buf.is_empty() {
                let copy_length = buf.len() - 1;
                buf.copy_from_slice(unsafe { std::slice::from_raw_parts(result as *const u8, copy_length) });
                buf[copy_length] = 0; // Null-terminate the buffer
            }
            return 34; // ERANGE
        }
    }
}

