extern "C" {
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gkstrlen(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    n = Xstrlen(s);
    if *s as ::core::ffi::c_int == GKPRT_OFF {
        s = s.offset(1);
        n -= 1;
    }
    while *s != 0 {
        if (*s as ::core::ffi::c_int == ACUTE || *s as ::core::ffi::c_int == GRAVE
            || *s as ::core::ffi::c_int == CIRCUMFLEX
            || (*s as ::core::ffi::c_int == HARDLONG
                || *s as ::core::ffi::c_int == HARDSHORT)
            || (*s as ::core::ffi::c_int == ROUGHBR
                || *s as ::core::ffi::c_int == SMOOTHBR)
            || *s as ::core::ffi::c_int == SUBSCR
            || *s as ::core::ffi::c_int == DIAERESIS)
            && !(*s as ::core::ffi::c_int == HARDLONG
                || *s as ::core::ffi::c_int == HARDSHORT)
        {
            n -= 1;
        }
        s = s.offset(1);
    }
    return n;
}
pub const GKPRT_OFF: ::core::ffi::c_int = '!' as i32;
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const GRAVE: ::core::ffi::c_int = '\\' as i32;
pub const CIRCUMFLEX: ::core::ffi::c_int = '=' as i32;
pub const DIAERESIS: ::core::ffi::c_int = '+' as i32;
pub const SUBSCR: ::core::ffi::c_int = '|' as i32;
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const HARDSHORT: ::core::ffi::c_int = '^' as i32;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
