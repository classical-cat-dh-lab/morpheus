use ::c2rust_bitfields;
extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn CombPbStem(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: Dialect,
        _: *mut MorphFlags,
    ) -> ::core::ffi::c_int;
    fn CpGkAnal(_: *mut gk_word, _: *mut gk_word) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn checkhalf1(_: *mut gk_word, _: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn is_preverb(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn set_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type size_t = usize;
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
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const GRAVE: ::core::ffi::c_int = '\\' as i32;
pub const CIRCUMFLEX: ::core::ffi::c_int = '=' as i32;
pub const HOMERIC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const NON_HOMERIC_EPIC: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const EPIC: ::core::ffi::c_int = NON_HOMERIC_EPIC | HOMERIC;
pub const PROSE: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const APOCOPE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const DOUBLED_CONS: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn strippreverb(
    mut Gkword: *mut gk_word,
    mut endkeys: *mut ::core::ffi::c_char,
    mut rval: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut a: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut b: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut curstem: [::core::ffi::c_char; 60] = [0; 60];
    let mut foo: ::core::ffi::c_int = 0;
    let mut WorkGkword: gk_word = gk_word {
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
    if checkhalf1(Gkword, endkeys) != 0 {
        rval += 1;
    }
    WorkGkword = *Gkword;
    Xstrncpy(
        &raw mut curstem as *mut ::core::ffi::c_char,
        &raw mut WorkGkword.gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    a = &raw mut curstem as *mut ::core::ffi::c_char;
    b = &raw mut (*Gkword).st_rawprvb as *mut ::core::ffi::c_char;
    loop {
        set_morphflag(
            &raw mut WorkGkword.gs_preverb.gs_morphflags as *mut MorphFlags,
            0 as ::core::ffi::c_int,
        );
        if *a == 0 {
            return rval;
        }
        let fresh0 = a;
        a = a.offset(1);
        let fresh1 = b;
        b = b.offset(1);
        *fresh1 = *fresh0;
        *b = 0 as ::core::ffi::c_char;
        Xstrncpy(
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            a,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        if *a as ::core::ffi::c_int == ACUTE || *a as ::core::ffi::c_int == GRAVE
            || *a as ::core::ffi::c_int == CIRCUMFLEX
        {
            continue;
        }
        WorkGkword = *Gkword;
        if !(is_preverb(
            &raw mut WorkGkword.st_rawprvb as *mut ::core::ffi::c_char,
            &raw mut WorkGkword.gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut WorkGkword.gs_preverb,
        ) != 0)
        {
            continue;
        }
        if WorkGkword.gs_stem.gs_gkstring[0 as ::core::ffi::c_int as usize] == 0 {
            continue;
        }
        if CombPbStem(
            &raw mut WorkGkword.st_rawprvb as *mut ::core::ffi::c_char,
            &raw mut WorkGkword.gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            WorkGkword.gs_dialect,
            &raw mut WorkGkword.gs_preverb.gs_morphflags as *mut MorphFlags,
        ) == 0
        {
            continue;
        }
        if has_morphflag(
            &raw mut WorkGkword.gs_preverb.gs_morphflags as *mut MorphFlags,
            APOCOPE,
        ) != 0 && WorkGkword.gs_dialect as ::core::ffi::c_int & PROSE != 0
        {
            continue;
        }
        if doubled_cons(
            &raw mut WorkGkword.gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
        ) != 0
        {
            if (AndDialect(WorkGkword.gs_dialect, EPIC as Dialect) as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                continue;
            }
            if WorkGkword.gs_dialect as ::core::ffi::c_int & PROSE != 0 {
                continue;
            }
            memmove(
                &raw mut WorkGkword.gs_stem.gs_gkstring as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
                (&raw mut WorkGkword.gs_stem.gs_gkstring as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                strlen(
                        (&raw mut WorkGkword.gs_stem.gs_gkstring
                            as *mut ::core::ffi::c_char)
                            .offset(1 as ::core::ffi::c_int as isize),
                    )
                    .wrapping_add(1 as size_t),
            );
            add_morphflag(
                &raw mut WorkGkword.gs_preverb.gs_morphflags as *mut MorphFlags,
                DOUBLED_CONS,
            );
            WorkGkword.gs_stem.gs_dialect = (WorkGkword.gs_stem.gs_dialect
                as ::core::ffi::c_int
                | (0o2000 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int
                    | 31 as ::core::ffi::c_int)) as Dialect;
        }
        if checkhalf1(&raw mut WorkGkword, endkeys) != 0 {
            CpGkAnal(Gkword, &raw mut WorkGkword);
            rval += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn doubled_cons(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if *s as ::core::ffi::c_int == 'r' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    if !((if 0 as ::core::ffi::c_int != 0 {
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
                || *s as ::core::ffi::c_int == 'W' as i32)))
    {
        return 0 as ::core::ffi::c_int;
    }
    if (if 0 as ::core::ffi::c_int != 0 {
        isalpha(*s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
    } else {
        ((*s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
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
                    == 'W' as i32))
    {
        return 0 as ::core::ffi::c_int;
    }
    if *s as ::core::ffi::c_int
        == *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
