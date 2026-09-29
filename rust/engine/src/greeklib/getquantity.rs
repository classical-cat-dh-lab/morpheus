extern "C" {
    fn getaccent(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn quantprim(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: bool_0,
        _: bool_0,
    ) -> ::core::ffi::c_int;
    fn getsyll(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
pub type bool_0 = ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn getquantity(
    mut word: *mut ::core::ffi::c_char,
    mut syll: ::core::ffi::c_int,
    mut nom: *mut ::core::ffi::c_char,
    mut is_ending: bool_0,
    mut is_oblique: bool_0,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = getsyll(word, syll);
    if p == P_ERR {
        return -(1 as ::core::ffi::c_int);
    }
    p = p.offset(1);
    if *p as ::core::ffi::c_int == HARDLONG || *p as ::core::ffi::c_int == HARDSHORT {
        if *p as ::core::ffi::c_int == HARDLONG {
            return 1 as ::core::ffi::c_int
        } else if *p as ::core::ffi::c_int == HARDSHORT {
            return 0 as ::core::ffi::c_int
        } else if nom.is_null() {
            return 0 as ::core::ffi::c_int
        } else if getaccent(nom, PENULT) == ACUTE {
            return 1 as ::core::ffi::c_int
        } else {
            return 0 as ::core::ffi::c_int
        }
    } else {
        return quantprim(word, syll, is_ending, is_oblique)
    };
}
pub const I_ERR: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const P_ERR: *mut ::core::ffi::c_char = -(1 as ::core::ffi::c_int)
    as *mut ::core::ffi::c_char;
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const PENULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LONG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SHORT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const HARDSHORT: ::core::ffi::c_int = '^' as i32;
