extern "C" {
    fn getsyll(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn getaccp(
    mut word: *mut ::core::ffi::c_char,
    mut syll: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = getsyll(word, syll);
    if p == P_ERR {
        return -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_char;
    }
    p = p.offset(1);
    while *p as ::core::ffi::c_int == ROUGHBR || *p as ::core::ffi::c_int == SMOOTHBR
        || (*p as ::core::ffi::c_int == HARDLONG
            || *p as ::core::ffi::c_int == HARDSHORT)
    {
        p = p.offset(1);
    }
    return p;
}
pub const P_ERR: *mut ::core::ffi::c_char = -(1 as ::core::ffi::c_int)
    as *mut ::core::ffi::c_char;
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const HARDSHORT: ::core::ffi::c_int = '^' as i32;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
