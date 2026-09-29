use ::c2rust_bitfields;
extern "C" {
    fn tolower(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn CpGkAnal(_: *mut gk_word, _: *mut gk_word) -> ::core::ffi::c_int;
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
    fn checkstring3(_: *mut gk_word) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poss_crasis {
    pub mungedword: [::core::ffi::c_char; 12],
    pub wordstart: [::core::ffi::c_char; 12],
    pub preword: [::core::ffi::c_char; 12],
    pub possdial: Dialect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct precise_crasis {
    pub crasis: [::core::ffi::c_char; 60],
    pub curstring: [::core::ffi::c_char; 60],
    pub w_gender: ::core::ffi::c_int,
    pub w_case: ::core::ffi::c_int,
    pub w_number: ::core::ffi::c_int,
    pub possdial: Dialect,
}
pub const IONIC: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const HOMERIC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const DORIC: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const NON_HOMERIC_EPIC: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const EPIC: ::core::ffi::c_int = NON_HOMERIC_EPIC | HOMERIC;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const SINGULAR: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const PLURAL: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const NOMINATIVE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const ACCUSATIVE: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const VOCATIVE: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const MASCULINE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const FEMININE: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const NEUTER: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
#[no_mangle]
pub static mut CrasTab: [precise_crasis; 15] = unsafe {
    [
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"o(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ai(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: FEMININE,
            w_case: NOMINATIVE,
            w_number: PLURAL,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"o(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"oi(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: MASCULINE,
            w_case: NOMINATIVE,
            w_number: PLURAL,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"o(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"o(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: MASCULINE,
            w_case: NOMINATIVE,
            w_number: SINGULAR,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"o(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"h(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: FEMININE,
            w_case: NOMINATIVE,
            w_number: SINGULAR,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"to/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"to/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: NEUTER,
            w_case: NOMINATIVE,
            w_number: SINGULAR,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"to/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"to/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: NEUTER,
            w_case: VOCATIVE,
            w_number: SINGULAR,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"to/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"to/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: NEUTER,
            w_case: ACCUSATIVE,
            w_number: SINGULAR,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"to/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ta/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: NEUTER,
            w_case: NOMINATIVE,
            w_number: PLURAL,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"to/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ta/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: NEUTER,
            w_case: VOCATIVE,
            w_number: PLURAL,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"to/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ta/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: NEUTER,
            w_case: ACCUSATIVE,
            w_number: PLURAL,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ta/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ta/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: NEUTER,
            w_case: NOMINATIVE,
            w_number: PLURAL,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ta/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ta/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: NEUTER,
            w_case: VOCATIVE,
            w_number: PLURAL,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ta/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ta/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: NEUTER,
            w_case: ACCUSATIVE,
            w_number: PLURAL,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ta/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ta/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: 0 as ::core::ffi::c_int,
            w_case: 0 as ::core::ffi::c_int,
            w_number: 0 as ::core::ffi::c_int,
            possdial: 0 as Dialect,
        },
        precise_crasis {
            crasis: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"to/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            curstring: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"to/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            w_gender: 0 as ::core::ffi::c_int,
            w_case: 0 as ::core::ffi::c_int,
            w_number: 0 as ::core::ffi::c_int,
            possdial: 0 as Dialect,
        },
    ]
};
#[no_mangle]
pub static mut PossCras: [poss_crasis; 131] = unsafe {
    [
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*)=w\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*)/a\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*)=w\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*)e\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ka)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*)a\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(=\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(\0\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(\0\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ei(\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ei)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au(\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"dau)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"de/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"dh)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"dh/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"da)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"dh/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"hu(\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eu)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qat\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(t\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qa)/\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qoi)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qai)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qou(\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qh)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"th=|\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qh)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qh)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qa)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"th=|\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qa)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qa)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qa/t\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(/t\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ai)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: (DORIC | EPIC) as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kei)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ei)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kei)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ei)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"keu)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eu)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ka)=|\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ei)=\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ka)|\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ai)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta)|\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ai)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ka)=\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ka)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"katta\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kata/\0\0\0\0\0\0\0"),
            possdial: DORIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kadd\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"d\0\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kata/\0\0\0\0\0\0\0"),
            possdial: EPIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ka)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ou)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kau)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"khu)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"hu)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"moi)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"oi)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"mou\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ma)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"mh/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"mh)/\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"mh/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"mou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"mou\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ou(\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ou(\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ou(\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ou(\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"qw)/\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"pottw/s\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw/s\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"poti/\0\0\0\0\0\0\0"),
            possdial: DORIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta)/|\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ai)/\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta)=\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta)/\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*)/a\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"th)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"th=|\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tau)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tau)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tu/xa)gaqh=|"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)gaqh=|\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tu/xh|\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tu/xa)gaqh|\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)gaqh=|\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tu/xh|\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*)a\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)=\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"toi/\0\0\0\0\0\0\0\0"),
            possdial: DORIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw=|\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"sou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"sou\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"sou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"sou\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"sou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"sou\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou)=\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou)=\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o)/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw=|\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ou)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)u\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta/\0\0\0\0\0\0\0\0\0"),
            possdial: IONIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)u\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: IONIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)u\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: IONIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"twu)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ta/\0\0\0\0\0\0\0\0\0"),
            possdial: IONIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"twu)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: IONIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"twu)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tou=\0\0\0\0\0\0\0\0"),
            possdial: IONIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: IONIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"toi/\0\0\0\0\0\0\0\0"),
            possdial: DORIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"tau)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"to/\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ou(\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"prou)\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"proe\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"sumprou\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"sumproe\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"cumprou\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"cumproe\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)ntiprou\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)ntiproe\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"prou\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"proe\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"prou\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"proo\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"prwu\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"proau\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)gw)=i\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"oi)=\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)gw/\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)gw)=|\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"oi)=\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)gw/\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)gw=|\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"oi)=\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)gw/\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)mou)\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)moi\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kh)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"E)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kh)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xh)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xh(\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xa)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: DORIC as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xa)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xai)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ai(\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xoi)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"oi(\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xau)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au(\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xou)\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ou(\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xu)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xw(\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"xw)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"kai/\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(n\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)n\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(/n\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)/n\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"oi(\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(u\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"wu)\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(u\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"au)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"oi(\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=gaqe\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)gaqe/\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)|\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"oi)\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o)/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)\0\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(=\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"oi(\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)/\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)/\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
    ]
};
#[no_mangle]
pub static mut LatSync: [poss_crasis; 18] = unsafe {
    [
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"cognor\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"cognover\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ignor\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ignover\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"cognoss\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"cognoviss\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"nosse\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"novisse\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"copt\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"coopt\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"der\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"deer\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"des\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"dees\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"abin\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"abis\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ne\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"adeon\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"adeo\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ne\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ain\0\0\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ais\0\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ne\0\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eccam\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ecce\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eam\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eccum\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ecce\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eum\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eccas\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ecce\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eas\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eccos\0\0\0\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ecce\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eos\0\0\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eccillum\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ecce\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"illum\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eccillam\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ecce\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"illam\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eccistum\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ecce\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"istum\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
        poss_crasis {
            mungedword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"eccistam\0\0\0\0"),
            wordstart: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"ecce\0\0\0\0\0\0\0\0"),
            preword: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"istam\0\0\0\0\0\0\0"),
            possdial: 0 as ::core::ffi::c_int as Dialect,
        },
    ]
};
#[no_mangle]
pub static mut nocrasis: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn checkcrasis(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut saveword: [::core::ffi::c_char; 60] = [0; 60];
    let mut string: *mut ::core::ffi::c_char = &raw mut (*Gkword).st_workword
        as *mut ::core::ffi::c_char;
    let mut mungedword: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if nocrasis != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if cur_lang() == LATIN {
        Xstrncpy(
            &raw mut saveword as *mut ::core::ffi::c_char,
            string,
            MAXWORDSIZE as size_t,
        );
        i = 0 as ::core::ffi::c_int;
        while i < MAXWORDSIZE {
            if *string.offset(i as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                break;
            }
            *string.offset(i as isize) = tolower(
                *string.offset(i as isize) as ::core::ffi::c_int,
            ) as ::core::ffi::c_char;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[poss_crasis; 18]>() as usize)
                .wrapping_div(::core::mem::size_of::<poss_crasis>() as usize)
        {
            mungedword = &raw mut (*(&raw mut LatSync as *mut poss_crasis)
                .offset(i as isize))
                .mungedword as *mut ::core::ffi::c_char;
            if *string as ::core::ffi::c_int == *mungedword as ::core::ffi::c_int
                && Xstrncmp(mungedword, string, Xstrlen(mungedword) as size_t) == 0
            {
                rval
                    += testcrasis(
                        Gkword,
                        mungedword,
                        &raw mut (*(&raw mut LatSync as *mut poss_crasis)
                            .offset(i as isize))
                            .wordstart as *mut ::core::ffi::c_char,
                        &raw mut (*(&raw mut LatSync as *mut poss_crasis)
                            .offset(i as isize))
                            .preword as *mut ::core::ffi::c_char,
                        LatSync[i as usize].possdial,
                    );
            }
            i += 1;
        }
        return rval;
    }
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[poss_crasis; 131]>() as usize)
            .wrapping_div(::core::mem::size_of::<poss_crasis>() as usize)
    {
        mungedword = &raw mut (*(&raw mut PossCras as *mut poss_crasis)
            .offset(i as isize))
            .mungedword as *mut ::core::ffi::c_char;
        if *string as ::core::ffi::c_int == *mungedword as ::core::ffi::c_int
            && Xstrncmp(mungedword, string, Xstrlen(mungedword) as size_t) == 0
        {
            rval
                += testcrasis(
                    Gkword,
                    mungedword,
                    &raw mut (*(&raw mut PossCras as *mut poss_crasis)
                        .offset(i as isize))
                        .wordstart as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut PossCras as *mut poss_crasis)
                        .offset(i as isize))
                        .preword as *mut ::core::ffi::c_char,
                    PossCras[i as usize].possdial,
                );
        }
        i += 1;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn set_nocrasis() -> ::core::ffi::c_int {
    nocrasis = 1 as ::core::ffi::c_int;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn testcrasis(
    mut Gkword: *mut gk_word,
    mut mungedword: *mut ::core::ffi::c_char,
    mut wordstart: *mut ::core::ffi::c_char,
    mut preword: *mut ::core::ffi::c_char,
    mut possdial: Dialect,
) -> ::core::ffi::c_int {
    let mut saveword: [::core::ffi::c_char; 60] = [0; 60];
    let mut word1: [::core::ffi::c_char; 60] = [0; 60];
    let mut word2: [::core::ffi::c_char; 60] = [0; 60];
    let mut olddial: Dialect = 0 as Dialect;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
    tmpGkword = *Gkword;
    Xstrncpy(
        &raw mut saveword as *mut ::core::ffi::c_char,
        &raw mut tmpGkword.st_workword as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
            as size_t,
    );
    Xstrncpy(
        &raw mut tmpGkword.st_workword as *mut ::core::ffi::c_char,
        wordstart,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
            as size_t,
    );
    Xstrncat(
        &raw mut tmpGkword.st_workword as *mut ::core::ffi::c_char,
        (&raw mut saveword as *mut ::core::ffi::c_char)
            .offset(Xstrlen(mungedword) as isize),
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
            as size_t,
    );
    Xstrncpy(
        &raw mut tmpGkword.st_crasis as *mut ::core::ffi::c_char,
        preword,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    olddial = tmpGkword.gs_dialect;
    tmpGkword.gs_dialect = (tmpGkword.gs_dialect as ::core::ffi::c_int
        | possdial as ::core::ffi::c_int) as Dialect;
    tmpGkword.gs_stem.gs_dialect = (tmpGkword.gs_stem.gs_dialect as ::core::ffi::c_int
        | possdial as ::core::ffi::c_int) as Dialect;
    rval = checkstring3(&raw mut tmpGkword);
    if rval != 0 {
        let mut curanal: *mut gk_analysis = tmpGkword.gw_analysis;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nanals: ::core::ffi::c_int = 0;
        nanals = tmpGkword.gw_totanal;
        i = 0 as ::core::ffi::c_int;
        while i < nanals {
            curanal = tmpGkword.gw_analysis.offset(i as isize);
            (*curanal).gs_dialect = ((*curanal).gs_dialect as ::core::ffi::c_int
                | possdial as ::core::ffi::c_int) as Dialect;
            i += 1;
        }
        CpGkAnal(Gkword, &raw mut tmpGkword);
    }
    tmpGkword.gs_dialect = olddial;
    Xstrncpy(
        &raw mut tmpGkword.st_crasis as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    Xstrncpy(
        &raw mut tmpGkword.st_workword as *mut ::core::ffi::c_char,
        &raw mut saveword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn do_crasis(
    mut gstring: *mut gk_string,
    mut crasis: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut gend: ::core::ffi::c_int = 0;
    let mut num: ::core::ffi::c_int = 0;
    let mut wcase: ::core::ffi::c_int = 0;
    let mut saw_this_crasis: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    num = (*gstring).gs_forminfo.f_number() as ::core::ffi::c_int;
    gend = (*gstring).gs_forminfo.f_gender() as ::core::ffi::c_int;
    wcase = (*gstring).gs_forminfo.f_case() as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
