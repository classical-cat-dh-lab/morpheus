extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn xFree(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
pub const BUFSIZ: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn Xstrncpy(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_int {
    strcpy(s1, s2);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Ystrncpy(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_int {
    if Xstrlen(s2) as size_t >= len {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if len < 5 as size_t || len > (BUFSIZ * 4 as ::core::ffi::c_int) as size_t {
            fprintf(
                stderr,
                b"Xstrncpy: hey! len %d for [%s] \n\0" as *const u8
                    as *const ::core::ffi::c_char,
                len,
                s2,
            );
        }
        p = malloc(len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
        if p.is_null() {
            fprintf(
                stderr,
                b"could not allocate %d byte buf in Xstrncpy!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                len.wrapping_add(1 as size_t),
            );
            *s1 = 0 as ::core::ffi::c_char;
            return 0;
        }
        strncpy(p, s2, len);
        *p.offset(len as isize).offset(-(1 as ::core::ffi::c_int as isize)) = 0
            as ::core::ffi::c_char;
        strcpy(s1, p);
        xFree(
            p,
            b"Xstrncpy buffer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"%d bytes into %d:%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            Xstrlen(s2),
            len,
            s2,
        );
    } else {
        strcpy(s1, s2);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn Xstrncat(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut nlen: size_t = 0;
    if len < 5 as size_t || len > (BUFSIZ * 4 as ::core::ffi::c_int) as size_t {
        fprintf(
            stderr,
            b"Xstrncat: hey! len %d for [%s] \n\0" as *const u8
                as *const ::core::ffi::c_char,
            len,
            s2,
        );
    }
    if (Xstrlen(s1) + Xstrlen(s2)) as size_t > len.wrapping_sub(1 as size_t) {
        fprintf(
            stderr,
            b"limit: %d; tacking [%s] + [%s] is too big!\n\0" as *const u8
                as *const ::core::ffi::c_char,
            len,
            s1,
            s2,
        );
        nlen = len.wrapping_sub(Xstrlen(s1) as size_t).wrapping_sub(1 as size_t);
        fprintf(stderr, b"nlen %d\n\0" as *const u8 as *const ::core::ffi::c_char, nlen);
        strncat(s1, s2, nlen);
        *s1.offset(len as isize).offset(-(1 as ::core::ffi::c_int as isize)) = 0
            as ::core::ffi::c_char;
    } else {
        strcat(s1, s2);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn Xstrncmp(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_int {
    return strncmp(s1, s2, len);
}
#[no_mangle]
pub unsafe extern "C" fn Xstrlen(
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return strlen(s) as ::core::ffi::c_int;
}
