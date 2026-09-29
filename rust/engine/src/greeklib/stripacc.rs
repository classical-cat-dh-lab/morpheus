extern "C" {
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn nsylls(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strsqz(_: *mut ::core::ffi::c_char, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stripacc(
    mut word: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    p = word.offset(Xstrlen(word) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    while p >= word {
        if *p as ::core::ffi::c_int == ACUTE || *p as ::core::ffi::c_int == GRAVE
            || *p as ::core::ffi::c_int == CIRCUMFLEX
        {
            if rval == 0 {
                rval = nsylls(p) + 1 as ::core::ffi::c_int;
            }
            strsqz(p, 1 as ::core::ffi::c_int);
        }
        p = p.offset(-1);
    }
    return rval;
}
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const GRAVE: ::core::ffi::c_int = '\\' as i32;
pub const CIRCUMFLEX: ::core::ffi::c_int = '=' as i32;
