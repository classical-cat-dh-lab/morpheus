extern "C" {
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nextkey(
    mut keylist: *mut ::core::ffi::c_char,
    mut nextkey_0: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut a: *mut ::core::ffi::c_char = keylist;
    let mut b: *mut ::core::ffi::c_char = nextkey_0;
    while __isspace(*a as ::core::ffi::c_int) != 0 {
        a = a.offset(1);
    }
    if *a == 0 {
        return 0 as ::core::ffi::c_int;
    }
    while *a as ::core::ffi::c_int != 0 && __isspace(*a as ::core::ffi::c_int) == 0 {
        let fresh0 = a;
        a = a.offset(1);
        let fresh1 = b;
        b = b.offset(1);
        *fresh1 = *fresh0;
    }
    *b = 0 as ::core::ffi::c_char;
    while __isspace(*a as ::core::ffi::c_int) != 0 {
        a = a.offset(1);
    }
    if *a != 0 {
        strcpy(keylist, a);
    } else {
        *keylist = 0 as ::core::ffi::c_char;
    }
    return 1 as ::core::ffi::c_int;
}
