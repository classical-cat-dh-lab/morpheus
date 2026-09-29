use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
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
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn MorphFopen(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> *mut FILE;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn AccComposForm(_: *mut gk_string) -> ::core::ffi::c_int;
    fn AddNewGstr(_: *mut gk_string) -> ::core::ffi::c_int;
    fn CheckForBreathing(_: *mut gk_string) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn add_numovable(_: *mut gk_string) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn do_euph(_: *mut gk_string, _: Dialect) -> *mut gk_string;
    fn fix_eta(_: *mut gk_string) -> *mut gk_string;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn is_blank(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn merge_keys(
        _: *mut gk_string,
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn nextkey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn nsylls(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn poss_contracts(_: *mut gk_string, _: Dialect) -> *mut gk_string;
    fn takes_nu_movable(_: *mut gk_string) -> ::core::ffi::c_int;
    fn zap_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
pub const SUFF_ACC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CONTRACTED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const ACCENT_OPTIONAL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NEEDS_ACCENT: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const DISSIMILATION: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const HAS_AUGMENT: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const INDECLFORM: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const IS_DERIV: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
pub const COMMENT_CHAR: ::core::ffi::c_int = '#' as i32;
static mut AvoidGstr: gk_string = gk_string {
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
pub unsafe extern "C" fn mk_end(
    mut havestr: *mut ::core::ffi::c_char,
    mut Have: *mut gk_string,
    mut Avoid: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut savestr: [::core::ffi::c_char; 60] = [0; 60];
    let mut contr_forms: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut euph_forms: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut saw_vowel: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    strcpy(&raw mut savestr as *mut ::core::ffi::c_char, havestr);
    s = &raw mut savestr as *mut ::core::ffi::c_char;
    while *s != 0 {
        if *s as ::core::ffi::c_int == 'a' as i32
            || *s as ::core::ffi::c_int == 'e' as i32
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
                || *s as ::core::ffi::c_int == 'W' as i32)
        {
            saw_vowel = 1 as ::core::ffi::c_int;
        }
        if *s as ::core::ffi::c_int == '@' as i32 {
            *s = 0 as ::core::ffi::c_char;
            mk_compend(
                Have,
                Avoid,
                &raw mut savestr as *mut ::core::ffi::c_char,
                s.offset(1 as ::core::ffi::c_int as isize),
            );
            return 0;
        }
        s = s.offset(1);
    }
    join_end(Have, &raw mut (*Have).gs_gkstring as *mut ::core::ffi::c_char, saw_vowel);
    euph_forms = fix_eta(Have);
    if !euph_forms.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (*euph_forms.offset(i as isize))
            .gs_gkstring[0 as ::core::ffi::c_int as usize] != 0
        {
            mk_end(
                &raw mut (*euph_forms.offset(i as isize)).gs_gkstring
                    as *mut ::core::ffi::c_char,
                euph_forms.offset(i as isize),
                &raw mut AvoidGstr,
            );
            i += 1;
        }
        FreeGkString(euph_forms);
    }
    euph_forms = do_euph(Have, (*Avoid).gs_dialect);
    if !euph_forms.is_null() {
        let mut i_0: ::core::ffi::c_int = 0;
        i_0 = 0 as ::core::ffi::c_int;
        while (*euph_forms.offset(i_0 as isize))
            .gs_gkstring[0 as ::core::ffi::c_int as usize] != 0
        {
            mk_end(
                &raw mut (*euph_forms.offset(i_0 as isize)).gs_gkstring
                    as *mut ::core::ffi::c_char,
                euph_forms.offset(i_0 as isize),
                &raw mut AvoidGstr,
            );
            i_0 += 1;
        }
        FreeGkString(euph_forms);
        return 0;
    }
    if has_morphflag(&raw mut (*Have).gs_morphflags as *mut MorphFlags, CONTRACTED) == 0
        && {
            contr_forms = poss_contracts(Have, (*Avoid).gs_dialect);
            !contr_forms.is_null()
        }
    {
        let mut i_1: ::core::ffi::c_int = 0;
        i_1 = 0 as ::core::ffi::c_int;
        while (*contr_forms.offset(i_1 as isize))
            .gs_gkstring[0 as ::core::ffi::c_int as usize] != 0
        {
            mk_end(
                &raw mut (*contr_forms.offset(i_1 as isize)).gs_gkstring
                    as *mut ::core::ffi::c_char,
                contr_forms.offset(i_1 as isize),
                &raw mut AvoidGstr,
            );
            i_1 += 1;
        }
        FreeGkString(contr_forms);
    } else {
        if takes_nu_movable(Have) != 0 && cur_lang() != ITALIAN && cur_lang() != LATIN {
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
            TmpGstr = *Have;
            add_numovable(&raw mut TmpGstr);
            mk_end(
                &raw mut TmpGstr.gs_gkstring as *mut ::core::ffi::c_char,
                &raw mut TmpGstr,
                &raw mut AvoidGstr,
            );
        }
        if morph_port_unavailable(
            b"gkends/mkend.c:94:do_dissim\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        {
            add_morphflag(
                &raw mut (*Have).gs_morphflags as *mut MorphFlags,
                DISSIMILATION,
            );
        }
        AddNewGstr(Have);
    }
    return 0;
}
unsafe extern "C" fn mk_compend(
    mut Have: *mut gk_string,
    mut Avoid: *mut gk_string,
    mut curstr: *mut ::core::ffi::c_char,
    mut endtype: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut i: ::core::ffi::c_int = 0;
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut savestem: [::core::ffi::c_char; 60] = [0; 60];
    let mut TmpHave: gk_string = gk_string {
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
    let mut TmpAvoid: gk_string = gk_string {
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
    sprintf(
        &raw mut line as *mut ::core::ffi::c_char,
        b"endtables/basics/%s.end\0" as *const u8 as *const ::core::ffi::c_char,
        endtype,
    );
    f = MorphFopen(
        &raw mut line as *mut ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"could not open [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
            endtype,
        );
        return -(1 as ::core::ffi::c_int);
    }
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            f,
        )
        .is_null()
    {
        let mut curendstr: [::core::ffi::c_char; 60] = [0; 60];
        strcpy(&raw mut savestem as *mut ::core::ffi::c_char, curstr);
        if is_blank(&raw mut line as *mut ::core::ffi::c_char) != 0 {
            continue;
        }
        if *(&raw mut line as *mut ::core::ffi::c_char) as ::core::ffi::c_int
            == COMMENT_CHAR
        {
            continue;
        }
        TmpHave = *Have;
        TmpAvoid = *Avoid;
        nextkey(
            &raw mut line as *mut ::core::ffi::c_char,
            &raw mut curendstr as *mut ::core::ffi::c_char,
        );
        update_end(
            &raw mut TmpHave,
            &raw mut TmpAvoid,
            &raw mut savestem as *mut ::core::ffi::c_char,
            &raw mut curendstr as *mut ::core::ffi::c_char,
            &raw mut line as *mut ::core::ffi::c_char,
        );
    }
    fclose(f);
    return 0;
}
unsafe extern "C" fn update_end(
    mut Have: *mut gk_string,
    mut Avoid: *mut gk_string,
    mut stem: *mut ::core::ffi::c_char,
    mut endstr: *mut ::core::ffi::c_char,
    mut newkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut savestem: [::core::ffi::c_char; 60] = [0; 60];
    strcpy(&raw mut savestem as *mut ::core::ffi::c_char, stem);
    if merge_keys(
        Have,
        Avoid,
        &raw mut savestem as *mut ::core::ffi::c_char,
        endstr,
        newkeys,
    ) != 0
    {
        Xstrncpy(
            &raw mut (*Have).gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut savestem as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        CompStemEnd(
            Have,
            &raw mut (*Have).gs_gkstring as *mut ::core::ffi::c_char,
            endstr,
        );
        strcat(&raw mut (*Have).gs_gkstring as *mut ::core::ffi::c_char, endstr);
        mk_end(&raw mut (*Have).gs_gkstring as *mut ::core::ffi::c_char, Have, Avoid);
    }
    return 0;
}
unsafe extern "C" fn join_end(
    mut Have: *mut gk_string,
    mut stem: *mut ::core::ffi::c_char,
    mut saw_vowel: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
    Xstrncpy(
        &raw mut (*Have).gs_gkstring as *mut ::core::ffi::c_char,
        stem,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
    );
    if has_morphflag(&raw mut (*Have).gs_morphflags as *mut MorphFlags, NEEDS_ACCENT)
        == 0
        && has_morphflag(&raw mut (*Have).gs_morphflags as *mut MorphFlags, SUFF_ACC)
            == 0
        && has_morphflag(&raw mut (*Have).gs_morphflags as *mut MorphFlags, HAS_AUGMENT)
            == 0
        && nsylls(&raw mut (*Have).gs_gkstring as *mut ::core::ffi::c_char)
            < 3 as ::core::ffi::c_int
    {
        add_morphflag(
            &raw mut (*Have).gs_morphflags as *mut MorphFlags,
            ACCENT_OPTIONAL,
        );
    }
    if has_morphflag(&raw mut (*Have).gs_morphflags as *mut MorphFlags, IS_DERIV) == 0
        && (*Have).gs_gkstring[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != '*' as i32
    {
        AccComposForm(Have);
    }
    if (*Have).gs_gkstring[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == '*' as i32
    {
        zap_morphflag(
            &raw mut (*Have).gs_morphflags as *mut MorphFlags,
            ACCENT_OPTIONAL,
        );
    }
    if has_morphflag(&raw mut (*Have).gs_morphflags as *mut MorphFlags, INDECLFORM) != 0
    {
        CheckForBreathing(Have);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn CompStemEnd(
    mut gstr: *mut gk_string,
    mut stem: *mut ::core::ffi::c_char,
    mut endstr: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut lastc: ::core::ffi::c_int = 0;
    let mut ep: *mut ::core::ffi::c_char = endstr;
    lastc = *stem
        .offset(strlen(stem) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
    if (lastc == ROUGHBR || lastc == SMOOTHBR) && *ep as ::core::ffi::c_int == HARDLONG
        && cur_lang() != LATIN && cur_lang() != ITALIAN
    {
        *ep = lastc as ::core::ffi::c_char;
        lastc = *stem
            .offset(strlen(stem) as isize)
            .offset(-(2 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
        ep = stem
            .offset(strlen(stem) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        *ep = HARDLONG as ::core::ffi::c_char;
    }
    if cur_lang() != LATIN && cur_lang() != ITALIAN {
        if lastc == 'e' as i32 && *ep as ::core::ffi::c_int == HARDLONG {
            *ep = 'i' as i32 as ::core::ffi::c_char;
        } else if lastc == 'o' as i32 && *ep as ::core::ffi::c_int == HARDLONG {
            *ep = 'u' as i32 as ::core::ffi::c_char;
        }
    }
    *ep as ::core::ffi::c_int == '*' as i32;
    zap_extra_lmarks(stem);
    if (*stem.offset(Xstrlen(stem) as isize).offset(-(1 as ::core::ffi::c_int as isize))
        as ::core::ffi::c_int == 'h' as i32
        || *stem
            .offset(Xstrlen(stem) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'w' as i32
        || *stem
            .offset(Xstrlen(stem) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'H' as i32
        || *stem
            .offset(Xstrlen(stem) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'W' as i32) && *endstr as ::core::ffi::c_int == HARDLONG
    {
        strcpy(endstr, endstr.offset(1 as ::core::ffi::c_int as isize));
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn zap_extra_lmarks(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *s != 0 {
        if (*s as ::core::ffi::c_int == 'h' as i32
            || *s as ::core::ffi::c_int == 'w' as i32
            || *s as ::core::ffi::c_int == 'H' as i32
            || *s as ::core::ffi::c_int == 'W' as i32)
            && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == HARDLONG
        {
            strcpy(
                s.offset(1 as ::core::ffi::c_int as isize),
                s.offset(2 as ::core::ffi::c_int as isize),
            );
        }
        s = s.offset(1);
    }
    return 0;
}
