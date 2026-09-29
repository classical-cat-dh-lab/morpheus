use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
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
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn load_ccontr(_: *mut ::core::ffi::c_int) -> *mut gk_string;
    fn load_vcontr(_: *mut ::core::ffi::c_int) -> *mut gk_string;
    fn AccComposForm(_: *mut gk_string) -> ::core::ffi::c_int;
    fn FixRecAcc(
        _: *mut gk_word,
        _: *mut MorphFlags,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn add_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn addbreath(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn getbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn nsylls(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn set_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
pub const PPARTMASK: ::core::ffi::c_int = 0o70000000 as ::core::ffi::c_int;
pub const PARTICIPLE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ACUTE: ::core::ffi::c_int = '/' as i32;
pub const SUBSCR: ::core::ffi::c_int = '|' as i32;
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const SUFF_ACC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const IS_DERIV: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const LOST_ACC: ::core::ffi::c_int = 69 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const MAXCONTRACTS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MAXSUBSTRING: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut Blnk: gk_string = gk_string {
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
pub static mut Vow_contr: *mut gk_string = ::core::ptr::null::<gk_string>()
    as *mut gk_string;
#[no_mangle]
pub static mut Cons_euph: *mut gk_string = ::core::ptr::null::<gk_string>()
    as *mut gk_string;
static mut numcontr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut numeuphs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn poss_contracts(
    mut gstr: *mut gk_string,
    mut skipdial: Dialect,
) -> *mut gk_string {
    let mut Poss_contracts: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    Poss_contracts = CreatGkString(MAXCONTRACTS + 1 as ::core::ffi::c_int);
    if Vow_contr.is_null() {
        Vow_contr = load_vcontr(&raw mut numcontr);
        if Vow_contr.is_null() {
            fprintf(
                stderr,
                b"Could not create poss_contracts!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<gk_string>();
        }
    }
    if sub_for_euph(gstr, skipdial, Poss_contracts, MAXCONTRACTS, Vow_contr, numcontr)
        != 0
    {
        return Poss_contracts;
    }
    FreeGkString(Poss_contracts);
    return ::core::ptr::null_mut::<gk_string>();
}
pub const MAXEUPHS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn do_euph(
    mut gstr: *mut gk_string,
    mut skipdial: Dialect,
) -> *mut gk_string {
    let mut euphs: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut hits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut orgstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut curs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    euphs = CreatGkString(MAXEUPHS);
    if Cons_euph.is_null() {
        Cons_euph = load_ccontr(&raw mut numeuphs);
        if Cons_euph.is_null() {
            fprintf(
                stderr,
                b"Could not create Cons_euph!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<gk_string>();
        }
    }
    hits = sub_for_euph(gstr, skipdial, euphs, MAXEUPHS, Cons_euph, numeuphs);
    if hits != 0 {
        return euphs;
    }
    FreeGkString(euphs);
    return ::core::ptr::null_mut::<gk_string>();
}
#[no_mangle]
pub unsafe extern "C" fn sub_for_euph(
    mut gstr: *mut gk_string,
    mut skipdial: Dialect,
    mut poss_subs: *mut gk_string,
    mut possno: ::core::ffi::c_int,
    mut sub_table: *mut gk_string,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut raw: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cooked: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut haveseen: [::core::ffi::c_char; 60] = [0; 60];
    let mut hp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut sofar: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut curs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    curs = &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char;
    hp = &raw mut haveseen as *mut ::core::ffi::c_char;
    *hp = 0 as ::core::ffi::c_char;
    *poss_subs.offset(sofar as isize) = *gstr;
    while *curs != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < len && sofar < possno {
            raw = &raw mut (*sub_table.offset(i as isize)).gs_gkstring
                as *mut ::core::ffi::c_char;
            cooked = (&raw mut (*sub_table.offset(i as isize)).gs_gkstring
                as *mut ::core::ffi::c_char)
                .offset(MAXSUBSTRING as isize);
            if sofar != 0 {
                if i > 0 as ::core::ffi::c_int
                    && strcmp(
                        &raw mut (*sub_table.offset(i as isize)).gs_gkstring
                            as *mut ::core::ffi::c_char,
                        &raw mut (*sub_table
                            .offset(i as isize)
                            .offset(-(1 as ::core::ffi::c_int as isize)))
                            .gs_gkstring as *mut ::core::ffi::c_char,
                    ) != 0
                {
                    return sofar;
                }
            }
            *poss_subs.offset(sofar as isize) = *gstr;
            if needs_sub(
                poss_subs.offset(sofar as isize),
                skipdial,
                sub_table.offset(i as isize),
                &raw mut haveseen as *mut ::core::ffi::c_char,
                curs,
                raw,
                cooked,
            ) != 0
            {
                sofar += 1;
            } else {
                *poss_subs.offset(sofar as isize) = Blnk;
            }
            i += 1;
        }
        if sofar != 0 {
            return sofar;
        }
        let fresh0 = curs;
        curs = curs.offset(1);
        let fresh1 = hp;
        hp = hp.offset(1);
        *fresh1 = *fresh0;
        *hp = 0 as ::core::ffi::c_char;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn needs_sub(
    mut gstr: *mut gk_string,
    mut skipdial: Dialect,
    mut matchgstr: *mut gk_string,
    mut haveseen: *mut ::core::ffi::c_char,
    mut curstring: *mut ::core::ffi::c_char,
    mut raw: *mut ::core::ffi::c_char,
    mut cooked: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut p2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut syllno: ::core::ffi::c_int = 0;
    let mut curdial: Dialect = 0;
    let mut dial: Dialect = 0;
    let mut gregion: GeogRegion = 0;
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
    let mut savestr: [::core::ffi::c_char; 60] = [0; 60];
    let mut savecur: [::core::ffi::c_char; 60] = [0; 60];
    let mut curbreath: ::core::ffi::c_int = 0;
    savecur[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    savestr[0 as ::core::ffi::c_int as usize] = savecur[0 as ::core::ffi::c_int
        as usize];
    dial = (*matchgstr).gs_dialect;
    set_morphflags(
        &raw mut SaveGstr,
        &raw mut (*matchgstr).gs_morphflags as *mut MorphFlags,
    );
    gregion = (*matchgstr).gs_geogregion;
    if skipdial as ::core::ffi::c_int & dial as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    curdial = AndDialect((*gstr).gs_dialect, dial);
    if (curdial as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    strcpy(&raw mut savestr as *mut ::core::ffi::c_char, haveseen);
    strcpy(&raw mut savecur as *mut ::core::ffi::c_char, curstring);
    curbreath = getbreath(&raw mut savecur as *mut ::core::ffi::c_char);
    stripbreath(&raw mut savecur as *mut ::core::ffi::c_char);
    if strncmp(raw, &raw mut savecur as *mut ::core::ffi::c_char, strlen(raw)) == 0 {
        let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
        tmp[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        strcpy(&raw mut tmp as *mut ::core::ffi::c_char, cooked);
        p1 = (&raw mut tmp as *mut ::core::ffi::c_char)
            .offset(strlen(&raw mut tmp as *mut ::core::ffi::c_char) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            (&raw mut savecur as *mut ::core::ffi::c_char).offset(strlen(raw) as isize),
        );
        if *p1 as ::core::ffi::c_int == SUBSCR
            && *p1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '/' as i32
        {
            let fresh2 = p1;
            p1 = p1.offset(1);
            *fresh2 = ACUTE as ::core::ffi::c_char;
            let fresh3 = p1;
            p1 = p1.offset(1);
            *fresh3 = SUBSCR as ::core::ffi::c_char;
            if *p1 as ::core::ffi::c_int == HARDLONG {
                strcpy(p1, p1.offset(1 as ::core::ffi::c_int as isize));
            }
        }
        strcat(
            &raw mut savestr as *mut ::core::ffi::c_char,
            &raw mut tmp as *mut ::core::ffi::c_char,
        );
        strcpy(
            &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut savestr as *mut ::core::ffi::c_char,
        );
        if cur_lang() != LATIN {
            addbreath(
                &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
                curbreath,
            );
        }
        (*gstr).gs_dialect = curdial;
        (*gstr).gs_geogregion |= gregion;
        add_morphflags(gstr, &raw mut SaveGstr.gs_morphflags as *mut MorphFlags);
        return 1 as ::core::ffi::c_int;
    } else {
        syllno = stripacc(&raw mut savecur as *mut ::core::ffi::c_char);
        if syllno != 0
            && strncmp(raw, &raw mut savecur as *mut ::core::ffi::c_char, strlen(raw))
                == 0
        {
            let mut tmp_0: [::core::ffi::c_char; 60] = [0; 60];
            let mut accs: [::core::ffi::c_char; 60] = [0; 60];
            strcpy(
                &raw mut tmp_0 as *mut ::core::ffi::c_char,
                &raw mut savestr as *mut ::core::ffi::c_char,
            );
            strcat(&raw mut tmp_0 as *mut ::core::ffi::c_char, cooked);
            strcat(
                &raw mut tmp_0 as *mut ::core::ffi::c_char,
                (&raw mut savecur as *mut ::core::ffi::c_char)
                    .offset(strlen(raw) as isize),
            );
            p1 = &raw mut tmp_0 as *mut ::core::ffi::c_char;
            while *p1 != 0 {
                if *p1 as ::core::ffi::c_int == SUBSCR
                    && *p1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == HARDLONG
                {
                    p1 = p1.offset(1);
                    strcpy(p1, p1.offset(1 as ::core::ffi::c_int as isize));
                }
                p1 = p1.offset(1);
            }
            strcpy(
                &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
                &raw mut tmp_0 as *mut ::core::ffi::c_char,
            );
            if cur_lang() != LATIN {
                addbreath(
                    &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
                    curbreath,
                );
            }
            (*gstr).gs_dialect = curdial;
            (*gstr).gs_geogregion |= gregion;
            add_morphflags(gstr, &raw mut SaveGstr.gs_morphflags as *mut MorphFlags);
            if has_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags, IS_DERIV)
                == 0
            {
                if syllno
                    == nsylls(&raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char)
                        - 1 as ::core::ffi::c_int
                {
                    add_morphflag(
                        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                        SUFF_ACC,
                    );
                }
                add_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    LOST_ACC,
                );
                if (*gstr).gs_steminfo & 0o70000000 as ::core::ffi::c_int as Stemtype
                    != 0
                    && (*gstr).gs_forminfo.f_mood() as ::core::ffi::c_int != PARTICIPLE
                    && strchr(
                            &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
                            '!' as i32,
                        )
                        .is_null()
                {
                    FixRecAcc(
                        gstr as *mut gk_word,
                        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                        &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
                    );
                } else {
                    AccComposForm(gstr);
                }
                zap_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    LOST_ACC,
                );
            }
            return 1 as ::core::ffi::c_int;
        }
    }
    if cur_lang() != LATIN {
        addbreath(&raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char, curbreath);
    }
    return 0 as ::core::ffi::c_int;
}
