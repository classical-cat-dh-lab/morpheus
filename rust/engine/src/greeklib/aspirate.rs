#[no_mangle]
pub unsafe extern "C" fn aspirate(
    mut p: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !(*p as ::core::ffi::c_int == 'p' as i32 || *p as ::core::ffi::c_int == 't' as i32
        || *p as ::core::ffi::c_int == 'k' as i32
        || (*p as ::core::ffi::c_int == 'b' as i32
            || *p as ::core::ffi::c_int == 'd' as i32
            || *p as ::core::ffi::c_int == 'g' as i32))
    {
        return 0;
    }
    match *p as ::core::ffi::c_int {
        116 | 100 => {
            *p = 'q' as i32 as ::core::ffi::c_char;
        }
        112 | 98 => {
            *p = 'f' as i32 as ::core::ffi::c_char;
        }
        107 | 103 => {
            *p = 'x' as i32 as ::core::ffi::c_char;
        }
        _ => {}
    }
    return 0;
}
