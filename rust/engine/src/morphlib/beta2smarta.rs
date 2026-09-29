extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn islower(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ispunct(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tolower(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn toupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const HARDSHORT: ::core::ffi::c_int = '^' as i32;
pub const ACUTEFLAG: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const GRAVEFLAG: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const CIRCUMFLAG: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const SMOOTHFLAG: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const ROUGHFLAG: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const LONGMARK: ::core::ffi::c_int = 0o40 as ::core::ffi::c_int;
pub const DIAERFLAG: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const SHORTFLAG: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const ISUBFLAG: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const EQUALS: ::core::ffi::c_int = '=' as i32;
pub const UCASEMARKER: ::core::ffi::c_int = '^' as i32;
pub const SMARTA_ROUGH_RHO: ::core::ffi::c_int = 0o373 as ::core::ffi::c_int;
pub const SMK_ROUGH_RHO: ::core::ffi::c_int = 0o75 as ::core::ffi::c_int;
pub const TERMINAL_SIGMA: ::core::ffi::c_int = 'w' as i32;
pub const GREEK: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const ROMAN: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const AISUB: ::core::ffi::c_int = 0o46 as ::core::ffi::c_int;
pub const HISUB: ::core::ffi::c_int = 0o372 as ::core::ffi::c_int;
pub const WISUB: ::core::ffi::c_int = 0o304 as ::core::ffi::c_int;
pub const GKFONT: [::core::ffi::c_char; 9] = unsafe {
    ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"}{\\f132 \0")
};
pub const ROMANFONT: [::core::ffi::c_char; 3] = unsafe {
    ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"}{\0")
};
pub const BOLDFONT: [::core::ffi::c_char; 6] = unsafe {
    ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"}{\\b \0")
};
pub const ITALICFONT: [::core::ffi::c_char; 8] = unsafe {
    ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"}{\\ulw \0")
};
static mut charstyle_flag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ITALIC: ::core::ffi::c_int = '3' as i32;
pub const BOLD: ::core::ffi::c_int = '1' as i32;
static mut acctab: [::core::ffi::c_int; 11] = [
    ACUTEFLAG,
    GRAVEFLAG,
    CIRCUMFLAG,
    SMOOTHFLAG,
    ROUGHFLAG,
    SMOOTHFLAG | ACUTEFLAG,
    ROUGHFLAG | ACUTEFLAG,
    SMOOTHFLAG | GRAVEFLAG,
    ROUGHFLAG | GRAVEFLAG,
    SMOOTHFLAG | CIRCUMFLAG,
    ROUGHFLAG | CIRCUMFLAG,
];
static mut gktab: [::core::ffi::c_int; 256] = [0; 256];
static mut accenttab: [::core::ffi::c_int; 256] = [0; 256];
static mut gkinit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut cur_font: ::core::ffi::c_int = GREEK;
unsafe extern "C" fn init_gktab() -> ::core::ffi::c_int {
    gkinit += 1;
    accenttab['/' as i32 as usize] = ACUTEFLAG;
    accenttab['\\' as i32 as usize] = GRAVEFLAG;
    accenttab['=' as i32 as usize] = CIRCUMFLAG;
    accenttab[')' as i32 as usize] = SMOOTHFLAG;
    accenttab['(' as i32 as usize] = ROUGHFLAG;
    accenttab[HARDLONG as usize] = LONGMARK;
    accenttab['+' as i32 as usize] = DIAERFLAG;
    accenttab[HARDSHORT as usize] = SHORTFLAG;
    accenttab['|' as i32 as usize] = ISUBFLAG;
    gktab[' ' as i32 as usize] = 0o200 as ::core::ffi::c_int;
    gktab['a' as i32 as usize] = 0o213 as ::core::ffi::c_int;
    gktab['e' as i32 as usize] = 0o241 as ::core::ffi::c_int;
    gktab['h' as i32 as usize] = 0o256 as ::core::ffi::c_int;
    gktab['v' as i32 as usize] = 0o305 as ::core::ffi::c_int;
    gktab['i' as i32 as usize] = 0o333 as ::core::ffi::c_int;
    gktab['u' as i32 as usize] = 0o346 as ::core::ffi::c_int;
    gktab['o' as i32 as usize] = 0o361 as ::core::ffi::c_int;
    gktab[AISUB as usize] = 0o226 as ::core::ffi::c_int;
    gktab[WISUB as usize] = 0o320 as ::core::ffi::c_int;
    gktab[HISUB as usize] = 0o271 as ::core::ffi::c_int;
    return 0;
}
pub const SMARTA: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SMK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn beta2smarta(
    mut source: *mut ::core::ffi::c_char,
    mut res: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    beta2mac(source, res, SMARTA);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn beta2smk(
    mut source: *mut ::core::ffi::c_char,
    mut res: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    beta2mac(source, res, SMK);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_greek() -> ::core::ffi::c_int {
    cur_font = GREEK;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_roman() -> ::core::ffi::c_int {
    cur_font = ROMAN;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn beta2mac(
    mut source: *mut ::core::ffi::c_char,
    mut res: *mut ::core::ffi::c_char,
    mut xlit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut rp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut acc: ::core::ffi::c_int = 0;
    let mut saw_isub: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut long_vowel: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if gkinit == 0 {
        init_gktab();
    }
    sp = source;
    rp = res;
    while *sp != 0 {
        if *sp as ::core::ffi::c_int == '$' as i32 {
            if charstyle_flag != 0 {
                if rp == res {
                    if xlit == SMARTA {
                        let fresh0 = rp;
                        rp = rp.offset(1);
                        *fresh0 = 0o253 as ::core::ffi::c_int as ::core::ffi::c_char;
                    }
                } else {
                    if rp > res {
                        rp = rp.offset(-1);
                    }
                    if ispunct(*rp as ::core::ffi::c_int) != 0 {
                        *rp.offset(1 as ::core::ffi::c_int as isize) = *rp;
                        if xlit == SMARTA {
                            let fresh1 = rp;
                            rp = rp.offset(1);
                            *fresh1 = 0o253 as ::core::ffi::c_int as ::core::ffi::c_char;
                        }
                        rp = rp.offset(1);
                    } else {
                        rp = rp.offset(1);
                        if xlit == SMARTA {
                            let fresh2 = rp;
                            rp = rp.offset(1);
                            *fresh2 = 0o253 as ::core::ffi::c_int as ::core::ffi::c_char;
                        }
                    }
                }
                charstyle_flag = 0 as ::core::ffi::c_int;
            }
            sp = greekfont(sp);
            if xlit == SMK {
                strcpy(rp, GKFONT.as_ptr());
                rp = rp.offset(Xstrlen(GKFONT.as_ptr()) as isize);
            }
        } else if *sp as ::core::ffi::c_int == '&' as i32 {
            if charstyle_flag != 0
                && !(*sp as ::core::ffi::c_int == '&' as i32
                    && (*sp.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '3' as i32
                        || *sp.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == '1' as i32))
            {
                if xlit == SMARTA {
                    let fresh3 = rp;
                    rp = rp.offset(1);
                    *fresh3 = 0o253 as ::core::ffi::c_int as ::core::ffi::c_char;
                }
                charstyle_flag = 0 as ::core::ffi::c_int;
            }
            if *sp as ::core::ffi::c_int == '&' as i32
                && (*sp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '3' as i32
                    || *sp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '1' as i32)
            {
                if *sp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '3' as i32 && charstyle_flag == BOLD
                    && (*sp.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '1' as i32 && charstyle_flag == ITALIC)
                {
                    if xlit == SMARTA {
                        let fresh4 = rp;
                        rp = rp.offset(1);
                        *fresh4 = 0o253 as ::core::ffi::c_int as ::core::ffi::c_char;
                    }
                    charstyle_flag = 0 as ::core::ffi::c_int;
                }
                if *sp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '3' as i32
                {
                    charstyle_flag = ITALIC;
                } else if *sp.offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int == '1' as i32
                {
                    charstyle_flag = BOLD;
                }
                if xlit == SMARTA {
                    let fresh5 = rp;
                    rp = rp.offset(1);
                    *fresh5 = 0o137 as ::core::ffi::c_char;
                } else if charstyle_flag == ITALIC {
                    strcpy(rp, ITALICFONT.as_ptr());
                    rp = rp.offset(Xstrlen(ITALICFONT.as_ptr()) as isize);
                } else {
                    strcpy(rp, BOLDFONT.as_ptr());
                    rp = rp.offset(Xstrlen(BOLDFONT.as_ptr()) as isize);
                }
                sp = sp.offset(2 as ::core::ffi::c_int as isize);
                while __isspace(*sp as ::core::ffi::c_int) != 0 {
                    sp = sp.offset(1);
                }
            }
            sp = romanfont(sp);
            if xlit == SMK && charstyle_flag == 0
                && *rp.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    != '}' as i32
            {
                strcpy(rp, ROMANFONT.as_ptr());
                rp = rp.offset(Xstrlen(ROMANFONT.as_ptr()) as isize);
            }
        } else if *sp as ::core::ffi::c_int == '%' as i32 {
            let mut n: ::core::ffi::c_int = 0;
            let mut numbuf: [::core::ffi::c_char; 8] = [0; 8];
            let mut np: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            np = &raw mut numbuf as *mut ::core::ffi::c_char;
            sp = sp.offset(1);
            n = atoi(sp);
            while if 0 as ::core::ffi::c_int != 0 {
                isdigit(*sp as ::core::ffi::c_int)
            } else {
                ((*sp as ::core::ffi::c_uint)
                    .wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                    < 10 as ::core::ffi::c_uint) as ::core::ffi::c_int
            } != 0
            {
                let fresh6 = sp;
                sp = sp.offset(1);
                let fresh7 = np;
                np = np.offset(1);
                *fresh7 = *fresh6;
            }
            *np = 0 as ::core::ffi::c_char;
            let mut current_block_103: u64;
            match n {
                1 => {
                    let fresh8 = rp;
                    rp = rp.offset(1);
                    *fresh8 = '?' as i32 as ::core::ffi::c_char;
                    current_block_103 = 7416055328783156979;
                }
                2 => {
                    let fresh9 = rp;
                    rp = rp.offset(1);
                    *fresh9 = '*' as i32 as ::core::ffi::c_char;
                    current_block_103 = 7416055328783156979;
                }
                4 => {
                    let fresh10 = rp;
                    rp = rp.offset(1);
                    *fresh10 = '!' as i32 as ::core::ffi::c_char;
                    current_block_103 = 7416055328783156979;
                }
                6 => {
                    if xlit == SMARTA {
                        let fresh11 = rp;
                        rp = rp.offset(1);
                        *fresh11 = EQUALS as ::core::ffi::c_char;
                    } else if cur_font == GREEK {
                        strcpy(rp, ROMANFONT.as_ptr());
                        rp = rp.offset(Xstrlen(ROMANFONT.as_ptr()) as isize);
                        strcpy(rp, b"=}{\0" as *const u8 as *const ::core::ffi::c_char);
                        rp = rp.offset(3 as ::core::ffi::c_int as isize);
                        strcpy(rp, GKFONT.as_ptr());
                        rp = rp.offset(Xstrlen(GKFONT.as_ptr()) as isize);
                    } else {
                        let fresh12 = rp;
                        rp = rp.offset(1);
                        *fresh12 = '=' as i32 as ::core::ffi::c_char;
                    }
                    current_block_103 = 7416055328783156979;
                }
                10 => {
                    if xlit == SMARTA {
                        let fresh13 = rp;
                        rp = rp.offset(1);
                        *fresh13 = '`' as i32 as ::core::ffi::c_char;
                    } else {
                        let fresh14 = rp;
                        rp = rp.offset(1);
                        *fresh14 = ':' as i32 as ::core::ffi::c_char;
                    }
                    current_block_103 = 7416055328783156979;
                }
                40 => {
                    if xlit == SMK {
                        let fresh15 = rp;
                        rp = rp.offset(1);
                        *fresh15 = ' ' as i32 as ::core::ffi::c_char;
                        let fresh16 = rp;
                        rp = rp.offset(1);
                        *fresh16 = SMK_SHORTMARK as ::core::ffi::c_char;
                        current_block_103 = 17086616517068404508;
                    } else if xlit == SMARTA {
                        let fresh17 = rp;
                        rp = rp.offset(1);
                        *fresh17 = SMARTA_SHORTMARK as ::core::ffi::c_char;
                        current_block_103 = 7416055328783156979;
                    } else {
                        current_block_103 = 17086616517068404508;
                    }
                }
                41 => {
                    current_block_103 = 17086616517068404508;
                }
                _ => {
                    current_block_103 = 1197696952469282886;
                }
            }
            match current_block_103 {
                17086616517068404508 => {
                    if xlit == SMK {
                        let fresh18 = rp;
                        rp = rp.offset(1);
                        *fresh18 = ' ' as i32 as ::core::ffi::c_char;
                        let fresh19 = rp;
                        rp = rp.offset(1);
                        *fresh19 = SMK_LONGMARK as ::core::ffi::c_char;
                        current_block_103 = 7416055328783156979;
                    } else {
                        if xlit == SMARTA {
                            let fresh20 = rp;
                            rp = rp.offset(1);
                            *fresh20 = SMARTA_LONGMARK as ::core::ffi::c_char;
                        }
                        current_block_103 = 1197696952469282886;
                    }
                }
                _ => {}
            }
            match current_block_103 {
                1197696952469282886 => {
                    np = &raw mut numbuf as *mut ::core::ffi::c_char;
                    let fresh21 = rp;
                    rp = rp.offset(1);
                    *fresh21 = '%' as i32 as ::core::ffi::c_char;
                    while *np != 0 {
                        let fresh22 = np;
                        np = np.offset(1);
                        let fresh23 = rp;
                        rp = rp.offset(1);
                        *fresh23 = *fresh22;
                    }
                }
                _ => {}
            }
        } else if *sp as ::core::ffi::c_int == '*' as i32 && cur_font == ROMAN {
            if xlit == SMARTA {
                let fresh24 = rp;
                rp = rp.offset(1);
                *fresh24 = UCASEMARKER as ::core::ffi::c_char;
            } else if xlit == SMK {
                sp = sp.offset(1);
                strcpy(rp, sp);
                if if 0 as ::core::ffi::c_int != 0 {
                    islower(*rp as ::core::ffi::c_int)
                } else {
                    ((*rp as ::core::ffi::c_uint)
                        .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                        < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
                } != 0
                {
                    *rp = toupper(*rp as ::core::ffi::c_int) as ::core::ffi::c_char;
                }
                rp = rp.offset(1);
            }
            sp = sp.offset(1);
        } else if cur_font == ROMAN
            && (if 0 as ::core::ffi::c_int != 0 {
                isalpha(*sp as ::core::ffi::c_int)
            } else {
                ((*sp as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
            }) != 0
        {
            if if 0 as ::core::ffi::c_int != 0 {
                isupper(*sp as ::core::ffi::c_int)
            } else {
                ((*sp as ::core::ffi::c_uint)
                    .wrapping_sub('A' as i32 as ::core::ffi::c_uint)
                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
            } != 0
            {
                let fresh25 = rp;
                rp = rp.offset(1);
                *fresh25 = UCASEMARKER as ::core::ffi::c_char;
                let fresh26 = sp;
                sp = sp.offset(1);
                let fresh27 = rp;
                rp = rp.offset(1);
                *fresh27 = *fresh26;
            } else if xlit == SMARTA {
                let fresh28 = sp;
                sp = sp.offset(1);
                let fresh29 = rp;
                rp = rp.offset(1);
                *fresh29 = toupper(*fresh28 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            } else {
                let fresh30 = sp;
                sp = sp.offset(1);
                let fresh31 = rp;
                rp = rp.offset(1);
                *fresh31 = *fresh30;
            }
        } else if *sp as ::core::ffi::c_int == '[' as i32
            && *sp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '1' as i32
            && *sp.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '.' as i32
        {
            let fresh32 = rp;
            rp = rp.offset(1);
            *fresh32 = '(' as i32 as ::core::ffi::c_char;
            sp = sp.offset(2 as ::core::ffi::c_int as isize);
        } else if *sp as ::core::ffi::c_int == ']' as i32
            && *sp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '1' as i32
        {
            let fresh33 = rp;
            rp = rp.offset(1);
            *fresh33 = ')' as i32 as ::core::ffi::c_char;
            sp = sp.offset(2 as ::core::ffi::c_int as isize);
        } else {
            if (if 0 as ::core::ffi::c_int != 0 {
                isalpha(*sp as ::core::ffi::c_int)
            } else {
                ((*sp as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
            }) != 0 || *sp as ::core::ffi::c_int == '*' as i32
            {
                acc = 0 as ::core::ffi::c_int;
                if xlit == SMK && *sp as ::core::ffi::c_int == '*' as i32 {
                    if accenttab[*sp.offset(1 as ::core::ffi::c_int as isize) as usize]
                        > 0 as ::core::ffi::c_int
                        && accenttab[*sp.offset(1 as ::core::ffi::c_int as isize)
                            as usize] <= ISUBFLAG
                    {
                        let mut t: *mut ::core::ffi::c_char = sp;
                        *sp = ' ' as i32 as ::core::ffi::c_char;
                        while *t as ::core::ffi::c_int != 0
                            && (if 0 as ::core::ffi::c_int != 0 {
                                isalpha(*t as ::core::ffi::c_int)
                            } else {
                                ((*t as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
                            }) == 0
                        {
                            t = t.offset(1);
                        }
                        if (if 0 as ::core::ffi::c_int != 0 {
                            isalpha(*t as ::core::ffi::c_int)
                        } else {
                            ((*t as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
                        }) != 0
                            && (if 0 as ::core::ffi::c_int != 0 {
                                islower(*t as ::core::ffi::c_int)
                            } else {
                                ((*t as ::core::ffi::c_uint)
                                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
                            }) != 0
                        {
                            *t = toupper(*t as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        }
                    } else {
                        strcpy(sp, sp.offset(1 as ::core::ffi::c_int as isize));
                        if if 0 as ::core::ffi::c_int != 0 {
                            islower(*sp as ::core::ffi::c_int)
                        } else {
                            ((*sp as ::core::ffi::c_uint)
                                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
                        } != 0
                        {
                            *sp = toupper(*sp as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        }
                    }
                }
                let fresh34 = sp;
                sp = sp.offset(1);
                *rp = *fresh34;
                if (if 0 as ::core::ffi::c_int != 0 {
                    isupper(*rp as ::core::ffi::c_int)
                } else {
                    ((*rp as ::core::ffi::c_uint)
                        .wrapping_sub('A' as i32 as ::core::ffi::c_uint)
                        < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
                }) != 0 && xlit == SMARTA
                {
                    *rp.offset(1 as ::core::ffi::c_int as isize) = tolower(
                        *rp as ::core::ffi::c_int,
                    ) as ::core::ffi::c_char;
                    let fresh35 = rp;
                    rp = rp.offset(1);
                    *fresh35 = UCASEMARKER as ::core::ffi::c_char;
                }
                *rp = smk_char_xlit(*rp as ::core::ffi::c_int, sp, xlit)
                    as ::core::ffi::c_char;
                while accenttab[*sp as usize] > 0 as ::core::ffi::c_int
                    && accenttab[*sp as usize] <= ISUBFLAG
                {
                    if *sp as ::core::ffi::c_int == HARDLONG {
                        long_vowel += 1;
                        sp = sp.offset(1);
                    } else if *sp as ::core::ffi::c_int == '|' as i32 {
                        saw_isub += 1;
                        sp = sp.offset(1);
                    } else if *sp as ::core::ffi::c_int == HARDSHORT {
                        sp = sp.offset(1);
                    } else {
                        let fresh36 = sp;
                        sp = sp.offset(1);
                        acc += accenttab[*fresh36 as usize];
                    }
                }
                if acc != 0 && *rp as ::core::ffi::c_int == UCASEMARKER && xlit == SMARTA
                {
                    if if 0 as ::core::ffi::c_int != 0 {
                        isalpha(*sp as ::core::ffi::c_int)
                    } else {
                        ((*sp as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                            .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                            < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
                    } != 0
                    {
                        let fresh37 = sp;
                        sp = sp.offset(1);
                        rp = rp.offset(1);
                        *rp = *fresh37;
                        *rp = smk_char_xlit(*rp as ::core::ffi::c_int, sp, xlit)
                            as ::core::ffi::c_char;
                    }
                }
                if long_vowel != 0 {
                    if acc & CIRCUMFLAG == 0 {
                        if xlit == SMARTA {
                            if *rp as ::core::ffi::c_int == 'a' as i32 {
                                *rp = 0o46 as ::core::ffi::c_char;
                            } else if *rp as ::core::ffi::c_int == 'i' as i32 {
                                *rp = 0o372 as ::core::ffi::c_int as ::core::ffi::c_char;
                                *rp = (*rp as ::core::ffi::c_int
                                    & 0o377 as ::core::ffi::c_int) as ::core::ffi::c_char;
                            } else if *rp as ::core::ffi::c_int == 'u' as i32 {
                                *rp = 0o304 as ::core::ffi::c_int as ::core::ffi::c_char;
                                *rp = (*rp as ::core::ffi::c_int
                                    & 0o377 as ::core::ffi::c_int) as ::core::ffi::c_char;
                            }
                        } else if xlit == SMK {
                            *rp.offset(1 as ::core::ffi::c_int as isize) = *rp;
                            let fresh38 = rp;
                            rp = rp.offset(1);
                            *fresh38 = '*' as i32 as ::core::ffi::c_char;
                        }
                    }
                    long_vowel = 0 as ::core::ffi::c_int;
                }
                if saw_isub != 0 && xlit == SMK {
                    match *rp as ::core::ffi::c_int {
                        97 => {
                            *rp = AISUB as ::core::ffi::c_char;
                        }
                        104 => {
                            *rp = HISUB as ::core::ffi::c_char;
                            *rp = (*rp as ::core::ffi::c_int
                                & 0o377 as ::core::ffi::c_int) as ::core::ffi::c_char;
                        }
                        118 => {
                            *rp = WISUB as ::core::ffi::c_char;
                            *rp = (*rp as ::core::ffi::c_int
                                & 0o377 as ::core::ffi::c_int) as ::core::ffi::c_char;
                        }
                        _ => {}
                    }
                    saw_isub = 0 as ::core::ffi::c_int;
                }
                if acc != 0 {
                    if *rp as ::core::ffi::c_int == 'r' as i32 && acc == ROUGHFLAG {
                        if xlit == SMK {
                            *rp = SMK_ROUGH_RHO as ::core::ffi::c_char;
                        } else {
                            *rp = SMARTA_ROUGH_RHO as ::core::ffi::c_char;
                        }
                    } else if acc == DIAERFLAG
                        && (*rp as ::core::ffi::c_int == 'i' as i32
                            || *rp as ::core::ffi::c_int == 'u' as i32)
                    {
                        if *rp as ::core::ffi::c_int == 'i' as i32 {
                            *rp = 0o363 as ::core::ffi::c_int as ::core::ffi::c_char;
                        } else {
                            *rp = 0o43 as ::core::ffi::c_char;
                        }
                    } else if acc == DIAERFLAG | ACUTEFLAG
                        && (*rp as ::core::ffi::c_int == 'i' as i32
                            || *rp as ::core::ffi::c_int == 'u' as i32)
                    {
                        if *rp as ::core::ffi::c_int == 'i' as i32 {
                            *rp = 0o375 as ::core::ffi::c_int as ::core::ffi::c_char;
                        } else {
                            *rp = 0o100 as ::core::ffi::c_char;
                        }
                    } else if acc == DIAERFLAG | GRAVEFLAG
                        && (*rp as ::core::ffi::c_int == 'i' as i32
                            || *rp as ::core::ffi::c_int == 'u' as i32)
                    {
                        if *rp as ::core::ffi::c_int == 'i' as i32 {
                            *rp = 0o376 as ::core::ffi::c_int as ::core::ffi::c_char;
                        } else {
                            *rp = 0o243 as ::core::ffi::c_int as ::core::ffi::c_char;
                        }
                    } else if gktab[*rp as usize] == 0 {
                        *rp.offset(1 as ::core::ffi::c_int as isize) = *rp;
                        *rp = '?' as i32 as ::core::ffi::c_char;
                        rp = rp.offset(2 as ::core::ffi::c_int as isize);
                        *rp = '?' as i32 as ::core::ffi::c_char;
                    } else {
                        *rp = (gktab[*rp as usize] + accnum(acc)) as ::core::ffi::c_uchar
                            as ::core::ffi::c_char;
                    }
                }
                if saw_isub != 0 && xlit == SMARTA {
                    saw_isub = 0 as ::core::ffi::c_int;
                    rp = rp.offset(1);
                    *rp = 'i' as i32 as ::core::ffi::c_char;
                }
            } else if *sp as ::core::ffi::c_int == '_' as i32 {
                *rp = '-' as i32 as ::core::ffi::c_char;
                sp = sp.offset(1);
            } else {
                let fresh39 = sp;
                sp = sp.offset(1);
                *rp = *fresh39;
            }
            rp = rp.offset(1);
        }
    }
    *rp = 0 as ::core::ffi::c_char;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn accnum(mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[::core::ffi::c_int; 11]>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
    {
        if n == acctab[i as usize] {
            return i;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn romanfont(
    mut s: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    cur_font = ROMAN;
    while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int == '&' as i32 {
        s = s.offset(1);
    }
    if if 0 as ::core::ffi::c_int != 0 {
        isdigit(*s as ::core::ffi::c_int)
    } else {
        ((*s as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint)
            < 10 as ::core::ffi::c_uint) as ::core::ffi::c_int
    } != 0
    {
        while if 0 as ::core::ffi::c_int != 0 {
            isdigit(*s as ::core::ffi::c_int)
        } else {
            ((*s as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                < 10 as ::core::ffi::c_uint) as ::core::ffi::c_int
        } != 0
        {
            s = s.offset(1);
        }
    } else if *s as ::core::ffi::c_int == ' ' as i32
        && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32
    {
        s = s.offset(1);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn greekfont(
    mut s: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    cur_font = GREEK;
    while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int == '$' as i32 {
        s = s.offset(1);
    }
    if if 0 as ::core::ffi::c_int != 0 {
        isdigit(*s as ::core::ffi::c_int)
    } else {
        ((*s as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint)
            < 10 as ::core::ffi::c_uint) as ::core::ffi::c_int
    } != 0
    {
        while if 0 as ::core::ffi::c_int != 0 {
            isdigit(*s as ::core::ffi::c_int)
        } else {
            ((*s as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                < 10 as ::core::ffi::c_uint) as ::core::ffi::c_int
        } != 0
        {
            s = s.offset(1);
        }
    } else if *s as ::core::ffi::c_int == ' ' as i32
        && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32
    {
        s = s.offset(1);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn smk_char_xlit(
    mut c: ::core::ffi::c_int,
    mut s: *mut ::core::ffi::c_char,
    mut xlit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if c == 's' as i32
        && (if 0 as ::core::ffi::c_int != 0 {
            isalpha(*s as ::core::ffi::c_int)
        } else {
            ((*s as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) == 0 && *s as ::core::ffi::c_int != '\'' as i32
        && *s as ::core::ffi::c_int != '-' as i32
    {
        c = TERMINAL_SIGMA;
    } else if c == 'w' as i32 {
        c = 'v' as i32;
    } else if c == 'q' as i32 {
        c = 'y' as i32;
    } else if c == 'Q' as i32 {
        c = 'Y' as i32;
    } else if c == 'c' as i32 {
        c = 'j' as i32;
    } else if c == 'C' as i32 {
        c = 'J' as i32;
    } else if c == 'y' as i32 {
        c = 'c' as i32;
    } else if c == 'W' as i32 {
        c = 'V' as i32;
    } else if c == 'V' as i32 {
        c = 'C' as i32;
    } else if c == 'v' as i32 {
        if xlit == SMARTA {
            c = 'q' as i32;
        } else {
            c = 'W' as i32;
        }
    } else if c == '*' as i32 && xlit == SMARTA {
        c = UCASEMARKER;
    }
    return c;
}
pub const SMARTA_LONGMARK: ::core::ffi::c_int = '^' as i32;
pub const SMARTA_SHORTMARK: ::core::ffi::c_int = 0o255 as ::core::ffi::c_int;
pub const SMK_LONGMARK: ::core::ffi::c_int = '*' as i32;
pub const SMK_SHORTMARK: ::core::ffi::c_int = 0o255 as ::core::ffi::c_int;
