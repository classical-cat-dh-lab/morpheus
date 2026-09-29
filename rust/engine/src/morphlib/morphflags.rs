use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn NameOfMorphFlags(_: ::core::ffi::c_long) -> *mut ::core::ffi::c_char;
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
pub const MORPHFLAG_BYTES: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const MORPHFLAG_MASK: ::core::ffi::c_int = 0o377 as ::core::ffi::c_int;
pub const SUFF_ACC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const STEM_ACC: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PERS_NAME: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ANT_ACC: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const NO_COMP: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SHORT_PEN: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const LONG_PEN: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const ACCENT_OPTIONAL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NEEDS_ACCENT: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const NOT_IN_COMPOSITION: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const HAS_PREVERB: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const DISSIMILATION: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const APOCOPE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const HAS_AUGMENT: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const PREVB_AUGMENT: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const ELIDE_PREVERB: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const ROOT_PREVERB: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const RAW_PREVERB: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const UNASP_PREVERB: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const DOUBLED_CONS: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn add_morphflags(
    mut gstr: *mut gk_string,
    mut Flags: *mut MorphFlags,
) -> ::core::ffi::c_int {
    let mut Mf: *mut ::core::ffi::c_uchar = &raw mut (*gstr).gs_morphflags
        as *mut ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MORPHFLAG_BYTES {
        (*gstr).gs_morphflags[i as usize] = ((*gstr).gs_morphflags[i as usize]
            as ::core::ffi::c_int | *Flags.offset(i as isize) as ::core::ffi::c_int)
            as MorphFlags;
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_morphflags(
    mut gstr: *mut gk_string,
    mut Flags: *mut MorphFlags,
) -> ::core::ffi::c_int {
    let mut Mf: *mut ::core::ffi::c_uchar = &raw mut (*gstr).gs_morphflags
        as *mut ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MORPHFLAG_BYTES {
        (*gstr).gs_morphflags[i as usize] = *Flags.offset(i as isize);
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_gwmorphflags(
    mut gkword: *mut gk_word,
    mut Flags: *mut MorphFlags,
) -> ::core::ffi::c_int {
    let mut Mf: *mut ::core::ffi::c_uchar = &raw mut (*gkword).gs_morphflags
        as *mut ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MORPHFLAG_BYTES {
        (*gkword).gs_morphflags[i as usize] = *Flags.offset(i as isize);
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn zap_morphflags(
    mut gstr: *mut gk_string,
    mut Flags: *mut MorphFlags,
) -> ::core::ffi::c_int {
    let mut Mf: *mut ::core::ffi::c_uchar = &raw mut (*gstr).gs_morphflags
        as *mut ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MORPHFLAG_BYTES {
        (*gstr).gs_morphflags[i as usize] = ((*gstr).gs_morphflags[i as usize]
            as ::core::ffi::c_int & !(*Flags.offset(i as isize) as ::core::ffi::c_int))
            as MorphFlags;
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn has_morphflags(
    mut gstr: *mut gk_string,
    mut Flags: *mut MorphFlags,
) -> ::core::ffi::c_int {
    let mut Mf: *mut ::core::ffi::c_uchar = &raw mut (*gstr).gs_morphflags
        as *mut ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MORPHFLAG_BYTES {
        if (*gstr).gs_morphflags[i as usize] as ::core::ffi::c_int
            & *Flags.offset(i as isize) as ::core::ffi::c_int != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn no_morphflags(mut gstr: *mut gk_string) -> ::core::ffi::c_int {
    let mut Mf: *mut MorphFlags = &raw mut (*gstr).gs_morphflags as *mut MorphFlags;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MORPHFLAG_BYTES {
        if (*gstr).gs_morphflags[i as usize] != 0 {
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn add_morphflag(
    mut Mf: *mut MorphFlags,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    let mut setbit: ::core::ffi::c_int = 0;
    n = n & MORPHFLAG_MASK;
    mflag_num_to_bits(n, &raw mut index, &raw mut setbit);
    let ref mut fresh0 = *Mf.offset(index as isize);
    *fresh0 = (*fresh0 as ::core::ffi::c_int | setbit & 0o377 as ::core::ffi::c_int)
        as MorphFlags;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn overlap_morphflags(
    mut Mf1: *mut MorphFlags,
    mut Mf2: *mut MorphFlags,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MORPHFLAG_BYTES {
        if *Mf1.offset(i as isize) as ::core::ffi::c_int
            & *Mf2.offset(i as isize) as ::core::ffi::c_int != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn has_morphflag(
    mut Mf: *mut MorphFlags,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    let mut setbit: ::core::ffi::c_int = 0;
    n = n & MORPHFLAG_MASK;
    mflag_num_to_bits(n, &raw mut index, &raw mut setbit);
    return *Mf.offset(index as isize) as ::core::ffi::c_int
        & (setbit & 0o377 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zap_morphflag(
    mut Mf: *mut MorphFlags,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    let mut setbit: ::core::ffi::c_int = 0;
    n = n & MORPHFLAG_MASK;
    mflag_num_to_bits(n, &raw mut index, &raw mut setbit);
    let ref mut fresh1 = *Mf.offset(index as isize);
    *fresh1 = (*fresh1 as ::core::ffi::c_int & !(setbit & 0o377 as ::core::ffi::c_int))
        as MorphFlags;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_morphflag(
    mut Mf: *mut MorphFlags,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut index: ::core::ffi::c_int = 0;
    let mut setbit: ::core::ffi::c_int = 0;
    n = n & MORPHFLAG_MASK;
    mflag_num_to_bits(n, &raw mut index, &raw mut setbit);
    i = 0 as ::core::ffi::c_int;
    while i < MORPHFLAG_BYTES {
        *Mf.offset(i as isize) = 0 as MorphFlags;
        i += 1;
    }
    if n > 0 as ::core::ffi::c_int {
        *Mf.offset(index as isize) = (setbit & 0o377 as ::core::ffi::c_int)
            as MorphFlags;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn no_morphflag(mut mf: *mut MorphFlags) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MORPHFLAG_BYTES {
        if *mf.offset(i as isize) != 0 {
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mflag_num_to_bits(
    mut n: ::core::ffi::c_int,
    mut ind: *mut ::core::ffi::c_int,
    mut bitnum: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if n % 8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        *ind = n / 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
        *bitnum = 0o200 as ::core::ffi::c_int;
    } else {
        *ind = n / 8 as ::core::ffi::c_int;
        *bitnum = (1 as ::core::ffi::c_int)
            << n % 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn mflag_bit_to_num(
    mut ind: ::core::ffi::c_int,
    mut bitnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return ind * 8 as ::core::ffi::c_int + bitnum;
}
#[no_mangle]
pub unsafe extern "C" fn Dump_morphflag(mut mf: *mut MorphFlags) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MORPHFLAG_BYTES {
        printf(
            b"byte %d [%o]\n\0" as *const u8 as *const ::core::ffi::c_char,
            i,
            *mf.offset(i as isize) as ::core::ffi::c_int,
        );
        i += 1;
    }
    return 0;
}
static mut ugly_tab: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn is_pretty_morphflag(
    mut mnum: ::core::ffi::c_long,
) -> ::core::ffi::c_int {
    if ugly_tab.is_null() {
        init_ugly_tab();
    }
    return (*ugly_tab.offset(mnum as ::core::ffi::c_int as isize) == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init_ugly_tab() -> ::core::ffi::c_int {
    ugly_tab = calloc(
        (MORPHFLAG_BYTES as size_t).wrapping_mul(8 as size_t).wrapping_add(1 as size_t),
        ::core::mem::size_of::<::core::ffi::c_char>(),
    ) as *mut ::core::ffi::c_char;
    *ugly_tab.offset(PERS_NAME as isize) = 1 as ::core::ffi::c_char;
    *ugly_tab.offset(SUFF_ACC as isize) = 1 as ::core::ffi::c_char;
    *ugly_tab.offset(STEM_ACC as isize) = 1 as ::core::ffi::c_char;
    *ugly_tab.offset(ANT_ACC as isize) = 1 as ::core::ffi::c_char;
    *ugly_tab.offset(NO_COMP as isize) = 1 as ::core::ffi::c_char;
    *ugly_tab.offset(SHORT_PEN as isize) = 1 as ::core::ffi::c_char;
    *ugly_tab.offset(LONG_PEN as isize) = 1 as ::core::ffi::c_char;
    *ugly_tab.offset(ACCENT_OPTIONAL as isize) = 1 as ::core::ffi::c_char;
    *ugly_tab.offset(NEEDS_ACCENT as isize) = 1 as ::core::ffi::c_char;
    *ugly_tab.offset(NOT_IN_COMPOSITION as isize) = 1 as ::core::ffi::c_char;
    *ugly_tab.offset(HAS_PREVERB as isize) = 1 as ::core::ffi::c_char;
    *ugly_tab.offset(HAS_AUGMENT as isize) = 1 as ::core::ffi::c_char;
    return 0;
}
#[no_mangle]
pub static mut prvb_tab: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn is_prvb_morphflag(
    mut mnum: ::core::ffi::c_long,
) -> ::core::ffi::c_int {
    if prvb_tab.is_null() {
        init_prvb_tab();
    }
    return *prvb_tab.offset(mnum as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init_prvb_tab() -> ::core::ffi::c_int {
    if !prvb_tab.is_null() {
        return 0;
    }
    prvb_tab = calloc(
        (MORPHFLAG_BYTES * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<::core::ffi::c_char>(),
    ) as *mut ::core::ffi::c_char;
    if prvb_tab.is_null() {
        fprintf(stderr, b"Damn!\n\0" as *const u8 as *const ::core::ffi::c_char);
        exit(-(1 as ::core::ffi::c_int));
    }
    *prvb_tab.offset(DISSIMILATION as isize) = 1 as ::core::ffi::c_char;
    *prvb_tab.offset(PREVB_AUGMENT as isize) = 1 as ::core::ffi::c_char;
    *prvb_tab.offset(ELIDE_PREVERB as isize) = 1 as ::core::ffi::c_char;
    *prvb_tab.offset(ROOT_PREVERB as isize) = 1 as ::core::ffi::c_char;
    *prvb_tab.offset(RAW_PREVERB as isize) = 1 as ::core::ffi::c_char;
    *prvb_tab.offset(UNASP_PREVERB as isize) = 1 as ::core::ffi::c_char;
    *prvb_tab.offset(APOCOPE as isize) = 1 as ::core::ffi::c_char;
    *prvb_tab.offset(DOUBLED_CONS as isize) = 1 as ::core::ffi::c_char;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn xfer_prvbflags(
    mut word_mf: *mut MorphFlags,
    mut prvb_mf: *mut MorphFlags,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MORPHFLAG_BYTES * 8 as ::core::ffi::c_int {
        if is_prvb_morphflag(i as ::core::ffi::c_long) != 0
            && has_morphflag(word_mf, i) != 0
        {
            zap_morphflag(word_mf, i);
            add_morphflag(prvb_mf, i);
        }
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn MorphNames(
    mut mf: *mut MorphFlags,
    mut res: *mut ::core::ffi::c_char,
    mut dels: *mut ::core::ffi::c_char,
    mut pretty: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_long = 0;
    let mut j: ::core::ffi::c_long = 0;
    let mut curnum: ::core::ffi::c_long = 0;
    let mut mask: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut hit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *res = 0 as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_long;
    while i < MORPHFLAG_BYTES as ::core::ffi::c_long {
        mask = 1 as ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_long;
        while j < 8 as ::core::ffi::c_long {
            if *mf.offset(i as isize) as ::core::ffi::c_int & mask != 0 {
                curnum = mflag_bit_to_num(
                    i as ::core::ffi::c_int,
                    j as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                ) as ::core::ffi::c_long;
                if pretty == 0 || is_pretty_morphflag(curnum) != 0 {
                    hit += 1;
                    s = NameOfMorphFlags(curnum);
                    if *s != 0 {
                        if *res != 0 {
                            strcat(res, dels);
                        }
                        strcat(res, s);
                    }
                }
            }
            mask = mask << 1 as ::core::ffi::c_int;
            j += 1;
        }
        i += 1;
    }
    return 0;
}
