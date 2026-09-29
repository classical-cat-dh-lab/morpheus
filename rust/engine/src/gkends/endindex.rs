extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fseek(
        _: *mut FILE,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(_: *mut FILE) -> ::core::ffi::c_long;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn MorphFopen(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> *mut FILE;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn Xstrncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn stripquant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn vax_fread(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn morphstrncmp(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct endind {
    pub ebuf: *mut ::core::ffi::c_char,
    pub eptr: *mut *mut ::core::ffi::c_char,
    pub nelems: ::core::ffi::c_int,
}
pub const BUFSIZ: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
#[no_mangle]
pub static mut DictEntTags: *mut endind = ::core::ptr::null::<endind>() as *mut endind;
#[no_mangle]
pub static mut CmpVbtags: *mut endind = ::core::ptr::null::<endind>() as *mut endind;
#[no_mangle]
pub static mut VbEtags: *mut endind = ::core::ptr::null::<endind>() as *mut endind;
#[no_mangle]
pub static mut DerEtags: *mut endind = ::core::ptr::null::<endind>() as *mut endind;
#[no_mangle]
pub static mut NomEtags: *mut endind = ::core::ptr::null::<endind>() as *mut endind;
#[no_mangle]
pub static mut VstemEtags: *mut endind = ::core::ptr::null::<endind>() as *mut endind;
#[no_mangle]
pub unsafe extern "C" fn chcknend(
    mut endstr: *mut ::core::ffi::c_char,
    mut keys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut startoff: ::core::ffi::c_long = 0;
    let mut tmpendstr: [::core::ffi::c_char; 61] = [0; 61];
    Xstrncpy(
        &raw mut tmpendstr as *mut ::core::ffi::c_char,
        endstr,
        ::core::mem::size_of::<[::core::ffi::c_char; 61]>() as ::core::ffi::c_int
            as size_t,
    );
    stripquant(&raw mut tmpendstr as *mut ::core::ffi::c_char);
    if NomEtags.is_null() {
        NomEtags = calloc(
            1 as ::core::ffi::c_int as size_t,
            ::core::mem::size_of::<endind>(),
        ) as *mut endind;
        if NomEtags.is_null() {
            fprintf(
                stderr,
                b"could not allcoate NomEtags\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        init_endind(NENDLIST.as_ptr() as *mut ::core::ffi::c_char, NomEtags);
    }
    return checkendind(
        NomEtags,
        &raw mut tmpendstr as *mut ::core::ffi::c_char,
        keys,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    size_t,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                morphstrncmp
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn chckdictent(
    mut possent: *mut ::core::ffi::c_char,
    mut keys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut startoff: ::core::ffi::c_long = 0;
    let mut tmpendstr: [::core::ffi::c_char; 61] = [0; 61];
    Xstrncpy(
        &raw mut tmpendstr as *mut ::core::ffi::c_char,
        possent,
        ::core::mem::size_of::<[::core::ffi::c_char; 61]>() as ::core::ffi::c_int
            as size_t,
    );
    stripquant(&raw mut tmpendstr as *mut ::core::ffi::c_char);
    if DictEntTags.is_null() {
        DictEntTags = calloc(
            1 as ::core::ffi::c_int as size_t,
            ::core::mem::size_of::<endind>(),
        ) as *mut endind;
        if DictEntTags.is_null() {
            fprintf(
                stderr,
                b"could not allcoate DictEntTags\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        init_endind(DICTENTLIST.as_ptr() as *mut ::core::ffi::c_char, DictEntTags);
    }
    return checkendind(
        DictEntTags,
        &raw mut tmpendstr as *mut ::core::ffi::c_char,
        keys,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    size_t,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                strncmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn chckcmpvb(
    mut endstr: *mut ::core::ffi::c_char,
    mut keys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut startoff: ::core::ffi::c_long = 0;
    let mut tmpendstr: [::core::ffi::c_char; 61] = [0; 61];
    Xstrncpy(
        &raw mut tmpendstr as *mut ::core::ffi::c_char,
        endstr,
        ::core::mem::size_of::<[::core::ffi::c_char; 61]>() as ::core::ffi::c_int
            as size_t,
    );
    stripquant(&raw mut tmpendstr as *mut ::core::ffi::c_char);
    if CmpVbtags.is_null() {
        CmpVbtags = calloc(
            1 as ::core::ffi::c_int as size_t,
            ::core::mem::size_of::<endind>(),
        ) as *mut endind;
        if CmpVbtags.is_null() {
            fprintf(
                stderr,
                b"could not allcoate CmpVbtags\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        init_endind(CMPVBLIST.as_ptr() as *mut ::core::ffi::c_char, CmpVbtags);
    }
    return checkendind(
        CmpVbtags,
        &raw mut tmpendstr as *mut ::core::ffi::c_char,
        keys,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    size_t,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                strncmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn chckend(
    mut endstring: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    tmp[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    return (chckvend(endstring, &raw mut tmp as *mut ::core::ffi::c_char) != 0
        || chcknend(endstring, &raw mut tmp as *mut ::core::ffi::c_char) != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chckvend(
    mut endstr: *mut ::core::ffi::c_char,
    mut keys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut startoff: ::core::ffi::c_long = 0;
    let mut curhit: ::core::ffi::c_int = 0;
    if VbEtags.is_null() {
        VbEtags = calloc(
            1 as ::core::ffi::c_int as size_t,
            ::core::mem::size_of::<endind>(),
        ) as *mut endind;
        if VbEtags.is_null() {
            fprintf(
                stderr,
                b"could not allcoate VbEtags\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        init_endind(VENDLIST.as_ptr() as *mut ::core::ffi::c_char, VbEtags);
    }
    curhit = checkendind(
        VbEtags,
        endstr,
        keys,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    size_t,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                morphstrncmp
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
    return curhit;
}
#[no_mangle]
pub unsafe extern "C" fn chckvstem(
    mut stemstr: *mut ::core::ffi::c_char,
    mut keys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut startoff: ::core::ffi::c_long = 0;
    let mut curhit: ::core::ffi::c_int = 0;
    if VstemEtags.is_null() {
        VstemEtags = calloc(
            1 as ::core::ffi::c_int as size_t,
            ::core::mem::size_of::<endind>(),
        ) as *mut endind;
        if VstemEtags.is_null() {
            fprintf(
                stderr,
                b"could not allcoate VstemEtags\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        init_endind(VBINDEX.as_ptr() as *mut ::core::ffi::c_char, VstemEtags);
    }
    curhit = checkendind(
        VstemEtags,
        stemstr,
        keys,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    size_t,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                morphstrncmp
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
    return curhit;
}
#[no_mangle]
pub unsafe extern "C" fn chckdvend(
    mut endstr: *mut ::core::ffi::c_char,
    mut keys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut startoff: ::core::ffi::c_long = 0;
    let mut curhit: ::core::ffi::c_int = 0;
    if DerEtags.is_null() {
        DerEtags = calloc(
            1 as ::core::ffi::c_int as size_t,
            ::core::mem::size_of::<endind>(),
        ) as *mut endind;
        if DerEtags.is_null() {
            fprintf(
                stderr,
                b"could not allcoate DerEtags\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        init_endind(DERENDLIST.as_ptr() as *mut ::core::ffi::c_char, DerEtags);
    }
    curhit = checkendind(
        DerEtags,
        endstr,
        keys,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    size_t,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                morphstrncmp
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
    return curhit;
}
#[no_mangle]
pub unsafe extern "C" fn init_endind(
    mut fname: *mut ::core::ffi::c_char,
    mut etags: *mut endind,
) -> *mut endind {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pp: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_char,
    >();
    let mut flen: ::core::ffi::c_long = 0;
    let mut nread: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_long = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nlines: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sofar: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    f = MorphFopen(
        fname,
        b"r\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"init_endind: could not open %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            fname,
        );
        return ::core::ptr::null_mut::<endind>();
    }
    fseek(f, 0 as ::core::ffi::c_long, 2 as ::core::ffi::c_int);
    flen = ftell(f);
    fseek(f, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    (*etags).ebuf = calloc(
        (flen as size_t).wrapping_add(1 as size_t),
        ::core::mem::size_of::<::core::ffi::c_char>(),
    ) as *mut ::core::ffi::c_char;
    if (*etags).ebuf.is_null() {
        fprintf(
            stderr,
            b"could not build buffer for endtags\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<endind>();
    }
    s = (*etags).ebuf;
    loop {
        nread = vax_fread(
            s.offset(sofar as isize),
            ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
            BUFSIZ,
            f,
        );
        if nread <= 0 as ::core::ffi::c_int {
            break;
        }
        sofar += nread as ::core::ffi::c_long;
    }
    i = 0 as ::core::ffi::c_long;
    while i < sofar {
        if *s.offset(i as isize) as ::core::ffi::c_int == '\n' as i32 {
            nlines += 1;
        }
        i += 1;
    }
    nlines += 1;
    (*etags).eptr = calloc(
        nlines as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
    ) as *mut *mut ::core::ffi::c_char;
    if (*etags).eptr.is_null() {
        fprintf(
            stderr,
            b"ran out of memory in init_endind\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<endind>();
    }
    pp = (*etags).eptr;
    i = 0 as ::core::ffi::c_long;
    while i < nlines as ::core::ffi::c_long {
        let ref mut fresh0 = *pp.offset(i as isize);
        *fresh0 = s;
        while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != '\n' as i32 {
            s = s.offset(1);
        }
        if *s == 0 {
            break;
        }
        if *s as ::core::ffi::c_int == '\n' as i32 {
            *s = 0 as ::core::ffi::c_char;
            s = s.offset(1);
        }
        i += 1;
    }
    (*etags).nelems = nlines;
    return etags;
}
#[no_mangle]
pub unsafe extern "C" fn checkendind(
    mut etags: *mut endind,
    mut endstr: *mut ::core::ffi::c_char,
    mut keys: *mut ::core::ffi::c_char,
    mut scmp: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    let mut high: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut low: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut comp: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ntags: ::core::ffi::c_int = 0;
    let mut curtag: [::core::ffi::c_char; 60] = [0; 60];
    let mut taglen: size_t = 0;
    let mut pp: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_char,
    >();
    Xstrncpy(&raw mut curtag as *mut ::core::ffi::c_char, endstr, MAXWORDSIZE as size_t);
    Xstrncat(
        &raw mut curtag as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    taglen = strlen(&raw mut curtag as *mut ::core::ffi::c_char);
    ntags = (*etags).nelems;
    pp = (*etags).eptr;
    high = ntags - 1 as ::core::ffi::c_int;
    low = 0 as ::core::ffi::c_int;
    while low <= high {
        mid = (low + high) / 2 as ::core::ffi::c_int;
        comp = ::core::mem::transmute::<
            _,
            unsafe extern "C" fn(_, _, _) -> ::core::ffi::c_int,
        >(
            Some(scmp.expect("non-null function pointer"))
                .expect("non-null function pointer"),
        )(&raw mut curtag as *mut ::core::ffi::c_char, *pp.offset(mid as isize), taglen);
        if comp < 0 as ::core::ffi::c_int {
            high = mid - 1 as ::core::ffi::c_int;
        } else if comp > 0 as ::core::ffi::c_int {
            low = mid + 1 as ::core::ffi::c_int;
        } else {
            Xstrncpy(
                keys,
                (*pp.offset(mid as isize)).offset(taglen as isize),
                LONGSTRING as size_t,
            );
            return 1 as ::core::ffi::c_int;
        }
    }
    *keys = 0 as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
pub const VBINDEX: [::core::ffi::c_char; 15] = unsafe {
    ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"steminds/vbind\0")
};
pub const VENDLIST: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<
        [u8; 27],
        [::core::ffi::c_char; 27],
    >(*b"endtables/indices/vbendind\0")
};
pub const NENDLIST: [::core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<
        [u8; 26],
        [::core::ffi::c_char; 26],
    >(*b"endtables/indices/nendind\0")
};
pub const DERENDLIST: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<
        [u8; 24],
        [::core::ffi::c_char; 24],
    >(*b"derivs/indices/derivind\0")
};
pub const CMPVBLIST: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<
        [u8; 19],
        [::core::ffi::c_char; 19],
    >(*b"stemsrc/vbs.cmp.ml\0")
};
pub const DICTENTLIST: [::core::ffi::c_char; 16] = unsafe {
    ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"stemsrc/lemlist\0")
};
