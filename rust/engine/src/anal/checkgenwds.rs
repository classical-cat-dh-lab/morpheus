use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn CreatGkAnal(_: ::core::ffi::c_int) -> *mut gk_analysis;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn add_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn chckcmpvb(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn do_crasis(_: *mut gk_string, _: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn eq_forminfo(_: word_form, _: word_form) -> ::core::ffi::c_int;
    fn getbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn morphstrcmp(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn near_miss(
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn set_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripacute(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripdiaer(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripmetachars(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripquant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn zap2acc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn zap_extra_breath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type int32 = ::core::ffi::c_uint;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
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
pub const IONIC: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const HOMERIC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const INDICATIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const HARDSHORT: ::core::ffi::c_int = '^' as i32;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const COMP_ONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENCLITIC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const R_E_I_ALPHA: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const NOT_IN_COMPOSITION: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const UNAUGMENTED: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const APOCOPE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const POETIC: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const UNASP_PREVERB: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const IGNORE_ACCENTS: ::core::ffi::c_int = 0o20000 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const MAXANALYSES: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
static mut anals_seen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut lems_seen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CheckGenWords(
    mut Gkword: *mut gk_word,
    mut gkforms: *mut gk_word,
) -> ::core::ffi::c_int {
    #[cfg(feature = "trace")] crate::trace::word("CheckGenWords:Gkword", Gkword as *const _);

    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut hits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut accword: [::core::ffi::c_char; 61] = [0; 61];
    let mut wordnoacute: [::core::ffi::c_char; 61] = [0; 61];
    let mut curform: [::core::ffi::c_char; 61] = [0; 61];
    let mut checks: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut preverb: *mut ::core::ffi::c_char = &raw mut (*gkforms)
        .gs_preverb
        .gs_gkstring as *mut ::core::ffi::c_char;
    let mut lemma: *mut ::core::ffi::c_char = &raw mut (*gkforms).st_lemma
        as *mut ::core::ffi::c_char;
    let mut origword: *mut ::core::ffi::c_char = &raw mut (*Gkword).st_workword
        as *mut ::core::ffi::c_char;
    Xstrncpy(
        &raw mut accword as *mut ::core::ffi::c_char,
        origword,
        ::core::mem::size_of::<[::core::ffi::c_char; 61]>() as ::core::ffi::c_int
            as size_t,
    );
    zap_extra_breath(&raw mut accword as *mut ::core::ffi::c_char);
    stripdiaer(&raw mut accword as *mut ::core::ffi::c_char);
    stripquant(&raw mut accword as *mut ::core::ffi::c_char);
    Xstrncpy(
        &raw mut wordnoacute as *mut ::core::ffi::c_char,
        &raw mut accword as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 61]>() as ::core::ffi::c_int
            as size_t,
    );
    stripacute(&raw mut wordnoacute as *mut ::core::ffi::c_char);
    hits = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while (*gkforms.offset(i as isize)).st_workword[0 as ::core::ffi::c_int as usize]
        != 0
    {
        Xstrncpy(
            &raw mut curform as *mut ::core::ffi::c_char,
            &raw mut (*gkforms.offset(i as isize)).st_workword
                as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 61]>() as ::core::ffi::c_int
                as size_t,
        );
        if (*Gkword).gs_prntflags as ::core::ffi::c_int & IGNORE_ACCENTS != 0 {
            let mut wordnoacc: [::core::ffi::c_char; 60] = [0; 60];
            strcpy(
                &raw mut wordnoacc as *mut ::core::ffi::c_char,
                &raw mut wordnoacute as *mut ::core::ffi::c_char,
            );
            stripacc(&raw mut wordnoacc as *mut ::core::ffi::c_char);
            checks = &raw mut wordnoacc as *mut ::core::ffi::c_char;
            stripacc(&raw mut curform as *mut ::core::ffi::c_char);
        } else if has_morphflag(
            &raw mut (*gkforms.offset(i as isize)).gs_stem.gs_morphflags
                as *mut MorphFlags,
            ENCLITIC,
        ) != 0 && *preverb == 0
        {
            checks = &raw mut wordnoacute as *mut ::core::ffi::c_char;
        } else {
            checks = &raw mut accword as *mut ::core::ffi::c_char;
        }
        zap_extra_breath(&raw mut curform as *mut ::core::ffi::c_char);
        zap2acc(&raw mut curform as *mut ::core::ffi::c_char);
        if !(*(&raw mut curform as *mut ::core::ffi::c_char)
            .offset(Xstrlen(&raw mut curform as *mut ::core::ffi::c_char) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == HARDLONG
            && *origword
                .offset(Xstrlen(origword) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == HARDSHORT)
        {
            stripdiaer(&raw mut curform as *mut ::core::ffi::c_char);
            stripmetachars(&raw mut curform as *mut ::core::ffi::c_char);
            if morphstrcmp(&raw mut curform as *mut ::core::ffi::c_char, checks) == 0 {
                if has_morphflag(
                    &raw mut (*gkforms.offset(i as isize)).gs_stem.gs_morphflags
                        as *mut MorphFlags,
                    COMP_ONLY,
                ) != 0 && *preverb == 0
                {
                    near_miss(
                        gkforms.offset(i as isize) as *mut gk_string,
                        checks,
                        COMP_ONLY,
                    );
                } else if has_morphflag(
                    &raw mut (*gkforms.offset(i as isize)).gs_stem.gs_morphflags
                        as *mut MorphFlags,
                    NOT_IN_COMPOSITION,
                ) != 0 && *preverb as ::core::ffi::c_int != 0
                {
                    near_miss(
                        gkforms.offset(i as isize) as *mut gk_string,
                        checks,
                        NOT_IN_COMPOSITION,
                    );
                } else if AddAnalysis(Gkword, gkforms.offset(i as isize)) != 0 {
                    hits += 1;
                }
            } else {
                near_miss(
                    gkforms.offset(i as isize) as *mut gk_string,
                    checks,
                    0 as ::core::ffi::c_int,
                );
            }
        }
        i += 1;
    }
    return hits;
}
static mut analerror: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn AddAnalysis(
    mut Gkword: *mut gk_word,
    mut gkform: *mut gk_word,
) -> ::core::ffi::c_int {
    #[cfg(feature = "trace")] crate::trace::word("AddAnalysis:Gkword", Gkword as *const _);
    #[cfg(feature = "trace")] crate::trace::word("AddAnalysis:gkform", gkform as *const _);

    let mut curanal: *mut gk_analysis = ::core::ptr::null_mut::<gk_analysis>();
    let mut i: ::core::ffi::c_int = 0;
    let mut newlem: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut tmplem: [::core::ffi::c_char; 60] = [0; 60];
    let mut cmplem: [::core::ffi::c_char; 60] = [0; 60];
    if analerror != 0 {
        fprintf(
            stderr,
            b"something wrong with the analysis storage!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (*Gkword).gw_analysis.is_null() {
        (*Gkword).gw_analysis = CreatGkAnal(MAXANALYSES + 1 as ::core::ffi::c_int);
        if (*Gkword).gw_analysis.is_null() {
            fprintf(
                stderr,
                b"not enough memory for greek analysis\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            analerror += 1;
            return 0 as ::core::ffi::c_int;
        }
        if (*Gkword).gw_totanal != 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"hey! anal pointer NULL but totanal is %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*Gkword).gw_totanal,
            );
            analerror += 1;
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*Gkword).gw_totanal >= MAXANALYSES {
        fprintf(
            stderr,
            b"%s:  ran out of space with %d analyses!\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut (*Gkword).st_rawword as *mut ::core::ffi::c_char,
            (*Gkword).gw_totanal,
        );
        return 0 as ::core::ffi::c_int;
    }
    curanal = (*Gkword).gw_analysis.offset((*Gkword).gw_totanal as isize);
    if (*gkform).st_crasis[0 as ::core::ffi::c_int as usize] != 0 {
        Xstrncpy(
            &raw mut (*curanal).st_crasis as *mut ::core::ffi::c_char,
            &raw mut (*gkform).st_crasis as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        if do_crasis(
            gkform as *mut gk_string,
            &raw mut (*curanal).st_crasis as *mut ::core::ffi::c_char,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    cmplem[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    if (*gkform).gs_preverb.gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
        let mut s: *mut ::core::ffi::c_char = &raw mut tmplem
            as *mut ::core::ffi::c_char;
        let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut tmphalf1: [::core::ffi::c_char; 60] = [0; 60];
        cmplem[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        tmplem[0 as ::core::ffi::c_int as usize] = cmplem[0 as ::core::ffi::c_int
            as usize];
        tmphalf1[0 as ::core::ffi::c_int as usize] = tmplem[0 as ::core::ffi::c_int
            as usize];
        sprintf(
            &raw mut tmplem as *mut ::core::ffi::c_char,
            b"%s-%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*gkform).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut (*gkform).st_lemma as *mut ::core::ffi::c_char,
        );
        loop {
            chckcmpvb(s, &raw mut cmplem as *mut ::core::ffi::c_char);
            if cmplem[0 as ::core::ffi::c_int as usize] != 0 {
                if tmphalf1[0 as ::core::ffi::c_int as usize] != 0 {
                    strcat(
                        &raw mut tmphalf1 as *mut ::core::ffi::c_char,
                        b"-\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    strcat(
                        &raw mut tmphalf1 as *mut ::core::ffi::c_char,
                        &raw mut cmplem as *mut ::core::ffi::c_char,
                    );
                    strcpy(
                        &raw mut cmplem as *mut ::core::ffi::c_char,
                        &raw mut tmphalf1 as *mut ::core::ffi::c_char,
                    );
                }
                break;
            } else {
                t = s;
                s = strchr(s, ',' as i32);
                if s.is_null() {
                    if tmphalf1[0 as ::core::ffi::c_int as usize] != 0 {
                        strcat(
                            &raw mut tmphalf1 as *mut ::core::ffi::c_char,
                            b",\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    strcat(&raw mut tmphalf1 as *mut ::core::ffi::c_char, t);
                    strcpy(
                        &raw mut tmplem as *mut ::core::ffi::c_char,
                        &raw mut tmphalf1 as *mut ::core::ffi::c_char,
                    );
                    break;
                } else {
                    let fresh0 = s;
                    s = s.offset(1);
                    *fresh0 = 0 as ::core::ffi::c_char;
                    if tmphalf1[0 as ::core::ffi::c_int as usize] != 0 {
                        strcat(
                            &raw mut tmphalf1 as *mut ::core::ffi::c_char,
                            b",\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    strcat(&raw mut tmphalf1 as *mut ::core::ffi::c_char, t);
                }
            }
        }
        if cmplem[0 as ::core::ffi::c_int as usize] != 0 {
            Xstrncpy(
                &raw mut (*curanal).st_lemma as *mut ::core::ffi::c_char,
                &raw mut cmplem as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
        } else {
            Xstrncpy(
                &raw mut (*curanal).st_lemma as *mut ::core::ffi::c_char,
                &raw mut tmplem as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
        }
    } else {
        Xstrncpy(
            &raw mut (*curanal).st_lemma as *mut ::core::ffi::c_char,
            &raw mut (*gkform).st_lemma as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
    }
    Xstrncpy(
        &raw mut (*curanal).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut (*gkform).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    (*curanal).gs_preverb = (*gkform).gs_preverb;
    (*curanal).gs_aug1 = (*gkform).gs_aug1;
    (*curanal).gs_stem = (*gkform).gs_stem;
    (*curanal).gs_endstring = (*gkform).gs_endstring;
    Xstrncpy(
        &raw mut (*curanal).st_rawword as *mut ::core::ffi::c_char,
        &raw mut (*gkform).st_rawword as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    Xstrncpy(
        &raw mut (*curanal).st_workword as *mut ::core::ffi::c_char,
        &raw mut (*gkform).st_workword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    (*curanal).gs_forminfo = (*gkform).gs_forminfo;
    (*curanal).gs_geogregion = (*gkform).gs_geogregion;
    (*curanal).gs_dialect = ((*curanal).gs_preverb.gs_dialect as ::core::ffi::c_int
        | (*curanal).gs_aug1.gs_dialect as ::core::ffi::c_int
        | (*curanal).gs_stem.gs_dialect as ::core::ffi::c_int
        | (*curanal).gs_endstring.gs_dialect as ::core::ffi::c_int) as Dialect;
    if has_morphflag(
        &raw mut (*gkform).gs_preverb.gs_morphflags as *mut MorphFlags,
        APOCOPE,
    ) != 0
    {
        if (*Gkword).gs_dialect as ::core::ffi::c_int & HOMERIC != 0 {
            (*curanal).gs_dialect = ((*curanal).gs_dialect as ::core::ffi::c_int
                | 0o100 as ::core::ffi::c_int as Dialect as ::core::ffi::c_int)
                as Dialect;
        }
        if (*Gkword).gs_dialect as ::core::ffi::c_int != HOMERIC {
            add_morphflag(&raw mut (*curanal).gs_morphflags as *mut MorphFlags, POETIC);
        }
    }
    if has_morphflag(
        &raw mut (*gkform).gs_preverb.gs_morphflags as *mut MorphFlags,
        UNASP_PREVERB,
    ) != 0
        && getbreath(&raw mut (*gkform).gs_stem.gs_gkstring as *mut ::core::ffi::c_char)
            == ROUGHBR
    {
        let mut d: Dialect = 0;
        d = AndDialect((*gkform).gs_preverb.gs_dialect, IONIC as Dialect);
        if (d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        d = AndDialect((*curanal).gs_dialect, IONIC as Dialect);
        (*curanal).gs_dialect = d;
    }
    if cur_lang() != LATIN
        && has_morphflag(
            &raw mut (*gkform).gs_stem.gs_morphflags as *mut MorphFlags,
            UNAUGMENTED,
        ) != 0
    {
        if (*curanal).gs_dialect != 0 {
            (*curanal).gs_dialect = ((*curanal).gs_dialect as ::core::ffi::c_int
                | (*curanal).gs_dialect as ::core::ffi::c_int
                    & (0o100 as ::core::ffi::c_int | 0o10 as ::core::ffi::c_int))
                as Dialect;
        } else {
            (*curanal).gs_dialect = ((*curanal).gs_dialect as ::core::ffi::c_int
                | (0o100 as ::core::ffi::c_int | 0o10 as ::core::ffi::c_int)) as Dialect;
        }
        if (*curanal).gs_forminfo.f_mood() as ::core::ffi::c_int != INDICATIVE {
            return 0 as ::core::ffi::c_int;
        }
    }
    (*curanal).gs_steminfo = (*gkform).gs_steminfo;
    (*curanal).gs_derivtype = (*gkform).gs_derivtype;
    set_morphflags(
        curanal as *mut gk_string,
        &raw mut (*gkform).gs_morphflags as *mut MorphFlags,
    );
    add_morphflags(
        curanal as *mut gk_string,
        &raw mut (*gkform).gs_preverb.gs_morphflags as *mut MorphFlags,
    );
    add_morphflags(
        curanal as *mut gk_string,
        &raw mut (*gkform).gs_stem.gs_morphflags as *mut MorphFlags,
    );
    add_morphflags(
        curanal as *mut gk_string,
        &raw mut (*gkform).gs_endstring.gs_morphflags as *mut MorphFlags,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*Gkword).gw_totanal {
        if equiv_anal(curanal, (*Gkword).gw_analysis.offset(i as isize)) != 0 {
            merge_anal_dialects((*Gkword).gw_analysis.offset(i as isize), curanal);
            return 0 as ::core::ffi::c_int;
        }
        if strcmp(
            &raw mut (*curanal).st_lemma as *mut ::core::ffi::c_char,
            &raw mut (*(*Gkword).gw_analysis.offset(i as isize)).st_lemma
                as *mut ::core::ffi::c_char,
        ) == 0
        {
            newlem = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    (*Gkword).gw_totanal += 1;
    anals_seen += 1;
    lems_seen += newlem;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn show_totanals() -> ::core::ffi::c_int {
    return anals_seen;
}
#[no_mangle]
pub unsafe extern "C" fn show_totlems() -> ::core::ffi::c_int {
    return lems_seen;
}
#[no_mangle]
pub unsafe extern "C" fn merge_anal_dialects(
    mut anal1: *mut gk_analysis,
    mut anal2: *mut gk_analysis,
) -> ::core::ffi::c_int {
    if (*anal1).gs_dialect != 0 {
        (*anal1).gs_dialect = ((*anal1).gs_dialect as ::core::ffi::c_int
            | (*anal2).gs_dialect as ::core::ffi::c_int) as Dialect;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn equiv_anal(
    mut anal1: *mut gk_analysis,
    mut anal2: *mut gk_analysis,
) -> ::core::ffi::c_int {
    if strcmp(
        &raw mut (*anal1).st_lemma as *mut ::core::ffi::c_char,
        &raw mut (*anal2).st_lemma as *mut ::core::ffi::c_char,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw mut (*anal1).gs_aug1.gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut (*anal2).gs_aug1.gs_gkstring as *mut ::core::ffi::c_char,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw mut (*anal1).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut (*anal2).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw mut (*anal1).st_workword as *mut ::core::ffi::c_char,
        &raw mut (*anal2).st_workword as *mut ::core::ffi::c_char,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw mut (*anal1).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut (*anal2).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if eq_forminfo((*anal1).gs_forminfo, (*anal2).gs_forminfo) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if has_morphflag(
        &raw mut (*anal1).gs_stem.gs_morphflags as *mut MorphFlags,
        R_E_I_ALPHA,
    )
        != has_morphflag(
            &raw mut (*anal2).gs_stem.gs_morphflags as *mut MorphFlags,
            R_E_I_ALPHA,
        )
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*anal1).gs_steminfo != (*anal2).gs_steminfo {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
