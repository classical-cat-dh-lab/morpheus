#[no_mangle]
pub unsafe extern "C" fn has_quant(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *s != 0 {
        if *s as ::core::ffi::c_int == HARDLONG || *s as ::core::ffi::c_int == HARDSHORT
        {
            return 1 as ::core::ffi::c_int;
        }
        s = s.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const HARDSHORT: ::core::ffi::c_int = '^' as i32;
