#[no_mangle]
pub unsafe extern "C" fn strsqz(
    mut p: *mut ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut enid: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    enid = p.offset(n as isize);
    while *enid as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != 0 {
        *p = *enid;
        enid = enid.offset(1);
        p = p.offset(1);
    }
    *p = *enid;
    return 0;
}
