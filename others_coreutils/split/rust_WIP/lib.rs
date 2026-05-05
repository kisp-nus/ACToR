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
pub mod alignalloc;
pub mod basename_lgpl;
pub mod binary_io;
pub mod c32isprint;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod close_stream;
pub mod closeout;
pub mod dup_safer;
pub mod dup_safer_flag;
pub mod exitfail;
pub mod fadvise;
pub mod fclose;
pub mod fcntl;
pub mod fd_reopen;
pub mod fd_safer;
pub mod fd_safer_flag;
pub mod fflush;
pub mod fpurge;
pub mod fseeko;
pub mod full_write;
pub mod hard_locale;
pub mod ialloc;
pub mod localcharset;
pub mod mbrtoc32;
pub mod mbszero;
pub mod mkstemp_safer;
pub mod open_safer;
pub mod progname;
pub mod propername_lite;
pub mod quotearg;
pub mod safe_write;
pub mod same_inode;
pub mod setlocale_null;
pub mod setlocale_null_unlocked;
pub mod sig2str;
pub mod stdbit;
pub mod stdc_leading_zeros;
pub mod temp_stream;
pub mod tmpdir;
pub mod version;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod xalignalloc;
pub mod xalloc_die;
pub mod xbinary_io;
pub mod xdectoimax;
pub mod xdectoumax;
pub mod xmalloc;
pub mod xstrtoimax;
pub mod xstrtoumax;
} // mod src
