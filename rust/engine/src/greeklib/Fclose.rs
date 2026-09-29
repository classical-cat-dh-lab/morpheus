extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn free(_: *mut ::core::ffi::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn xFclose(mut f: *mut FILE) -> ::core::ffi::c_int {
    if f.is_null() {
        fprintf(
            stderr,
            b"hey! trying to close a NULL pointer!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0;
    }
    fclose(f);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn xFree(
    mut p: *mut ::core::ffi::c_char,
    mut errmess: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if p.is_null() {
        fprintf(
            stderr,
            b"asked to free a null pointer for %s!\n\0" as *const u8
                as *const ::core::ffi::c_char,
            errmess,
        );
        return -(1 as ::core::ffi::c_int);
    }
    free(p as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
