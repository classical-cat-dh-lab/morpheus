extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn is_diphth(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> bool_0;
    fn getsyll(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
pub type bool_0 = ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn quantprim(
    mut word: *mut ::core::ffi::c_char,
    mut syll: ::core::ffi::c_int,
    mut is_ending: bool_0,
    mut is_oblique: bool_0,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = getsyll(word, syll);
    if p == P_ERR {
        return -(1 as ::core::ffi::c_int);
    }
    s = p;
    while !((if 0 as ::core::ffi::c_int != 0 {
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
                || *s as ::core::ffi::c_int == 'W' as i32)))
        && *s as ::core::ffi::c_int != 0
    {
        if *s as ::core::ffi::c_int == HARDLONG {
            return 1 as ::core::ffi::c_int
        } else if *s as ::core::ffi::c_int == HARDSHORT {
            return 0 as ::core::ffi::c_int
        }
        s = s.offset(1);
    }
    if long_by_isub(p) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if is_diphth(p, word) != 0 {
        if *p as ::core::ffi::c_int == 'i' as i32
            && (*p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'a' as i32
                || *p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'o' as i32)
        {
            if is_ending == YES && is_oblique == NO
                && (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    || (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ACUTE
                        || *p.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == GRAVE
                        || *p.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == CIRCUMFLEX)
                        && *p.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            {
                return 0 as ::core::ffi::c_int
            } else {
                return 1 as ::core::ffi::c_int
            }
        } else {
            return 1 as ::core::ffi::c_int
        }
    } else if *p as ::core::ffi::c_int == 'h' as i32
        || *p as ::core::ffi::c_int == 'w' as i32
        || *p as ::core::ffi::c_int == 'H' as i32
        || *p as ::core::ffi::c_int == 'W' as i32
    {
        return 1 as ::core::ffi::c_int
    } else if *p as ::core::ffi::c_int == 'a' as i32
        || *p as ::core::ffi::c_int == 'e' as i32
        || *p as ::core::ffi::c_int == 'i' as i32
        || *p as ::core::ffi::c_int == 'o' as i32
        || *p as ::core::ffi::c_int == 'u' as i32
        || *p as ::core::ffi::c_int == 'A' as i32
        || *p as ::core::ffi::c_int == 'E' as i32
        || *p as ::core::ffi::c_int == 'I' as i32
        || *p as ::core::ffi::c_int == 'O' as i32
        || *p as ::core::ffi::c_int == 'U' as i32
    {
        if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == SUBSCR {
            return 1 as ::core::ffi::c_int
        } else {
            return 0 as ::core::ffi::c_int
        }
    } else {
        return -(1 as ::core::ffi::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn long_by_isub(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '|' as i32
        || (*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ROUGHBR
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == SMOOTHBR)
            && *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '|' as i32
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I_ERR: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const P_ERR: *mut ::core::ffi::c_char = -(1 as ::core::ffi::c_int)
    as *mut ::core::ffi::c_char;
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const GRAVE: ::core::ffi::c_int = '\\' as i32;
pub const CIRCUMFLEX: ::core::ffi::c_int = '=' as i32;
pub const LONG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SHORT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SUBSCR: ::core::ffi::c_int = '|' as i32;
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const HARDSHORT: ::core::ffi::c_int = '^' as i32;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
