#[no_mangle]
pub unsafe extern "C" fn subchar(
    mut s: *mut ::core::ffi::c_char,
    mut c1: ::core::ffi::c_int,
    mut c2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    while *s != 0 {
        if *s as ::core::ffi::c_int == c1 {
            *s = c2 as ::core::ffi::c_char;
        }
        s = s.offset(1);
    }
    return 0;
}
