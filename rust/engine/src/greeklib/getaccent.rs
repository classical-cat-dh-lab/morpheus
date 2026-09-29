extern "C" {
    fn getaccp(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn getaccent(
    mut word: *mut ::core::ffi::c_char,
    mut syll: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = getaccp(word, syll);
    if p == P_ERR {
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_char as ::core::ffi::c_int;
    }
    if *p as ::core::ffi::c_int == ACUTE || *p as ::core::ffi::c_int == GRAVE
        || *p as ::core::ffi::c_int == CIRCUMFLEX
    {
        return *p as ::core::ffi::c_int;
    }
    return ' ' as i32;
}
pub const P_ERR: *mut ::core::ffi::c_char = -(1 as ::core::ffi::c_int)
    as *mut ::core::ffi::c_char;
pub const C_ERR: ::core::ffi::c_char = -(1 as ::core::ffi::c_int) as ::core::ffi::c_char;
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const GRAVE: ::core::ffi::c_int = '\\' as i32;
pub const CIRCUMFLEX: ::core::ffi::c_int = '=' as i32;
pub const NOACCENT: ::core::ffi::c_int = ' ' as i32;
