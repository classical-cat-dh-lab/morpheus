use ::c2rust_bitfields;
extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
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
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn cinsert(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn CreatGkword(_: ::core::ffi::c_int) -> *mut gk_word;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn FreeGkword(_: *mut gk_word) -> ::core::ffi::c_int;
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
    fn cur_lang() -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn morphstrncmp(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn set_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn striphyph(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripquant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
pub type bool_0 = ::core::ffi::c_int;
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
pub struct augtable {
    pub noaug: [::core::ffi::c_char; 8],
    pub withaug: [::core::ffi::c_char; 8],
    pub augdial: Dialect,
    pub uniqueflag: ::core::ffi::c_char,
}
pub const ATTIC: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const IONIC: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const AEOLIC: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const HOMERIC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const DORIC: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const ALL_DIAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NON_HOMERIC_EPIC: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const EPIC: ::core::ffi::c_int = NON_HOMERIC_EPIC | HOMERIC;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SECONDARY: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const IMPERF: ::core::ffi::c_int = 10;
pub const AORIST: ::core::ffi::c_int = 12;
pub const PLUPERF: ::core::ffi::c_int = 6;
pub const INDICATIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SYLL_AUG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HARDSHORT: ::core::ffi::c_int = '^' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
pub const SYLL_AUGMENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const UNAUGMENTED: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const HAS_AUGMENT: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const ATTIC_REDUPL: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const RAW_SONANT: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
#[no_mangle]
pub static mut TempAugments: [augtable; 35] = unsafe {
    [
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ai)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h)|\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ai(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h(|\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h)|\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h(|\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"oi)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w)|\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"oi(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w(|\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"oi)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w)|\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"oi(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w(|\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"au)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"hu)\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"au(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"hu(\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"au)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"au)\0\0\0\0\0"),
            augdial: DORIC as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"au(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"au(\0\0\0\0\0"),
            augdial: DORIC as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"eu)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"hu)\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"eu(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"hu(\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)e\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)e\0\0\0\0\0"),
            augdial: EPIC as Dialect,
            uniqueflag: 1 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(e\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(e\0\0\0\0\0"),
            augdial: EPIC as Dialect,
            uniqueflag: 1 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"a)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h)\0\0\0\0\0\0"),
            augdial: (ATTIC | IONIC | EPIC) as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"a(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h(\0\0\0\0\0\0"),
            augdial: (ATTIC | IONIC | EPIC) as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"a)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"a_)\0\0\0\0\0"),
            augdial: (DORIC | AEOLIC) as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"a(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"a_(\0\0\0\0\0"),
            augdial: (DORIC | AEOLIC) as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h)\0\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h(\0\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h(\0\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h)\0\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"i(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"i_(\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"i)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"i_)\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"o)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w)\0\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"o(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w(\0\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w)\0\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w(\0\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"u)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"u_)\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"u(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"u_(\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ou)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ou)\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ou(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ou(\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: [0 as ::core::ffi::c_int as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0],
            withaug: [0; 8],
            augdial: 0,
            uniqueflag: 0,
        },
    ]
};
#[no_mangle]
pub static mut SyllAugments: [augtable; 28] = unsafe {
    [
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"i)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei)\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"i(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei(\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"i)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)i\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"i(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(i\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"oi)w\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"oi)w\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)oi\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)w|\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 1 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(oi\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(w|\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 1 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"oi)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)w|\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"oi(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(w|\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei(\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei)\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei)\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei(\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"a)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)a\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"a(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(a\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)h\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"h(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(h\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)w\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"w(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(w\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"o)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)w\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"o(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(w\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei)\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(\0\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ei(\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"eu)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"eu)\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"eu(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"eu(\0\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ou)\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e)ou\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"ou(\0\0\0\0\0"),
            withaug: ::core::mem::transmute::<
                [u8; 8],
                [::core::ffi::c_char; 8],
            >(*b"e(ou\0\0\0\0"),
            augdial: ALL_DIAL as Dialect,
            uniqueflag: 0 as ::core::ffi::c_char,
        },
        augtable {
            noaug: [0 as ::core::ffi::c_int as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0],
            withaug: [0; 8],
            augdial: 0,
            uniqueflag: 0,
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn do_syllaug(
    mut gkform: *mut gk_word,
    mut maxaugs: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut naugs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: Dialect = 0;
    let mut compval: ::core::ffi::c_int = 0;
    let mut wstart: ::core::ffi::c_int = 0;
    let mut tmpstem: [::core::ffi::c_char; 60] = [0; 60];
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
    TmpGkword = *gkform;
    Xstrncpy(
        &raw mut tmpstem as *mut ::core::ffi::c_char,
        &raw mut (*gkform).st_workword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    striphyph(&raw mut tmpstem as *mut ::core::ffi::c_char);
    i = 0 as ::core::ffi::c_int;
    while SyllAugments[i as usize].noaug[0 as ::core::ffi::c_int as usize] != 0 {
        compval = Xstrncmp(
            &raw mut (*(&raw mut SyllAugments as *mut augtable).offset(i as isize)).noaug
                as *mut ::core::ffi::c_char,
            &raw mut tmpstem as *mut ::core::ffi::c_char,
            Xstrlen(
                &raw mut (*(&raw mut SyllAugments as *mut augtable).offset(i as isize))
                    .noaug as *mut ::core::ffi::c_char,
            ) as size_t,
        );
        if compval == 0 {
            let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
            *gkform.offset(naugs as isize) = TmpGkword;
            d = AndDialect(
                (*gkform.offset(naugs as isize)).gs_dialect,
                SyllAugments[i as usize].augdial,
            );
            if !((d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int) {
                if d != 0 {
                    (*gkform.offset(naugs as isize)).gs_dialect = d;
                }
                Xstrncpy(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut SyllAugments as *mut augtable)
                        .offset(i as isize))
                        .withaug as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                wstart = Xstrlen(
                    &raw mut (*(&raw mut SyllAugments as *mut augtable)
                        .offset(i as isize))
                        .noaug as *mut ::core::ffi::c_char,
                );
                if tmpstem[wstart as usize] as ::core::ffi::c_int == HARDSHORT {
                    wstart += 1;
                }
                Xstrncat(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    (&raw mut tmpstem as *mut ::core::ffi::c_char)
                        .offset(wstart as isize),
                    MAXWORDSIZE as size_t,
                );
                sprintf(
                    &raw mut (*gkform.offset(naugs as isize)).gs_aug1.gs_gkstring
                        as *mut ::core::ffi::c_char,
                    b"%s>%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut (*(&raw mut SyllAugments as *mut augtable)
                        .offset(i as isize))
                        .noaug as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut SyllAugments as *mut augtable)
                        .offset(i as isize))
                        .withaug as *mut ::core::ffi::c_char,
                );
                (*gkform.offset(naugs as isize)).gs_aug1.gs_dialect = SyllAugments[i
                        as usize]
                    .augdial;
                add_morphflag(
                    &raw mut (*gkform.offset(naugs as isize)).gs_aug1.gs_morphflags
                        as *mut MorphFlags,
                    SYLL_AUG,
                );
                zap_morphflag(
                    &raw mut (*gkform.offset(naugs as isize)).gs_stem.gs_morphflags
                        as *mut MorphFlags,
                    SYLL_AUG,
                );
                Xstrncpy(
                    &raw mut (*gkform.offset(naugs as isize)).st_workword
                        as *mut ::core::ffi::c_char,
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                naugs += 1;
                if naugs >= maxaugs {
                    fprintf(
                        stderr,
                        b"temp: got naugs %d with max %d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        naugs,
                        maxaugs,
                    );
                    break;
                } else if SyllAugments[i as usize].uniqueflag != 0 {
                    break;
                }
            }
        }
        i += 1;
    }
    return naugs;
}
#[no_mangle]
pub unsafe extern "C" fn do_tempaug(
    mut gkform: *mut gk_word,
    mut maxaugs: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut wstart: ::core::ffi::c_int = 0;
    let mut naugs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmpstem: [::core::ffi::c_char; 60] = [0; 60];
    let mut d: Dialect = 0;
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
    TmpGkword = *gkform;
    Xstrncpy(
        &raw mut tmpstem as *mut ::core::ffi::c_char,
        &raw mut (*gkform).st_workword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    striphyph(&raw mut tmpstem as *mut ::core::ffi::c_char);
    i = 0 as ::core::ffi::c_int;
    while TempAugments[i as usize].noaug[0 as ::core::ffi::c_int as usize] != 0 {
        if Xstrncmp(
            &raw mut (*(&raw mut TempAugments as *mut augtable).offset(i as isize)).noaug
                as *mut ::core::ffi::c_char,
            &raw mut tmpstem as *mut ::core::ffi::c_char,
            Xstrlen(
                &raw mut (*(&raw mut TempAugments as *mut augtable).offset(i as isize))
                    .noaug as *mut ::core::ffi::c_char,
            ) as size_t,
        ) == 0
        {
            let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
            *gkform.offset(naugs as isize) = TmpGkword;
            d = AndDialect(
                (*gkform.offset(naugs as isize)).gs_dialect,
                TempAugments[i as usize].augdial,
            );
            if !((d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int) {
                if d != 0 {
                    (*gkform.offset(naugs as isize)).gs_dialect = d;
                }
                Xstrncpy(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut TempAugments as *mut augtable)
                        .offset(i as isize))
                        .withaug as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                wstart = Xstrlen(
                    &raw mut (*(&raw mut TempAugments as *mut augtable)
                        .offset(i as isize))
                        .noaug as *mut ::core::ffi::c_char,
                );
                if tmpstem[wstart as usize] as ::core::ffi::c_int == HARDSHORT {
                    wstart += 1;
                }
                Xstrncat(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    (&raw mut tmpstem as *mut ::core::ffi::c_char)
                        .offset(wstart as isize),
                    MAXWORDSIZE as size_t,
                );
                sprintf(
                    &raw mut (*gkform.offset(naugs as isize)).gs_aug1.gs_gkstring
                        as *mut ::core::ffi::c_char,
                    b"%s>%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut (*(&raw mut TempAugments as *mut augtable)
                        .offset(i as isize))
                        .noaug as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut TempAugments as *mut augtable)
                        .offset(i as isize))
                        .withaug as *mut ::core::ffi::c_char,
                );
                (*gkform.offset(naugs as isize)).gs_aug1.gs_dialect = TempAugments[i
                        as usize]
                    .augdial;
                Xstrncpy(
                    &raw mut (*gkform.offset(naugs as isize)).st_workword
                        as *mut ::core::ffi::c_char,
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                naugs += 1;
                if naugs >= maxaugs {
                    fprintf(
                        stderr,
                        b"temp: got naugs %d with max %d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        naugs,
                        maxaugs,
                    );
                    break;
                } else if TempAugments[i as usize].uniqueflag != 0 {
                    break;
                }
            }
        }
        i += 1;
    }
    return naugs;
}
#[no_mangle]
pub unsafe extern "C" fn unaugment(
    mut s: *mut ::core::ffi::c_char,
    mut possibs: *mut *mut gk_string,
    mut qpossibs: *mut *mut gk_string,
    mut maxstems: ::core::ffi::c_int,
    mut dial: Dialect,
    mut wantsyllaugs: ::core::ffi::c_int,
    mut wantredupl: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut compval: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut augnoquant: [::core::ffi::c_char; 60] = [0; 60];
    let mut d: Dialect = 0;
    if Xstrncmp(s, b"e)rr\0" as *const u8 as *const ::core::ffi::c_char, 4 as size_t)
        == 0
    {
        rval = 1 as ::core::ffi::c_int;
        Xstrncpy(
            &raw mut (**possibs.offset(0 as ::core::ffi::c_int as isize)).gs_gkstring
                as *mut ::core::ffi::c_char,
            b"r(\0" as *const u8 as *const ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        Xstrncat(
            &raw mut (**possibs.offset(0 as ::core::ffi::c_int as isize)).gs_gkstring
                as *mut ::core::ffi::c_char,
            s.offset(4 as ::core::ffi::c_int as isize),
            MAXWORDSIZE as size_t,
        );
        if wantredupl == 0 {
            let ref mut fresh0 = (**possibs.offset(0 as ::core::ffi::c_int as isize))
                .gs_forminfo;
            (*fresh0).set_f_mood(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        return rval;
    }
    if Xstrncmp(s, b"e)r\0" as *const u8 as *const ::core::ffi::c_char, 3 as size_t) == 0
    {
        rval = 1 as ::core::ffi::c_int;
        Xstrncpy(
            &raw mut (**possibs.offset(0 as ::core::ffi::c_int as isize)).gs_gkstring
                as *mut ::core::ffi::c_char,
            b"r(\0" as *const u8 as *const ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        Xstrncat(
            &raw mut (**possibs.offset(0 as ::core::ffi::c_int as isize)).gs_gkstring
                as *mut ::core::ffi::c_char,
            s.offset(3 as ::core::ffi::c_int as isize),
            MAXWORDSIZE as size_t,
        );
        add_morphflag(
            &raw mut (**possibs.offset(0 as ::core::ffi::c_int as isize)).gs_morphflags
                as *mut MorphFlags,
            RAW_SONANT,
        );
        if wantredupl == 0 {
            let ref mut fresh1 = (**possibs.offset(0 as ::core::ffi::c_int as isize))
                .gs_forminfo;
            (*fresh1).set_f_mood(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        return rval;
    }
    if Xstrncmp(s, b"e)\0" as *const u8 as *const ::core::ffi::c_char, 2 as size_t) == 0
        && ((if 0 as ::core::ffi::c_int != 0 {
            isalpha(*s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        } else {
            ((*s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) != 0
            && *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'j' as i32
            && *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'v' as i32
            && *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'J' as i32
            && *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'V' as i32
            && !(*s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'a' as i32
                || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'e' as i32
                || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'i' as i32
                || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'o' as i32
                || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'u' as i32
                || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'A' as i32
                || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'E' as i32
                || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'I' as i32
                || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'O' as i32
                || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'U' as i32
                || (*s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'h' as i32
                    || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'w' as i32
                    || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'H' as i32
                    || *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'W' as i32)))
    {
        rval = 1 as ::core::ffi::c_int;
        Xstrncpy(
            &raw mut (**possibs.offset(0 as ::core::ffi::c_int as isize)).gs_gkstring
                as *mut ::core::ffi::c_char,
            s.offset(2 as ::core::ffi::c_int as isize),
            MAXWORDSIZE as size_t,
        );
        if *s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == *s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        {
            rval = 2 as ::core::ffi::c_int;
            Xstrncpy(
                &raw mut (**possibs.offset(1 as ::core::ffi::c_int as isize)).gs_gkstring
                    as *mut ::core::ffi::c_char,
                s.offset(3 as ::core::ffi::c_int as isize),
                MAXWORDSIZE as size_t,
            );
            add_morphflag(
                &raw mut (**possibs.offset(1 as ::core::ffi::c_int as isize))
                    .gs_morphflags as *mut MorphFlags,
                SYLL_AUGMENT,
            );
        }
        if wantredupl == 0 {
            let ref mut fresh2 = (**possibs.offset(0 as ::core::ffi::c_int as isize))
                .gs_forminfo;
            (*fresh2).set_f_mood(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        return rval;
    }
    i = 0 as ::core::ffi::c_int;
    while TempAugments[i as usize].noaug[0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int != 0 && rval < maxstems
    {
        Xstrncpy(
            &raw mut augnoquant as *mut ::core::ffi::c_char,
            &raw mut (*(&raw mut TempAugments as *mut augtable).offset(i as isize))
                .withaug as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        stripquant(&raw mut augnoquant as *mut ::core::ffi::c_char);
        compval = morphstrncmp(
            &raw mut augnoquant as *mut ::core::ffi::c_char,
            s,
            strlen(&raw mut augnoquant as *mut ::core::ffi::c_char),
        );
        if compval == 0 {
            let mut tmp: [::core::ffi::c_char; 128] = [0; 128];
            Xstrncpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut TempAugments as *mut augtable).offset(i as isize))
                    .noaug as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                s
                    .offset(
                        Xstrlen(&raw mut augnoquant as *mut ::core::ffi::c_char) as isize,
                    ),
                MAXWORDSIZE as size_t,
            );
            Xstrncpy(
                &raw mut (**possibs.offset(rval as isize)).gs_gkstring
                    as *mut ::core::ffi::c_char,
                &raw mut tmp as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            if strcmp(
                &raw mut augnoquant as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut TempAugments as *mut augtable).offset(i as isize))
                    .withaug as *mut ::core::ffi::c_char,
            ) != 0
            {
                Xstrncpy(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut TempAugments as *mut augtable)
                        .offset(i as isize))
                        .withaug as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                Xstrncat(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    s
                        .offset(
                            Xstrlen(&raw mut augnoquant as *mut ::core::ffi::c_char)
                                as isize,
                        ),
                    MAXWORDSIZE as size_t,
                );
                Xstrncpy(
                    &raw mut (**qpossibs.offset(rval as isize)).gs_gkstring
                        as *mut ::core::ffi::c_char,
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                (**possibs.offset(rval as isize)).gs_dialect = TempAugments[i as usize]
                    .augdial;
            }
            if wantredupl == 0 {
                let ref mut fresh3 = (**possibs.offset(rval as isize)).gs_forminfo;
                (*fresh3).set_f_mood(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            rval += 1;
        }
        i += 1;
    }
    if wantsyllaugs == 0 {
        return rval;
    }
    i = 0 as ::core::ffi::c_int;
    while SyllAugments[i as usize].noaug[0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int != 0 && rval < maxstems
    {
        Xstrncpy(
            &raw mut augnoquant as *mut ::core::ffi::c_char,
            &raw mut (*(&raw mut SyllAugments as *mut augtable).offset(i as isize))
                .withaug as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        stripquant(&raw mut augnoquant as *mut ::core::ffi::c_char);
        compval = morphstrncmp(
            &raw mut augnoquant as *mut ::core::ffi::c_char,
            s,
            strlen(&raw mut augnoquant as *mut ::core::ffi::c_char),
        );
        if compval == 0 {
            let mut tmp_0: [::core::ffi::c_char; 128] = [0; 128];
            Xstrncpy(
                &raw mut tmp_0 as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut SyllAugments as *mut augtable).offset(i as isize))
                    .noaug as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            Xstrncat(
                &raw mut tmp_0 as *mut ::core::ffi::c_char,
                s
                    .offset(
                        Xstrlen(&raw mut augnoquant as *mut ::core::ffi::c_char) as isize,
                    ),
                MAXWORDSIZE as size_t,
            );
            Xstrncpy(
                &raw mut (**possibs.offset(rval as isize)).gs_gkstring
                    as *mut ::core::ffi::c_char,
                &raw mut tmp_0 as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            (**possibs.offset(rval as isize)).gs_dialect = SyllAugments[i as usize]
                .augdial;
            add_morphflag(
                &raw mut (**possibs.offset(rval as isize)).gs_morphflags
                    as *mut MorphFlags,
                SYLL_AUGMENT,
            );
            if wantredupl == 0 {
                let ref mut fresh4 = (**possibs.offset(rval as isize)).gs_forminfo;
                (*fresh4).set_f_mood(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            rval += 1;
        }
        i += 1;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn unaugfromlemma(
    mut stem: *mut ::core::ffi::c_char,
    mut lemma: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    let mut i: ::core::ffi::c_int = 0;
    let mut withlen: ::core::ffi::c_int = 0;
    let mut noauglen: ::core::ffi::c_int = 0;
    let mut compval: ::core::ffi::c_int = 0;
    if Xstrncmp(stem, b"e)\0" as *const u8 as *const ::core::ffi::c_char, 2 as size_t)
        == 0
    {
        if (if 0 as ::core::ffi::c_int != 0 {
            isalpha(*lemma as ::core::ffi::c_int)
        } else {
            ((*lemma as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) != 0 && *lemma as ::core::ffi::c_int != 'j' as i32
            && *lemma as ::core::ffi::c_int != 'v' as i32
            && *lemma as ::core::ffi::c_int != 'J' as i32
            && *lemma as ::core::ffi::c_int != 'V' as i32
            && !(*lemma as ::core::ffi::c_int == 'a' as i32
                || *lemma as ::core::ffi::c_int == 'e' as i32
                || *lemma as ::core::ffi::c_int == 'i' as i32
                || *lemma as ::core::ffi::c_int == 'o' as i32
                || *lemma as ::core::ffi::c_int == 'u' as i32
                || *lemma as ::core::ffi::c_int == 'A' as i32
                || *lemma as ::core::ffi::c_int == 'E' as i32
                || *lemma as ::core::ffi::c_int == 'I' as i32
                || *lemma as ::core::ffi::c_int == 'O' as i32
                || *lemma as ::core::ffi::c_int == 'U' as i32
                || (*lemma as ::core::ffi::c_int == 'h' as i32
                    || *lemma as ::core::ffi::c_int == 'w' as i32
                    || *lemma as ::core::ffi::c_int == 'H' as i32
                    || *lemma as ::core::ffi::c_int == 'W' as i32))
        {
            Xstrncpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                stem.offset(2 as ::core::ffi::c_int as isize),
                MAXWORDSIZE as size_t,
            );
            Xstrncpy(
                stem,
                &raw mut tmp as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            return SYLL_AUGMENT * 2 as ::core::ffi::c_int;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while TempAugments[i as usize].noaug[0 as ::core::ffi::c_int as usize] != 0 {
        withlen = Xstrlen(
            &raw mut (*(&raw mut TempAugments as *mut augtable).offset(i as isize))
                .withaug as *mut ::core::ffi::c_char,
        );
        if Xstrncmp(
            &raw mut (*(&raw mut TempAugments as *mut augtable).offset(i as isize))
                .withaug as *mut ::core::ffi::c_char,
            stem,
            withlen as size_t,
        ) == 0
        {
            noauglen = Xstrlen(
                &raw mut (*(&raw mut TempAugments as *mut augtable).offset(i as isize))
                    .noaug as *mut ::core::ffi::c_char,
            );
            if Xstrncmp(
                &raw mut (*(&raw mut TempAugments as *mut augtable).offset(i as isize))
                    .noaug as *mut ::core::ffi::c_char,
                lemma,
                noauglen as size_t,
            ) == 0
            {
                Xstrncpy(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut TempAugments as *mut augtable)
                        .offset(i as isize))
                        .noaug as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                Xstrncat(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    stem
                        .offset(
                            Xstrlen(
                                &raw mut (*(&raw mut TempAugments as *mut augtable)
                                    .offset(i as isize))
                                    .withaug as *mut ::core::ffi::c_char,
                            ) as isize,
                        ),
                    MAXWORDSIZE as size_t,
                );
                Xstrncpy(
                    stem,
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return SYLL_AUGMENT * 2 as ::core::ffi::c_int;
            }
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while SyllAugments[i as usize].noaug[0 as ::core::ffi::c_int as usize] != 0 {
        withlen = Xstrlen(
            &raw mut (*(&raw mut SyllAugments as *mut augtable).offset(i as isize))
                .withaug as *mut ::core::ffi::c_char,
        );
        compval = Xstrncmp(
            &raw mut (*(&raw mut SyllAugments as *mut augtable).offset(i as isize))
                .withaug as *mut ::core::ffi::c_char,
            stem,
            withlen as size_t,
        );
        if compval == 0 {
            noauglen = Xstrlen(
                &raw mut (*(&raw mut SyllAugments as *mut augtable).offset(i as isize))
                    .noaug as *mut ::core::ffi::c_char,
            );
            if Xstrncmp(
                &raw mut (*(&raw mut SyllAugments as *mut augtable).offset(i as isize))
                    .noaug as *mut ::core::ffi::c_char,
                lemma,
                noauglen as size_t,
            ) == 0
            {
                Xstrncpy(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut SyllAugments as *mut augtable)
                        .offset(i as isize))
                        .noaug as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                Xstrncat(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    stem
                        .offset(
                            Xstrlen(
                                &raw mut (*(&raw mut SyllAugments as *mut augtable)
                                    .offset(i as isize))
                                    .withaug as *mut ::core::ffi::c_char,
                            ) as isize,
                        ),
                    MAXWORDSIZE as size_t,
                );
                Xstrncpy(
                    stem,
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return 1 as ::core::ffi::c_int;
            }
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn add_augment(
    mut gkform: *mut gk_word,
    mut mf: *mut MorphFlags,
    mut maxaugs: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut syllabic: bool_0 = 0;
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut SaveGstr: gk_string = gk_string {
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
    let mut tmpgstr: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    tmpgstr = &raw mut SaveGstr;
    syllabic = (if has_morphflag(
        &raw mut (*gkform).gs_stem.gs_morphflags as *mut MorphFlags,
        SYLL_AUGMENT,
    ) != 0
    {
        YES
    } else {
        NO
    }) as bool_0;
    res = &raw mut (*gkform).st_workword as *mut ::core::ffi::c_char;
    set_morphflags(&raw mut SaveGstr, mf);
    if has_morphflag(&raw mut (*tmpgstr).gs_morphflags as *mut MorphFlags, UNAUGMENTED)
        != 0
        || has_morphflag(
            &raw mut (*gkform).gs_endstring.gs_morphflags as *mut MorphFlags,
            UNAUGMENTED,
        ) != 0
    {
        add_morphflag(
            &raw mut (*gkform).gs_stem.gs_morphflags as *mut MorphFlags,
            UNAUGMENTED,
        );
        return 0 as ::core::ffi::c_int;
    }
    if cur_lang() == LATIN || cur_lang() == ITALIAN {
        return 0 as ::core::ffi::c_int;
    }
    if needs_augment2(gkform, res) == 0 {
        zap_morphflag(
            &raw mut (*gkform).gs_stem.gs_morphflags as *mut MorphFlags,
            SYLL_AUGMENT,
        );
        if has_morphflag(
            &raw mut (*gkform).gs_stem.gs_morphflags as *mut MorphFlags,
            HAS_AUGMENT,
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    rval = augmentit(gkform, syllabic, maxaugs);
    add_morphflag(
        &raw mut (*gkform).gs_stem.gs_morphflags as *mut MorphFlags,
        HAS_AUGMENT,
    );
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn needs_augment(mut gstr: *mut gk_string) -> ::core::ffi::c_int {
    let mut TmpGkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut rval: ::core::ffi::c_int = 0;
    TmpGkword = CreatGkword(2 as ::core::ffi::c_int);
    if TmpGkword.is_null() {
        fprintf(
            stderr,
            b"no memory for TmpGkword in needs_augment\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    (*TmpGkword).gs_stem = *gstr;
    (*TmpGkword).gs_forminfo = (*gstr).gs_forminfo;
    rval = needs_augment2(
        TmpGkword,
        b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    FreeGkword(TmpGkword);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn needs_augment2(
    mut gkform: *mut gk_word,
    mut stem: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut v_form: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    v_form = (*gkform).gs_forminfo;
    if has_morphflag(
        &raw mut (*gkform).gs_stem.gs_morphflags as *mut MorphFlags,
        HAS_AUGMENT,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if v_form.f_mood() as ::core::ffi::c_int != INDICATIVE {
        return 0 as ::core::ffi::c_int;
    }
    match v_form.f_tense() as ::core::ffi::c_int {
        IMPERF => return 1 as ::core::ffi::c_int,
        AORIST => return 1 as ::core::ffi::c_int,
        PLUPERF => {
            if has_morphflag(
                &raw mut (*gkform).gs_stem.gs_morphflags as *mut MorphFlags,
                ATTIC_REDUPL,
            ) != 0
            {
                return 1 as ::core::ffi::c_int;
            }
            if *stem as ::core::ffi::c_int == 'e' as i32
                || *stem as ::core::ffi::c_int == 'h' as i32
            {
                return 0 as ::core::ffi::c_int;
            }
            return 1 as ::core::ffi::c_int;
        }
        _ => return 0 as ::core::ffi::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn simpleaugment(
    mut s: *mut ::core::ffi::c_char,
    mut syllabic: bool_0,
) -> ::core::ffi::c_int {
    let mut gkform: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    gkform = CreatGkword(6 as ::core::ffi::c_int);
    if gkform.is_null() {
        fprintf(
            stderr,
            b"no memory for gkform in simpleaugment of [%s]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            s,
        );
        return 0;
    }
    Xstrncpy(
        &raw mut (*gkform).st_workword as *mut ::core::ffi::c_char,
        s,
        MAXWORDSIZE as size_t,
    );
    augmentit(gkform, syllabic, 5 as ::core::ffi::c_int);
    Xstrncpy(
        s,
        &raw mut (*gkform).st_workword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    FreeGkword(gkform);
    return 0;
}
unsafe extern "C" fn augmentit(
    mut gkform: *mut gk_word,
    mut syllabic: bool_0,
    mut maxaugs: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = &raw mut (*gkform).st_workword
        as *mut ::core::ffi::c_char;
    let mut stem_gstr: *mut gk_string = &raw mut (*gkform).gs_stem;
    if (if 0 as ::core::ffi::c_int != 0 {
        isalpha(*s as ::core::ffi::c_int)
    } else {
        ((*s as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    }) != 0 && *s as ::core::ffi::c_int != 'j' as i32
        && *s as ::core::ffi::c_int != 'v' as i32
        && *s as ::core::ffi::c_int != 'J' as i32
        && *s as ::core::ffi::c_int != 'V' as i32
        && !(*s as ::core::ffi::c_int == 'a' as i32
            || *s as ::core::ffi::c_int == 'e' as i32
            || *s as ::core::ffi::c_int == 'i' as i32
            || *s as ::core::ffi::c_int == 'o' as i32
            || *s as ::core::ffi::c_int == 'u' as i32
            || *s as ::core::ffi::c_int == 'A' as i32
            || *s as ::core::ffi::c_int == 'E' as i32
            || *s as ::core::ffi::c_int == 'I' as i32
            || *s as ::core::ffi::c_int == 'O' as i32
            || *s as ::core::ffi::c_int == 'U' as i32
            || (*s as ::core::ffi::c_int == 'h' as i32
                || *s as ::core::ffi::c_int == 'w' as i32
                || *s as ::core::ffi::c_int == 'H' as i32
                || *s as ::core::ffi::c_int == 'W' as i32))
    {
        if 0 as ::core::ffi::c_int
            == Xstrncmp(
                b"r(\0" as *const u8 as *const ::core::ffi::c_char,
                s,
                2 as size_t,
            )
        {
            if has_morphflag(
                &raw mut (*stem_gstr).gs_morphflags as *mut MorphFlags,
                RAW_SONANT,
            ) != 0
            {
                stripbreath(s);
            } else {
                *s.offset(1 as ::core::ffi::c_int as isize) = 'r' as i32
                    as ::core::ffi::c_char;
            }
            strcpy(
                &raw mut (*gkform).gs_aug1.gs_gkstring as *mut ::core::ffi::c_char,
                b"r(>e)rr\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else if has_morphflag(
            &raw mut (*stem_gstr).gs_morphflags as *mut MorphFlags,
            SYLL_AUGMENT,
        ) != 0
        {
            cinsert(*s as ::core::ffi::c_int, s);
            sprintf(
                &raw mut (*gkform).gs_aug1.gs_gkstring as *mut ::core::ffi::c_char,
                b"%c>e)%c%c\0" as *const u8 as *const ::core::ffi::c_char,
                *s as ::core::ffi::c_int,
                *s as ::core::ffi::c_int,
                *s as ::core::ffi::c_int,
            );
        }
        cinsert(SMOOTHBR, s);
        cinsert('e' as i32, s);
        if (*gkform).gs_aug1.gs_gkstring[0 as ::core::ffi::c_int as usize] == 0 {
            strcpy(
                &raw mut (*gkform).gs_aug1.gs_gkstring as *mut ::core::ffi::c_char,
                b"e)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        return 1 as ::core::ffi::c_int;
    }
    if syllabic != 0 {
        return do_syllaug(gkform, maxaugs)
    } else {
        return do_tempaug(gkform, maxaugs)
    };
}
#[no_mangle]
pub unsafe extern "C" fn simpleredupit(
    mut s: *mut ::core::ffi::c_char,
    mut syllabic: ::core::ffi::c_int,
    mut redupc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut gkform: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    gkform = CreatGkword(6 as ::core::ffi::c_int);
    if gkform.is_null() {
        fprintf(
            stderr,
            b"no memory for gkform in simpleaugment of [%s]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            s,
        );
        return 0;
    }
    Xstrncpy(
        &raw mut (*gkform).st_workword as *mut ::core::ffi::c_char,
        s,
        MAXWORDSIZE as size_t,
    );
    redupit2(gkform, syllabic, redupc, 5 as ::core::ffi::c_int);
    Xstrncpy(
        s,
        &raw mut (*gkform).st_workword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    FreeGkword(gkform);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn redupit2(
    mut gkform: *mut gk_word,
    mut syllabic: ::core::ffi::c_int,
    mut redupc: ::core::ffi::c_int,
    mut nredups: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_char = 0;
    let mut s: *mut ::core::ffi::c_char = &raw mut (*gkform).st_workword
        as *mut ::core::ffi::c_char;
    if (if 0 as ::core::ffi::c_int != 0 {
        isalpha(*s as ::core::ffi::c_int)
    } else {
        ((*s as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    }) != 0 && *s as ::core::ffi::c_int != 'j' as i32
        && *s as ::core::ffi::c_int != 'v' as i32
        && *s as ::core::ffi::c_int != 'J' as i32
        && *s as ::core::ffi::c_int != 'V' as i32
        && !(*s as ::core::ffi::c_int == 'a' as i32
            || *s as ::core::ffi::c_int == 'e' as i32
            || *s as ::core::ffi::c_int == 'i' as i32
            || *s as ::core::ffi::c_int == 'o' as i32
            || *s as ::core::ffi::c_int == 'u' as i32
            || *s as ::core::ffi::c_int == 'A' as i32
            || *s as ::core::ffi::c_int == 'E' as i32
            || *s as ::core::ffi::c_int == 'I' as i32
            || *s as ::core::ffi::c_int == 'O' as i32
            || *s as ::core::ffi::c_int == 'U' as i32
            || (*s as ::core::ffi::c_int == 'h' as i32
                || *s as ::core::ffi::c_int == 'w' as i32
                || *s as ::core::ffi::c_int == 'H' as i32
                || *s as ::core::ffi::c_int == 'W' as i32))
        && ((if 0 as ::core::ffi::c_int != 0 {
            isalpha(*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        } else {
            ((*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) != 0
            && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'j' as i32
            && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'v' as i32
            && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'J' as i32
            && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'V' as i32
            && !(*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'a' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'e' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'i' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'o' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'u' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'A' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'E' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'I' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'O' as i32
                || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'U' as i32
                || (*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'h' as i32
                    || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'w' as i32
                    || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'H' as i32
                    || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'W' as i32)))
        && !(*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'l' as i32
            || *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'r' as i32) && redupc != 'i' as i32
    {
        return augmentit(gkform, syllabic as bool_0, nredups)
    } else if *s as ::core::ffi::c_int == 'z' as i32
        || *s as ::core::ffi::c_int == 'c' as i32
        || *s as ::core::ffi::c_int == 'y' as i32
    {
        return augmentit(gkform, syllabic as bool_0, nredups)
    } else if *s as ::core::ffi::c_int == 'a' as i32
        || *s as ::core::ffi::c_int == 'e' as i32
        || *s as ::core::ffi::c_int == 'i' as i32
        || *s as ::core::ffi::c_int == 'o' as i32
        || *s as ::core::ffi::c_int == 'u' as i32
        || *s as ::core::ffi::c_int == 'A' as i32
        || *s as ::core::ffi::c_int == 'E' as i32
        || *s as ::core::ffi::c_int == 'I' as i32
        || *s as ::core::ffi::c_int == 'O' as i32
        || *s as ::core::ffi::c_int == 'U' as i32
        || (*s as ::core::ffi::c_int == 'h' as i32
            || *s as ::core::ffi::c_int == 'w' as i32
            || *s as ::core::ffi::c_int == 'H' as i32
            || *s as ::core::ffi::c_int == 'W' as i32)
        || *s as ::core::ffi::c_int == 'r' as i32
    {
        return augmentit(gkform, syllabic as bool_0, nredups)
    } else if (Xstrncmp(
        b"gn\0" as *const u8 as *const ::core::ffi::c_char,
        s,
        2 as size_t,
    ) == 0
        || Xstrncmp(b"gl\0" as *const u8 as *const ::core::ffi::c_char, s, 2 as size_t)
            == 0) && redupc != 'i' as i32
    {
        return augmentit(gkform, syllabic as bool_0, nredups)
    } else {
        c = *s;
        if c as ::core::ffi::c_int == 'q' as i32 {
            c = 't' as i32 as ::core::ffi::c_char;
        }
        if c as ::core::ffi::c_int == 'x' as i32 {
            c = 'k' as i32 as ::core::ffi::c_char;
        }
        if c as ::core::ffi::c_int == 'f' as i32 {
            c = 'p' as i32 as ::core::ffi::c_char;
        }
        cinsert(redupc, s);
        cinsert(c as ::core::ffi::c_int, s);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn un_redupl(
    mut src: *mut ::core::ffi::c_char,
    mut res: *mut ::core::ffi::c_char,
    mut redupc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sbuf: [::core::ffi::c_char; 61] = [0; 61];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *res = 0 as ::core::ffi::c_char;
    Xstrncpy(&raw mut sbuf as *mut ::core::ffi::c_char, src, MAXWORDSIZE as size_t);
    stripacc(&raw mut sbuf as *mut ::core::ffi::c_char);
    if *(&raw mut sbuf as *mut ::core::ffi::c_char)
        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != redupc
    {
        return 0 as ::core::ffi::c_int;
    }
    p = (&raw mut sbuf as *mut ::core::ffi::c_char)
        .offset(2 as ::core::ffi::c_int as isize);
    if *p as ::core::ffi::c_int == 'q' as i32 {
        *p = 't' as i32 as ::core::ffi::c_char;
    } else if *p as ::core::ffi::c_int == 'x' as i32 {
        *p = 'k' as i32 as ::core::ffi::c_char;
    } else if *p as ::core::ffi::c_int == 'f' as i32 {
        *p = 'p' as i32 as ::core::ffi::c_char;
    }
    if sbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == sbuf[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    {
        Xstrncpy(
            res,
            src.offset(2 as ::core::ffi::c_int as isize),
            MAXWORDSIZE as size_t,
        );
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn add_double_augment(
    mut s: *mut ::core::ffi::c_char,
    mut oddpb: *mut MorphFlags,
) -> ::core::ffi::c_int {
    simpleaugment(s, NO);
    add_morphflag(oddpb, HAS_AUGMENT);
    return 0;
}
