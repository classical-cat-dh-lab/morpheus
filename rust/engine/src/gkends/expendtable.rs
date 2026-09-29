use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stdoutp")]
    static stdout: *mut FILE;
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fseek(
        _: *mut FILE,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn CreatGkword(_: ::core::ffi::c_int) -> *mut gk_word;
    fn MorphFopen(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> *mut FILE;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn ScanAsciiKeys(
        _: *mut ::core::ffi::c_char,
        _: *mut gk_word,
        _: *mut gk_string,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn FreeGkword(_: *mut gk_word) -> ::core::ffi::c_int;
    fn InitGstrMem() -> ::core::ffi::c_int;
    fn PrntNewGstrings(_: *mut FILE, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn is_blank(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn mk_end(
        _: *mut ::core::ffi::c_char,
        _: *mut gk_string,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn nextkey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type int32 = ::core::ffi::c_uint;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C, align(4))]
pub struct word_form {
    #[bitfield(name = "f_voice", ty = "::core::ffi::c_uint", bits = "0..=2")]
    #[bitfield(name = "f_mood", ty = "::core::ffi::c_uint", bits = "3..=6")]
    #[bitfield(name = "f_tense", ty = "::core::ffi::c_uint", bits = "7..=10")]
    #[bitfield(name = "f_person", ty = "::core::ffi::c_uint", bits = "11..=13")]
    #[bitfield(name = "f_number", ty = "::core::ffi::c_uint", bits = "14..=16")]
    #[bitfield(name = "f_case", ty = "::core::ffi::c_uint", bits = "17..=22")]
    #[bitfield(name = "f_degree", ty = "::core::ffi::c_uint", bits = "23..=24")]
    #[bitfield(name = "f_gender", ty = "::core::ffi::c_uint", bits = "25..=28")]
    pub f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [u8; 4],
}
pub type Dialect = ::core::ffi::c_short;
pub type GeogRegion = int32;
pub type Stemtype = ::core::ffi::c_uint;
pub type Derivtype = ::core::ffi::c_uint;
pub type MorphFlags = ::core::ffi::c_uchar;
pub type PrntFlags = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_string {
    pub gs_forminfo: word_form,
    pub gs_steminfo: Stemtype,
    pub gs_derivtype: Derivtype,
    pub gs_dialect: Dialect,
    pub gs_geogregion: GeogRegion,
    pub gs_morphflags: [MorphFlags; 12],
    pub st_domains: [::core::ffi::c_char; 21],
    pub gs_gkstring: [::core::ffi::c_char; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_analysis {
    pub gs_forminfo: word_form,
    pub gs_steminfo: Stemtype,
    pub gs_derivtype: Derivtype,
    pub gs_dialect: Dialect,
    pub gs_geogregion: GeogRegion,
    pub gs_morphflags: [MorphFlags; 12],
    pub st_domains: [::core::ffi::c_char; 21],
    pub st_lemma: [::core::ffi::c_char; 60],
    pub st_dictform: [::core::ffi::c_char; 60],
    pub st_engform: [::core::ffi::c_char; 60],
    pub gs_preverb: gk_string,
    pub gs_aug1: gk_string,
    pub gs_stem: gk_string,
    pub gs_suffix: gk_string,
    pub gs_endstring: gk_string,
    pub st_rawprvb: [::core::ffi::c_char; 60],
    pub st_rawword: [::core::ffi::c_char; 60],
    pub st_workword: [::core::ffi::c_char; 60],
    pub st_crasis: [::core::ffi::c_char; 60],
    pub z: [::core::ffi::c_char; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_word {
    pub gs_forminfo: word_form,
    pub gs_steminfo: Stemtype,
    pub gs_derivtype: Derivtype,
    pub gs_dialect: Dialect,
    pub gs_geogregion: GeogRegion,
    pub gs_morphflags: [MorphFlags; 12],
    pub st_domains: [::core::ffi::c_char; 21],
    pub gs_prntflags: PrntFlags,
    pub gw_totanal: ::core::ffi::c_int,
    pub st_lemma: [::core::ffi::c_char; 60],
    pub gs_preverb: gk_string,
    pub gs_aug1: gk_string,
    pub gs_stem: gk_string,
    pub gs_suffix: gk_string,
    pub gs_endstring: gk_string,
    pub st_rawprvb: [::core::ffi::c_char; 60],
    pub st_rawword: [::core::ffi::c_char; 60],
    pub st_workword: [::core::ffi::c_char; 60],
    pub st_crasis: [::core::ffi::c_char; 60],
    pub st_oddkeys: *mut ::core::ffi::c_char,
    pub gw_analysis: *mut gk_analysis,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const ADJSTEM: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const NOUNSTEM: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const IS_DERIV: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const MAXENDSTRING: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const COMMENT_CHAR: ::core::ffi::c_int = '#' as i32;
pub const DIRCHAR: ::core::ffi::c_int = '/' as i32;
static mut Blnk: gk_string = gk_string {
    gs_forminfo: word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    },
    gs_steminfo: 0,
    gs_derivtype: 0,
    gs_dialect: 0,
    gs_geogregion: 0,
    gs_morphflags: [0; 12],
    st_domains: [0; 21],
    gs_gkstring: [0; 60],
};
#[no_mangle]
pub unsafe extern "C" fn expendtables(
    mut tabname: *mut ::core::ffi::c_char,
    mut maintable: ::core::ffi::c_int,
    mut formcode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut finput: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut foutput: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut shortname: [::core::ffi::c_char; 60] = [0; 60];
    let mut fname: [::core::ffi::c_char; 512] = [0; 512];
    let mut inpfname: [::core::ffi::c_char; 512] = [0; 512];
    let mut outfname: [::core::ffi::c_char; 512] = [0; 512];
    let mut curendstr: [::core::ffi::c_char; 60] = [0; 60];
    let mut maxstring: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut basename: [::core::ffi::c_char; 60] = [0; 60];
    let mut typep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut TmpGstr: gk_string = gk_string {
        gs_forminfo: word_form {
            f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
        },
        gs_steminfo: 0,
        gs_derivtype: 0,
        gs_dialect: 0,
        gs_geogregion: 0,
        gs_morphflags: [0; 12],
        st_domains: [0; 21],
        gs_gkstring: [0; 60],
    };
    let mut stype: Stemtype = 0;
    s = tabname;
    while *s != 0 {
        s = s.offset(1);
    }
    s = s.offset(-1);
    while s > tabname && *s as ::core::ffi::c_int != DIRCHAR {
        s = s.offset(-1);
    }
    if maintable != 0 {
        if *s as ::core::ffi::c_int == DIRCHAR {
            s = s.offset(1);
        }
        strcpy(&raw mut fname as *mut ::core::ffi::c_char, s);
        strcpy(&raw mut basename as *mut ::core::ffi::c_char, s);
        TmpGstr = Blnk;
        stype = 0 as Stemtype;
        if formcode == DODERIV {
            add_morphflag(&raw mut TmpGstr.gs_morphflags as *mut MorphFlags, IS_DERIV);
            sprintf(
                &raw mut shortname as *mut ::core::ffi::c_char,
                b"%s.deriv\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut fname as *mut ::core::ffi::c_char,
            );
            finput = fopen(
                &raw mut shortname as *mut ::core::ffi::c_char,
                b"r\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if finput.is_null() {
                sprintf(
                    &raw mut shortname as *mut ::core::ffi::c_char,
                    b"derivs%csource%c%s.deriv\0" as *const u8
                        as *const ::core::ffi::c_char,
                    DIRCHAR,
                    DIRCHAR,
                    &raw mut fname as *mut ::core::ffi::c_char,
                );
                finput = MorphFopen(
                    &raw mut shortname as *mut ::core::ffi::c_char,
                    b"r\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                if finput.is_null() {
                    printf(
                        b"could not open [%s.deriv] or [%s]\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &raw mut fname as *mut ::core::ffi::c_char,
                        &raw mut shortname as *mut ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                sprintf(
                    &raw mut shortname as *mut ::core::ffi::c_char,
                    b"%s%cout%c%s.out\0" as *const u8 as *const ::core::ffi::c_char,
                    DERIVTABLEDIR.as_ptr(),
                    DIRCHAR,
                    DIRCHAR,
                    &raw mut fname as *mut ::core::ffi::c_char,
                );
            }
        } else {
            let mut TmpGkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
            TmpGkword = CreatGkword(1 as ::core::ffi::c_int);
            ScanAsciiKeys(
                &raw mut basename as *mut ::core::ffi::c_char,
                TmpGkword,
                &raw mut TmpGstr,
                ::core::ptr::null_mut::<gk_string>(),
            );
            FreeGkword(TmpGkword);
            stype = TmpGstr.gs_steminfo;
            sprintf(
                &raw mut shortname as *mut ::core::ffi::c_char,
                b"%s.end\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut fname as *mut ::core::ffi::c_char,
            );
            finput = fopen(
                &raw mut shortname as *mut ::core::ffi::c_char,
                b"r\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if finput.is_null() {
                sprintf(
                    &raw mut shortname as *mut ::core::ffi::c_char,
                    b"endtables%csource%c%s.end\0" as *const u8
                        as *const ::core::ffi::c_char,
                    DIRCHAR,
                    DIRCHAR,
                    &raw mut fname as *mut ::core::ffi::c_char,
                );
                finput = MorphFopen(
                    &raw mut shortname as *mut ::core::ffi::c_char,
                    b"r\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                if finput.is_null() {
                    printf(
                        b"could not open [%s.end] or [%s]\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &raw mut fname as *mut ::core::ffi::c_char,
                        &raw mut shortname as *mut ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
            }
            sprintf(
                &raw mut shortname as *mut ::core::ffi::c_char,
                b"%s%cout%c%s.out\0" as *const u8 as *const ::core::ffi::c_char,
                ENDTABLEDIR.as_ptr(),
                DIRCHAR,
                DIRCHAR,
                &raw mut fname as *mut ::core::ffi::c_char,
            );
        }
        foutput = MorphFopen(
            &raw mut shortname as *mut ::core::ffi::c_char,
            b"wb\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        if foutput.is_null() {
            fclose(finput);
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        strcpy(&raw mut fname as *mut ::core::ffi::c_char, s);
        basename[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        finput = fopen(
            &raw mut fname as *mut ::core::ffi::c_char,
            b"r\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if finput.is_null() {
            fprintf(
                stderr,
                b"could not open %s for reading\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                &raw mut fname as *mut ::core::ffi::c_char,
            );
            fclose(finput);
            return -(1 as ::core::ffi::c_int);
        }
    }
    if formcode == DODERIV {
        strcat(
            &raw mut basename as *mut ::core::ffi::c_char,
            b" is_deriv\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if formcode == DOWORD {
        strcat(
            &raw mut basename as *mut ::core::ffi::c_char,
            b" indeclform\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if InitGstrMem() == 0 {
        fprintf(
            stderr,
            b"Could not allocate storage for ending array\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            finput,
        )
        .is_null()
    {
        if is_blank(&raw mut line as *mut ::core::ffi::c_char) != 0 {
            continue;
        }
        if *(&raw mut line as *mut ::core::ffi::c_char) as ::core::ffi::c_int
            == COMMENT_CHAR
        {
            continue;
        }
        nextkey(
            &raw mut line as *mut ::core::ffi::c_char,
            &raw mut curendstr as *mut ::core::ffi::c_char,
        );
        if strlen(&raw mut curendstr as *mut ::core::ffi::c_char) >= maxstring as size_t
            && strlen(&raw mut curendstr as *mut ::core::ffi::c_char)
                <= MAXENDSTRING as size_t
        {
            maxstring = strlen(&raw mut curendstr as *mut ::core::ffi::c_char)
                .wrapping_add(1 as size_t) as ::core::ffi::c_int;
        }
    }
    fseek(finput, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            finput,
        )
        .is_null()
    {
        let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
        if is_blank(&raw mut line as *mut ::core::ffi::c_char) != 0 {
            continue;
        }
        if *(&raw mut line as *mut ::core::ffi::c_char) as ::core::ffi::c_int
            == COMMENT_CHAR
        {
            continue;
        }
        if AddEndLine(
            &raw mut line as *mut ::core::ffi::c_char,
            &raw mut basename as *mut ::core::ffi::c_char,
            maxstring,
        ) < 0 as ::core::ffi::c_int
        {
            break;
        }
    }
    fclose(finput);
    if maintable != 0 && formcode != DOWORD {
        PrntNewGstrings(foutput, 1 as ::core::ffi::c_int);
        if foutput != stdout {
            fclose(foutput);
        }
    }
    sprintf(
        &raw mut shortname as *mut ::core::ffi::c_char,
        b"%s%cascii%c%s.asc\0" as *const u8 as *const ::core::ffi::c_char,
        if formcode == DODERIV { DERIVTABLEDIR.as_ptr() } else { ENDTABLEDIR.as_ptr() },
        DIRCHAR,
        DIRCHAR,
        &raw mut fname as *mut ::core::ffi::c_char,
    );
    printf(
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut shortname as *mut ::core::ffi::c_char,
    );
    foutput = MorphFopen(
        &raw mut shortname as *mut ::core::ffi::c_char,
        b"w\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if foutput.is_null() {
        fprintf(
            stderr,
            b"Could not open [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut shortname as *mut ::core::ffi::c_char,
        );
    } else {
        PrntNewGstrings(foutput, 0 as ::core::ffi::c_int);
        fclose(foutput);
    }
    if stype & ADJSTEM as Stemtype != 0 {
        stype |= NOUNSTEM as Stemtype;
    } else if stype & NOUNSTEM as Stemtype != 0 {
        stype |= ADJSTEM as Stemtype;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn AddEndLine(
    mut el: *mut ::core::ffi::c_char,
    mut basename: *mut ::core::ffi::c_char,
    mut maxstring: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut havestr: [::core::ffi::c_char; 60] = [0; 60];
    let mut Have: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut Avoid: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut TmpGkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    Have = CreatGkString(1 as ::core::ffi::c_int);
    Avoid = CreatGkString(1 as ::core::ffi::c_int);
    TmpGkword = CreatGkword(1 as ::core::ffi::c_int);
    nextkey(el, &raw mut havestr as *mut ::core::ffi::c_char);
    ScanAsciiKeys(basename, TmpGkword, Have, Avoid);
    ScanAsciiKeys(el, TmpGkword, Have, Avoid);
    Xstrncpy(
        &raw mut (*Have).gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut havestr as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    mk_end(&raw mut havestr as *mut ::core::ffi::c_char, Have, Avoid);
    FreeGkString(Have);
    FreeGkString(Avoid);
    FreeGkword(TmpGkword);
    return 1 as ::core::ffi::c_int;
}
pub const ENDTABLEDIR: [::core::ffi::c_char; 10] = unsafe {
    ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"endtables\0")
};
pub const DERIVTABLEDIR: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"derivs\0")
};
pub const DOWORD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DODERIV: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
