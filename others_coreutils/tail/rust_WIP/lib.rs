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
pub mod argmatch;
pub mod basename_lgpl;
pub mod binary_io;
pub mod bitrotate;
pub mod c32isprint;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod c_strtod;
pub mod cl_strtod;
pub mod close_stream;
pub mod closeout;
pub mod dirname_lgpl;
pub mod dtotimespec;
pub mod dup_safer;
pub mod exitfail;
pub mod fclose;
pub mod fcntl;
pub mod fd_safer;
pub mod fflush;
pub mod fpurge;
pub mod fseeko;
pub mod hard_locale;
pub mod hash;
pub mod ialloc;
pub mod iopoll;
pub mod isapipe;
pub mod localcharset;
pub mod mbrtoc32;
pub mod mbszero;
pub mod nanosleep;
pub mod offtostr;
pub mod open_safer;
pub mod posixver;
pub mod progname;
pub mod propername_lite;
pub mod quotearg;
pub mod safe_read;
pub mod setlocale_null;
pub mod setlocale_null_unlocked;
pub mod stat_time;
pub mod timespec;
pub mod version;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod xalloc_die;
pub mod xbinary_io;
pub mod xdectoumax;
pub mod xmalloc;
pub mod xnanosleep;
pub mod xstrtod;
pub mod xstrtoumax;
} // mod src
