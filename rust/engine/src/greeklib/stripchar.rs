extern "C" {
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn stripchar(
    mut s: *mut ::core::ffi::c_char,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    s1 = s;
    while *s1 != 0 {
        if *s1 as ::core::ffi::c_int == c {
            strcpy(s1, s1.offset(1 as ::core::ffi::c_int as isize));
        } else {
            s1 = s1.offset(1);
        }
    }
    return 0;
}
