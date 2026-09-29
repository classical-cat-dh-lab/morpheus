use ::c2rust_bitfields;
extern "C" {
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
    fn cinsert(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
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
    fn add_double_augment(
        _: *mut ::core::ffi::c_char,
        _: *mut MorphFlags,
    ) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn addbreath(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn aspirate(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn exp_preverb(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn getbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn set_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strsqz(_: *mut ::core::ffi::c_char, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn zap_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type int32 = ::core::ffi::c_uint;
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
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CONTCHAR: ::core::ffi::c_int = '>' as i32;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
pub const NOBREATH: ::core::ffi::c_int = ' ' as i32;
pub const HAS_PREVERB: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const DISSIMILATION: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const APOCOPE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const PREVB_AUGMENT: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const ELIDE_PREVERB: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const INDECLFORM: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const RAW_PREVERB: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const UNASP_PREVERB: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const DOUBLED_CONS: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
pub const IOTA_INTENS: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const SIG_TO_CI: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
pub const SHORT_EIS: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const PROS_TO_POTI: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const META_TO_PEDA: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
pub const PROS_TO_PROTI: ::core::ffi::c_int = 74 as ::core::ffi::c_int;
pub const UPO_TO_UPAI: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const PARA_TO_PARAI: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const UPER_TO_UPEIR: ::core::ffi::c_int = 77 as ::core::ffi::c_int;
pub const EN_TO_ENI: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const D_PREVB: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
pub const T_PREVB: ::core::ffi::c_int = 83 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
#[no_mangle]
pub static mut prevbs: [[::core::ffi::c_char; 8]; 31] = unsafe {
    [
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"a)mf\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"a)mfi/\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"a)n\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"a)na/\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"a)nt\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"a)nti/\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"a)p\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"a)po/\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"di\0\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"dia/\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"e)s\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"ei)s\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"e)n\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"e)k\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"e)c\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"e)p\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"e)pi/\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"kat\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"kata/\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"met\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"meta/\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"par\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"para/\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"peri/\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"pro/s\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"pro/\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"cu/n\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"su/n\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"u(pe/r\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"u(p\0\0\0\0\0"),
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"u(po/\0\0\0"),
    ]
};
pub const NUMPREVBS: usize = (::core::mem::size_of::<[[::core::ffi::c_char; 8]; 31]>()
    as usize)
    .wrapping_div(::core::mem::size_of::<[::core::ffi::c_char; 8]>() as usize);
