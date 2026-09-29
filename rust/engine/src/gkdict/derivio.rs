use ::c2rust_bitfields;
extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
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
    fn ClearGkstring(_: *mut gk_string) -> ::core::ffi::c_int;
    fn CompatKeys(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn FreeGkword(_: *mut gk_word) -> ::core::ffi::c_int;
    fn SprintGkFlags(
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn chckderiv(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn chckdvend(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn has_diaeresis(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn nextkey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn redupit2(
        _: *mut gk_word,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn simpleredupit(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn un_redupl(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn unaugment(
        _: *mut ::core::ffi::c_char,
        _: *mut *mut gk_string,
        _: *mut *mut gk_string,
        _: ::core::ffi::c_int,
        _: Dialect,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
pub const ALL_DIAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PPARTMASK: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const PP_PF: ::core::ffi::c_int = 0o40000000 as ::core::ffi::c_int;
pub const PP_PP: ::core::ffi::c_int = 0o50000000 as ::core::ffi::c_int;
pub const PP_FP: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SYLL_AUGMENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const R_E_I_ALPHA: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const REDUPL: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const IS_DERIV: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
#[no_mangle]
pub static mut checkedsuffs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut checkedderivs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut realderivs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn checkforderiv(
    mut stemstr: *mut ::core::ffi::c_char,
    mut stemkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rval2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    extern "C" {
        #[link_name = "is_substring"]
        fn is_substring_0(
            _: *mut ::core::ffi::c_char,
            _: *mut ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
    }
    let mut stemkeys2: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut oldsuffs: ::core::ffi::c_int = checkedsuffs;
    stemkeys2[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    checkedderivs += 1;
    rval = checkforderiv2(
        stemstr,
        stemkeys,
        b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    rval2 = checkforredupderiv(stemstr, &raw mut stemkeys2 as *mut ::core::ffi::c_char);
    if rval2 != 0 {
        if *stemkeys != 0 {
            strcat(stemkeys, b" \0" as *const u8 as *const ::core::ffi::c_char);
        }
        strcat(stemkeys, &raw mut stemkeys2 as *mut ::core::ffi::c_char);
    }
    add_deriv_cache(stemstr, stemkeys);
    return rval + rval2;
}
#[no_mangle]
pub unsafe extern "C" fn checkforredupderiv(
    mut stemstr: *mut ::core::ffi::c_char,
    mut stemkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut noredup: [::core::ffi::c_char; 60] = [0; 60];
    if (if 0 as ::core::ffi::c_int != 0 {
        isalpha(*stemstr as ::core::ffi::c_int)
    } else {
        ((*stemstr as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    }) != 0 && *stemstr as ::core::ffi::c_int != 'j' as i32
        && *stemstr as ::core::ffi::c_int != 'v' as i32
        && *stemstr as ::core::ffi::c_int != 'J' as i32
        && *stemstr as ::core::ffi::c_int != 'V' as i32
        && !(*stemstr as ::core::ffi::c_int == 'a' as i32
            || *stemstr as ::core::ffi::c_int == 'e' as i32
            || *stemstr as ::core::ffi::c_int == 'i' as i32
            || *stemstr as ::core::ffi::c_int == 'o' as i32
            || *stemstr as ::core::ffi::c_int == 'u' as i32
            || *stemstr as ::core::ffi::c_int == 'A' as i32
            || *stemstr as ::core::ffi::c_int == 'E' as i32
            || *stemstr as ::core::ffi::c_int == 'I' as i32
            || *stemstr as ::core::ffi::c_int == 'O' as i32
            || *stemstr as ::core::ffi::c_int == 'U' as i32
            || (*stemstr as ::core::ffi::c_int == 'h' as i32
                || *stemstr as ::core::ffi::c_int == 'w' as i32
                || *stemstr as ::core::ffi::c_int == 'H' as i32
                || *stemstr as ::core::ffi::c_int == 'W' as i32)) && cur_lang() != LATIN
        && cur_lang() != ITALIAN
    {
        if un_redupl(stemstr, &raw mut noredup as *mut ::core::ffi::c_char, 'e' as i32)
            == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        return checkforderiv2(
            &raw mut noredup as *mut ::core::ffi::c_char,
            stemkeys,
            b"redupl\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
    }
    return checkaugredup(stemstr, stemkeys);
}
pub const MAXREDUPLS: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
static mut init_stor: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut tstemtab: [*mut gk_string; 4] = [::core::ptr::null::<gk_string>()
    as *mut gk_string; 4];
static mut tqstemtab: [*mut gk_string; 4] = [::core::ptr::null::<gk_string>()
    as *mut gk_string; 4];
#[no_mangle]
pub unsafe extern "C" fn checkaugredup(
    mut stemstr: *mut ::core::ffi::c_char,
    mut stemkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut hits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut poss_redupls: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut possno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut had_redupl: [::core::ffi::c_char; 60] = [0; 60];
    let mut tmpkeys: [::core::ffi::c_char; 1024] = [0; 1024];
    if init_stor == 0 {
        init_stor = 1 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < MAXREDUPLS {
            tstemtab[i as usize] = CreatGkString(1 as ::core::ffi::c_int);
            tqstemtab[i as usize] = CreatGkString(1 as ::core::ffi::c_int);
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAXREDUPLS {
        ClearGkstring(tstemtab[i as usize]);
        ClearGkstring(tqstemtab[i as usize]);
        i += 1;
    }
    if Xstrncmp(stemstr, b"e)\0" as *const u8 as *const ::core::ffi::c_char, 2 as size_t)
        == 0
        && !((if 0 as ::core::ffi::c_int != 0 {
            isalpha(
                *stemstr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            )
        } else {
            ((*stemstr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
                | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) != 0
            && *stemstr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'j' as i32
            && *stemstr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'v' as i32
            && *stemstr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'J' as i32
            && *stemstr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'V' as i32
            && !(*stemstr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'a' as i32
                || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'e' as i32
                || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'i' as i32
                || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'o' as i32
                || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'u' as i32
                || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'A' as i32
                || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'E' as i32
                || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'I' as i32
                || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'O' as i32
                || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'U' as i32
                || (*stemstr.offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'h' as i32
                    || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'w' as i32
                    || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'H' as i32
                    || *stemstr.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'W' as i32)))
    {
        if *stemstr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 'z' as i32
            && *stemstr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'c' as i32
            && *stemstr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'y' as i32
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    poss_redupls = unaugment(
        stemstr,
        &raw mut tstemtab as *mut *mut gk_string,
        &raw mut tqstemtab as *mut *mut gk_string,
        MAXREDUPLS,
        ALL_DIAL as Dialect,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    i = 0 as ::core::ffi::c_int;
    while i < poss_redupls {
        let mut tempstem: [::core::ffi::c_char; 1024] = [0; 1024];
        tmpkeys[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        tempstem[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        SprintGkFlags(
            tstemtab[i as usize],
            &raw mut tmpkeys as *mut ::core::ffi::c_char,
            b"\t\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        Xstrncpy(
            &raw mut tempstem as *mut ::core::ffi::c_char,
            &raw mut (**(&raw mut tstemtab as *mut *mut gk_string).offset(i as isize))
                .gs_gkstring as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int
                as size_t,
        );
        if has_morphflag(
            &raw mut (**(&raw mut tstemtab as *mut *mut gk_string).offset(i as isize))
                .gs_morphflags as *mut MorphFlags,
            SYLL_AUGMENT,
        ) != 0
        {
            strcpy(
                &raw mut had_redupl as *mut ::core::ffi::c_char,
                b"syll_aug\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            strcpy(
                &raw mut had_redupl as *mut ::core::ffi::c_char,
                b"temp_aug\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if tmpkeys[0 as ::core::ffi::c_int as usize] != 0 {
            strcat(
                &raw mut had_redupl as *mut ::core::ffi::c_char,
                b" \0" as *const u8 as *const ::core::ffi::c_char,
            );
            strcat(
                &raw mut had_redupl as *mut ::core::ffi::c_char,
                &raw mut tmpkeys as *mut ::core::ffi::c_char,
            );
        }
        if checkforderiv2(
            &raw mut tempstem as *mut ::core::ffi::c_char,
            &raw mut tmpkeys as *mut ::core::ffi::c_char,
            &raw mut had_redupl as *mut ::core::ffi::c_char,
            (if (*tqstemtab[i as usize]).gs_gkstring[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int != 0
            {
                &raw mut (**(&raw mut tqstemtab as *mut *mut gk_string)
                    .offset(i as isize))
                    .gs_gkstring as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            }) as *mut ::core::ffi::c_char,
        ) != 0
        {
            if *stemkeys != 0 {
                Xstrncat(
                    stemkeys,
                    b" \0" as *const u8 as *const ::core::ffi::c_char,
                    LONGSTRING as size_t,
                );
            }
            Xstrncat(
                stemkeys,
                &raw mut tmpkeys as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            hits += 1;
        }
        i += 1;
    }
    return hits;
}
unsafe extern "C" fn checkforderiv2(
    mut stemstr: *mut ::core::ffi::c_char,
    mut stemkeys: *mut ::core::ffi::c_char,
    mut had_redupl: *mut ::core::ffi::c_char,
    mut redupstem: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut derivstr: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sofar: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut slen: ::core::ffi::c_int = 0;
    let mut derivkeys2: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut resbuf: [::core::ffi::c_char; 2048] = [0; 2048];
    ep = stemstr
        .offset(Xstrlen(stemstr) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    slen = Xstrlen(stemstr) - 1 as ::core::ffi::c_int;
    resbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    derivkeys2[0 as ::core::ffi::c_int as usize] = resbuf[0 as ::core::ffi::c_int
        as usize];
    Xstrncpy(
        &raw mut derivstr as *mut ::core::ffi::c_char,
        stemstr,
        LONGSTRING as size_t,
    );
    rval = chckdvend(
        b"*\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        stemkeys,
    );
    if chckderiv(
        &raw mut derivstr as *mut ::core::ffi::c_char,
        &raw mut derivkeys2 as *mut ::core::ffi::c_char,
    ) != 0
    {
        let mut n: ::core::ffi::c_int = 0;
        n = checkcomderivs(
            stemkeys,
            &raw mut derivstr as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut derivkeys2 as *mut ::core::ffi::c_char,
            &raw mut resbuf as *mut ::core::ffi::c_char,
            had_redupl,
            redupstem,
        );
        if n != 0 {
            sofar += 1;
        }
    }
    while ep > stemstr {
        rval = chckdvend(ep, stemkeys);
        if rval != 0 {
            derivstr[slen as usize] = 0 as ::core::ffi::c_char;
            if chckderiv(
                &raw mut derivstr as *mut ::core::ffi::c_char,
                &raw mut derivkeys2 as *mut ::core::ffi::c_char,
            ) != 0
            {
                let mut n_0: ::core::ffi::c_int = 0;
                n_0 = checkcomderivs(
                    stemkeys,
                    &raw mut derivstr as *mut ::core::ffi::c_char,
                    ep,
                    &raw mut derivkeys2 as *mut ::core::ffi::c_char,
                    &raw mut resbuf as *mut ::core::ffi::c_char,
                    had_redupl,
                    redupstem,
                );
                if n_0 != 0 {
                    sofar += 1;
                }
            }
        }
        ep = ep.offset(-1);
        slen -= 1;
    }
    if sofar != 0 {
        Xstrncpy(
            stemkeys,
            &raw mut resbuf as *mut ::core::ffi::c_char,
            LONGSTRING as size_t,
        );
    }
    return sofar;
}
#[no_mangle]
pub unsafe extern "C" fn checkcomderivs(
    mut derivs: *mut ::core::ffi::c_char,
    mut defstem: *mut ::core::ffi::c_char,
    mut suffix: *mut ::core::ffi::c_char,
    mut lemmkeys: *mut ::core::ffi::c_char,
    mut nkeys: *mut ::core::ffi::c_char,
    mut had_redupl: *mut ::core::ffi::c_char,
    mut redupstem: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut lkeybuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut curlemmkeys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    lkeybuf = malloc(LONGSTRING as size_t) as *mut ::core::ffi::c_char;
    curlemmkeys = malloc(LONGSTRING as size_t) as *mut ::core::ffi::c_char;
    Xstrncpy(curlemmkeys, lemmkeys, LONGSTRING as size_t);
    while nextkey(curlemmkeys, lkeybuf) != 0 {
        rval
            += checkcomderiv(
                derivs,
                defstem,
                suffix,
                lkeybuf,
                nkeys,
                had_redupl,
                redupstem,
            );
    }
    free(lkeybuf as *mut ::core::ffi::c_void);
    free(curlemmkeys as *mut ::core::ffi::c_void);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn checkcomderiv(
    mut derivstr: *mut ::core::ffi::c_char,
    mut defstem: *mut ::core::ffi::c_char,
    mut suffix: *mut ::core::ffi::c_char,
    mut lkeys: *mut ::core::ffi::c_char,
    mut rkeys: *mut ::core::ffi::c_char,
    mut had_redupl: *mut ::core::ffi::c_char,
    mut redupstem: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut asuffkeys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut dstemkeys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut lemma: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut tmpdstem: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut tmpsuff: *mut ::core::ffi::c_char = suffix;
    let mut markedstem: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    asuffkeys = malloc((LONGSTRING as size_t).wrapping_mul(2 as size_t))
        as *mut ::core::ffi::c_char;
    dstemkeys = malloc((LONGSTRING as size_t).wrapping_mul(2 as size_t))
        as *mut ::core::ffi::c_char;
    lemma = malloc((LONGSTRING as size_t).wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    tmpdstem = malloc((LONGSTRING as size_t).wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    Xstrncpy(asuffkeys, derivstr, (LONGSTRING * 2 as ::core::ffi::c_int) as size_t);
    Xstrncpy(tmpdstem, lkeys, LONGSTRING as size_t);
    s = tmpdstem;
    while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != ':' as i32 {
        s = s.offset(1);
    }
    if *s as ::core::ffi::c_int == ':' as i32 {
        let fresh0 = s;
        s = s.offset(1);
        *fresh0 = 0 as ::core::ffi::c_char;
    }
    Xstrncpy(lemma, s, LONGSTRING as size_t);
    if *tmpdstem.offset(0 as ::core::ffi::c_int as isize) != 0 {
        markedstem += 1;
    } else {
        Xstrncpy(tmpdstem, defstem, MAXWORDSIZE as size_t);
    }
    s = lemma;
    while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != ':' as i32 {
        s = s.offset(1);
    }
    if *s as ::core::ffi::c_int == ':' as i32 {
        let fresh1 = s;
        s = s.offset(1);
        *fresh1 = 0 as ::core::ffi::c_char;
    }
    Xstrncpy(dstemkeys, s, (LONGSTRING * 2 as ::core::ffi::c_int) as size_t);
    s = dstemkeys;
    while *s != 0 {
        if *s as ::core::ffi::c_int == ':' as i32 {
            *s = ' ' as i32 as ::core::ffi::c_char;
        }
        s = s.offset(1);
    }
    if *had_redupl as ::core::ffi::c_int != 0
        && (*tmpdstem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'a' as i32
            || *tmpdstem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'e' as i32
            || *tmpdstem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'i' as i32
            || *tmpdstem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'o' as i32
            || *tmpdstem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'u' as i32
            || *tmpdstem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'A' as i32
            || *tmpdstem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'E' as i32
            || *tmpdstem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'I' as i32
            || *tmpdstem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'O' as i32
            || *tmpdstem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'U' as i32
            || (*tmpdstem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'h' as i32
                || *tmpdstem.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'w' as i32
                || *tmpdstem.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'H' as i32
                || *tmpdstem.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == 'W' as i32)) && *redupstem == 0
    {
        rval
            += checkmultredups(
                asuffkeys,
                tmpdstem,
                dstemkeys,
                tmpsuff,
                lemma,
                lkeys,
                rkeys,
                had_redupl,
                markedstem,
            );
    } else {
        if *had_redupl != 0 {
            if *redupstem != 0 {
                markedstem += 1;
                strcpy(tmpdstem, redupstem);
                tmpsuff = b"\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
            } else {
                simpleredupit(tmpdstem, NO, 'e' as i32);
            }
        }
        rval
            += checkcomderiv2(
                asuffkeys,
                tmpdstem,
                dstemkeys,
                tmpsuff,
                lemma,
                lkeys,
                rkeys,
                had_redupl,
                markedstem,
            );
    }
    free(asuffkeys as *mut ::core::ffi::c_void);
    free(dstemkeys as *mut ::core::ffi::c_void);
    free(lemma as *mut ::core::ffi::c_void);
    free(tmpdstem as *mut ::core::ffi::c_void);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn checkmultredups(
    mut asuffkeys: *mut ::core::ffi::c_char,
    mut dstem: *mut ::core::ffi::c_char,
    mut dstemkeys: *mut ::core::ffi::c_char,
    mut suffix: *mut ::core::ffi::c_char,
    mut lemma: *mut ::core::ffi::c_char,
    mut lkeys: *mut ::core::ffi::c_char,
    mut rkeys: *mut ::core::ffi::c_char,
    mut had_redupl: *mut ::core::ffi::c_char,
    mut markedstem: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut gotredups: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut gkform: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut gstr: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut curstemkeys: [::core::ffi::c_char; 1024] = [0; 1024];
    gkform = CreatGkword(6 as ::core::ffi::c_int);
    gstr = CreatGkString(1 as ::core::ffi::c_int);
    if gkform.is_null() {
        fprintf(
            stderr,
            b"no memory for gkform in checkmultredups of [%s]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            asuffkeys,
        );
        crate::unavailable("checkmultredups: undefined original return");
    }
    Xstrncpy(
        &raw mut (*gkform).st_workword as *mut ::core::ffi::c_char,
        dstem,
        MAXWORDSIZE as size_t,
    );
    (*gstr).gs_dialect = 0 as ::core::ffi::c_int as Dialect;
    ScanAsciiKeys(dstemkeys, gkform, gstr, NULL as *mut gk_string);
    (*gkform).gs_dialect = (*gstr).gs_dialect;
    gotredups = redupit2(gkform, NO, 'e' as i32, 5 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < gotredups {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        curstemkeys[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        (*gstr).gs_dialect = (*gkform.offset(i as isize)).gs_dialect;
        SprintGkFlags(
            gstr,
            &raw mut curstemkeys as *mut ::core::ffi::c_char,
            b" \0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        p = &raw mut (*gkform.offset(i as isize)).st_workword
            as *mut ::core::ffi::c_char;
        rval
            += checkcomderiv2(
                asuffkeys,
                p,
                &raw mut curstemkeys as *mut ::core::ffi::c_char,
                suffix,
                lemma,
                lkeys,
                rkeys,
                had_redupl,
                markedstem,
            );
        i += 1;
    }
    FreeGkword(gkform);
    FreeGkString(gstr);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn checkcomderiv2(
    mut asuffkeys: *mut ::core::ffi::c_char,
    mut dstem: *mut ::core::ffi::c_char,
    mut dstemkeys: *mut ::core::ffi::c_char,
    mut suffix: *mut ::core::ffi::c_char,
    mut lemma: *mut ::core::ffi::c_char,
    mut lkeys: *mut ::core::ffi::c_char,
    mut rkeys: *mut ::core::ffi::c_char,
    mut had_redupl: *mut ::core::ffi::c_char,
    mut markedstem: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut derivsuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut tmpdsuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut stembuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut dbuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gstr: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    derivsuff = malloc(LONGSTRING as size_t) as *mut ::core::ffi::c_char;
    tmpdsuff = malloc(LONGSTRING as size_t) as *mut ::core::ffi::c_char;
    dbuf = malloc(LONGSTRING as size_t) as *mut ::core::ffi::c_char;
    stembuf = malloc(MAXWORDSIZE as size_t) as *mut ::core::ffi::c_char;
    gstr = CreatGkString(1 as ::core::ffi::c_int);
    Xstrncpy(dbuf, asuffkeys, LONGSTRING as size_t);
    while nextkey(dbuf, derivsuff) != 0 {
        *stembuf.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        s = derivsuff;
        if *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != ':' as i32 {
            let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            Xstrncpy(tmpdsuff, s, LONGSTRING as size_t);
            s = s.offset(1);
            while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != ':' as i32
            {
                s = s.offset(1);
            }
            if *s != 0 {
                s = s.offset(1);
            }
            memmove(
                derivsuff as *mut ::core::ffi::c_void,
                s as *const ::core::ffi::c_void,
                strlen(s).wrapping_add(1 as size_t),
            );
            p = derivsuff;
            while *p != 0 {
                if *p as ::core::ffi::c_int == ':' as i32 {
                    *p = ' ' as i32 as ::core::ffi::c_char;
                }
                p = p.offset(1);
            }
            p = tmpdsuff;
            while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != ':' as i32
            {
                p = p.offset(1);
            }
            if *p != 0 {
                *p = 0 as ::core::ffi::c_char;
            }
            markedstem += 1;
        } else {
            Xstrncpy(tmpdsuff, suffix, LONGSTRING as size_t);
        }
        if markedstem != 0 {
            sprintf(
                stembuf,
                b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                dstem,
                tmpdsuff,
            );
        } else {
            *stembuf.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        }
        while *s != 0 {
            if *s as ::core::ffi::c_int == ':' as i32 {
                *s = ' ' as i32 as ::core::ffi::c_char;
            }
            s = s.offset(1);
        }
        if !(DstemTakesDsuff(derivsuff, dstemkeys, gstr, dstem, tmpdsuff) != 0) {
            continue;
        }
        let mut tmp1: [::core::ffi::c_char; 2048] = [0; 2048];
        let mut tmp2: [::core::ffi::c_char; 2048] = [0; 2048];
        tmp1[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        if cur_lang() != LATIN && cur_lang() != ITALIAN {
            if *had_redupl == 0
                && ((*gstr).gs_steminfo & PPARTMASK as Stemtype == PP_PP as Stemtype
                    || (*gstr).gs_steminfo & PPARTMASK as Stemtype == PP_PF as Stemtype
                    || (*gstr).gs_steminfo & PPARTMASK as Stemtype == PP_FP as Stemtype)
            {
                continue;
            }
        }
        if *had_redupl as ::core::ffi::c_int != 0
            && !((*gstr).gs_steminfo & PPARTMASK as Stemtype == PP_PP as Stemtype
                || (*gstr).gs_steminfo & PPARTMASK as Stemtype == PP_PF as Stemtype
                || (*gstr).gs_steminfo & PPARTMASK as Stemtype == PP_FP as Stemtype)
        {
            continue;
        }
        if Xstrncmp(
            had_redupl,
            b"syll_aug\0" as *const u8 as *const ::core::ffi::c_char,
            Xstrlen(b"syll_aug\0" as *const u8 as *const ::core::ffi::c_char) as size_t,
        ) == 0
            && has_morphflag(
                &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                SYLL_AUGMENT,
            ) == 0
        {
            continue;
        }
        if *rkeys != 0 {
            Xstrncat(
                rkeys,
                b" \0" as *const u8 as *const ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
        }
        if *had_redupl != 0 {
            let mut gkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
            add_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, REDUPL);
            gkword = CreatGkword(1 as ::core::ffi::c_int);
            ScanAsciiKeys(had_redupl, gkword, gstr, NULL as *mut gk_string);
            FreeGkword(gkword);
        }
        SprintGkFlags(
            gstr,
            &raw mut tmp1 as *mut ::core::ffi::c_char,
            b":\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        sprintf(
            &raw mut tmp2 as *mut ::core::ffi::c_char,
            b"%s:%s%s\0" as *const u8 as *const ::core::ffi::c_char,
            stembuf,
            lemma,
            &raw mut tmp1 as *mut ::core::ffi::c_char,
        );
        Xstrncat(rkeys, &raw mut tmp2 as *mut ::core::ffi::c_char, LONGSTRING as size_t);
        rval += 1;
    }
    free(derivsuff as *mut ::core::ffi::c_void);
    free(tmpdsuff as *mut ::core::ffi::c_void);
    free(stembuf as *mut ::core::ffi::c_void);
    free(dbuf as *mut ::core::ffi::c_void);
    FreeGkString(gstr);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn DstemTakesDsuff(
    mut dsuffkeys: *mut ::core::ffi::c_char,
    mut dstemkeys: *mut ::core::ffi::c_char,
    mut gstr: *mut gk_string,
    mut defstem: *mut ::core::ffi::c_char,
    mut derivstr: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    checkedsuffs += 1;
    add_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, IS_DERIV);
    if has_diaeresis(derivstr) != 0 && ends_in_vowel(defstem) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    rval = CompatKeys(dsuffkeys, dstemkeys, gstr);
    zap_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, IS_DERIV);
    if rval == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if Xstrncmp(
        derivstr,
        b"a_\0" as *const u8 as *const ::core::ffi::c_char,
        2 as size_t,
    ) == 0
        && !(*defstem
            .offset(Xstrlen(defstem) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'r' as i32
            || *defstem
                .offset(Xstrlen(defstem) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'i' as i32
            || *defstem
                .offset(Xstrlen(defstem) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'e' as i32) && need_rei_alpha(dsuffkeys) != 0
    {
        rval = 0 as ::core::ffi::c_int;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn need_rei_alpha(
    mut dsuffkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut gstr: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut Gkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    gstr = CreatGkString(1 as ::core::ffi::c_int);
    Gkword = CreatGkword(1 as ::core::ffi::c_int);
    ScanAsciiKeys(dsuffkeys, Gkword, gstr, NULL as *mut gk_string);
    if has_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, R_E_I_ALPHA) != 0
    {
        rval = 1 as ::core::ffi::c_int;
    }
    FreeGkString(gstr);
    FreeGkword(Gkword);
    return rval;
}
pub const BADTRIES: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
#[no_mangle]
pub static mut cache_stems: [*mut ::core::ffi::c_char; 12] = [::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char; 12];
#[no_mangle]
pub static mut cache_keys: [*mut ::core::ffi::c_char; 12] = [::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char; 12];
static mut badinit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut badindex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn stemstr_in_cache(
    mut s: *mut ::core::ffi::c_char,
    mut stemkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < BADTRIES {
        if strcmp(s, cache_stems[i as usize]) == 0 {
            if !cache_keys[i as usize].is_null() {
                Xstrncpy(stemkeys, cache_keys[i as usize], LONGSTRING as size_t);
            } else {
                *stemkeys = 0 as ::core::ffi::c_char;
            }
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn add_deriv_cache(
    mut s: *mut ::core::ffi::c_char,
    mut keys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if badinit == 0 {
        i = 0 as ::core::ffi::c_int;
        while i < BADTRIES {
            cache_stems[i as usize] = malloc(
                (MAXWORDSIZE as size_t).wrapping_add(1 as size_t),
            ) as *mut ::core::ffi::c_char;
            *cache_stems[i as usize].offset(0 as ::core::ffi::c_int as isize) = 0
                as ::core::ffi::c_char;
            cache_keys[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
            i += 1;
        }
        badinit = 1 as ::core::ffi::c_int;
    }
    if badindex >= BADTRIES {
        badindex = 0 as ::core::ffi::c_int;
    }
    Xstrncpy(cache_stems[badindex as usize], s, MAXWORDSIZE as size_t);
    if !cache_keys[badindex as usize].is_null() {
        free(cache_keys[badindex as usize] as *mut ::core::ffi::c_void);
    }
    if *keys == 0 {
        cache_keys[badindex as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        cache_keys[badindex as usize] = malloc(
            (Xstrlen(keys) as size_t).wrapping_add(1 as size_t),
        ) as *mut ::core::ffi::c_char;
        strcpy(cache_keys[badindex as usize], keys);
    }
    badindex += 1;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn ends_in_vowel(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = s.offset(Xstrlen(s) as isize).offset(-(1 as ::core::ffi::c_int as isize));
    while p >= s
        && (if 0 as ::core::ffi::c_int != 0 {
            isalpha(*p as ::core::ffi::c_int)
        } else {
            ((*p as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) == 0
    {
        p = p.offset(-1);
    }
    return (*p as ::core::ffi::c_int == 'a' as i32
        || *p as ::core::ffi::c_int == 'e' as i32
        || *p as ::core::ffi::c_int == 'i' as i32
        || *p as ::core::ffi::c_int == 'o' as i32
        || *p as ::core::ffi::c_int == 'u' as i32
        || *p as ::core::ffi::c_int == 'A' as i32
        || *p as ::core::ffi::c_int == 'E' as i32
        || *p as ::core::ffi::c_int == 'I' as i32
        || *p as ::core::ffi::c_int == 'O' as i32
        || *p as ::core::ffi::c_int == 'U' as i32
        || (*p as ::core::ffi::c_int == 'h' as i32
            || *p as ::core::ffi::c_int == 'w' as i32
            || *p as ::core::ffi::c_int == 'H' as i32
            || *p as ::core::ffi::c_int == 'W' as i32)) as ::core::ffi::c_int;
}
