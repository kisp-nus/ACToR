use std::ptr;

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
            buf[0] = b'\0';
        }
        return 22;
    } else {
        let length: usize;
        unsafe {
            length = strlen(result) as usize;
        }
        
        if length < buf.len() {
            unsafe {
                std::ptr::copy_nonoverlapping(result as *const u8, buf.as_mut_ptr(), length);
            }
            buf[length] = b'\0';
            return 0;
        } else {
            if !buf.is_empty() {
                let copy_length = buf.len() - 1;
                unsafe {
                    std::ptr::copy_nonoverlapping(result as *const u8, buf.as_mut_ptr(), copy_length);
                }
                buf[copy_length] = b'\0';
            }
            return 34;
        }
    }
}

