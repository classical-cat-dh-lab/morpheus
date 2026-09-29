use ::c2rust_bitfields;
extern "C" {
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Xstrncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn addbreath(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn getbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn next_cons_rough(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn nextpreverb(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn set_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
pub const IONIC: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const AEOLIC: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
pub const NOBREATH: ::core::ffi::c_int = ' ' as i32;
pub const DISSIMILATION: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const APOCOPE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const POETIC: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const ELIDE_PREVERB: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const RAW_PREVERB: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const UNASP_PREVERB: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const D_PREVB: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CombPbStem(
    mut curpb: *mut ::core::ffi::c_char,
    mut restofs: *mut ::core::ffi::c_char,
    mut dial: Dialect,
    mut pbflags: *mut MorphFlags,
) -> ::core::ffi::c_int {
    if *restofs == 0 {
        return 1 as ::core::ffi::c_int;
    }
    if cur_lang() == LATIN || cur_lang() == ITALIAN {
        return CombPbStemL(curpb, restofs, dial, pbflags)
    } else {
        return CombPbStemG(curpb, restofs, dial, pbflags)
    };
}
#[no_mangle]
pub unsafe extern "C" fn CombPbStemL(
    mut curpb: *mut ::core::ffi::c_char,
    mut restofs: *mut ::core::ffi::c_char,
    mut dial: Dialect,
    mut pbflags: *mut MorphFlags,
) -> ::core::ffi::c_int {
    let mut lastc: ::core::ffi::c_int = 0;
    let mut lastc2: ::core::ffi::c_int = 0;
    let mut lastc3: ::core::ffi::c_int = 0;
    let mut curbreath: ::core::ffi::c_int = 0;
    let mut workrest: [::core::ffi::c_char; 60] = [0; 60];
    let mut noaccpb: [::core::ffi::c_char; 60] = [0; 60];
    if strcmp(b"circum\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0
        && *restofs as ::core::ffi::c_int == 'i' as i32
    {
        add_morphflag(pbflags, RAW_PREVERB);
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(b"red\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0
        || strcmp(b"prod\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0
    {
        add_morphflag(pbflags, D_PREVB);
        return 1 as ::core::ffi::c_int;
    }
    lastc = *curpb
        .offset(strlen(curpb) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
    if strcmp(b"amb\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0
        && strchr(
                b"aeiou\0" as *const u8 as *const ::core::ffi::c_char,
                *restofs as ::core::ffi::c_int,
            )
            .is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(b"sub\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0
        || strcmp(b"ob\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0
    {
        match *restofs as ::core::ffi::c_int {
            99 | 102 | 103 | 112 => {
                add_morphflag(pbflags, RAW_PREVERB);
            }
            _ => {}
        }
    }
    if strcmp(b"ex\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0
        || strcmp(b"ec\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0
    {
        if strchr(
                b"aeioucpqtf\0" as *const u8 as *const ::core::ffi::c_char,
                *restofs as ::core::ffi::c_int,
            )
            .is_null()
        {
            add_morphflag(pbflags, RAW_PREVERB);
        }
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(b"trans\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0 {
        match *restofs as ::core::ffi::c_int {
            105 | 106 | 100 | 108 | 109 | 110 | 115 => {
                add_morphflag(pbflags, RAW_PREVERB);
            }
            _ => {}
        }
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(b"dis\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0 {
        if strncmp(
            b"di\0" as *const u8 as *const ::core::ffi::c_char,
            restofs,
            2 as size_t,
        ) == 0
        {
            return 1 as ::core::ffi::c_int;
        }
        match *restofs as ::core::ffi::c_int {
            98 | 100 | 103 | 108 | 109 | 110 | 114 | 118 | 102 => {
                add_morphflag(pbflags, RAW_PREVERB);
            }
            _ => {}
        }
    }
    if strcmp(b"sub\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0 {
        match *restofs as ::core::ffi::c_int {
            109 | 114 => {
                add_morphflag(pbflags, RAW_PREVERB);
            }
            _ => {}
        }
    }
    if strcmp(b"in\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0 {
        match *restofs as ::core::ffi::c_int {
            98 | 108 | 112 | 109 => {
                add_morphflag(pbflags, RAW_PREVERB);
            }
            _ => {}
        }
    }
    if strcmp(b"con\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0 {
        match *restofs as ::core::ffi::c_int {
            98 | 112 | 109 | 114 | 108 | 110 => {
                add_morphflag(pbflags, RAW_PREVERB);
            }
            _ => {}
        }
    }
    if strcmp(b"ad\0" as *const u8 as *const ::core::ffi::c_char, curpb) == 0 {
        match *restofs as ::core::ffi::c_int {
            99 | 102 | 103 | 108 | 110 | 112 | 114 | 115 | 116 => {
                add_morphflag(pbflags, RAW_PREVERB);
            }
            _ => {}
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CombPbStemG(
    mut curpb: *mut ::core::ffi::c_char,
    mut restofs: *mut ::core::ffi::c_char,
    mut dial: Dialect,
    mut pbflags: *mut MorphFlags,
) -> ::core::ffi::c_int {
    let mut lastc: ::core::ffi::c_int = 0;
    let mut lastc2: ::core::ffi::c_int = 0;
    let mut lastc3: ::core::ffi::c_int = 0;
    let mut curbreath: ::core::ffi::c_int = 0;
    let mut workrest: [::core::ffi::c_char; 60] = [0; 60];
    let mut noaccpb: [::core::ffi::c_char; 60] = [0; 60];
    if has_morphflag(pbflags, DISSIMILATION) != 0 && next_cons_rough(restofs) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    Xstrncpy(&raw mut noaccpb as *mut ::core::ffi::c_char, curpb, MAXWORDSIZE as size_t);
    stripacc(&raw mut noaccpb as *mut ::core::ffi::c_char);
    stripbreath(&raw mut noaccpb as *mut ::core::ffi::c_char);
    lastc = *(&raw mut noaccpb as *mut ::core::ffi::c_char)
        .offset(Xstrlen(&raw mut noaccpb as *mut ::core::ffi::c_char) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
    if Xstrlen(&raw mut noaccpb as *mut ::core::ffi::c_char) > 1 as ::core::ffi::c_int {
        lastc2 = *(&raw mut noaccpb as *mut ::core::ffi::c_char)
            .offset(Xstrlen(&raw mut noaccpb as *mut ::core::ffi::c_char) as isize)
            .offset(-(2 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
    } else {
        lastc2 = 0 as ::core::ffi::c_int;
    }
    if Xstrlen(&raw mut noaccpb as *mut ::core::ffi::c_char) > 2 as ::core::ffi::c_int {
        lastc3 = *(&raw mut noaccpb as *mut ::core::ffi::c_char)
            .offset(Xstrlen(&raw mut noaccpb as *mut ::core::ffi::c_char) as isize)
            .offset(-(3 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
    } else {
        lastc3 = 0 as ::core::ffi::c_int;
    }
    if (if 0 as ::core::ffi::c_int != 0 {
        isalpha(lastc)
    } else {
        ((lastc as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    }) != 0 && lastc != 'j' as i32 && lastc != 'v' as i32 && lastc != 'J' as i32
        && lastc != 'V' as i32
        && !(lastc == 'a' as i32 || lastc == 'e' as i32 || lastc == 'i' as i32
            || lastc == 'o' as i32 || lastc == 'u' as i32 || lastc == 'A' as i32
            || lastc == 'E' as i32 || lastc == 'I' as i32 || lastc == 'O' as i32
            || lastc == 'U' as i32
            || (lastc == 'h' as i32 || lastc == 'w' as i32 || lastc == 'H' as i32
                || lastc == 'W' as i32)) && lastc != 's' as i32 && lastc2 != 'e' as i32
        && ((if 0 as ::core::ffi::c_int != 0 {
            isalpha(*restofs as ::core::ffi::c_int)
        } else {
            ((*restofs as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) != 0 && *restofs as ::core::ffi::c_int != 'j' as i32
            && *restofs as ::core::ffi::c_int != 'v' as i32
            && *restofs as ::core::ffi::c_int != 'J' as i32
            && *restofs as ::core::ffi::c_int != 'V' as i32
            && !(*restofs as ::core::ffi::c_int == 'a' as i32
                || *restofs as ::core::ffi::c_int == 'e' as i32
                || *restofs as ::core::ffi::c_int == 'i' as i32
                || *restofs as ::core::ffi::c_int == 'o' as i32
                || *restofs as ::core::ffi::c_int == 'u' as i32
                || *restofs as ::core::ffi::c_int == 'A' as i32
                || *restofs as ::core::ffi::c_int == 'E' as i32
                || *restofs as ::core::ffi::c_int == 'I' as i32
                || *restofs as ::core::ffi::c_int == 'O' as i32
                || *restofs as ::core::ffi::c_int == 'U' as i32
                || (*restofs as ::core::ffi::c_int == 'h' as i32
                    || *restofs as ::core::ffi::c_int == 'w' as i32
                    || *restofs as ::core::ffi::c_int == 'H' as i32
                    || *restofs as ::core::ffi::c_int == 'W' as i32)))
    {
        if lastc2 != 'u' as i32 && lastc3 != 's' as i32 {
            add_morphflag(pbflags, APOCOPE);
        }
    }
    if lastc == 'q' as i32 || lastc == 'f' as i32 {
        if !(*restofs as ::core::ffi::c_int == 'a' as i32
            || *restofs as ::core::ffi::c_int == 'e' as i32
            || *restofs as ::core::ffi::c_int == 'i' as i32
            || *restofs as ::core::ffi::c_int == 'o' as i32
            || *restofs as ::core::ffi::c_int == 'u' as i32
            || *restofs as ::core::ffi::c_int == 'A' as i32
            || *restofs as ::core::ffi::c_int == 'E' as i32
            || *restofs as ::core::ffi::c_int == 'I' as i32
            || *restofs as ::core::ffi::c_int == 'O' as i32
            || *restofs as ::core::ffi::c_int == 'U' as i32
            || (*restofs as ::core::ffi::c_int == 'h' as i32
                || *restofs as ::core::ffi::c_int == 'w' as i32
                || *restofs as ::core::ffi::c_int == 'H' as i32
                || *restofs as ::core::ffi::c_int == 'W' as i32))
        {
            return 0 as ::core::ffi::c_int;
        }
        if *curpb
            .offset(Xstrlen(curpb) as isize)
            .offset(-(2 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'm' as i32 && lastc == 'f' as i32
        {
            add_morphflag(pbflags, ELIDE_PREVERB);
            return 1 as ::core::ffi::c_int;
        }
        curbreath = getbreath(restofs);
        if has_morphflag(pbflags, UNASP_PREVERB) != 0 {
            zap_morphflag(pbflags, UNASP_PREVERB);
        }
        if curbreath == NOBREATH {
            addbreath(restofs, ROUGHBR);
        } else if curbreath == SMOOTHBR {
            return 0 as ::core::ffi::c_int
        }
    } else if (lastc == 't' as i32 || lastc == 'p' as i32)
        && (*restofs as ::core::ffi::c_int == 'a' as i32
            || *restofs as ::core::ffi::c_int == 'e' as i32
            || *restofs as ::core::ffi::c_int == 'i' as i32
            || *restofs as ::core::ffi::c_int == 'o' as i32
            || *restofs as ::core::ffi::c_int == 'u' as i32
            || *restofs as ::core::ffi::c_int == 'A' as i32
            || *restofs as ::core::ffi::c_int == 'E' as i32
            || *restofs as ::core::ffi::c_int == 'I' as i32
            || *restofs as ::core::ffi::c_int == 'O' as i32
            || *restofs as ::core::ffi::c_int == 'U' as i32
            || (*restofs as ::core::ffi::c_int == 'h' as i32
                || *restofs as ::core::ffi::c_int == 'w' as i32
                || *restofs as ::core::ffi::c_int == 'H' as i32
                || *restofs as ::core::ffi::c_int == 'W' as i32))
    {
        curbreath = getbreath(restofs);
        if AndDialect(dial, (AEOLIC | IONIC) as Dialect) as ::core::ffi::c_int
            >= 0 as ::core::ffi::c_int && (curbreath == ROUGHBR || curbreath == NOBREATH)
        {
            add_morphflag(pbflags, UNASP_PREVERB);
            return 1 as ::core::ffi::c_int;
        } else if curbreath == NOBREATH {
            return 1 as ::core::ffi::c_int
        } else if curbreath == ROUGHBR {
            return 0 as ::core::ffi::c_int
        }
    } else if lastc == 'c' as i32 {
        if !(*restofs as ::core::ffi::c_int == 'a' as i32
            || *restofs as ::core::ffi::c_int == 'e' as i32
            || *restofs as ::core::ffi::c_int == 'i' as i32
            || *restofs as ::core::ffi::c_int == 'o' as i32
            || *restofs as ::core::ffi::c_int == 'u' as i32
            || *restofs as ::core::ffi::c_int == 'A' as i32
            || *restofs as ::core::ffi::c_int == 'E' as i32
            || *restofs as ::core::ffi::c_int == 'I' as i32
            || *restofs as ::core::ffi::c_int == 'O' as i32
            || *restofs as ::core::ffi::c_int == 'U' as i32
            || (*restofs as ::core::ffi::c_int == 'h' as i32
                || *restofs as ::core::ffi::c_int == 'w' as i32
                || *restofs as ::core::ffi::c_int == 'H' as i32
                || *restofs as ::core::ffi::c_int == 'W' as i32))
        {
            return 0 as ::core::ffi::c_int;
        }
    } else if lastc == 'u' as i32 {
        if *restofs as ::core::ffi::c_int != 's' as i32
            && *restofs as ::core::ffi::c_int != 'z' as i32
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *restofs as ::core::ffi::c_int == 'k' as i32
        || *restofs as ::core::ffi::c_int == 'g' as i32
        || *restofs as ::core::ffi::c_int == 'x' as i32
        || *restofs as ::core::ffi::c_int == 'c' as i32
        || *restofs as ::core::ffi::c_int == 'c' as i32
        || (*restofs as ::core::ffi::c_int == 'p' as i32
            || *restofs as ::core::ffi::c_int == 'b' as i32
            || *restofs as ::core::ffi::c_int == 'f' as i32
            || *restofs as ::core::ffi::c_int == 'y' as i32)
        || *restofs as ::core::ffi::c_int == 'l' as i32
        || *restofs as ::core::ffi::c_int == 'r' as i32
        || *restofs as ::core::ffi::c_int == 'm' as i32
    {
        if lastc == 'n' as i32 {
            add_morphflag(pbflags, RAW_PREVERB);
            add_morphflag(pbflags, POETIC);
            return 1 as ::core::ffi::c_int;
        }
    }
    if lastc == 'i' as i32 && lastc2 == 'd' as i32 {
        if !(*restofs as ::core::ffi::c_int == 'a' as i32
            || *restofs as ::core::ffi::c_int == 'e' as i32
            || *restofs as ::core::ffi::c_int == 'i' as i32
            || *restofs as ::core::ffi::c_int == 'o' as i32
            || *restofs as ::core::ffi::c_int == 'u' as i32
            || *restofs as ::core::ffi::c_int == 'A' as i32
            || *restofs as ::core::ffi::c_int == 'E' as i32
            || *restofs as ::core::ffi::c_int == 'I' as i32
            || *restofs as ::core::ffi::c_int == 'O' as i32
            || *restofs as ::core::ffi::c_int == 'U' as i32
            || (*restofs as ::core::ffi::c_int == 'h' as i32
                || *restofs as ::core::ffi::c_int == 'w' as i32
                || *restofs as ::core::ffi::c_int == 'H' as i32
                || *restofs as ::core::ffi::c_int == 'W' as i32))
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if lastc == 'g' as i32
        && !(*restofs as ::core::ffi::c_int == 'k' as i32
            || *restofs as ::core::ffi::c_int == 'g' as i32
            || *restofs as ::core::ffi::c_int == 'x' as i32
            || *restofs as ::core::ffi::c_int == 'c' as i32)
    {
        return 0 as ::core::ffi::c_int;
    }
    if lastc2 == 'u' as i32 || lastc2 == 'e' as i32 && lastc3 != 'p' as i32 {
        match lastc {
            103 => {
                if !(*restofs as ::core::ffi::c_int == 'k' as i32
                    || *restofs as ::core::ffi::c_int == 'g' as i32
                    || *restofs as ::core::ffi::c_int == 'x' as i32
                    || *restofs as ::core::ffi::c_int == 'c' as i32)
                    && *restofs as ::core::ffi::c_int != 'c' as i32
                {
                    return 0 as ::core::ffi::c_int;
                }
                return 1 as ::core::ffi::c_int;
            }
            109 => {
                if !(*restofs as ::core::ffi::c_int == 'p' as i32
                    || *restofs as ::core::ffi::c_int == 'b' as i32
                    || *restofs as ::core::ffi::c_int == 'f' as i32)
                    && *restofs as ::core::ffi::c_int != 'y' as i32
                    && *restofs as ::core::ffi::c_int != 'm' as i32
                {
                    return 0 as ::core::ffi::c_int;
                }
                return 1 as ::core::ffi::c_int;
            }
            108 => {
                if *restofs as ::core::ffi::c_int != 'l' as i32 {
                    return 0 as ::core::ffi::c_int;
                }
                return 1 as ::core::ffi::c_int;
            }
            114 => {
                if lastc2 == 'u' as i32 && *restofs as ::core::ffi::c_int != 'r' as i32 {
                    return 0 as ::core::ffi::c_int;
                }
                if *restofs as ::core::ffi::c_int != 'r' as i32
                    && !(*restofs as ::core::ffi::c_int == 'a' as i32
                        || *restofs as ::core::ffi::c_int == 'e' as i32
                        || *restofs as ::core::ffi::c_int == 'i' as i32
                        || *restofs as ::core::ffi::c_int == 'o' as i32
                        || *restofs as ::core::ffi::c_int == 'u' as i32
                        || *restofs as ::core::ffi::c_int == 'A' as i32
                        || *restofs as ::core::ffi::c_int == 'E' as i32
                        || *restofs as ::core::ffi::c_int == 'I' as i32
                        || *restofs as ::core::ffi::c_int == 'O' as i32
                        || *restofs as ::core::ffi::c_int == 'U' as i32
                        || (*restofs as ::core::ffi::c_int == 'h' as i32
                            || *restofs as ::core::ffi::c_int == 'w' as i32
                            || *restofs as ::core::ffi::c_int == 'H' as i32
                            || *restofs as ::core::ffi::c_int == 'W' as i32))
                {
                    return 0 as ::core::ffi::c_int;
                }
                return 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
    }
    if (lastc == 'a' as i32 || lastc == 'e' as i32 || lastc == 'i' as i32
        || lastc == 'o' as i32 || lastc == 'u' as i32 || lastc == 'A' as i32
        || lastc == 'E' as i32 || lastc == 'I' as i32 || lastc == 'O' as i32
        || lastc == 'U' as i32
        || (lastc == 'h' as i32 || lastc == 'w' as i32 || lastc == 'H' as i32
            || lastc == 'W' as i32))
        && (*restofs as ::core::ffi::c_int == 'a' as i32
            || *restofs as ::core::ffi::c_int == 'e' as i32
            || *restofs as ::core::ffi::c_int == 'i' as i32
            || *restofs as ::core::ffi::c_int == 'o' as i32
            || *restofs as ::core::ffi::c_int == 'u' as i32
            || *restofs as ::core::ffi::c_int == 'A' as i32
            || *restofs as ::core::ffi::c_int == 'E' as i32
            || *restofs as ::core::ffi::c_int == 'I' as i32
            || *restofs as ::core::ffi::c_int == 'O' as i32
            || *restofs as ::core::ffi::c_int == 'U' as i32
            || (*restofs as ::core::ffi::c_int == 'h' as i32
                || *restofs as ::core::ffi::c_int == 'w' as i32
                || *restofs as ::core::ffi::c_int == 'H' as i32
                || *restofs as ::core::ffi::c_int == 'W' as i32))
        && Xstrncmp(
            (&raw mut noaccpb as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut noaccpb as *mut ::core::ffi::c_char) as isize)
                .offset(-(2 as ::core::ffi::c_int as isize)),
            b"di\0" as *const u8 as *const ::core::ffi::c_char,
            2 as size_t,
        ) != 0
    {
        add_morphflag(pbflags, RAW_PREVERB);
        if lastc3 != 'p' as i32 && lastc2 != 'r' as i32 {
            add_morphflag(pbflags, POETIC);
            return 1 as ::core::ffi::c_int;
        }
    }
    if (lastc == 'a' as i32 || lastc == 'e' as i32 || lastc == 'i' as i32
        || lastc == 'o' as i32 || lastc == 'u' as i32 || lastc == 'A' as i32
        || lastc == 'E' as i32 || lastc == 'I' as i32 || lastc == 'O' as i32
        || lastc == 'U' as i32
        || (lastc == 'h' as i32 || lastc == 'w' as i32 || lastc == 'H' as i32
            || lastc == 'W' as i32)) && *restofs as ::core::ffi::c_int == 'r' as i32
    {
        if *restofs.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 'r' as i32
        {
            let mut tbuf: [::core::ffi::c_char; 60] = [0; 60];
            add_morphflag(pbflags, RAW_PREVERB);
            add_morphflag(pbflags, POETIC);
            Xstrncpy(
                &raw mut tbuf as *mut ::core::ffi::c_char,
                b"r(\0" as *const u8 as *const ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            Xstrncpy(
                (&raw mut tbuf as *mut ::core::ffi::c_char)
                    .offset(2 as ::core::ffi::c_int as isize),
                restofs.offset(1 as ::core::ffi::c_int as isize),
                MAXWORDSIZE as size_t,
            );
            Xstrncpy(
                restofs,
                &raw mut tbuf as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
        } else {
            *restofs.offset(1 as ::core::ffi::c_int as isize) = '(' as i32
                as ::core::ffi::c_char;
        }
        return 1 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_preverb(
    mut rawpb: *mut ::core::ffi::c_char,
    mut fullpb: *mut ::core::ffi::c_char,
    mut gstr: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0;
    if *rawpb == 0 {
        return 0 as ::core::ffi::c_int;
    }
    *fullpb = 0 as ::core::ffi::c_char;
    set_morphflag(
        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        0 as ::core::ffi::c_int,
    );
    rval = exp_preverb(rawpb, fullpb, gstr);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn exp_preverb(
    mut rawpb: *mut ::core::ffi::c_char,
    mut fullpb: *mut ::core::ffi::c_char,
    mut gstr: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut savepb: [::core::ffi::c_char; 60] = [0; 60];
    let mut tmppb: [::core::ffi::c_char; 60] = [0; 60];
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *rawpb == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !strchr(rawpb, ',' as i32).is_null() {
        Xstrncpy(fullpb, rawpb, MAXWORDSIZE as size_t);
        return 1 as ::core::ffi::c_int;
    }
    if fullpb.is_null() {
        fullpb = &raw mut tmppb as *mut ::core::ffi::c_char;
    }
    Xstrncpy(&raw mut savepb as *mut ::core::ffi::c_char, rawpb, MAXWORDSIZE as size_t);
    stripacc(&raw mut savepb as *mut ::core::ffi::c_char);
    rval = exp_prevb2(&raw mut savepb as *mut ::core::ffi::c_char, fullpb, gstr);
    return rval;
}
static mut recursion_level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn exp_prevb2(
    mut str: *mut ::core::ffi::c_char,
    mut fullpb: *mut ::core::ffi::c_char,
    mut gstr: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut startpb: [::core::ffi::c_char; 60] = [0; 60];
    let mut startfpb: [::core::ffi::c_char; 60] = [0; 60];
    let mut preverb: [::core::ffi::c_char; 60] = [0; 60];
    let mut pblemma: [::core::ffi::c_char; 60] = [0; 60];
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
    let mut SaveGstr: gk_string = gk_string {
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
    set_morphflag(
        &raw mut Gstr.gs_morphflags as *mut MorphFlags,
        0 as ::core::ffi::c_int,
    );
    strncpy(&raw mut startpb as *mut ::core::ffi::c_char, str, MAXWORDSIZE as size_t);
    strncpy(
        &raw mut startfpb as *mut ::core::ffi::c_char,
        fullpb,
        MAXWORDSIZE as size_t,
    );
    SaveGstr = *gstr;
    preverb[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    if *str == 0 {
        return 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    loop {
        let mut rval: ::core::ffi::c_int = 0;
        rval = nextpreverb(
            str,
            &raw mut preverb as *mut ::core::ffi::c_char,
            &raw mut pblemma as *mut ::core::ffi::c_char,
            gstr,
        );
        if rval == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if *fullpb != 0 {
            strncat(
                fullpb,
                b",\0" as *const u8 as *const ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
        }
        strncat(
            fullpb,
            &raw mut pblemma as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        if CombPbStem(
            &raw mut preverb as *mut ::core::ffi::c_char,
            str,
            0 as ::core::ffi::c_int as Dialect,
            &raw mut Gstr.gs_morphflags as *mut MorphFlags,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        recursion_level += 1;
        rval = exp_prevb2(str, fullpb, gstr);
        if rval != 0 {
            if has_morphflag(
                &raw mut Gstr.gs_morphflags as *mut MorphFlags,
                UNASP_PREVERB,
            ) != 0
            {
                add_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    UNASP_PREVERB,
                );
            }
            return rval;
        }
        strncpy(
            str,
            &raw mut startpb as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        strncpy(
            fullpb,
            &raw mut startfpb as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        *gstr = SaveGstr;
        i += 1;
    };
}
