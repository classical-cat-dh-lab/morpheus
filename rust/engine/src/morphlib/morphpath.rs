extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn cur_lang() -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
pub const DIRCHAR: ::core::ffi::c_int = '/' as i32;
static mut filesopened: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn MorphFopen(
    mut fname: *mut ::core::ffi::c_char,
    mut mode: *mut ::core::ffi::c_char,
) -> *mut FILE {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut tmpname: [::core::ffi::c_char; 1024] = [0; 1024];
    filesopened += 1;
    MorphPathName(fname, &raw mut tmpname as *mut ::core::ffi::c_char);
    f = fopen(&raw mut tmpname as *mut ::core::ffi::c_char, mode) as *mut FILE;
    if f.is_null() {
        fprintf(
            stderr,
            b"MorphFopen: could not open [%s]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut tmpname as *mut ::core::ffi::c_char,
        );
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn NumFilesOpened() -> ::core::ffi::c_int {
    printf(
        b"filesopened [%d]\n\0" as *const u8 as *const ::core::ffi::c_char,
        filesopened,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn MorphPathName(
    mut shorts: *mut ::core::ffi::c_char,
    mut full: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut vRefNum: ::core::ffi::c_short = 0;
    s = getenv(b"MORPHLIB\0" as *const u8 as *const ::core::ffi::c_char);
    if s.is_null() {
        printf(
            b"MORPHLIB not set in your environment!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0;
    }
    if cur_lang() == LATIN {
        sprintf(
            full,
            b"%s/Latin/%s\0" as *const u8 as *const ::core::ffi::c_char,
            s,
            shorts,
        );
    } else if cur_lang() == ITALIAN {
        sprintf(
            full,
            b"%s/Italian/%s\0" as *const u8 as *const ::core::ffi::c_char,
            s,
            shorts,
        );
    } else {
        sprintf(
            full,
            b"%s/Greek/%s\0" as *const u8 as *const ::core::ffi::c_char,
            s,
            shorts,
        );
    }
    if DIRCHAR != '/' as i32 {
        s = full;
        while *s != 0 {
            if *s as ::core::ffi::c_int == '/' as i32 {
                *s = DIRCHAR as ::core::ffi::c_char;
            }
            s = s.offset(1);
        }
    } else if DIRCHAR == '/' as i32 {
        s = full;
        while *s != 0 {
            if *s as ::core::ffi::c_int == ':' as i32 {
                *s = DIRCHAR as ::core::ffi::c_char;
            }
            s = s.offset(1);
        }
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn SysFolderFile(
    mut fullname: *mut ::core::ffi::c_char,
    mut shorts: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut vRefNum: ::core::ffi::c_short = 0 as ::core::ffi::c_short;
    let mut vName: [::core::ffi::c_char; 128] = [0; 128];
    if DIRCHAR != '/' as i32 {
        s = fullname;
        while *s != 0 {
            if *s as ::core::ffi::c_int == '/' as i32 {
                *s = DIRCHAR as ::core::ffi::c_char;
            }
            s = s.offset(1);
        }
    }
    return 0;
}
