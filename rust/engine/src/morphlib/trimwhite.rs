#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trimwhite(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut starts: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    starts = s;
    while *s != 0 {
        s = s.offset(1);
    }
    s = s.offset(-1);
    while __isspace(*s as ::core::ffi::c_int) != 0 && s > starts {
        let fresh0 = s;
        s = s.offset(-1);
        *fresh0 = 0 as ::core::ffi::c_char;
    }
    return 0;
}
