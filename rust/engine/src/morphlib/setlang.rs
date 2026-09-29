static mut curlanguage: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn set_lang(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    curlanguage = n;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn cur_lang() -> ::core::ffi::c_int {
    return curlanguage;
}
