#[no_mangle]
pub unsafe extern "C" fn shortanalog(
    mut p: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    match *p as ::core::ffi::c_int {
        104 => {
            *p = 'e' as i32 as ::core::ffi::c_char;
        }
        119 => {
            *p = 'o' as i32 as ::core::ffi::c_char;
        }
        _ => {}
    }
    return 0;
}
