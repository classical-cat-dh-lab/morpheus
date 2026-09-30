use ::c2rust_bitfields;
extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tolower(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn toupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[cfg_attr(target_os = "macos", link_name = "__stdoutp")]
    static stdout: *mut FILE;
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn CreatGkword(_: ::core::ffi::c_int) -> *mut gk_word;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn NameOfStemtype(st: Stemtype) -> *mut ::core::ffi::c_char;
    fn stand_phonetics(Gkword: *mut gk_word) -> ::core::ffi::c_int;
    fn standword(s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn is_blank(s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn FreeGkword(_: *mut gk_word) -> ::core::ffi::c_int;
    fn PrntAnalyses(_: *mut gk_word, _: PrntFlags, _: *mut FILE) -> ::core::ffi::c_int;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Xstrncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn beta_tolower(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn chckend(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn checkaccent(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn checkcrasis(_: *mut gk_word) -> ::core::ffi::c_int;
    fn checkindecl(_: *mut gk_word) -> ::core::ffi::c_int;
    fn checknom(_: *mut gk_word) -> ::core::ffi::c_int;
    fn checkword(_: *mut gk_word) -> ::core::ffi::c_int;
    fn cmpend(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn hasaccent(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn is_rawpreverb(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn naccents(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn nsylls(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn set_gwmorphflags(_: *mut gk_word, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn set_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct enclitic_word {
    pub enclitic: [::core::ffi::c_char; 60],
    pub stemtype: Stemtype,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const ATTIC: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const IONIC: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const HOMERIC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const ALL_DIAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NON_HOMERIC_EPIC: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const EPIC: ::core::ffi::c_int = NON_HOMERIC_EPIC | HOMERIC;
pub const PROSE: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const ADJSTEM: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const NOUNSTEM: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const PRONOUN: ::core::ffi::c_int = 0o40006 as ::core::ffi::c_int;
pub const PERS_PRON: ::core::ffi::c_int = 0o40010 as ::core::ffi::c_int;
pub const PPARTMASK: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const C_ERR: ::core::ffi::c_char = -(1 as ::core::ffi::c_int) as ::core::ffi::c_char;
pub const BETA_UCASE_MARKER: ::core::ffi::c_int = '*' as i32;
pub const ULTIMA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UNAUGMENTED: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const POETIC: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const STRICT_CASE: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const GREEK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const LEMCOUNT: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
#[no_mangle]
pub static mut WantDialects: Dialect = ALL_DIAL as Dialect;
#[no_mangle]
pub static mut BlankWord: gk_word = gk_word {
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
    st_oddkeys: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    gw_analysis: ::core::ptr::null::<gk_analysis>() as *mut gk_analysis,
};
#[no_mangle]
pub static mut CheckWord: gk_word = gk_word {
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
    st_oddkeys: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    gw_analysis: ::core::ptr::null::<gk_analysis>() as *mut gk_analysis,
};
#[no_mangle]
pub unsafe extern "C" fn teststring(
    mut string: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return checkstring(string, 0 as ::core::ffi::c_int, stdout);
}
#[no_mangle]
pub unsafe extern "C" fn checkstring(
    mut string: *mut ::core::ffi::c_char,
    mut prntflags: PrntFlags,
    mut fout: *mut FILE,
) -> ::core::ffi::c_int {
    let mut Gkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut fcurout: *mut FILE = fout;
    let mut nanals: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nlems: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if is_blank(string) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if strlen(string) >= MAXWORDSIZE as size_t {
        return 0 as ::core::ffi::c_int;
    }
    Gkword = CreatGkword(1 as ::core::ffi::c_int);
    (*Gkword).gs_dialect = WantDialects;
    Xstrncpy(
        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
        string,
        MAXWORDSIZE as size_t,
    );
    (*Gkword).gs_prntflags = prntflags;
    Xstrncpy(
        &raw mut (*Gkword).st_rawword as *mut ::core::ffi::c_char,
        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    if cur_lang() != ITALIAN {
        standword(&raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char);
    }
    stand_phonetics(Gkword);
    checkstring1(Gkword);
    if prntflags as ::core::ffi::c_int & LEMCOUNT != 0 {
        nlems = cntlems(Gkword);
        FreeGkword(Gkword);
        return nlems;
    }
    if prntflags != 0
        && {
            nanals = (*Gkword).gw_totanal;
            nanals > 0 as ::core::ffi::c_int
        }
    {
        PrntAnalyses(Gkword, prntflags, fcurout);
    }
    FreeGkword(Gkword);
    return nanals;
}
#[no_mangle]
pub unsafe extern "C" fn cntlems(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut Anal: *mut gk_analysis = ::core::ptr::null_mut::<gk_analysis>();
    let mut prevlem: [::core::ffi::c_char; 1024] = [0; 1024];
    prevlem[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < (*Gkword).gw_totanal {
        Anal = (*Gkword).gw_analysis.offset(i as isize);
        if strchr(&raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char, '-' as i32)
            .is_null()
        {
            if strcmp(
                &raw mut prevlem as *mut ::core::ffi::c_char,
                &raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char,
            ) != 0
            {
                cnt += 1;
            }
            strcpy(
                &raw mut prevlem as *mut ::core::ffi::c_char,
                &raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char,
            );
        }
        i += 1;
    }
    return cnt;
}
#[no_mangle]
pub unsafe extern "C" fn is_article(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut curanal: *mut gk_analysis = (*Gkword).gw_analysis;
    i = 0 as ::core::ffi::c_int;
    while i < (*Gkword).gw_totanal {
        if strcmp(
            b"article\0" as *const u8 as *const ::core::ffi::c_char,
            NameOfStemtype((*curanal.offset(i as isize)).gs_steminfo),
        ) == 0
        {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn end_phrase(
    mut checkw: *mut gk_word,
    mut Gkword: *mut gk_word,
) -> ::core::ffi::c_int {
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn checkstring1(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    if (*Gkword).st_workword[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == '\'' as i32
    {
        let mut savework: [::core::ffi::c_char; 60] = [0; 60];
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        Xstrncpy(
            &raw mut savework as *mut ::core::ffi::c_char,
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        Xstrncpy(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            b"e)\0" as *const u8 as *const ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        Xstrncat(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            (&raw mut savework as *mut ::core::ffi::c_char)
                .offset(1 as ::core::ffi::c_int as isize),
            MAXWORDSIZE as size_t,
        );
        n = checkstring2(Gkword);
        if n == 0 {
            let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
            strcpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut savework as *mut ::core::ffi::c_char,
            );
            if hasaccent(&raw mut tmp as *mut ::core::ffi::c_char) != 0 {
                stripacc(&raw mut tmp as *mut ::core::ffi::c_char);
            }
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                b"e)/\0" as *const u8 as *const ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            Xstrncat(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                (&raw mut tmp as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                MAXWORDSIZE as size_t,
            );
            n = checkstring2(Gkword);
        }
        Xstrncpy(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            b"a)\0" as *const u8 as *const ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        Xstrncat(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            (&raw mut savework as *mut ::core::ffi::c_char)
                .offset(1 as ::core::ffi::c_int as isize),
            MAXWORDSIZE as size_t,
        );
        n = checkstring2(Gkword);
        if n == 0 && hasaccent(&raw mut savework as *mut ::core::ffi::c_char) == 0 {
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                b"a)/\0" as *const u8 as *const ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            Xstrncat(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                (&raw mut savework as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                MAXWORDSIZE as size_t,
            );
            n = checkstring2(Gkword);
        }
    } else {
        checkstring2(Gkword);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn checkstring2(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0;
    let mut d: Dialect = 0;
    rval = checkstring3(Gkword);
    if rval == 0 && (*Gkword).gw_totanal == 0 {
        rval = checkcrasis(Gkword);
    }
    if cur_lang() == LATIN {
        return (*Gkword).gw_totanal;
    }
    d = AndDialect((*Gkword).gs_dialect, (HOMERIC | IONIC) as Dialect);
    if d as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        || (*Gkword).gs_dialect as ::core::ffi::c_int & PROSE == 0
    {
        let mut olddial: Dialect = (*Gkword).gs_dialect;
        let mut m: gk_string = gk_string {
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
        let mut m2: gk_string = gk_string {
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
        set_morphflags(&raw mut m, &raw mut (*Gkword).gs_morphflags as *mut MorphFlags);
        set_morphflags(&raw mut m2, &raw mut (*Gkword).gs_morphflags as *mut MorphFlags);
        add_morphflag(&raw mut (*Gkword).gs_morphflags as *mut MorphFlags, UNAUGMENTED);
        add_morphflag(
            &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
            UNAUGMENTED,
        );
        if (*Gkword).gs_dialect as ::core::ffi::c_int & (IONIC | PROSE) == 0 {
            (*Gkword).gs_dialect = ((*Gkword).gs_dialect as ::core::ffi::c_int
                | (0o10 as ::core::ffi::c_int
                    | (0o2000 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int)))
                as Dialect;
        }
        (*Gkword).gs_dialect = ((*Gkword).gs_dialect as ::core::ffi::c_int
            | 0o10 as ::core::ffi::c_int
            | (0o2000 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int)) as Dialect;
        rval = checkstring3(Gkword);
        if rval == 0 && (*Gkword).gw_totanal == 0 {
            rval = checkcrasis(Gkword);
        }
        (*Gkword).gs_dialect = olddial;
        set_gwmorphflags(Gkword, &raw mut m.gs_morphflags as *mut MorphFlags);
        set_morphflags(
            &raw mut (*Gkword).gs_stem,
            &raw mut m2.gs_morphflags as *mut MorphFlags,
        );
        if cur_lang() != LATIN && rval == 0
            && (*Gkword).gs_dialect as ::core::ffi::c_int & (IONIC | PROSE) == 0
        {
            add_morphflag(&raw mut (*Gkword).gs_morphflags as *mut MorphFlags, POETIC);
            add_morphflag(
                &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
                UNAUGMENTED,
            );
            add_morphflag(
                &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
                UNAUGMENTED,
            );
            rval = checkstring3(Gkword);
            if rval == 0 && (*Gkword).gw_totanal == 0 {
                rval = checkcrasis(Gkword);
            }
            (*Gkword).gs_dialect = olddial;
            set_gwmorphflags(Gkword, &raw mut m.gs_morphflags as *mut MorphFlags);
            set_morphflags(
                &raw mut (*Gkword).gs_stem,
                &raw mut m2.gs_morphflags as *mut MorphFlags,
            );
        }
    }
    return (*Gkword).gw_totanal;
}
#[no_mangle]
pub static mut GreekSuff: [enclitic_word; 2] = unsafe {
    [
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"per\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: (NOUNSTEM | ADJSTEM) as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
    ]
};
#[no_mangle]
pub static mut LatinSuff: [enclitic_word; 13] = unsafe {
    [
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"que\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"cumque\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"cunque\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ne\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ve\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"libet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"vis\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"piam\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"dem\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"dum\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"met\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: (PRONOUN | PERS_PRON) as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
    ]
};
#[no_mangle]
pub static mut ItalianSuff: [enclitic_word; 19] = unsafe {
    [
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"glie\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"gli\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"mi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"me\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ci\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ce\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ti\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"te\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"vi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ve\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"lo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"li\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"la\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"le\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"loro\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ne\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"si\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"se\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: PPARTMASK as Stemtype,
        },
        enclitic_word {
            enclitic: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            stemtype: 0 as Stemtype,
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn checkstring3(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut saveword: [::core::ffi::c_char; 60] = [0; 60];
    let mut workword: [::core::ffi::c_char; 60] = [0; 60];
    let _frame = crate::legacy_stack::Frame::enter(128, [(64,(&raw mut saveword).cast(),60),(0,(&raw mut workword).cast(),60)]);
    let mut string: *mut ::core::ffi::c_char = &raw mut (*Gkword).st_workword
        as *mut ::core::ffi::c_char;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut workval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut oldanal: *mut gk_analysis = ::core::ptr::null_mut::<gk_analysis>();
    let mut newanal: *mut gk_analysis = ::core::ptr::null_mut::<gk_analysis>();
    let mut totanal: ::core::ffi::c_int = 0;
    let mut acount: ::core::ffi::c_int = 0;
    let mut idx: ::core::ffi::c_int = 0;
    let mut EnclitArr: *mut enclitic_word = ::core::ptr::null_mut::<enclitic_word>();
    match cur_lang() {
        LATIN => {
            EnclitArr = &raw mut LatinSuff as *mut enclitic_word;
        }
        ITALIAN => {
            EnclitArr = &raw mut ItalianSuff as *mut enclitic_word;
        }
        _ => {
            EnclitArr = &raw mut GreekSuff as *mut enclitic_word;
        }
    }
    Xstrncpy(
        &raw mut saveword as *mut ::core::ffi::c_char,
        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
            as size_t,
    );
    rval = checkstring4(Gkword);
    if ((if 0 as ::core::ffi::c_int != 0 {
        isupper(*string as ::core::ffi::c_int)
    } else {
        ((*string as ::core::ffi::c_uint).wrapping_sub('A' as i32 as ::core::ffi::c_uint)
            < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
    }) != 0 || *string as ::core::ffi::c_int == BETA_UCASE_MARKER)
        && (*Gkword).gs_prntflags as ::core::ffi::c_int & STRICT_CASE == 0
    {
        if cur_lang() == LATIN || cur_lang() == ITALIAN {
            *string = tolower(*string as ::core::ffi::c_int) as ::core::ffi::c_char;
            if *string as ::core::ffi::c_int == 'v' as i32
                && (if 0 as ::core::ffi::c_int != 0 {
                    isalpha(
                        *string.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int,
                    )
                } else {
                    ((*string.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                        .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                        < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
                }) != 0
            {
                *string = 'u' as i32 as ::core::ffi::c_char;
            }
        } else {
            beta_tolower(string);
        }
        rval = checkstring4(Gkword);
        if rval > 0 as ::core::ffi::c_int {
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                &raw mut saveword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            return rval;
        }
        if cur_lang() != GREEK {
            idx = 0 as ::core::ffi::c_int;
            while *string.offset(idx as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                *string.offset(idx as isize) = tolower(
                    *string.offset(idx as isize) as ::core::ffi::c_int,
                ) as ::core::ffi::c_char;
                if *string.offset(idx as isize) as ::core::ffi::c_int == 'v' as i32
                    && strchr(
                            b"aeiou\0" as *const u8 as *const ::core::ffi::c_char,
                            *string.offset((idx + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int,
                        )
                        .is_null()
                {
                    *string.offset(idx as isize) = 'u' as i32 as ::core::ffi::c_char;
                }
                idx += 1;
            }
            rval = checkstring4(Gkword);
            if rval > 0 as ::core::ffi::c_int {
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut saveword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return rval;
            }
            *string = toupper(*string as ::core::ffi::c_int) as ::core::ffi::c_char;
            rval = checkstring4(Gkword);
            if rval > 0 as ::core::ffi::c_int {
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut saveword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return rval;
            }
        }
        Xstrncpy(
            string,
            &raw mut saveword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    }
    if *(&raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char)
        .offset(
            Xstrlen(&raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char) as isize,
        )
        .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int == '\'' as i32
    {
        rval += checkapostr(Gkword);
        if rval != 0 {
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                &raw mut saveword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            return rval;
        }
    }
    while (*Gkword).gw_totanal == 0
        && *(&raw mut (*EnclitArr).enclitic as *mut ::core::ffi::c_char)
            as ::core::ffi::c_int != '\0' as i32
    {
        if cmpend(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            &raw mut (*EnclitArr).enclitic as *mut ::core::ffi::c_char,
            &raw mut workword as *mut ::core::ffi::c_char,
        ) != 0
        {
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            rval += checkstring3(Gkword);
            if (*EnclitArr).stemtype != 0 {
                oldanal = (*Gkword).gw_analysis;
                totanal = 0 as ::core::ffi::c_int;
                newanal = oldanal;
                acount = 0 as ::core::ffi::c_int;
                while acount < (*Gkword).gw_totanal {
                    if (*oldanal).gs_steminfo & (*EnclitArr).stemtype != 0 {
                        let fresh0 = newanal;
                        newanal = newanal.offset(1);
                        *fresh0 = *oldanal;
                        totanal += 1;
                    }
                    acount += 1;
                    oldanal = oldanal.offset(1);
                }
                (*Gkword).gw_totanal = totanal;
            }
            if rval != 0 {
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut saveword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return rval;
            }
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                &raw mut saveword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
        }
        EnclitArr = EnclitArr.offset(1);
    }
    if cur_lang() == LATIN {
        if cmpend(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            b"ast\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut workword as *mut ::core::ffi::c_char,
        ) != 0
            || cmpend(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                b"est\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
            ) != 0
            || cmpend(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                b"umst\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
            ) != 0
            || cmpend(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                b"amst\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
            ) != 0
            || cmpend(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                b"emst\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
            ) != 0
            || cmpend(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                b"omst\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
            ) != 0
        {
            strcpy(
                &raw mut workword as *mut ::core::ffi::c_char,
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            );
            workword[strlen(&raw mut workword as *mut ::core::ffi::c_char)
                .wrapping_sub(2 as size_t) as usize] = 0 as ::core::ffi::c_char;
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            rval = checkstring3(Gkword);
            if rval != 0 {
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut saveword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return rval;
            }
        }
        if cmpend(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            b"ust\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut workword as *mut ::core::ffi::c_char,
        ) != 0
        {
            strcpy(
                &raw mut workword as *mut ::core::ffi::c_char,
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            );
            workword[strlen(&raw mut workword as *mut ::core::ffi::c_char)
                .wrapping_sub(1 as size_t) as usize] = 0 as ::core::ffi::c_char;
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            rval = checkstring3(Gkword);
            if rval != 0 {
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut saveword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return rval;
            }
        }
        if cmpend(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            b"ist\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut workword as *mut ::core::ffi::c_char,
        ) != 0
            || cmpend(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                b"ost\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
            ) != 0
        {
            strcpy(
                &raw mut workword as *mut ::core::ffi::c_char,
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            );
            workword[strlen(&raw mut workword as *mut ::core::ffi::c_char)
                .wrapping_sub(1 as size_t) as usize] = 0 as ::core::ffi::c_char;
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            workval = checkstring3(Gkword);
            if workval != 0 {
                rval += workval;
            }
            workword[strlen(&raw mut workword as *mut ::core::ffi::c_char)
                .wrapping_sub(1 as size_t) as usize] = 0 as ::core::ffi::c_char;
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            workval = checkstring3(Gkword);
            if workval != 0 {
                rval += workval;
            }
            if rval != 0 {
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut saveword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return rval;
            }
        }
        if (*Gkword).gw_totanal == 0 {
            if cmpend(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                b"n\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
            ) != 0
            {
                strcpy(
                    &raw mut workword as *mut ::core::ffi::c_char,
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                );
                workword[strlen(&raw mut workword as *mut ::core::ffi::c_char)
                    .wrapping_sub(1 as size_t) as usize] = 's' as i32
                    as ::core::ffi::c_char;
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut workword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                workval = checkstring3(Gkword);
                if workval != 0 {
                    rval += workval;
                }
                workword[strlen(&raw mut workword as *mut ::core::ffi::c_char)
                    .wrapping_sub(1 as size_t) as usize] = 0 as ::core::ffi::c_char;
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut workword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                workval = checkstring3(Gkword);
                if workval != 0 {
                    rval += workval;
                }
                if rval != 0 {
                    Xstrncpy(
                        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                        &raw mut saveword as *mut ::core::ffi::c_char,
                        MAXWORDSIZE as size_t,
                    );
                    return rval;
                }
            }
        }
    }
    if cur_lang() == LATIN {
        let mut a: *mut ::core::ffi::c_char = &raw mut workword
            as *mut ::core::ffi::c_char;
        strcpy(
            &raw mut workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
        );
        if u2v(&raw mut workword as *mut ::core::ffi::c_char) != 0 {
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            rval = checkstring3(Gkword);
            if rval != 0 {
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut saveword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return rval;
            }
        }
        strcpy(
            &raw mut workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
        );
    } else if cur_lang() == ITALIAN && (*Gkword).gw_totanal == 0 {
        let mut a_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        strcpy(
            &raw mut workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
        );
        a_0 = &raw mut workword as *mut ::core::ffi::c_char;
        acount = 0 as ::core::ffi::c_int;
        while *a_0 as ::core::ffi::c_int != '\0' as i32 {
            if *a_0 as ::core::ffi::c_int == 'U' as i32 {
                *a_0 = 'V' as i32 as ::core::ffi::c_char;
                acount += 1;
            } else if *a_0 as ::core::ffi::c_int == 'u' as i32 {
                *a_0 = 'v' as i32 as ::core::ffi::c_char;
                acount += 1;
            }
            a_0 = a_0.offset(1);
        }
        if acount != 0 {
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            rval += checkstring3(Gkword);
            if rval != 0 {
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut saveword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return rval;
            }
        }
        strcpy(
            &raw mut workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
        );
    }
    if cur_lang() == LATIN {
        let mut a_1: *mut ::core::ffi::c_char = &raw mut workword
            as *mut ::core::ffi::c_char;
        strcpy(
            &raw mut workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
        );
        if *a_1 as ::core::ffi::c_int == 'I' as i32 {
            *a_1 = 'J' as i32 as ::core::ffi::c_char;
            Xstrncpy(
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            rval = checkstring3(Gkword);
            if rval != 0 {
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut saveword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return rval;
            }
        }
        while *a_1 != 0 {
            if *a_1 as ::core::ffi::c_int == 'i' as i32
                && *a_1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 0
                && !strchr(
                        b"aeiou\0" as *const u8 as *const ::core::ffi::c_char,
                        *a_1.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int,
                    )
                    .is_null()
            {
                *a_1 = 'j' as i32 as ::core::ffi::c_char;
                Xstrncpy(
                    &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                    &raw mut workword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                rval = checkstring3(Gkword);
                if rval != 0 {
                    Xstrncpy(
                        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                        &raw mut saveword as *mut ::core::ffi::c_char,
                        MAXWORDSIZE as size_t,
                    );
                    return rval;
                } else {
                    *a_1 = 'i' as i32 as ::core::ffi::c_char;
                }
            }
            a_1 = a_1.offset(1);
        }
    }
    if cur_lang() == LATIN {
        if strncmp(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            b"ex\0" as *const u8 as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            let mut p_word: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut p_tail: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            let mut p_start: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            strcpy(
                &raw mut workword as *mut ::core::ffi::c_char,
                &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            );
            p_word = &raw mut workword as *mut ::core::ffi::c_char;
            p_word = p_word.offset(2 as ::core::ffi::c_int as isize);
            match *p_word as ::core::ffi::c_int {
                99 | 112 | 116 => {
                    p_start = p_word;
                    p_tail = p_word.offset(1 as ::core::ffi::c_int as isize);
                    memmove(
                        p_tail as *mut ::core::ffi::c_void,
                        p_word as *const ::core::ffi::c_void,
                        strlen(p_word),
                    );
                    *p_start = 's' as i32 as ::core::ffi::c_char;
                    Xstrncpy(
                        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                        &raw mut workword as *mut ::core::ffi::c_char,
                        MAXWORDSIZE as size_t,
                    );
                    rval = checkstring3(Gkword);
                    if rval != 0 {
                        Xstrncpy(
                            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
                            &raw mut saveword as *mut ::core::ffi::c_char,
                            MAXWORDSIZE as size_t,
                        );
                        return rval;
                    }
                }
                _ => {}
            }
        }
    }
    Xstrncpy(
        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
        &raw mut saveword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    return rval;
}
unsafe extern "C" fn checkstring4(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut saveword: [::core::ffi::c_char; 60] = [0; 60];
    let mut wordnoacc: [::core::ffi::c_char; 60] = [0; 60];
    let mut workword: [::core::ffi::c_char; 60] = [0; 60];
    let _frame = crate::legacy_stack::Frame::enter(192, [(128,(&raw mut saveword).cast(),60),(64,(&raw mut wordnoacc).cast(),60),(0,(&raw mut workword).cast(),60)]);
    let mut a: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut string: *mut ::core::ffi::c_char = &raw mut (*Gkword).st_workword
        as *mut ::core::ffi::c_char;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rval = checkword(Gkword);
    if rval > 0 as ::core::ffi::c_int {
        return rval;
    }
    Xstrncpy(
        &raw mut workword as *mut ::core::ffi::c_char,
        string,
        MAXWORDSIZE as size_t,
    );
    Xstrncpy(
        &raw mut saveword as *mut ::core::ffi::c_char,
        &raw mut workword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    Xstrncpy(
        &raw mut wordnoacc as *mut ::core::ffi::c_char,
        &raw mut workword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    stripacc(&raw mut wordnoacc as *mut ::core::ffi::c_char);
    if cur_lang() == GREEK {
        if has_cun(&raw mut workword as *mut ::core::ffi::c_char) != 0 {
            Xstrncpy(
                string,
                &raw mut workword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            rval += checkindecl(Gkword);
            rval += checknom(Gkword);
            Xstrncpy(
                string,
                &raw mut saveword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            if rval > 0 as ::core::ffi::c_int {
                return rval;
            }
        }
        Xstrncpy(
            &raw mut workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        if has_tt(&raw mut workword as *mut ::core::ffi::c_char) != 0 {
            Xstrncpy(
                string,
                &raw mut workword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            rval = checkstring4(Gkword);
            Xstrncpy(
                string,
                &raw mut saveword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            if rval > 0 as ::core::ffi::c_int {
                return rval;
            }
        }
        Xstrncpy(
            &raw mut workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn has_cun(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    while *s != 0 {
        if *s as ::core::ffi::c_int == 'c' as i32
            && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'u' as i32
        {
            *s = 's' as i32 as ::core::ffi::c_char;
            return 1 as ::core::ffi::c_int;
        }
        s = s.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkapostr(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut saveword: [::core::ffi::c_char; 60] = [0; 60];
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
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut curval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut num_sylls: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    Xstrncpy(
        &raw mut saveword as *mut ::core::ffi::c_char,
        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    p = &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char;
    while *p != 0 {
        p = p.offset(1);
    }
    p = p.offset(-1);
    if *p as ::core::ffi::c_int != '\'' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    if p > &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char
        && (*p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'q' as i32
            || *p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'x' as i32
            || *p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'f' as i32)
    {
        if *p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'q' as i32
        {
            *p.offset(-(1 as ::core::ffi::c_int as isize)) = 't' as i32
                as ::core::ffi::c_char;
            if *p.offset(-(2 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'x' as i32
            {
                *p.offset(-(2 as ::core::ffi::c_int as isize)) = 'k' as i32
                    as ::core::ffi::c_char;
            }
        } else if *p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'x' as i32
        {
            *p.offset(-(1 as ::core::ffi::c_int as isize)) = 'k' as i32
                as ::core::ffi::c_char;
        } else {
            *p.offset(-(1 as ::core::ffi::c_int as isize)) = 'p' as i32
                as ::core::ffi::c_char;
        }
        rval += checkapostr(Gkword);
        Xstrncpy(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    }
    num_sylls = nsylls(&raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char);
    if num_sylls >= 1 as ::core::ffi::c_int {
        add_apostrvowel(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            p,
            b"a\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        curval = checkstring3(Gkword);
        Xstrncpy(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        rval += curval;
    }
    if num_sylls >= 1 as ::core::ffi::c_int {
        add_apostrvowel(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            p,
            b"i\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        curval = checkstring3(Gkword);
        Xstrncpy(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        rval += curval;
    }
    if num_sylls >= 1 as ::core::ffi::c_int {
        add_apostrvowel(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            p,
            b"o\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        curval = checkstring3(Gkword);
        Xstrncpy(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        rval += curval;
    }
    add_apostrvowel(
        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
        p,
        b"e\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    curval = checkstring3(Gkword);
    Xstrncpy(
        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
        &raw mut saveword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    rval += curval;
    if num_sylls >= 1 as ::core::ffi::c_int {
        add_apostrvowel(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            p,
            b"ai\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        set_morphflags(
            &raw mut TmpGstr,
            &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
        );
        add_morphflag(&raw mut (*Gkword).gs_morphflags as *mut MorphFlags, POETIC);
        curval = checkstring3(Gkword);
        Xstrncpy(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        rval += curval;
        set_gwmorphflags(Gkword, &raw mut TmpGstr.gs_morphflags as *mut MorphFlags);
    }
    if rval == 0 {
        let mut syllno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut accnum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        Xstrncpy(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        checkaccent(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            &raw mut syllno,
            &raw mut accnum,
        );
        if syllno == ULTIMA && accnum != C_ERR as ::core::ffi::c_int {
            stripacc(&raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char);
            rval = checkapostr(Gkword);
        }
        Xstrncpy(
            &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
            &raw mut saveword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    }
    return rval;
}
unsafe extern "C" fn add_apostrvowel(
    mut word: *mut ::core::ffi::c_char,
    mut end: *mut ::core::ffi::c_char,
    mut vow: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    Xstrncpy(end, vow, MAXWORDSIZE as size_t);
    if naccents(word) == 0 as ::core::ffi::c_int {
        Xstrncat(
            word,
            b"/\0" as *const u8 as *const ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    }
    if *end as ::core::ffi::c_int == 'u' as i32
        || *end as ::core::ffi::c_int == 'i' as i32
        || *end as ::core::ffi::c_int == 'a' as i32
    {
        Xstrncat(
            word,
            b"^\0" as *const u8 as *const ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn has_tt(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    while *s != 0 {
        if *s as ::core::ffi::c_int == 't' as i32
            && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 't' as i32
        {
            let ref mut fresh5 = *s.offset(1 as ::core::ffi::c_int as isize);
            *fresh5 = 's' as i32 as ::core::ffi::c_char;
            *s = *fresh5;
            return 1 as ::core::ffi::c_int;
        }
        s = s.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setepic() -> ::core::ffi::c_int {
    AddWantDialect((NON_HOMERIC_EPIC | HOMERIC) as Dialect);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn setatticprose() -> ::core::ffi::c_int {
    SetWantDialect((ATTIC | PROSE) as Dialect);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn SetWantDialect(mut dial: Dialect) -> ::core::ffi::c_int {
    WantDialects = dial;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn AddWantDialect(mut dial: Dialect) -> ::core::ffi::c_int {
    WantDialects = (WantDialects as ::core::ffi::c_int | dial as ::core::ffi::c_int)
        as Dialect;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn ZapWantDialect(mut dial: Dialect) -> ::core::ffi::c_int {
    WantDialects = (WantDialects as ::core::ffi::c_int & !(dial as ::core::ffi::c_int))
        as Dialect;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn GetWantDialect() -> Dialect {
    return WantDialects;
}
#[no_mangle]
pub unsafe extern "C" fn updateDialect(mut dial: Dialect) -> ::core::ffi::c_int {
    let mut curdial: Dialect = 0;
    curdial = GetWantDialect();
    if dial as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        SetWantDialect(dial);
        return 0 as ::core::ffi::c_int;
    } else if dial as ::core::ffi::c_int & curdial as ::core::ffi::c_int != 0 {
        ZapWantDialect(dial);
        return -(1 as ::core::ffi::c_int);
    } else {
        AddWantDialect(dial);
        return 1 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn u2v(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut nchanges: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut half1: [::core::ffi::c_char; 1024] = [0; 1024];
    let _frame = crate::legacy_stack::Frame::enter(1024, [(0,(&raw mut half1).cast(),1024)]);
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    half1[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    t = &raw mut half1 as *mut ::core::ffi::c_char;
    if (*s as ::core::ffi::c_int == 'U' as i32 || *s as ::core::ffi::c_int == 'u' as i32)
        && !strchr(
                b"aeiouAEIOU\0" as *const u8 as *const ::core::ffi::c_char,
                *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            )
            .is_null()
    {
        if *s as ::core::ffi::c_int == 'U' as i32 {
            *s = 'V' as i32 as ::core::ffi::c_char;
        }
        if *s as ::core::ffi::c_int == 'u' as i32 {
            *s = 'v' as i32 as ::core::ffi::c_char;
        }
        nchanges += 1;
    }
    let fresh1 = s;
    s = s.offset(1);
    let fresh2 = t;
    t = t.offset(1);
    *fresh2 = *fresh1;
    *t = 0 as ::core::ffi::c_char;
    while *s != 0 {
        if !strchr(
                b"aeiouAEIOU\0" as *const u8 as *const ::core::ffi::c_char,
                *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int,
            )
            .is_null()
            && !strchr(
                    b"aeiouAEIOU\0" as *const u8 as *const ::core::ffi::c_char,
                    *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                )
                .is_null() && *s as ::core::ffi::c_int == 'u' as i32
        {
            *s = 'v' as i32 as ::core::ffi::c_char;
            nchanges += 1;
        }
        if is_rawpreverb(&raw mut half1 as *mut ::core::ffi::c_char) != 0
            && *s as ::core::ffi::c_int == 'u' as i32
            && !strchr(
                    b"aeiouAEIOU\0" as *const u8 as *const ::core::ffi::c_char,
                    *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                )
                .is_null()
        {
            *s = 'v' as i32 as ::core::ffi::c_char;
            nchanges += 1;
        }
        if *s as ::core::ffi::c_int == 'u' as i32
            && chckend(s.offset(1 as ::core::ffi::c_int as isize)) != 0
            && !strchr(
                    b"aeiouAEIOU\0" as *const u8 as *const ::core::ffi::c_char,
                    *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                )
                .is_null()
        {
            *s = 'v' as i32 as ::core::ffi::c_char;
            nchanges += 1;
        }
        let fresh3 = s;
        s = s.offset(1);
        let fresh4 = t;
        t = t.offset(1);
        *fresh4 = *fresh3;
        *t = 0 as ::core::ffi::c_char;
    }
    return nchanges;
}
