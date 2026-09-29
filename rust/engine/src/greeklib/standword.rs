extern "C" {
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripdiaer(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn zap_rr_breath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn standword(
    mut word: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut a: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut b: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    stripdiaer(word);
    zap_rr_breath(word);
    a = word;
    b = &raw mut tmp as *mut ::core::ffi::c_char;
    while (*a as ::core::ffi::c_int == ',' as i32
        || *a as ::core::ffi::c_int == '.' as i32
        || *a as ::core::ffi::c_int == ':' as i32
        || *a as ::core::ffi::c_int == ';' as i32
        || *a as ::core::ffi::c_int == '"' as i32
        || *a as ::core::ffi::c_int == '<' as i32
        || *a as ::core::ffi::c_int == '>' as i32
        || *a as ::core::ffi::c_int == '[' as i32
        || *a as ::core::ffi::c_int == ']' as i32
        || *a as ::core::ffi::c_int == '^' as i32
        || *a as ::core::ffi::c_int == '-' as i32) && *a as ::core::ffi::c_int != 0
    {
        a = a.offset(1);
    }
    while *a != 0 {
        if *a as ::core::ffi::c_int == ACUTE || *a as ::core::ffi::c_int == GRAVE
            || *a as ::core::ffi::c_int == CIRCUMFLEX
        {
            stripacc(a.offset(1 as ::core::ffi::c_int as isize));
        }
        if *a as ::core::ffi::c_int == GRAVE {
            *a = ACUTE as ::core::ffi::c_char;
        }
        if *a as ::core::ffi::c_int == '*' as i32 && a > word {
            a = a.offset(1);
        } else if !(*a as ::core::ffi::c_int == ',' as i32
            || *a as ::core::ffi::c_int == '.' as i32
            || *a as ::core::ffi::c_int == ':' as i32
            || *a as ::core::ffi::c_int == ';' as i32
            || *a as ::core::ffi::c_int == '"' as i32
            || *a as ::core::ffi::c_int == '<' as i32
            || *a as ::core::ffi::c_int == '>' as i32
            || *a as ::core::ffi::c_int == '[' as i32
            || *a as ::core::ffi::c_int == ']' as i32
            || *a as ::core::ffi::c_int == '^' as i32
            || *a as ::core::ffi::c_int == '-' as i32)
        {
            let fresh0 = a;
            a = a.offset(1);
            let fresh1 = b;
            b = b.offset(1);
            *fresh1 = *fresh0;
        } else {
            a = a.offset(1);
        }
    }
    *b = 0 as ::core::ffi::c_char;
    strcpy(word, &raw mut tmp as *mut ::core::ffi::c_char);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn zap2acc(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut haveacc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *s != 0 {
        if *s as ::core::ffi::c_int == ACUTE || *s as ::core::ffi::c_int == GRAVE
            || *s as ::core::ffi::c_int == CIRCUMFLEX
        {
            if haveacc != 0 {
                strcpy(s, s.offset(1 as ::core::ffi::c_int as isize));
                continue;
            } else {
                haveacc = 1 as ::core::ffi::c_int;
            }
        }
        s = s.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn striphyph(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *s != 0 {
        if *s as ::core::ffi::c_int == '-' as i32 {
            strcpy(s, s.offset(1 as ::core::ffi::c_int as isize));
            if *s == 0 {
                break;
            }
        }
        s = s.offset(1);
    }
    return 0;
}
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const GRAVE: ::core::ffi::c_int = '\\' as i32;
pub const CIRCUMFLEX: ::core::ffi::c_int = '=' as i32;
