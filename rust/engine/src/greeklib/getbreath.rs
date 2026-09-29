#[no_mangle]
pub unsafe extern "C" fn getbreath(
    mut p: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *p != 0 {
        if *p as ::core::ffi::c_int == ROUGHBR || *p as ::core::ffi::c_int == SMOOTHBR {
            return *p as ::core::ffi::c_int;
        }
        p = p.offset(1);
    }
    return ' ' as i32;
}
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
pub const NOBREATH: ::core::ffi::c_int = ' ' as i32;
