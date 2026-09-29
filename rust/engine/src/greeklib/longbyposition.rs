extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn longbyposition(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
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
        || *s as ::core::ffi::c_int != 0
            && (if 0 as ::core::ffi::c_int != 0 {
                isalpha(*s as ::core::ffi::c_int)
            } else {
                ((*s as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
            }) == 0
    {
        s = s.offset(1);
    }
    if *s as ::core::ffi::c_int == 'c' as i32 || *s as ::core::ffi::c_int == 'y' as i32
        || *s as ::core::ffi::c_int == 'z' as i32
    {
        return 1 as ::core::ffi::c_int;
    }
    if !((if 0 as ::core::ffi::c_int != 0 {
        isalpha(*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
    } else {
        ((*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    }) != 0
        && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 'j' as i32
        && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 'v' as i32
        && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 'J' as i32
        && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 'V' as i32
        && !(*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'a' as i32
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'e' as i32
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'i' as i32
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'o' as i32
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'u' as i32
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'A' as i32
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'E' as i32
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'I' as i32
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'O' as i32
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'U' as i32
            || (*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'h' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'w' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'H' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'W' as i32)))
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && (*s as ::core::ffi::c_int == 'p' as i32
            || *s as ::core::ffi::c_int == 't' as i32
            || *s as ::core::ffi::c_int == 'k' as i32
            || (*s as ::core::ffi::c_int == 'b' as i32
                || *s as ::core::ffi::c_int == 'd' as i32
                || *s as ::core::ffi::c_int == 'g' as i32)
            || (*s as ::core::ffi::c_int == 'f' as i32
                || *s as ::core::ffi::c_int == 'q' as i32
                || *s as ::core::ffi::c_int == 'x' as i32))) as ::core::ffi::c_int
        == 'p' as i32
        || (*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && (*s as ::core::ffi::c_int == 'p' as i32
                || *s as ::core::ffi::c_int == 't' as i32
                || *s as ::core::ffi::c_int == 'k' as i32
                || (*s as ::core::ffi::c_int == 'b' as i32
                    || *s as ::core::ffi::c_int == 'd' as i32
                    || *s as ::core::ffi::c_int == 'g' as i32)
                || (*s as ::core::ffi::c_int == 'f' as i32
                    || *s as ::core::ffi::c_int == 'q' as i32
                    || *s as ::core::ffi::c_int == 'x' as i32))) as ::core::ffi::c_int
            == 'b' as i32
        || (*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && (*s as ::core::ffi::c_int == 'p' as i32
                || *s as ::core::ffi::c_int == 't' as i32
                || *s as ::core::ffi::c_int == 'k' as i32
                || (*s as ::core::ffi::c_int == 'b' as i32
                    || *s as ::core::ffi::c_int == 'd' as i32
                    || *s as ::core::ffi::c_int == 'g' as i32)
                || (*s as ::core::ffi::c_int == 'f' as i32
                    || *s as ::core::ffi::c_int == 'q' as i32
                    || *s as ::core::ffi::c_int == 'x' as i32))) as ::core::ffi::c_int
            == 'f' as i32
    {
        return 2 as ::core::ffi::c_int;
    }
    if (if 0 as ::core::ffi::c_int != 0 {
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
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
