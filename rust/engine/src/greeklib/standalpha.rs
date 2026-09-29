extern "C" {
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn standalpha(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *s != 0 {
        if strncmp(b"a_\0" as *const u8 as *const ::core::ffi::c_char, s, 2 as size_t)
            == 0
        {
            let fresh0 = s;
            s = s.offset(1);
            *fresh0 = 'h' as i32 as ::core::ffi::c_char;
            strcpy(s, s.offset(1 as ::core::ffi::c_int as isize));
        }
        s = s.offset(1);
    }
    return 0;
}
