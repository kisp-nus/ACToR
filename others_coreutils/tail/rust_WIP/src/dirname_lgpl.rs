use std::ffi::CStr;

use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn dir_len(mut file: *const libc::c_char) -> size_t {
    let mut prefix_length: size_t = 0 as libc::c_int as size_t;
    let mut length: size_t = 0;
    prefix_length = (prefix_length as libc::c_ulong)
        .wrapping_add(
            (if prefix_length != 0 as libc::c_int as libc::c_ulong {
                (0 as libc::c_int != 0
                    && *file.offset(prefix_length as isize) as libc::c_int == '/' as i32)
                    as libc::c_int
            } else if *file.offset(0 as libc::c_int as isize) as libc::c_int
                == '/' as i32
            {
                if 0 as libc::c_int != 0
                    && *file.offset(1 as libc::c_int as isize) as libc::c_int
                        == '/' as i32
                    && !(*file.offset(2 as libc::c_int as isize) as libc::c_int
                        == '/' as i32)
                {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                }
            } else {
                0 as libc::c_int
            }) as libc::c_ulong,
        ) as size_t as size_t;
    length = (last_component(file)).offset_from(file) as libc::c_long as size_t;
    while prefix_length < length {
        if !(*file
            .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32)
        {
            break;
        }
        length = length.wrapping_sub(1);
        length;
    }
    return length;
}
#[no_mangle]
pub fn mdir_name(file: *const libc::c_char) -> Option<String> {
    unsafe {
        let length = dir_len(file) as usize;
        let append_dot = length == 0 || (length == 0 && *file.offset(2) != 0 && *file.offset(2) != b'/' as i8);

        let mut dir = String::with_capacity(length + if append_dot { 1 } else { 0 } + 1);
        dir.push_str(std::ffi::CStr::from_ptr(file).to_str().unwrap());
        
        if append_dot {
            dir.push('.');
        }
        
        dir.push('\0'); // This is just to mimic the C-style string termination, but in Rust, it's not necessary.
        
        Some(dir)
    }
}

