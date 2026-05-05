
use std::ffi::{CString, CStr};

use ::libc;
extern "C" {
    pub type __locale_data;
    fn newlocale(
        __category_mask: libc::c_int,
        __locale: *const libc::c_char,
        __base: locale_t,
    ) -> locale_t;
    fn strtod_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __loc: locale_t,
    ) -> libc::c_double;
}
pub type locale_t = __locale_t;
pub type __locale_t = *mut __locale_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const libc::c_ushort,
    pub __ctype_tolower: *const libc::c_int,
    pub __ctype_toupper: *const libc::c_int,
    pub __names: [*const libc::c_char; 13],
}
static mut c_locale_cache: locale_t = 0 as *const __locale_struct
    as *mut __locale_struct;
fn c_locale() -> locale_t {
    use std::sync::{Once, Mutex};

    static INIT: Once = Once::new();
    static mut c_locale_cache: Option<locale_t> = None;
    static CACHE_LOCK: Mutex<()> = Mutex::new(());

    INIT.call_once(|| {
        let _lock = CACHE_LOCK.lock().unwrap();
        unsafe {
            c_locale_cache = Some(newlocale(
                (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10) | (1 << 11),
                b"C\0".as_ptr() as *const libc::c_char,
                0 as locale_t,
            ));
        }
    });

    let _lock = CACHE_LOCK.lock().unwrap();
    unsafe { c_locale_cache.unwrap() }
}

#[no_mangle]
pub fn c_strtod<'a>(
    nptr: &'a str,
    endptr: &mut Option<&'a str>,
) -> f64 {
    let locale = c_locale();
    if locale.is_null() {
        *endptr = Some(nptr);
        return 0.0;
    }
    
    let c_str = std::ffi::CString::new(nptr).unwrap();
    let mut end: *mut libc::c_char = std::ptr::null_mut();
    let r = unsafe { strtod_l(c_str.as_ptr(), &mut end, locale) };
    
    if endptr.is_some() {
        *endptr = unsafe { Some(std::ffi::CStr::from_ptr(end).to_str().unwrap()) };
    }
    
    r
}

