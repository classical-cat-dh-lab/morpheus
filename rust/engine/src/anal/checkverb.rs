use ::c2rust_bitfields;
extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn CpGkAnal(_: *mut gk_word, _: *mut gk_word) -> ::core::ffi::c_int;
    fn chckvend(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strippreverb(
        _: *mut gk_word,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn stripquant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn try_irregvb(_: *mut gk_word) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type int32 = ::core::ffi::c_uint;
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
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn checkverb(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    #[cfg(feature = "trace")] crate::trace::word("checkverb:Gkword", Gkword as *const _);

    let mut wp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut a1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut workword: [::core::ffi::c_char; 60] = [0; 60];
    let mut half1: [::core::ffi::c_char; 60] = [0; 60];
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    Xstrncpy(
        &raw mut workword as *mut ::core::ffi::c_char,
        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    wp = &raw mut workword as *mut ::core::ffi::c_char;
    a1 = &raw mut half1 as *mut ::core::ffi::c_char;
    *a1 = 0 as ::core::ffi::c_char;
    while *wp != 0 {
        Xstrncpy(
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            wp,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        Xstrncpy(
            &raw mut (*Gkword).st_rawprvb as *mut ::core::ffi::c_char,
            &raw mut half1 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        rval += try_irregvb(Gkword);
        Xstrncpy(
            &raw mut (*Gkword).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        Xstrncpy(
            &raw mut (*Gkword).st_rawprvb as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        Xstrncpy(
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut half1 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        Xstrncpy(
            &raw mut (*Gkword).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
            wp,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        rval += analyzed_verb(Gkword);
        let fresh0 = wp;
        wp = wp.offset(1);
        let fresh1 = a1;
        a1 = a1.offset(1);
        *fresh1 = *fresh0;
        *a1 = 0 as ::core::ffi::c_char;
        while *wp as ::core::ffi::c_int != 0
            && (if 0 as ::core::ffi::c_int != 0 {
                isalpha(*wp as ::core::ffi::c_int)
            } else {
                ((*wp as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
            }) == 0 && *wp as ::core::ffi::c_int != '|' as i32
            && *wp as ::core::ffi::c_int != '(' as i32
            && *wp as ::core::ffi::c_int != ')' as i32
        {
            wp = wp.offset(1);
        }
    }
    if rval == 0 {
        Xstrncpy(
            &raw mut (*Gkword).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        Xstrncpy(
            &raw mut (*Gkword).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
            b"*\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        Xstrncpy(
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut workword as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        rval += analyzed_verb(Gkword);
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn analyzed_verb(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut tmpendstring: [::core::ffi::c_char; 60] = [0; 60];
    let mut endkeys: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut rval: ::core::ffi::c_int = 0;
    *(&raw mut endkeys as *mut ::core::ffi::c_char) = 0 as ::core::ffi::c_char;
    Xstrncpy(
        &raw mut tmpendstring as *mut ::core::ffi::c_char,
        &raw mut (*Gkword).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    stripacc(&raw mut tmpendstring as *mut ::core::ffi::c_char);
    stripquant(&raw mut tmpendstring as *mut ::core::ffi::c_char);
    rval = chckvend(
        &raw mut tmpendstring as *mut ::core::ffi::c_char,
        &raw mut endkeys as *mut ::core::ffi::c_char,
    );
    if rval != 0 {
        let mut TmpGkword: gk_word = gk_word {
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
        let mut preverbs: [::core::ffi::c_char; 60] = [0; 60];
        TmpGkword = *Gkword;
        stripacc(&raw mut TmpGkword.gs_stem.gs_gkstring as *mut ::core::ffi::c_char);
        rval = strippreverb(
            &raw mut TmpGkword,
            &raw mut endkeys as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        if rval != 0 {
            CpGkAnal(Gkword, &raw mut TmpGkword);
        }
    }
    return rval;
}
