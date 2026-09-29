use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fseek(
        _: *mut FILE,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
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
    fn MorphFopen(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> *mut FILE;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn GetTableLine(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn new_degree(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_person(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_morphflags(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_gender(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_case(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_number(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_tense(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_voice(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_mood(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_dialect(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_region(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_stemtype(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_derivtype(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn new_domain(_: *mut gk_string, _: ::core::ffi::c_ulong);
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn binlook(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: bool_0,
        _: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    ) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn nextkey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn xfer_prvbflags(_: *mut MorphFlags, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn zap_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
pub type Morph_flags = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Morph_args {
    pub morph_key: [::core::ffi::c_char; 60],
    pub morph_flags: Morph_flags,
    pub add_val: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub class_name: [::core::ffi::c_char; 60],
    pub class_num: ::core::ffi::c_long,
}
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const ATTIC: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const IONIC: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const AEOLIC: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const HOMERIC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const DORIC: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const PARADIGM: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const ALL_DIAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NON_HOMERIC_EPIC: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const EPIC: ::core::ffi::c_int = NON_HOMERIC_EPIC | HOMERIC;
pub const PROSE: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const PHOCIS: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const LOCRIS: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const ELIS: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const LACONIA: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const HERACLEA: ::core::ffi::c_int = 0o40 as ::core::ffi::c_int;
pub const MEGARID: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const ARGOLID: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const RHODES: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const COS: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const THERA: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const CYRENE: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const CRETE: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const ARCADIA: ::core::ffi::c_int = 0o20000 as ::core::ffi::c_int;
pub const CYPRUS: ::core::ffi::c_int = 0o40000 as ::core::ffi::c_int;
pub const BOEOTIA: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const INDECL: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const DECL1: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const DECL2: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const DECL3: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const DECL4: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const DECL5: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const ADJSTEM: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const NOUNSTEM: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const VERBSTEM: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
pub const REG_DERIV: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const PRIM_DERIV: ::core::ffi::c_int = 0o4000000 as ::core::ffi::c_int;
pub const PRIM_CONJ: ::core::ffi::c_int = PRIM_DERIV | VERBSTEM;
pub const REG_CONJ: ::core::ffi::c_int = REG_DERIV | VERBSTEM;
pub const PPARTMASK: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const PP_PR: ::core::ffi::c_int = 0o10000000 as ::core::ffi::c_int;
pub const PP_FU: ::core::ffi::c_int = 0o20000000 as ::core::ffi::c_int;
pub const PP_AO: ::core::ffi::c_int = 0o30000000 as ::core::ffi::c_int;
pub const PP_PF: ::core::ffi::c_int = 0o40000000 as ::core::ffi::c_int;
pub const PP_PP: ::core::ffi::c_int = 0o50000000 as ::core::ffi::c_int;
pub const PP_AP: ::core::ffi::c_int = 0o60000000 as ::core::ffi::c_int;
pub const PP_FP: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const PP_VA: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const PP_VN: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const PP_SU: ::core::ffi::c_int = 0o60000000 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const SECONDARY: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const PRESENT: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const IMPERF: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int | SECONDARY;
pub const FUTURE: ::core::ffi::c_int = 0o3 as ::core::ffi::c_int;
pub const AORIST: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int | SECONDARY;
pub const PERFECT: ::core::ffi::c_int = 0o5 as ::core::ffi::c_int;
pub const PLUPERF: ::core::ffi::c_int = 0o6 as ::core::ffi::c_int;
pub const FUTPERF: ::core::ffi::c_int = 0o7 as ::core::ffi::c_int | SECONDARY;
pub const PASTABSOLUTE: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const ACTIVE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const MIDDLE: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const PASSIVE: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const MEDIO_PASS: ::core::ffi::c_int = MIDDLE | PASSIVE;
pub const INDICATIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SUBJUNCTIVE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OPTATIVE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const IMPERATIVE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const INFINITIVE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PARTICIPLE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const GERUNDIVE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SUPINE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CONDITIONAL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SINGULAR: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const DUAL: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const PLURAL: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const PERS1: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const PERS2: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const PERS3: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const NOMINATIVE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const GENITIVE: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const DATIVE: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const ACCUSATIVE: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const VOCATIVE: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const ABLATIVE: ::core::ffi::c_int = 0o40 as ::core::ffi::c_int;
pub const MASCULINE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const FEMININE: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const NEUTER: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const ADVERBIAL: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const COMPARATIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SUPERLATIVE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SYLL_AUGMENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COMP_ONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENCLITIC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ITERATIVE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SUFF_ACC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const STEM_ACC: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CONTRACTED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const PERS_NAME: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ANT_ACC: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const IRREG_SUPERL: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const IRREG_COMP: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const NO_COMP: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SHORT_PEN: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const LONG_PEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const REC_ACC: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const NEEDS_ACCENT: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const R_E_I_ALPHA: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const NOT_IN_COMPOSITION: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const HAS_PREVERB: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const UNAUGMENTED: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const DISSIMILATION: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const PROCLITIC: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const APOCOPE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const HAS_AUGMENT: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const NU_MOVABLE: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const INTERV_S_TO_H: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const PREVB_AUGMENT: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const POETIC: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const UNCONTR_STEM: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const METATHESIS: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const ELIDE_PREVERB: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const INDECLFORM: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const ROOT_PREVERB: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const DIMINUTIVE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const LATE: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const RARE: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const RAW_PREVERB: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const EARLY: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const SHORT_SUBJ: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const UNASP_PREVERB: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const REDUPL: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const UNCONTR_END: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const IS_DERIV: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const ATTIC_REDUPL: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const NO_REDUPL: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const N_INFIX: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
pub const SYNCOPE: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const IMPERSONAL: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const NEEDS_RBREATH: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const NO_CIRCUMFLEX: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const CAUSAL: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const INTRANS: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
pub const TMESIS: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const RAW_SONANT: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
pub const PRODELISION: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
pub const FREQUENTAT: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const LATER: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const DOUBLE_AUGMENT: ::core::ffi::c_int = 61 as ::core::ffi::c_int;
pub const DOUBLE_REDUPL: ::core::ffi::c_int = 62 as ::core::ffi::c_int;
pub const DESIDERATIVE: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
pub const PRES_REDUPL: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const ENDS_IN_DIGAMMA: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const GEOG_NAME: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const DOUBLED_CONS: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
pub const IOTA_INTENS: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const SIG_TO_CI: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
pub const SHORT_EIS: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const PROS_TO_POTI: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const META_TO_PEDA: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
pub const PROS_TO_PROTI: ::core::ffi::c_int = 74 as ::core::ffi::c_int;
pub const UPO_TO_UPAI: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const PARA_TO_PARAI: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const UPER_TO_UPEIR: ::core::ffi::c_int = 77 as ::core::ffi::c_int;
pub const EN_TO_ENI: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const A_PRIV: ::core::ffi::c_int = 79 as ::core::ffi::c_int;
pub const A_COPUL: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
pub const METRICAL_LONG: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const GROUP_NAME: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const PROSEAUTHOR: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
static mut keys_inited: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut nstems: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut nderivs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut ndomains: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut nkeys: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut key_table: *mut *mut Morph_args = ::core::ptr::null::<*mut Morph_args>()
    as *mut *mut Morph_args;
#[no_mangle]
pub unsafe extern "C" fn ScanAsciiKeys(
    mut s: *mut ::core::ffi::c_char,
    mut Gkword: *mut gk_word,
    mut want: *mut gk_string,
    mut avoid: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut savekeys: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut curkey: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut gstr: *mut gk_string = want;
    let mut preverb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut lemma: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if Gkword.is_null() {
        fprintf(
            stderr,
            b"Hey! null Gkword in scanasciikeys!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    } else {
        preverb = &raw mut (*Gkword).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char;
        lemma = &raw mut (*Gkword).st_lemma as *mut ::core::ffi::c_char;
    }
    if !Gkword.is_null() && !(*Gkword).st_oddkeys.is_null() {
        (*Gkword).st_oddkeys = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    Xstrncpy(
        &raw mut savekeys as *mut ::core::ffi::c_char,
        s as *const ::core::ffi::c_char,
        LONGSTRING as size_t,
    );
    while nextkey(
        &raw mut savekeys as *mut ::core::ffi::c_char,
        &raw mut curkey as *mut ::core::ffi::c_char,
    ) != 0
    {
        if strcmp(
            b"not\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut curkey as *mut ::core::ffi::c_char,
        ) == 0
        {
            if avoid.is_null() {
                break;
            }
            gstr = avoid;
        } else if strcmp(
            b"crasis\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut curkey as *mut ::core::ffi::c_char,
        ) == 0
        {
            nextkey(
                &raw mut savekeys as *mut ::core::ffi::c_char,
                &raw mut curkey as *mut ::core::ffi::c_char,
            );
            Xstrncpy(
                &raw mut (*Gkword).st_crasis as *mut ::core::ffi::c_char,
                &raw mut curkey as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
        } else if strcmp(
            &raw mut curkey as *mut ::core::ffi::c_char,
            b"pb\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            nextkey(
                &raw mut savekeys as *mut ::core::ffi::c_char,
                &raw mut curkey as *mut ::core::ffi::c_char,
            );
            if !preverb.is_null() {
                Xstrncpy(
                    preverb,
                    &raw mut curkey as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                zap_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    ROOT_PREVERB,
                );
                add_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    HAS_PREVERB,
                );
            }
        } else if strcmp(
            &raw mut curkey as *mut ::core::ffi::c_char,
            b"rpb\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            nextkey(
                &raw mut savekeys as *mut ::core::ffi::c_char,
                &raw mut curkey as *mut ::core::ffi::c_char,
            );
            if !preverb.is_null() {
                Xstrncpy(
                    preverb,
                    &raw mut curkey as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                zap_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    HAS_PREVERB,
                );
                add_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    ROOT_PREVERB,
                );
            }
        } else if GetGkFlag(
            &raw mut curkey as *mut ::core::ffi::c_char,
            gstr,
            &raw mut (*Gkword).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
            preverb,
            lemma,
        ) == 0
        {
            if !Gkword.is_null() {
                if (*Gkword).st_oddkeys.is_null() {
                    (*Gkword).st_oddkeys = malloc(
                        (LONGSTRING + 1 as ::core::ffi::c_int) as size_t,
                    ) as *mut ::core::ffi::c_char;
                    *(*Gkword).st_oddkeys = 0 as ::core::ffi::c_char;
                }
                if *(*Gkword).st_oddkeys != 0 {
                    Xstrncat(
                        (*Gkword).st_oddkeys,
                        b" \0" as *const u8 as *const ::core::ffi::c_char,
                        LONGSTRING as size_t,
                    );
                }
                Xstrncat(
                    (*Gkword).st_oddkeys,
                    &raw mut curkey as *mut ::core::ffi::c_char,
                    LONGSTRING as size_t,
                );
            }
        }
    }
    if (*Gkword).gs_preverb.gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
        if has_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, ROOT_PREVERB)
            != 0
        {
            add_morphflag(
                &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
                ROOT_PREVERB,
            );
            add_morphflag(
                &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
                ROOT_PREVERB,
            );
            zap_morphflag(
                &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
                HAS_PREVERB,
            );
            zap_morphflag(
                &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                HAS_PREVERB,
            );
            zap_morphflag(
                &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
                HAS_PREVERB,
            );
        } else {
            zap_morphflag(
                &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
                ROOT_PREVERB,
            );
            zap_morphflag(
                &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
                ROOT_PREVERB,
            );
            add_morphflag(
                &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
                HAS_PREVERB,
            );
            add_morphflag(
                &raw mut (*Gkword).gs_stem.gs_morphflags as *mut MorphFlags,
                HAS_PREVERB,
            );
        }
    }
    if !Gkword.is_null() {
        RearrangeMorphflags(Gkword, want);
    }
    if (*want).gs_steminfo == 0 as Stemtype {
        return 0 as ::core::ffi::c_int
    } else {
        return 1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn RearrangeMorphflags(
    mut Gkword: *mut gk_word,
    mut gstr: *mut gk_string,
) -> ::core::ffi::c_int {
    xfer_prvbflags(
        &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
        &raw mut (*Gkword).gs_preverb.gs_morphflags as *mut MorphFlags,
    );
    xfer_prvbflags(
        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        &raw mut (*Gkword).gs_preverb.gs_morphflags as *mut MorphFlags,
    );
    return 0;
}
unsafe extern "C" fn GetGkFlag(
    mut field: *mut ::core::ffi::c_char,
    mut gstr: *mut gk_string,
    mut endstring: *mut ::core::ffi::c_char,
    mut preverb: *mut ::core::ffi::c_char,
    mut lemma: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if keys_inited == 0 {
        init_keys();
    }
    if AddMorphKey(gstr, field) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if Xstrncmp(field, b"end:\0" as *const u8 as *const ::core::ffi::c_char, 4 as size_t)
        == 0
    {
        Xstrncpy(
            endstring,
            field.offset(4 as ::core::ffi::c_int as isize),
            MAXWORDSIZE as size_t,
        );
        return 1 as ::core::ffi::c_int;
    }
    if Xstrncmp(
        field,
        b"pb:\0" as *const u8 as *const ::core::ffi::c_char,
        Xstrlen(b"pb:\0" as *const u8 as *const ::core::ffi::c_char) as size_t,
    ) == 0
    {
        if !preverb.is_null() {
            Xstrncpy(
                preverb,
                field.offset(3 as ::core::ffi::c_int as isize),
                MAXWORDSIZE as size_t,
            );
            zap_morphflag(
                &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                ROOT_PREVERB,
            );
            add_morphflag(
                &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                HAS_PREVERB,
            );
        }
        return 1 as ::core::ffi::c_int;
    }
    if Xstrncmp(
        field,
        b"rpb:\0" as *const u8 as *const ::core::ffi::c_char,
        Xstrlen(b"rpb:\0" as *const u8 as *const ::core::ffi::c_char) as size_t,
    ) == 0
    {
        if !preverb.is_null() {
            Xstrncpy(
                preverb,
                field.offset(4 as ::core::ffi::c_int as isize),
                MAXWORDSIZE as size_t,
            );
            zap_morphflag(
                &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                HAS_PREVERB,
            );
            add_morphflag(
                &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                ROOT_PREVERB,
            );
        }
        return 1 as ::core::ffi::c_int;
    }
    if Xstrncmp(
        field,
        b"le:\0" as *const u8 as *const ::core::ffi::c_char,
        Xstrlen(b"le:\0" as *const u8 as *const ::core::ffi::c_char) as size_t,
    ) == 0
    {
        if lemma.is_null() {
            fprintf(
                stderr,
                b"got handed lemma %s but had no place to put it!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                field.offset(3 as ::core::ffi::c_int as isize),
            );
        } else {
            Xstrncpy(
                lemma,
                field.offset(3 as ::core::ffi::c_int as isize),
                MAXWORDSIZE as size_t,
            );
        }
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn NextEndTable(
    mut index: *mut ::core::ffi::c_int,
    mut mask: Stemtype,
) -> *mut ::core::ffi::c_char {
    let mut morph_args: *mut Morph_args = ::core::ptr::null_mut::<Morph_args>();
    let mut oldmask: Dialect = mask as Dialect;
    mask &= (PPARTMASK | ADJSTEM | NOUNSTEM) as Stemtype;
    if keys_inited == 0 {
        init_keys();
    }
    morph_args = arg_stemtype.offset(*index as isize);
    while (*morph_args).morph_key[0 as ::core::ffi::c_int as usize] != 0 {
        if mask == 0 {
            *index += 1;
            if mask != 0 {
                printf(
                    b"0 mask returns [%s]\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &raw mut (*morph_args).morph_key as *mut ::core::ffi::c_char,
                );
            }
            return &raw mut (*morph_args).morph_key as *mut ::core::ffi::c_char;
        } else if mask as Morph_flags & (*morph_args).morph_flags != 0 {
            *index += 1;
            return &raw mut (*morph_args).morph_key as *mut ::core::ffi::c_char;
        }
        *index += 1;
        morph_args = morph_args.offset(1);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn NameOfDerivtype(mut st: Derivtype) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(st as ::core::ffi::c_long, arg_derivtype);
}
#[no_mangle]
pub unsafe extern "C" fn NameOfStemtype(mut st: Stemtype) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(st as ::core::ffi::c_long, arg_stemtype);
}
#[no_mangle]
pub unsafe extern "C" fn NameOfDomain(mut st: Stemtype) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(st as ::core::ffi::c_long, arg_domain);
}
#[no_mangle]
pub unsafe extern "C" fn NameOfPerson(mut vf: word_form) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(
        vf.f_person() as ::core::ffi::c_long,
        &raw mut arg_person as *mut Morph_args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NameOfNumber(mut vf: word_form) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(
        vf.f_number() as ::core::ffi::c_long,
        &raw mut arg_number as *mut Morph_args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NameOfTense(mut vf: word_form) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(
        vf.f_tense() as ::core::ffi::c_long,
        &raw mut arg_tense as *mut Morph_args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NameOfMood(mut vf: word_form) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(
        vf.f_mood() as ::core::ffi::c_long,
        &raw mut arg_mood as *mut Morph_args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NameOfVoice(mut vf: word_form) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(
        vf.f_voice() as ::core::ffi::c_long,
        &raw mut arg_voice as *mut Morph_args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NameOfDialect(mut di: Dialect) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(
        di as ::core::ffi::c_long,
        &raw mut arg_dialect as *mut Morph_args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn DomainNames(
    mut domp: *mut ::core::ffi::c_char,
    mut res: *mut ::core::ffi::c_char,
    mut dels: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = domp;
    while *p != 0 {
        if *res != 0 {
            Xstrncat(res, dels, MAXWORDSIZE as size_t);
        }
        Xstrncat(res, NameOfDomain(*p as Stemtype), MAXWORDSIZE as size_t);
        p = p.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn DialectNames(
    mut di: Dialect,
    mut res: *mut ::core::ffi::c_char,
    mut dels: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut mask: Dialect = 1 as Dialect;
    let mut sofar: Dialect = 0 as Dialect;
    let mut mf: Morph_flags = 0;
    let mut morph_args: *mut Morph_args = ::core::ptr::null_mut::<Morph_args>();
    morph_args = &raw mut arg_dialect as *mut Morph_args;
    *res = 0 as ::core::ffi::c_char;
    if di == 0 {
        return 0;
    }
    mf = di as Morph_flags;
    while (*morph_args).morph_key[0 as ::core::ffi::c_int as usize] != 0 {
        if (*morph_args).morph_flags != 0
            && mf & (*morph_args).morph_flags == (*morph_args).morph_flags
        {
            if *res != 0 {
                Xstrncat(res, dels, LONGSTRING as size_t);
            }
            Xstrncat(
                res,
                &raw mut (*morph_args).morph_key as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            mf &= !(*morph_args).morph_flags;
        }
        morph_args = morph_args.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn GeogRegionNames(
    mut gr: GeogRegion,
    mut res: *mut ::core::ffi::c_char,
    mut dels: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut mask: GeogRegion = 1 as GeogRegion;
    let mut sofar: GeogRegion = 0 as GeogRegion;
    let mut mf: Morph_flags = 0;
    let mut morph_args: *mut Morph_args = ::core::ptr::null_mut::<Morph_args>();
    morph_args = &raw mut arg_geogregion as *mut Morph_args;
    *res = 0 as ::core::ffi::c_char;
    if gr == 0 {
        return 0;
    }
    mf = gr as Morph_flags;
    while (*morph_args).morph_key[0 as ::core::ffi::c_int as usize] != 0 {
        if (*morph_args).morph_flags != 0
            && mf & (*morph_args).morph_flags == (*morph_args).morph_flags
        {
            if *res != 0 {
                Xstrncat(res, dels, LONGSTRING as size_t);
            }
            Xstrncat(
                res,
                &raw mut (*morph_args).morph_key as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            mf &= !(*morph_args).morph_flags;
        }
        morph_args = morph_args.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn NameOfGender(mut af: word_form) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(
        af.f_gender() as ::core::ffi::c_long,
        &raw mut arg_gender as *mut Morph_args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NameOfCase(mut af: word_form) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(
        af.f_case() as ::core::ffi::c_long,
        &raw mut arg_case as *mut Morph_args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NameOfDegree(mut wf: word_form) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(
        wf.f_degree() as ::core::ffi::c_long,
        &raw mut arg_degree as *mut Morph_args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NameOfMorphFlags(
    mut mf: ::core::ffi::c_long,
) -> *mut ::core::ffi::c_char {
    if keys_inited == 0 {
        init_keys();
    }
    return p_eq_morph_keys(mf, &raw mut arg_morphflags as *mut Morph_args);
}
#[no_mangle]
pub unsafe extern "C" fn MatchMorphKey(
    mut field: *mut ::core::ffi::c_char,
) -> *mut Morph_args {
    let mut rval: ::core::ffi::c_int = 0;
    if keys_inited == 0 {
        init_keys();
    }
    rval = binlook(
        key_table as *mut ::core::ffi::c_char,
        field,
        nkeys,
        ::core::mem::size_of::<*mut Morph_args>() as ::core::ffi::c_int,
        1 as bool_0,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut *mut Morph_args,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                keycomp2
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut *mut Morph_args,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
    if rval < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<Morph_args>();
    }
    return *key_table.offset(rval as isize);
}
#[no_mangle]
pub unsafe extern "C" fn GetStemNum(mut field: *mut ::core::ffi::c_char) -> Stemtype {
    let mut mp: *mut Morph_args = ::core::ptr::null_mut::<Morph_args>();
    mp = MatchMorphKey(field);
    if mp.is_null() {
        return 0 as ::core::ffi::c_int as Stemtype;
    }
    return (*mp).morph_flags as Stemtype;
}
unsafe extern "C" fn p_eq_morph_keys(
    mut flag: ::core::ffi::c_long,
    mut morph_args: *mut Morph_args,
) -> *mut ::core::ffi::c_char {
    if flag == 0 {
        return b"\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    }
    while (*morph_args).morph_key[0 as ::core::ffi::c_int as usize] != 0 {
        if flag == (*morph_args).morph_flags {
            return &raw mut (*morph_args).morph_key as *mut ::core::ffi::c_char;
        }
        morph_args = morph_args.offset(1);
    }
    return b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn InitStemSuffs(
    mut fname: *mut ::core::ffi::c_char,
    mut curfunc: Option<unsafe extern "C" fn() -> ()>,
    mut classfunc: Option<unsafe extern "C" fn() -> Stemtype>,
    mut snum: *mut ::core::ffi::c_int,
) -> *mut Morph_args {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut stemnum: Stemtype = 0 as Stemtype;
    let mut declnum: Stemtype = 0 as Stemtype;
    let mut stemname: [::core::ffi::c_char; 60] = [0; 60];
    let mut decl: [::core::ffi::c_char; 60] = [0; 60];
    let mut targs: *mut Morph_args = ::core::ptr::null_mut::<Morph_args>();
    f = MorphFopen(
        fname,
        b"r\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"could not open [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
            fname,
        );
        return ::core::ptr::null_mut::<Morph_args>();
    }
    while GetTableLine(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
        f,
    ) != 0
    {
        *snum += 1;
    }
    fseek(f, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    targs = calloc(
        (*snum + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<Morph_args>(),
    ) as *mut Morph_args;
    i = 0 as ::core::ffi::c_int;
    while GetTableLine(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
        f,
    ) != 0
    {
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        stemnum = 0 as Stemtype;
        declnum = stemnum;
        if has_octal(&raw mut line as *mut ::core::ffi::c_char) != 0 {
            sscanf(
                &raw mut line as *mut ::core::ffi::c_char,
                b"%s %o %s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut stemname as *mut ::core::ffi::c_char,
                &raw mut n,
                &raw mut decl as *mut ::core::ffi::c_char,
            );
            stemnum = n as Stemtype;
        } else {
            sscanf(
                &raw mut line as *mut ::core::ffi::c_char,
                b"%s %d %s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut stemname as *mut ::core::ffi::c_char,
                &raw mut n,
                &raw mut decl as *mut ::core::ffi::c_char,
            );
            stemnum = n as Stemtype;
        }
        Xstrncpy(
            &raw mut (*targs.offset(i as isize)).morph_key as *mut ::core::ffi::c_char,
            &raw mut stemname as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        declnum = ::core::mem::transmute::<
            _,
            unsafe extern "C" fn(_) -> Stemtype,
        >(
            Some(classfunc.expect("non-null function pointer"))
                .expect("non-null function pointer"),
        )(&raw mut decl as *mut ::core::ffi::c_char);
        (*targs.offset(i as isize)).morph_flags = (stemnum | declnum) as Morph_flags;
        let ref mut fresh2 = (*targs.offset(i as isize)).add_val;
        *fresh2 = curfunc;
        i += 1;
    }
    fclose(f);
    return targs;
}
#[no_mangle]
pub unsafe extern "C" fn init_stems() -> ::core::ffi::c_int {
    arg_stemtype = InitStemSuffs(
        STEMTYPES.as_ptr() as *mut ::core::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                new_stemtype
                    as unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> Stemtype>,
            Option<unsafe extern "C" fn() -> Stemtype>,
        >(
            Some(
                GetStemClass
                    as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> Stemtype,
            ),
        ),
        &raw mut nstems,
    );
    arg_derivtype = InitStemSuffs(
        DERIVTYPES.as_ptr() as *mut ::core::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                new_derivtype
                    as unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> Stemtype>,
            Option<unsafe extern "C" fn() -> Stemtype>,
        >(
            Some(
                GetStemClass
                    as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> Stemtype,
            ),
        ),
        &raw mut nderivs,
    );
    arg_domain = InitStemSuffs(
        DOMAINLIST.as_ptr() as *mut ::core::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                new_domain
                    as unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> Stemtype>,
            Option<unsafe extern "C" fn() -> Stemtype>,
        >(
            Some(
                GetIsProse as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> Stemtype,
            ),
        ),
        &raw mut ndomains,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn has_octal(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *s as ::core::ffi::c_int != 0 && __isspace(*s as ::core::ffi::c_int) == 0 {
        s = s.offset(1);
    }
    while __isspace(*s as ::core::ffi::c_int) != 0 {
        s = s.offset(1);
    }
    if *s as ::core::ffi::c_int == '0' as i32 {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init_keys() -> ::core::ffi::c_int {
    let mut sofar: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    extern "C" {
        #[link_name = "morph_key_comp"]
        fn morph_key_comp_0() -> ::core::ffi::c_int;
    }
    keys_inited += 1;
    init_stems();
    nkeys = ((nstems + nderivs + ndomains) as usize)
        .wrapping_add(
            (::core::mem::size_of::<[Morph_args; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<Morph_args>() as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<[Morph_args; 10]>() as usize)
                .wrapping_div(::core::mem::size_of::<Morph_args>() as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<[Morph_args; 92]>() as usize)
                .wrapping_div(::core::mem::size_of::<Morph_args>() as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<[Morph_args; 12]>() as usize)
                .wrapping_div(::core::mem::size_of::<Morph_args>() as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<[Morph_args; 21]>() as usize)
                .wrapping_div(::core::mem::size_of::<Morph_args>() as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<[Morph_args; 7]>() as usize)
                .wrapping_div(::core::mem::size_of::<Morph_args>() as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<[Morph_args; 17]>() as usize)
                .wrapping_div(::core::mem::size_of::<Morph_args>() as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<[Morph_args; 11]>() as usize)
                .wrapping_div(::core::mem::size_of::<Morph_args>() as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<[Morph_args; 22]>() as usize)
                .wrapping_div(::core::mem::size_of::<Morph_args>() as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<[Morph_args; 13]>() as usize)
                .wrapping_div(::core::mem::size_of::<Morph_args>() as usize),
        )
        .wrapping_add(
            (::core::mem::size_of::<[Morph_args; 17]>() as usize)
                .wrapping_div(::core::mem::size_of::<Morph_args>() as usize),
        ) as ::core::ffi::c_int;
    key_table = calloc(
        (nkeys as size_t).wrapping_add(1 as size_t),
        ::core::mem::size_of::<*mut Morph_args>(),
    ) as *mut *mut Morph_args;
    sofar += add_keyarr(key_table.offset(sofar as isize), arg_stemtype);
    sofar += add_keyarr(key_table.offset(sofar as isize), arg_derivtype);
    sofar += add_keyarr(key_table.offset(sofar as isize), arg_domain);
    sofar
        += add_keyarr(
            key_table.offset(sofar as isize),
            &raw mut arg_degree as *mut Morph_args,
        );
    sofar
        += add_keyarr(
            key_table.offset(sofar as isize),
            &raw mut arg_person as *mut Morph_args,
        );
    sofar
        += add_keyarr(
            key_table.offset(sofar as isize),
            &raw mut arg_gender as *mut Morph_args,
        );
    sofar
        += add_keyarr(
            key_table.offset(sofar as isize),
            &raw mut arg_case as *mut Morph_args,
        );
    sofar
        += add_keyarr(
            key_table.offset(sofar as isize),
            &raw mut arg_number as *mut Morph_args,
        );
    sofar
        += add_keyarr(
            key_table.offset(sofar as isize),
            &raw mut arg_tense as *mut Morph_args,
        );
    sofar
        += add_keyarr(
            key_table.offset(sofar as isize),
            &raw mut arg_voice as *mut Morph_args,
        );
    sofar
        += add_keyarr(
            key_table.offset(sofar as isize),
            &raw mut arg_mood as *mut Morph_args,
        );
    sofar
        += add_keyarr(
            key_table.offset(sofar as isize),
            &raw mut arg_dialect as *mut Morph_args,
        );
    sofar
        += add_keyarr(
            key_table.offset(sofar as isize),
            &raw mut arg_geogregion as *mut Morph_args,
        );
    sofar
        += add_keyarr(
            key_table.offset(sofar as isize),
            &raw mut arg_morphflags as *mut Morph_args,
        );
    qsort(
        key_table as *mut ::core::ffi::c_void,
        sofar as size_t,
        ::core::mem::size_of::<*mut Morph_args>(),
        Some(
            keycomp1
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    nkeys = sofar;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn keycomp1(
    mut k1: *const ::core::ffi::c_void,
    mut k2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut m1: *mut *mut Morph_args = ::core::ptr::null_mut::<*mut Morph_args>();
    let mut m2: *mut *mut Morph_args = ::core::ptr::null_mut::<*mut Morph_args>();
    m1 = k1 as *mut *mut Morph_args;
    m2 = k2 as *mut *mut Morph_args;
    return strcmp(
        &raw mut (**m1).morph_key as *mut ::core::ffi::c_char,
        &raw mut (**m2).morph_key as *mut ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn keycomp2(
    mut s: *mut ::core::ffi::c_char,
    mut kp: *mut *mut Morph_args,
) -> ::core::ffi::c_int {
    let mut m: *mut Morph_args = ::core::ptr::null_mut::<Morph_args>();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    m = *kp;
    rval = strcmp(s, &raw mut (*m).morph_key as *mut ::core::ffi::c_char);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn add_keyarr(
    mut ktab: *mut *mut Morph_args,
    mut morph_args: *mut Morph_args,
) -> ::core::ffi::c_int {
    let mut ms: *mut Morph_args = morph_args;
    while (*morph_args).morph_key[0 as ::core::ffi::c_int as usize] != 0 {
        let fresh0 = morph_args;
        morph_args = morph_args.offset(1);
        let fresh1 = ktab;
        ktab = ktab.offset(1);
        *fresh1 = fresh0;
    }
    return morph_args.offset_from(ms) as ::core::ffi::c_long as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GetStemClass(mut classp: *mut ::core::ffi::c_char) -> Stemtype {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[C2RustUnnamed; 28]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
            as ::core::ffi::c_int
    {
        if strcmp(
            classp,
            &raw mut (*(&raw mut arg_stemclass as *mut C2RustUnnamed).offset(i as isize))
                .class_name as *mut ::core::ffi::c_char,
        ) == 0
        {
            return arg_stemclass[i as usize].class_num as Stemtype;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int) as Stemtype;
}
#[no_mangle]
pub unsafe extern "C" fn GetIsProse(mut classp: *mut ::core::ffi::c_char) -> Stemtype {
    if strcmp(classp, b"prose\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return PROSEAUTHOR as Stemtype;
    }
    return 0 as ::core::ffi::c_int as Stemtype;
}
#[no_mangle]
pub unsafe extern "C" fn AddMorphKey(
    mut gstr: *mut gk_string,
    mut field: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut func: Option<unsafe extern "C" fn(*mut gk_string, Morph_flags) -> ()> = None;
    let mut mp: *mut Morph_args = ::core::ptr::null_mut::<Morph_args>();
    mp = MatchMorphKey(field);
    if mp.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    func = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        Option<unsafe extern "C" fn(*mut gk_string, Morph_flags) -> ()>,
    >((*mp).add_val);
    Some(func.expect("non-null function pointer"))
        .expect("non-null function pointer")(gstr, (*mp).morph_flags);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut arg_stemclass: [C2RustUnnamed; 28] = unsafe {
    [
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"adj1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (ADJSTEM | DECL1 | DECL2) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"adj2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (ADJSTEM | DECL1 | DECL2) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"adj3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (ADJSTEM | DECL3) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"noun1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (NOUNSTEM | DECL1) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"noun2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (NOUNSTEM | DECL2) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"noun3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (NOUNSTEM | DECL3) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"noun4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (NOUNSTEM | DECL4) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"noun5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (NOUNSTEM | DECL5) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"indecl1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (INDECL | NOUNSTEM | DECL1) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"indecl2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (INDECL | NOUNSTEM | DECL2) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"indecl3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (INDECL | NOUNSTEM | DECL3) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pron1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (INDECL | NOUNSTEM | DECL1 | DECL2) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pron3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (INDECL | NOUNSTEM | DECL3) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pp_pr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: PP_PR as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pp_fu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: PP_FU as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pp_ao\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: PP_AO as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pp_pf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: PP_PF as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pp_pp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: PP_PP as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pp_ap\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: PP_AP as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pp_fp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: PP_FP as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pp_p4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: PP_SU as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pp_va\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: PP_VA as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pp_vn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: PP_VN as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"verbstem\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: VERBSTEM as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"prim_deriv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (VERBSTEM | PRIM_CONJ) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"reg_deriv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: (VERBSTEM | REG_CONJ) as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"indecl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            class_num: INDECL as ::core::ffi::c_long,
        },
        C2RustUnnamed {
            class_name: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            class_num: 0,
        },
    ]
};
#[no_mangle]
pub static mut arg_degree: [Morph_args; 5] = unsafe {
    [
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"comp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: COMPARATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_degree
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"comparative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: COMPARATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_degree
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"superl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SUPERLATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_degree
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"superlative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SUPERLATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_degree
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            morph_flags: 0,
            add_val: None,
        },
    ]
};
#[no_mangle]
pub static mut arg_person: [Morph_args; 10] = unsafe {
    [
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"1st\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERS1 as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_person
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"first\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERS1 as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_person
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pers1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERS1 as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_person
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"2nd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERS2 as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_person
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"second\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERS2 as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_person
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pers2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERS2 as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_person
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"3rd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERS3 as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_person
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"third\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERS3 as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_person
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pers3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERS3 as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_person
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            morph_flags: 0,
            add_val: None,
        },
    ]
};
#[no_mangle]
pub static mut arg_morphflags: [Morph_args; 92] = unsafe {
    [
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"syll_augment\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SYLL_AUGMENT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"syll_aug\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SYLL_AUGMENT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"comp_only\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: COMP_ONLY as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"not_in_comp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NOT_IN_COMPOSITION as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"enclitic\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ENCLITIC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"proclitic\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PROCLITIC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"iterative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ITERATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ant_acc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ANT_ACC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"stem_acc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: STEM_ACC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pen_acc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: STEM_ACC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"suff_acc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SUFF_ACC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ult_acc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SUFF_ACC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"rec_acc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: REC_ACC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"needs_acc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NEEDS_ACCENT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"contr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: CONTRACTED as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"contracted\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: CONTRACTED as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"uncontr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: UNCONTR_END as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"uncontr_end\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: UNCONTR_END as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"uncontracted\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: UNCONTR_END as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"uncontr_stem\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: UNCONTR_STEM as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pers_name\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERS_NAME as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"is_group\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: GROUP_NAME as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"prevb_aug\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PREVB_AUGMENT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"prevb_augment\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PREVB_AUGMENT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"double_aug\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: DOUBLE_AUGMENT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"double_augment\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: DOUBLE_AUGMENT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"no_comp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NO_COMP as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"irreg_comp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IRREG_COMP as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"irrcomp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IRREG_COMP as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"irreg_superl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IRREG_SUPERL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"irrsuperl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IRREG_SUPERL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"short_pen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SHORT_PEN as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"long_pen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: LONG_PEN as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"r_e_i_alpha\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: R_E_I_ALPHA as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"unaugmented\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: UNAUGMENTED as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"apocope\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: APOCOPE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"has_augment\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: HAS_AUGMENT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"nu_movable\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NU_MOVABLE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"interv_s_to_h\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: INTERV_S_TO_H as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"poetic\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: POETIC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"dissimilation\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: DISSIMILATION as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"metath\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: METATHESIS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"metathesis\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: METATHESIS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"elide_preverb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ELIDE_PREVERB as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"root_preverb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ROOT_PREVERB as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"diminutive\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: DIMINUTIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"early\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: EARLY as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"late\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: LATE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"rare\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: RARE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"raw_preverb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: RAW_PREVERB as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"short_subj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SHORT_SUBJ as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"unasp_preverb\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: UNASP_PREVERB as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"redupl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: REDUPL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"attic_redupl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ATTIC_REDUPL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"is_deriv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IS_DERIV as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"no_redupl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NO_REDUPL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"n_infix\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: N_INFIX as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"syncope\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SYNCOPE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"impersonal\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IMPERSONAL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"indeclform\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: INDECLFORM as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"needs_rbreath\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NEEDS_RBREATH as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"no_circumflex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NO_CIRCUMFLEX as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"causal\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: CAUSAL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"intrans\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: INTRANS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"tmesis\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: TMESIS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"raw_sonant\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: RAW_SONANT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"prodelision\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PRODELISION as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"frequentat\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: FREQUENTAT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"frequentative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: FREQUENTAT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"desiderative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: DESIDERATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"impers\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IMPERSONAL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"later\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: LATER as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"double_redupl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: DOUBLE_REDUPL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pres_redupl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PRES_REDUPL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ends_in_dig\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ENDS_IN_DIGAMMA as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ends_in_digamma\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ENDS_IN_DIGAMMA as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"geog_name\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: GEOG_NAME as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"doubled_cons\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: DOUBLED_CONS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"iota_intens\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IOTA_INTENS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"sig_to_ci\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SIG_TO_CI as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"short_eis\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SHORT_EIS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pros_to_poti\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PROS_TO_POTI as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pros_to_proti\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PROS_TO_PROTI as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"meta_to_peda\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: META_TO_PEDA as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"upo_to_upai\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: UPO_TO_UPAI as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"para_to_parai\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PARA_TO_PARAI as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"uper_to_upeir\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: UPER_TO_UPEIR as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"en_to_eni\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: EN_TO_ENI as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"a_priv\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: A_PRIV as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"a_copul\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: A_COPUL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"metrical_long\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: METRICAL_LONG as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_morphflags
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            morph_flags: 0,
            add_val: None,
        },
    ]
};
#[no_mangle]
pub static mut arg_gender: [Morph_args; 12] = unsafe {
    [
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"masc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: MASCULINE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_gender
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"masculine\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: MASCULINE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_gender
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"fem\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: FEMININE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_gender
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"feminine\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: FEMININE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_gender
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"neut\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NEUTER as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_gender
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"neuter\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NEUTER as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_gender
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"masc/neut\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (MASCULINE | NEUTER) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_gender
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"masc/fem\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (MASCULINE | FEMININE) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_gender
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"masc/fem/neut\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (MASCULINE | FEMININE | NEUTER) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_gender
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"common\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (MASCULINE | FEMININE | NEUTER) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_gender
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"adverbial\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ADVERBIAL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_gender
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            morph_flags: 0,
            add_val: None,
        },
    ]
};
#[no_mangle]
pub static mut arg_case: [Morph_args; 21] = unsafe {
    [
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"nom/voc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (NOMINATIVE | VOCATIVE) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"nom/acc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (NOMINATIVE | ACCUSATIVE) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"nom/voc/acc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (NOMINATIVE | VOCATIVE | ACCUSATIVE) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"gen/dat\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (GENITIVE | DATIVE) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"nom\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NOMINATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"nominative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NOMINATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"gen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: GENITIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"genitive\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: GENITIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"abl/dat\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (ABLATIVE | DATIVE) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"dat/abl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (ABLATIVE | DATIVE) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ablative/dative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (ABLATIVE | DATIVE) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"dative/ablative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (ABLATIVE | DATIVE) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"abl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ABLATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ablative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ABLATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"dat\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: DATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"dative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: DATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"acc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ACCUSATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"accusative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ACCUSATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"voc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: VOCATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"voctive\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: VOCATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_case
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            morph_flags: 0,
            add_val: None,
        },
    ]
};
#[no_mangle]
pub static mut arg_number: [Morph_args; 7] = unsafe {
    [
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"sg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SINGULAR as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_number
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"sing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SINGULAR as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_number
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"singular\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SINGULAR as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_number
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PLURAL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_number
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"plural\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PLURAL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_number
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"dual\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: DUAL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_number
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            morph_flags: 0,
            add_val: None,
        },
    ]
};
#[no_mangle]
pub static mut arg_tense: [Morph_args; 17] = unsafe {
    [
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pres\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PRESENT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"present\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PRESENT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"fut\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: FUTURE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"future\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: FUTURE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"aor\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: AORIST as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"aorist\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: AORIST as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"perf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERFECT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"perfect\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PERFECT as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"imperf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IMPERF as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"imperfect\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IMPERF as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"plup\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PLUPERF as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pluperfect\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PLUPERF as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"futperf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: FUTPERF as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"futperfect\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: FUTPERF as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pastabs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PASTABSOLUTE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pastabsolute\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PASTABSOLUTE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_tense
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            morph_flags: 0,
            add_val: None,
        },
    ]
};
#[no_mangle]
pub static mut arg_voice: [Morph_args; 11] = unsafe {
    [
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"mid\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: MIDDLE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_voice
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"middle\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: MIDDLE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_voice
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"act\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ACTIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_voice
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"active\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ACTIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_voice
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"pass\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PASSIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_voice
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"passive\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PASSIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_voice
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"dep\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (ACTIVE | MIDDLE) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_voice
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"mp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: MEDIO_PASS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_voice
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"medio_pass\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: MEDIO_PASS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_voice
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"medio-pass\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: MEDIO_PASS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_voice
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            morph_flags: 0,
            add_val: None,
        },
    ]
};
#[no_mangle]
pub static mut arg_mood: [Morph_args; 22] = unsafe {
    [
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ind\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: INDICATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"indic\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: INDICATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"indicative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: INDICATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"subj\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SUBJUNCTIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"subjunc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SUBJUNCTIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"subjunctive\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SUBJUNCTIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"part\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PARTICIPLE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"partic\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PARTICIPLE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"participle\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PARTICIPLE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"opt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: OPTATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"optat\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: OPTATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"optative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: OPTATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"imperat\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IMPERATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"imperative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IMPERATIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"inf\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: INFINITIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"infin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: INFINITIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"infinitive\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: INFINITIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"gerundive\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: GERUNDIVE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"supine\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: SUPINE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"cond\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: CONDITIONAL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"conditional\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: CONDITIONAL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_mood
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            morph_flags: 0,
            add_val: None,
        },
    ]
};
#[no_mangle]
pub static mut arg_dialect: [Morph_args; 13] = unsafe {
    [
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"attic\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ATTIC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"att\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ATTIC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"epic\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: EPIC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"homeric\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: HOMERIC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"non_homeric_epic\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: NON_HOMERIC_EPIC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"doric\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: DORIC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ionic\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: IONIC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"aeolic\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: AEOLIC as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"parad_form\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PARADIGM as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"all_dial\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ALL_DIAL as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"ionic/homeric\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: (IONIC | HOMERIC) as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"prose\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PROSE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_dialect
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            morph_flags: 0,
            add_val: None,
        },
    ]
};
#[no_mangle]
pub static mut arg_geogregion: [Morph_args; 17] = unsafe {
    [
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"phocis\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: PHOCIS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"locris\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: LOCRIS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"elis\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ELIS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"locris\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: LOCRIS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"laconia\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: LACONIA as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"heraclea\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: HERACLEA as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"megarid\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: MEGARID as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"argolid\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ARGOLID as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"rhodes\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: RHODES as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"cos\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: COS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"thera\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: THERA as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"cyrene\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: CYRENE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"crete\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: CRETE as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"arcadia\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: ARCADIA as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"cyprus\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: CYPRUS as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: ::core::mem::transmute::<
                [u8; 60],
                [::core::ffi::c_char; 60],
            >(
                *b"boeotia\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            morph_flags: BOEOTIA as Morph_flags,
            add_val: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut gk_string, ::core::ffi::c_ulong) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    new_region
                        as unsafe extern "C" fn(
                            *mut gk_string,
                            ::core::ffi::c_ulong,
                        ) -> (),
                ),
            ),
        },
        Morph_args {
            morph_key: [
                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            morph_flags: 0,
            add_val: None,
        },
    ]
};
#[no_mangle]
pub static mut arg_stemtype: *mut Morph_args = ::core::ptr::null::<Morph_args>()
    as *mut Morph_args;
#[no_mangle]
pub static mut arg_derivtype: *mut Morph_args = ::core::ptr::null::<Morph_args>()
    as *mut Morph_args;
#[no_mangle]
pub static mut arg_domain: *mut Morph_args = ::core::ptr::null::<Morph_args>()
    as *mut Morph_args;
pub const STEMTYPES: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<
        [u8; 27],
        [::core::ffi::c_char; 27],
    >(*b"rule_files/stemtypes.table\0")
};
pub const DERIVTYPES: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<
        [u8; 28],
        [::core::ffi::c_char; 28],
    >(*b"rule_files/derivtypes.table\0")
};
pub const DOMAINLIST: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<
        [u8; 28],
        [::core::ffi::c_char; 28],
    >(*b"rule_files/domainlist.table\0")
};
