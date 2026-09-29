#[no_mangle]
pub unsafe extern "C" fn has_diaeresis(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *s != 0 {
        if *s as ::core::ffi::c_int == DIAERESIS {
            return 1 as ::core::ffi::c_int;
        }
        s = s.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
pub const DIAERESIS: ::core::ffi::c_int = '+' as i32;
