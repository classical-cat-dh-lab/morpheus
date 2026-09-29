use ::c2rust_bitfields;
extern "C" {
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn GenStemForms(
        _: *mut gk_word,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut gk_word;
    fn CheckGenWords(_: *mut gk_word, _: *mut gk_word) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
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
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn is_substring(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn nsylls(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn poss_thirdmono(
        _: Stemtype,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn subchar(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn zap_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type int32 = ::core::ffi::c_uint;
pub type size_t = usize;
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
pub const SYLL_AUGMENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SUFF_ACC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn checkdict(
    mut Gkword: *mut gk_word,
    mut stem: *mut gk_string,
    mut stemkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut hits: ::core::ffi::c_int = 0;
    let mut gkforms: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    let mut SaveGkword: gk_word = gk_word {
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
    let mut prevb: *mut ::core::ffi::c_char = &raw mut (*Gkword).gs_preverb.gs_gkstring
        as *mut ::core::ffi::c_char;
    let mut pbptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut curkeys: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut keyp: [::core::ffi::c_char; 1025] = [0; 1025];
    hits = 0 as ::core::ffi::c_int;
    SaveGkword = *Gkword;
    Xstrncpy(
        &raw mut curkeys as *mut ::core::ffi::c_char,
        stemkeys,
        LONGSTRING as size_t,
    );
    (*Gkword).gs_stem = *stem;
    if has_morphflag(&raw mut (*stem).gs_morphflags as *mut MorphFlags, SYLL_AUGMENT)
        != 0
    {
        zap_morphflag(
            &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
            SYLL_AUGMENT,
        );
        add_morphflag(&raw mut (*Gkword).gs_morphflags as *mut MorphFlags, SYLL_AUGMENT);
    } else {
        zap_morphflag(&raw mut (*Gkword).gs_morphflags as *mut MorphFlags, SYLL_AUGMENT);
    }
    strcpy(
        &raw mut keyp as *mut ::core::ffi::c_char,
        GetLemmStem(
            &raw mut curkeys as *mut ::core::ffi::c_char,
            &raw mut (*Gkword).st_lemma as *mut ::core::ffi::c_char,
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
        ),
    );
    pbptr = is_substring(
        b"pb:\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut keyp as *mut ::core::ffi::c_char,
    );
    if !pbptr.is_null() && pbptr > &raw mut keyp as *mut ::core::ffi::c_char
        && *pbptr.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'r' as i32
    {
        pbptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !pbptr.is_null() {
        if *prevb.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *prevb.offset(0 as ::core::ffi::c_int as isize) != 0 {
        if pbptr.is_null() {
            Xstrncat(
                &raw mut keyp as *mut ::core::ffi::c_char,
                b":pb:\0" as *const u8 as *const ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            Xstrncat(
                &raw mut keyp as *mut ::core::ffi::c_char,
                prevb,
                LONGSTRING as size_t,
            );
            current_block = 6669252993407410313;
        } else {
            let mut tmppb: [::core::ffi::c_char; 1024] = [0; 1024];
            strcpy(
                &raw mut tmppb as *mut ::core::ffi::c_char,
                pbptr.offset(3 as ::core::ffi::c_int as isize),
            );
            if *(&raw mut tmppb as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut tmppb as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == ':' as i32
            {
                *(&raw mut tmppb as *mut ::core::ffi::c_char)
                    .offset(Xstrlen(&raw mut tmppb as *mut ::core::ffi::c_char) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 0
                    as ::core::ffi::c_char;
            }
            if Xstrncmp(
                &raw mut tmppb as *mut ::core::ffi::c_char,
                prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(
                        -(Xstrlen(&raw mut tmppb as *mut ::core::ffi::c_char) as isize),
                    ),
                Xstrlen(prevb) as size_t,
            ) != 0
            {
                current_block = 8834666743744267323;
            } else {
                Xstrncat(
                    &raw mut keyp as *mut ::core::ffi::c_char,
                    b":pb:\0" as *const u8 as *const ::core::ffi::c_char,
                    LONGSTRING as size_t,
                );
                Xstrncat(
                    &raw mut keyp as *mut ::core::ffi::c_char,
                    prevb,
                    LONGSTRING as size_t,
                );
                current_block = 6669252993407410313;
            }
        }
    } else {
        current_block = 6669252993407410313;
    }
    match current_block {
        6669252993407410313 => {
            subchar(&raw mut keyp as *mut ::core::ffi::c_char, ':' as i32, ' ' as i32);
            gkforms = GenStemForms(
                Gkword,
                &raw mut keyp as *mut ::core::ffi::c_char,
                ANALYSIS,
            );
            if gkforms.is_null() {
                if has_morphflag(
                    &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
                    SUFF_ACC,
                ) != 0
                    || nsylls(
                        &raw mut (*Gkword).gs_stem.gs_gkstring
                            as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || poss_thirdmono(
                        (*Gkword).gs_steminfo,
                        &raw mut (*Gkword).gs_stem.gs_gkstring
                            as *mut ::core::ffi::c_char,
                        &raw mut (*Gkword).gs_endstring.gs_gkstring
                            as *mut ::core::ffi::c_char,
                    ) != 0
                {
                    stripacc(
                        &raw mut (*Gkword).gs_endstring.gs_gkstring
                            as *mut ::core::ffi::c_char,
                    );
                    gkforms = GenStemForms(
                        Gkword,
                        &raw mut keyp as *mut ::core::ffi::c_char,
                        ANALYSIS,
                    );
                }
            }
            if gkforms.is_null() {
                *Gkword = SaveGkword;
                return 0 as ::core::ffi::c_int;
            }
            hits += CheckGenWords(Gkword, gkforms);
        }
        _ => {}
    }
    (*Gkword).gs_preverb = SaveGkword.gs_preverb;
    (*Gkword).gs_aug1 = SaveGkword.gs_aug1;
    (*Gkword).gs_stem = SaveGkword.gs_stem;
    (*Gkword).gs_endstring = SaveGkword.gs_endstring;
    if !gkforms.is_null() {
        FreeGkString(gkforms as *mut gk_string);
        gkforms = ::core::ptr::null_mut::<gk_word>();
    }
    return hits;
}
#[no_mangle]
pub unsafe extern "C" fn GetLemmStem(
    mut keys: *mut ::core::ffi::c_char,
    mut lemma: *mut ::core::ffi::c_char,
    mut stem: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut a: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *keys as ::core::ffi::c_int == ':' as i32 {
        keys = keys.offset(1);
    }
    a = lemma;
    while *keys as ::core::ffi::c_int != 0 && *keys as ::core::ffi::c_int != ':' as i32 {
        let fresh0 = keys;
        keys = keys.offset(1);
        let fresh1 = a;
        a = a.offset(1);
        *fresh1 = *fresh0;
    }
    *a = 0 as ::core::ffi::c_char;
    if *keys as ::core::ffi::c_int == ':' as i32 {
        keys = keys.offset(1);
    }
    a = stem;
    while *keys as ::core::ffi::c_int != 0 && *keys as ::core::ffi::c_int != ':' as i32 {
        let fresh2 = keys;
        keys = keys.offset(1);
        let fresh3 = a;
        a = a.offset(1);
        *fresh3 = *fresh2;
    }
    *a = 0 as ::core::ffi::c_char;
    if *keys as ::core::ffi::c_int == ':' as i32 {
        keys = keys.offset(1);
    }
    return keys;
}
pub const ANALYSIS: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
