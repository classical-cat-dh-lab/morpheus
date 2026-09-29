extern "C" {
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn MorphFopen(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> *mut FILE;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const MAXPATHNAME: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
static mut fsuff: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
static mut noderivfile: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn NextSuffTable(
    mut entry: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if fsuff.is_null() {
        if noderivfile != 0 {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if OpenDerivFile() == 0 {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    if fgets(entry, MAXPATHNAME, fsuff).is_null() {
        fclose(fsuff);
        fsuff = ::core::ptr::null_mut::<FILE>();
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *entry.offset(strlen(entry) as isize).offset(-(1 as ::core::ffi::c_int as isize)) = 0
        as ::core::ffi::c_char;
    return entry;
}
unsafe extern "C" fn OpenDerivFile() -> ::core::ffi::c_int {
    fsuff = MorphFopen(
        DERIVTYPES.as_ptr() as *mut ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if fsuff.is_null() {
        noderivfile = 1 as ::core::ffi::c_int;
    }
    return (noderivfile == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const DERIVTYPES: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<
        [u8; 28],
        [::core::ffi::c_char; 28],
    >(*b"rule_files/derivtypes.table\0")
};
