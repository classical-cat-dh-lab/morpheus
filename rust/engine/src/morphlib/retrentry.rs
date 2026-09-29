extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fseek(
        _: *mut FILE,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(_: *mut FILE) -> ::core::ffi::c_long;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn MorphFopen(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> *mut FILE;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn ReadKey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
        _: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn binlook(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: bool_0,
        _: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
pub type bool_0 = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct endtags {
    pub tagstring: [::core::ffi::c_char; 9],
    pub tagoffset: ::core::ffi::c_long,
}
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const KEYLEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LINDEXSUFFIX: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"lindex\0")
};
#[no_mangle]
pub unsafe extern "C" fn init_preind(
    mut fname: *mut ::core::ffi::c_char,
    mut maxkeys: *mut ::core::ffi::c_int,
) -> *mut endtags {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut etags: *mut endtags = ::core::ptr::null_mut::<endtags>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut flen: ::core::ffi::c_int = 0;
    let mut divisor: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    sprintf(
        &raw mut tmp as *mut ::core::ffi::c_char,
        b"%s.%s\0" as *const u8 as *const ::core::ffi::c_char,
        fname,
        LINDEXSUFFIX.as_ptr(),
    );
    f = MorphFopen(
        &raw mut tmp as *mut ::core::ffi::c_char,
        b"rb\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"init_preind: could not open %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<endtags>();
    }
    fseek(f, 0 as ::core::ffi::c_long, 2 as ::core::ffi::c_int);
    divisor = (KEYLEN as ::core::ffi::c_long as usize)
        .wrapping_add(::core::mem::size_of::<::core::ffi::c_long>() as usize)
        as ::core::ffi::c_long;
    flen = (ftell(f) / divisor) as ::core::ffi::c_int;
    fseek(f, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    *maxkeys = flen;
    etags = calloc(
        (flen as size_t).wrapping_add(1 as size_t),
        ::core::mem::size_of::<endtags>(),
    ) as *mut endtags;
    i = 0 as ::core::ffi::c_int;
    while i < flen {
        if ReadKey(
            &raw mut (*etags.offset(i as isize)).tagstring as *mut ::core::ffi::c_char,
            &raw mut (*etags.offset(i as isize)).tagoffset as *mut ::core::ffi::c_int,
            f,
        ) == 0
        {
            break;
        }
        i += 1;
    }
    fclose(f);
    return etags;
}
#[no_mangle]
pub unsafe extern "C" fn ChckPreIndex(
    mut etags: *mut endtags,
    mut tag: *mut ::core::ffi::c_char,
    mut ntags: ::core::ffi::c_int,
    mut exact_match: ::core::ffi::c_int,
    mut scmp: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
) -> ::core::ffi::c_long {
    let mut rval: ::core::ffi::c_int = 0;
    let mut roff: ::core::ffi::c_long = 0;
    let mut curtag: [::core::ffi::c_char; 9] = [0; 9];
    if Xstrlen(tag) > KEYLEN {
        exact_match = NO;
    }
    strncpy(&raw mut curtag as *mut ::core::ffi::c_char, tag, KEYLEN as size_t);
    curtag[KEYLEN as usize] = 0 as ::core::ffi::c_char;
    rval = binlook(
        etags as *mut ::core::ffi::c_char,
        &raw mut curtag as *mut ::core::ffi::c_char,
        ntags,
        ::core::mem::size_of::<endtags>() as ::core::ffi::c_int,
        exact_match as bool_0,
        scmp,
    );
    if rval < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    }
    if rval == 0 {
        roff = 0 as ::core::ffi::c_long;
    } else {
        roff = (*etags.offset(rval as isize)).tagoffset;
    }
    return roff;
}
#[no_mangle]
pub unsafe extern "C" fn ChckFullIndex(
    mut s: *mut ::core::ffi::c_char,
    mut keys: *mut ::core::ffi::c_char,
    mut fname: *mut ::core::ffi::c_char,
    mut offset: ::core::ffi::c_long,
    mut scmp: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut a: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut slen: size_t = 0;
    let mut comp: ::core::ffi::c_int = 0;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut firstline: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    *keys = 0 as ::core::ffi::c_char;
    f = MorphFopen(
        fname,
        b"r\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"ChckFullIndex(): could not open:%s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            fname,
        );
        return -(1 as ::core::ffi::c_int);
    }
    fseek(f, offset, 0 as ::core::ffi::c_int);
    slen = strlen(s);
    while !fgets(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int,
            f,
        )
        .is_null()
    {
        comp = ::core::mem::transmute::<
            _,
            unsafe extern "C" fn(_, _, _) -> ::core::ffi::c_int,
        >(
            Some(scmp.expect("non-null function pointer"))
                .expect("non-null function pointer"),
        )(s, &raw mut buf as *mut ::core::ffi::c_char, slen);
        if comp == 0
            && __isspace(
                *(&raw mut buf as *mut ::core::ffi::c_char).offset(slen as isize)
                    as ::core::ffi::c_int,
            ) != 0
        {
            a = (&raw mut buf as *mut ::core::ffi::c_char).offset(slen as isize);
            while __isspace(*a as ::core::ffi::c_int) != 0 {
                a = a.offset(1);
            }
            Xstrncpy(keys, a, LONGSTRING as size_t);
            rval = 1 as ::core::ffi::c_int;
            break;
        } else {
            if !(comp < 0 as ::core::ffi::c_int) {
                continue;
            }
            rval = 0 as ::core::ffi::c_int;
            break;
        }
    }
    fclose(f);
    return rval;
}
