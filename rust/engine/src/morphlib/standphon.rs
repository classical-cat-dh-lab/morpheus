use ::c2rust_bitfields;
extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn islower(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn toupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn addbreath(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
pub const h_AS_ROUGH: ::core::ffi::c_int = '!' as i32;
pub const H_AS_ROUGH: ::core::ffi::c_int = '%' as i32;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const INTERV_S_TO_H: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn stand_phonetics(
    mut Gkword: *mut gk_word,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = &raw mut (*Gkword).st_workword
        as *mut ::core::ffi::c_char;
    let mut lastc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *s != 0 {
        if *s as ::core::ffi::c_int == h_AS_ROUGH {
            if (lastc == 'a' as i32 || lastc == 'e' as i32 || lastc == 'i' as i32
                || lastc == 'o' as i32 || lastc == 'u' as i32 || lastc == 'A' as i32
                || lastc == 'E' as i32 || lastc == 'I' as i32 || lastc == 'O' as i32
                || lastc == 'U' as i32
                || (lastc == 'h' as i32 || lastc == 'w' as i32 || lastc == 'H' as i32
                    || lastc == 'W' as i32))
                && (*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
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
                        || *s.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 'w' as i32
                        || *s.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 'H' as i32
                        || *s.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 'W' as i32))
            {
                *s = 's' as i32 as ::core::ffi::c_char;
                add_morphflag(
                    &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
                    INTERV_S_TO_H,
                );
                (*Gkword).gs_geogregion |= 0o20 as GeogRegion;
            } else {
                strcpy(s, s.offset(1 as ::core::ffi::c_int as isize));
                addbreath(s, ROUGHBR);
            }
            s = s.offset(1);
        } else if *s as ::core::ffi::c_int == H_AS_ROUGH {
            strcpy(s, s.offset(1 as ::core::ffi::c_int as isize));
            addbreath(s, ROUGHBR);
            if if 0 as ::core::ffi::c_int != 0 {
                islower(*s as ::core::ffi::c_int)
            } else {
                ((*s as ::core::ffi::c_uint)
                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
            } != 0
            {
                *s = toupper(*s as ::core::ffi::c_int) as ::core::ffi::c_char;
            }
        } else {
            if if 0 as ::core::ffi::c_int != 0 {
                isalpha(*s as ::core::ffi::c_int)
            } else {
                ((*s as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
            } != 0
            {
                lastc = *s as ::core::ffi::c_int;
            }
            s = s.offset(1);
        }
    }
    return 0;
}
