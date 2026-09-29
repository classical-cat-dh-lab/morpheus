extern "C" {
    fn stripquant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripstemsep(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripzeroend(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn zap_rr_breath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stripmetachars(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    stripquant(s);
    stripzeroend(s);
    stripstemsep(s);
    zap_rr_breath(s);
    return 0;
}
