use ::c2rust_bitfields;
extern "C" {
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
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
pub const DECL3: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const PERFECT: ::core::ffi::c_int = 0o5 as ::core::ffi::c_int;
pub const ACTIVE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const MIDDLE: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const PASSIVE: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const MEDIO_PASS: ::core::ffi::c_int = MIDDLE | PASSIVE;
pub const INFINITIVE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PARTICIPLE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SINGULAR: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const NOMINATIVE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const ACCUSATIVE: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const VOCATIVE: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const NEUTER: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const STEM_ACC: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PERS_NAME: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn penult_form(
    mut gstring: *mut gk_string,
    mut form_info: word_form,
) -> ::core::ffi::c_int {
    let mut stemtype: Stemtype = 0;
    stemtype = (*gstring).gs_steminfo;
    if form_info.f_case() as ::core::ffi::c_int == VOCATIVE
        && form_info.f_number() as ::core::ffi::c_int == SINGULAR
        && stemtype & DECL3 as Stemtype != 0
        && has_morphflag(&raw mut (*gstring).gs_morphflags as *mut MorphFlags, PERS_NAME)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if has_morphflag(&raw mut (*gstring).gs_morphflags as *mut MorphFlags, STEM_ACC) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    if form_info.f_voice() as ::core::ffi::c_int == ACTIVE
        && form_info.f_mood() as ::core::ffi::c_int == PARTICIPLE
        && form_info.f_gender() as ::core::ffi::c_int == NEUTER
        && form_info.f_number() as ::core::ffi::c_int == SINGULAR
        && (form_info.f_case() as ::core::ffi::c_int == NOMINATIVE
            || form_info.f_case() as ::core::ffi::c_int == ACCUSATIVE)
        || form_info.f_voice() as ::core::ffi::c_int & MEDIO_PASS != 0
            && form_info.f_mood() as ::core::ffi::c_int == INFINITIVE
            && form_info.f_tense() as ::core::ffi::c_int == PERFECT
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
