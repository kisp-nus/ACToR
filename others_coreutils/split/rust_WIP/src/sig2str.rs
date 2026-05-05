use std::ffi::CString;

use ::libc;
extern "C" {
    fn __libc_current_sigrtmin() -> libc::c_int;
    fn __libc_current_sigrtmax() -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct numname {
    pub num: libc::c_int,
    pub name: [libc::c_char; 8],
}
static mut numname_table: [numname; 35] = unsafe {
    [
        {
            let mut init = numname {
                num: 1 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"HUP\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 2 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"INT\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 3 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"QUIT\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 4 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"ILL\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 5 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"TRAP\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 6 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"ABRT\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 8 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"FPE\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 9 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"KILL\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 11 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"SEGV\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 7 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"BUS\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 13 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"PIPE\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 14 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"ALRM\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 15 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"TERM\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 10 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"USR1\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 12 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"USR2\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 17 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"CHLD\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 23 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"URG\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 19 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"STOP\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 20 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"TSTP\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 18 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"CONT\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 21 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"TTIN\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 22 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"TTOU\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 31 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"SYS\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 29 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"POLL\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 26 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"VTALRM\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 27 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"PROF\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 24 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"XCPU\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 25 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"XFSZ\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 6 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"IOT\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 17 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"CLD\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 30 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"PWR\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 28 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"WINCH\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 29 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"IO\0\0\0\0\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 16 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"STKFLT\0\0"),
            };
            init
        },
        {
            let mut init = numname {
                num: 0 as libc::c_int,
                name: *::core::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_char; 8],
                >(b"EXIT\0\0\0\0"),
            };
            init
        },
    ]
};
unsafe extern "C" fn str2signum(mut signame: *const libc::c_char) -> libc::c_int {
    if (*signame as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut n: libc::c_long = strtol(signame, &mut endp, 10 as libc::c_int);
        if *endp == 0
            && n
                <= (64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int)
                    as libc::c_long
        {
            return n as libc::c_int;
        }
    } else {
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::core::mem::size_of::<[numname; 35]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<numname>() as libc::c_ulong)
        {
            if strcmp((numname_table[i as usize].name).as_ptr(), signame)
                == 0 as libc::c_int
            {
                return numname_table[i as usize].num;
            }
            i = i.wrapping_add(1);
            i;
        }
        let mut endp_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut rtmin: libc::c_int = __libc_current_sigrtmin();
        let mut rtmax: libc::c_int = __libc_current_sigrtmax();
        if (0 as libc::c_int) < rtmin
            && strncmp(
                signame,
                b"RTMIN\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            let mut n_0: libc::c_long = strtol(
                signame.offset(5 as libc::c_int as isize),
                &mut endp_0,
                10 as libc::c_int,
            );
            if *endp_0 == 0 && 0 as libc::c_int as libc::c_long <= n_0
                && n_0 <= (rtmax - rtmin) as libc::c_long
            {
                return (rtmin as libc::c_long + n_0) as libc::c_int;
            }
        } else if (0 as libc::c_int) < rtmax
            && strncmp(
                signame,
                b"RTMAX\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            let mut n_1: libc::c_long = strtol(
                signame.offset(5 as libc::c_int as isize),
                &mut endp_0,
                10 as libc::c_int,
            );
            if *endp_0 == 0 && (rtmin - rtmax) as libc::c_long <= n_1
                && n_1 <= 0 as libc::c_int as libc::c_long
            {
                return (rtmax as libc::c_long + n_1) as libc::c_int;
            }
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub fn str2sig(signame: &str, signum: &mut i32) -> i32 {
    let c_signame = std::ffi::CString::new(signame).unwrap();
    unsafe {
        *signum = str2signum(c_signame.as_ptr());
    }
    if *signum < 0 {
        -1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn sig2str(
    mut signum: libc::c_int,
    mut signame: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[numname; 35]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<numname>() as libc::c_ulong)
    {
        if numname_table[i as usize].num == signum {
            strcpy(signame, (numname_table[i as usize].name).as_ptr());
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut rtmin: libc::c_int = __libc_current_sigrtmin();
    let mut rtmax: libc::c_int = __libc_current_sigrtmax();
    let mut base: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    if !(rtmin <= signum && signum <= rtmax) {
        return -(1 as libc::c_int);
    }
    if signum <= rtmin + (rtmax - rtmin) / 2 as libc::c_int {
        strcpy(signame, b"RTMIN\0" as *const u8 as *const libc::c_char);
        base = rtmin;
    } else {
        strcpy(signame, b"RTMAX\0" as *const u8 as *const libc::c_char);
        base = rtmax;
    }
    delta = signum - base;
    if delta != 0 as libc::c_int {
        sprintf(
            signame.offset(5 as libc::c_int as isize),
            b"%+d\0" as *const u8 as *const libc::c_char,
            delta,
        );
    }
    return 0 as libc::c_int;
}
