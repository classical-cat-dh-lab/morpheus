extern "C" {
    fn simpleredupit(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pres_redupl(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    simpleredupit(s, 0 as ::core::ffi::c_int, 'i' as i32);
    return 0;
}
