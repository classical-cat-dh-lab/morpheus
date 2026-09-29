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
    fn getsyll(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn Xstrncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn nsylls(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripquant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type int32 = ::core::ffi::c_uint;
pub type size_t = usize;
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
pub const DECL3: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const NOUNSTEM: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const P_ERR: *mut ::core::ffi::c_char = -(1 as ::core::ffi::c_int)
    as *mut ::core::ffi::c_char;
pub const DUAL: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const GENITIVE: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const DATIVE: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const ULTIMA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PENULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CONTRACTED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
static mut thirdexceptions: [[::core::ffi::c_char; 60]; 16] = unsafe {
    [
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"paidwn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"paidoin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"dmwwn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"dmwoin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"qwwn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"qwoin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"Trwwn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"Trwoin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"da|doin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"da|dwn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"daidoin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"daidwn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"fwtwn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"fwtoin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"w)twn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<
            [u8; 60],
            [::core::ffi::c_char; 60],
        >(
            *b"w)toin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn is_thirdmono(
    mut stemgstr: *mut gk_string,
    mut endgstr: *mut gk_string,
    mut stem: *mut ::core::ffi::c_char,
    mut endstring: *mut ::core::ffi::c_char,
    mut form_info: word_form,
    mut is_ending: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut stemtype: Stemtype = (*stemgstr).gs_steminfo;
    if is_ending != 0 && *stem == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if stemtype & NOUNSTEM as Stemtype == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if stemtype & DECL3 as Stemtype != 0 && is_mono_stem(stem, endstring) != 0 {
        if has_morphflag(
            &raw mut (*stemgstr).gs_morphflags as *mut MorphFlags,
            CONTRACTED,
        ) != 0
            || has_morphflag(
                &raw mut (*endgstr).gs_morphflags as *mut MorphFlags,
                CONTRACTED,
            ) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if diphth_end(stem, endstring) != 0
            && form_info.f_number() as ::core::ffi::c_int != DUAL
        {
            return 0 as ::core::ffi::c_int;
        }
        if form_info.f_case() as ::core::ffi::c_int & GENITIVE == 0
            && form_info.f_case() as ::core::ffi::c_int & DATIVE == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_mono_stem(
    mut stems: *mut ::core::ffi::c_char,
    mut ends: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return (nsylls(stems) + nsylls(ends) == 2 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_thirdexception(
    mut stem: *mut ::core::ffi::c_char,
    mut endstring: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut workword: [::core::ffi::c_char; 60] = [0; 60];
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    Xstrncpy(&raw mut workword as *mut ::core::ffi::c_char, stem, MAXWORDSIZE as size_t);
    Xstrncat(
        &raw mut workword as *mut ::core::ffi::c_char,
        endstring,
        MAXWORDSIZE as size_t,
    );
    stripquant(&raw mut workword as *mut ::core::ffi::c_char);
    len = (::core::mem::size_of::<[[::core::ffi::c_char; 60]; 16]>() as usize)
        .wrapping_div(::core::mem::size_of::<[::core::ffi::c_char; 60]>() as usize)
        as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        if strcmp(
            &raw mut workword as *mut ::core::ffi::c_char,
            &raw mut *(&raw mut thirdexceptions as *mut [::core::ffi::c_char; 60])
                .offset(i as isize) as *mut ::core::ffi::c_char,
        ) == 0
        {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn poss_thirdmono(
    mut stemtype: Stemtype,
    mut stem: *mut ::core::ffi::c_char,
    mut endstring: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut p2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if stemtype & DECL3 as Stemtype != 0
        && {
            p1 = getsyll(stem, PENULT);
            p1 == P_ERR
        }
        && {
            p2 = getsyll(endstring, PENULT);
            p2 == P_ERR
        }
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn diphth_end(
    mut stem: *mut ::core::ffi::c_char,
    mut endstring: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strcpy(&raw mut tmp as *mut ::core::ffi::c_char, stem);
    strcat(&raw mut tmp as *mut ::core::ffi::c_char, endstring);
    s = getsyll(&raw mut tmp as *mut ::core::ffi::c_char, ULTIMA);
    if s == P_ERR {
        return 0 as ::core::ffi::c_int;
    }
    if (*s as ::core::ffi::c_int == 'a' as i32 || *s as ::core::ffi::c_int == 'e' as i32
        || *s as ::core::ffi::c_int == 'i' as i32
        || *s as ::core::ffi::c_int == 'o' as i32
        || *s as ::core::ffi::c_int == 'u' as i32
        || *s as ::core::ffi::c_int == 'A' as i32
        || *s as ::core::ffi::c_int == 'E' as i32
        || *s as ::core::ffi::c_int == 'I' as i32
        || *s as ::core::ffi::c_int == 'O' as i32
        || *s as ::core::ffi::c_int == 'U' as i32
        || (*s as ::core::ffi::c_int == 'h' as i32
            || *s as ::core::ffi::c_int == 'w' as i32
            || *s as ::core::ffi::c_int == 'H' as i32
            || *s as ::core::ffi::c_int == 'W' as i32))
        && (*s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'a' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'e' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'i' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'o' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'u' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'A' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'E' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'I' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'O' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'U' as i32
            || (*s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'h' as i32
                || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'w' as i32
                || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'H' as i32
                || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'W' as i32))
    {
        *s.offset(-(1 as ::core::ffi::c_int as isize)) = 0 as ::core::ffi::c_char;
        if nsylls(&raw mut tmp as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
