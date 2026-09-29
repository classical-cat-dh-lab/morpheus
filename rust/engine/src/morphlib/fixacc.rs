use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn getsyll(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn getsyll2(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn addaccent(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn CreatGkword(_: ::core::ffi::c_int) -> *mut gk_word;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn FreeGkword(_: *mut gk_word) -> ::core::ffi::c_int;
    fn Xstrncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn antepen_form(_: *mut gk_string, _: word_form) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn getquantity(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: bool_0,
        _: bool_0,
    ) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn is_diphth(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> bool_0;
    fn is_thirdmono(
        _: *mut gk_string,
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: word_form,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn naccents(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn nsylls(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn penult_form(_: *mut gk_string, _: word_form) -> ::core::ffi::c_int;
    fn quantprim(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: bool_0,
        _: bool_0,
    ) -> ::core::ffi::c_int;
    fn ulttakescirc(_: *mut gk_string, _: word_form) -> ::core::ffi::c_int;
}
pub type int32 = ::core::ffi::c_uint;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
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
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const P_ERR: *mut ::core::ffi::c_char = -(1 as ::core::ffi::c_int)
    as *mut ::core::ffi::c_char;
pub const NULL_P: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
    ::core::ffi::c_char,
>();
pub const GENITIVE: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const DATIVE: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const GRAVE: ::core::ffi::c_int = '\\' as i32;
pub const CIRCUMFLEX: ::core::ffi::c_int = '=' as i32;
pub const ULTIMA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PENULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ANTEPENULT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LONG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SHORT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENCLITIC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SUFF_ACC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const STEM_ACC: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CONTRACTED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const ACCENT_OPTIONAL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PROCLITIC: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn putsimpleacc(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut gkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut mflags: *mut MorphFlags = ::core::ptr::null_mut::<MorphFlags>();
    let mut tmpw: [::core::ffi::c_char; 60] = [0; 60];
    if cur_lang() == LATIN || cur_lang() == ITALIAN {
        return 0;
    }
    gkword = CreatGkword(1 as ::core::ffi::c_int);
    if gkword.is_null() {
        fprintf(
            stderr,
            b"no memory for gstring in putsimpleacc\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0;
    }
    mflags = calloc(1 as size_t, ::core::mem::size_of::<MorphFlags>() as size_t)
        as *mut MorphFlags;
    Xstrncpy(
        &raw mut (*gkword).st_workword as *mut ::core::ffi::c_char,
        s,
        MAXWORDSIZE as size_t,
    );
    FixRecAcc(
        gkword,
        mflags,
        &raw mut (*gkword).st_workword as *mut ::core::ffi::c_char,
    );
    Xstrncpy(
        s,
        &raw mut (*gkword).st_workword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    FreeGkword(gkword);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn FixRecAcc(
    mut gkform: *mut gk_word,
    mut mflags: *mut MorphFlags,
    mut word: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut form_info: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    if cur_lang() == LATIN || cur_lang() == ITALIAN {
        return 0;
    }
    form_info = (*gkform).gs_forminfo;
    p = word;
    while *p != 0 {
        if *p as ::core::ffi::c_int == ACUTE || *p as ::core::ffi::c_int == GRAVE
            || *p as ::core::ffi::c_int == CIRCUMFLEX
        {
            return 0;
        }
        p = p.offset(1);
    }
    if getsyll(word, ULTIMA) == P_ERR {
        return 0;
    }
    if getquantity(word, ULTIMA, NULL_P, YES, NO) == LONG {
        p = getsyll(word, PENULT);
        if p == P_ERR {
            p = getsyll(word, ULTIMA);
            if has_morphflag(mflags, ACCENT_OPTIONAL) != 0 {
                return 0;
            }
            if ulttakescirc(
                &raw mut (*gkform).gs_endstring,
                (*gkform).gs_endstring.gs_forminfo,
            ) != 0
            {
                addaccent(word, CIRCUMFLEX, p);
            } else {
                addaccent(word, ACUTE, p);
            }
        } else {
            addaccent(word, ACUTE, p);
        }
    } else {
        p = getsyll(word, ANTEPENULT);
        if p != P_ERR && penult_form(&raw mut (*gkform).gs_endstring, form_info) == 0 {
            addaccent(word, ACUTE, p);
        } else {
            if has_morphflag(mflags, ACCENT_OPTIONAL) != 0 {
                return 0;
            }
            p = getsyll(word, PENULT);
            if p == P_ERR {
                p = getsyll(word, ULTIMA);
            }
            if getquantity(word, PENULT, NULL_P, YES, NO) == LONG
                && has_morphflag(mflags, ENCLITIC) == 0
            {
                addaccent(word, CIRCUMFLEX, p);
            } else {
                addaccent(word, ACUTE, p);
            }
        }
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn FixPersAcc(
    mut gstring: *mut gk_string,
    mut mflags: *mut MorphFlags,
    mut stemgstr: *mut gk_string,
    mut endstring: *mut ::core::ffi::c_char,
    mut word: *mut ::core::ffi::c_char,
    mut form_info: word_form,
    mut is_ending: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    FixPersAcc2(gstring, mflags, stemgstr, endstring, word, form_info, is_ending);
    if *word.offset(strlen(word) as isize).offset(-(1 as ::core::ffi::c_int as isize))
        as ::core::ffi::c_int == '*' as i32
    {
        *word
            .offset(strlen(word) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) = 0 as ::core::ffi::c_char;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn FixPersAcc2(
    mut gstring: *mut gk_string,
    mut mflags: *mut MorphFlags,
    mut stemgstr: *mut gk_string,
    mut endstring: *mut ::core::ffi::c_char,
    mut word: *mut ::core::ffi::c_char,
    mut form_info: word_form,
    mut is_ending: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    let mut workstem: [::core::ffi::c_char; 60] = [0; 60];
    let mut stem: *mut ::core::ffi::c_char = &raw mut (*stemgstr).gs_gkstring
        as *mut ::core::ffi::c_char;
    let mut is_oblique: bool_0 = 0;
    *word = 0 as ::core::ffi::c_char;
    if cur_lang() == LATIN || cur_lang() == ITALIAN {
        Xstrncpy(word, stem, MAXWORDSIZE as size_t);
        Xstrncat(word, endstring, MAXWORDSIZE as size_t);
        return 0;
    }
    if nsylls(stem) == 0 && nsylls(endstring) == 0 {
        Xstrncpy(word, stem, MAXWORDSIZE as size_t);
        Xstrncat(word, endstring, MAXWORDSIZE as size_t);
        return 0;
    }
    if has_morphflag(mflags, PROCLITIC) != 0 {
        Xstrncpy(word, stem, MAXWORDSIZE as size_t);
        return 0;
    }
    if form_info.f_case() as ::core::ffi::c_int == GENITIVE
        || form_info.f_case() as ::core::ffi::c_int == DATIVE
    {
        is_oblique = YES as bool_0;
    } else {
        is_oblique = NO as bool_0;
    }
    Xstrncpy(&raw mut workstem as *mut ::core::ffi::c_char, stem, MAXWORDSIZE as size_t);
    p = endstring;
    while *p != 0 {
        if *p as ::core::ffi::c_int == ACUTE || *p as ::core::ffi::c_int == GRAVE
            || *p as ::core::ffi::c_int == CIRCUMFLEX
        {
            Xstrncpy(word, stem, MAXWORDSIZE as size_t);
            Xstrncat(word, endstring, MAXWORDSIZE as size_t);
            return 0;
        }
        p = p.offset(1);
    }
    p = stem;
    while *p != 0 {
        if *p as ::core::ffi::c_int == ACUTE || *p as ::core::ffi::c_int == GRAVE
            || *p as ::core::ffi::c_int == CIRCUMFLEX
        {
            Xstrncpy(word, stem, MAXWORDSIZE as size_t);
            Xstrncat(word, endstring, MAXWORDSIZE as size_t);
            return 0;
        }
        p = p.offset(1);
    }
    if has_morphflag(mflags, ACCENT_OPTIONAL) != 0
        && has_morphflag(mflags, ENCLITIC) != 0
    {
        Xstrncpy(word, stem, MAXWORDSIZE as size_t);
        Xstrncat(word, endstring, MAXWORDSIZE as size_t);
        return 0;
    }
    if is_thirdmono(stemgstr, gstring, stem, endstring, form_info, is_ending) != 0
        && nsylls(endstring) == 2 as ::core::ffi::c_int
    {
        let mut ep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if has_morphflag(mflags, ACCENT_OPTIONAL) != 0 {
            Xstrncpy(word, stem, MAXWORDSIZE as size_t);
            Xstrncat(word, endstring, MAXWORDSIZE as size_t);
            return 0;
        }
        ep = getsyll(endstring, ULTIMA);
        if is_diphth(ep, endstring) != 0 {
            ep = ep.offset(-1);
        }
        fixnacc2(ep, gstring, form_info, is_ending, is_oblique);
        Xstrncpy(
            &raw mut tmp as *mut ::core::ffi::c_char,
            &raw mut workstem as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        Xstrncat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            endstring,
            MAXWORDSIZE as size_t,
        );
        Xstrncpy(word, &raw mut tmp as *mut ::core::ffi::c_char, MAXWORDSIZE as size_t);
        return 0;
    } else if has_morphflag(
        &raw mut (*stemgstr).gs_morphflags as *mut MorphFlags,
        SUFF_ACC,
    ) != 0 || is_thirdmono(stemgstr, gstring, stem, endstring, form_info, is_ending) != 0
        || nsylls(stem) == 0 as ::core::ffi::c_int
    {
        if nsylls(endstring) >= 1 as ::core::ffi::c_int {
            if naccents(endstring) == 0 {
                fixnacc2(endstring, gstring, form_info, is_ending, is_oblique);
            }
            Xstrncpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut workstem as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                endstring,
                MAXWORDSIZE as size_t,
            );
            Xstrncpy(
                word,
                &raw mut tmp as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            return 0;
        } else if nsylls(endstring) == 0 as ::core::ffi::c_int {
            let mut p_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            Xstrncpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                stem,
                MAXWORDSIZE as size_t,
            );
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                endstring,
                MAXWORDSIZE as size_t,
            );
            p_0 = getsyll2(&raw mut tmp as *mut ::core::ffi::c_char, ULTIMA);
            if p_0 != P_ERR {
                fixnacc2(p_0, gstring, form_info, is_ending, is_oblique);
                Xstrncpy(
                    word,
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
            }
            return 0;
        }
    } else if penult_form(stemgstr, form_info) != 0
        || has_morphflag(&raw mut (*gstring).gs_morphflags as *mut MorphFlags, STEM_ACC)
            != 0 || nsylls(stem) == 1 as ::core::ffi::c_int
    {
        p = getsyll(&raw mut workstem as *mut ::core::ffi::c_char, ULTIMA);
        if nsylls(endstring) == 1 as ::core::ffi::c_int
            && quantprim(
                &raw mut workstem as *mut ::core::ffi::c_char,
                ULTIMA,
                NO,
                is_oblique,
            ) == LONG && quantprim(endstring, ULTIMA, YES, is_oblique) == SHORT
            && has_morphflag(mflags, ENCLITIC) == 0
        {
            if has_morphflag(mflags, ACCENT_OPTIONAL) != 0 {
                Xstrncpy(word, stem, MAXWORDSIZE as size_t);
                Xstrncat(word, endstring, MAXWORDSIZE as size_t);
                return 0;
            }
            addaccent(&raw mut workstem as *mut ::core::ffi::c_char, CIRCUMFLEX, p);
        } else if quantprim(endstring, ULTIMA, YES, is_oblique) == LONG
            || nsylls(endstring) == 2 as ::core::ffi::c_int
        {
            addaccent(&raw mut workstem as *mut ::core::ffi::c_char, ACUTE, p);
        } else if quantprim(
            &raw mut workstem as *mut ::core::ffi::c_char,
            ULTIMA,
            NO,
            is_oblique,
        ) == LONG
            && has_morphflag(
                &raw mut (*stemgstr).gs_morphflags as *mut MorphFlags,
                CONTRACTED,
            ) != 0
        {
            addaccent(&raw mut workstem as *mut ::core::ffi::c_char, CIRCUMFLEX, p);
        } else {
            addaccent(&raw mut workstem as *mut ::core::ffi::c_char, ACUTE, p);
        }
        Xstrncpy(
            &raw mut tmp as *mut ::core::ffi::c_char,
            &raw mut workstem as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        Xstrncat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            endstring,
            MAXWORDSIZE as size_t,
        );
        Xstrncpy(word, &raw mut tmp as *mut ::core::ffi::c_char, MAXWORDSIZE as size_t);
        return 0;
    } else if antepen_form(stemgstr, form_info) != 0 {
        p = getsyll(&raw mut workstem as *mut ::core::ffi::c_char, PENULT);
        if nsylls(endstring) == 0 as ::core::ffi::c_int
            && getquantity(
                &raw mut workstem as *mut ::core::ffi::c_char,
                PENULT,
                NULL_P,
                NO,
                is_oblique,
            ) == LONG
            && getquantity(
                &raw mut workstem as *mut ::core::ffi::c_char,
                ULTIMA,
                NULL_P,
                NO,
                is_oblique,
            ) == SHORT && has_morphflag(mflags, ENCLITIC) == 0
        {
            addaccent(&raw mut workstem as *mut ::core::ffi::c_char, CIRCUMFLEX, p);
        } else if nsylls(endstring) > 1 as ::core::ffi::c_int
            || getquantity(endstring, ULTIMA, NULL_P, YES, is_oblique) == LONG
        {
            p = getsyll(&raw mut workstem as *mut ::core::ffi::c_char, ULTIMA);
            addaccent(&raw mut workstem as *mut ::core::ffi::c_char, ACUTE, p);
        } else {
            addaccent(&raw mut workstem as *mut ::core::ffi::c_char, ACUTE, p);
        }
        Xstrncpy(
            &raw mut tmp as *mut ::core::ffi::c_char,
            &raw mut workstem as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        Xstrncat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            endstring,
            MAXWORDSIZE as size_t,
        );
        Xstrncpy(word, &raw mut tmp as *mut ::core::ffi::c_char, MAXWORDSIZE as size_t);
        return 0;
    }
    Xstrncpy(
        &raw mut tmp as *mut ::core::ffi::c_char,
        &raw mut workstem as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    Xstrncat(&raw mut tmp as *mut ::core::ffi::c_char, endstring, MAXWORDSIZE as size_t);
    fixnacc2(
        &raw mut tmp as *mut ::core::ffi::c_char,
        gstring,
        form_info,
        is_ending,
        is_oblique,
    );
    Xstrncpy(word, &raw mut tmp as *mut ::core::ffi::c_char, MAXWORDSIZE as size_t);
    return 0;
}
unsafe extern "C" fn fixnacc2(
    mut targstring: *mut ::core::ffi::c_char,
    mut gstring: *mut gk_string,
    mut form_info: word_form,
    mut is_ending: ::core::ffi::c_int,
    mut is_oblique: bool_0,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mflags: *mut MorphFlags = &raw mut (*gstring).gs_morphflags
        as *mut MorphFlags;
    let mut is_contr: bool_0 = 0;
    is_contr = has_morphflag(mflags, CONTRACTED) as bool_0;
    if cur_lang() == LATIN || cur_lang() == ITALIAN {
        return 0;
    }
    p = targstring;
    while *p != 0 {
        if *p as ::core::ffi::c_int == ACUTE || *p as ::core::ffi::c_int == GRAVE
            || *p as ::core::ffi::c_int == CIRCUMFLEX
        {
            return 0;
        }
        p = p.offset(1);
    }
    is_ending = YES;
    if getquantity(targstring, ULTIMA, NULL_P, is_ending as bool_0, is_oblique) == LONG
        || nsylls(targstring) == 1 as ::core::ffi::c_int
            && (is_ending != 0 && is_contr != 0)
    {
        p = getsyll(targstring, PENULT);
        if p == P_ERR {
            if has_morphflag(mflags, ACCENT_OPTIONAL) != 0 {
                return 0;
            }
            p = getsyll(targstring, ULTIMA);
            if ulttakescirc(gstring, form_info) != 0 {
                addaccent(targstring, CIRCUMFLEX, p);
            } else {
                addaccent(targstring, ACUTE, p);
            }
        } else {
            addaccent(targstring, ACUTE, p);
        }
    } else {
        p = getsyll(targstring, ANTEPENULT);
        if p != P_ERR {
            addaccent(targstring, ACUTE, p);
        } else {
            if has_morphflag(mflags, ACCENT_OPTIONAL) != 0 {
                return 0;
            }
            p = getsyll(targstring, PENULT);
            if p == P_ERR {
                p = getsyll(targstring, ULTIMA);
            }
            if getquantity(
                targstring,
                PENULT,
                NULL_P,
                is_ending as bool_0,
                (is_oblique != 0 && is_contr == 0) as ::core::ffi::c_int,
            ) == LONG && has_morphflag(mflags, ENCLITIC) == 0
            {
                addaccent(targstring, CIRCUMFLEX, p);
            } else {
                addaccent(targstring, ACUTE, p);
            }
        }
    }
    return 0;
}
