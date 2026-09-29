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
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn is_substring(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn ClearGkstring(_: *mut gk_string) -> ::core::ffi::c_int;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Xstrncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn chckstem(
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
    fn xFree(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const ALL_DIAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXAUGSTEMS: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
static mut digstem: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut tstemtab: [*mut gk_string; 12] = [::core::ptr::null::<gk_string>()
    as *mut gk_string; 12];
static mut tqstemtab: [*mut gk_string; 12] = [::core::ptr::null::<gk_string>()
    as *mut gk_string; 12];
static mut tkeytab: [*mut ::core::ffi::c_char; 12] = [::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char; 12];
static mut init_stor: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn checkstem(
    mut poss_stem: *mut ::core::ffi::c_char,
    mut endkeys: *mut ::core::ffi::c_char,
    mut stemtab: *mut *mut gk_string,
    mut keytab: *mut *mut ::core::ffi::c_char,
    mut maxstems: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut curstemkeys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut i: ::core::ffi::c_int = 0;
    let mut hits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut poss_augs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut possno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    curstemkeys = malloc((LONGSTRING as size_t).wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    if init_stor == 0 {
        init_stor = 1 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < MAXAUGSTEMS {
            tstemtab[i as usize] = CreatGkString(1 as ::core::ffi::c_int);
            tqstemtab[i as usize] = CreatGkString(1 as ::core::ffi::c_int);
            tkeytab[i as usize] = malloc(
                (LONGSTRING as size_t).wrapping_add(1 as size_t),
            ) as *mut ::core::ffi::c_char;
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAXAUGSTEMS {
        ClearGkstring(tstemtab[i as usize]);
        ClearGkstring(tqstemtab[i as usize]);
        *tkeytab[i as usize] = 0 as ::core::ffi::c_char;
        i += 1;
    }
    *curstemkeys = 0 as ::core::ffi::c_char;
    rval = stemexists(poss_stem, endkeys, curstemkeys, 0 as ::core::ffi::c_int);
    if rval != 0 {
        if *curstemkeys.offset(0 as ::core::ffi::c_int as isize) != 0 {
            Xstrncpy(
                &raw mut (**stemtab.offset(possno as isize)).gs_gkstring
                    as *mut ::core::ffi::c_char,
                poss_stem,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
            Xstrncpy(*keytab.offset(possno as isize), curstemkeys, LONGSTRING as size_t);
            *curstemkeys.offset(0 as ::core::ffi::c_int as isize) = 0
                as ::core::ffi::c_char;
            possno += 1;
            hits += 1;
        }
    }
    if !((if 0 as ::core::ffi::c_int != 0 {
        isalpha(*poss_stem as ::core::ffi::c_int)
    } else {
        ((*poss_stem as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
            .wrapping_sub('a' as i32 as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    }) != 0 && *poss_stem as ::core::ffi::c_int != 'j' as i32
        && *poss_stem as ::core::ffi::c_int != 'v' as i32
        && *poss_stem as ::core::ffi::c_int != 'J' as i32
        && *poss_stem as ::core::ffi::c_int != 'V' as i32
        && !(*poss_stem as ::core::ffi::c_int == 'a' as i32
            || *poss_stem as ::core::ffi::c_int == 'e' as i32
            || *poss_stem as ::core::ffi::c_int == 'i' as i32
            || *poss_stem as ::core::ffi::c_int == 'o' as i32
            || *poss_stem as ::core::ffi::c_int == 'u' as i32
            || *poss_stem as ::core::ffi::c_int == 'A' as i32
            || *poss_stem as ::core::ffi::c_int == 'E' as i32
            || *poss_stem as ::core::ffi::c_int == 'I' as i32
            || *poss_stem as ::core::ffi::c_int == 'O' as i32
            || *poss_stem as ::core::ffi::c_int == 'U' as i32
            || (*poss_stem as ::core::ffi::c_int == 'h' as i32
                || *poss_stem as ::core::ffi::c_int == 'w' as i32
                || *poss_stem as ::core::ffi::c_int == 'H' as i32
                || *poss_stem as ::core::ffi::c_int == 'W' as i32)))
    {
        poss_augs = unaugment(
            poss_stem,
            &raw mut tstemtab as *mut *mut gk_string,
            &raw mut tqstemtab as *mut *mut gk_string,
            MAXAUGSTEMS,
            ALL_DIAL as Dialect,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        i = 0 as ::core::ffi::c_int;
        while i < poss_augs {
            *curstemkeys = 0 as ::core::ffi::c_char;
            if stemexists(
                &raw mut (**(&raw mut tstemtab as *mut *mut gk_string)
                    .offset(i as isize))
                    .gs_gkstring as *mut ::core::ffi::c_char,
                endkeys,
                curstemkeys,
                0 as ::core::ffi::c_int,
            ) != 0
            {
                if *curstemkeys.offset(0 as ::core::ffi::c_int as isize) != 0 {
                    **stemtab.offset(hits as isize) = *tstemtab[i as usize];
                    Xstrncpy(
                        *keytab.offset(hits as isize),
                        curstemkeys,
                        LONGSTRING as size_t,
                    );
                    hits += 1;
                }
            }
            i += 1;
        }
    }
    xFree(
        curstemkeys,
        b"curstemkeys\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    curstemkeys = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return hits;
}
#[no_mangle]
pub unsafe extern "C" fn stemexists(
    mut s: *mut ::core::ffi::c_char,
    mut endkeys: *mut ::core::ffi::c_char,
    mut stemkeys: *mut ::core::ffi::c_char,
    mut is_nom: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rval = chckstem(s, stemkeys, is_nom);
    if rval == 0 && digstem != 0 {
        longeststem(s);
    }
    if rval == 0 {
        return 0 as ::core::ffi::c_int;
    }
    rval = comstemtypes(s, stemkeys, endkeys);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn comstemtypes(
    mut stem: *mut ::core::ffi::c_char,
    mut stemkeys: *mut ::core::ffi::c_char,
    mut endkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut cstemtype: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut cstem: [::core::ffi::c_char; 60] = [0; 60];
    let mut clemma: [::core::ffi::c_char; 60] = [0; 60];
    let mut stembuf: [::core::ffi::c_char; 1024] = [0; 1024];
    s = stemkeys;
    p = &raw mut tmp as *mut ::core::ffi::c_char;
    tmp[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    stembuf[0 as ::core::ffi::c_int as usize] = tmp[0 as ::core::ffi::c_int as usize];
    while *s != 0 {
        setstemvars(
            s,
            &raw mut cstem as *mut ::core::ffi::c_char,
            &raw mut clemma as *mut ::core::ffi::c_char,
            &raw mut cstemtype as *mut ::core::ffi::c_char,
            &raw mut stembuf as *mut ::core::ffi::c_char,
        );
        if wantcurstemtype(&raw mut cstemtype as *mut ::core::ffi::c_char, endkeys) != 0
        {
            if tmp[0 as ::core::ffi::c_int as usize] != 0 {
                Xstrncat(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    b" \0" as *const u8 as *const ::core::ffi::c_char,
                    LONGSTRING as size_t,
                );
            }
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut clemma as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                b":\0" as *const u8 as *const ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            if cstem[0 as ::core::ffi::c_int as usize] != 0 {
                Xstrncat(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    &raw mut cstem as *mut ::core::ffi::c_char,
                    LONGSTRING as size_t,
                );
            } else {
                Xstrncat(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    stem,
                    LONGSTRING as size_t,
                );
            }
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                b":\0" as *const u8 as *const ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut cstemtype as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                b":\0" as *const u8 as *const ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
            if *(&raw mut stembuf as *mut ::core::ffi::c_char) != 0 {
                Xstrncat(
                    &raw mut tmp as *mut ::core::ffi::c_char,
                    &raw mut stembuf as *mut ::core::ffi::c_char,
                    LONGSTRING as size_t,
                );
            }
        }
        while __isspace(*s as ::core::ffi::c_int) == 0 && *s as ::core::ffi::c_int != 0 {
            s = s.offset(1);
        }
        while __isspace(*s as ::core::ffi::c_int) != 0 {
            s = s.offset(1);
        }
    }
    Xstrncpy(stemkeys, p, LONGSTRING as size_t);
    if *stemkeys != 0 {
        return 1 as ::core::ffi::c_int
    } else {
        return 0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn wantcurstemtype(
    mut curst: *mut ::core::ffi::c_char,
    mut stlist: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    s = is_substring(curst, stlist);
    if !s.is_null() {
        rval = 1 as ::core::ffi::c_int;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn setstemvars(
    mut s: *mut ::core::ffi::c_char,
    mut cstem: *mut ::core::ffi::c_char,
    mut clemma: *mut ::core::ffi::c_char,
    mut cstemtype: *mut ::core::ffi::c_char,
    mut cstemkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    *cstemtype = 0 as ::core::ffi::c_char;
    *clemma = *cstemtype;
    *cstem = *clemma;
    *cstemkeys = *cstem;
    s = parsefield(s, cstem, ':' as i32, MAXWORDSIZE);
    s = parsefield(s, clemma, ':' as i32, MAXWORDSIZE);
    s = parsefield(s, cstemtype, ':' as i32, MAXWORDSIZE);
    s = parsefield(s, cstemkeys, ' ' as i32, LONGSTRING);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn parsefield(
    mut s: *mut ::core::ffi::c_char,
    mut buf: *mut ::core::ffi::c_char,
    mut c: ::core::ffi::c_int,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    *buf = 0 as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != c
        && __isspace(*s as ::core::ffi::c_int) == 0
    {
        let fresh0 = s;
        s = s.offset(1);
        let fresh1 = buf;
        buf = buf.offset(1);
        *fresh1 = *fresh0;
        if i >= len {
            fprintf(
                stderr,
                b"Hey %d chars; %d s [%s] left!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                len,
                Xstrlen(s),
                s,
            );
            *buf = 0 as ::core::ffi::c_char;
            while *s as ::core::ffi::c_int != 0
                && __isspace(*s as ::core::ffi::c_int) == 0
            {
                s = s.offset(1);
            }
            break;
        } else {
            i += 1;
        }
    }
    *buf = 0 as ::core::ffi::c_char;
    if __isspace(*s as ::core::ffi::c_int) != 0 {
        return b"\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    }
    if *s != 0 {
        s = s.offset(1);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn longeststem(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = s;
    let mut tmp: [::core::ffi::c_char; 256] = [0; 256];
    let mut tmp2: [::core::ffi::c_char; 256] = [0; 256];
    let mut stemkeys: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    strcpy(&raw mut tmp as *mut ::core::ffi::c_char, s);
    stemkeys[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    p = &raw mut tmp as *mut ::core::ffi::c_char;
    while *p != 0 {
        p = p.offset(1);
    }
    p = p.offset(-1);
    while p >= &raw mut tmp as *mut ::core::ffi::c_char {
        strcpy(&raw mut tmp2 as *mut ::core::ffi::c_char, p);
        *p = 0 as ::core::ffi::c_char;
        rval
            += chckstem(
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut stemkeys as *mut ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            );
        if rval != 0 {
            printf(
                b"%s-%s\tn\t%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut tmp2 as *mut ::core::ffi::c_char,
                &raw mut stemkeys as *mut ::core::ffi::c_char,
            );
        }
        if chckstem(
            &raw mut tmp as *mut ::core::ffi::c_char,
            &raw mut stemkeys as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) != 0
        {
            printf(
                b"%s-%s\tv\t%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut tmp2 as *mut ::core::ffi::c_char,
                &raw mut stemkeys as *mut ::core::ffi::c_char,
            );
            break;
        } else {
            if rval != 0 {
                break;
            }
            p = p.offset(-1);
        }
    }
    return 0;
}
