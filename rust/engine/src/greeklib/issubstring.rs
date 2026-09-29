extern "C" {
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Xstrncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
#[no_mangle]
pub unsafe extern "C" fn is_substring(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut slen: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    slen = Xstrlen(s1);
    if slen == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if slen > Xstrlen(s2) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = s2;
    while *p != 0 {
        if *p as ::core::ffi::c_int == *s1 as ::core::ffi::c_int {
            if Xstrncmp(p, s1, slen as size_t) == 0 {
                return p;
            }
        }
        p = p.offset(1);
        while *p as ::core::ffi::c_int != 0
            && *p as ::core::ffi::c_int != *s1 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
