use ::c2rust_bitfields;
extern "C" {
    fn addbreath(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn getbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn zap_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
pub const NOBREATH: ::core::ffi::c_int = ' ' as i32;
pub const NEEDS_RBREATH: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CheckForBreathing(
    mut gstr: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = &raw mut (*gstr).gs_gkstring
        as *mut ::core::ffi::c_char;
    let mut rbreath: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if cur_lang() == LATIN || cur_lang() == ITALIAN {
        return 0;
    }
    rbreath = has_morphflag(
        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        NEEDS_RBREATH,
    );
    if rbreath != 0 {
        zap_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, NEEDS_RBREATH);
    }
    if !(*s as ::core::ffi::c_int == 'a' as i32 || *s as ::core::ffi::c_int == 'e' as i32
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
            || *s as ::core::ffi::c_int == 'W' as i32)) || getbreath(s) != NOBREATH
    {
        return 0;
    }
    addbreath(s, if rbreath != 0 { ROUGHBR } else { SMOOTHBR });
    return 0;
}