static mut verbose: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn checkprevb(
    mut word: *mut ::core::ffi::c_char,
    mut prevb: *mut ::core::ffi::c_char,
    mut brflg: *mut bool_0,
) -> bool_0 {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as usize) < NUMPREVBS {
        if *prevb == 0 {
            break;
        }
        if strcmp(
            prevb,
            &raw mut *(&raw mut prevbs as *mut [::core::ffi::c_char; 8])
                .offset(i as isize) as *mut ::core::ffi::c_char,
        ) == 0
        {
            i += 1;
            break;
        } else {
            i += 1;
        }
    }
    while (i as usize) < NUMPREVBS {
        if prvbcmp(
            &raw mut *(&raw mut prevbs as *mut [::core::ffi::c_char; 8])
                .offset(i as isize) as *mut ::core::ffi::c_char,
            word,
            brflg,
        ) != 0
        {
            Xstrncpy(
                prevb,
                &raw mut *(&raw mut prevbs as *mut [::core::ffi::c_char; 8])
                    .offset(i as isize) as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            return 1 as bool_0;
        }
        i += 1;
    }
    return 0 as bool_0;
}
#[no_mangle]
pub unsafe extern "C" fn prvbcmp(
    mut prevb: *mut ::core::ffi::c_char,
    mut word: *mut ::core::ffi::c_char,
    mut brflg: *mut bool_0,
) -> bool_0 {
    let mut workp: [::core::ffi::c_char; 8] = [0; 8];
    let mut workw: [::core::ffi::c_char; 60] = [0; 60];
    let mut workrest: [::core::ffi::c_char; 60] = [0; 60];
    let mut lastc: ::core::ffi::c_int = *(&raw mut workp as *mut ::core::ffi::c_char)
        .offset(Xstrlen(&raw mut workp as *mut ::core::ffi::c_char) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
    Xstrncpy(&raw mut workp as *mut ::core::ffi::c_char, prevb, MAXWORDSIZE as size_t);
    Xstrncpy(&raw mut workw as *mut ::core::ffi::c_char, word, MAXWORDSIZE as size_t);
    stripacc(&raw mut workp as *mut ::core::ffi::c_char);
    stripacc(&raw mut workw as *mut ::core::ffi::c_char);
    *brflg = NO as bool_0;
    if (lastc == 'a' as i32 || lastc == 'e' as i32 || lastc == 'i' as i32
        || lastc == 'o' as i32 || lastc == 'u' as i32 || lastc == 'A' as i32
        || lastc == 'E' as i32 || lastc == 'I' as i32 || lastc == 'O' as i32
        || lastc == 'U' as i32
        || (lastc == 'h' as i32 || lastc == 'w' as i32 || lastc == 'H' as i32
            || lastc == 'W' as i32))
        && 0 as ::core::ffi::c_int
            != strcmp(
                (&raw mut workp as *mut ::core::ffi::c_char)
                    .offset(Xstrlen(&raw mut workp as *mut ::core::ffi::c_char) as isize)
                    .offset(-(3 as ::core::ffi::c_int as isize)),
                b"pro\0" as *const u8 as *const ::core::ffi::c_char,
            )
    {
        strsqz(
            (&raw mut workp as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut workp as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
            1 as ::core::ffi::c_int,
        );
        if 0 as ::core::ffi::c_int
            == Xstrncmp(
                &raw mut workp as *mut ::core::ffi::c_char,
                &raw mut workw as *mut ::core::ffi::c_char,
                Xstrlen(&raw mut workp as *mut ::core::ffi::c_char) as size_t,
            )
        {
            getrest(
                &raw mut workrest as *mut ::core::ffi::c_char,
                word,
                &raw mut workw as *mut ::core::ffi::c_char,
                &raw mut workp as *mut ::core::ffi::c_char,
            );
            if workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'a' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'e' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'i' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'o' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'u' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'A' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'E' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'I' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'O' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'U' as i32
                || (workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'h' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'w' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'H' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'W' as i32)
            {
                addbreath(&raw mut workrest as *mut ::core::ffi::c_char, SMOOTHBR);
                if !(lastc == 'p' as i32 || lastc == 't' as i32 || lastc == 'k' as i32
                    || (lastc == 'b' as i32 || lastc == 'd' as i32
                        || lastc == 'g' as i32))
                {
                    *brflg = YES as bool_0;
                }
                Xstrncpy(
                    word,
                    &raw mut workrest as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return 1 as bool_0;
            }
        }
    }
    if 0 as ::core::ffi::c_int
        == Xstrncmp(
            &raw mut workp as *mut ::core::ffi::c_char,
            &raw mut workw as *mut ::core::ffi::c_char,
            Xstrlen(&raw mut workp as *mut ::core::ffi::c_char) as size_t,
        )
    {
        getrest(
            &raw mut workrest as *mut ::core::ffi::c_char,
            word,
            &raw mut workw as *mut ::core::ffi::c_char,
            &raw mut workp as *mut ::core::ffi::c_char,
        );
        if workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 'a' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'e' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'i' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'o' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'u' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'A' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'E' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'I' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'O' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'U' as i32
            || (workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'h' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'w' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'H' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'W' as i32)
        {
            addbreath(&raw mut workrest as *mut ::core::ffi::c_char, SMOOTHBR);
            if !(lastc == 'p' as i32 || lastc == 't' as i32 || lastc == 'k' as i32
                || (lastc == 'b' as i32 || lastc == 'd' as i32 || lastc == 'g' as i32))
            {
                *brflg = YES as bool_0;
            }
        } else if workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 'r' as i32
        {
            if workrest[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'r' as i32
            {
                strsqz(
                    &raw mut workrest as *mut ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                );
            }
            cinsert(
                ROUGHBR,
                (&raw mut workrest as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
        }
        Xstrncpy(
            word,
            &raw mut workrest as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        return 1 as bool_0;
    }
    if 0 as ::core::ffi::c_int
        == strcmp(
            &raw mut workp as *mut ::core::ffi::c_char,
            b"e)k\0" as *const u8 as *const ::core::ffi::c_char,
        )
        && 0 as ::core::ffi::c_int
            == Xstrncmp(
                &raw mut workw as *mut ::core::ffi::c_char,
                b"e)c\0" as *const u8 as *const ::core::ffi::c_char,
                3 as size_t,
            )
    {
        getrest(
            &raw mut workrest as *mut ::core::ffi::c_char,
            word,
            &raw mut workw as *mut ::core::ffi::c_char,
            &raw mut workp as *mut ::core::ffi::c_char,
        );
        if workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 'a' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'e' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'i' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'o' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'u' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'A' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'E' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'I' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'O' as i32
            || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'U' as i32
            || (workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'h' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'w' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'H' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'W' as i32)
        {
            addbreath(&raw mut workrest as *mut ::core::ffi::c_char, SMOOTHBR);
            *brflg = YES as bool_0;
            Xstrncpy(
                word,
                &raw mut workrest as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            return 1 as bool_0;
        }
    }
    if lastc == 'p' as i32 || lastc == 't' as i32 || lastc == 'k' as i32
        || (lastc == 'b' as i32 || lastc == 'd' as i32 || lastc == 'g' as i32)
    {
        aspirate(
            (&raw mut workp as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut workp as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
        );
        if 0 as ::core::ffi::c_int
            == Xstrncmp(
                &raw mut workp as *mut ::core::ffi::c_char,
                &raw mut workw as *mut ::core::ffi::c_char,
                Xstrlen(&raw mut workp as *mut ::core::ffi::c_char) as size_t,
            )
        {
            getrest(
                &raw mut workrest as *mut ::core::ffi::c_char,
                word,
                &raw mut workw as *mut ::core::ffi::c_char,
                &raw mut workp as *mut ::core::ffi::c_char,
            );
            if workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 'a' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'e' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'i' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'o' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'u' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'A' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'E' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'I' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'O' as i32
                || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'U' as i32
                || (workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'h' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'w' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'H' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'W' as i32)
            {
                addbreath(&raw mut workrest as *mut ::core::ffi::c_char, ROUGHBR);
                Xstrncpy(
                    word,
                    &raw mut workrest as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
                return 1 as bool_0;
            }
        }
    }
    if (lastc == 'n' as i32 || lastc == 'g' as i32 || lastc == 'r' as i32
        || lastc == 'l' as i32)
        && 0 as ::core::ffi::c_int
            == Xstrncmp(
                &raw mut workp as *mut ::core::ffi::c_char,
                &raw mut workw as *mut ::core::ffi::c_char,
                (Xstrlen(&raw mut workp as *mut ::core::ffi::c_char)
                    - 1 as ::core::ffi::c_int) as size_t,
            )
    {
        Xstrncpy(
            &raw mut workp as *mut ::core::ffi::c_char,
            &raw mut workw as *mut ::core::ffi::c_char,
            Xstrlen(&raw mut workp as *mut ::core::ffi::c_char) as size_t,
        );
        getrest(
            &raw mut workrest as *mut ::core::ffi::c_char,
            word,
            &raw mut workw as *mut ::core::ffi::c_char,
            &raw mut workp as *mut ::core::ffi::c_char,
        );
        match lastc {
            109 => {
                if workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'p' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'b' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'f' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'm' as i32
                {
                    Xstrncpy(
                        word,
                        &raw mut workrest as *mut ::core::ffi::c_char,
                        MAXWORDSIZE as size_t,
                    );
                    return 1 as bool_0;
                }
            }
            103 => {
                if workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'k' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'g' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'x' as i32
                    || workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        == 'c' as i32
                {
                    Xstrncpy(
                        word,
                        &raw mut workrest as *mut ::core::ffi::c_char,
                        MAXWORDSIZE as size_t,
                    );
                    return 1 as bool_0;
                }
            }
            108 => {
                if workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'l' as i32
                {
                    Xstrncpy(
                        word,
                        &raw mut workrest as *mut ::core::ffi::c_char,
                        MAXWORDSIZE as size_t,
                    );
                    return 1 as bool_0;
                }
            }
            114 => {
                if workrest[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 'r' as i32
                {
                    cinsert(
                        ROUGHBR,
                        (&raw mut workrest as *mut ::core::ffi::c_char)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char,
                    );
                    Xstrncpy(
                        word,
                        &raw mut workrest as *mut ::core::ffi::c_char,
                        MAXWORDSIZE as size_t,
                    );
                    return 1 as bool_0;
                }
            }
            _ => {}
        }
    }
    return 0 as bool_0;
}
#[no_mangle]
pub unsafe extern "C" fn getrest(
    mut workrest: *mut ::core::ffi::c_char,
    mut word: *mut ::core::ffi::c_char,
    mut workw: *mut ::core::ffi::c_char,
    mut workp: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut p2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    p1 = workw.offset(Xstrlen(workp) as isize);
    p2 = word.offset(Xstrlen(word) as isize).offset(-(Xstrlen(p1) as isize));
    if 0 as ::core::ffi::c_int == strcmp(p1, p2) {
        Xstrncpy(workrest, p2, MAXWORDSIZE as size_t);
    } else {
        Xstrncpy(
            workrest,
            p2.offset(-(1 as ::core::ffi::c_int as isize)),
            MAXWORDSIZE as size_t,
        );
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn rstprevb(
    mut word: *mut ::core::ffi::c_char,
    mut prevb: *mut ::core::ffi::c_char,
    mut gstr: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut max: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut work: [::core::ffi::c_char; 60] = [0; 60];
    let mut fullpb: [::core::ffi::c_char; 60] = [0; 60];
    let mut tmpword: [::core::ffi::c_char; 60] = [0; 60];
    let mut TmpGstr: gk_string = gk_string {
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
    let mut oddpb: *mut MorphFlags = &raw mut (*gstr).gs_morphflags as *mut MorphFlags;
    fullpb[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    if has_morphflag(oddpb, INDECLFORM) != 0 {
        return 0;
    }
    if cur_lang() == LATIN || cur_lang() == ITALIAN {
        if has_morphflag(oddpb, RAW_PREVERB) == 0 {
            let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            strcpy(&raw mut work as *mut ::core::ffi::c_char, prevb);
            t = (&raw mut work as *mut ::core::ffi::c_char)
                .offset(strlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize));
            if strcmp(prevb, b"circum\0" as *const u8 as *const ::core::ffi::c_char) == 0
                && *word as ::core::ffi::c_int == 'i' as i32
            {
                *t = 0 as ::core::ffi::c_char;
            }
            if has_morphflag(oddpb, D_PREVB) != 0 {
                strcpy(&raw mut work as *mut ::core::ffi::c_char, prevb);
                strcat(
                    &raw mut work as *mut ::core::ffi::c_char,
                    b"d\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if has_morphflag(oddpb, T_PREVB) != 0
                && strcmp(prevb, b"re\0" as *const u8 as *const ::core::ffi::c_char) == 0
            {
                strcpy(
                    &raw mut work as *mut ::core::ffi::c_char,
                    b"ret\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if strcmp(prevb, b"sub\0" as *const u8 as *const ::core::ffi::c_char) == 0
                || strcmp(prevb, b"ob\0" as *const u8 as *const ::core::ffi::c_char) == 0
            {
                match *word as ::core::ffi::c_int {
                    99 | 102 | 103 | 112 => {
                        *t = *word;
                    }
                    _ => {}
                }
            }
            if strcmp(prevb, b"ex\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                if strchr(
                        b"aeioucpqt\0" as *const u8 as *const ::core::ffi::c_char,
                        *word as ::core::ffi::c_int,
                    )
                    .is_null()
                {
                    if *word as ::core::ffi::c_int == 'f' as i32 {
                        *t = *word;
                    } else {
                        *t = '_' as i32 as ::core::ffi::c_char;
                    }
                }
            }
            if strcmp(prevb, b"trans\0" as *const u8 as *const ::core::ffi::c_char) == 0
            {
                match *word as ::core::ffi::c_int {
                    105 | 106 | 100 | 108 | 109 | 110 => {
                        *t.offset(-(1 as ::core::ffi::c_int as isize)) = 0
                            as ::core::ffi::c_char;
                    }
                    115 => {
                        *t = 0 as ::core::ffi::c_char;
                    }
                    _ => {}
                }
            }
            if strcmp(prevb, b"dis\0" as *const u8 as *const ::core::ffi::c_char) == 0
                && strncmp(
                    word,
                    b"di\0" as *const u8 as *const ::core::ffi::c_char,
                    2 as size_t,
                ) != 0
            {
                match *word as ::core::ffi::c_int {
                    98 | 100 | 103 | 108 | 109 | 110 | 114 | 118 => {
                        *t = 0 as ::core::ffi::c_char;
                    }
                    102 => {
                        *t = 'f' as i32 as ::core::ffi::c_char;
                    }
                    _ => {}
                }
            }
            if strcmp(prevb, b"sub\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                match *word as ::core::ffi::c_int {
                    114 | 109 => {
                        *t = *word;
                    }
                    115 => {
                        *t = 0 as ::core::ffi::c_char;
                    }
                    _ => {}
                }
            }
            if strcmp(prevb, b"in\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                match *word as ::core::ffi::c_int {
                    108 => {
                        *t = 'l' as i32 as ::core::ffi::c_char;
                    }
                    98 | 112 | 109 => {
                        *t = 'm' as i32 as ::core::ffi::c_char;
                    }
                    _ => {}
                }
            }
            if strcmp(prevb, b"con\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                match *word as ::core::ffi::c_int {
                    98 | 112 | 109 => {
                        *t = 'm' as i32 as ::core::ffi::c_char;
                    }
                    114 | 108 => {
                        *t = *word;
                    }
                    110 => {
                        *t = '_' as i32 as ::core::ffi::c_char;
                    }
                    _ => {}
                }
            }
            if strcmp(prevb, b"ad\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                match *word as ::core::ffi::c_int {
                    99 | 102 | 108 | 110 | 112 | 115 | 116 => {
                        *(&raw mut work as *mut ::core::ffi::c_char)
                            .offset(
                                strlen(&raw mut work as *mut ::core::ffi::c_char) as isize,
                            )
                            .offset(-(1 as ::core::ffi::c_int as isize)) = *word;
                    }
                    103 => {
                        *t = 0 as ::core::ffi::c_char;
                    }
                    _ => {}
                }
            }
            strcat(&raw mut work as *mut ::core::ffi::c_char, word);
            strcpy(word, &raw mut work as *mut ::core::ffi::c_char);
            return 0;
        }
    }
    if has_morphflag(oddpb, DOUBLED_CONS) != 0 {
        i = 0 as ::core::ffi::c_int;
        while *word as ::core::ffi::c_int != 0
            && !((if 0 as ::core::ffi::c_int != 0 {
                isalpha(*word as ::core::ffi::c_int)
            } else {
                ((*word as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                    .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
            }) != 0 && *word as ::core::ffi::c_int != 'j' as i32
                && *word as ::core::ffi::c_int != 'v' as i32
                && *word as ::core::ffi::c_int != 'J' as i32
                && *word as ::core::ffi::c_int != 'V' as i32
                && !(*word as ::core::ffi::c_int == 'a' as i32
                    || *word as ::core::ffi::c_int == 'e' as i32
                    || *word as ::core::ffi::c_int == 'i' as i32
                    || *word as ::core::ffi::c_int == 'o' as i32
                    || *word as ::core::ffi::c_int == 'u' as i32
                    || *word as ::core::ffi::c_int == 'A' as i32
                    || *word as ::core::ffi::c_int == 'E' as i32
                    || *word as ::core::ffi::c_int == 'I' as i32
                    || *word as ::core::ffi::c_int == 'O' as i32
                    || *word as ::core::ffi::c_int == 'U' as i32
                    || (*word as ::core::ffi::c_int == 'h' as i32
                        || *word as ::core::ffi::c_int == 'w' as i32
                        || *word as ::core::ffi::c_int == 'H' as i32
                        || *word as ::core::ffi::c_int == 'W' as i32)))
        {
            let fresh0 = word;
            word = word.offset(1);
            tmpword[i as usize] = *fresh0;
            i += 1;
        }
        tmpword[i as usize] = *word;
        i += 1;
        Xstrncpy(
            (&raw mut tmpword as *mut ::core::ffi::c_char).offset(i as isize),
            word,
            MAXWORDSIZE as size_t,
        );
    } else {
        Xstrncpy(
            &raw mut tmpword as *mut ::core::ffi::c_char,
            word,
            MAXWORDSIZE as size_t,
        );
    }
    if has_morphflag(oddpb, RAW_PREVERB) != 0 {
        Xstrncpy(
            &raw mut work as *mut ::core::ffi::c_char,
            prevb,
            MAXWORDSIZE as size_t,
        );
        comp_preverb(
            &raw mut work as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            oddpb,
        );
        if (*(&raw mut work as *mut ::core::ffi::c_char)
            .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'a' as i32
            || *(&raw mut work as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'e' as i32
            || *(&raw mut work as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'i' as i32
            || *(&raw mut work as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'o' as i32
            || *(&raw mut work as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'u' as i32
            || *(&raw mut work as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'A' as i32
            || *(&raw mut work as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'E' as i32
            || *(&raw mut work as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'I' as i32
            || *(&raw mut work as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'O' as i32
            || *(&raw mut work as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'U' as i32
            || (*(&raw mut work as *mut ::core::ffi::c_char)
                .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'h' as i32
                || *(&raw mut work as *mut ::core::ffi::c_char)
                    .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'w' as i32
                || *(&raw mut work as *mut ::core::ffi::c_char)
                    .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'H' as i32
                || *(&raw mut work as *mut ::core::ffi::c_char)
                    .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'W' as i32))
            && (*word as ::core::ffi::c_int == 'a' as i32
                || *word as ::core::ffi::c_int == 'e' as i32
                || *word as ::core::ffi::c_int == 'i' as i32
                || *word as ::core::ffi::c_int == 'o' as i32
                || *word as ::core::ffi::c_int == 'u' as i32
                || *word as ::core::ffi::c_int == 'A' as i32
                || *word as ::core::ffi::c_int == 'E' as i32
                || *word as ::core::ffi::c_int == 'I' as i32
                || *word as ::core::ffi::c_int == 'O' as i32
                || *word as ::core::ffi::c_int == 'U' as i32
                || (*word as ::core::ffi::c_int == 'h' as i32
                    || *word as ::core::ffi::c_int == 'w' as i32
                    || *word as ::core::ffi::c_int == 'H' as i32
                    || *word as ::core::ffi::c_int == 'W' as i32))
        {
            strcat(
                &raw mut work as *mut ::core::ffi::c_char,
                b"+\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        set_odd_prvb(oddpb, &raw mut work as *mut ::core::ffi::c_char);
        stripbreath(&raw mut tmpword as *mut ::core::ffi::c_char);
        Xstrncat(
            &raw mut work as *mut ::core::ffi::c_char,
            &raw mut tmpword as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
        Xstrncpy(word, &raw mut work as *mut ::core::ffi::c_char, MAXWORDSIZE as size_t);
        return 0;
    }
    set_morphflag(
        &raw mut TmpGstr.gs_morphflags as *mut MorphFlags,
        0 as ::core::ffi::c_int,
    );
    exp_preverb(prevb, &raw mut fullpb as *mut ::core::ffi::c_char, &raw mut TmpGstr);
    if fullpb[0 as ::core::ffi::c_int as usize] != 0 {
        Xstrncpy(
            &raw mut work as *mut ::core::ffi::c_char,
            &raw mut fullpb as *mut ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    } else {
        Xstrncpy(
            &raw mut work as *mut ::core::ffi::c_char,
            prevb,
            MAXWORDSIZE as size_t,
        );
    }
    stripacc(&raw mut work as *mut ::core::ffi::c_char);
    if has_morphflag(oddpb, APOCOPE) != 0 {
        let mut s: *mut ::core::ffi::c_char = (&raw mut work as *mut ::core::ffi::c_char)
            .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        if *s as ::core::ffi::c_int == 'a' as i32
            || *s as ::core::ffi::c_int == 'o' as i32
        {
            *s = 0 as ::core::ffi::c_char;
        }
    }
    if has_morphflag(oddpb, IOTA_INTENS) != 0
        && *(&raw mut work as *mut ::core::ffi::c_char)
            .offset(Xstrlen(&raw mut work as *mut ::core::ffi::c_char) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'n' as i32
    {
        strcat(
            &raw mut work as *mut ::core::ffi::c_char,
            b"i\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    comp_preverb(
        &raw mut work as *mut ::core::ffi::c_char,
        has_morphflag(oddpb, UNASP_PREVERB),
        oddpb,
    );
    set_odd_prvb(oddpb, &raw mut work as *mut ::core::ffi::c_char);
    getprvbform(
        &raw mut tmpword as *mut ::core::ffi::c_char,
        &raw mut work as *mut ::core::ffi::c_char,
        oddpb,
    );
    max = MAXWORDSIZE - Xstrlen(&raw mut work as *mut ::core::ffi::c_char);
    if Xstrlen(&raw mut tmpword as *mut ::core::ffi::c_char) >= max {
        tmpword[(max - 1 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_char;
        tmpword[(max - 2 as ::core::ffi::c_int) as usize] = CONTCHAR
            as ::core::ffi::c_char;
    }
    stripbreath(&raw mut tmpword as *mut ::core::ffi::c_char);
    Xstrncat(
        &raw mut work as *mut ::core::ffi::c_char,
        &raw mut tmpword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    Xstrncpy(word, &raw mut work as *mut ::core::ffi::c_char, MAXWORDSIZE as size_t);
    return 0;
}
unsafe extern "C" fn comp_preverb(
    mut pb: *mut ::core::ffi::c_char,
    mut unasp: ::core::ffi::c_int,
    mut oddpb: *mut MorphFlags,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
    let mut added_aug: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    s = pb;
    set_morphflag(&raw mut Gstr.gs_morphflags as *mut MorphFlags, HAS_PREVERB);
    if unasp != 0 {
        add_morphflag(&raw mut Gstr.gs_morphflags as *mut MorphFlags, UNASP_PREVERB);
    }
    if has_morphflag(oddpb, EN_TO_ENI) != 0 {
        add_morphflag(&raw mut Gstr.gs_morphflags as *mut MorphFlags, EN_TO_ENI);
    }
    if strchr(s, ',' as i32).is_null() && has_morphflag(oddpb, PREVB_AUGMENT) != 0
        && added_aug == 0
    {
        add_double_augment(s, oddpb);
        added_aug += 1;
    }
    while *s != 0 {
        if *s as ::core::ffi::c_int == ',' as i32 {
            *s = 0 as ::core::ffi::c_char;
            s = s.offset(1);
            if strchr(s, ',' as i32).is_null()
                && has_morphflag(oddpb, PREVB_AUGMENT) != 0 && added_aug == 0
            {
                add_double_augment(s, oddpb);
                added_aug += 1;
            }
            rstprevb(s, pb, &raw mut Gstr);
            Xstrncpy(pb, s, MAXWORDSIZE as size_t);
            s = pb;
        } else {
            s = s.offset(1);
        }
    }
    return 0;
}
unsafe extern "C" fn getprvbform(
    mut word: *mut ::core::ffi::c_char,
    mut prevb: *mut ::core::ffi::c_char,
    mut oddpb: *mut MorphFlags,
) -> ::core::ffi::c_int {
    stripacc(prevb);
    if verbose == 0 {
        if (strcmp(
            prevb
                .offset(Xstrlen(prevb) as isize)
                .offset(-(4 as ::core::ffi::c_int as isize)),
            b"peri\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
            || strcmp(
                prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(5 as ::core::ffi::c_int as isize)),
                b"proti\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0) && has_morphflag(oddpb, ELIDE_PREVERB) != 0
        {
            *prevb
                .offset(Xstrlen(prevb) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) = 0 as ::core::ffi::c_char;
        } else if strcmp(
            prevb
                .offset(Xstrlen(prevb) as isize)
                .offset(-(3 as ::core::ffi::c_int as isize)),
            b"mfi\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            if has_morphflag(oddpb, DISSIMILATION) != 0 {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(2 as ::core::ffi::c_int as isize)) = 'p' as i32
                    as ::core::ffi::c_char;
                if *word as ::core::ffi::c_int == 'a' as i32
                    || *word as ::core::ffi::c_int == 'e' as i32
                    || *word as ::core::ffi::c_int == 'i' as i32
                    || *word as ::core::ffi::c_int == 'o' as i32
                    || *word as ::core::ffi::c_int == 'u' as i32
                    || *word as ::core::ffi::c_int == 'A' as i32
                    || *word as ::core::ffi::c_int == 'E' as i32
                    || *word as ::core::ffi::c_int == 'I' as i32
                    || *word as ::core::ffi::c_int == 'O' as i32
                    || *word as ::core::ffi::c_int == 'U' as i32
                    || (*word as ::core::ffi::c_int == 'h' as i32
                        || *word as ::core::ffi::c_int == 'w' as i32
                        || *word as ::core::ffi::c_int == 'H' as i32
                        || *word as ::core::ffi::c_int == 'W' as i32)
                {
                    strsqz(
                        prevb
                            .offset(Xstrlen(prevb) as isize)
                            .offset(-(1 as ::core::ffi::c_int as isize)),
                        1 as ::core::ffi::c_int,
                    );
                }
                if getbreath(word) == ROUGHBR {
                    *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize)) = 'f' as i32
                        as ::core::ffi::c_char;
                    zap_morphflag(oddpb, DISSIMILATION);
                }
            } else if *word as ::core::ffi::c_int == 'i' as i32
                || has_morphflag(oddpb, ELIDE_PREVERB) != 0
            {
                strsqz(
                    prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize)),
                    1 as ::core::ffi::c_int,
                );
            }
        } else if strcmp(
            prevb
                .offset(Xstrlen(prevb) as isize)
                .offset(-(3 as ::core::ffi::c_int as isize)),
            b"nti\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
            && (*word as ::core::ffi::c_int == 'a' as i32
                || *word as ::core::ffi::c_int == 'e' as i32
                || *word as ::core::ffi::c_int == 'i' as i32
                || *word as ::core::ffi::c_int == 'o' as i32
                || *word as ::core::ffi::c_int == 'u' as i32
                || *word as ::core::ffi::c_int == 'A' as i32
                || *word as ::core::ffi::c_int == 'E' as i32
                || *word as ::core::ffi::c_int == 'I' as i32
                || *word as ::core::ffi::c_int == 'O' as i32
                || *word as ::core::ffi::c_int == 'U' as i32
                || (*word as ::core::ffi::c_int == 'h' as i32
                    || *word as ::core::ffi::c_int == 'w' as i32
                    || *word as ::core::ffi::c_int == 'H' as i32
                    || *word as ::core::ffi::c_int == 'W' as i32))
        {
            if *word as ::core::ffi::c_int != 'o' as i32 {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 0
                    as ::core::ffi::c_char;
            } else if *word as ::core::ffi::c_int == 'o' as i32
                && has_morphflag(oddpb, ELIDE_PREVERB) != 0
            {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 0
                    as ::core::ffi::c_char;
            } else if has_morphflag(oddpb, ELIDE_PREVERB) != 0 {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 0
                    as ::core::ffi::c_char;
            }
            if *prevb
                .offset(Xstrlen(prevb) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 't' as i32 && getbreath(word) == ROUGHBR
                && has_morphflag(oddpb, UNASP_PREVERB) == 0
            {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 'q' as i32
                    as ::core::ffi::c_char;
            }
        } else if *word as ::core::ffi::c_int == 'a' as i32
            || *word as ::core::ffi::c_int == 'e' as i32
            || *word as ::core::ffi::c_int == 'i' as i32
            || *word as ::core::ffi::c_int == 'o' as i32
            || *word as ::core::ffi::c_int == 'u' as i32
            || *word as ::core::ffi::c_int == 'A' as i32
            || *word as ::core::ffi::c_int == 'E' as i32
            || *word as ::core::ffi::c_int == 'I' as i32
            || *word as ::core::ffi::c_int == 'O' as i32
            || *word as ::core::ffi::c_int == 'U' as i32
            || (*word as ::core::ffi::c_int == 'h' as i32
                || *word as ::core::ffi::c_int == 'w' as i32
                || *word as ::core::ffi::c_int == 'H' as i32
                || *word as ::core::ffi::c_int == 'W' as i32)
        {
            if strcmp(
                prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(3 as ::core::ffi::c_int as isize)),
                b"pro\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0
                && strcmp(
                    prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize)),
                    b"peri\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                && strcmp(
                    prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(5 as ::core::ffi::c_int as isize)),
                    b"proti\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
            {
                if *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'a' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'e' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'i' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'o' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'u' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'A' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'E' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'I' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'O' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'U' as i32
                    || (*prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'h' as i32
                        || *prevb
                            .offset(Xstrlen(prevb) as isize)
                            .offset(-(1 as ::core::ffi::c_int as isize))
                            as ::core::ffi::c_int == 'w' as i32
                        || *prevb
                            .offset(Xstrlen(prevb) as isize)
                            .offset(-(1 as ::core::ffi::c_int as isize))
                            as ::core::ffi::c_int == 'H' as i32
                        || *prevb
                            .offset(Xstrlen(prevb) as isize)
                            .offset(-(1 as ::core::ffi::c_int as isize))
                            as ::core::ffi::c_int == 'W' as i32)
                {
                    if *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int != 'i' as i32
                    {
                        strsqz(
                            prevb
                                .offset(Xstrlen(prevb) as isize)
                                .offset(-(1 as ::core::ffi::c_int as isize)),
                            1 as ::core::ffi::c_int,
                        );
                    } else if *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(2 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int != 'd' as i32
                    {
                        strsqz(
                            prevb
                                .offset(Xstrlen(prevb) as isize)
                                .offset(-(1 as ::core::ffi::c_int as isize)),
                            1 as ::core::ffi::c_int,
                        );
                    }
                }
                if *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'k' as i32
                {
                    *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize)) = 'c' as i32
                        as ::core::ffi::c_char;
                } else if (*prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'p' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 't' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'k' as i32
                    || (*prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'b' as i32
                        || *prevb
                            .offset(Xstrlen(prevb) as isize)
                            .offset(-(1 as ::core::ffi::c_int as isize))
                            as ::core::ffi::c_int == 'd' as i32
                        || *prevb
                            .offset(Xstrlen(prevb) as isize)
                            .offset(-(1 as ::core::ffi::c_int as isize))
                            as ::core::ffi::c_int == 'g' as i32))
                    && getbreath(word) == ROUGHBR
                    && has_morphflag(oddpb, UNASP_PREVERB) == 0
                {
                    aspirate(
                        prevb
                            .offset(Xstrlen(prevb) as isize)
                            .offset(-(1 as ::core::ffi::c_int as isize)),
                    );
                }
            } else {
                strcat(prevb, b"+\0" as *const u8 as *const ::core::ffi::c_char);
            }
        } else if *prevb
            .offset(Xstrlen(prevb) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 't' as i32
        {
            if *word as ::core::ffi::c_int == 'f' as i32 {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 'p' as i32
                    as ::core::ffi::c_char;
            }
            if *word as ::core::ffi::c_int == 'd' as i32 {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 'd' as i32
                    as ::core::ffi::c_char;
            }
            if *word as ::core::ffi::c_int == 'n' as i32 {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 'n' as i32
                    as ::core::ffi::c_char;
            } else if *word as ::core::ffi::c_int == 's' as i32 {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 0
                    as ::core::ffi::c_char;
            } else if *word as ::core::ffi::c_int == 'p' as i32
                || *word as ::core::ffi::c_int == 'b' as i32
                || *word as ::core::ffi::c_int == 'f' as i32
                || (*word as ::core::ffi::c_int == 'l' as i32
                    || *word as ::core::ffi::c_int == 'r' as i32)
                || *word as ::core::ffi::c_int == 'p' as i32
                || *word as ::core::ffi::c_int == 'b' as i32
                || *word as ::core::ffi::c_int == 'k' as i32
            {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = *word;
            }
        } else if *prevb
            .offset(Xstrlen(prevb) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'p' as i32
        {
            if *word as ::core::ffi::c_int == 'b' as i32 {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 'b' as i32
                    as ::core::ffi::c_char;
            }
        } else if *prevb
            .offset(Xstrlen(prevb) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'n' as i32
        {
            if *word as ::core::ffi::c_int == 'p' as i32
                || *word as ::core::ffi::c_int == 'b' as i32
                || *word as ::core::ffi::c_int == 'f' as i32
                || *word as ::core::ffi::c_int == 'm' as i32
                || *word as ::core::ffi::c_int == 'y' as i32
            {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 'm' as i32
                    as ::core::ffi::c_char;
            } else if *word as ::core::ffi::c_int == 'k' as i32
                || *word as ::core::ffi::c_int == 'g' as i32
                || *word as ::core::ffi::c_int == 'x' as i32
                || *word as ::core::ffi::c_int == 'c' as i32
            {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = 'g' as i32
                    as ::core::ffi::c_char;
            } else if *word as ::core::ffi::c_int == 'r' as i32
                || *word as ::core::ffi::c_int == 'l' as i32
            {
                *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) = *word;
            } else if (*word as ::core::ffi::c_int == 's' as i32
                || *word as ::core::ffi::c_int == 'z' as i32)
                && *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(2 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'u' as i32
            {
                if *word.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'p' as i32
                    || *word.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 't' as i32
                    || *word.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'k' as i32
                    || (*word.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'b' as i32
                        || *word.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 'd' as i32
                        || *word.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 'g' as i32)
                    || (*word.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'f' as i32
                        || *word.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 'q' as i32
                        || *word.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int == 'x' as i32)
                    || *word as ::core::ffi::c_int == 'z' as i32
                {
                    *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize)) = 0
                        as ::core::ffi::c_char;
                } else {
                    *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize)) = 's' as i32
                        as ::core::ffi::c_char;
                }
            }
        } else if *word as ::core::ffi::c_int == 'r' as i32
            && (*prevb
                .offset(Xstrlen(prevb) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'a' as i32
                || *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'e' as i32
                || *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'i' as i32
                || *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'o' as i32
                || *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'u' as i32
                || *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'A' as i32
                || *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'E' as i32
                || *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'I' as i32
                || *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'O' as i32
                || *prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'U' as i32
                || (*prevb
                    .offset(Xstrlen(prevb) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'h' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'w' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'H' as i32
                    || *prevb
                        .offset(Xstrlen(prevb) as isize)
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        as ::core::ffi::c_int == 'W' as i32))
            && has_morphflag(oddpb, RAW_PREVERB) == 0
        {
            cinsert('r' as i32, word);
        }
        if getbreath(word) != NOBREATH {
            stripbreath(word);
        }
    } else {
        Xstrncat(
            prevb,
            b" + \0" as *const u8 as *const ::core::ffi::c_char,
            MAXWORDSIZE as size_t,
        );
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn First_K_aspirate(
    mut word: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *word as ::core::ffi::c_int != 0
        && !((if 0 as ::core::ffi::c_int != 0 {
            isalpha(*word as ::core::ffi::c_int)
        } else {
            ((*word as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) != 0 && *word as ::core::ffi::c_int != 'j' as i32
            && *word as ::core::ffi::c_int != 'v' as i32
            && *word as ::core::ffi::c_int != 'J' as i32
            && *word as ::core::ffi::c_int != 'V' as i32
            && !(*word as ::core::ffi::c_int == 'a' as i32
                || *word as ::core::ffi::c_int == 'e' as i32
                || *word as ::core::ffi::c_int == 'i' as i32
                || *word as ::core::ffi::c_int == 'o' as i32
                || *word as ::core::ffi::c_int == 'u' as i32
                || *word as ::core::ffi::c_int == 'A' as i32
                || *word as ::core::ffi::c_int == 'E' as i32
                || *word as ::core::ffi::c_int == 'I' as i32
                || *word as ::core::ffi::c_int == 'O' as i32
                || *word as ::core::ffi::c_int == 'U' as i32
                || (*word as ::core::ffi::c_int == 'h' as i32
                    || *word as ::core::ffi::c_int == 'w' as i32
                    || *word as ::core::ffi::c_int == 'H' as i32
                    || *word as ::core::ffi::c_int == 'W' as i32)))
    {
        word = word.offset(1);
    }
    if *word == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if *word as ::core::ffi::c_int == 'f' as i32
        || *word as ::core::ffi::c_int == 'q' as i32
        || *word as ::core::ffi::c_int == 'x' as i32
        || (*word.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'f' as i32
            || *word.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'q' as i32
            || *word.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'x' as i32)
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn shift_su_to_cu(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *s != 0 {
        if *s as ::core::ffi::c_int == 's' as i32
            && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'u' as i32
        {
            *s = 'c' as i32 as ::core::ffi::c_char;
        }
        s = s.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn shift_eis_to_es(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *s != 0 {
        if Xstrncmp(s, b"eis\0" as *const u8 as *const ::core::ffi::c_char, 3 as size_t)
            == 0
            || Xstrncmp(
                s,
                b"ei)s\0" as *const u8 as *const ::core::ffi::c_char,
                4 as size_t,
            ) == 0
        {
            strcpy(
                s.offset(1 as ::core::ffi::c_int as isize),
                s.offset(2 as ::core::ffi::c_int as isize),
            );
            return 0;
        }
        s = s.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn shift_pros_to_poti(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    while *s != 0 {
        if Xstrncmp(s, b"pros\0" as *const u8 as *const ::core::ffi::c_char, 4 as size_t)
            == 0
        {
            strcpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                s.offset(4 as ::core::ffi::c_int as isize),
            );
            strcpy(s, b"poti\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(s, &raw mut tmp as *mut ::core::ffi::c_char);
            return 0;
        }
        s = s.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn shift_pros_to_proti(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    while *s != 0 {
        if Xstrncmp(s, b"pros\0" as *const u8 as *const ::core::ffi::c_char, 4 as size_t)
            == 0
        {
            strcpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                s.offset(4 as ::core::ffi::c_int as isize),
            );
            strcpy(s, b"proti\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(s, &raw mut tmp as *mut ::core::ffi::c_char);
            return 0;
        }
        s = s.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn shift_upo_to_upai(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    while *s != 0 {
        if Xstrncmp(s, b"upo\0" as *const u8 as *const ::core::ffi::c_char, 3 as size_t)
            == 0
        {
            strcpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                s.offset(3 as ::core::ffi::c_int as isize),
            );
            strcpy(s, b"upai\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(s, &raw mut tmp as *mut ::core::ffi::c_char);
            return 0;
        } else if Xstrncmp(
            s,
            b"u(po\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0
        {
            strcpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                s.offset(4 as ::core::ffi::c_int as isize),
            );
            strcpy(s, b"u(pai\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(s, &raw mut tmp as *mut ::core::ffi::c_char);
            return 0;
        }
        s = s.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn shift_uper_to_upeir(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    while *s != 0 {
        if Xstrncmp(s, b"uper\0" as *const u8 as *const ::core::ffi::c_char, 4 as size_t)
            == 0
        {
            strcpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                s.offset(4 as ::core::ffi::c_int as isize),
            );
            strcpy(s, b"upeir\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(s, &raw mut tmp as *mut ::core::ffi::c_char);
            return 0;
        } else if Xstrncmp(
            s,
            b"u(per\0" as *const u8 as *const ::core::ffi::c_char,
            5 as size_t,
        ) == 0
        {
            strcpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                s.offset(5 as ::core::ffi::c_int as isize),
            );
            strcpy(s, b"u(peir\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(s, &raw mut tmp as *mut ::core::ffi::c_char);
            return 0;
        }
        s = s.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn shift_para_to_parai(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    while *s != 0 {
        if Xstrncmp(s, b"para\0" as *const u8 as *const ::core::ffi::c_char, 4 as size_t)
            == 0
        {
            strcpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                s.offset(4 as ::core::ffi::c_int as isize),
            );
            strcpy(s, b"parai\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(s, &raw mut tmp as *mut ::core::ffi::c_char);
            return 0;
        }
        s = s.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn shift_meta_to_peda(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    while *s != 0 {
        if Xstrncmp(s, b"meta\0" as *const u8 as *const ::core::ffi::c_char, 4 as size_t)
            == 0
        {
            strcpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                s.offset(4 as ::core::ffi::c_int as isize),
            );
            strcpy(s, b"peda\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(s, &raw mut tmp as *mut ::core::ffi::c_char);
            return 0;
        }
        s = s.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn shift_en_to_eni(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    while *s != 0 {
        if Xstrncmp(s, b"e)n\0" as *const u8 as *const ::core::ffi::c_char, 3 as size_t)
            == 0
            && Xstrncmp(
                s,
                b"e)ni\0" as *const u8 as *const ::core::ffi::c_char,
                4 as size_t,
            ) != 0
        {
            strcpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                s.offset(3 as ::core::ffi::c_int as isize),
            );
            strcpy(s, b"e)ni\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(s, &raw mut tmp as *mut ::core::ffi::c_char);
            return 0;
        }
        if Xstrncmp(s, b"en\0" as *const u8 as *const ::core::ffi::c_char, 2 as size_t)
            == 0
            && Xstrncmp(
                s,
                b"eni\0" as *const u8 as *const ::core::ffi::c_char,
                3 as size_t,
            ) != 0
        {
            strcpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                s.offset(2 as ::core::ffi::c_int as isize),
            );
            strcpy(s, b"eni\0" as *const u8 as *const ::core::ffi::c_char);
            strcat(s, &raw mut tmp as *mut ::core::ffi::c_char);
            return 0;
        }
        s = s.offset(1);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_odd_prvb(
    mut oddpb: *mut MorphFlags,
    mut work: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if has_morphflag(oddpb, SIG_TO_CI) != 0 {
        shift_su_to_cu(work);
    }
    if has_morphflag(oddpb, SHORT_EIS) != 0 {
        shift_eis_to_es(work);
    }
    if has_morphflag(oddpb, PROS_TO_POTI) != 0 {
        shift_pros_to_poti(work);
    }
    if has_morphflag(oddpb, PROS_TO_PROTI) != 0 {
        shift_pros_to_proti(work);
    }
    if has_morphflag(oddpb, META_TO_PEDA) != 0 {
        shift_meta_to_peda(work);
    }
    if has_morphflag(oddpb, UPO_TO_UPAI) != 0 {
        shift_upo_to_upai(work);
    }
    if has_morphflag(oddpb, PARA_TO_PARAI) != 0 {
        shift_para_to_parai(work);
    }
    if has_morphflag(oddpb, UPER_TO_UPEIR) != 0 {
        shift_uper_to_upeir(work);
    }
    if has_morphflag(oddpb, EN_TO_ENI) != 0 {
        shift_en_to_eni(work);
    }
    return 0;
}
