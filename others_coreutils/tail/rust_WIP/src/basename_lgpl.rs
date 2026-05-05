
use std::str;

use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub fn last_component(name: &str) -> &str {
    let mut base = name.trim_start_matches('/');
    let mut last_was_slash = false;

    for (i, c) in base.char_indices() {
        if c == '/' {
            last_was_slash = true;
        } else if last_was_slash {
            base = &base[i..];
            last_was_slash = false;
        }
    }

    base
}

#[no_mangle]
pub fn base_len(name: &str, prefix_len: usize) -> usize {
    let mut len = name.len();
    
    while len > 0 && name.as_bytes()[len - 1] == b'/' {
        len -= 1;
    }
    
    if len == 1 && name.as_bytes()[0] == b'/' && name.as_bytes().get(1) == Some(&b'/') && name.as_bytes().get(2).is_none() {
        return 2;
    }
    
    if len == prefix_len && name.as_bytes().get(prefix_len) == Some(&b'/') {
        return prefix_len + 1;
    }
    
    len
}

