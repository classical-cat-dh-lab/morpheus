use ::c2rust_bitfields;
extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn CreatGkword(_: ::core::ffi::c_int) -> *mut gk_word;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn ScanAsciiKeys(
        _: *mut ::core::ffi::c_char,
        _: *mut gk_word,
        _: *mut gk_string,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn NameOfStemtype(st: Stemtype) -> *mut ::core::ffi::c_char;
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn CompGkString(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn EuphEnd(
        _: *mut gk_string,
        _: *mut gk_string,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn FreeGkword(_: *mut gk_word) -> ::core::ffi::c_int;
    fn GetCurrentEndList(
        _: *mut gk_string,
        _: *mut ::core::ffi::c_int,
    ) -> *mut gk_string;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn add_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn dictstrcmp(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn has_quant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn hasaccent(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn markstem(_: *mut ::core::ffi::c_char, _: *mut gk_string) -> ::core::ffi::c_int;
    fn no_morphflags(_: *mut gk_string) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripquant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const PPARTMASK: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const PP_PF: ::core::ffi::c_int = 0o40000000 as ::core::ffi::c_int;
pub const PP_SU: ::core::ffi::c_int = 0o60000000 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ACTIVE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const MIDDLE: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const DEPONENT: ::core::ffi::c_int = MIDDLE | ACTIVE;
pub const PARTICIPLE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const GERUNDIVE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const COMPARATIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SUPERLATIVE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const HARDSHORT: ::core::ffi::c_int = '^' as i32;
pub const COMP_ONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PERS_NAME: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const IRREG_SUPERL: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const IRREG_COMP: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const NO_COMP: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const NOT_IN_COMPOSITION: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const HAS_PREVERB: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const IS_DERIV: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const GROUP_NAME: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
static mut Cur_gkend: gk_string = gk_string {
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
static mut WantEnd: gk_string = gk_string {
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
static mut AvoidEnd: gk_string = gk_string {
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
static mut BlankGkend: gk_string = gk_string {
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
static mut start_match: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn chckendings(
    mut endstr: *mut ::core::ffi::c_char,
    mut restricts: *mut ::core::ffi::c_char,
    mut stemstr: *mut ::core::ffi::c_char,
    mut prevbstr: *mut ::core::ffi::c_char,
    mut dial: Dialect,
    mut nends: *mut ::core::ffi::c_int,
) -> *mut gk_string {
    let mut gstring: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut stemkeys: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut OrDialect: Dialect = 0;
    let mut tmpgstr: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut BlnkGkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    Xstrncpy(
        &raw mut stemkeys as *mut ::core::ffi::c_char,
        restricts,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int
            as size_t,
    );
    WantEnd = BlankGkend;
    AvoidEnd = BlankGkend;
    tmpgstr = &raw mut WantEnd;
    BlnkGkword = CreatGkword(1 as ::core::ffi::c_int);
    ScanAsciiKeys(
        &raw mut stemkeys as *mut ::core::ffi::c_char,
        BlnkGkword,
        &raw mut WantEnd,
        &raw mut AvoidEnd,
    );
    FreeGkword(BlnkGkword);
    if WantEnd.gs_steminfo == 0 {
        printf(
            b"could not find a stemtype for [%s] with restricts [%s]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            restricts,
        );
        return ::core::ptr::null_mut::<gk_string>();
    }
    if !endstr.is_null() {
        Xstrncpy(
            &raw mut WantEnd.gs_gkstring as *mut ::core::ffi::c_char,
            endstr,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
    }
    if !stemstr.is_null() {
        markstem(stemstr, &raw mut WantEnd);
    }
    OrDialect = WantEnd.gs_dialect;
    if dial != 0 {
        WantEnd.gs_dialect = (WantEnd.gs_dialect as ::core::ffi::c_int
            | dial as ::core::ffi::c_int) as Dialect;
    } else {
        WantEnd.gs_dialect = dial;
    }
    if !prevbstr.is_null() {
        if *prevbstr != 0 {
            add_morphflag(
                &raw mut (*tmpgstr).gs_morphflags as *mut MorphFlags,
                HAS_PREVERB,
            );
        }
    }
    if has_morphflag(&raw mut (*tmpgstr).gs_morphflags as *mut MorphFlags, PERS_NAME)
        != 0
    {
        if { let form = WantEnd.gs_forminfo; form }.f_number() == 0 {
            { let mut form = WantEnd.gs_forminfo; form.set_f_number(0o1 as ::core::ffi::c_uint as ::core::ffi::c_uint); WantEnd.gs_forminfo = form; }
        }
    }
    if has_morphflag(&raw mut WantEnd.gs_morphflags as *mut MorphFlags, GROUP_NAME) != 0
    {
        if { let form = WantEnd.gs_forminfo; form }.f_number() == 0 {
            { let mut form = WantEnd.gs_forminfo; form.set_f_number(0o4 as ::core::ffi::c_uint as ::core::ffi::c_uint); WantEnd.gs_forminfo = form; }
        }
    }
    gstring = RetrCompEnds(&raw mut WantEnd, &raw mut AvoidEnd, nends, OrDialect);
    return gstring;
}
#[no_mangle]
pub unsafe extern "C" fn CompatKeys(
    mut keys1: *mut ::core::ffi::c_char,
    mut keys2: *mut ::core::ffi::c_char,
    mut gstr: *mut gk_string,
) -> ::core::ffi::c_int {
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
    let mut Gstr2: gk_string = gk_string {
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
    let mut BlnkGkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_deriv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    BlnkGkword = CreatGkword(1 as ::core::ffi::c_int);
    is_deriv = has_morphflag(
        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        IS_DERIV,
    );
    Gstr = BlankGkend;
    Gstr2 = BlankGkend;
    *gstr = BlankGkend;
    if is_deriv != 0 {
        add_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, IS_DERIV);
    }
    ScanAsciiKeys(keys1, BlnkGkword, gstr, ::core::ptr::null_mut::<gk_string>());
    ScanAsciiKeys(
        keys2,
        BlnkGkword,
        &raw mut Gstr2,
        ::core::ptr::null_mut::<gk_string>(),
    );
    FreeGkword(BlnkGkword);
    rval = EndingOk(keys2, gstr, &raw mut Gstr, 1 as ::core::ffi::c_int);
    if rval != 0 {
        if Gstr2.gs_forminfo.f_tense() != 0 {
            (*gstr)
                .gs_forminfo
                .set_f_tense(Gstr2.gs_forminfo.f_tense() as ::core::ffi::c_uint);
        }
        if Gstr2.gs_forminfo.f_mood() != 0 {
            (*gstr)
                .gs_forminfo
                .set_f_mood(Gstr2.gs_forminfo.f_mood() as ::core::ffi::c_uint);
        }
        if Gstr2.gs_forminfo.f_voice() != 0 {
            (*gstr)
                .gs_forminfo
                .set_f_voice(Gstr2.gs_forminfo.f_voice() as ::core::ffi::c_uint);
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn EndingOk(
    mut keys: *mut ::core::ffi::c_char,
    mut gstr: *mut gk_string,
    mut avoidgstr: *mut gk_string,
    mut wantderiv: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut good: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut BlnkGkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    Cur_gkend = BlankGkend;
    BlnkGkword = CreatGkword(1 as ::core::ffi::c_int);
    ScanAsciiKeys(keys, BlnkGkword, &raw mut Cur_gkend, avoidgstr);
    FreeGkword(BlnkGkword);
    if wantderiv != 0 {
        if (*gstr).gs_derivtype == 0 || (*gstr).gs_derivtype < 0 as Derivtype {
            return 0 as ::core::ffi::c_int;
        }
        if (*gstr).gs_derivtype != Cur_gkend.gs_derivtype {
            return 0 as ::core::ffi::c_int;
        }
    }
    good = (WantGkEnd(gstr, &raw mut Cur_gkend, NO, 0 as ::core::ffi::c_int)
        > 0 as ::core::ffi::c_int
        && NoWantGkEnd(avoidgstr, &raw mut Cur_gkend, 1 as ::core::ffi::c_int)
            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if good != 0 {
        let mut case1: ::core::ffi::c_int = 0;
        let mut case2: ::core::ffi::c_int = 0;
        let mut gend1: ::core::ffi::c_int = 0;
        let mut gend2: ::core::ffi::c_int = 0;
        let mut num2: ::core::ffi::c_int = 0;
        let mut pers2: ::core::ffi::c_int = 0;
        let mut voice1: ::core::ffi::c_int = 0;
        let mut voice2: ::core::ffi::c_int = 0;
        let mut dial1: Dialect = 0;
        let mut dial2: Dialect = 0;
        voice1 = { let form = Cur_gkend.gs_forminfo; form }.f_voice() as ::core::ffi::c_int;
        voice2 = (*gstr).gs_forminfo.f_voice() as ::core::ffi::c_int;
        if voice1 != 0 {
            (*gstr)
                .gs_forminfo
                .set_f_voice(voice1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        case1 = { let form = Cur_gkend.gs_forminfo; form }.f_case() as ::core::ffi::c_int;
        case2 = (*gstr).gs_forminfo.f_case() as ::core::ffi::c_int;
        if case1 & case2 != 0 {
            (*gstr)
                .gs_forminfo
                .set_f_case(
                    (case1 & case2) as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
        } else if case1 != 0 && case2 == 0 {
            (*gstr)
                .gs_forminfo
                .set_f_case(case1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        num2 = { let form = Cur_gkend.gs_forminfo; form }.f_number() as ::core::ffi::c_int;
        if num2 != 0 {
            (*gstr)
                .gs_forminfo
                .set_f_number({ let form = Cur_gkend.gs_forminfo; form }.f_number() as ::core::ffi::c_uint);
        }
        pers2 = { let form = Cur_gkend.gs_forminfo; form }.f_person() as ::core::ffi::c_int;
        if pers2 != 0 {
            (*gstr)
                .gs_forminfo
                .set_f_person({ let form = Cur_gkend.gs_forminfo; form }.f_person() as ::core::ffi::c_uint);
        }
        gend1 = { let form = Cur_gkend.gs_forminfo; form }.f_gender() as ::core::ffi::c_int;
        gend2 = (*gstr).gs_forminfo.f_gender() as ::core::ffi::c_int;
        if gend1 & gend2 != 0 {
            (*gstr)
                .gs_forminfo
                .set_f_gender(
                    (gend1 & gend2) as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
        } else if gend1 != 0 && gend2 == 0 {
            (*gstr)
                .gs_forminfo
                .set_f_gender(gend1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        dial1 = Cur_gkend.gs_dialect;
        dial2 = (*gstr).gs_dialect;
        if dial1 as ::core::ffi::c_int & dial2 as ::core::ffi::c_int != 0 {
            (*gstr).gs_dialect = (dial1 as ::core::ffi::c_int
                & dial2 as ::core::ffi::c_int) as Dialect;
        } else if dial1 as ::core::ffi::c_int != 0 && dial2 == 0 {
            (*gstr).gs_dialect = dial1;
        }
        add_morphflags(gstr, &raw mut Cur_gkend.gs_morphflags as *mut MorphFlags);
    }
    return good;
}
unsafe extern "C" fn RetrCompEnds(
    mut wantgkend: *mut gk_string,
    mut avoidgkend: *mut gk_string,
    mut nends: *mut ::core::ffi::c_int,
    mut OrDialect: Dialect,
) -> *mut gk_string {
    let mut ListOfEnds: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut CurrentList: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut rval: ::core::ffi::c_int = 0;
    let mut avoidrval: ::core::ffi::c_int = 0;
    let mut lno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut maxend: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut i: ::core::ffi::c_int = 0;
    *nends = 0 as ::core::ffi::c_int;
    CurrentList = GetCurrentEndList(wantgkend, &raw mut lno);
    ListOfEnds = CreatGkString(lno + 2 as ::core::ffi::c_int);
    if ListOfEnds.is_null() {
        fprintf(
            stderr,
            b"Could not find space to store %d greek endings for stem %s\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            lno + 2 as ::core::ffi::c_int,
            NameOfStemtype((*wantgkend).gs_steminfo),
        );
        return ::core::ptr::null_mut::<gk_string>();
    }
    i = 0 as ::core::ffi::c_int;
    while i < lno {
        Cur_gkend = *CurrentList.offset(i as isize);
        rval = WantGkEnd(wantgkend, &raw mut Cur_gkend, YES, YES);
        avoidrval = NoWantGkEnd(avoidgkend, &raw mut Cur_gkend, 0 as ::core::ffi::c_int);
        if rval < 0 as ::core::ffi::c_int || avoidrval != 0 {
            break;
        }
        if rval != 0 && avoidrval == 0 {
            Cur_gkend.gs_steminfo = (*wantgkend).gs_steminfo;
            AddNewEnd(ListOfEnds, &raw mut Cur_gkend, *nends);
            *nends += 1;
        }
        i += 1;
    }
    if *nends == 0 {
        FreeGkString(ListOfEnds);
        ListOfEnds = ::core::ptr::null_mut::<gk_string>();
        return ::core::ptr::null_mut::<gk_string>();
    }
    return ListOfEnds;
}
unsafe extern "C" fn NoWantGkEnd(
    mut skipend: *mut gk_string,
    mut haveend: *mut gk_string,
    mut strict: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*skipend).gs_dialect != 0 {
        if (*haveend).gs_dialect == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if AndDialect((*skipend).gs_dialect, (*haveend).gs_dialect) as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
    }
    if (*skipend).gs_gkstring[0 as ::core::ffi::c_int as usize] == 0
        && (*skipend).gs_forminfo.f_mood() == 0 && (*skipend).gs_forminfo.f_tense() == 0
        && (*skipend).gs_forminfo.f_voice() == 0
        && (*skipend).gs_forminfo.f_number() == 0
        && (*skipend).gs_forminfo.f_person() == 0
        && (*skipend).gs_forminfo.f_gender() == 0 && (*skipend).gs_forminfo.f_case() == 0
        && (*skipend).gs_steminfo == 0 && no_morphflags(skipend) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return if WantGkEnd(skipend, haveend, NO, strict) <= 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn WantGkEnd(
    mut wantend: *mut gk_string,
    mut haveend: *mut gk_string,
    mut writeflag: bool_0,
    mut strict: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut compval: ::core::ffi::c_int = 0;
    let mut wform: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    let mut hform: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    let mut wdial: Dialect = 0;
    let mut hdial: Dialect = 0;
    let mut d: Dialect = 0;
    let mut wmood: ::core::ffi::c_int = (*wantend).gs_forminfo.f_mood()
        as ::core::ffi::c_int;
    let mut hmood: ::core::ffi::c_int = (*haveend).gs_forminfo.f_mood()
        as ::core::ffi::c_int;
    let mut wvoice: ::core::ffi::c_int = (*wantend).gs_forminfo.f_voice()
        as ::core::ffi::c_int;
    let mut hvoice: ::core::ffi::c_int = (*haveend).gs_forminfo.f_voice()
        as ::core::ffi::c_int;
    let mut wtense: ::core::ffi::c_int = (*wantend).gs_forminfo.f_tense()
        as ::core::ffi::c_int;
    let mut htense: ::core::ffi::c_int = (*haveend).gs_forminfo.f_tense()
        as ::core::ffi::c_int;
    let mut wgender: ::core::ffi::c_int = (*wantend).gs_forminfo.f_gender()
        as ::core::ffi::c_int;
    let mut hgender: ::core::ffi::c_int = (*haveend).gs_forminfo.f_gender()
        as ::core::ffi::c_int;
    let mut wdegree: ::core::ffi::c_int = (*wantend).gs_forminfo.f_degree()
        as ::core::ffi::c_int;
    let mut hdegree: ::core::ffi::c_int = (*haveend).gs_forminfo.f_degree()
        as ::core::ffi::c_int;
    let mut wcase: ::core::ffi::c_int = (*wantend).gs_forminfo.f_case()
        as ::core::ffi::c_int;
    let mut hcase: ::core::ffi::c_int = (*haveend).gs_forminfo.f_case()
        as ::core::ffi::c_int;
    let mut wperson: ::core::ffi::c_int = (*wantend).gs_forminfo.f_person()
        as ::core::ffi::c_int;
    let mut hperson: ::core::ffi::c_int = (*haveend).gs_forminfo.f_person()
        as ::core::ffi::c_int;
    let mut wnumber: ::core::ffi::c_int = (*wantend).gs_forminfo.f_number()
        as ::core::ffi::c_int;
    let mut hnumber: ::core::ffi::c_int = (*haveend).gs_forminfo.f_number()
        as ::core::ffi::c_int;
    let mut wendstr: [::core::ffi::c_char; 60] = [0; 60];
    let mut hendstr: *mut ::core::ffi::c_char = &raw mut (*haveend).gs_gkstring
        as *mut ::core::ffi::c_char;
    wform = (*wantend).gs_forminfo;
    hform = (*haveend).gs_forminfo;
    setwendstr(
        &raw mut wendstr as *mut ::core::ffi::c_char,
        &raw mut (*wantend).gs_gkstring as *mut ::core::ffi::c_char,
    );
    if wmood != 0 && hmood != wmood {
        if hmood != 0 || hmood == 0 && strict != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*wantend).gs_steminfo & PPARTMASK as Stemtype == PP_PF as Stemtype {
        wvoice = ACTIVE;
        wform.set_f_voice(ACTIVE as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if (*wantend).gs_steminfo & PPARTMASK as Stemtype == PP_SU as Stemtype
        && cur_lang() == LATIN && wvoice == DEPONENT
    {
        (*haveend)
            .gs_forminfo
            .set_f_voice(wvoice as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if cur_lang() == LATIN && wvoice == DEPONENT {
        if hvoice == ACTIVE && hmood != PARTICIPLE && hmood != GERUNDIVE {
            return 0 as ::core::ffi::c_int;
        }
    } else if wvoice != 0 {
        if wform.f_voice() as ::core::ffi::c_int & hform.f_voice() as ::core::ffi::c_int
            == 0
        {
            if hvoice != 0 || hvoice == 0 && strict != 0 {
                return 0 as ::core::ffi::c_int;
            }
        } else if writeflag == YES {
            (*haveend)
                .gs_forminfo
                .set_f_voice(wvoice as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
    if wtense != 0 && htense != wtense {
        if htense != 0 || htense == 0 && strict != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if wgender != 0 {
        if hgender != 0 {
            if wform.f_gender() as ::core::ffi::c_int
                & hform.f_gender() as ::core::ffi::c_int == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        if writeflag != 0 {
            if wform.f_gender() as ::core::ffi::c_int
                & hform.f_gender() as ::core::ffi::c_int != 0
            {
                (*haveend)
                    .gs_forminfo
                    .set_f_gender(
                        (wform.f_gender() as ::core::ffi::c_int
                            & hform.f_gender() as ::core::ffi::c_int)
                            as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
            } else {
                (*haveend)
                    .gs_forminfo
                    .set_f_gender(wgender as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
    }
    if wcase != 0 {
        if wform.f_case() as ::core::ffi::c_int & hform.f_case() as ::core::ffi::c_int
            == 0
        {
            if hcase != 0 || hcase == 0 && strict != 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        if writeflag != 0 {
            (*haveend)
                .gs_forminfo
                .set_f_case(
                    (wcase & hcase) as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
        }
    }
    if wperson != 0 && hperson != wperson {
        if hperson != 0 || hperson == 0 && strict != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if wnumber != 0 && hnumber != wnumber {
        if hnumber != 0 || hnumber == 0 && strict != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *(&raw mut wendstr as *mut ::core::ffi::c_char) as ::core::ffi::c_int != 0
        && strict != 0
    {
        compval = endstrcmp(&raw mut wendstr as *mut ::core::ffi::c_char, hendstr);
        if compval != 0 {
            if compval < 0 as ::core::ffi::c_int
                && noaccstrcmp(&raw mut wendstr as *mut ::core::ffi::c_char, hendstr)
                    != 0
            {
                return compval;
            }
            if has_quantacc(hendstr)
                != has_quantacc(&raw mut wendstr as *mut ::core::ffi::c_char)
            {
                let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
                Xstrncpy(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    hendstr,
                    MAXWORDSIZE as size_t,
                );
                stripquantacc(&raw mut tmp as *mut ::core::ffi::c_char);
                compval = endstrcmp(
                    &raw mut wendstr as *mut ::core::ffi::c_char,
                    &raw mut tmp as *mut ::core::ffi::c_char,
                );
                if compval != 0 {
                    return 0 as ::core::ffi::c_int;
                }
            } else {
                return 0 as ::core::ffi::c_int
            }
        } else if hasaccent(&raw mut wendstr as *mut ::core::ffi::c_char) != 0 {
            let mut tmp1: [::core::ffi::c_char; 60] = [0; 60];
            let mut tmp2: [::core::ffi::c_char; 60] = [0; 60];
            Xstrncpy(
                &raw mut tmp1 as *mut ::core::ffi::c_char,
                &raw mut wendstr as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
                    as size_t,
            );
            stripquant(&raw mut tmp1 as *mut ::core::ffi::c_char);
            Xstrncpy(
                &raw mut tmp2 as *mut ::core::ffi::c_char,
                hendstr,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
                    as size_t,
            );
            stripquant(&raw mut tmp2 as *mut ::core::ffi::c_char);
            if strcmp(
                &raw mut tmp1 as *mut ::core::ffi::c_char,
                &raw mut tmp2 as *mut ::core::ffi::c_char,
            ) != 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    if EuphEnd(wantend, haveend, strict) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    wdial = (*wantend).gs_dialect;
    hdial = (*haveend).gs_dialect;
    d = AndDialect(hdial, wdial);
    if (d as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if wdegree != 0 {
        if hdegree != 0 && wdegree != hdegree && strict != 0 {
            if hdegree != 0 || hdegree == 0 && strict != 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        if wdegree != 0 && writeflag != 0 {
            (*haveend)
                .gs_forminfo
                .set_f_degree(wdegree as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
    if RightMorphflags(wantend, haveend) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RightMorphflags(
    mut wantend: *mut gk_string,
    mut haveend: *mut gk_string,
) -> ::core::ffi::c_int {
    if (has_morphflag(&raw mut (*wantend).gs_morphflags as *mut MorphFlags, IRREG_COMP)
        != 0
        || has_morphflag(
            &raw mut (*wantend).gs_morphflags as *mut MorphFlags,
            IRREG_SUPERL,
        ) != 0
        || has_morphflag(&raw mut (*wantend).gs_morphflags as *mut MorphFlags, NO_COMP)
            != 0)
        && ((*haveend).gs_forminfo.f_degree() as ::core::ffi::c_int == SUPERLATIVE
            || (*haveend).gs_forminfo.f_degree() as ::core::ffi::c_int == COMPARATIVE)
    {
        return 0 as ::core::ffi::c_int;
    }
    if has_morphflag(&raw mut (*haveend).gs_morphflags as *mut MorphFlags, COMP_ONLY)
        != 0
        && has_morphflag(
            &raw mut (*wantend).gs_morphflags as *mut MorphFlags,
            HAS_PREVERB,
        ) == 0
    {
        if has_morphflag(&raw mut (*wantend).gs_morphflags as *mut MorphFlags, IS_DERIV)
            == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if has_morphflag(
        &raw mut (*haveend).gs_morphflags as *mut MorphFlags,
        NOT_IN_COMPOSITION,
    ) != 0
        && has_morphflag(
            &raw mut (*wantend).gs_morphflags as *mut MorphFlags,
            HAS_PREVERB,
        ) != 0
    {
        if has_morphflag(&raw mut (*wantend).gs_morphflags as *mut MorphFlags, IS_DERIV)
            == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn AddNewEnd(
    mut gstrings: *mut gk_string,
    mut newgstr: *mut gk_string,
    mut sofar: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut rval: ::core::ffi::c_int = 0;
    Xstrncpy(
        &raw mut (*gstrings
            .offset(sofar as isize)
            .offset(1 as ::core::ffi::c_int as isize))
            .gs_gkstring as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    if sofar == 0 as ::core::ffi::c_int {
        *gstrings = *newgstr;
        return 0;
    }
    i = sofar;
    while i >= 1 as ::core::ffi::c_int {
        rval = CompGkString(
            gstrings.offset(i as isize).offset(-(1 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_void,
            newgstr as *const ::core::ffi::c_void,
        );
        if rval > 0 as ::core::ffi::c_int {
            *gstrings.offset(i as isize) = *newgstr;
            return 0;
        }
        *gstrings.offset(i as isize) = *gstrings
            .offset(i as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        i -= 1;
    }
    *gstrings = *newgstr;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn setwendstr(
    mut wendstr: *mut ::core::ffi::c_char,
    mut str: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = str;
    let mut s: *mut ::core::ffi::c_char = wendstr;
    *wendstr = 0 as ::core::ffi::c_char;
    while *p != 0 {
        let fresh2 = p;
        p = p.offset(1);
        let fresh3 = s;
        s = s.offset(1);
        *fresh3 = *fresh2;
    }
    if *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int == HARDSHORT
    {
        s = s.offset(-1);
        *s = 0 as ::core::ffi::c_char;
    }
    if *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int == '-' as i32
        && p > str
    {
        start_match = 1 as ::core::ffi::c_int;
        s = s.offset(-1);
    } else {
        start_match = 0 as ::core::ffi::c_int;
    }
    *s = 0 as ::core::ffi::c_char;
    stripacc(wendstr);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn endstrcmp(
    mut wendstr: *mut ::core::ffi::c_char,
    mut haveendstr: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    let mut hp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut wlen: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if start_match != 0 {
        wlen = Xstrlen(wendstr);
        hp = haveendstr;
        sp = &raw mut tmp as *mut ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < wlen {
            if (if 0 as ::core::ffi::c_int != 0 {
                isalpha(*hp as ::core::ffi::c_int)
            } else {
                ((*hp as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
            }) == 0
                && *wendstr.offset(i as isize) as ::core::ffi::c_int
                    != *hp as ::core::ffi::c_int
            {
                hp = hp.offset(1);
            } else {
                let fresh0 = hp;
                hp = hp.offset(1);
                let fresh1 = sp;
                sp = sp.offset(1);
                *fresh1 = *fresh0;
                i += 1;
            }
        }
        *sp = 0 as ::core::ffi::c_char;
        haveendstr = &raw mut tmp as *mut ::core::ffi::c_char;
    }
    return dictstrcmp(wendstr, haveendstr);
}
#[no_mangle]
pub unsafe extern "C" fn noaccstrcmp(
    mut wendstr: *mut ::core::ffi::c_char,
    mut hendstr: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp1: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tmp2: [::core::ffi::c_char; 1024] = [0; 1024];
    strcpy(&raw mut tmp1 as *mut ::core::ffi::c_char, wendstr);
    stripquantacc(&raw mut tmp1 as *mut ::core::ffi::c_char);
    strcpy(&raw mut tmp2 as *mut ::core::ffi::c_char, hendstr);
    stripquantacc(&raw mut tmp2 as *mut ::core::ffi::c_char);
    return strcmp(
        &raw mut tmp1 as *mut ::core::ffi::c_char,
        &raw mut tmp2 as *mut ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn has_quantacc(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if has_quant(s) != 0 {
        rval += 0o10 as ::core::ffi::c_int;
    }
    if hasaccent(s) != 0 {
        rval += 0o20 as ::core::ffi::c_int;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn stripquantacc(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    stripquant(s);
    stripacc(s);
    return 0;
}

unsafe extern "C" fn ProcEndRecord(
    mut s: *mut ::core::ffi::c_char,
    mut gkend: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut BlnkGkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    BlnkGkword = CreatGkword(1 as ::core::ffi::c_int);
    s = GetEndString(s, gkend);
    ScanAsciiKeys(s, BlnkGkword, gkend, ::core::ptr::null_mut::<gk_string>());
    FreeGkword(BlnkGkword);
    return 0;
}

unsafe extern "C" fn GetEndString(
    mut s: *mut ::core::ffi::c_char,
    mut gkend: *mut gk_string,
) -> *mut ::core::ffi::c_char {
    let mut tmp: [::core::ffi::c_char; 128] = [0; 128];
    let mut a: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    a = &raw mut tmp as *mut ::core::ffi::c_char;
    while __isspace(*s as ::core::ffi::c_int) != 0 {
        s = s.offset(1);
    }
    if *s == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    while __isspace(*s as ::core::ffi::c_int) == 0 && *s as ::core::ffi::c_int != 0 {
        let fresh4 = s;
        s = s.offset(1);
        let fresh5 = a;
        a = a.offset(1);
        *fresh5 = *fresh4;
    }
    *a = 0 as ::core::ffi::c_char;
    while __isspace(*s as ::core::ffi::c_int) != 0 {
        s = s.offset(1);
    }
    if *s == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    Xstrncpy(
        &raw mut (*gkend).gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut tmp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    return s;
}

unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
