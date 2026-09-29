use ::c2rust_bitfields;
extern "C" {
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
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
pub const IONIC: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const HOMERIC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const RHO_ETA_DIAL: ::core::ffi::c_int = EPIC | IONIC;
pub const NON_HOMERIC_EPIC: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const EPIC: ::core::ffi::c_int = NON_HOMERIC_EPIC | HOMERIC;
pub const DECL1: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const DECL2: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const PPARTMASK: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const PARTICIPLE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const COMPARATIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SUPERLATIVE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const ITERATIVE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CONTRACTED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SHORT_PEN: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const LONG_PEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const R_E_I_ALPHA: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const NU_MOVABLE: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const UNCONTR_END: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const IS_DERIV: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const ENDS_IN_DIGAMMA: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn EuphEnd(
    mut want: *mut gk_string,
    mut have: *mut gk_string,
    mut strict: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut d: Dialect = 0;
    let mut curs: *mut ::core::ffi::c_char = &raw mut (*have).gs_gkstring
        as *mut ::core::ffi::c_char;
    if strict != 0
        && has_morphflag(&raw mut (*want).gs_morphflags as *mut MorphFlags, ITERATIVE)
            != 0
        && has_morphflag(&raw mut (*have).gs_morphflags as *mut MorphFlags, ITERATIVE)
            == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if has_morphflag(&raw mut (*want).gs_morphflags as *mut MorphFlags, UNCONTR_END) != 0
        && has_morphflag(&raw mut (*have).gs_morphflags as *mut MorphFlags, CONTRACTED)
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if has_morphflag(&raw mut (*want).gs_morphflags as *mut MorphFlags, NU_MOVABLE) != 0
        && has_morphflag(&raw mut (*have).gs_morphflags as *mut MorphFlags, NU_MOVABLE)
            == 0
    {
        if has_morphflag(&raw mut (*want).gs_morphflags as *mut MorphFlags, IS_DERIV)
            == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *curs as ::core::ffi::c_int == 'h' as i32
        && has_morphflag(
            &raw mut (*want).gs_morphflags as *mut MorphFlags,
            ENDS_IN_DIGAMMA,
        ) == 0
        && ((*have).gs_steminfo & (DECL1 | DECL2) as Stemtype != 0
            || (*have).gs_forminfo.f_mood() as ::core::ffi::c_int == PARTICIPLE
            || has_morphflag(&raw mut (*want).gs_morphflags as *mut MorphFlags, IS_DERIV)
                != 0)
    {
        if has_morphflag(&raw mut (*want).gs_morphflags as *mut MorphFlags, R_E_I_ALPHA)
            != 0
        {
            let mut dial1: Dialect = 0;
            d = AndDialect((*want).gs_dialect, RHO_ETA_DIAL as Dialect);
            if (d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                if (*want).gs_steminfo & 0o70000000 as ::core::ffi::c_int as Stemtype
                    == 0
                    || has_morphflag(
                        &raw mut (*want).gs_morphflags as *mut MorphFlags,
                        IS_DERIV,
                    ) != 0
                {
                    return 0 as ::core::ffi::c_int
                } else {
                    return 1 as ::core::ffi::c_int
                }
            }
            (*have).gs_dialect = (0o2000 as ::core::ffi::c_int
                | 0o100 as ::core::ffi::c_int | 0o10 as ::core::ffi::c_int) as Dialect;
            zap_morphflag(
                &raw mut (*want).gs_morphflags as *mut MorphFlags,
                R_E_I_ALPHA,
            );
        }
    } else if *curs as ::core::ffi::c_int == 'a' as i32
        && *curs.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == HARDLONG
    {
        if has_morphflag(&raw mut (*have).gs_morphflags as *mut MorphFlags, R_E_I_ALPHA)
            != 0
        {
            if has_morphflag(
                &raw mut (*want).gs_morphflags as *mut MorphFlags,
                R_E_I_ALPHA,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            if has_morphflag(
                &raw mut (*want).gs_morphflags as *mut MorphFlags,
                ENDS_IN_DIGAMMA,
            ) != 0
                || has_morphflag(
                    &raw mut (*want).gs_morphflags as *mut MorphFlags,
                    R_E_I_ALPHA,
                ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    if (*want).gs_forminfo.f_degree() as ::core::ffi::c_int == COMPARATIVE
        && !((*have).gs_forminfo.f_degree() as ::core::ffi::c_int == COMPARATIVE)
    {
        if strict != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*want).gs_forminfo.f_degree() as ::core::ffi::c_int == SUPERLATIVE
        && !((*have).gs_forminfo.f_degree() as ::core::ffi::c_int == SUPERLATIVE)
    {
        if strict != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if has_morphflag(&raw mut (*have).gs_morphflags as *mut MorphFlags, LONG_PEN) != 0
        && has_morphflag(&raw mut (*want).gs_morphflags as *mut MorphFlags, LONG_PEN)
            != has_morphflag(&raw mut (*have).gs_morphflags as *mut MorphFlags, LONG_PEN)
        || has_morphflag(&raw mut (*have).gs_morphflags as *mut MorphFlags, SHORT_PEN)
            != 0
            && has_morphflag(
                &raw mut (*want).gs_morphflags as *mut MorphFlags,
                SHORT_PEN,
            )
                != has_morphflag(
                    &raw mut (*have).gs_morphflags as *mut MorphFlags,
                    SHORT_PEN,
                )
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
