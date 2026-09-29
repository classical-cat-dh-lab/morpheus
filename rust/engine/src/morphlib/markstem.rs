use ::c2rust_bitfields;
extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getsyll(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ends_in(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn getquantity(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: bool_0,
        _: bool_0,
    ) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn longbyposition(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type int32 = ::core::ffi::c_uint;
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
pub const ADJSTEM: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const NOUNSTEM: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I_ERR: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const PARTICIPLE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ULTIMA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LONG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SHORT_PEN: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const LONG_PEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const R_E_I_ALPHA: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const IS_DERIV: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const ENDS_IN_DIGAMMA: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn markstem(
    mut stemstr: *mut ::core::ffi::c_char,
    mut gstr: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut lastp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    lastp = stemstr;
    while *lastp != 0 {
        lastp = lastp.offset(1);
    }
    while lastp > stemstr
        && (if 0 as ::core::ffi::c_int != 0 {
            isalpha(*lastp as ::core::ffi::c_int)
        } else {
            ((*lastp as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) == 0
    {
        lastp = lastp.offset(-1);
    }
    if ((*gstr).gs_steminfo & 0o10000 as Stemtype != 0
        || (*gstr).gs_steminfo & 0o4000 as Stemtype != 0
        || (*gstr).gs_forminfo.f_mood() as ::core::ffi::c_int == PARTICIPLE
        || has_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, IS_DERIV)
            != 0)
        && (*lastp as ::core::ffi::c_int == 'r' as i32
            || *lastp as ::core::ffi::c_int == 'i' as i32
            || *lastp as ::core::ffi::c_int == 'e' as i32)
    {
        if has_morphflag(
            &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
            ENDS_IN_DIGAMMA,
        ) == 0
        {
            add_morphflag(
                &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                R_E_I_ALPHA,
            );
        }
    }
    if (*gstr).gs_steminfo & 0o4000 as Stemtype != 0 {
        let mut quant: ::core::ffi::c_int = 0;
        quant = getquantity(
            stemstr,
            ULTIMA,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            NO,
            NO,
        );
        if quant != I_ERR {
            if quant == LONG || longbyposition(getsyll(stemstr, ULTIMA)) != 0
                || ends_in(
                    stemstr,
                    b"hi\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                ) != 0
            {
                add_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    LONG_PEN,
                );
            } else {
                add_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    SHORT_PEN,
                );
            }
        }
    }
    return 0;
}
