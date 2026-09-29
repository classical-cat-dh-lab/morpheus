extern "C" {
    fn getaccent(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkaccent(
    mut word: *mut ::core::ffi::c_char,
    mut syll: *mut ::core::ffi::c_int,
    mut acc: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *syll = ULTIMA;
    *acc = getaccent(word, *syll);
    if *acc != NOACCENT {
        return 0 as ::core::ffi::c_int;
    }
    *syll = PENULT;
    *acc = getaccent(word, *syll);
    if *acc != NOACCENT {
        return 1 as ::core::ffi::c_int;
    }
    *syll = ANTEPENULT;
    *acc = getaccent(word, *syll);
    if *acc != NOACCENT {
        return 2 as ::core::ffi::c_int;
    }
    *acc = NOACCENT;
    *syll = -(1 as ::core::ffi::c_int);
    return -(1 as ::core::ffi::c_int);
}
pub const NOACCENT: ::core::ffi::c_int = ' ' as i32;
pub const ULTIMA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PENULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ANTEPENULT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
