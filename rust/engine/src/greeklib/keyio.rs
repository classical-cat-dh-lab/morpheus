extern "C" {
    fn vax_fread(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn vax_fwrite(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
pub const KEYLEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn WriteKey(
    mut key: *mut ::core::ffi::c_char,
    mut offp: *mut ::core::ffi::c_int,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut nwritten: ::core::ffi::c_int = 0;
    nwritten = vax_fwrite(key, ::core::mem::size_of::<::core::ffi::c_char>(), KEYLEN, f);
    if nwritten <= 0 as ::core::ffi::c_int {
        return nwritten;
    }
    nwritten = vax_fwrite(
        offp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<::core::ffi::c_int>(),
        1 as ::core::ffi::c_int,
        f,
    );
    if nwritten <= 0 as ::core::ffi::c_int {
        return nwritten;
    }
    return nwritten;
}
#[no_mangle]
pub unsafe extern "C" fn ReadKey(
    mut key: *mut ::core::ffi::c_char,
    mut offp: *mut ::core::ffi::c_int,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut nread: ::core::ffi::c_int = 0;
    nread = vax_fread(
        key,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        KEYLEN,
        f,
    );
    if nread <= 0 as ::core::ffi::c_int {
        return nread;
    }
    nread = vax_fread(
        offp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as ::core::ffi::c_int,
        f,
    );
    if nread <= 0 as ::core::ffi::c_int {
        return nread;
    }
    return nread;
}
