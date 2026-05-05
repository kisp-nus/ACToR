use ::libc;
#[no_mangle]
pub static mut exit_failure: libc::c_int = 1 as libc::c_int;
