extern "C" {
    fn strsqz(_: *mut ::core::ffi::c_char, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stripacute(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *s != 0 {
        if *s as ::core::ffi::c_int == ACUTE {
            strsqz(s, 1 as ::core::ffi::c_int);
        }
        s = s.offset(1);
    }
    return 0;
}
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
