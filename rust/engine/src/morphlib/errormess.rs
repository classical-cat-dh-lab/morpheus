extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn ErrorMess(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const ::core::ffi::c_char, s);
    return 0;
}
