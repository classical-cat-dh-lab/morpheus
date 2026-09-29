extern "C" {
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn is_diphth(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> bool_0;
    fn nsylls(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type bool_0 = ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn getsyll(
    mut word: *mut ::core::ffi::c_char,
    mut syll: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut syllcount: ::core::ffi::c_int = 0;
    if syll < 0 as ::core::ffi::c_int || syll > 2 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_char;
    }
    syllcount = 0 as ::core::ffi::c_int;
    p = word.offset(Xstrlen(word) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    while p >= word {
        if *p as ::core::ffi::c_int == 'a' as i32
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
                || *p as ::core::ffi::c_int == 'W' as i32)
        {
            if syllcount == syll {
                return p;
            }
            if is_diphth(p, word) == 0 {
                syllcount += 1;
            }
        }
        p = p.offset(-1);
    }
    return -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn getsyll2(
    mut word: *mut ::core::ffi::c_char,
    mut syll: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if nsylls(word) == 0 as ::core::ffi::c_int {
        return word;
    }
    p = getsyll(word, syll);
    if p == P_ERR {
        return p;
    }
    if is_diphth(p, word) != 0 {
        return p.offset(-(1 as ::core::ffi::c_int as isize))
    } else {
        return p
    };
}
pub const P_ERR: *mut ::core::ffi::c_char = -(1 as ::core::ffi::c_int)
    as *mut ::core::ffi::c_char;
