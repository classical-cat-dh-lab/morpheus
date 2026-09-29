extern "C" {
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cinsert(
    mut c: ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut enid: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    enid = p
        .offset(Xstrlen(p) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize))
        .offset(2 as ::core::ffi::c_int as isize);
    while enid > p {
        *enid = *enid.offset(-(1 as ::core::ffi::c_int as isize));
        enid = enid.offset(-1);
    }
    *enid = c as ::core::ffi::c_char;
    return 0;
}
