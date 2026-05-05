use ::libc;
extern "C" {
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn locale_charset() -> *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn proper_name_lite(
    mut name_ascii: *const libc::c_char,
    mut name_utf8: *const libc::c_char,
) -> *const libc::c_char {
    let mut translation: *const libc::c_char = gettext(name_ascii);
    return if translation != name_ascii {
        translation
    } else if c_strcasecmp(
        locale_charset(),
        b"UTF-8\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        name_utf8
    } else {
        name_ascii
    };
}
