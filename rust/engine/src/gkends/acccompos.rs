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
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn getsyll2(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn CreatGkword(_: ::core::ffi::c_int) -> *mut gk_word;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn FixPersAcc(
        _: *mut gk_string,
        _: *mut MorphFlags,
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: word_form,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn FixRecAcc(
        _: *mut gk_word,
        _: *mut MorphFlags,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn FreeGkword(_: *mut gk_word) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn hasaccent(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn nsylls(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn set_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn zap_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub const INDECL: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const ADJSTEM: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const NOUNSTEM: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const PPARTMASK: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const PARTICIPLE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ULTIMA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PENULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENCLITIC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SUFF_ACC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const STEM_ACC: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ACCENT_OPTIONAL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NEEDS_ACCENT: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const INDECLFORM: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn AccComposForm(mut gstr: *mut gk_string) -> ::core::ffi::c_int {
    let mut gkform: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut p: *mut ::core::ffi::c_char = &raw mut (*gstr).gs_gkstring
        as *mut ::core::ffi::c_char;
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut word: [::core::ffi::c_char; 60] = [0; 60];
    let mut prefword: [::core::ffi::c_char; 60] = [0; 60];
    let mut saveword: [::core::ffi::c_char; 60] = [0; 60];
    let mut had_stem_acc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut had_suff_acc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    gkform = CreatGkword(1 as ::core::ffi::c_int);
    saveword[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    prefword[0 as ::core::ffi::c_int as usize] = saveword[0 as ::core::ffi::c_int
        as usize];
    word[0 as ::core::ffi::c_int as usize] = prefword[0 as ::core::ffi::c_int as usize];
    if !(has_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, ENCLITIC) != 0)
    {
        s = strchr(p, '!' as i32);
        if !s.is_null() {
            strcpy(&raw mut prefword as *mut ::core::ffi::c_char, p);
            p = s.offset(1 as ::core::ffi::c_int as isize);
            Xstrncpy(
                &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
            add_morphflag(
                &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                NEEDS_ACCENT,
            );
            zap_morphflag(
                &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                ACCENT_OPTIONAL,
            );
            Xstrncpy(
                &raw mut (*gkform).st_workword as *mut ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            s = strchr(&raw mut prefword as *mut ::core::ffi::c_char, '!' as i32);
            *s.offset(1 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        } else {
            strcpy(&raw mut saveword as *mut ::core::ffi::c_char, p);
            Xstrncpy(
                &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
            p = &raw mut saveword as *mut ::core::ffi::c_char;
        }
        (*gkform).gs_endstring = *gstr;
        set_morphflags(
            gkform as *mut gk_string,
            &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        );
        if hasaccent(&raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char) != 0
            || nsylls(&raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char)
                > 1 as ::core::ffi::c_int
        {
            zap_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, STEM_ACC);
        } else {
            if has_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, STEM_ACC)
                != 0
                && has_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    INDECLFORM,
                ) == 0
            {
                had_stem_acc = 1 as ::core::ffi::c_int;
                zap_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    STEM_ACC,
                );
            }
            if has_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, SUFF_ACC)
                != 0
                && has_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    INDECLFORM,
                ) == 0
            {
                had_suff_acc = 1 as ::core::ffi::c_int;
                zap_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    SUFF_ACC,
                );
            }
            if (*gstr).gs_steminfo & 0o4000 as Stemtype != 0
                || (*gstr).gs_steminfo & 0o10000 as Stemtype != 0
                || (*gstr).gs_forminfo.f_mood() as ::core::ffi::c_int == PARTICIPLE
                || (*gstr).gs_steminfo | INDECL as Stemtype != 0
                || has_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    INDECLFORM,
                ) != 0
            {
                FixPersAcc(
                    gstr,
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    gstr,
                    p,
                    &raw mut word as *mut ::core::ffi::c_char,
                    (*gstr).gs_forminfo,
                    if has_morphflag(
                        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                        INDECLFORM,
                    ) != 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    },
                );
                if word[0 as ::core::ffi::c_int as usize] != 0 {
                    if prefword[0 as ::core::ffi::c_int as usize] != 0 {
                        strcat(
                            &raw mut prefword as *mut ::core::ffi::c_char,
                            &raw mut word as *mut ::core::ffi::c_char,
                        );
                        strcpy(
                            &raw mut word as *mut ::core::ffi::c_char,
                            &raw mut prefword as *mut ::core::ffi::c_char,
                        );
                    }
                    strcpy(
                        &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
                        &raw mut word as *mut ::core::ffi::c_char,
                    );
                }
            } else if (*gstr).gs_steminfo & 0o70000000 as ::core::ffi::c_int as Stemtype
                != 0 || had_suff_acc != 0
            {
                FixRecAcc(gkform, &raw mut (*gstr).gs_morphflags as *mut MorphFlags, p);
                if prefword[0 as ::core::ffi::c_int as usize] != 0 {
                    strcat(&raw mut prefword as *mut ::core::ffi::c_char, p);
                    strcpy(p, &raw mut prefword as *mut ::core::ffi::c_char);
                }
            }
        }
    }
    if had_suff_acc != 0 {
        add_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, SUFF_ACC);
    }
    if had_stem_acc != 0 {
        add_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, STEM_ACC);
    }
    zap_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, ACCENT_OPTIONAL);
    zap_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, NEEDS_ACCENT);
    if (hasaccent(&raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char) != 0
        || nsylls(&raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char)
            > 1 as ::core::ffi::c_int)
        && has_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, STEM_ACC)
            != 0
    {
        zap_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, STEM_ACC);
    }
    FreeGkword(gkform);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn skip_to_syll(
    mut s: *mut ::core::ffi::c_char,
    mut nsyll: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if nsylls(s) > 3 as ::core::ffi::c_int || nsylls(s) == 1 as ::core::ffi::c_int {
        return s
    } else if nsylls(s) == 3 as ::core::ffi::c_int {
        return getsyll2(s, PENULT)
    } else if nsylls(s) == 2 as ::core::ffi::c_int {
        return getsyll2(s, ULTIMA)
    } else {
        return s
    };
}
