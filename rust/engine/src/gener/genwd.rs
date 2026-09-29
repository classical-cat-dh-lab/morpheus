use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
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
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn qsort(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn CreatGkword(_: ::core::ffi::c_int) -> *mut gk_word;
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
    fn chckendings(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: Dialect,
        _: *mut ::core::ffi::c_int,
    ) -> *mut gk_string;
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn CompGkForms(gkform1: *mut gk_word, gkform2: *mut gk_word) -> ::core::ffi::c_int;
    fn ErrorMess(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn FixPersAcc(
        _: *mut gk_string,
        _: *mut MorphFlags,
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: word_form,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn FixRecAcc(
        _: *mut gk_word,
        _: *mut MorphFlags,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn FreeGkword(_: *mut gk_word) -> ::core::ffi::c_int;
    fn SprintGkFlags(
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Xstrncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn Xstrncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn add_augment(
        _: *mut gk_word,
        _: *mut MorphFlags,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn add_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn is_blank(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn needs_augment(_: *mut gk_string) -> ::core::ffi::c_int;
    fn nextkey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn nsylls(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn rstprevb(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn set_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripacute(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripmetachars(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripstemsep(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn zap_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const LEMMA: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const IRREG_INDECL: ::core::ffi::c_int = 0o11 as ::core::ffi::c_int;
pub const IRREG_VERB: ::core::ffi::c_int = 0o12 as ::core::ffi::c_int;
pub const INDECL: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const ADJSTEM: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const NOUNSTEM: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const PPARTMASK: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SECONDARY: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const AORIST: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int | SECONDARY;
pub const PERFECT: ::core::ffi::c_int = 0o5 as ::core::ffi::c_int;
pub const PLUPERF: ::core::ffi::c_int = 0o6 as ::core::ffi::c_int;
pub const FUTPERF: ::core::ffi::c_int = 0o7 as ::core::ffi::c_int | SECONDARY;
pub const MIDDLE: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const IMPERATIVE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const INFINITIVE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PARTICIPLE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SYLL_AUGMENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENCLITIC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STEM_ACC: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const HAS_PREVERB: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const UNAUGMENTED: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const IRREGFORM: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const PREVB_AUGMENT: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const INDECLFORM: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const NO_CIRCUMFLEX: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const COMMENT_CHAR: ::core::ffi::c_int = '#' as i32;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const SKIPLINE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
#[no_mangle]
pub static mut BlankGstr: gk_string = gk_string {
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
pub static mut TmpGkword: gk_word = gk_word {
    gs_forminfo: word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    },
    gs_steminfo: 0,
    gs_derivtype: 0,
    gs_dialect: 0,
    gs_geogregion: 0,
    gs_morphflags: [0; 12],
    st_domains: [0; 21],
    gs_prntflags: 0,
    gw_totanal: 0,
    st_lemma: [0; 60],
    gs_preverb: gk_string {
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
    },
    gs_aug1: gk_string {
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
    },
    gs_stem: gk_string {
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
    },
    gs_suffix: gk_string {
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
    },
    gs_endstring: gk_string {
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
    },
    st_rawprvb: [0; 60],
    st_rawword: [0; 60],
    st_workword: [0; 60],
    st_crasis: [0; 60],
    st_oddkeys: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    gw_analysis: ::core::ptr::null_mut::<gk_analysis>(),
};
#[no_mangle]
pub unsafe extern "C" fn GenDictEntry(
    mut Gkword: *mut gk_word,
    mut dentry: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut gkforms: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut formcnt: ::core::ffi::c_int = 0;
    let mut stype: Stemtype = 0;
    let mut keys: [::core::ffi::c_char; 4096] = [0; 4096];
    TmpGkword = *Gkword;
    keys[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    stype = TmpGkword.gs_stem.gs_steminfo;
    TmpGkword.gs_stem.gs_steminfo = stype;
    set_morphflag(
        &raw mut TmpGkword.gs_stem.gs_morphflags as *mut MorphFlags,
        0 as ::core::ffi::c_int,
    );
    set_morphflag(
        &raw mut TmpGkword.gs_morphflags as *mut MorphFlags,
        0 as ::core::ffi::c_int,
    );
    SprintGkFlags(
        &raw mut TmpGkword.gs_stem,
        &raw mut keys as *mut ::core::ffi::c_char,
        b"\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    TmpGkword.gs_endstring = BlankGstr;
    gkforms = GenStemForms(
        &raw mut TmpGkword,
        &raw mut keys as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if gkforms.is_null() {
        return 0;
    }
    formcnt = 0 as ::core::ffi::c_int;
    while (*gkforms.offset(formcnt as isize))
        .st_workword[0 as ::core::ffi::c_int as usize] != 0
    {
        formcnt += 1;
    }
    qsort(
        gkforms as *mut ::core::ffi::c_void,
        formcnt as size_t,
        ::core::mem::size_of::<gk_word>() as size_t,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut gk_word, *mut gk_word) -> ::core::ffi::c_int,
            >,
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
        >(
            Some(
                CompGkForms
                    as unsafe extern "C" fn(
                        *mut gk_word,
                        *mut gk_word,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
    stripmetachars(&raw mut (*gkforms).st_workword as *mut ::core::ffi::c_char);
    strcpy(dentry, &raw mut (*gkforms).st_workword as *mut ::core::ffi::c_char);
    FreeGkword(gkforms);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn GenNxtWord(
    mut f: *mut FILE,
    mut mode: ::core::ffi::c_int,
    mut fout: *mut FILE,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut Rvals: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut lemmakeys: [::core::ffi::c_char; 256] = [0; 256];
    let mut stemkeys: [::core::ffi::c_char; 256] = [0; 256];
    let mut Gkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    stemkeys[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    Gkword = CreatGkword(1 as ::core::ffi::c_int);
    if Gkword.is_null() {
        fprintf(
            stderr,
            b"could not allocate memory for Gkword in GenNxtWord\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0;
    }
    while !(NextDictLine(
        f,
        &raw mut (*Gkword).st_lemma as *mut ::core::ffi::c_char,
        &raw mut lemmakeys as *mut ::core::ffi::c_char,
        b":le:\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != SKIPLINE)
    {}
    loop {
        rval = NextDictLine(
            f,
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut stemkeys as *mut ::core::ffi::c_char,
            b":\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        if !(rval > 0 as ::core::ffi::c_int) {
            break;
        }
        if rval == SKIPLINE {
            break;
        }
        if rval == IRREG_VERB || rval == IRREG_INDECL {
            Rvals = GenIrregForm(
                Gkword,
                &raw mut stemkeys as *mut ::core::ffi::c_char,
                mode,
            );
            if Rvals.is_null() {
                continue;
            }
        } else {
            Rvals = GenStemForms(
                Gkword,
                &raw mut stemkeys as *mut ::core::ffi::c_char,
                mode,
            );
            if Rvals.is_null() {
                continue;
            }
            fprintf(fout, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
        FreeGkword(Rvals);
        Rvals = ::core::ptr::null_mut::<gk_word>();
        Xstrncpy(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        Xstrncpy(
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        Xstrncpy(
            &raw mut (*Gkword).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
    }
    FreeGkword(Gkword);
    Gkword = ::core::ptr::null_mut::<gk_word>();
    return if rval != 0 { 1 as ::core::ffi::c_int } else { 0 as ::core::ffi::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn GenStemForms(
    mut Gkword: *mut gk_word,
    mut keys: *mut ::core::ffi::c_char,
    mut mode: ::core::ffi::c_int,
) -> *mut gk_word {
    let mut stem_gstring: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut gstring: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut gkforms: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut tmpGkword: gk_word = gk_word {
        gs_forminfo: word_form {
            f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
        },
        gs_steminfo: 0,
        gs_derivtype: 0,
        gs_dialect: 0,
        gs_geogregion: 0,
        gs_morphflags: [0; 12],
        st_domains: [0; 21],
        gs_prntflags: 0,
        gw_totanal: 0,
        st_lemma: [0; 60],
        gs_preverb: gk_string {
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
        },
        gs_aug1: gk_string {
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
        },
        gs_stem: gk_string {
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
        },
        gs_suffix: gk_string {
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
        },
        gs_endstring: gk_string {
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
        },
        st_rawprvb: [0; 60],
        st_rawword: [0; 60],
        st_workword: [0; 60],
        st_crasis: [0; 60],
        st_oddkeys: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        gw_analysis: ::core::ptr::null_mut::<gk_analysis>(),
    };
    let mut stemkeys: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut dial: Dialect = 0;
    let mut nends: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut maxforms: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    stemkeys[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    strcpy(&raw mut stemkeys as *mut ::core::ffi::c_char, keys);
    tmpGkword = *Gkword;
    stem_gstring = &raw mut tmpGkword.gs_stem;
    if ScanAsciiKeys(
        &raw mut stemkeys as *mut ::core::ffi::c_char,
        &raw mut tmpGkword,
        stem_gstring,
        NULL as *mut gk_string,
    ) == 0
    {
        fprintf(
            stderr,
            b"stem [%s] lemma [%s]: Something wrong with stemkeys [%s]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut (*stem_gstring).gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut tmpGkword.st_lemma as *mut ::core::ffi::c_char,
            &raw mut stemkeys as *mut ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"stemtype:%d\n\0" as *const u8 as *const ::core::ffi::c_char,
            tmpGkword.gs_steminfo,
        );
        return ::core::ptr::null_mut::<gk_word>();
    }
    if has_morphflag(&raw mut (*Gkword).gs_morphflags as *mut MorphFlags, SYLL_AUGMENT)
        != 0
        && has_morphflag(
            &raw mut (*stem_gstring).gs_morphflags as *mut MorphFlags,
            SYLL_AUGMENT,
        ) == 0
    {
        return ::core::ptr::null_mut::<gk_word>();
    }
    if (*Gkword).gs_preverb.gs_gkstring[0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int != 0
        && strcmp(
            &raw mut (*Gkword).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut tmpGkword.gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
        ) != 0
    {
        return ::core::ptr::null_mut::<gk_word>();
    }
    tmpGkword.gs_steminfo = (*stem_gstring).gs_steminfo;
    tmpGkword.gs_derivtype = (*stem_gstring).gs_derivtype;
    dial = AndDialect(tmpGkword.gs_dialect, (*stem_gstring).gs_dialect);
    if (dial as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<gk_word>();
    }
    gstring = chckendings(
        &raw mut tmpGkword.gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut stemkeys as *mut ::core::ffi::c_char,
        &raw mut tmpGkword.gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut tmpGkword.gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
        tmpGkword.gs_dialect,
        &raw mut nends,
    );
    if gstring.is_null() {
        return ::core::ptr::null_mut::<gk_word>();
    }
    maxforms = nends * 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
    gkforms = CreatGkword(maxforms);
    if gkforms.is_null() {
        fprintf(
            stderr,
            b"no memory for %d gkforms of %s in genwd\n\0" as *const u8
                as *const ::core::ffi::c_char,
            maxforms,
            &raw mut (*Gkword).st_lemma as *mut ::core::ffi::c_char,
        );
        FreeGkString(gstring);
        gstring = ::core::ptr::null_mut::<gk_string>();
        return ::core::ptr::null_mut::<gk_word>();
    }
    tmpGkword.gs_steminfo = (*gstring).gs_steminfo;
    AddWdEndings(&raw mut tmpGkword, gstring, gkforms, maxforms);
    FreeGkString(gstring);
    gstring = ::core::ptr::null_mut::<gk_string>();
    return gkforms;
}
#[no_mangle]
pub unsafe extern "C" fn GenIrregForm(
    mut Gkword: *mut gk_word,
    mut keys: *mut ::core::ffi::c_char,
    mut mode: ::core::ffi::c_int,
) -> *mut gk_word {
    let mut gstring: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut gkforms: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut stemkeys: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut preverb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut dial: Dialect = 0;
    Xstrncpy(&raw mut stemkeys as *mut ::core::ffi::c_char, keys, LONGSTRING as size_t);
    gstring = CreatGkString(2 as ::core::ffi::c_int);
    if gstring.is_null() {
        fprintf(
            stderr,
            b"could not allocate memory for gstring  in GenIrregForm\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gk_word>();
    }
    gkforms = CreatGkword(4 as ::core::ffi::c_int);
    if gkforms.is_null() {
        fprintf(
            stderr,
            b"could not allocate memory for  gkforms in GenIrregForm\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        FreeGkString(gstring);
        gstring = ::core::ptr::null_mut::<gk_string>();
        return ::core::ptr::null_mut::<gk_word>();
    }
    if ScanAsciiKeys(
        &raw mut stemkeys as *mut ::core::ffi::c_char,
        Gkword,
        gstring,
        ::core::ptr::null_mut::<gk_string>(),
    ) == 0
    {
        let mut errmess: [::core::ffi::c_char; 1024] = [0; 1024];
        FreeGkString(gstring);
        sprintf(
            &raw mut errmess as *mut ::core::ffi::c_char,
            b"GenIrregForm Error: no stemtype seen in [%s:%s]\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            &raw mut stemkeys as *mut ::core::ffi::c_char,
        );
        ErrorMess(&raw mut errmess as *mut ::core::ffi::c_char);
        return ::core::ptr::null_mut::<gk_word>();
    }
    add_morphflags(
        &raw mut (*Gkword).gs_stem,
        &raw mut (*gstring).gs_morphflags as *mut MorphFlags,
    );
    if mode & INDECL != 0 {
        add_morphflag(&raw mut (*Gkword).gs_morphflags as *mut MorphFlags, INDECLFORM);
        add_morphflag(&raw mut (*gstring).gs_morphflags as *mut MorphFlags, INDECLFORM);
    }
    add_morphflag(&raw mut (*Gkword).gs_morphflags as *mut MorphFlags, IRREGFORM);
    add_morphflag(&raw mut (*gstring).gs_morphflags as *mut MorphFlags, IRREGFORM);
    if mode & INDECL != 0
        && (*Gkword).gs_preverb.gs_gkstring[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int != 0
    {
        preverb = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        preverb = &raw mut (*Gkword).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char;
    }
    if (*gstring).gs_steminfo == 0 {
        printf(
            b"GenIrregForm: could not find a stemtype for [%s] with keys [%s]\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            keys,
        );
        FreeGkString(gstring);
        gstring = ::core::ptr::null_mut::<gk_string>();
        FreeGkword(gkforms);
        gkforms = ::core::ptr::null_mut::<gk_word>();
        return ::core::ptr::null_mut::<gk_word>();
    }
    Xstrncpy(
        &raw mut (*Gkword).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    (*Gkword).gs_steminfo = (*gstring).gs_steminfo;
    dial = AndDialect((*gstring).gs_dialect, (*Gkword).gs_dialect);
    if dial as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        if (*gstring).gs_dialect != 0 {
            (*gstring).gs_dialect = dial;
        }
    } else {
        FreeGkString(gstring);
        gstring = ::core::ptr::null_mut::<gk_string>();
        FreeGkword(gkforms);
        gkforms = ::core::ptr::null_mut::<gk_word>();
        return ::core::ptr::null_mut::<gk_word>();
    }
    (*Gkword).gs_forminfo = (*gstring).gs_forminfo;
    if has_morphflag(
        &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
        UNAUGMENTED,
    ) != 0 && needs_augment(gstring) == 0
    {
        FreeGkString(gstring);
        gstring = ::core::ptr::null_mut::<gk_string>();
        FreeGkword(gkforms);
        gkforms = ::core::ptr::null_mut::<gk_word>();
        return ::core::ptr::null_mut::<gk_word>();
    }
    BuildAWord(Gkword, gstring, gkforms);
    set_morphflag(
        &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
        0 as ::core::ffi::c_int,
    );
    zap_morphflag(&raw mut (*Gkword).gs_morphflags as *mut MorphFlags, INDECLFORM);
    FreeGkString(gstring);
    gstring = ::core::ptr::null_mut::<gk_string>();
    return gkforms;
}
#[no_mangle]
pub unsafe extern "C" fn NextDictLine(
    mut f: *mut FILE,
    mut word: *mut ::core::ffi::c_char,
    mut wordkeys: *mut ::core::ffi::c_char,
    mut starts: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    loop {
        if fgets(
                &raw mut tmp as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                    as ::core::ffi::c_int,
                f,
            )
            .is_null()
        {
            return -(1 as ::core::ffi::c_int);
        }
        if is_blank(&raw mut tmp as *mut ::core::ffi::c_char) != 0 {
            return 100 as ::core::ffi::c_int;
        }
        if *(&raw mut tmp as *mut ::core::ffi::c_char) as ::core::ffi::c_int
            == COMMENT_CHAR
        {
            continue;
        }
        s = &raw mut tmp as *mut ::core::ffi::c_char;
        if Xstrncmp(
            &raw mut tmp as *mut ::core::ffi::c_char,
            starts,
            Xstrlen(starts) as size_t,
        ) == 0
        {
            let mut tagged: [::core::ffi::c_char; 60] = [0; 60];
            s = s.offset(1);
            nextkey(s, &raw mut tagged as *mut ::core::ffi::c_char);
            Xstrncpy(wordkeys, s, LONGSTRING as size_t);
            Xstrncpy(
                word,
                (&raw mut tagged as *mut ::core::ffi::c_char)
                    .offset(3 as ::core::ffi::c_int as isize),
                LONGSTRING as size_t,
            );
            *(&raw mut tagged as *mut ::core::ffi::c_char)
                .offset(3 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
            if strcmp(starts, b":le:\0" as *const u8 as *const ::core::ffi::c_char) == 0
            {
                return 1 as ::core::ffi::c_int;
            }
            if strcmp(
                &raw mut tagged as *mut ::core::ffi::c_char,
                b"vb:\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                return 0o12 as ::core::ffi::c_int
            } else if strcmp(
                &raw mut tagged as *mut ::core::ffi::c_char,
                b"wd:\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                return 0o11 as ::core::ffi::c_int
            }
            if strcmp(
                &raw mut tagged as *mut ::core::ffi::c_char,
                b"vs:\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0
                && strcmp(
                    &raw mut tagged as *mut ::core::ffi::c_char,
                    b"no:\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                && strcmp(
                    &raw mut tagged as *mut ::core::ffi::c_char,
                    b"aj:\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
            {
                return 100 as ::core::ffi::c_int;
            }
            return 0o1 as ::core::ffi::c_int;
        }
        while __isspace(*s as ::core::ffi::c_int) != 0 {
            let fresh0 = s;
            s = s.offset(1);
            *fresh0;
        }
        if *s == 0 {
            return 0 as ::core::ffi::c_int;
        }
    };
}
pub const MAX_FORM_VARIANTS: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
unsafe extern "C" fn AddWdEndings(
    mut Gkword: *mut gk_word,
    mut Endings: *mut gk_string,
    mut Forms: *mut gk_word,
    mut maxforms: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut SaveGkWord: gk_word = gk_word {
        gs_forminfo: word_form {
            f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
        },
        gs_steminfo: 0,
        gs_derivtype: 0,
        gs_dialect: 0,
        gs_geogregion: 0,
        gs_morphflags: [0; 12],
        st_domains: [0; 21],
        gs_prntflags: 0,
        gw_totanal: 0,
        st_lemma: [0; 60],
        gs_preverb: gk_string {
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
        },
        gs_aug1: gk_string {
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
        },
        gs_stem: gk_string {
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
        },
        gs_suffix: gk_string {
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
        },
        gs_endstring: gk_string {
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
        },
        st_rawprvb: [0; 60],
        st_rawword: [0; 60],
        st_workword: [0; 60],
        st_crasis: [0; 60],
        st_oddkeys: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        gw_analysis: ::core::ptr::null_mut::<gk_analysis>(),
    };
    let mut d: Dialect = 0;
    let mut formvars: ::core::ffi::c_int = 0;
    let mut CurBuf: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut CurEnd: gk_string = gk_string {
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
    CurBuf = CreatGkword(MAX_FORM_VARIANTS + 1 as ::core::ffi::c_int);
    if CurBuf.is_null() {
        fprintf(
            stderr,
            b"no memory for CurBuf in AddWdEndings: raww [%s]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut (*Gkword).st_rawword as *mut ::core::ffi::c_char,
        );
        return 0;
    }
    SaveGkWord = *Gkword;
    j = 0 as ::core::ffi::c_int;
    let mut current_block_19: u64;
    i = 0 as ::core::ffi::c_int;
    while (*Endings.offset(i as isize)).gs_gkstring[0 as ::core::ffi::c_int as usize]
        != 0
    {
        CurEnd = *Endings.offset(i as isize);
        if !(has_morphflag(
            &raw mut (*Gkword).gs_preverb.gs_morphflags as *mut MorphFlags,
            PREVB_AUGMENT,
        ) != 0 && needs_augment(Endings.offset(i as isize)) == 0)
        {
            if !((has_morphflag(
                &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
                UNAUGMENTED,
            ) != 0
                || has_morphflag(
                    &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
                    UNAUGMENTED,
                ) != 0
                || has_morphflag(
                    &raw mut (*Endings.offset(i as isize)).gs_morphflags
                        as *mut MorphFlags,
                    UNAUGMENTED,
                ) != 0) && needs_augment(Endings.offset(i as isize)) == 0)
            {
                if (*Gkword).gs_stem.gs_forminfo.f_mood() != 0 {
                    if (*Gkword).gs_stem.gs_forminfo.f_mood() as ::core::ffi::c_int
                        != (*Endings.offset(i as isize)).gs_forminfo.f_mood()
                            as ::core::ffi::c_int
                    {
                        current_block_19 = 7351195479953500246;
                    } else {
                        current_block_19 = 12599329904712511516;
                    }
                } else {
                    current_block_19 = 12599329904712511516;
                }
                match current_block_19 {
                    7351195479953500246 => {}
                    _ => {
                        CurEnd.gs_steminfo = (*Gkword).gs_steminfo;
                        Xstrncpy(
                            &raw mut (*Gkword).gs_endstring.gs_gkstring
                                as *mut ::core::ffi::c_char,
                            &raw mut (*Endings.offset(i as isize)).gs_gkstring
                                as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
                        );
                        (*Gkword).gs_forminfo = CurEnd.gs_forminfo;
                        formvars = BuildAWord(Gkword, &raw mut CurEnd, CurBuf);
                        if formvars != 0 {
                            k = 0 as ::core::ffi::c_int;
                            while k < formvars {
                                *Forms.offset(j as isize) = *CurBuf.offset(k as isize);
                                j += 1;
                                k += 1;
                            }
                        }
                        *Gkword = SaveGkWord;
                    }
                }
            }
        }
        i += 1;
    }
    (*Forms.offset(j as isize)).st_workword[0 as ::core::ffi::c_int as usize] = 0
        as ::core::ffi::c_char;
    if j > maxforms {
        fprintf(
            stderr,
            b"%d > %d for %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            j,
            maxforms,
            &raw mut (*Gkword).st_lemma as *mut ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    (*CurBuf).gw_analysis = ::core::ptr::null_mut::<gk_analysis>();
    FreeGkword(CurBuf);
    CurBuf = ::core::ptr::null_mut::<gk_word>();
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn BuildAWord(
    mut Gkword: *mut gk_word,
    mut CurEnding: *mut gk_string,
    mut CurForms: *mut gk_word,
) -> ::core::ffi::c_int {
    let mut dial: Dialect = 0;
    *CurForms = *Gkword;
    (*CurForms).gs_endstring = *CurEnding;
    add_morphflags(
        &raw mut (*CurForms).gs_stem,
        &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
    );
    dial = AndDialect((*CurEnding).gs_dialect, (*Gkword).gs_dialect);
    if (dial as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*Gkword).gs_steminfo & 0o70000000 as ::core::ffi::c_int as Stemtype != 0 {
        return BuildAVerb(Gkword, CurEnding, CurForms)
    } else {
        return BuildANoun(Gkword, CurEnding, CurForms)
    };
}
#[no_mangle]
pub unsafe extern "C" fn BuildANoun(
    mut Gkword: *mut gk_word,
    mut CurEnding: *mut gk_string,
    mut CurForms: *mut gk_word,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    (*CurForms).gs_endstring = *CurEnding;
    if (*Gkword).gs_steminfo & 0o10000 as Stemtype != 0
        || (*Gkword).gs_steminfo & 0o4000 as Stemtype != 0
        || (*CurEnding).gs_forminfo.f_mood() as ::core::ffi::c_int == PARTICIPLE
        || (*CurEnding).gs_forminfo.f_mood() as ::core::ffi::c_int == INFINITIVE
    {
        FixPersAcc(
            CurEnding,
            &raw mut (*CurEnding).gs_morphflags as *mut MorphFlags,
            &raw mut (*CurForms).gs_stem,
            &raw mut (*CurEnding).gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
            (*CurEnding).gs_forminfo,
            YES,
        );
        Xstrncpy(
            &raw mut (*CurEnding).gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    } else {
        Xstrncpy(
            &raw mut (*CurEnding).gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    }
    if has_morphflag(
        &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
        ENCLITIC,
    ) != 0
    {
        stripacute(&raw mut (*CurEnding).gs_gkstring as *mut ::core::ffi::c_char);
    }
    Xstrncpy(
        &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
        &raw mut (*CurEnding).gs_gkstring as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BuildAVerb(
    mut Gkword: *mut gk_word,
    mut CurEnding: *mut gk_string,
    mut CurForms: *mut gk_word,
) -> ::core::ffi::c_int {
    let mut tmpstem: [::core::ffi::c_char; 61] = [0; 61];
    let mut preverb: [::core::ffi::c_char; 61] = [0; 61];
    let mut prvb_gstr: *mut gk_string = &raw mut (*CurForms).gs_preverb;
    let mut stem: *mut ::core::ffi::c_char = &raw mut (*Gkword).gs_stem.gs_gkstring
        as *mut ::core::ffi::c_char;
    let mut endstring: *mut ::core::ffi::c_char = &raw mut (*CurForms)
        .gs_endstring
        .gs_gkstring as *mut ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let mut augs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nforms: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*CurForms).gs_endstring = *CurEnding;
    Xstrncpy(&raw mut tmpstem as *mut ::core::ffi::c_char, stem, MAXWORDSIZE as size_t);
    if has_morphflag(
        &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
        HAS_PREVERB,
    ) != 0
    {
        Xstrncpy(
            &raw mut preverb as *mut ::core::ffi::c_char,
            &raw mut (*Gkword).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        stripacc(&raw mut preverb as *mut ::core::ffi::c_char);
    } else {
        preverb[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    }
    if (*CurForms).gs_forminfo.f_mood() as ::core::ffi::c_int == PARTICIPLE
        || (*CurForms).gs_forminfo.f_mood() as ::core::ffi::c_int == INFINITIVE
    {
        zap_morphflag(
            &raw mut (*CurForms).gs_stem.gs_morphflags as *mut MorphFlags,
            SYLL_AUGMENT,
        );
        nforms = BuildANoun(Gkword, CurEnding, CurForms);
        if preverb[0 as ::core::ffi::c_int as usize] != 0 {
            i = 0 as ::core::ffi::c_int;
            while i < nforms {
                rstprevb(
                    &raw mut (*CurForms.offset(i as isize)).st_workword
                        as *mut ::core::ffi::c_char,
                    &raw mut preverb as *mut ::core::ffi::c_char,
                    &raw mut (*CurForms.offset(i as isize)).gs_preverb,
                );
                i += 1;
            }
        }
        return nforms;
    }
    if *endstring as ::core::ffi::c_int != 0
        && *endstring as ::core::ffi::c_int != '*' as i32
    {
        Xstrncat(
            &raw mut tmpstem as *mut ::core::ffi::c_char,
            b"-\0" as *const u8 as *const ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        Xstrncat(
            &raw mut tmpstem as *mut ::core::ffi::c_char,
            endstring,
            ::core::mem::size_of::<[::core::ffi::c_char; 61]>() as ::core::ffi::c_int
                as size_t,
        );
    }
    Xstrncpy(
        &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
        &raw mut tmpstem as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    if *endstring as ::core::ffi::c_int != 0
        && has_morphflag(
            &raw mut (*CurForms).gs_stem.gs_morphflags as *mut MorphFlags,
            ENCLITIC,
        ) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    if *endstring == 0 {
        add_morphflags(
            &raw mut (*CurForms).gs_stem,
            &raw mut (*CurEnding).gs_morphflags as *mut MorphFlags,
        );
        set_morphflag(
            &raw mut (*CurEnding).gs_morphflags as *mut MorphFlags,
            0 as ::core::ffi::c_int,
        );
        set_morphflag(
            &raw mut (*CurForms).gs_endstring.gs_morphflags as *mut MorphFlags,
            0 as ::core::ffi::c_int,
        );
        set_morphflag(
            &raw mut (*CurForms).gs_morphflags as *mut MorphFlags,
            0 as ::core::ffi::c_int,
        );
        if preverb[0 as ::core::ffi::c_int as usize] != 0 {
            if nsylls(&raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char)
                == 1 as ::core::ffi::c_int
            {
                MonoSyllVb(
                    CurForms,
                    (*CurEnding).gs_forminfo,
                    &raw mut preverb as *mut ::core::ffi::c_char,
                );
                stripstemsep(
                    &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
                );
            } else {
                if has_morphflag(
                    &raw mut (*CurForms).gs_preverb.gs_morphflags as *mut MorphFlags,
                    PREVB_AUGMENT,
                ) != 0
                {
                    add_morphflag(
                        &raw mut (*CurForms).gs_stem.gs_morphflags as *mut MorphFlags,
                        PREVB_AUGMENT,
                    );
                }
                rstprevb(
                    &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
                    &raw mut preverb as *mut ::core::ffi::c_char,
                    &raw mut (*CurForms).gs_stem,
                );
                zap_morphflag(
                    &raw mut (*CurForms).gs_stem.gs_morphflags as *mut MorphFlags,
                    PREVB_AUGMENT,
                );
            }
        }
        if has_morphflag(
            &raw mut (*CurForms).gs_stem.gs_morphflags as *mut MorphFlags,
            ENCLITIC,
        ) == 0
            || has_morphflag(
                &raw mut (*CurForms).gs_stem.gs_morphflags as *mut MorphFlags,
                ENCLITIC,
            ) != 0
                && preverb[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        {
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
            set_morphflag(
                &raw mut TmpGstr.gs_morphflags as *mut MorphFlags,
                0 as ::core::ffi::c_int,
            );
            if has_morphflag(
                &raw mut (*CurForms).gs_stem.gs_morphflags as *mut MorphFlags,
                NO_CIRCUMFLEX,
            ) != 0
            {
                add_morphflag(
                    &raw mut (*CurForms).gs_endstring.gs_morphflags as *mut MorphFlags,
                    NO_CIRCUMFLEX,
                );
            }
            add_morphflags(
                &raw mut TmpGstr,
                &raw mut (*CurForms).gs_stem.gs_morphflags as *mut MorphFlags,
            );
            FixRecAcc(
                CurForms,
                &raw mut TmpGstr.gs_morphflags as *mut MorphFlags,
                &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
            );
            stripstemsep(&raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char);
        } else {
            stripacc(&raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char);
        }
    } else {
        augs = add_augment(
            CurForms,
            &raw mut (*CurForms).gs_stem.gs_morphflags as *mut MorphFlags,
            MAX_FORM_VARIANTS,
        );
        if augs != 0
            || ((*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int == 'a' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'e' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'i' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'o' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'u' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'A' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'E' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'I' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'O' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'U' as i32
                || ((*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'h' as i32
                    || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int == 'w' as i32
                    || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int == 'H' as i32
                    || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int == 'W' as i32))
                && ((*CurForms).gs_forminfo.f_tense() as ::core::ffi::c_int == PLUPERF
                    || (*CurForms).gs_forminfo.f_tense() as ::core::ffi::c_int == FUTPERF
                    || (*CurForms).gs_forminfo.f_tense() as ::core::ffi::c_int
                        == PERFECT)
        {
            if augs == 0 {
                augs = 1 as ::core::ffi::c_int;
            }
            i = 0 as ::core::ffi::c_int;
            while i < augs {
                if has_morphflag(
                    &raw mut (*CurForms.offset(i as isize)).gs_preverb.gs_morphflags
                        as *mut MorphFlags,
                    PREVB_AUGMENT,
                ) != 0
                {
                    rstprevb(
                        &raw mut (*CurForms.offset(i as isize)).st_workword
                            as *mut ::core::ffi::c_char,
                        &raw mut preverb as *mut ::core::ffi::c_char,
                        &raw mut (*CurForms.offset(i as isize)).gs_preverb,
                    );
                    FixRecAcc(
                        CurForms.offset(i as isize),
                        &raw mut (*CurForms.offset(i as isize))
                            .gs_endstring
                            .gs_morphflags as *mut MorphFlags,
                        &raw mut (*CurForms.offset(i as isize)).st_workword
                            as *mut ::core::ffi::c_char,
                    );
                    stripstemsep(
                        &raw mut (*CurForms.offset(i as isize)).st_workword
                            as *mut ::core::ffi::c_char,
                    );
                } else {
                    FixRecAcc(
                        CurForms.offset(i as isize),
                        &raw mut (*CurForms.offset(i as isize))
                            .gs_endstring
                            .gs_morphflags as *mut MorphFlags,
                        &raw mut (*CurForms.offset(i as isize)).st_workword
                            as *mut ::core::ffi::c_char,
                    );
                    stripstemsep(
                        &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
                    );
                    if preverb[0 as ::core::ffi::c_int as usize] != 0 {
                        rstprevb(
                            &raw mut (*CurForms.offset(i as isize)).st_workword
                                as *mut ::core::ffi::c_char,
                            &raw mut preverb as *mut ::core::ffi::c_char,
                            &raw mut (*CurForms.offset(i as isize)).gs_preverb,
                        );
                    }
                }
                i += 1;
            }
            return augs;
        } else if nsylls(&raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char)
            == 1 as ::core::ffi::c_int
            && preverb[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        {
            MonoSyllVb(
                CurForms,
                (*CurForms).gs_forminfo,
                &raw mut preverb as *mut ::core::ffi::c_char,
            );
        } else {
            if preverb[0 as ::core::ffi::c_int as usize] != 0 {
                rstprevb(
                    &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
                    &raw mut preverb as *mut ::core::ffi::c_char,
                    &raw mut (*CurForms).gs_preverb,
                );
            }
            FixRecAcc(
                CurForms,
                &raw mut (*CurForms).gs_stem.gs_morphflags as *mut MorphFlags,
                &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
            );
            stripstemsep(&raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char);
        }
    }
    stripstemsep(&raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MonoSyllVb(
    mut CurForms: *mut gk_word,
    mut winfo: word_form,
    mut preverb: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if winfo.f_mood() as ::core::ffi::c_int == IMPERATIVE
        && winfo.f_tense() as ::core::ffi::c_int == AORIST
        && winfo.f_voice() as ::core::ffi::c_int & MIDDLE != 0
        && (nsylls(preverb) == 1 as ::core::ffi::c_int
            || ((*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int == 'a' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'e' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'i' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'o' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'u' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'A' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'E' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'I' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'O' as i32
                || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'U' as i32
                || ((*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 'h' as i32
                    || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int == 'w' as i32
                    || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int == 'H' as i32
                    || (*CurForms).st_workword[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int == 'W' as i32)))
    {
        FixRecAcc(
            CurForms,
            &raw mut (*CurForms).gs_endstring.gs_morphflags as *mut MorphFlags,
            &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
        );
        rstprevb(
            &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
            preverb,
            &raw mut (*CurForms).gs_preverb,
        );
    } else {
        rstprevb(
            &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
            preverb,
            &raw mut (*CurForms).gs_preverb,
        );
        add_morphflag(
            &raw mut (*CurForms).gs_endstring.gs_morphflags as *mut MorphFlags,
            STEM_ACC,
        );
        FixRecAcc(
            CurForms,
            &raw mut (*CurForms).gs_endstring.gs_morphflags as *mut MorphFlags,
            &raw mut (*CurForms).st_workword as *mut ::core::ffi::c_char,
        );
    }
    return 0;
}
