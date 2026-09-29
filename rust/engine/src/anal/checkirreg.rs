use ::c2rust_bitfields;
extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn CheckGenWords(_: *mut gk_word, _: *mut gk_word) -> ::core::ffi::c_int;
    fn CombPbStem(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: Dialect,
        _: *mut MorphFlags,
    ) -> ::core::ffi::c_int;
    fn CpGkAnal(_: *mut gk_word, _: *mut gk_word) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn GenIrregForm(
        _: *mut gk_word,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut gk_word;
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
    fn add_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn addbreath(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn chckirrverb(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn getbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn is_preverb(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn is_substring(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn nextkey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn parsefield(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn set_gwmorphflags(_: *mut gk_word, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn set_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn set_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripdiaer(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripquant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn subchar(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xFree(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn zap_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const PROSE: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
pub const NOBREATH: ::core::ffi::c_int = ' ' as i32;
pub const HAS_PREVERB: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const APOCOPE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const UNASP_PREVERB: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXIRR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
static mut IrrForms: [*mut ::core::ffi::c_char; 3] = [::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char; 3];
static mut IrrKeys: [*mut ::core::ffi::c_char; 3] = [::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char; 3];
static mut init_stor: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn try_irregvb(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut Workword: gk_word = gk_word {
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
    let mut saveirrform: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut keys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut fullpreverb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut unasp_prev: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut irrform: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut rawprvb: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    saveirrform = malloc(MAXWORDSIZE as size_t) as *mut ::core::ffi::c_char;
    keys = malloc(LONGSTRING as size_t) as *mut ::core::ffi::c_char;
    *keys = 0 as ::core::ffi::c_char;
    *saveirrform = *keys;
    if init_stor == 0 {
        init_stor = 1 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < MAXIRR {
            IrrForms[i as usize] = malloc(MAXWORDSIZE as size_t)
                as *mut ::core::ffi::c_char;
            IrrKeys[i as usize] = malloc(LONGSTRING as size_t)
                as *mut ::core::ffi::c_char;
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAXIRR {
        *IrrForms[i as usize] = 0 as ::core::ffi::c_char;
        *IrrKeys[i as usize] = 0 as ::core::ffi::c_char;
        i += 1;
    }
    Workword = *Gkword;
    rawprvb = &raw mut Workword.st_rawprvb as *mut ::core::ffi::c_char;
    fullpreverb = &raw mut Workword.gs_preverb.gs_gkstring as *mut ::core::ffi::c_char;
    irrform = &raw mut Workword.gs_stem.gs_gkstring as *mut ::core::ffi::c_char;
    set_morphflag(
        &raw mut Workword.gs_preverb.gs_morphflags as *mut MorphFlags,
        0 as ::core::ffi::c_int,
    );
    *IrrKeys[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    *IrrKeys[1 as ::core::ffi::c_int as usize] = *IrrKeys[2 as ::core::ffi::c_int
        as usize];
    *IrrKeys[0 as ::core::ffi::c_int as usize] = *IrrKeys[1 as ::core::ffi::c_int
        as usize];
    *fullpreverb = *IrrKeys[0 as ::core::ffi::c_int as usize];
    if *rawprvb != 0 {
        if is_preverb(rawprvb, fullpreverb, &raw mut Workword.gs_preverb) == 0 {
            rval = 0 as ::core::ffi::c_int;
            current_block = 762973551452200390;
        } else {
            add_morphflag(
                &raw mut Workword.gs_stem.gs_morphflags as *mut MorphFlags,
                HAS_PREVERB,
            );
            current_block = 11298138898191919651;
        }
    } else {
        current_block = 11298138898191919651;
    }
    match current_block {
        11298138898191919651 => {
            if has_morphflag(
                &raw mut Workword.gs_preverb.gs_morphflags as *mut MorphFlags,
                APOCOPE,
            ) != 0
            {
                if AndDialect(
                    Workword.gs_dialect,
                    0o4000 as ::core::ffi::c_int as Dialect,
                ) as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                {
                    current_block = 762973551452200390;
                } else {
                    current_block = 6669252993407410313;
                }
            } else {
                current_block = 6669252993407410313;
            }
            match current_block {
                762973551452200390 => {}
                _ => {
                    if *rawprvb as ::core::ffi::c_int != 0
                        && CombPbStem(
                            rawprvb,
                            irrform,
                            Workword.gs_dialect,
                            &raw mut Workword.gs_preverb.gs_morphflags as *mut MorphFlags,
                        ) == 0
                    {
                        rval = 0 as ::core::ffi::c_int;
                    } else {
                        add_morphflags(
                            &raw mut Workword.gs_stem,
                            &raw mut Workword.gs_preverb.gs_morphflags as *mut MorphFlags,
                        );
                        if *rawprvb == 0
                            || (if 0 as ::core::ffi::c_int != 0 {
                                isalpha(*irrform as ::core::ffi::c_int)
                            } else {
                                ((*irrform as ::core::ffi::c_uint
                                    | 32 as ::core::ffi::c_uint)
                                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
                            }) != 0 && *irrform as ::core::ffi::c_int != 'j' as i32
                                && *irrform as ::core::ffi::c_int != 'v' as i32
                                && *irrform as ::core::ffi::c_int != 'J' as i32
                                && *irrform as ::core::ffi::c_int != 'V' as i32
                                && !(*irrform as ::core::ffi::c_int == 'a' as i32
                                    || *irrform as ::core::ffi::c_int == 'e' as i32
                                    || *irrform as ::core::ffi::c_int == 'i' as i32
                                    || *irrform as ::core::ffi::c_int == 'o' as i32
                                    || *irrform as ::core::ffi::c_int == 'u' as i32
                                    || *irrform as ::core::ffi::c_int == 'A' as i32
                                    || *irrform as ::core::ffi::c_int == 'E' as i32
                                    || *irrform as ::core::ffi::c_int == 'I' as i32
                                    || *irrform as ::core::ffi::c_int == 'O' as i32
                                    || *irrform as ::core::ffi::c_int == 'U' as i32
                                    || (*irrform as ::core::ffi::c_int == 'h' as i32
                                        || *irrform as ::core::ffi::c_int == 'w' as i32
                                        || *irrform as ::core::ffi::c_int == 'H' as i32
                                        || *irrform as ::core::ffi::c_int == 'W' as i32))
                            || cur_lang() == LATIN
                        {
                            rval = chckirrvform(
                                irrform,
                                IrrKeys[0 as ::core::ffi::c_int as usize],
                            );
                            if rval != 0 {
                                rval = ChckIrrLemms(
                                    &raw mut Workword,
                                    irrform,
                                    IrrKeys[0 as ::core::ffi::c_int as usize],
                                );
                                current_block = 762973551452200390;
                            } else {
                                current_block = 18377268871191777778;
                            }
                        } else {
                            current_block = 18377268871191777778;
                        }
                        match current_block {
                            762973551452200390 => {}
                            _ => {
                                if *irrform as ::core::ffi::c_int == 'a' as i32
                                    || *irrform as ::core::ffi::c_int == 'e' as i32
                                    || *irrform as ::core::ffi::c_int == 'i' as i32
                                    || *irrform as ::core::ffi::c_int == 'o' as i32
                                    || *irrform as ::core::ffi::c_int == 'u' as i32
                                    || *irrform as ::core::ffi::c_int == 'A' as i32
                                    || *irrform as ::core::ffi::c_int == 'E' as i32
                                    || *irrform as ::core::ffi::c_int == 'I' as i32
                                    || *irrform as ::core::ffi::c_int == 'O' as i32
                                    || *irrform as ::core::ffi::c_int == 'U' as i32
                                    || (*irrform as ::core::ffi::c_int == 'h' as i32
                                        || *irrform as ::core::ffi::c_int == 'w' as i32
                                        || *irrform as ::core::ffi::c_int == 'H' as i32
                                        || *irrform as ::core::ffi::c_int == 'W' as i32)
                                {
                                    if getbreath(irrform) != NOBREATH {
                                        rval = chckirrvform(
                                            irrform,
                                            IrrKeys[0 as ::core::ffi::c_int as usize],
                                        );
                                        if rval != 0 {
                                            Xstrncpy(
                                                IrrForms[0 as ::core::ffi::c_int as usize],
                                                irrform,
                                                MAXWORDSIZE as size_t,
                                            );
                                        } else {
                                            *IrrForms[0 as ::core::ffi::c_int as usize] = 0
                                                as ::core::ffi::c_char;
                                        }
                                    }
                                    Xstrncpy(saveirrform, irrform, MAXWORDSIZE as size_t);
                                    if (cur_lang() != LATIN
                                        && !(*rawprvb
                                            .offset(Xstrlen(rawprvb) as isize)
                                            .offset(-(1 as ::core::ffi::c_int as isize))
                                            as ::core::ffi::c_int == 'f' as i32
                                            || *rawprvb
                                                .offset(Xstrlen(rawprvb) as isize)
                                                .offset(-(1 as ::core::ffi::c_int as isize))
                                                as ::core::ffi::c_int == 'q' as i32
                                            || *rawprvb
                                                .offset(Xstrlen(rawprvb) as isize)
                                                .offset(-(1 as ::core::ffi::c_int as isize))
                                                as ::core::ffi::c_int == 'x' as i32)
                                        || mfi_prvb(rawprvb) != 0) && getbreath(irrform) == NOBREATH
                                    {
                                        addbreath(irrform, SMOOTHBR);
                                        if has_morphflag(
                                            &raw mut Workword.gs_stem.gs_morphflags as *mut MorphFlags,
                                            UNASP_PREVERB,
                                        ) != 0
                                        {
                                            unasp_prev = 1 as ::core::ffi::c_int;
                                            zap_morphflag(
                                                &raw mut Workword.gs_stem.gs_morphflags as *mut MorphFlags,
                                                UNASP_PREVERB,
                                            );
                                        }
                                        rval
                                            += chckirrvform(
                                                irrform,
                                                IrrKeys[0 as ::core::ffi::c_int as usize],
                                            );
                                        if unasp_prev != 0 {
                                            unasp_prev = 0 as ::core::ffi::c_int;
                                            add_morphflag(
                                                &raw mut Workword.gs_stem.gs_morphflags as *mut MorphFlags,
                                                UNASP_PREVERB,
                                            );
                                        }
                                        if rval != 0 {
                                            Xstrncpy(
                                                IrrForms[0 as ::core::ffi::c_int as usize],
                                                irrform,
                                                MAXWORDSIZE as size_t,
                                            );
                                        } else {
                                            *IrrForms[0 as ::core::ffi::c_int as usize] = 0
                                                as ::core::ffi::c_char;
                                        }
                                    }
                                    Xstrncpy(irrform, saveirrform, MAXWORDSIZE as size_t);
                                    if cur_lang() != LATIN && cur_lang() != ITALIAN
                                        && getbreath(irrform) == NOBREATH
                                        && (!(*irrform as ::core::ffi::c_int == 'a' as i32
                                            || *irrform as ::core::ffi::c_int == 'e' as i32
                                            || *irrform as ::core::ffi::c_int == 'i' as i32
                                            || *irrform as ::core::ffi::c_int == 'o' as i32
                                            || *irrform as ::core::ffi::c_int == 'u' as i32
                                            || *irrform as ::core::ffi::c_int == 'A' as i32
                                            || *irrform as ::core::ffi::c_int == 'E' as i32
                                            || *irrform as ::core::ffi::c_int == 'I' as i32
                                            || *irrform as ::core::ffi::c_int == 'O' as i32
                                            || *irrform as ::core::ffi::c_int == 'U' as i32
                                            || (*irrform as ::core::ffi::c_int == 'h' as i32
                                                || *irrform as ::core::ffi::c_int == 'w' as i32
                                                || *irrform as ::core::ffi::c_int == 'H' as i32
                                                || *irrform as ::core::ffi::c_int == 'W' as i32))
                                            || *rawprvb as ::core::ffi::c_int != 0)
                                    {
                                        addbreath(irrform, ROUGHBR);
                                        rval = chckirrvform(
                                            irrform,
                                            IrrKeys[1 as ::core::ffi::c_int as usize],
                                        );
                                        if rval != 0 {
                                            Xstrncpy(
                                                IrrForms[rval as usize],
                                                irrform,
                                                MAXWORDSIZE as size_t,
                                            );
                                        } else {
                                            *IrrForms[1 as ::core::ffi::c_int as usize] = 0
                                                as ::core::ffi::c_char;
                                        }
                                    }
                                    Xstrncpy(irrform, saveirrform, MAXWORDSIZE as size_t);
                                    rval = 0 as ::core::ffi::c_int;
                                    if *IrrForms[1 as ::core::ffi::c_int as usize] != 0 {
                                        rval
                                            += ChckIrrLemms(
                                                &raw mut Workword,
                                                IrrForms[1 as ::core::ffi::c_int as usize],
                                                IrrKeys[1 as ::core::ffi::c_int as usize],
                                            );
                                    }
                                    rval
                                        += ChckIrrLemms(
                                            &raw mut Workword,
                                            IrrForms[0 as ::core::ffi::c_int as usize],
                                            IrrKeys[0 as ::core::ffi::c_int as usize],
                                        );
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    xFree(
        saveirrform,
        b"saveirrform\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    xFree(
        keys,
        b"keys\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    fullpreverb = ::core::ptr::null_mut::<::core::ffi::c_char>();
    keys = fullpreverb;
    saveirrform = keys;
    if rval != 0 {
        CpGkAnal(Gkword, &raw mut Workword);
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn ChckIrrLemms(
    mut Gkword: *mut gk_word,
    mut irrform: *mut ::core::ffi::c_char,
    mut irrkey: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut stemkeys: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut curlemma: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tmpword: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut curkeys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
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
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut curval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while nextkey(irrkey, &raw mut curlemma as *mut ::core::ffi::c_char) != 0 {
        sp = &raw mut curlemma as *mut ::core::ffi::c_char;
        TmpGkword = *Gkword;
        sp = parsefield(
            sp,
            &raw mut tmpword as *mut ::core::ffi::c_char,
            ':' as i32,
            LONGSTRING,
        );
        if tmpword[0 as ::core::ffi::c_int as usize] != 0 {
            Xstrncpy(
                irrform,
                &raw mut tmpword as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
        } else {
            Xstrncpy(
                irrform,
                &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            stripacc(irrform);
            stripquant(irrform);
            stripdiaer(irrform);
        }
        sp = parsefield(
            sp,
            &raw mut TmpGkword.st_lemma as *mut ::core::ffi::c_char,
            ':' as i32,
            LONGSTRING,
        );
        sp = parsefield(
            sp,
            &raw mut stemkeys as *mut ::core::ffi::c_char,
            ' ' as i32,
            LONGSTRING,
        );
        subchar(&raw mut stemkeys as *mut ::core::ffi::c_char, ':' as i32, ' ' as i32);
        curval
            += CheckIrregForm(
                &raw mut TmpGkword,
                irrform,
                &raw mut stemkeys as *mut ::core::ffi::c_char,
            );
        if curval != 0 {
            CpGkAnal(Gkword, &raw mut TmpGkword);
        }
        rval += curval;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn CheckIrregForm(
    mut Gkword: *mut gk_word,
    mut stem: *mut ::core::ffi::c_char,
    mut stemkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut Forms: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut StemForms: gk_word = gk_word {
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
    let mut Gstr: gk_string = gk_string {
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
    let mut prevb: *mut ::core::ffi::c_char = &raw mut (*Gkword).gs_preverb.gs_gkstring
        as *mut ::core::ffi::c_char;
    let mut pbptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    StemForms = *Gkword;
    Xstrncpy(
        &raw mut StemForms.gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
        stem,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    if *prevb != 0 {
        pbptr = is_substring(
            stemkeys,
            b"pb:\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        if pbptr.is_null() {
            Xstrncat(
                stemkeys,
                b" pb:\0" as *const u8 as *const ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            Xstrncat(stemkeys, prevb, LONGSTRING as size_t);
        } else if Xstrncmp(
            pbptr.offset(3 as ::core::ffi::c_int as isize),
            prevb,
            Xstrlen(prevb) as size_t,
        ) != 0
        {
            return 0 as ::core::ffi::c_int
        }
    }
    Forms = GenIrregForm(&raw mut StemForms, stemkeys, 0 as ::core::ffi::c_int);
    set_morphflags(&raw mut Gstr, &raw mut (*Gkword).gs_morphflags as *mut MorphFlags);
    if !Forms.is_null() {
        rval = CheckGenWords(Gkword, Forms);
        FreeGkString(Forms as *mut gk_string);
    }
    set_gwmorphflags(Gkword, &raw mut Gstr.gs_morphflags as *mut MorphFlags);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn chckirrvform(
    mut form: *mut ::core::ffi::c_char,
    mut keys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmpform: [::core::ffi::c_char; 60] = [0; 60];
    let mut rval: ::core::ffi::c_int = 0;
    let mut curacc: ::core::ffi::c_int = 0;
    let mut cursyll: ::core::ffi::c_int = 0;
    rval = chckirrverb(form, keys);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn mfi_prvb(
    mut rawprvb: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut slen: ::core::ffi::c_int = Xstrlen(rawprvb);
    if *rawprvb.offset((slen - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
        == 'f' as i32
        && *rawprvb.offset((slen - 2 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int == 'm' as i32
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
