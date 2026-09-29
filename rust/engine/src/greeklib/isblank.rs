#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_blank(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *s as ::core::ffi::c_int != 0 && __isspace(*s as ::core::ffi::c_int) != 0 {
        s = s.offset(1);
    }
    if *s == 0 {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
