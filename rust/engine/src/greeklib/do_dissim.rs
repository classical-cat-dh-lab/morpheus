extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type Stemtype = ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn do_dissim(
    mut s: *mut ::core::ffi::c_char,
    mut stype: Stemtype,
) -> ::core::ffi::c_int {
    let mut p1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut p2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if (*s.offset(Xstrlen(s) as isize).offset(-(1 as ::core::ffi::c_int as isize))
        as ::core::ffi::c_int == 'f' as i32
        || *s.offset(Xstrlen(s) as isize).offset(-(1 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_int == 'q' as i32
        || *s.offset(Xstrlen(s) as isize).offset(-(1 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_int == 'x' as i32)
        && stype & VERBSTEM as Stemtype == PP_AP as Stemtype
    {
        return 0 as ::core::ffi::c_int;
    }
    p1 = s;
    while *p1 != 0 {
        if (*p1 as ::core::ffi::c_int == 'f' as i32
            || *p1 as ::core::ffi::c_int == 'q' as i32
            || *p1 as ::core::ffi::c_int == 'x' as i32) && next_cons_rough(p1) != 0
        {
            match *p1 as ::core::ffi::c_int {
                113 => {
                    *p1 = 't' as i32 as ::core::ffi::c_char;
                }
                120 => {
                    *p1 = 'k' as i32 as ::core::ffi::c_char;
                }
                102 => {
                    *p1 = 'p' as i32 as ::core::ffi::c_char;
                }
                _ => {}
            }
            return 1 as ::core::ffi::c_int;
        }
        p1 = p1.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn next_cons_rough(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    cp = next_cons(s);
    rval = (*cp as ::core::ffi::c_int == 'f' as i32
        || *cp as ::core::ffi::c_int == 'q' as i32
        || *cp as ::core::ffi::c_int == 'x' as i32) as ::core::ffi::c_int;
    if rval == 0 {
        if *cp as ::core::ffi::c_int == 's' as i32
            && (*cp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'f' as i32
                || *cp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'q' as i32
                || *cp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'x' as i32)
        {
            rval = 1 as ::core::ffi::c_int;
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn next_cons(
    mut s: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    while (if 0 as ::core::ffi::c_int != 0 {
        isalpha(*s as ::core::ffi::c_int)
    } else {
        ((*s as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    }) != 0 && *s as ::core::ffi::c_int != 'j' as i32
        && *s as ::core::ffi::c_int != 'v' as i32
        && *s as ::core::ffi::c_int != 'J' as i32
        && *s as ::core::ffi::c_int != 'V' as i32
        && !(*s as ::core::ffi::c_int == 'a' as i32
            || *s as ::core::ffi::c_int == 'e' as i32
            || *s as ::core::ffi::c_int == 'i' as i32
            || *s as ::core::ffi::c_int == 'o' as i32
            || *s as ::core::ffi::c_int == 'u' as i32
            || *s as ::core::ffi::c_int == 'A' as i32
            || *s as ::core::ffi::c_int == 'E' as i32
            || *s as ::core::ffi::c_int == 'I' as i32
            || *s as ::core::ffi::c_int == 'O' as i32
            || *s as ::core::ffi::c_int == 'U' as i32
            || (*s as ::core::ffi::c_int == 'h' as i32
                || *s as ::core::ffi::c_int == 'w' as i32
                || *s as ::core::ffi::c_int == 'H' as i32
                || *s as ::core::ffi::c_int == 'W' as i32))
        || (*s as ::core::ffi::c_int == ACUTE || *s as ::core::ffi::c_int == GRAVE
            || *s as ::core::ffi::c_int == CIRCUMFLEX)
    {
        s = s.offset(1);
    }
    while *s as ::core::ffi::c_int == 'a' as i32
        || *s as ::core::ffi::c_int == 'e' as i32
        || *s as ::core::ffi::c_int == 'i' as i32
        || *s as ::core::ffi::c_int == 'o' as i32
        || *s as ::core::ffi::c_int == 'u' as i32
        || *s as ::core::ffi::c_int == 'A' as i32
        || *s as ::core::ffi::c_int == 'E' as i32
        || *s as ::core::ffi::c_int == 'I' as i32
        || *s as ::core::ffi::c_int == 'O' as i32
        || *s as ::core::ffi::c_int == 'U' as i32
        || (*s as ::core::ffi::c_int == 'h' as i32
            || *s as ::core::ffi::c_int == 'w' as i32
            || *s as ::core::ffi::c_int == 'H' as i32
            || *s as ::core::ffi::c_int == 'W' as i32)
        || (*s as ::core::ffi::c_int == ACUTE || *s as ::core::ffi::c_int == GRAVE
            || *s as ::core::ffi::c_int == CIRCUMFLEX)
    {
        s = s.offset(1);
    }
    return s;
}
pub const VERBSTEM: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
pub const PP_AP: ::core::ffi::c_int = 0o60000000 as ::core::ffi::c_int;
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const GRAVE: ::core::ffi::c_int = '\\' as i32;
pub const CIRCUMFLEX: ::core::ffi::c_int = '=' as i32;
