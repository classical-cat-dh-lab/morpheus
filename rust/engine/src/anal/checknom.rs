use ::c2rust_bitfields;
extern "C" {
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn StemWorks(
        _: *mut gk_word,
        _: *mut ::core::ffi::c_char,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn chcknend(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn checkforcompnoun(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn stemexists(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn xFree(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const VERBS_ONLY: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const ZEROEND: ::core::ffi::c_int = '*' as i32;
#[no_mangle]
pub unsafe extern "C" fn checknom(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    #[cfg(feature = "trace")] crate::trace::word("checknom:Gkword", Gkword as *const _);

    let mut rval: ::core::ffi::c_int = 0;
    if (*Gkword).gs_prntflags as ::core::ffi::c_int & VERBS_ONLY != 0 {
        return 0 as ::core::ffi::c_int;
    }
    rval = checkregnom(Gkword);
    if rval != 0 {
        return rval;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkregnom(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut wp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut workword: [::core::ffi::c_char; 60] = [0; 60];
    let mut half1: [::core::ffi::c_char; 60] = [0; 60];
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    Xstrncpy(
        &raw mut workword as *mut ::core::ffi::c_char,
        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
            as size_t,
    );
    Xstrncpy(
        &raw mut half1 as *mut ::core::ffi::c_char,
        &raw mut workword as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
            as size_t,
    );
    sprintf(
        &raw mut (*Gkword).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
        b"%c\0" as *const u8 as *const ::core::ffi::c_char,
        ZEROEND,
    );
    Xstrncpy(
        &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut workword as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    rval += gotnom(Gkword);
    wp = &raw mut workword as *mut ::core::ffi::c_char;
    while *wp != 0 {
        Xstrncpy(
            &raw mut (*Gkword).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
            wp,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        Xstrncpy(
            &raw mut half1 as *mut ::core::ffi::c_char,
            &raw mut workword as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
                as size_t,
        );
        half1[wp.offset_from(&raw mut workword as *mut ::core::ffi::c_char)
            as ::core::ffi::c_long as usize] = 0 as ::core::ffi::c_char;
        wp = wp.offset(1);
        Xstrncpy(
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut half1 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        rval += gotnom(Gkword);
    }
    return rval;
}
unsafe extern "C" fn gotnom(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut is_ending: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut endkeys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut curstem: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut curend: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut stemkeys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    curend = ::core::ptr::null_mut::<::core::ffi::c_char>();
    stemkeys = curend;
    endkeys = stemkeys;
    curstem = ::core::ptr::null_mut::<gk_string>();
    endkeys = malloc(LONGSTRING as size_t) as *mut ::core::ffi::c_char;
    stemkeys = malloc(LONGSTRING as size_t) as *mut ::core::ffi::c_char;
    curstem = CreatGkString(1 as ::core::ffi::c_int);
    curend = malloc(MAXWORDSIZE as size_t) as *mut ::core::ffi::c_char;
    let ref mut fresh0 = *stemkeys.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = 0 as ::core::ffi::c_char;
    *endkeys.offset(0 as ::core::ffi::c_int as isize) = *fresh0;
    stripacc(&raw mut (*Gkword).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char);
    Xstrncpy(
        curend,
        &raw mut (*Gkword).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    is_ending = chcknend(curend, endkeys);
    if is_ending != 0 {
        Xstrncpy(
            &raw mut (*curstem).gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
                as size_t,
        );
        stripacc(&raw mut (*curstem).gs_gkstring as *mut ::core::ffi::c_char);
        *stemkeys = 0 as ::core::ffi::c_char;
        if stemexists(
            &raw mut (*curstem).gs_gkstring as *mut ::core::ffi::c_char,
            endkeys,
            stemkeys,
            1 as ::core::ffi::c_int,
        ) != 0
        {
            rval += StemWorks(Gkword, stemkeys, curstem);
        }
    }
    if rval == 0 && 0 as ::core::ffi::c_int != 0 {
        if rval == 0 {
            checkforcompnoun(
                &raw mut (*curstem).gs_gkstring as *mut ::core::ffi::c_char,
                endkeys,
                stemkeys,
            );
        }
    }
    xFree(
        endkeys,
        b"endkeys\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    xFree(
        stemkeys,
        b"stemkeys\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    FreeGkString(curstem);
    xFree(
        curend,
        b"curend\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    curend = ::core::ptr::null_mut::<::core::ffi::c_char>();
    stemkeys = curend;
    endkeys = stemkeys;
    curstem = ::core::ptr::null_mut::<gk_string>();
    return rval;
}
