extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn zap_extra_breath(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut breath_is_extra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *s as ::core::ffi::c_int == 'r' as i32
        && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ROUGHBR
    {
        s = s.offset(2 as ::core::ffi::c_int as isize);
    }
    while *s != 0 {
        if (if 0 as ::core::ffi::c_int != 0 {
            isalpha(*s as ::core::ffi::c_int)
        } else {
            ((*s as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
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
            breath_is_extra += 1;
            s = s.offset(1);
        } else {
            if *s as ::core::ffi::c_int == ROUGHBR
                || *s as ::core::ffi::c_int == SMOOTHBR
            {
                if breath_is_extra == 0 {
                    breath_is_extra += 1;
                    s = s.offset(1);
                    continue;
                } else {
                    strcpy(s, s.offset(1 as ::core::ffi::c_int as isize));
                }
            }
            s = s.offset(1);
        }
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn has_extra_breath(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut breath_is_extra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *s != 0 {
        if (if 0 as ::core::ffi::c_int != 0 {
            isalpha(*s as ::core::ffi::c_int)
        } else {
            ((*s as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
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
            breath_is_extra += 1;
            s = s.offset(1);
        } else if *s as ::core::ffi::c_int == ROUGHBR
            || *s as ::core::ffi::c_int == SMOOTHBR
        {
            if breath_is_extra == 0 {
                breath_is_extra += 1;
                s = s.offset(1);
            } else {
                return 1 as ::core::ffi::c_int
            }
        } else {
            s = s.offset(1);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zap_rr_breath(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    s = s.offset(1);
    while *s != 0 {
        if *s as ::core::ffi::c_int == 'r' as i32
            && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ')' as i32
            && *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'r' as i32
            && *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '(' as i32
        {
            strcpy(s, b"rr\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(s, s.offset(4 as ::core::ffi::c_int as isize));
        }
        s = s.offset(1);
    }
    return 0;
}
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
