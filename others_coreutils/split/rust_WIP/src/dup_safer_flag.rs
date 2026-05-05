use std::io;

use ::libc;
extern "C" {
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
}
#[no_mangle]
pub fn dup_safer_flag(fd: i32, flag: i32) -> Result<i32, std::io::Error> {
    let command = if flag & 0o2000000 != 0 {
        1030
    } else {
        0
    };
    let result = unsafe { rpl_fcntl(fd, command, 2 + 1) };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(result)
    }
}

