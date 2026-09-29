extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stdoutp")]
    static stdout: *mut FILE;
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn ftell(_: *mut FILE) -> ::core::ffi::c_long;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn MorphFopen(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> *mut FILE;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn WriteKey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
        _: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Xstrncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn is_blank(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn morphstrcmp(
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
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MODULUS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const KEYLEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub static mut curkey: [::core::ffi::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut prevkey: [::core::ffi::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut nkeys: ::core::ffi::c_int = MODULUS + 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn index_list(
    mut listname: *mut ::core::ffi::c_char,
    mut tagstring: *mut ::core::ffi::c_char,
    mut modulus: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut finput: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut foutput: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut outfile: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut line: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut curlemma: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut field: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut curoff: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut taglen: ::core::ffi::c_int = 0;
    if modulus > MODULUS {
        modulus = MODULUS;
    }
    finput = MorphFopen(
        listname,
        b"r\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if finput.is_null() {
        fprintf(
            stderr,
            b"Could not open input %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            listname,
        );
        return -(1 as ::core::ffi::c_int);
    }
    sprintf(
        &raw mut outfile as *mut ::core::ffi::c_char,
        b"%s.lindex\0" as *const u8 as *const ::core::ffi::c_char,
        listname,
    );
    foutput = MorphFopen(
        &raw mut outfile as *mut ::core::ffi::c_char,
        b"wb\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if finput.is_null() {
        fprintf(
            stderr,
            b"Could not open output  %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut outfile as *mut ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if !tagstring.is_null() {
        taglen = Xstrlen(tagstring);
    }
    i = 0 as ::core::ffi::c_int;
    loop {
        curoff = ftell(finput) as ::core::ffi::c_int;
        if fgets(
                &raw mut line as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
                    as ::core::ffi::c_int,
                finput,
            )
            .is_null()
        {
            break;
        }
        if Xstrlen(&raw mut line as *mut ::core::ffi::c_char) >= LONGSTRING {
            let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
            f = fopen(
                b"inderr\0" as *const u8 as *const ::core::ffi::c_char,
                b"a\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if !f.is_null() {
                fprintf(
                    f,
                    b"fat line %d bytes:%s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    Xstrlen(&raw mut line as *mut ::core::ffi::c_char),
                    &raw mut line as *mut ::core::ffi::c_char,
                );
                fclose(f);
            }
            printf(
                b"fat line %d bytes:%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                Xstrlen(&raw mut line as *mut ::core::ffi::c_char),
                &raw mut line as *mut ::core::ffi::c_char,
            );
        }
        if !(is_blank(&raw mut line as *mut ::core::ffi::c_char) != 0) {
            if !(line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == '#' as i32)
            {
                if !tagstring.is_null() {
                    if Xstrncmp(
                        &raw mut line as *mut ::core::ffi::c_char,
                        tagstring,
                        taglen as size_t,
                    ) == 0
                    {
                        prockeyline(
                            (&raw mut line as *mut ::core::ffi::c_char)
                                .offset(taglen as isize),
                            modulus,
                            curoff,
                            foutput,
                        );
                    }
                } else {
                    prockeyline(
                        &raw mut line as *mut ::core::ffi::c_char,
                        modulus,
                        curoff,
                        foutput,
                    );
                }
            }
        }
        i += 1;
    }
    fclose(finput);
    fclose(foutput);
    return 0;
}
static mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn prockeyline(
    mut s: *mut ::core::ffi::c_char,
    mut modulus: ::core::ffi::c_int,
    mut curoff: ::core::ffi::c_int,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut curlemma: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut prntflag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    p = s;
    i = 0 as ::core::ffi::c_int;
    while i < KEYLEN {
        let fresh0 = p;
        p = p.offset(1);
        curkey[i as usize] = *fresh0;
        curkey[(i + 1 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_char;
        if *p == 0 || __isspace(*p as ::core::ffi::c_int) != 0 {
            break;
        }
        i += 1;
    }
    nkeys += 1;
    if nkeys >= modulus
        && morphstrcmp(
            &raw mut curkey as *mut ::core::ffi::c_char,
            &raw mut prevkey as *mut ::core::ffi::c_char,
        ) != 0
    {
        if prntflag != 0 {
            fprintf(
                stdout,
                b"%s\t%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut curkey as *mut ::core::ffi::c_char,
                curoff,
            );
        }
        WriteKey(&raw mut curkey as *mut ::core::ffi::c_char, &raw mut curoff, f);
        nkeys = 0 as ::core::ffi::c_int;
    } else if prntflag != 0 {
        printf(
            b"not writing key [%s]:nkeys %d modulus %d prev %s curkey [%s] curoff %ld\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            s,
            nkeys,
            modulus,
            &raw mut prevkey as *mut ::core::ffi::c_char,
            &raw mut curkey as *mut ::core::ffi::c_char,
            curoff,
        );
    }
    Xstrncpy(
        &raw mut prevkey as *mut ::core::ffi::c_char,
        &raw mut curkey as *mut ::core::ffi::c_char,
        LONGSTRING as size_t,
    );
    count += 1;
    return 0;
}
