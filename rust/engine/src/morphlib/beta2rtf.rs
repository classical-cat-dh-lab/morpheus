extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stdoutp")]
    static stdout: *mut FILE;
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
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
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn beta2smk(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn set_roman() -> ::core::ffi::c_int;
    fn exit(_: ::core::ffi::c_int) -> !;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut domlist: [*mut ::core::ffi::c_char; 256] = [::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char; 256];
#[no_mangle]
pub static mut lastdom: [::core::ffi::c_char; 1024] = [0; 1024];
#[no_mangle]
pub unsafe extern "C" fn _main(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut line: [::core::ffi::c_char; 6144] = [0; 6144];
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    extern "C" {
        #[link_name = "MorphFopen"]
        fn MorphFopen_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
        ) -> *mut FILE;
    }
    let mut fname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut basename: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut outfname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fout: *mut FILE = stdout;
    let mut nfile: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut result: [::core::ffi::c_uchar; 6144] = [0; 6144];
    sprintf(
        &raw mut basename as *mut ::core::ffi::c_char,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        *argv.offset(1 as ::core::ffi::c_int as isize),
    );
    sprintf(
        &raw mut outfname as *mut ::core::ffi::c_char,
        b"%s.rtf\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut basename as *mut ::core::ffi::c_char,
    );
    fout = fopen(
        &raw mut outfname as *mut ::core::ffi::c_char,
        b"w\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if fout.is_null() {
        fprintf(
            stderr,
            b"could not open [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut outfname as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    f = fopen(
        b"rtfhead\0" as *const u8 as *const ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if f.is_null() {
        fprintf(
            stderr,
            b"could not open rtfhead!\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"place a copy of the file \"rtfhead\" into this folder!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 6144]>() as ::core::ffi::c_int,
            f,
        )
        .is_null()
    {
        fprintf(
            fout,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut line as *mut ::core::ffi::c_char,
        );
    }
    fclose(f);
    f = fopen(
        &raw mut basename as *mut ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if f.is_null() {
        fprintf(
            stderr,
            b"could not open [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut basename as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 6144]>() as ::core::ffi::c_int,
            f,
        )
        .is_null()
    {
        if __isspace(line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int) != 0
            || line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '?' as i32
            || line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32
        {
            continue;
        }
        p = (&raw mut line as *mut ::core::ffi::c_char)
            .offset(strlen(&raw mut line as *mut ::core::ffi::c_char) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        if *p as ::core::ffi::c_int == '\n' as i32 {
            *p = 0 as ::core::ffi::c_char;
        }
        conv_defline(&raw mut line as *mut ::core::ffi::c_char, fout);
    }
    fclose(f);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn conv_defline(
    mut s: *mut ::core::ffi::c_char,
    mut fout: *mut FILE,
) -> ::core::ffi::c_int {
    let mut res1: [::core::ffi::c_char; 128] = [0; 128];
    let mut res2: [::core::ffi::c_char; 128] = [0; 128];
    let mut result: [::core::ffi::c_char; 6144] = [0; 6144];
    let mut introp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut levnum: ::core::ffi::c_int = 0;
    res1[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    introp = b"\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
    if has_pref(
        s,
        b":dnum\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        while *s as ::core::ffi::c_int != 0 && __isspace(*s as ::core::ffi::c_int) == 0 {
            s = s.offset(1);
        }
        while __isspace(*s as ::core::ffi::c_int) != 0 {
            s = s.offset(1);
        }
        levnum = check_deflev(
            s,
            &raw mut res1 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as ::core::ffi::c_int,
        );
        set_roman();
        beta2smk(
            &raw mut res1 as *mut ::core::ffi::c_char,
            &raw mut res2 as *mut ::core::ffi::c_char,
        );
        strcat(
            &raw mut res2 as *mut ::core::ffi::c_char,
            b"\t\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if levnum == 2 as ::core::ffi::c_int {
            introp = b"\\s2\\fi-510\\li1134\\sb80\\sa80\\tx1134 \\f20\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else if levnum == 3 as ::core::ffi::c_int {
            introp = b"\\s3\\fi-539\\li1701\\sb60\\sa60\\tx1729 \\f20\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else if levnum == 4 as ::core::ffi::c_int {
            introp = b"\\s4\\fi-510\\li652\\sb80\\sa80\\tx652 \\f20\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else if levnum == 5 as ::core::ffi::c_int {
            introp = b"\\s5\\fi-539\\li2296\\sb80\\sa80\\tx2296 \\f20\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else if levnum == 9 as ::core::ffi::c_int {
            introp = b"\\s9\\sb60\\sa60 \\f20\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            strcpy(
                &raw mut res2 as *mut ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        fprintf(
            fout,
            b"\\pard\\plain{%s {%s\0" as *const u8 as *const ::core::ffi::c_char,
            introp,
            &raw mut res2 as *mut ::core::ffi::c_char,
        );
        while *s as ::core::ffi::c_int != 0 && __isspace(*s as ::core::ffi::c_int) == 0 {
            s = s.offset(1);
        }
        while __isspace(*s as ::core::ffi::c_int) != 0 {
            s = s.offset(1);
        }
        beta2smk(s, &raw mut result as *mut ::core::ffi::c_char);
        fprintf(
            fout,
            b"%s}\\par\\pard\\plain}\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut result as *mut ::core::ffi::c_char,
        );
        return 0;
    }
    if has_pref(
        s,
        b":xref\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        while *s as ::core::ffi::c_int != 0 && __isspace(*s as ::core::ffi::c_int) == 0 {
            s = s.offset(1);
        }
        while __isspace(*s as ::core::ffi::c_int) != 0 {
            s = s.offset(1);
        }
        beta2smk(s, &raw mut result as *mut ::core::ffi::c_char);
        fprintf(
            fout,
            b"\\s6{%s}\\par\\pard\\plain\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut result as *mut ::core::ffi::c_char,
        );
        return 0;
    }
    if has_pref(
        s,
        b":le:\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
        || has_pref(
            s,
            b":cv:\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        ) != 0
    {
        *s.offset(3 as ::core::ffi::c_int as isize) = '$' as i32 as ::core::ffi::c_char;
        beta2smk(
            s.offset(3 as ::core::ffi::c_int as isize),
            &raw mut result as *mut ::core::ffi::c_char,
        );
        fprintf(
            fout,
            b"\\s7\\sb160 \\b\\f20\\fs24{%s}\\par\\pard\\plain\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut result as *mut ::core::ffi::c_char,
        );
        return 0;
    }
    if has_pref(
        s,
        b":comm\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
    {
        while *s as ::core::ffi::c_int != 0 && __isspace(*s as ::core::ffi::c_int) == 0 {
            s = s.offset(1);
        }
        while __isspace(*s as ::core::ffi::c_int) != 0 {
            s = s.offset(1);
        }
        beta2smk(s, &raw mut result as *mut ::core::ffi::c_char);
        fprintf(
            fout,
            b"\\s8{%s}\\par\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut result as *mut ::core::ffi::c_char,
        );
        return 0;
    }
    fprintf(fout, b"%s\\par\n\0" as *const u8 as *const ::core::ffi::c_char, s);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn check_deflev(
    mut p: *mut ::core::ffi::c_char,
    mut res: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *p as ::core::ffi::c_int == '[' as i32 {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == '&' as i32 {
        p = p.offset(1);
    }
    strncpy(res, p, len as size_t);
    s = res;
    while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != ']' as i32 {
        s = s.offset(1);
    }
    *s = 0 as ::core::ffi::c_char;
    s = res;
    if *res as ::core::ffi::c_int == '0' as i32 {
        return 9 as ::core::ffi::c_int;
    }
    if *res as ::core::ffi::c_int == '*' as i32 {
        res = res.offset(1);
        if *res as ::core::ffi::c_int == 'i' as i32
            || *res as ::core::ffi::c_int == 'v' as i32
        {
            return 2 as ::core::ffi::c_int;
        }
        if if 0 as ::core::ffi::c_int != 0 {
            isalpha(*res as ::core::ffi::c_int)
        } else {
            ((*res as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        } != 0
        {
            return 4 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    if if 0 as ::core::ffi::c_int != 0 {
        isdigit(*res as ::core::ffi::c_int)
    } else {
        ((*res as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint)
            < 10 as ::core::ffi::c_uint) as ::core::ffi::c_int
    } != 0
    {
        return 3 as ::core::ffi::c_int;
    }
    if if 0 as ::core::ffi::c_int != 0 {
        isalpha(*res as ::core::ffi::c_int)
    } else {
        ((*res as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    } != 0
    {
        return 5 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn has_pref(
    mut s: *mut ::core::ffi::c_char,
    mut prefs: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return (strncmp(s, prefs, strlen(prefs)) == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_greek(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *s as ::core::ffi::c_int != 0 && __isspace(*s as ::core::ffi::c_int) == 0 {
        match *s as ::core::ffi::c_int {
            61 | 47 | 92 => return 1 as ::core::ffi::c_int,
            _ => {}
        }
        s = s.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
