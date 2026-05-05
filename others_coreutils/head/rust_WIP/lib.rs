#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(linkage)]


extern crate libc;
pub mod src {
pub mod binary_io;
pub mod c32isprint;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod close_stream;
pub mod closeout;
pub mod exitfail;
pub mod fclose;
pub mod fflush;
pub mod fpurge;
pub mod fseeko;
pub mod full_read;
pub mod hard_locale;
pub mod ialloc;
pub mod localcharset;
pub mod mbrtoc32;
pub mod mbszero;
pub mod offtostr;
pub mod progname;
pub mod propername_lite;
pub mod quotearg;
pub mod safe_read;
pub mod setlocale_null;
pub mod setlocale_null_unlocked;
pub mod umaxtostr;
pub mod version;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod xalloc_die;
pub mod xbinary_io;
pub mod xdectoumax;
pub mod xmalloc;
pub mod xstrtoumax;
} // mod src
