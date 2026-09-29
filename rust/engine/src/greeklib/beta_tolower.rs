extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn beta_tolower(
    mut word: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *word as ::core::ffi::c_int != BETA_UCASE_MARKER {
        return 0 as ::core::ffi::c_int;
    }
    s = word;
    while (if 0 as ::core::ffi::c_int != 0 {
        isalpha(*s as ::core::ffi::c_int)
    } else {
        ((*s as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    }) == 0 && *s as ::core::ffi::c_int != 0
    {
        s = s.offset(1);
    }
    *word = *s;
    strcpy(s, s.offset(1 as ::core::ffi::c_int as isize));
    return 0;
}
pub const BETA_UCASE_MARKER: ::core::ffi::c_int = '*' as i32;
