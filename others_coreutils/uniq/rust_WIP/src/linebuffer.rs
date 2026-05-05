
use std::os::raw::c_char;

use std::ptr;

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ptrdiff_t = libc::c_long;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linebuffer {
    pub size: idx_t,
    pub length: idx_t,
    pub buffer: *mut libc::c_char,
}
#[no_mangle]
pub fn initbuffer(linebuffer: &mut linebuffer) {
    linebuffer.buffer = ptr::null_mut(); // Assuming buffer is a mutable pointer
    linebuffer.length = 0;
    linebuffer.size = 0;
}

#[no_mangle]
pub fn readlinebuffer<'a>(
    linebuffer: &'a mut linebuffer,
    stream: &'a mut FILE,
) -> &'a mut linebuffer {
    let result = unsafe {
        readlinebuffer_delim(linebuffer, stream, b'\n' as i32 as libc::c_char)
    };
    unsafe { &mut *result }
}

#[no_mangle]
pub unsafe extern "C" fn readlinebuffer_delim(
    mut linebuffer: *mut linebuffer,
    mut stream: *mut FILE,
    mut delimiter: libc::c_char,
) -> *mut linebuffer {
    let mut c: libc::c_int = 0;
    let mut buffer: *mut libc::c_char = (*linebuffer).buffer;
    let mut p: *mut libc::c_char = (*linebuffer).buffer;
    let mut end: *mut libc::c_char = buffer.offset((*linebuffer).size as isize);
    if feof_unlocked(stream) != 0 {
        return 0 as *mut linebuffer;
    }
    loop {
        c = getc_unlocked(stream);
        if c == -(1 as libc::c_int) {
            if p == buffer || ferror_unlocked(stream) != 0 {
                return 0 as *mut linebuffer;
            }
            if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == delimiter as libc::c_int
            {
                break;
            }
            c = delimiter as libc::c_int;
        }
        if p == end {
            let mut oldsize: idx_t = (*linebuffer).size;
            buffer = xpalloc(
                buffer as *mut libc::c_void,
                &mut (*linebuffer).size,
                1 as libc::c_int as idx_t,
                -(1 as libc::c_int) as ptrdiff_t,
                1 as libc::c_int as idx_t,
            ) as *mut libc::c_char;
            p = buffer.offset(oldsize as isize);
            (*linebuffer).buffer = buffer;
            end = buffer.offset((*linebuffer).size as isize);
        }
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = c as libc::c_char;
        if !(c != delimiter as libc::c_int) {
            break;
        }
    }
    (*linebuffer).length = p.offset_from(buffer) as libc::c_long;
    return linebuffer;
}
#[no_mangle]
pub fn freebuffer(linebuffer: &mut linebuffer) {
    if !linebuffer.buffer.is_null() {
        unsafe {
            let _ = Box::from_raw(linebuffer.buffer); // Convert the raw pointer back to a Box to free the memory
        }
    }
}

