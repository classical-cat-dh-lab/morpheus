extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Xstrncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn Xstrncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strsqz(_: *mut ::core::ffi::c_char, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn fixcontr(
    mut stem: *mut ::core::ffi::c_char,
    mut verb: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if 0 as ::core::ffi::c_int
        == strcmp(
            verb
                .offset(Xstrlen(verb) as isize)
                .offset(-(3 as ::core::ffi::c_int as isize)),
            b"a/w\0" as *const u8 as *const ::core::ffi::c_char,
        )
        || 0 as ::core::ffi::c_int
            == strcmp(
                verb
                    .offset(Xstrlen(verb) as isize)
                    .offset(-(6 as ::core::ffi::c_int as isize)),
                b"a/omai\0" as *const u8 as *const ::core::ffi::c_char,
            )
        || (0 as ::core::ffi::c_int
            == strcmp(
                verb
                    .offset(Xstrlen(verb) as isize)
                    .offset(-(3 as ::core::ffi::c_int as isize)),
                b"e/w\0" as *const u8 as *const ::core::ffi::c_char,
            )
            || 0 as ::core::ffi::c_int
                == strcmp(
                    verb
                        .offset(Xstrlen(verb) as isize)
                        .offset(-(6 as ::core::ffi::c_int as isize)),
                    b"e/omai\0" as *const u8 as *const ::core::ffi::c_char,
                ))
    {
        Xstrncat(
            stem,
            b"h\0" as *const u8 as *const ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    }
    if 0 as ::core::ffi::c_int
        == strcmp(
            verb
                .offset(Xstrlen(verb) as isize)
                .offset(-(3 as ::core::ffi::c_int as isize)),
            b"o/w\0" as *const u8 as *const ::core::ffi::c_char,
        )
        || 0 as ::core::ffi::c_int
            == strcmp(
                verb
                    .offset(Xstrlen(verb) as isize)
                    .offset(-(6 as ::core::ffi::c_int as isize)),
                b"o/omai\0" as *const u8 as *const ::core::ffi::c_char,
            )
    {
        Xstrncat(
            stem,
            b"w\0" as *const u8 as *const ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn makeperf(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = s.offset(Xstrlen(s) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    if *p as ::core::ffi::c_int == 't' as i32 || *p as ::core::ffi::c_int == 'd' as i32
        || *p as ::core::ffi::c_int == 'q' as i32
        || *p as ::core::ffi::c_int == 'z' as i32
    {
        *p = 'k' as i32 as ::core::ffi::c_char;
    } else if (*p as ::core::ffi::c_int == 'p' as i32
        || *p as ::core::ffi::c_int == 'b' as i32
        || *p as ::core::ffi::c_int == 'f' as i32)
        && *p as ::core::ffi::c_int != 'f' as i32
    {
        *p = 'f' as i32 as ::core::ffi::c_char;
    } else if (*p as ::core::ffi::c_int == 'k' as i32
        || *p as ::core::ffi::c_int == 'g' as i32
        || *p as ::core::ffi::c_int == 'x' as i32
        || *p as ::core::ffi::c_int == 'c' as i32)
        && *p as ::core::ffi::c_int != 'x' as i32
    {
        *p = 'x' as i32 as ::core::ffi::c_char;
    } else if !(*p as ::core::ffi::c_int == 'f' as i32
        || *p as ::core::ffi::c_int == 'x' as i32)
    {
        if !(*p as ::core::ffi::c_int == 'l' as i32
            || *p as ::core::ffi::c_int == 'r' as i32
            || (*p as ::core::ffi::c_int == 'm' as i32
                || *p as ::core::ffi::c_int == 'n' as i32))
        {
            conjoin(
                s,
                b"k\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn fixperf(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *s.offset(Xstrlen(s) as isize).offset(-(1 as ::core::ffi::c_int as isize))
        as ::core::ffi::c_int != 'k' as i32
    {
        return 0;
    }
    p = s.offset(Xstrlen(s) as isize).offset(-(2 as ::core::ffi::c_int as isize));
    if *p as ::core::ffi::c_int == 't' as i32 || *p as ::core::ffi::c_int == 'd' as i32
        || *p as ::core::ffi::c_int == 'q' as i32
        || *p as ::core::ffi::c_int == 'z' as i32
    {
        *p = 'k' as i32 as ::core::ffi::c_char;
        *p.offset(1 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    } else if (*p as ::core::ffi::c_int == 'p' as i32
        || *p as ::core::ffi::c_int == 'b' as i32
        || *p as ::core::ffi::c_int == 'f' as i32)
        && *p as ::core::ffi::c_int != 'f' as i32
    {
        *p = 'f' as i32 as ::core::ffi::c_char;
        *p.offset(1 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    } else if (*p as ::core::ffi::c_int == 'k' as i32
        || *p as ::core::ffi::c_int == 'g' as i32
        || *p as ::core::ffi::c_int == 'x' as i32
        || *p as ::core::ffi::c_int == 'c' as i32)
        && *p as ::core::ffi::c_int != 'x' as i32
    {
        *p = 'x' as i32 as ::core::ffi::c_char;
        *p.offset(1 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    } else if *p as ::core::ffi::c_int == 'f' as i32
        || *p as ::core::ffi::c_int == 'x' as i32
    {
        *p.offset(1 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    } else if *p as ::core::ffi::c_int == 'm' as i32
        || *p as ::core::ffi::c_int == 'n' as i32
    {
        *p.offset(1 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn conjstem(
    mut stem: *mut ::core::ffi::c_char,
    mut e: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ending: [::core::ffi::c_char; 60] = [0; 60];
    p = stem.offset(Xstrlen(stem) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    Xstrncpy(&raw mut ending as *mut ::core::ffi::c_char, e, MAXWORDSIZE as size_t);
    if (if 0 as ::core::ffi::c_int != 0 {
        isalpha(*p as ::core::ffi::c_int)
    } else {
        ((*p as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    }) != 0 && *p as ::core::ffi::c_int != 'j' as i32
        && *p as ::core::ffi::c_int != 'v' as i32
        && *p as ::core::ffi::c_int != 'J' as i32
        && *p as ::core::ffi::c_int != 'V' as i32
        && !(*p as ::core::ffi::c_int == 'a' as i32
            || *p as ::core::ffi::c_int == 'e' as i32
            || *p as ::core::ffi::c_int == 'i' as i32
            || *p as ::core::ffi::c_int == 'o' as i32
            || *p as ::core::ffi::c_int == 'u' as i32
            || *p as ::core::ffi::c_int == 'A' as i32
            || *p as ::core::ffi::c_int == 'E' as i32
            || *p as ::core::ffi::c_int == 'I' as i32
            || *p as ::core::ffi::c_int == 'O' as i32
            || *p as ::core::ffi::c_int == 'U' as i32
            || (*p as ::core::ffi::c_int == 'h' as i32
                || *p as ::core::ffi::c_int == 'w' as i32
                || *p as ::core::ffi::c_int == 'H' as i32
                || *p as ::core::ffi::c_int == 'W' as i32))
    {
        if 0 as ::core::ffi::c_int
            == strcmp(
                &raw mut ending as *mut ::core::ffi::c_char,
                b"ntai\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            Xstrncpy(
                &raw mut ending as *mut ::core::ffi::c_char,
                b"me/noi ei)si/{n}\0" as *const u8 as *const ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
        }
        if 0 as ::core::ffi::c_int
            == strcmp(
                &raw mut ending as *mut ::core::ffi::c_char,
                b"nto\0" as *const u8 as *const ::core::ffi::c_char,
            )
        {
            Xstrncpy(
                &raw mut ending as *mut ::core::ffi::c_char,
                b"me/noi h)=san\0" as *const u8 as *const ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            if 0 as ::core::ffi::c_int
                == Xstrncmp(
                    stem,
                    b"e)\0" as *const u8 as *const ::core::ffi::c_char,
                    2 as size_t,
                )
            {
                strsqz(stem, 2 as ::core::ffi::c_int);
                p = p.offset(-(2 as ::core::ffi::c_int as isize));
            }
        }
    }
    if 0 as ::core::ffi::c_int
        == strcmp(
            &raw mut ending as *mut ::core::ffi::c_char,
            b"hqi\0" as *const u8 as *const ::core::ffi::c_char,
        ) && *p as ::core::ffi::c_int == 'q' as i32
    {
        ending[1 as ::core::ffi::c_int as usize] = 't' as i32 as ::core::ffi::c_char;
    }
    conjoin(stem, &raw mut ending as *mut ::core::ffi::c_char);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn conjoin(
    mut stem: *mut ::core::ffi::c_char,
    mut e: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ending: [::core::ffi::c_char; 60] = [0; 60];
    let mut changed: ::core::ffi::c_int = 0;
    p = stem.offset(Xstrlen(stem) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    Xstrncpy(&raw mut ending as *mut ::core::ffi::c_char, e, MAXWORDSIZE as size_t);
    if (*p as ::core::ffi::c_int == 't' as i32 || *p as ::core::ffi::c_int == 'd' as i32
        || *p as ::core::ffi::c_int == 'q' as i32
        || *p as ::core::ffi::c_int == 'z' as i32
        || *p as ::core::ffi::c_int == 'n' as i32)
        && Xstrncmp(e, b"ss\0" as *const u8 as *const ::core::ffi::c_char, 2 as size_t)
            == 0
    {
        Xstrncpy(
            p,
            e,
            (MAXWORDSIZE
                - p.offset_from(stem) as ::core::ffi::c_long as ::core::ffi::c_int)
                as size_t,
        );
        return 0;
    }
    loop {
        changed = NO;
        match ending[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int {
            115 => {
                changed = do_sigma(p, &raw mut ending as *mut ::core::ffi::c_char);
            }
            113 => {
                changed = do_theta(p);
            }
            109 => {
                changed = do_mu(p);
            }
            116 => {
                changed = do_tau(p);
            }
            _ => {}
        }
        if !(changed != 0) {
            break;
        }
    }
    Xstrncat(stem, &raw mut ending as *mut ::core::ffi::c_char, MAXWORDSIZE as size_t);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn do_sigma(
    mut s: *mut ::core::ffi::c_char,
    mut ending: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut changed: ::core::ffi::c_int = 0;
    p = s.offset(Xstrlen(s) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    changed = YES;
    if *p as ::core::ffi::c_int == 'q' as i32
        && strncmp(
            ending,
            b"sk\0" as *const u8 as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0
    {
        *p = 's' as i32 as ::core::ffi::c_char;
        *ending.offset(1 as ::core::ffi::c_int as isize) = 'x' as i32
            as ::core::ffi::c_char;
    }
    if (if 0 as ::core::ffi::c_int != 0 {
        isalpha(*p as ::core::ffi::c_int)
    } else {
        ((*p as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    }) != 0 && *p as ::core::ffi::c_int != 'j' as i32
        && *p as ::core::ffi::c_int != 'v' as i32
        && *p as ::core::ffi::c_int != 'J' as i32
        && *p as ::core::ffi::c_int != 'V' as i32
        && !(*p as ::core::ffi::c_int == 'a' as i32
            || *p as ::core::ffi::c_int == 'e' as i32
            || *p as ::core::ffi::c_int == 'i' as i32
            || *p as ::core::ffi::c_int == 'o' as i32
            || *p as ::core::ffi::c_int == 'u' as i32
            || *p as ::core::ffi::c_int == 'A' as i32
            || *p as ::core::ffi::c_int == 'E' as i32
            || *p as ::core::ffi::c_int == 'I' as i32
            || *p as ::core::ffi::c_int == 'O' as i32
            || *p as ::core::ffi::c_int == 'U' as i32
            || (*p as ::core::ffi::c_int == 'h' as i32
                || *p as ::core::ffi::c_int == 'w' as i32
                || *p as ::core::ffi::c_int == 'H' as i32
                || *p as ::core::ffi::c_int == 'W' as i32))
        && ((if 0 as ::core::ffi::c_int != 0 {
            isalpha(
                *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            )
        } else {
            ((*ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) != 0
            && *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'j' as i32
            && *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'v' as i32
            && *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'J' as i32
            && *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'V' as i32
            && !(*ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'a' as i32
                || *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'e' as i32
                || *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'i' as i32
                || *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'o' as i32
                || *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'u' as i32
                || *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'A' as i32
                || *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'E' as i32
                || *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'I' as i32
                || *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'O' as i32
                || *ending.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'U' as i32
                || (*ending.offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'h' as i32
                    || *ending.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'w' as i32
                    || *ending.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'H' as i32
                    || *ending.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'W' as i32)))
    {
        strsqz(ending, 1 as ::core::ffi::c_int);
    } else if *p as ::core::ffi::c_int == 'p' as i32
        || *p as ::core::ffi::c_int == 'b' as i32
        || *p as ::core::ffi::c_int == 'f' as i32
    {
        *p = 'y' as i32 as ::core::ffi::c_char;
        strsqz(ending, 1 as ::core::ffi::c_int);
    } else if *p as ::core::ffi::c_int == 'k' as i32
        || *p as ::core::ffi::c_int == 'g' as i32
        || *p as ::core::ffi::c_int == 'x' as i32
        || *p as ::core::ffi::c_int == 'c' as i32
    {
        *p = 'c' as i32 as ::core::ffi::c_char;
        strsqz(ending, 1 as ::core::ffi::c_int);
    } else if *p as ::core::ffi::c_int == 't' as i32
        || *p as ::core::ffi::c_int == 'd' as i32
        || *p as ::core::ffi::c_int == 'q' as i32
        || (*p as ::core::ffi::c_int == 'm' as i32
            || *p as ::core::ffi::c_int == 'n' as i32)
        || *p as ::core::ffi::c_int == 'z' as i32
    {
        *p = 0 as ::core::ffi::c_char;
    } else if *p as ::core::ffi::c_int == 's' as i32
        && *ending as ::core::ffi::c_int == 's' as i32
    {
        *p = 0 as ::core::ffi::c_char;
    } else {
        changed = NO;
    }
    return changed;
}
#[no_mangle]
pub unsafe extern "C" fn do_theta(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut changed: ::core::ffi::c_int = 0;
    changed = YES;
    p = s.offset(Xstrlen(s) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    if (*p as ::core::ffi::c_int == 'p' as i32 || *p as ::core::ffi::c_int == 'b' as i32
        || *p as ::core::ffi::c_int == 'f' as i32)
        && *p as ::core::ffi::c_int != 'f' as i32
    {
        *p = 'f' as i32 as ::core::ffi::c_char;
    } else if (*p as ::core::ffi::c_int == 'k' as i32
        || *p as ::core::ffi::c_int == 'g' as i32
        || *p as ::core::ffi::c_int == 'x' as i32
        || *p as ::core::ffi::c_int == 'c' as i32)
        && *p as ::core::ffi::c_int != 'x' as i32
    {
        *p = 'x' as i32 as ::core::ffi::c_char;
    } else if *p as ::core::ffi::c_int == 't' as i32
        || *p as ::core::ffi::c_int == 'd' as i32
        || *p as ::core::ffi::c_int == 'q' as i32
        || *p as ::core::ffi::c_int == 'z' as i32
    {
        *p = 's' as i32 as ::core::ffi::c_char;
    } else {
        changed = NO;
    }
    return changed;
}
#[no_mangle]
pub unsafe extern "C" fn do_mu(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut changed: ::core::ffi::c_int = 0;
    changed = YES;
    p = s.offset(Xstrlen(s) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    if *p as ::core::ffi::c_int == 'p' as i32 || *p as ::core::ffi::c_int == 'b' as i32
        || *p as ::core::ffi::c_int == 'f' as i32
    {
        if *p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'm' as i32
        {
            *p = 0 as ::core::ffi::c_char;
        } else {
            *p = 'm' as i32 as ::core::ffi::c_char;
        }
    } else if (*p as ::core::ffi::c_int == 'k' as i32
        || *p as ::core::ffi::c_int == 'g' as i32
        || *p as ::core::ffi::c_int == 'x' as i32
        || *p as ::core::ffi::c_int == 'c' as i32)
        && *p as ::core::ffi::c_int != 'g' as i32
    {
        *p = 'g' as i32 as ::core::ffi::c_char;
    } else if *p as ::core::ffi::c_int == 't' as i32
        || *p as ::core::ffi::c_int == 'd' as i32
        || *p as ::core::ffi::c_int == 'q' as i32
        || *p as ::core::ffi::c_int == 'z' as i32
    {
        *p = 's' as i32 as ::core::ffi::c_char;
    } else {
        changed = NO;
    }
    return changed;
}
#[no_mangle]
pub unsafe extern "C" fn do_tau(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut changed: ::core::ffi::c_int = 0;
    changed = YES;
    p = s.offset(Xstrlen(s) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    if (*p as ::core::ffi::c_int == 'p' as i32 || *p as ::core::ffi::c_int == 'b' as i32
        || *p as ::core::ffi::c_int == 'f' as i32)
        && *p as ::core::ffi::c_int != 'p' as i32
    {
        *p = 'p' as i32 as ::core::ffi::c_char;
    } else if (*p as ::core::ffi::c_int == 'k' as i32
        || *p as ::core::ffi::c_int == 'g' as i32
        || *p as ::core::ffi::c_int == 'x' as i32
        || *p as ::core::ffi::c_int == 'c' as i32)
        && *p as ::core::ffi::c_int != 'k' as i32
    {
        *p = 'k' as i32 as ::core::ffi::c_char;
    } else if *p as ::core::ffi::c_int == 't' as i32
        || *p as ::core::ffi::c_int == 'd' as i32
        || *p as ::core::ffi::c_int == 'q' as i32
        || *p as ::core::ffi::c_int == 'z' as i32
    {
        *p = 's' as i32 as ::core::ffi::c_char;
    } else {
        changed = NO;
    }
    return changed;
}
