


use ::libc;
extern "C" {
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn strtoumax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> uintmax_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub const _ISspace: C2RustUnnamed = 8192;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
unsafe extern "C" fn bkm_scale(
    mut x: *mut uintmax_t,
    mut scale_factor: libc::c_int,
) -> strtol_error {
    let mut scaled: uintmax_t = 0;
    if if (0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t
        && (if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_ulong } else { *x })
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < 0 as libc::c_int as libc::c_ulong
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int } else { scale_factor })
            - 1 as libc::c_int) < 0 as libc::c_int
        && (if scale_factor < 0 as libc::c_int {
            if *x < 0 as libc::c_int as libc::c_ulong {
                if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        -(1 as libc::c_int) as uintmax_t
                    })
                        .wrapping_add(scale_factor as libc::c_ulong)
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                {
                    (*x
                        < (-(1 as libc::c_int) as uintmax_t)
                            .wrapping_div(scale_factor as libc::c_ulong)) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        scale_factor
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 1 as libc::c_int)
                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        (scale_factor
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                scale_factor
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    scale_factor
                                }) - 1 as libc::c_int
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int) < scale_factor) as libc::c_int
                    }) != 0
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) as libc::c_ulong)
                            .wrapping_add(-(1 as libc::c_int) as uintmax_t)
                            >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (-(1 as libc::c_int) as uintmax_t)
                            .wrapping_div(-scale_factor as libc::c_ulong)
                    }) <= (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(*x))
                        as libc::c_int
                }
            } else {
                let scale_factor_value = if scale_factor != 0 { scale_factor } else { 0 };
let x_value = *x;

if (if (if (if 1 != 0 {
                    0
                } else {
                    (scale_factor_value as u64).wrapping_add(0)
                })
                    .wrapping_sub(1)
                    < 0 {
                    !((if 1 != 0 {
                        0
                    } else {
                        (scale_factor_value as u64).wrapping_add(0)
                    })
                        .wrapping_add(1)
                        << (std::mem::size_of::<u64>() as u64 * 8 - 2)
                        .wrapping_sub(1)
                        .wrapping_mul(2)
                        .wrapping_add(1)
                    )
                } else {
                    (scale_factor_value as u64).wrapping_add(0)
                }) < 0 {
                    (((if 1 != 0 {
                        0
                    } else {
                        scale_factor_value
                    }) as u64)
                        .wrapping_add(0)
                        < (if (if 1 != 0 {
                            0
                        } else {
                            (scale_factor_value as u64).wrapping_add(0)
                        })
                            .wrapping_sub(1)
                            < 0 {
                            ((if 1 != 0 {
                                0
                            } else {
                                scale_factor_value
                            }) as u64)
                                .wrapping_add(1)
                                << (std::mem::size_of::<u64>() as u64 * 8 - 2)
                                .wrapping_sub(1)
                                .wrapping_mul(2)
                                .wrapping_add(1)
                        } else {
                            (if 1 != 0 {
                                0
                            } else {
                                scale_factor_value
                            }).wrapping_sub(1) as u64
                        })
                            .wrapping_neg()) as i32
                } else {
                    (0 < (scale_factor_value as u64)) as i32
                }) != 0 && scale_factor == -1 {
                    if (if 1 != 0 {
                        0
                    } else {
                        x_value
                    })
                        .wrapping_sub(1)
                        < 0 {
                        (0 < x_value.wrapping_add(0)) as i32
                    } else {
                        (0 < x_value && (-1i32 as u64).wrapping_sub(0) < x_value.wrapping_sub(1)) as i32
                    }
                } else {
                    (0u64.wrapping_div(scale_factor as u64) < x_value) as i32
                }

            }
        } else {
            if scale_factor == 0 as libc::c_int {
                0 as libc::c_int
            } else {
                if *x < 0 as libc::c_int as libc::c_ulong {
                    if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            *x
                        })
                            .wrapping_add(0 as libc::c_int as uintmax_t)
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        < 0 as libc::c_int as libc::c_ulong
                    {
                        !((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                *x
                            })
                                .wrapping_add(0 as libc::c_int as uintmax_t)
                        })
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            << (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                *x
                            })
                                .wrapping_add(0 as libc::c_int as uintmax_t)
                        })
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                    }) < 0 as libc::c_int as libc::c_ulong
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            *x
                        })
                            .wrapping_add(0 as libc::c_int as uintmax_t)
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    *x
                                })
                                    .wrapping_add(0 as libc::c_int as uintmax_t)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        *x
                                    })
                                        .wrapping_add(0 as libc::c_int as uintmax_t)
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::core::mem::size_of::<libc::c_ulong>()
                                        as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        *x
                                    })
                                        .wrapping_add(0 as libc::c_int as uintmax_t)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_ulong)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                *x
                            })
                                .wrapping_add(0 as libc::c_int as uintmax_t)) as libc::c_int
                    }) != 0 && *x == -(1 as libc::c_int) as libc::c_ulong
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            scale_factor
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            ((0 as libc::c_int as libc::c_ulong)
                                < (scale_factor as libc::c_ulong)
                                    .wrapping_add(0 as libc::c_int as uintmax_t)) as libc::c_int
                        } else {
                            ((-(1 as libc::c_int) as libc::c_ulong)
                                .wrapping_sub(0 as libc::c_int as uintmax_t)
                                < (scale_factor - 1 as libc::c_int) as libc::c_ulong)
                                as libc::c_int
                        }
                    } else {
                        ((0 as libc::c_int as uintmax_t).wrapping_div(*x)
                            < scale_factor as libc::c_ulong) as libc::c_int
                    }
                } else {
                    ((-(1 as libc::c_int) as uintmax_t)
                        .wrapping_div(scale_factor as libc::c_ulong) < *x) as libc::c_int
                }
            }
        }) != 0
    {
        let (fresh4, _fresh5) = (*x).overflowing_mul(scale_factor.try_into().unwrap());
        *(&mut scaled as *mut uintmax_t) = fresh4;
        1 as libc::c_int
    } else {
        let (fresh6, fresh7) = (*x).overflowing_mul(scale_factor.try_into().unwrap());
        *(&mut scaled as *mut uintmax_t) = fresh6;
        fresh7 as libc::c_int
    } != 0
    {
        *x = if *x < 0 as libc::c_int as libc::c_ulong {
            !if (0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t {
                -(1 as libc::c_int) as uintmax_t
            } else {
                ((1 as libc::c_int as uintmax_t)
                    << (::core::mem::size_of::<uintmax_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            }
        } else if (0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t {
            -(1 as libc::c_int) as uintmax_t
        } else {
            ((1 as libc::c_int as uintmax_t)
                << (::core::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        };
        return LONGINT_OVERFLOW;
    }
    *x = scaled;
    return LONGINT_OK;
}
fn bkm_scale_by_power(
    x: &mut u64,
    base: i32,
    mut power: i32,
) -> strtol_error {
    let mut err: strtol_error = LONGINT_OK;
    while power > 0 {
        power -= 1;
        // Call to bkm_scale is unsafe, so we need to wrap it in an unsafe block
        err = unsafe {
            (err as u32 | bkm_scale(x, base) as u32) as strtol_error
        };
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn xstrtoumax(
    mut s: *const libc::c_char,
    mut ptr: *mut *mut libc::c_char,
    mut strtol_base: libc::c_int,
    mut val: *mut uintmax_t,
    mut valid_suffixes: *const libc::c_char,
) -> strtol_error {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: uintmax_t = 0;
    let mut err: strtol_error = LONGINT_OK;
    if 0 as libc::c_int <= strtol_base && strtol_base <= 36 as libc::c_int {} else {
        __assert_fail(
            b"0 <= strtol_base && strtol_base <= 36\0" as *const u8
                as *const libc::c_char,
            b"./include/xstrtol.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"strtol_error xstrtoumax(const char *, char **, int, uintmax_t *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2256: {
        if 0 as libc::c_int <= strtol_base && strtol_base <= 36 as libc::c_int {} else {
            __assert_fail(
                b"0 <= strtol_base && strtol_base <= 36\0" as *const u8
                    as *const libc::c_char,
                b"./include/xstrtol.c\0" as *const u8 as *const libc::c_char,
                86 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 79],
                    &[libc::c_char; 79],
                >(
                    b"strtol_error xstrtoumax(const char *, char **, int, uintmax_t *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    p = if !ptr.is_null() { ptr } else { &mut t_ptr };
    *__errno_location() = 0 as libc::c_int;
    if (0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t {
        let mut q: *const libc::c_char = s;
        let mut ch: libc::c_uchar = *q as libc::c_uchar;
        while *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            q = q.offset(1);
            ch = *q as libc::c_uchar;
        }
        if ch as libc::c_int == '-' as i32 {
            return LONGINT_INVALID;
        }
    }
    tmp = strtoumax(s, p, strtol_base);
    if *p == s as *mut libc::c_char {
        if !valid_suffixes.is_null() && **p as libc::c_int != 0
            && !(strchr(valid_suffixes, **p as libc::c_int)).is_null()
        {
            tmp = 1 as libc::c_int as uintmax_t;
        } else {
            return LONGINT_INVALID
        }
    } else if *__errno_location() != 0 as libc::c_int {
        if *__errno_location() != 34 as libc::c_int {
            return LONGINT_INVALID;
        }
        err = LONGINT_OVERFLOW;
    }
    if valid_suffixes.is_null() {
        *val = tmp;
        return err;
    }
    if !(*p).is_null() && **p != 0 {
    let mut base: i32 = 1024;
    let mut suffixes: i32 = 1;
    let mut overflow: strtol_error = LONGINT_OK;

    let valid_suffixes_slice = unsafe { std::ffi::CStr::from_ptr(valid_suffixes).to_string_lossy() };
    let current_char = **p as u8 as char;
    if !valid_suffixes_slice.contains(current_char) {
        *val = tmp;
        return (err | LONGINT_INVALID_SUFFIX_CHAR) as strtol_error;
    }

    match current_char {
        'E' | 'G' | 'g' | 'k' | 'K' | 'M' | 'm' | 'P' | 'Q' | 'R' | 'T' | 't' | 'Y' | 'Z' => {
            if valid_suffixes_slice.contains('0') {
                match unsafe { *(*p).offset(1) } as u8 as char {
                    'i' => {
                        if unsafe { *(*p).offset(2) } as u8 as char == 'B' {
                            suffixes += 2;
                        }
                    }
                    'B' | 'D' => {
                        base = 1000;
                        suffixes += 1;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    match current_char {
        'b' => {
            overflow = bkm_scale(&mut tmp, 512);
        }
        'B' => {
            overflow = bkm_scale(&mut tmp, 1024);
        }
        'c' => {
            overflow = LONGINT_OK;
        }
        'E' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 6);
        }
        'G' | 'g' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 3);
        }
        'k' | 'K' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 1);
        }
        'M' | 'm' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 2);
        }
        'P' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 5);
        }
        'Q' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 10);
        }
        'R' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 9);
        }
        'T' | 't' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 4);
        }
        'w' => {
            overflow = bkm_scale(&mut tmp, 2);
        }
        'Y' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 8);
        }
        'Z' => {
            overflow = bkm_scale_by_power(&mut tmp, base, 7);
        }
        _ => {
            *val = tmp;
            return (err | LONGINT_INVALID_SUFFIX_CHAR) as strtol_error;
        }
    }

    err |= overflow as u32;
    *p = unsafe { (*p).offset(suffixes as isize) };
    if **p != 0 {
        err |= LONGINT_INVALID_SUFFIX_CHAR;
    }
}
*val = tmp;
return err;

}
