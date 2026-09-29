use ::c2rust_bitfields;
extern "C" {
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn Xstrncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ends_in(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const PERS3: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const NU_MOVABLE: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn takes_nu_movable(
    mut gstr: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    let mut s: *mut ::core::ffi::c_char = &raw mut tmp as *mut ::core::ffi::c_char;
    strcpy(
        &raw mut tmp as *mut ::core::ffi::c_char,
        &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
    );
    stripacc(s);
    if ends_in(
        s,
        b"si\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0
        || ends_in(
            s,
            b"ci\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        ) != 0
        || ends_in(
            s,
            b"yi\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        ) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    if ends_in(
        s,
        b"e\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    ) != 0 && (*gstr).gs_forminfo.f_person() as ::core::ffi::c_int == PERS3
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn add_numovable(mut gstr: *mut gk_string) -> ::core::ffi::c_int {
    Xstrncat(
        &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
        b"n\0" as *const u8 as *const ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    add_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, NU_MOVABLE);
    return 0;
}
