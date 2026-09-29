extern "C" {
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strsqz(_: *mut ::core::ffi::c_char, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stripquant(
    mut word: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    stripshortmark(word);
    striplongmark(word);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn stripshortmark(
    mut word: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = word.offset(Xstrlen(word) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    while p >= word {
        if *p as ::core::ffi::c_int == HARDSHORT {
            strsqz(p, 1 as ::core::ffi::c_int);
        }
        p = p.offset(-1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn striplongmark(
    mut word: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = word.offset(Xstrlen(word) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    while p >= word {
        if *p as ::core::ffi::c_int == HARDLONG {
            strsqz(p, 1 as ::core::ffi::c_int);
        }
        p = p.offset(-1);
    }
    return 0;
}
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const HARDSHORT: ::core::ffi::c_int = '^' as i32;
