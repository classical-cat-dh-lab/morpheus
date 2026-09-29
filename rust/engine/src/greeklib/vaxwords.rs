extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fread(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        _: *const ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn getc(_: *mut FILE) -> ::core::ffi::c_int;
    fn fputc(_: ::core::ffi::c_int, _: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
pub type int32 = ::core::ffi::c_uint;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn get_int32(
    mut lword: *mut int32,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut tmp: int32 = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    *lword = 0 as int32;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        c = getc(f);
        tmp = c as int32;
        tmp &= 0o377 as int32;
        tmp = tmp << 8 as ::core::ffi::c_int * i;
        *lword = (*lword).wrapping_add(tmp);
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn put_int32(
    mut lword: *mut int32,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut tmp: int32 = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        tmp = *lword;
        tmp = tmp >> 8 as ::core::ffi::c_int * i;
        c = (tmp & 0o377 as int32) as ::core::ffi::c_int;
        fputc(c, f);
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn get_double(
    mut lword: *mut ::core::ffi::c_double,
    mut dsize: ::core::ffi::c_int,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut tmp: int32 = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    *lword = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    i = 0 as ::core::ffi::c_int;
    while i < dsize {
        c = getc(f);
        tmp = c as ::core::ffi::c_long as int32;
        tmp = tmp << 8 as ::core::ffi::c_int * i;
        *lword += tmp as ::core::ffi::c_double;
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn put_double(
    mut lword: *mut ::core::ffi::c_double,
    mut dsize: ::core::ffi::c_int,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut tmp: int32 = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < dsize {
        tmp = *lword as int32;
        tmp = tmp >> 8 as ::core::ffi::c_int * i;
        c = (tmp & 0o377 as int32) as ::core::ffi::c_int;
        fputc(c, f);
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn get_short(
    mut sword: *mut ::core::ffi::c_ushort,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut tmp: ::core::ffi::c_ushort = 0;
    let mut i: ::core::ffi::c_ushort = 0;
    let mut c: ::core::ffi::c_uint = 0;
    *sword = 0 as ::core::ffi::c_ushort;
    i = 0 as ::core::ffi::c_ushort;
    while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        c = getc(f) as ::core::ffi::c_uint;
        tmp = (c & 0o377 as ::core::ffi::c_uint) as ::core::ffi::c_short
            as ::core::ffi::c_ushort;
        tmp = ((tmp as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int * i as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        *sword = (*sword as ::core::ffi::c_int + tmp as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        i = i.wrapping_add(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn put_short(
    mut sword: *mut ::core::ffi::c_short,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut tmp: ::core::ffi::c_ushort = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_ushort = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        tmp = *sword as ::core::ffi::c_ushort;
        tmp = (tmp as ::core::ffi::c_int >> 8 as ::core::ffi::c_int * i)
            as ::core::ffi::c_ushort;
        c = (tmp as ::core::ffi::c_int & 0o377 as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        fputc(c as ::core::ffi::c_int, f);
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn vax_fread(
    mut Buffer: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut nswap: ::core::ffi::c_int,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_uint = 0;
    let mut longp: *mut int32 = ::core::ptr::null_mut::<int32>();
    let mut shortp: *mut ::core::ffi::c_ushort = ::core::ptr::null_mut::<
        ::core::ffi::c_ushort,
    >();
    let mut doubp: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut sdoubp: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    match size {
        1 => {
            return fread(Buffer as *mut ::core::ffi::c_void, size, nswap as size_t, f)
                as ::core::ffi::c_int;
        }
        2 => {
            shortp = Buffer as *mut ::core::ffi::c_ushort;
            shortp = Buffer as *mut ::core::ffi::c_ushort;
            i = 1 as ::core::ffi::c_int;
            while i <= nswap {
                get_short(shortp, f);
                shortp = shortp.offset(1);
                i += 1;
            }
            return nswap;
        }
        4 => {
            longp = Buffer as *mut int32;
            i = 1 as ::core::ffi::c_int;
            while i <= nswap {
                get_int32(longp, f);
                longp = longp.offset(1);
                i += 1;
            }
            return nswap;
        }
        8 => {
            longp = Buffer as *mut int32;
            i = 1 as ::core::ffi::c_int;
            while i <= nswap {
                get_int32(longp, f);
                longp = longp.offset(1);
                i += 1;
            }
            return nswap;
        }
        _ => {
            fprintf(
                stderr,
                b"vax_words: byte swap error, size = %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                size,
            );
            return -(1 as ::core::ffi::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn vax_fwrite(
    mut Buffer: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut nswap: ::core::ffi::c_int,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut t: ::core::ffi::c_uint = 0;
    let mut longp: *mut int32 = ::core::ptr::null_mut::<int32>();
    let mut shortp: *mut ::core::ffi::c_short = ::core::ptr::null_mut::<
        ::core::ffi::c_short,
    >();
    let mut doubp: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut sdoubp: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    match size {
        1 => {
            return fwrite(Buffer as *const ::core::ffi::c_void, size, nswap as size_t, f)
                as ::core::ffi::c_int;
        }
        2 => {
            shortp = Buffer as *mut ::core::ffi::c_short;
            i = 1 as ::core::ffi::c_int;
            while i <= nswap {
                put_short(shortp, f);
                shortp = shortp.offset(1);
                i += 1;
            }
            return nswap;
        }
        4 => {
            longp = Buffer as *mut int32;
            i = 1 as ::core::ffi::c_int;
            while i <= nswap {
                put_int32(longp, f);
                longp = longp.offset(1);
                i += 1;
            }
            return nswap;
        }
        8 => {
            longp = Buffer as *mut int32;
            i = 1 as ::core::ffi::c_int;
            while i <= nswap {
                put_int32(longp, f);
                longp = longp.offset(1);
                i += 1;
            }
            return nswap;
        }
        10 | _ => {
            fprintf(
                stderr,
                b"vax_words: byte swap error, size = %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                size,
            );
            return -(1 as ::core::ffi::c_int);
        }
    };
}
