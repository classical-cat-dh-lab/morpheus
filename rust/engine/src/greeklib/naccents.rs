#[no_mangle]
pub unsafe extern "C" fn naccents(
    mut p: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    r = 0 as ::core::ffi::c_int;
    while *p != 0 {
        if *p as ::core::ffi::c_int == ACUTE || *p as ::core::ffi::c_int == GRAVE
            || *p as ::core::ffi::c_int == CIRCUMFLEX
        {
            r += 1;
        }
        p = p.offset(1);
    }
    return r;
}
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const GRAVE: ::core::ffi::c_int = '\\' as i32;
pub const CIRCUMFLEX: ::core::ffi::c_int = '=' as i32;
