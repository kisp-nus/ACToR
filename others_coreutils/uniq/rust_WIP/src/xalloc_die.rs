use ::libc;
extern "C" {
    fn abort() -> !;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut exit_failure: libc::c_int;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn xalloc_die() {
    if 0 != 0 {
        error(
            exit_failure,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            gettext(b"memory exhausted\0" as *const u8 as *const libc::c_char),
        );
        if exit_failure != 0 as libc::c_int {
            unreachable!();
        } else {};
    } else {
        ({
            let __errstatus: libc::c_int = exit_failure;
            error(
                __errstatus,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                gettext(b"memory exhausted\0" as *const u8 as *const libc::c_char),
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
        ({
            let __errstatus: libc::c_int = exit_failure;
            error(
                __errstatus,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                gettext(b"memory exhausted\0" as *const u8 as *const libc::c_char),
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
    };
    abort();
}
