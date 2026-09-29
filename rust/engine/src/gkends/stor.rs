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
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
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
    fn cinsert(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn CompByDictStr(
        gstr1: *const ::core::ffi::c_void,
        gstr2: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn CompGkString(
        gstr1: *const ::core::ffi::c_void,
        gstr2: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn SprintGkFlags(
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn WriteEnding(
        _: *mut FILE,
        _: *mut gk_string,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn is_diphth(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> bool_0;
    fn set_endheader(_: *mut FILE, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn stripchar(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn stripzeroend(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn zap_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub const PPARTMASK: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const DIAERESIS: ::core::ffi::c_int = '+' as i32;
pub const INDECLFORM: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const IS_DERIV: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
static mut StoreGstr: *mut gk_string = ::core::ptr::null::<gk_string>()
    as *mut gk_string;
pub const MAXENDINGS: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
static mut cur_endcnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut maxstring: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn InitGstrMem() -> ::core::ffi::c_int {
    if StoreGstr.is_null() {
        StoreGstr = CreatGkString(MAXENDINGS + 1 as ::core::ffi::c_int);
        if StoreGstr.is_null() {
            return 0 as ::core::ffi::c_int;
        }
    }
    cur_endcnt = 0 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AddNewGstr(mut gstr: *mut gk_string) -> ::core::ffi::c_int {
    let mut news: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut d: Dialect = 0;
    news = &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char;
    if cur_endcnt >= MAXENDINGS {
        fprintf(
            stderr,
            b"Hey! you only have space for %d endings!\n\0" as *const u8
                as *const ::core::ffi::c_char,
            MAXENDINGS,
        );
        fprintf(
            stderr,
            b"Change to variable MAXENDINGS to reflect the actual number you want!\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if strlen(news).wrapping_add(1 as size_t) >= maxstring as size_t {
        maxstring = strlen(news).wrapping_add(1 as size_t) as ::core::ffi::c_int;
    }
    if *news as ::core::ffi::c_int != '*' as i32 {
        stripzeroend(news);
    }
    hyphtodiaer(news);
    stripchar(news, '!' as i32);
    *StoreGstr.offset(cur_endcnt as isize) = *gstr;
    cur_endcnt += 1;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn ResetGstrBuf() -> ::core::ffi::c_int {
    cur_endcnt = 0 as ::core::ffi::c_int;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn PrntNewGstrings(
    mut f: *mut FILE,
    mut compiled_flag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut line: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut res: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut deriv: ::core::ffi::c_int = 0;
    let mut indeclform: ::core::ffi::c_int = 0;
    indeclform = has_morphflag(
        &raw mut (*StoreGstr).gs_morphflags as *mut MorphFlags,
        INDECLFORM,
    );
    deriv = has_morphflag(
        &raw mut (*StoreGstr).gs_morphflags as *mut MorphFlags,
        IS_DERIV,
    );
    if compiled_flag != 0 {
        set_endheader(f, maxstring);
        if deriv == 0 {
            qsort(
                StoreGstr as *mut ::core::ffi::c_void,
                cur_endcnt as size_t,
                ::core::mem::size_of::<gk_string>(),
                Some(
                    CompByDictStr
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
        }
    } else if deriv == 0 {
        qsort(
            StoreGstr as *mut ::core::ffi::c_void,
            cur_endcnt as size_t,
            ::core::mem::size_of::<gk_string>(),
            Some(
                CompGkString
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < cur_endcnt {
        tmp[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        line[0 as ::core::ffi::c_int as usize] = tmp[0 as ::core::ffi::c_int as usize];
        if compiled_flag != 0 {
            if deriv != 0 {
                zap_morphflag(
                    &raw mut (*StoreGstr.offset(i as isize)).gs_morphflags
                        as *mut MorphFlags,
                    IS_DERIV,
                );
            }
            rval = WriteEnding(f, StoreGstr.offset(i as isize), maxstring);
            if deriv != 0 {
                add_morphflag(
                    &raw mut (*StoreGstr.offset(i as isize)).gs_morphflags
                        as *mut MorphFlags,
                    IS_DERIV,
                );
            }
        } else {
            LPrntGstr(StoreGstr.offset(i as isize), f);
        }
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn LPrntGstr(
    mut gstr: *mut gk_string,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 256] = [0; 256];
    let mut res: [::core::ffi::c_char; 256] = [0; 256];
    let mut line: [::core::ffi::c_char; 256] = [0; 256];
    let mut indecl: ::core::ffi::c_int = 0;
    res[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    line[0 as ::core::ffi::c_int as usize] = res[0 as ::core::ffi::c_int as usize];
    tmp[0 as ::core::ffi::c_int as usize] = line[0 as ::core::ffi::c_int as usize];
    indecl = has_morphflag(
        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        INDECLFORM,
    );
    if has_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, IS_DERIV) != 0
        || indecl != 0
    {
        zap_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, IS_DERIV);
        zap_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, INDECLFORM);
        SprintGkFlags(
            gstr,
            &raw mut tmp as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        sprintf(
            &raw mut line as *mut ::core::ffi::c_char,
            b"%s  %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
        );
        if indecl != 0 {
            if (*gstr).gs_steminfo & 0o70000000 as ::core::ffi::c_int as Stemtype != 0 {
                fprintf(f, b":vb:\0" as *const u8 as *const ::core::ffi::c_char);
            } else {
                fprintf(f, b":wd:\0" as *const u8 as *const ::core::ffi::c_char);
            }
        }
        fprintf(
            f,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut line as *mut ::core::ffi::c_char,
        );
    } else {
        SprintGkFlags(
            gstr,
            &raw mut tmp as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        if cur_lang() == LATIN || cur_lang() == ITALIAN {
            sprintf(
                &raw mut line as *mut ::core::ffi::c_char,
                b"%s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
                &raw mut tmp as *mut ::core::ffi::c_char,
            );
        } else {
            sprintf(
                &raw mut line as *mut ::core::ffi::c_char,
                b"<G>%s</G>%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
                &raw mut tmp as *mut ::core::ffi::c_char,
            );
        }
        fprintf(
            f,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut line as *mut ::core::ffi::c_char,
        );
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn new_parad(
    mut gstr1: *mut gk_string,
    mut gstr2: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut wf1: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    let mut wf2: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    wf1 = (*gstr1).gs_forminfo;
    wf2 = (*gstr2).gs_forminfo;
    if wf1.f_tense() as ::core::ffi::c_int != wf2.f_tense() as ::core::ffi::c_int
        || wf1.f_mood() as ::core::ffi::c_int != wf2.f_mood() as ::core::ffi::c_int
        || wf1.f_voice() as ::core::ffi::c_int != wf2.f_voice() as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    if wf1.f_gender() as ::core::ffi::c_int != wf2.f_gender() as ::core::ffi::c_int
        && (*gstr1).gs_dialect as ::core::ffi::c_int
            != (*gstr2).gs_dialect as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn hyphtodiaer(
    mut news: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = news;
    while *s != 0 {
        if *s as ::core::ffi::c_int == '-' as i32 {
            strcpy(s, s.offset(1 as ::core::ffi::c_int as isize));
            if is_diphth(s, news) != 0 {
                cinsert(DIAERESIS, s.offset(1 as ::core::ffi::c_int as isize));
            }
        }
        s = s.offset(1);
    }
    return 0;
}
