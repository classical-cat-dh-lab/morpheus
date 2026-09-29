use ::c2rust_bitfields;
extern "C" {
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn is_substring(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const ATTIC: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const IONIC: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const AEOLIC: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const HOMERIC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const DORIC: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const RHO_ETA_DIAL: ::core::ffi::c_int = EPIC | IONIC;
pub const RHO_ALPHA_DIAL: ::core::ffi::c_int = ATTIC | DORIC | AEOLIC;
pub const NON_HOMERIC_EPIC: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const EPIC: ::core::ffi::c_int = NON_HOMERIC_EPIC | HOMERIC;
pub const DECL1: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const DECL2: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const PARTICIPLE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const R_E_I_ALPHA: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const INDECLFORM: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const IS_DERIV: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const MAXEUPHS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn fix_eta(mut gstr: *mut gk_string) -> *mut gk_string {
    let mut euphs: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut orgstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut curs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut d: Dialect = 0;
    euphs = CreatGkString(MAXEUPHS);
    orgstr = &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char;
    if *orgstr as ::core::ffi::c_int == 'h' as i32
        && has_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, INDECLFORM)
            == 0
        && ((*gstr).gs_steminfo & (DECL1 | DECL2) as Stemtype != 0
            || (*gstr).gs_forminfo.f_degree() as ::core::ffi::c_int != 0
            || (*gstr).gs_forminfo.f_mood() as ::core::ffi::c_int == PARTICIPLE
            || has_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, IS_DERIV)
                != 0)
    {
        d = AndDialect((*gstr).gs_dialect, RHO_ALPHA_DIAL as Dialect);
        if d as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            *euphs = *gstr;
            curs = &raw mut (*euphs).gs_gkstring as *mut ::core::ffi::c_char;
            strcpy(curs, b"a_\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(curs, orgstr.offset(1 as ::core::ffi::c_int as isize));
            add_morphflag(
                &raw mut (*euphs).gs_morphflags as *mut MorphFlags,
                R_E_I_ALPHA,
            );
            (*euphs).gs_dialect = 0o2 as Dialect;
            return euphs;
        }
    }
    if (*gstr).gs_dialect as ::core::ffi::c_int != RHO_ETA_DIAL
        && ((*gstr).gs_forminfo.f_degree() as ::core::ffi::c_int != 0
            || (*gstr).gs_forminfo.f_mood() as ::core::ffi::c_int == PARTICIPLE
            || (*gstr).gs_steminfo & (DECL1 | DECL2) as Stemtype != 0)
    {
        if !is_substring(
                b"rh\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            )
            .is_null()
        {
            let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
            let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            *euphs = *gstr;
            orgstr = &raw mut (*euphs).gs_gkstring as *mut ::core::ffi::c_char;
            s = is_substring(
                b"rh\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            );
            *s = 0 as ::core::ffi::c_char;
            s = s.offset(1);
            s = s.offset(1);
            strcpy(&raw mut tmp as *mut ::core::ffi::c_char, s);
            strcat(orgstr, b"ra_\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(orgstr, &raw mut tmp as *mut ::core::ffi::c_char);
            (*euphs).gs_dialect = 0o2 as Dialect;
            (*gstr).gs_dialect = ((*gstr).gs_dialect as ::core::ffi::c_int
                & !(0o2 as ::core::ffi::c_int)) as Dialect;
            return euphs;
        } else if !is_substring(
                b"ih\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            )
            .is_null()
        {
            let mut tmp_0: [::core::ffi::c_char; 60] = [0; 60];
            let mut s_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            *euphs = *gstr;
            orgstr = &raw mut (*euphs).gs_gkstring as *mut ::core::ffi::c_char;
            s_0 = is_substring(
                b"ih\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            );
            *s_0 = 0 as ::core::ffi::c_char;
            s_0 = s_0.offset(1);
            s_0 = s_0.offset(1);
            strcpy(&raw mut tmp_0 as *mut ::core::ffi::c_char, s_0);
            strcat(orgstr, b"ia_\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(orgstr, &raw mut tmp_0 as *mut ::core::ffi::c_char);
            (*euphs).gs_dialect = 0o2 as Dialect;
            (*gstr).gs_dialect = ((*gstr).gs_dialect as ::core::ffi::c_int
                & !(0o2 as ::core::ffi::c_int)) as Dialect;
            return euphs;
        } else if !is_substring(
                b"i!h\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            )
            .is_null()
        {
            let mut tmp_1: [::core::ffi::c_char; 60] = [0; 60];
            let mut s_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            *euphs = *gstr;
            orgstr = &raw mut (*euphs).gs_gkstring as *mut ::core::ffi::c_char;
            s_1 = is_substring(
                b"i!h\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            );
            *s_1 = 0 as ::core::ffi::c_char;
            s_1 = s_1.offset(1);
            s_1 = s_1.offset(1);
            s_1 = s_1.offset(1);
            strcpy(&raw mut tmp_1 as *mut ::core::ffi::c_char, s_1);
            if tmp_1[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == '=' as i32
            {
                strcat(orgstr, b"i!a\0" as *const u8 as *const ::core::ffi::c_char);
            } else {
                strcat(orgstr, b"i!a_\0" as *const u8 as *const ::core::ffi::c_char);
            }
            strcat(orgstr, &raw mut tmp_1 as *mut ::core::ffi::c_char);
            (*euphs).gs_dialect = 0o2 as Dialect;
            (*gstr).gs_dialect = ((*gstr).gs_dialect as ::core::ffi::c_int
                & !(0o2 as ::core::ffi::c_int)) as Dialect;
            return euphs;
        } else if !is_substring(
                b"i_h\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            )
            .is_null()
        {
            let mut tmp_2: [::core::ffi::c_char; 60] = [0; 60];
            let mut s_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            *euphs = *gstr;
            orgstr = &raw mut (*euphs).gs_gkstring as *mut ::core::ffi::c_char;
            s_2 = is_substring(
                b"i_h\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            );
            *s_2 = 0 as ::core::ffi::c_char;
            s_2 = s_2.offset(1);
            s_2 = s_2.offset(1);
            s_2 = s_2.offset(1);
            strcpy(&raw mut tmp_2 as *mut ::core::ffi::c_char, s_2);
            strcat(orgstr, b"i_a_\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(orgstr, &raw mut tmp_2 as *mut ::core::ffi::c_char);
            (*euphs).gs_dialect = 0o2 as Dialect;
            (*gstr).gs_dialect = ((*gstr).gs_dialect as ::core::ffi::c_int
                & !(0o2 as ::core::ffi::c_int)) as Dialect;
            return euphs;
        } else if !is_substring(
                b"i/h\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            )
            .is_null()
        {
            let mut tmp_3: [::core::ffi::c_char; 60] = [0; 60];
            let mut s_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            *euphs = *gstr;
            orgstr = &raw mut (*euphs).gs_gkstring as *mut ::core::ffi::c_char;
            s_3 = is_substring(
                b"i/h\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            );
            *s_3 = 0 as ::core::ffi::c_char;
            s_3 = s_3.offset(1);
            s_3 = s_3.offset(1);
            s_3 = s_3.offset(1);
            strcpy(&raw mut tmp_3 as *mut ::core::ffi::c_char, s_3);
            strcat(orgstr, b"i/a_\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(orgstr, &raw mut tmp_3 as *mut ::core::ffi::c_char);
            (*euphs).gs_dialect = 0o2 as Dialect;
            (*gstr).gs_dialect = ((*gstr).gs_dialect as ::core::ffi::c_int
                & !(0o2 as ::core::ffi::c_int)) as Dialect;
            return euphs;
        } else if !is_substring(
                b"i_/h\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            )
            .is_null()
        {
            let mut tmp_4: [::core::ffi::c_char; 60] = [0; 60];
            let mut s_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            *euphs = *gstr;
            orgstr = &raw mut (*euphs).gs_gkstring as *mut ::core::ffi::c_char;
            s_4 = is_substring(
                b"i_/h\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            );
            *s_4 = 0 as ::core::ffi::c_char;
            s_4 = s_4.offset(1);
            s_4 = s_4.offset(1);
            s_4 = s_4.offset(1);
            s_4 = s_4.offset(1);
            strcpy(&raw mut tmp_4 as *mut ::core::ffi::c_char, s_4);
            strcat(orgstr, b"i_/a_\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(orgstr, &raw mut tmp_4 as *mut ::core::ffi::c_char);
            (*euphs).gs_dialect = 0o2 as Dialect;
            (*gstr).gs_dialect = ((*gstr).gs_dialect as ::core::ffi::c_int
                & !(0o2 as ::core::ffi::c_int)) as Dialect;
            return euphs;
        } else if !is_substring(
                b"e/-h\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            )
            .is_null()
        {
            let mut tmp_5: [::core::ffi::c_char; 60] = [0; 60];
            let mut s_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            *euphs = *gstr;
            orgstr = &raw mut (*euphs).gs_gkstring as *mut ::core::ffi::c_char;
            s_5 = is_substring(
                b"e/-h\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                orgstr,
            );
            *s_5 = 0 as ::core::ffi::c_char;
            strcpy(
                &raw mut tmp_5 as *mut ::core::ffi::c_char,
                s_5
                    .offset(
                        strlen(b"e/-h\0" as *const u8 as *const ::core::ffi::c_char)
                            as isize,
                    ),
            );
            strcat(orgstr, b"e/-a_\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(orgstr, &raw mut tmp_5 as *mut ::core::ffi::c_char);
            (*euphs).gs_dialect = 0o2 as Dialect;
            (*gstr).gs_dialect = ((*gstr).gs_dialect as ::core::ffi::c_int
                & !(0o2 as ::core::ffi::c_int)) as Dialect;
            return euphs;
        }
    }
    FreeGkString(euphs);
    return ::core::ptr::null_mut::<gk_string>();
}
