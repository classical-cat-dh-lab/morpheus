use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn NameOfTense(vf: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfMood(vf: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfVoice(vf: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfPerson(vf: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfNumber(vf: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfGender(af: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfCase(af: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfDegree(af: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfDialect(di: Dialect) -> *mut ::core::ffi::c_char;
    fn NameOfStemtype(st: Stemtype) -> *mut ::core::ffi::c_char;
    fn NameOfDerivtype(st: Derivtype) -> *mut ::core::ffi::c_char;
    fn DialectNames(
        _: Dialect,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn DomainNames(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn GeogRegionNames(
        _: GeogRegion,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn MorphNames(
        _: *mut MorphFlags,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn NameOfDomain(_: Stemtype) -> *mut ::core::ffi::c_char;
    fn dictstrcmp(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn is_blank(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn xFree(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn morph_port_unavailable(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
pub const ATTIC: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const PARADIGM: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const ALL_DIAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DECL1: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const DECL2: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const DECL3: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const DECL4: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const DECL5: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const ADJSTEM: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const NOUNSTEM: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const VERBSTEM: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
pub const STEMTYPE: ::core::ffi::c_int = VERBSTEM | DECL1 | DECL2 | DECL3 | DECL4 | DECL5
    | NOUNSTEM | ADJSTEM;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ITERATIVE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const UNAUGMENTED: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const COMMENT_CHAR: ::core::ffi::c_int = '#' as i32;
#[no_mangle]
pub unsafe extern "C" fn CreatGkString(mut num: ::core::ffi::c_int) -> *mut gk_string {
    let mut tmpgstring: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    tmpgstring = calloc(num as size_t, ::core::mem::size_of::<gk_string>())
        as *mut gk_string;
    if tmpgstring.is_null() {
        fprintf(
            stderr,
            b"Out of memory for %ld bytes to create %d gstrings\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (num as usize).wrapping_mul(::core::mem::size_of::<gk_string>() as usize),
            num,
        );
    }
    return tmpgstring;
}
#[no_mangle]
pub unsafe extern "C" fn FreeGkString(
    mut gstring: *mut gk_string,
) -> ::core::ffi::c_int {
    if gstring.is_null() {
        fprintf(
            stderr,
            b"hey! asked to free NULL gstring \n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0;
    }
    xFree(
        gstring as *mut ::core::ffi::c_char,
        b"FreeGkString\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn CreatGkAnal(mut num: ::core::ffi::c_int) -> *mut gk_analysis {
    let mut tmpanal: *mut gk_analysis = ::core::ptr::null_mut::<gk_analysis>();
    tmpanal = calloc(num as size_t, ::core::mem::size_of::<gk_analysis>())
        as *mut gk_analysis;
    return tmpanal;
}
#[no_mangle]
pub unsafe extern "C" fn FreeGkAnal(mut gkanal: *mut gk_analysis) -> ::core::ffi::c_int {
    xFree(
        gkanal as *mut ::core::ffi::c_char,
        b"freegkanal\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn CreatGkword(mut num: ::core::ffi::c_int) -> *mut gk_word {
    let mut tmpgword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    tmpgword = calloc(num as size_t, ::core::mem::size_of::<gk_word>()) as *mut gk_word;
    if tmpgword.is_null() {
        fprintf(
            stderr,
            b"Could not allocate %d gwords\n\0" as *const u8
                as *const ::core::ffi::c_char,
            num,
        );
        return NULL as *mut gk_word;
    }
    return tmpgword;
}
static mut BlnkGstr: gk_string = gk_string {
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
#[no_mangle]
pub unsafe extern "C" fn ClearGkstring(mut gstr: *mut gk_string) -> ::core::ffi::c_int {
    let mut gstring: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    *gstr = BlnkGstr;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn FreeGkword(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    if Gkword.is_null() {
        fprintf(
            stderr,
            b"hey! asked to free NULL gkword \n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0;
    }
    if (*Gkword).gw_totanal != 0 && !(*Gkword).gw_analysis.is_null() {
        FreeGkAnal((*Gkword).gw_analysis);
    }
    if !(*Gkword).st_oddkeys.is_null() {
        free((*Gkword).st_oddkeys as *mut ::core::ffi::c_void);
    }
    xFree(
        Gkword as *mut ::core::ffi::c_char,
        b"freegkword\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn CpGkAnal(
    mut Gkword1: *mut gk_word,
    mut Gkword2: *mut gk_word,
) -> ::core::ffi::c_int {
    (*Gkword1).gw_totanal = (*Gkword2).gw_totanal;
    (*Gkword1).gw_analysis = (*Gkword2).gw_analysis;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn CompGkString(
    mut gstr1: *const ::core::ffi::c_void,
    mut gstr2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut persnum1: ::core::ffi::c_int = 0;
    let mut persnum2: ::core::ffi::c_int = 0;
    let mut f1: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    let mut f2: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    let mut n1: ::core::ffi::c_int = 0;
    let mut n2: ::core::ffi::c_int = 0;
    let mut s1: Stemtype = 0;
    let mut s2: Stemtype = 0;
    let mut d1: Dialect = 0;
    let mut d2: Dialect = 0;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    f1 = (*(gstr1 as *mut gk_string)).gs_forminfo;
    f2 = (*(gstr2 as *mut gk_string)).gs_forminfo;
    s1 = (*(gstr1 as *mut gk_string)).gs_steminfo & STEMTYPE as Stemtype;
    s2 = (*(gstr2 as *mut gk_string)).gs_steminfo & STEMTYPE as Stemtype;
    if s1 != s2 {
        rval = s1.wrapping_sub(s2) as ::core::ffi::c_int;
    } else if has_morphflag(
        &raw mut (*(gstr1 as *mut gk_string)).gs_morphflags as *mut MorphFlags,
        ITERATIVE,
    )
        != has_morphflag(
            &raw mut (*(gstr2 as *mut gk_string)).gs_morphflags as *mut MorphFlags,
            ITERATIVE,
        )
    {
        rval = has_morphflag(
            &raw mut (*(gstr1 as *mut gk_string)).gs_morphflags as *mut MorphFlags,
            ITERATIVE,
        )
            - has_morphflag(
                &raw mut (*(gstr2 as *mut gk_string)).gs_morphflags as *mut MorphFlags,
                ITERATIVE,
            );
    } else if f1.f_tense() as ::core::ffi::c_int != f2.f_tense() as ::core::ffi::c_int {
        rval = f1.f_tense() as ::core::ffi::c_int - f2.f_tense() as ::core::ffi::c_int;
    } else if f1.f_voice() as ::core::ffi::c_int != f2.f_voice() as ::core::ffi::c_int {
        rval = f1.f_voice() as ::core::ffi::c_int - f2.f_voice() as ::core::ffi::c_int;
    } else if f1.f_mood() as ::core::ffi::c_int != f2.f_mood() as ::core::ffi::c_int {
        rval = f1.f_mood() as ::core::ffi::c_int - f2.f_mood() as ::core::ffi::c_int;
    } else if f1.f_degree() as ::core::ffi::c_int != f2.f_degree() as ::core::ffi::c_int
    {
        rval = f1.f_degree() as ::core::ffi::c_int - f2.f_degree() as ::core::ffi::c_int;
    } else {
        if f1.f_case() as ::core::ffi::c_int != 0
            && f2.f_case() as ::core::ffi::c_int != 0
        {
            let mut c1: ::core::ffi::c_int = 0;
            let mut c2: ::core::ffi::c_int = 0;
            c1 = 1000 as ::core::ffi::c_int * f1.f_number() as ::core::ffi::c_int
                + 10 as ::core::ffi::c_int
                    * low_bit_of(f1.f_case() as ::core::ffi::c_int)
                + low_bit_of(f1.f_gender() as ::core::ffi::c_int);
            c2 = 1000 as ::core::ffi::c_int * f2.f_number() as ::core::ffi::c_int
                + 10 as ::core::ffi::c_int
                    * low_bit_of(f2.f_case() as ::core::ffi::c_int)
                + low_bit_of(f2.f_gender() as ::core::ffi::c_int);
            if c1 != c2 {
                rval = c1 - c2;
                current_block = 6158185872388761614;
            } else {
                current_block = 2668756484064249700;
            }
        } else {
            current_block = 2668756484064249700;
        }
        match current_block {
            6158185872388761614 => {}
            _ => {
                if f1.f_gender() as ::core::ffi::c_int != 0
                    && f2.f_gender() as ::core::ffi::c_int != 0
                {
                    if low_bit_of(f1.f_gender() as ::core::ffi::c_int)
                        != low_bit_of(f2.f_gender() as ::core::ffi::c_int)
                    {
                        rval = low_bit_of(f1.f_gender() as ::core::ffi::c_int)
                            - low_bit_of(f2.f_gender() as ::core::ffi::c_int);
                        current_block = 6158185872388761614;
                    } else {
                        current_block = 16203760046146113240;
                    }
                } else {
                    current_block = 16203760046146113240;
                }
                match current_block {
                    6158185872388761614 => {}
                    _ => {
                        if has_morphflag(
                            &raw mut (*(gstr1 as *mut gk_string)).gs_morphflags
                                as *mut MorphFlags,
                            UNAUGMENTED,
                        )
                            != has_morphflag(
                                &raw mut (*(gstr2 as *mut gk_string)).gs_morphflags
                                    as *mut MorphFlags,
                                UNAUGMENTED,
                            )
                        {
                            rval = has_morphflag(
                                &raw mut (*(gstr1 as *mut gk_string)).gs_morphflags
                                    as *mut MorphFlags,
                                UNAUGMENTED,
                            )
                                - has_morphflag(
                                    &raw mut (*(gstr2 as *mut gk_string)).gs_morphflags
                                        as *mut MorphFlags,
                                    UNAUGMENTED,
                                );
                        } else {
                            if f1.f_person() as ::core::ffi::c_int != 0
                                && f2.f_person() as ::core::ffi::c_int != 0
                            {
                                persnum1 = 10 as ::core::ffi::c_int
                                    * (*(gstr1 as *mut gk_string)).gs_forminfo.f_number()
                                        as ::core::ffi::c_int
                                    + (*(gstr1 as *mut gk_string)).gs_forminfo.f_person()
                                        as ::core::ffi::c_int;
                                persnum2 = 10 as ::core::ffi::c_int
                                    * (*(gstr2 as *mut gk_string)).gs_forminfo.f_number()
                                        as ::core::ffi::c_int
                                    + (*(gstr2 as *mut gk_string)).gs_forminfo.f_person()
                                        as ::core::ffi::c_int;
                                if persnum1 != persnum2 {
                                    rval = persnum1 - persnum2;
                                    current_block = 6158185872388761614;
                                } else {
                                    current_block = 2891135413264362348;
                                }
                            } else {
                                current_block = 2891135413264362348;
                            }
                            match current_block {
                                6158185872388761614 => {}
                                _ => {
                                    n1 = low_bit_of(f1.f_number() as ::core::ffi::c_int);
                                    n2 = low_bit_of(f2.f_number() as ::core::ffi::c_int);
                                    if f1.f_number() as ::core::ffi::c_int != 0
                                        && f2.f_number() as ::core::ffi::c_int != 0
                                    {
                                        n1 = low_bit_of(f1.f_number() as ::core::ffi::c_int);
                                        n2 = low_bit_of(f2.f_number() as ::core::ffi::c_int);
                                        if n1 != n2 {
                                            rval = n2 - n1;
                                            current_block = 6158185872388761614;
                                        } else {
                                            current_block = 14832935472441733737;
                                        }
                                    } else {
                                        current_block = 14832935472441733737;
                                    }
                                    match current_block {
                                        6158185872388761614 => {}
                                        _ => {
                                            d1 = ((*(gstr1 as *mut gk_string)).gs_dialect
                                                as ::core::ffi::c_int & PARADIGM) as Dialect;
                                            d2 = ((*(gstr2 as *mut gk_string)).gs_dialect
                                                as ::core::ffi::c_int & PARADIGM) as Dialect;
                                            if d1 as ::core::ffi::c_int != d2 as ::core::ffi::c_int {
                                                rval = d2 as ::core::ffi::c_int - d1 as ::core::ffi::c_int;
                                            } else {
                                                d1 = ((*(gstr1 as *mut gk_string)).gs_dialect
                                                    as ::core::ffi::c_int & ATTIC) as Dialect;
                                                d2 = ((*(gstr2 as *mut gk_string)).gs_dialect
                                                    as ::core::ffi::c_int & ATTIC) as Dialect;
                                                if d1 as ::core::ffi::c_int != d2 as ::core::ffi::c_int {
                                                    rval = d2 as ::core::ffi::c_int - d1 as ::core::ffi::c_int;
                                                } else if (*(gstr1 as *mut gk_string)).gs_dialect
                                                    as ::core::ffi::c_int
                                                    != (*(gstr2 as *mut gk_string)).gs_dialect
                                                        as ::core::ffi::c_int
                                                {
                                                    rval = (*(gstr1 as *mut gk_string)).gs_dialect
                                                        as ::core::ffi::c_int
                                                        - (*(gstr2 as *mut gk_string)).gs_dialect
                                                            as ::core::ffi::c_int;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if rval == 0 {
        let mut p1: ::core::ffi::c_int = 0;
        let mut p2: ::core::ffi::c_int = 0;
        p1 = has_morphflag(
            &raw mut (*(gstr1 as *mut gk_string)).gs_morphflags as *mut MorphFlags,
            PARADIGM,
        );
        p2 = has_morphflag(
            &raw mut (*(gstr2 as *mut gk_string)).gs_morphflags as *mut MorphFlags,
            PARADIGM,
        );
        if p1 != p2 {
            return p2 - p1;
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn CompGkForms(
    mut gkform1: *mut gk_word,
    mut gkform2: *mut gk_word,
) -> ::core::ffi::c_int {
    let mut Gstr1: gk_string = gk_string {
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
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rval = CompGkString(
        &raw mut (*gkform1).gs_endstring as *const ::core::ffi::c_void,
        &raw mut (*gkform2).gs_endstring as *const ::core::ffi::c_void,
    );
    if rval != 0 {
        return rval;
    }
    rval = CompGkString(
        &raw mut (*gkform1).gs_stem as *const ::core::ffi::c_void,
        &raw mut (*gkform2).gs_stem as *const ::core::ffi::c_void,
    );
    if rval != 0 {
        return rval;
    }
    rval = CompGkString(
        &raw mut (*gkform2).gs_aug1 as *const ::core::ffi::c_void,
        &raw mut (*gkform1).gs_aug1 as *const ::core::ffi::c_void,
    );
    if rval != 0 {
        return rval;
    }
    rval = CompGkString(
        &raw mut (*gkform1).gs_preverb as *const ::core::ffi::c_void,
        &raw mut (*gkform2).gs_preverb as *const ::core::ffi::c_void,
    );
    if rval != 0 {
        return rval;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn low_bit_of(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut mask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i
        < ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int
            * 8 as ::core::ffi::c_int
    {
        mask += i;
        if n & mask != 0 {
            return n & mask;
        }
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn CompByDictStr(
    mut gstr1: *const ::core::ffi::c_void,
    mut gstr2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return dictstrcmp(
        &raw mut (*(gstr1 as *mut gk_string)).gs_gkstring as *mut ::core::ffi::c_char,
        &raw mut (*(gstr2 as *mut gk_string)).gs_gkstring as *mut ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RevCompByStr(
    mut gstr1: *mut gk_string,
    mut gstr2: *mut gk_string,
) -> ::core::ffi::c_int {
    return CompByDictStr(
        gstr1 as *const ::core::ffi::c_void,
        gstr2 as *const ::core::ffi::c_void,
    ) * -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn PrntGkStrings(
    mut gstr: *mut gk_string,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    while (*gstr).gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
        PrntGkStr(gstr, f);
        gstr = gstr.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntGkParadigm(
    mut gstr: *mut gk_string,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut stemtype: Stemtype = 0;
    let mut tense: ::core::ffi::c_int = 0;
    let mut mood: ::core::ffi::c_int = 0;
    let mut voice: ::core::ffi::c_int = 0;
    let mut winfo: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    voice = 0 as ::core::ffi::c_int;
    mood = voice;
    tense = mood;
    while (*gstr).gs_gkstring[0 as ::core::ffi::c_int as usize] != 0 {
        winfo = (*gstr).gs_forminfo;
        if stemtype != (*gstr).gs_steminfo {
            voice = 0 as ::core::ffi::c_int;
            mood = voice;
            tense = mood;
            stemtype = (*gstr).gs_steminfo;
            PrntStemtype((*gstr).gs_steminfo, f);
            fprintf(f, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if tense != winfo.f_tense() as ::core::ffi::c_int
            || voice != winfo.f_voice() as ::core::ffi::c_int
            || mood != winfo.f_mood() as ::core::ffi::c_int
        {
            tense = winfo.f_tense() as ::core::ffi::c_int;
            mood = winfo.f_mood() as ::core::ffi::c_int;
            voice = winfo.f_voice() as ::core::ffi::c_int;
            fprintf(f, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            PrntParadigmInfo(winfo, f);
            fprintf(f, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
        PrntGkStr(gstr, f);
        gstr = gstr.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntGkStr(
    mut gstr: *mut gk_string,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    fprintf(
        f,
        b"%s \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
    );
    PrntGkFlags(gstr, f);
    fprintf(f, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntGkFlags(
    mut gstr: *mut gk_string,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    PrntVerbInfo((*gstr).gs_forminfo, f);
    PrntAdjInfo((*gstr).gs_forminfo, f);
    PrntStemtype((*gstr).gs_steminfo, f);
    if (*gstr).gs_dialect as ::core::ffi::c_int != ALL_DIAL {
        PrntDialect((*gstr).gs_dialect, f);
    }
    PrntMorphFlags(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, f);
    PrntDomains(&raw mut (*gstr).st_domains as *mut ::core::ffi::c_char, f);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntDomains(
    mut doms: *mut ::core::ffi::c_char,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = doms;
    while *p != 0 {
        fprintf(
            f,
            b"%s \0" as *const u8 as *const ::core::ffi::c_char,
            NameOfDomain(*p as Stemtype),
        );
        p = p.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntMorphFlags(
    mut mf: *mut MorphFlags,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    MorphNames(
        mf,
        &raw mut buf as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    fprintf(
        f,
        b"%s \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut buf as *mut ::core::ffi::c_char,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntVerbInfo(
    mut vf: word_form,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut paradigm: [::core::ffi::c_char; 1024] = [0; 1024];
    paradigm[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    AddParadigmInfo(
        &raw mut paradigm as *mut ::core::ffi::c_char,
        vf,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    AddPersNumInfo(
        &raw mut paradigm as *mut ::core::ffi::c_char,
        vf,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut paradigm as *mut ::core::ffi::c_char,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntParadigmInfo(
    mut vf: word_form,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut paradigm: [::core::ffi::c_char; 1024] = [0; 1024];
    paradigm[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    AddParadigmInfo(
        &raw mut paradigm as *mut ::core::ffi::c_char,
        vf,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut paradigm as *mut ::core::ffi::c_char,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn AddParadigmInfo(
    mut s: *mut ::core::ffi::c_char,
    mut vf: word_form,
    mut dels: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = NameOfTense(vf);
    if *p != 0 {
        strcat(s, dels);
        strcat(s, p);
    }
    p = NameOfMood(vf);
    if *p != 0 {
        strcat(s, dels);
        strcat(s, p);
    }
    p = NameOfVoice(vf);
    if *p != 0 {
        strcat(s, dels);
        strcat(s, p);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn AddPersNumInfo(
    mut s: *mut ::core::ffi::c_char,
    mut vf: word_form,
    mut dels: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = NameOfPerson(vf);
    if *p != 0 {
        strcat(s, dels);
        strcat(s, p);
    }
    p = NameOfNumber(vf);
    if *p != 0 {
        strcat(s, dels);
        strcat(s, p);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntPersNumInfo(
    mut vf: word_form,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    fprintf(f, b"%s \0" as *const u8 as *const ::core::ffi::c_char, NameOfPerson(vf));
    fprintf(f, b"%s \0" as *const u8 as *const ::core::ffi::c_char, NameOfNumber(vf));
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntAdjInfo(
    mut af: word_form,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut adjbuf: [::core::ffi::c_char; 60] = [0; 60];
    adjbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    AddAdjInfo(
        &raw mut adjbuf as *mut ::core::ffi::c_char,
        af,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut adjbuf as *mut ::core::ffi::c_char,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn AddAdjInfo(
    mut s: *mut ::core::ffi::c_char,
    mut vf: word_form,
    mut dels: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = NameOfGender(vf);
    if *p != 0 {
        strcat(s, dels);
        strcat(s, p);
    }
    p = NameOfCase(vf);
    if *p != 0 {
        strcat(s, dels);
        strcat(s, p);
    }
    p = NameOfDegree(vf);
    if *p != 0 {
        strcat(s, dels);
        strcat(s, p);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntStemtype(
    mut st: Stemtype,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    fprintf(f, b"%s \0" as *const u8 as *const ::core::ffi::c_char, NameOfStemtype(st));
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntDialect(
    mut di: Dialect,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut mask: Dialect = 1 as Dialect;
    let mut dialbuf: [::core::ffi::c_char; 60] = [0; 60];
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    morph_port_unavailable(
        b"morphlib/gkstring.c:527:AddDialect\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"%s \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
    );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn AddDialect(
    mut di: Dialect,
    mut dialb: *mut ::core::ffi::c_char,
    mut dels: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut mask: Dialect = 1 as Dialect;
    i = 0 as ::core::ffi::c_int;
    while i
        < ::core::mem::size_of::<Dialect>() as ::core::ffi::c_int
            * 8 as ::core::ffi::c_int
    {
        s = NameOfDialect(
            (di as ::core::ffi::c_int & mask as ::core::ffi::c_int) as Dialect,
        );
        if !s.is_null() {
            if *s != 0 {
                if *dialb != 0 {
                    strcat(dialb, dels);
                }
                strcat(dialb, s);
            }
        }
        mask = ((mask as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as Dialect;
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn AndDialect(mut Dial1: Dialect, mut Dial2: Dialect) -> Dialect {
    if Dial1 == 0 && Dial2 == 0 {
        return 0 as Dialect
    } else if Dial1 as ::core::ffi::c_int != 0 && Dial2 as ::core::ffi::c_int != 0 {
        Dial1 = (Dial1 as ::core::ffi::c_int & Dial2 as ::core::ffi::c_int) as Dialect;
        if Dial1 == 0 {
            return -(1 as ::core::ffi::c_int) as Dialect
        } else {
            return Dial1
        }
    } else if Dial1 == 0 && Dial2 as ::core::ffi::c_int != 0 {
        return Dial2
    } else if Dial1 as ::core::ffi::c_int != 0 && Dial2 == 0 {
        return Dial1
    } else {
        return 0 as ::core::ffi::c_int as Dialect
    };
}
static mut gkCompare: Option<unsafe extern "C" fn() -> ::core::ffi::c_int> = None;
#[no_mangle]
pub unsafe extern "C" fn xInsertGstr(
    mut oldgstr: *mut gk_string,
    mut newgstr: *mut gk_string,
    mut len: ::core::ffi::c_int,
    mut compare: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    mut backwards: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut news: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut olds: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut i: ::core::ffi::c_int = 0;
    gkCompare = compare;
    news = &raw mut (*newgstr).gs_gkstring as *mut ::core::ffi::c_char;
    if len == 0 as ::core::ffi::c_int {
        *oldgstr = *newgstr;
    } else {
        i = len;
        while i > 0 as ::core::ffi::c_int {
            olds = &raw mut (*oldgstr
                .offset(i as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)))
                .gs_gkstring as *mut ::core::ffi::c_char;
            *oldgstr.offset(i as isize) = *oldgstr
                .offset(i as isize)
                .offset(-(1 as ::core::ffi::c_int as isize));
            if backwards == NO {
                if ::core::mem::transmute::<
                    _,
                    unsafe extern "C" fn(_, _) -> ::core::ffi::c_int,
                >(
                    Some(gkCompare.expect("non-null function pointer"))
                        .expect("non-null function pointer"),
                )(olds, news) <= 0 as ::core::ffi::c_int
                {
                    break;
                }
            } else if ::core::mem::transmute::<
                _,
                unsafe extern "C" fn(_, _) -> ::core::ffi::c_int,
            >(
                Some(gkCompare.expect("non-null function pointer"))
                    .expect("non-null function pointer"),
            )(olds, news) > 0 as ::core::ffi::c_int
            {
                break;
            }
            i -= 1;
        }
        *oldgstr.offset(i as isize) = *newgstr;
    }
    len += 1;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn GetTableLine(
    mut s: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    while !fgets(s, len, f).is_null() {
        if is_blank(s) != 0 {
            continue;
        }
        if *s as ::core::ffi::c_int == COMMENT_CHAR {
            continue;
        }
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn eq_forminfo(
    mut f1: word_form,
    mut f2: word_form,
) -> ::core::ffi::c_int {
    if f1.f_voice() as ::core::ffi::c_int != f2.f_voice() as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if f1.f_mood() as ::core::ffi::c_int != f2.f_mood() as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if f1.f_tense() as ::core::ffi::c_int != f2.f_tense() as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if f1.f_person() as ::core::ffi::c_int != f2.f_person() as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if f1.f_number() as ::core::ffi::c_int != f2.f_number() as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if f1.f_degree() as ::core::ffi::c_int != f2.f_degree() as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if f1.f_gender() as ::core::ffi::c_int != f2.f_gender() as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if f1.f_case() as ::core::ffi::c_int != f2.f_case() as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SprintGkFlags(
    mut gstr: *mut gk_string,
    mut buf: *mut ::core::ffi::c_char,
    mut dels: *mut ::core::ffi::c_char,
    mut pretty: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dialbuf: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wf: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    wf = (*gstr).gs_forminfo;
    s = NameOfStemtype((*gstr).gs_steminfo);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, NameOfStemtype((*gstr).gs_steminfo));
    }
    s = NameOfDerivtype((*gstr).gs_derivtype);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, NameOfDerivtype((*gstr).gs_derivtype));
    }
    s = NameOfTense(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfMood(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfVoice(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfGender(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfCase(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfDegree(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfPerson(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfNumber(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, NameOfNumber(wf));
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    DialectNames((*gstr).gs_dialect, &raw mut dialbuf as *mut ::core::ffi::c_char, dels);
    if dialbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        || *dels as ::core::ffi::c_int == '\t' as i32
    {
        strcat(buf, dels);
    }
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    GeogRegionNames(
        (*gstr).gs_geogregion,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
        dels,
    );
    if dialbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        || *dels as ::core::ffi::c_int == '\t' as i32
    {
        strcat(buf, dels);
    }
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    DomainNames(
        &raw mut (*gstr).st_domains as *mut ::core::ffi::c_char,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
        dels,
    );
    if dialbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        || *dels as ::core::ffi::c_int == '\t' as i32
    {
        strcat(buf, dels);
    }
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    MorphNames(
        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
        dels,
        pretty,
    );
    if dialbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        || *dels as ::core::ffi::c_int == '\t' as i32
    {
        strcat(buf, dels);
    }
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn DbaseFormat(
    mut gstr: *mut gk_string,
    mut buf: *mut ::core::ffi::c_char,
    mut tabstr: *mut ::core::ffi::c_char,
    mut pretty: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dialbuf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wf: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    wf = (*gstr).gs_forminfo;
    s = NameOfStemtype((*gstr).gs_steminfo);
    if *s != 0 {
        strcat(buf, NameOfStemtype((*gstr).gs_steminfo));
    }
    strcat(buf, tabstr);
    s = NameOfDerivtype((*gstr).gs_derivtype);
    if *s != 0 {
        strcat(buf, NameOfDerivtype((*gstr).gs_derivtype));
    }
    strcat(buf, tabstr);
    AddParadigmInfo(
        buf,
        wf,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    strcat(buf, tabstr);
    AddPersNumInfo(
        buf,
        wf,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    strcat(buf, tabstr);
    AddAdjInfo(
        buf,
        wf,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    DialectNames(
        (*gstr).gs_dialect,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, tabstr);
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    MorphNames(
        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        pretty,
    );
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, tabstr);
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    return 0;
}
