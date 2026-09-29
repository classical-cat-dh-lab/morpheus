use ::c2rust_bitfields;
extern "C" {
    fn isupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn qsort(
        _: *mut ::core::ffi::c_void,
        _: size_t,
        _: size_t,
        _: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn AddAdjInfo(
        _: *mut ::core::ffi::c_char,
        _: word_form,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn AddParadigmInfo(
        _: *mut ::core::ffi::c_char,
        _: word_form,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn AddPersNumInfo(
        _: *mut ::core::ffi::c_char,
        _: word_form,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn DialectNames(
        _: Dialect,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn GregSprintGkFlags(
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn JakeSprintGkFlags(
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn MorphNames(
        _: *mut MorphFlags,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn PrntStemtype(_: Stemtype, _: *mut FILE) -> ::core::ffi::c_int;
    fn SprintGkFlags(
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Xstrncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn beta2smarta(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn set_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const ADJSTEM: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const NOUNSTEM: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const PPARTMASK: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const PARTICIPLE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SHOW_LEMMA: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const KEEP_BETA: ::core::ffi::c_int = 0o40 as ::core::ffi::c_int;
pub const SHOW_FULL_INFO: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const DBASEFORMAT: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const DBASESHORT: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int | DBASEFORMAT;
pub const PARSE_FORMAT: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const PERSEUS_FORMAT: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const ENDING_INDEX: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const LEXICON_OUTPUT: ::core::ffi::c_int = 0o40000 as ::core::ffi::c_int;
pub const GREEK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXANALYSES: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
static mut pbuf: *mut ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>()
    as *mut ::core::ffi::c_char;
pub const NEWLINE: [::core::ffi::c_char; 2] = unsafe {
    ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"\r\0")
};
static mut prevlemma: [::core::ffi::c_char; 60] = [0; 60];
static mut prevword: [::core::ffi::c_char; 60] = [0; 60];
static mut prevstem: [::core::ffi::c_char; 60] = [0; 60];
static mut curan: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn PrntAnalyses(
    mut Gkword: *mut gk_word,
    mut prntflags: PrntFlags,
    mut fout: *mut FILE,
) -> ::core::ffi::c_int {
    #[cfg(feature = "trace")] crate::trace::word("PrntAnalyses:Gkword", Gkword as *const _);

    let mut i: ::core::ffi::c_int = 0;
    let mut nanals: ::core::ffi::c_int = 0;
    let mut Anal: *mut gk_analysis = ::core::ptr::null_mut::<gk_analysis>();
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    if pbuf.is_null() {
        pbuf = calloc(
            (MAXANALYSES as size_t).wrapping_mul(128 as size_t),
            ::core::mem::size_of::<::core::ffi::c_char>(),
        ) as *mut ::core::ffi::c_char;
    }
    *pbuf = 0 as ::core::ffi::c_char;
    nanals = (*Gkword).gw_totanal;
    SortAnals((*Gkword).gw_analysis, nanals);
    #[cfg(feature = "trace")] crate::trace::word("PrntAnalyses:sorted", Gkword as *const _);
    if prntflags as ::core::ffi::c_int & SHOW_LEMMA != 0 {
        DumpLemmaInfo(Gkword, prntflags, fout);
        return nanals;
    }
    if prntflags as ::core::ffi::c_int
        & (DBASEFORMAT | SHOW_FULL_INFO | LEXICON_OUTPUT | PARSE_FORMAT | PERSEUS_FORMAT
            | ENDING_INDEX) != 0
    {
        dump_all_anals(Gkword, prntflags, fout);
        return nanals;
    }
    if prntflags as ::core::ffi::c_int & SHOW_LEMMA == 0 {
        sprintf(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"$%s&  %s%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*Gkword).st_rawword as *mut ::core::ffi::c_char,
            if nanals == 1 as ::core::ffi::c_int {
                b"is\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"could be\0" as *const u8 as *const ::core::ffi::c_char
            },
            NEWLINE.as_ptr(),
        );
        if prntflags as ::core::ffi::c_int & KEEP_BETA != 0 {
            strcat(pbuf, &raw mut tmp as *mut ::core::ffi::c_char);
        } else {
            beta2smarta(&raw mut tmp as *mut ::core::ffi::c_char, pbuf);
        }
    }
    prevlemma[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    Xstrncpy(
        &raw mut prevword as *mut ::core::ffi::c_char,
        &raw mut (*Gkword).st_rawword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    Xstrncpy(
        &raw mut prevstem as *mut ::core::ffi::c_char,
        &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    i = 0 as ::core::ffi::c_int;
    curan = 0 as ::core::ffi::c_int;
    while i < nanals {
        Anal = (*Gkword).gw_analysis.offset(i as isize);
        PrntOneAnalysis(Anal, prntflags, fout);
        Xstrncpy(
            &raw mut prevlemma as *mut ::core::ffi::c_char,
            &raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        i += 1;
    }
    if i != 0 {
        if prntflags as ::core::ffi::c_int & SHOW_LEMMA != 0 {
            Xstrncat(
                pbuf,
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                (MAXANALYSES * 128 as ::core::ffi::c_int) as size_t,
            );
        } else {
            Xstrncat(
                pbuf,
                b"\r\0" as *const u8 as *const ::core::ffi::c_char,
                (MAXANALYSES * 128 as ::core::ffi::c_int) as size_t,
            );
        }
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn anal_buf() -> *mut ::core::ffi::c_char {
    return pbuf;
}
#[no_mangle]
pub unsafe extern "C" fn GoodAnals(
    mut Gkword: *mut gk_word,
    mut lemmflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut curlem: [::core::ffi::c_char; 60] = [0; 60];
    let mut Anal: *mut gk_analysis = ::core::ptr::null_mut::<gk_analysis>();
    let mut goodanals: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut difflems: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    curlem[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < (*Gkword).gw_totanal {
        Anal = (*Gkword).gw_analysis.offset(i as isize);
        if strcmp(
            &raw mut curlem as *mut ::core::ffi::c_char,
            &raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char,
        ) != 0
        {
            difflems += 1;
            strcpy(
                &raw mut curlem as *mut ::core::ffi::c_char,
                &raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char,
            );
        }
        if strchr(&raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char, '-' as i32)
            .is_null()
        {
            goodanals += 1;
        }
        i += 1;
    }
    return if lemmflag != 0 { difflems } else { goodanals };
}
#[no_mangle]
pub unsafe extern "C" fn DumpLemmaInfo(
    mut Gkword: *mut gk_word,
    mut prntflags: PrntFlags,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut Anal: *mut gk_analysis = ::core::ptr::null_mut::<gk_analysis>();
    let mut goodanals: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut difflems: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut curlem: [::core::ffi::c_char; 60] = [0; 60];
    curlem[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    goodanals = GoodAnals(Gkword, 0 as ::core::ffi::c_int);
    difflems = GoodAnals(Gkword, 1 as ::core::ffi::c_int);
    if goodanals == 0 {
        goodanals = (*Gkword).gw_totanal;
    }
    curlem[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    fprintf(
        f,
        b"form:%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*Gkword).st_rawword as *mut ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*Gkword).gw_totanal {
        Anal = (*Gkword).gw_analysis.offset(i as isize);
        if strcmp(
            &raw mut curlem as *mut ::core::ffi::c_char,
            &raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char,
        ) != 0
        {
            if strchr(&raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char, '-' as i32)
                .is_null() || goodanals == (*Gkword).gw_totanal
            {
                fprintf(
                    f,
                    b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char,
                );
            }
            strcpy(
                &raw mut curlem as *mut ::core::ffi::c_char,
                &raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char,
            );
        }
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntOneAnalysis(
    mut Gkanal: *mut gk_analysis,
    mut prntflags: PrntFlags,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut showlemma: PrntFlags = 0;
    let mut TmpGstr: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut wtmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut prntlem: [::core::ffi::c_char; 60] = [0; 60];
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut funnyacc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    TmpGstr = CreatGkString(1 as ::core::ffi::c_int);
    tmp[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    curan += 1;
    Xstrncpy(
        &raw mut prntlem as *mut ::core::ffi::c_char,
        &raw mut (*Gkanal).st_lemma as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    if prntflags as ::core::ffi::c_int & SHOW_LEMMA != 0 {
        if strcmp(
            &raw mut (*Gkanal).st_lemma as *mut ::core::ffi::c_char,
            &raw mut prevlemma as *mut ::core::ffi::c_char,
        ) != 0
        {
            if (*Gkanal).gs_preverb.gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
                sprintf(
                    &raw mut wtmp as *mut ::core::ffi::c_char,
                    b"%s\t%s-%s %d\t\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut (*Gkanal).st_rawword as *mut ::core::ffi::c_char,
                    &raw mut (*Gkanal).gs_preverb.gs_gkstring
                        as *mut ::core::ffi::c_char,
                    &raw mut (*Gkanal).st_lemma as *mut ::core::ffi::c_char,
                    curan,
                );
            } else {
                sprintf(
                    &raw mut wtmp as *mut ::core::ffi::c_char,
                    b"%s\t%s %d\t\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut (*Gkanal).st_rawword as *mut ::core::ffi::c_char,
                    &raw mut (*Gkanal).st_lemma as *mut ::core::ffi::c_char,
                    curan,
                );
            }
            Xstrncat(
                pbuf,
                &raw mut wtmp as *mut ::core::ffi::c_char,
                (MAXANALYSES * 128 as ::core::ffi::c_int) as size_t,
            );
            curan = 0 as ::core::ffi::c_int;
            strcpy(
                &raw mut wtmp as *mut ::core::ffi::c_char,
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        if strcmp(
            &raw mut (*Gkanal).st_lemma as *mut ::core::ffi::c_char,
            &raw mut prevlemma as *mut ::core::ffi::c_char,
        ) != 0
        {
            sprintf(
                &raw mut wtmp as *mut ::core::ffi::c_char,
                b"   &from$  %s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut prntlem as *mut ::core::ffi::c_char,
            );
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut wtmp as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            Xstrncpy(
                &raw mut prevlemma as *mut ::core::ffi::c_char,
                &raw mut (*Gkanal).st_lemma as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            if (*Gkanal).gs_preverb.gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
                let mut tmp2: [::core::ffi::c_char; 128] = [0; 128];
                sprintf(
                    &raw mut tmp2 as *mut ::core::ffi::c_char,
                    b" [$%s&+$%s&]\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut (*Gkanal).gs_preverb.gs_gkstring
                        as *mut ::core::ffi::c_char,
                    &raw mut (*Gkanal).st_lemma as *mut ::core::ffi::c_char,
                );
                Xstrncat(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    &raw mut tmp2 as *mut ::core::ffi::c_char,
                    LONGSTRING as size_t,
                );
            }
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                NEWLINE.as_ptr(),
                LONGSTRING as size_t,
            );
        }
        if strcmp(
            &raw mut (*Gkanal).st_workword as *mut ::core::ffi::c_char,
            &raw mut prevword as *mut ::core::ffi::c_char,
        ) != 0
        {
            let mut tmp1: [::core::ffi::c_char; 128] = [0; 128];
            Xstrncpy(
                &raw mut wtmp as *mut ::core::ffi::c_char,
                b"      \0" as *const u8 as *const ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            if (*Gkanal).st_crasis[0 as ::core::ffi::c_int as usize] != 0 {
                sprintf(
                    &raw mut tmp1 as *mut ::core::ffi::c_char,
                    b"$%s& + $\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut (*Gkanal).st_crasis as *mut ::core::ffi::c_char,
                );
                Xstrncat(
                    &raw mut wtmp as *mut ::core::ffi::c_char,
                    &raw mut tmp1 as *mut ::core::ffi::c_char,
                    LONGSTRING as size_t,
                );
            }
            sprintf(
                &raw mut tmp1 as *mut ::core::ffi::c_char,
                b"$%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*Gkanal).st_workword as *mut ::core::ffi::c_char,
                NEWLINE.as_ptr(),
            );
            Xstrncat(
                &raw mut wtmp as *mut ::core::ffi::c_char,
                &raw mut tmp1 as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut wtmp as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            Xstrncpy(
                &raw mut prevword as *mut ::core::ffi::c_char,
                &raw mut (*Gkanal).st_workword as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
        }
        odd_morpheme(
            Gkanal,
            &raw mut (*Gkanal).gs_preverb,
            b"prvb\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        odd_morpheme(
            Gkanal,
            &raw mut (*Gkanal).gs_aug1,
            b"aug\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        if strcmp(
            &raw mut (*Gkanal).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut prevstem as *mut ::core::ffi::c_char,
        ) != 0
        {
            odd_morpheme(
                Gkanal,
                &raw mut (*Gkanal).gs_stem,
                b"stem\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut tmp as *mut ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
        } else {
            odd_morpheme(
                Gkanal,
                &raw mut (*Gkanal).gs_stem,
                b"stem\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                &raw mut tmp as *mut ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
        }
        odd_morpheme(
            Gkanal,
            &raw mut (*Gkanal).gs_endstring,
            b"end\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        Xstrncat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"         &\0" as *const u8 as *const ::core::ffi::c_char,
            LONGSTRING as size_t,
        );
        (*TmpGstr).gs_forminfo = (*Gkanal).gs_forminfo;
        (*TmpGstr).gs_dialect = (*Gkanal).gs_dialect;
        set_morphflags(TmpGstr, &raw mut (*Gkanal).gs_morphflags as *mut MorphFlags);
        (*TmpGstr).gs_steminfo = (*Gkanal).gs_steminfo;
        (*TmpGstr).gs_geogregion = (*Gkanal).gs_geogregion;
        SprintGkFlags(
            TmpGstr,
            &raw mut tmp as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        Xstrncat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            NEWLINE.as_ptr(),
            LONGSTRING as size_t,
        );
        strcat(pbuf, &raw mut tmp as *mut ::core::ffi::c_char);
    }
    FreeGkString(TmpGstr);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn near_miss(
    mut gstr: *mut gk_string,
    mut checks: *mut ::core::ffi::c_char,
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn odd_morpheme(
    mut Gkanal: *mut gk_analysis,
    mut gstr: *mut gk_string,
    mut tag: *mut ::core::ffi::c_char,
    mut bufp: *mut ::core::ffi::c_char,
    mut showflg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tmp2: [::core::ffi::c_char; 128] = [0; 128];
    let mut mflagbuf: [::core::ffi::c_char; 256] = [0; 256];
    mflagbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    tmp2[0 as ::core::ffi::c_int as usize] = mflagbuf[0 as ::core::ffi::c_int as usize];
    MorphNames(
        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        &raw mut mflagbuf as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    if (*gstr).gs_dialect as ::core::ffi::c_int != 0
        || mflagbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        || showflg != 0
    {
        if strcmp(tag, b"end\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            sprintf(
                &raw mut tmp2 as *mut ::core::ffi::c_char,
                b"        [&%s $-%s& \0" as *const u8 as *const ::core::ffi::c_char,
                tag,
                &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
            );
        } else {
            sprintf(
                &raw mut tmp2 as *mut ::core::ffi::c_char,
                b"        [&%s $%s-& \0" as *const u8 as *const ::core::ffi::c_char,
                tag,
                &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
            );
        }
        Xstrncat(bufp, &raw mut tmp2 as *mut ::core::ffi::c_char, LONGSTRING as size_t);
        if (*gstr).gs_dialect != 0 {
            DialectNames(
                (*gstr).gs_dialect,
                &raw mut tmp2 as *mut ::core::ffi::c_char,
                b" \0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            Xstrncat(
                bufp,
                &raw mut tmp2 as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
        }
        if mflagbuf[0 as ::core::ffi::c_int as usize] != 0 {
            Xstrncat(
                bufp,
                b" \0" as *const u8 as *const ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            Xstrncat(
                bufp,
                &raw mut mflagbuf as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
        }
        Xstrncat(
            bufp,
            b"]\0" as *const u8 as *const ::core::ffi::c_char,
            LONGSTRING as size_t,
        );
        Xstrncat(bufp, NEWLINE.as_ptr(), LONGSTRING as size_t);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn dump_all_anals(
    mut Gkword: *mut gk_word,
    mut prntflags: PrntFlags,
    mut fout: *mut FILE,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nanals: ::core::ffi::c_int = (*Gkword).gw_totanal;
    let mut goodanals: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut Anal: *mut gk_analysis = ::core::ptr::null_mut::<gk_analysis>();
    let mut curlem: [::core::ffi::c_char; 60] = [0; 60];
    let mut printedwork: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    goodanals = GoodAnals(Gkword, 0 as ::core::ffi::c_int);
    if prntflags as ::core::ffi::c_int & PERSEUS_FORMAT != 0 {
        fprintf(
            fout,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*Gkword).st_rawword as *mut ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < nanals {
        Anal = (*Gkword).gw_analysis.offset(i as isize);
        printedwork = 0 as ::core::ffi::c_int;
        if prntflags as ::core::ffi::c_int & PERSEUS_FORMAT != 0 {
            if goodanals == 0
                || goodanals != 0
                    && strchr(
                            &raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char,
                            '-' as i32,
                        )
                        .is_null()
            {
                DumpPerseusAnalysis(
                    Gkword,
                    prntflags,
                    Anal,
                    fout,
                    i + 1 as ::core::ffi::c_int,
                );
                strcpy(
                    &raw mut curlem as *mut ::core::ffi::c_char,
                    &raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char,
                );
            }
        } else if prntflags as ::core::ffi::c_int & ENDING_INDEX != 0 {
            DumpEndingIndex(Gkword, prntflags, Anal, fout, i + 1 as ::core::ffi::c_int);
        } else {
            DumpOneAnalysis(Gkword, prntflags, Anal, fout, i + 1 as ::core::ffi::c_int);
            Xstrncpy(
                &raw mut prevlemma as *mut ::core::ffi::c_char,
                &raw mut (*Anal).st_lemma as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
        }
        i += 1;
    }
    if prntflags as ::core::ffi::c_int & PERSEUS_FORMAT != 0 {
        fprintf(fout, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn SortAnals(
    mut Anal: *mut gk_analysis,
    mut nanals: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    qsort(
        Anal as *mut ::core::ffi::c_void,
        nanals as ::core::ffi::c_long as size_t,
        ::core::mem::size_of::<gk_analysis>() as ::core::ffi::c_int as size_t,
        Some(
            CompAnals
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn CompAnals(
    mut Anal1: *const ::core::ffi::c_void,
    mut Anal2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        &raw mut (*(Anal1 as *mut gk_analysis)).st_lemma as *mut ::core::ffi::c_char,
        &raw mut (*(Anal2 as *mut gk_analysis)).st_lemma as *mut ::core::ffi::c_char,
    );
}
static mut EndGstr: gk_string = gk_string {
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
static mut forminfo: word_form = word_form {
    f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
};
#[no_mangle]
pub unsafe extern "C" fn DumpPerseusAnalysis(
    mut Gkword: *mut gk_word,
    mut prntflags: PrntFlags,
    mut anal: *mut gk_analysis,
    mut fout: *mut FILE,
    mut cura: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tmp2: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut workw: [::core::ffi::c_char; 1024] = [0; 1024];
    workw[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    tmp[0 as ::core::ffi::c_int as usize] = workw[0 as ::core::ffi::c_int as usize];
    fprintf(fout, b"<NL>\0" as *const u8 as *const ::core::ffi::c_char);
    if (*anal).gs_forminfo.f_mood() as ::core::ffi::c_int == PARTICIPLE {
        fprintf(fout, b"P \0" as *const u8 as *const ::core::ffi::c_char);
    } else if (*anal).gs_steminfo & 0o10000 as Stemtype != 0
        || (*anal).gs_steminfo & 0o4000 as Stemtype != 0
    {
        if (if 0 as ::core::ffi::c_int != 0 {
            isupper(
                (*anal).st_lemma[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            )
        } else {
            (((*anal).st_lemma[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint)
                .wrapping_sub('A' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) != 0 && cur_lang() == GREEK
        {
            fprintf(fout, b"E \0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            fprintf(fout, b"N \0" as *const u8 as *const ::core::ffi::c_char);
        }
    } else if (*anal).gs_steminfo & 0o70000000 as ::core::ffi::c_int as Stemtype != 0 {
        fprintf(fout, b"V \0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        fprintf(fout, b"I \0" as *const u8 as *const ::core::ffi::c_char);
    }
    if strcmp(
        &raw mut (*anal).st_rawword as *mut ::core::ffi::c_char,
        &raw mut (*anal).st_workword as *mut ::core::ffi::c_char,
    ) != 0
    {
        let mut tmp_0: [::core::ffi::c_char; 60] = [0; 60];
        strcpy(
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
            &raw mut (*anal).st_workword as *mut ::core::ffi::c_char,
        );
        if strcmp(
            &raw mut (*anal).st_rawword as *mut ::core::ffi::c_char,
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
        ) != 0
        {
            fprintf(
                fout,
                b"%s,\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut tmp_0 as *mut ::core::ffi::c_char,
            );
        }
    }
    fprintf(
        fout,
        b"%s \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*anal).st_lemma as *mut ::core::ffi::c_char,
    );
    if prntflags as ::core::ffi::c_int & ENDING_INDEX != 0 {
        fprintf(
            fout,
            b"\t%d</NL>\0" as *const u8 as *const ::core::ffi::c_char,
            (*anal).gs_forminfo,
        );
    } else {
        GregSprintGkFlags(
            anal as *mut gk_string,
            &raw mut tmp as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        fprintf(
            fout,
            b"%s</NL>\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
        );
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn DumpEndingIndex(
    mut Gkword: *mut gk_word,
    mut prntflags: PrntFlags,
    mut anal: *mut gk_analysis,
    mut fout: *mut FILE,
    mut cura: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    tmp[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    PrntStemtype((*anal).gs_steminfo, fout);
    fprintf(
        fout,
        b"%s \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*anal).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
    );
    AddParadigmInfo(
        &raw mut tmp as *mut ::core::ffi::c_char,
        (*anal).gs_forminfo,
        b".\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    AddPersNumInfo(
        &raw mut tmp as *mut ::core::ffi::c_char,
        (*anal).gs_forminfo,
        b".\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    AddAdjInfo(
        &raw mut tmp as *mut ::core::ffi::c_char,
        (*anal).gs_forminfo,
        b".\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    fprintf(
        fout,
        b"%s %s %d\0" as *const u8 as *const ::core::ffi::c_char,
        (&raw mut tmp as *mut ::core::ffi::c_char)
            .offset(1 as ::core::ffi::c_int as isize),
        &raw mut (*anal).st_workword as *mut ::core::ffi::c_char,
        (*Gkword).gw_totanal,
    );
    if strcmp(
        &raw mut (*anal).st_workword as *mut ::core::ffi::c_char,
        &raw mut (*anal).st_rawword as *mut ::core::ffi::c_char,
    ) != 0
    {
        fprintf(
            fout,
            b" %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).st_rawword as *mut ::core::ffi::c_char,
        );
    }
    fprintf(fout, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn DumpOneAnalysis(
    mut Gkword: *mut gk_word,
    mut prntflags: PrntFlags,
    mut anal: *mut gk_analysis,
    mut fout: *mut FILE,
    mut cura: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tmp2: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut workw: [::core::ffi::c_char; 1024] = [0; 1024];
    workw[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    tmp[0 as ::core::ffi::c_int as usize] = workw[0 as ::core::ffi::c_int as usize];
    if prntflags as ::core::ffi::c_int & LEXICON_OUTPUT != 0 {
        sprintf(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).st_rawword as *mut ::core::ffi::c_char,
        );
        if strcmp(
            &raw mut (*anal).st_rawword as *mut ::core::ffi::c_char,
            &raw mut (*anal).st_workword as *mut ::core::ffi::c_char,
        ) != 0
        {
            strcat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                b" \0" as *const u8 as *const ::core::ffi::c_char,
            );
            strcat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut (*anal).st_workword as *mut ::core::ffi::c_char,
            );
        }
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char,
        );
        if (*anal).gs_preverb.gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
            strcat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut (*anal).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
            );
            strcat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                b"-\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            &raw mut (*anal).st_lemma as *mut ::core::ffi::c_char,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"\t\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            fout,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
        );
        tmp[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        JakeSprintGkFlags(
            anal as *mut gk_string,
            &raw mut tmp as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        fprintf(
            fout,
            b"%s\t\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
        );
        fprintf(
            fout,
            b"%s\t\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).st_crasis as *mut ::core::ffi::c_char,
        );
        fprintf(fout, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        return 0;
    }
    if prntflags as ::core::ffi::c_int & DBASEFORMAT != 0 {
        let mut tmp_0: [::core::ffi::c_char; 1024] = [0; 1024];
        tmp_0[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        fprintf(
            fout,
            b"\n:raw %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).st_rawword as *mut ::core::ffi::c_char,
        );
        fprintf(
            fout,
            b"\n:workw %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).st_workword as *mut ::core::ffi::c_char,
        );
        fprintf(
            fout,
            b":lem %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).st_lemma as *mut ::core::ffi::c_char,
        );
        fprintf(fout, b":prvb \0" as *const u8 as *const ::core::ffi::c_char);
        GregSprintGkFlags(
            &raw mut (*anal).gs_preverb,
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        fprintf(
            fout,
            b"%s\t%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
        );
        tmp_0[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        fprintf(fout, b":aug1 \0" as *const u8 as *const ::core::ffi::c_char);
        GregSprintGkFlags(
            &raw mut (*anal).gs_aug1,
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        fprintf(
            fout,
            b"%s\t%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).gs_aug1.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
        );
        tmp_0[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        fprintf(fout, b":stem \0" as *const u8 as *const ::core::ffi::c_char);
        GregSprintGkFlags(
            &raw mut (*anal).gs_stem,
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        fprintf(
            fout,
            b"%s\t%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
        );
        tmp_0[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        fprintf(fout, b":suff \0" as *const u8 as *const ::core::ffi::c_char);
        GregSprintGkFlags(
            &raw mut (*anal).gs_suffix,
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        fprintf(
            fout,
            b"%s\t%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).gs_suffix.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
        );
        tmp_0[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        fprintf(fout, b":end \0" as *const u8 as *const ::core::ffi::c_char);
        GregSprintGkFlags(
            &raw mut (*anal).gs_endstring,
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        fprintf(
            fout,
            b"%s\t%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut tmp_0 as *mut ::core::ffi::c_char,
        );
        tmp_0[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        return 0;
    }
    JakeSprintGkFlags(
        anal as *mut gk_string,
        &raw mut tmp as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    if (*anal).gs_preverb.gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
        strcpy(
            &raw mut workw as *mut ::core::ffi::c_char,
            &raw mut (*anal).gs_preverb.gs_gkstring as *mut ::core::ffi::c_char,
        );
        strcat(
            &raw mut workw as *mut ::core::ffi::c_char,
            b"-\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*anal).gs_aug1.gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(
            &raw mut workw as *mut ::core::ffi::c_char,
            &raw mut (*anal).gs_aug1.gs_gkstring as *mut ::core::ffi::c_char,
        );
        strcat(
            &raw mut workw as *mut ::core::ffi::c_char,
            b"-\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        fout,
        b":summ %d %s %s%s-%s %s %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        cura,
        &raw mut (*anal).st_rawword as *mut ::core::ffi::c_char,
        &raw mut workw as *mut ::core::ffi::c_char,
        &raw mut (*anal).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut (*anal).gs_endstring.gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut (*anal).st_lemma as *mut ::core::ffi::c_char,
        &raw mut tmp as *mut ::core::ffi::c_char,
    );
    if (*anal).gs_preverb.gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
        DumpGstr(
            b":pvb\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut (*anal).gs_preverb,
            fout,
            prntflags as ::core::ffi::c_int & PARSE_FORMAT,
        );
    }
    if (*anal).gs_aug1.gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
        DumpGstr(
            b":aug\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut (*anal).gs_aug1,
            fout,
            prntflags as ::core::ffi::c_int & PARSE_FORMAT,
        );
    }
    if (*anal).gs_stem.gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
        DumpGstr(
            b":stem\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut (*anal).gs_stem,
            fout,
            prntflags as ::core::ffi::c_int & PARSE_FORMAT,
        );
    }
    if (*anal).gs_endstring.gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
        EndGstr = (*anal).gs_endstring;
        EndGstr.gs_forminfo = forminfo;
        DumpGstr(
            b":end\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut EndGstr,
            fout,
            prntflags as ::core::ffi::c_int & PARSE_FORMAT,
        );
    }
    if (*anal).st_crasis[0 as ::core::ffi::c_int as usize] != 0 {
        fprintf(
            fout,
            b":crasis %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*anal).st_crasis as *mut ::core::ffi::c_char,
        );
    }
    fprintf(fout, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn DumpGstr(
    mut tags: *mut ::core::ffi::c_char,
    mut gstr: *mut gk_string,
    mut fout: *mut FILE,
    mut fullrec: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    tmp[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    fprintf(
        fout,
        b"%s\t%s\t\0" as *const u8 as *const ::core::ffi::c_char,
        tags,
        &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
    );
    SprintGkFlags(
        gstr,
        &raw mut tmp as *mut ::core::ffi::c_char,
        b"\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    if fullrec != 0 {
        let mut s: *mut ::core::ffi::c_char = &raw mut tmp as *mut ::core::ffi::c_char;
        while *s != 0 {
            if __isspace(*s as ::core::ffi::c_int) != 0 {
                fprintf(fout, b" \0" as *const u8 as *const ::core::ffi::c_char);
                while __isspace(*s as ::core::ffi::c_int) != 0 {
                    s = s.offset(1);
                }
            } else {
                let fresh0 = s;
                s = s.offset(1);
                fprintf(
                    fout,
                    b"%c\0" as *const u8 as *const ::core::ffi::c_char,
                    *fresh0 as ::core::ffi::c_int,
                );
            }
        }
        fprintf(fout, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        fprintf(
            fout,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
        );
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn DumpDbGkString(
    mut gstr: *mut gk_string,
    mut fout: *mut FILE,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    tmp[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    fprintf(
        fout,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
    );
    SprintGkFlags(
        gstr,
        &raw mut tmp as *mut ::core::ffi::c_char,
        b"\t\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    fprintf(
        fout,
        b"%s\t\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut tmp as *mut ::core::ffi::c_char,
    );
    return 0;
}
