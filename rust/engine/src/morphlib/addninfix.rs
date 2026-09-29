extern "C" {
    fn getaccp(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn cinsert(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn nsylls(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub const P_ERR: *mut ::core::ffi::c_char = -(1 as ::core::ffi::c_int)
    as *mut ::core::ffi::c_char;
pub const ULTIMA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn addninfix(
    mut word: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut syllp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut c: ::core::ffi::c_int = 'n' as i32;
    if nsylls(word) > 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    syllp = getaccp(word, ULTIMA);
    if syllp == P_ERR {
        return 0 as ::core::ffi::c_int;
    }
    if *syllp as ::core::ffi::c_int == 'p' as i32
        || *syllp as ::core::ffi::c_int == 'b' as i32
        || *syllp as ::core::ffi::c_int == 'f' as i32
    {
        c = 'm' as i32;
    } else if *syllp as ::core::ffi::c_int == 'k' as i32
        || *syllp as ::core::ffi::c_int == 'g' as i32
        || *syllp as ::core::ffi::c_int == 'x' as i32
        || *syllp as ::core::ffi::c_int == 'c' as i32
    {
        c = 'g' as i32;
    }
    cinsert(c, syllp);
    return 1 as ::core::ffi::c_int;
}
