#[no_mangle]
pub unsafe extern "C" fn hasaccent(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *s != 0 {
        if *s as ::core::ffi::c_int == ACUTE || *s as ::core::ffi::c_int == GRAVE
            || *s as ::core::ffi::c_int == CIRCUMFLEX
        {
            return 1 as ::core::ffi::c_int;
        }
        s = s.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const GRAVE: ::core::ffi::c_int = '\\' as i32;
pub const CIRCUMFLEX: ::core::ffi::c_int = '=' as i32;
