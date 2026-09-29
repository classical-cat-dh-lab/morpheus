use ::c2rust_bitfields;
extern "C" {
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
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
    fn AndDialect(_: Dialect, _: Dialect) -> Dialect;
    fn ClearGkstring(_: *mut gk_string) -> ::core::ffi::c_int;
    fn Xstrncat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn addbreath(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn checkdict(
        _: *mut gk_word,
        _: *mut gk_string,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn checkstem(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut *mut gk_string,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn getbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn nextkey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn starts_w_diphth(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripdiaer(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn xFree(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn zap_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub const BUFSIZ: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const IONIC: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const DIAERESIS: ::core::ffi::c_int = '+' as i32;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
pub const NOBREATH: ::core::ffi::c_int = ' ' as i32;
pub const UNASP_PREVERB: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const GREEK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAX_POSS_STEMS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn checkhalf1(
    mut Gkword: *mut gk_word,
    mut endkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stem: *mut ::core::ffi::c_char = &raw mut (*Gkword).gs_stem.gs_gkstring
        as *mut ::core::ffi::c_char;
    let mut savestem: [::core::ffi::c_char; 60] = [0; 60];
    let mut unasp_prev: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    strcpy(&raw mut savestem as *mut ::core::ffi::c_char, stem);
    if *stem as ::core::ffi::c_int == 'r' as i32 && getbreath(stem) == NOBREATH
        && cur_lang() == GREEK
    {
        let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
        Xstrncpy(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"r(\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
                as size_t,
        );
        Xstrncat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            stem.offset(1 as ::core::ffi::c_int as isize),
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
                as size_t,
        );
        Xstrncpy(stem, &raw mut tmp as *mut ::core::ffi::c_char, MAXWORDSIZE as size_t);
        rval = checkhalf2(Gkword, endkeys);
        return rval;
    }
    if cur_lang() == GREEK
        && (*stem as ::core::ffi::c_int == 'a' as i32
            || *stem as ::core::ffi::c_int == 'e' as i32
            || *stem as ::core::ffi::c_int == 'i' as i32
            || *stem as ::core::ffi::c_int == 'o' as i32
            || *stem as ::core::ffi::c_int == 'u' as i32
            || *stem as ::core::ffi::c_int == 'A' as i32
            || *stem as ::core::ffi::c_int == 'E' as i32
            || *stem as ::core::ffi::c_int == 'I' as i32
            || *stem as ::core::ffi::c_int == 'O' as i32
            || *stem as ::core::ffi::c_int == 'U' as i32
            || (*stem as ::core::ffi::c_int == 'h' as i32
                || *stem as ::core::ffi::c_int == 'w' as i32
                || *stem as ::core::ffi::c_int == 'H' as i32
                || *stem as ::core::ffi::c_int == 'W' as i32))
        && getbreath(stem) == NOBREATH
    {
        Xstrncpy(
            &raw mut savestem as *mut ::core::ffi::c_char,
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
                as size_t,
        );
        addbreath(stem, ROUGHBR);
        rval += checkhalf2(Gkword, endkeys);
        Xstrncpy(
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut savestem as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        addbreath(stem, SMOOTHBR);
        if has_morphflag(
            &raw mut (*Gkword).gs_preverb.gs_morphflags as *mut MorphFlags,
            UNASP_PREVERB,
        ) != 0
        {
            unasp_prev = 1 as ::core::ffi::c_int;
            zap_morphflag(
                &raw mut (*Gkword).gs_preverb.gs_morphflags as *mut MorphFlags,
                UNASP_PREVERB,
            );
        }
        rval += checkhalf2(Gkword, endkeys);
        if unasp_prev != 0 {
            unasp_prev = 0 as ::core::ffi::c_int;
            add_morphflag(
                &raw mut (*Gkword).gs_preverb.gs_morphflags as *mut MorphFlags,
                UNASP_PREVERB,
            );
        }
        if rval == 0 && starts_w_diphth(stem) != 0 {
            let mut diaerstem: [::core::ffi::c_char; 60] = [0; 60];
            strncpy(
                &raw mut diaerstem as *mut ::core::ffi::c_char,
                &raw mut savestem as *mut ::core::ffi::c_char,
                2 as size_t,
            );
            diaerstem[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            stripdiaer(&raw mut diaerstem as *mut ::core::ffi::c_char);
            strcat(
                &raw mut diaerstem as *mut ::core::ffi::c_char,
                b"+\0" as *const u8 as *const ::core::ffi::c_char,
            );
            strcat(
                &raw mut diaerstem as *mut ::core::ffi::c_char,
                (&raw mut savestem as *mut ::core::ffi::c_char)
                    .offset(2 as ::core::ffi::c_int as isize),
            );
            Xstrncpy(
                &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
                &raw mut diaerstem as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
            addbreath(stem, ROUGHBR);
            rval += checkhalf2(Gkword, endkeys);
            Xstrncpy(
                &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
                &raw mut diaerstem as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
            if has_morphflag(
                &raw mut (*Gkword).gs_preverb.gs_morphflags as *mut MorphFlags,
                UNASP_PREVERB,
            ) != 0
            {
                unasp_prev = 1 as ::core::ffi::c_int;
                zap_morphflag(
                    &raw mut (*Gkword).gs_preverb.gs_morphflags as *mut MorphFlags,
                    UNASP_PREVERB,
                );
            }
            addbreath(stem, SMOOTHBR);
            rval += checkhalf2(Gkword, endkeys);
            if unasp_prev != 0 {
                unasp_prev = 0 as ::core::ffi::c_int;
                add_morphflag(
                    &raw mut (*Gkword).gs_preverb.gs_morphflags as *mut MorphFlags,
                    UNASP_PREVERB,
                );
            }
            Xstrncpy(
                &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
                &raw mut savestem as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
        }
        return rval;
    } else {
        rval = checkhalf2(Gkword, endkeys);
        if rval != 0 {
            return rval
        } else if cur_lang() != LATIN
            && (*Gkword).st_rawprvb[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int != 0 && starts_w_diphth(stem) != 0
        {
            let mut cbreath: ::core::ffi::c_int = 0;
            let mut diaerstem_0: [::core::ffi::c_char; 60] = [0; 60];
            cbreath = getbreath(stem);
            stripbreath(stem);
            diaerstem_0[0 as ::core::ffi::c_int as usize] = *stem;
            diaerstem_0[1 as ::core::ffi::c_int as usize] = DIAERESIS
                as ::core::ffi::c_char;
            strcpy(
                (&raw mut diaerstem_0 as *mut ::core::ffi::c_char)
                    .offset(2 as ::core::ffi::c_int as isize),
                stem.offset(1 as ::core::ffi::c_int as isize),
            );
            addbreath(&raw mut diaerstem_0 as *mut ::core::ffi::c_char, cbreath);
            Xstrncpy(
                &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
                &raw mut diaerstem_0 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
            rval += checkhalf2(Gkword, endkeys);
            Xstrncpy(
                &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
                &raw mut savestem as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
        }
    }
    if rval == 0
        && (*Gkword).gs_preverb.gs_gkstring[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int != 0
        && getbreath(&raw mut savestem as *mut ::core::ffi::c_char) == SMOOTHBR
        && AndDialect((*Gkword).gs_dialect, IONIC as Dialect) as ::core::ffi::c_int
            >= 0 as ::core::ffi::c_int && cur_lang() != LATIN
    {
        add_morphflag(
            &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
            UNASP_PREVERB,
        );
        strcpy(stem, &raw mut savestem as *mut ::core::ffi::c_char);
        stripbreath(stem);
        addbreath(stem, ROUGHBR);
        rval += checkhalf2(Gkword, endkeys);
        Xstrncpy(
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut savestem as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
        zap_morphflag(
            &raw mut (*Gkword).gs_morphflags as *mut MorphFlags,
            UNASP_PREVERB,
        );
    }
    return rval;
}
static mut poss_stems: [*mut gk_string; 10] = [::core::ptr::null::<gk_string>()
    as *mut gk_string; 10];
static mut poss_keys: [*mut ::core::ffi::c_char; 10] = [::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char; 10];
static mut init_stor: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn checkhalf2(
    mut Gkword: *mut gk_word,
    mut endkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if init_stor == 0 {
        init_stor = 1 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < MAX_POSS_STEMS {
            poss_stems[i as usize] = CreatGkString(1 as ::core::ffi::c_int);
            poss_keys[i as usize] = malloc(LONGSTRING as size_t)
                as *mut ::core::ffi::c_char;
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < MAX_POSS_STEMS {
        ClearGkstring(poss_stems[i as usize]);
        *poss_keys[i as usize] = 0 as ::core::ffi::c_char;
        i += 1;
    }
    rval = checkstem(
        &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
        endkeys,
        &raw mut poss_stems as *mut *mut gk_string,
        &raw mut poss_keys as *mut *mut ::core::ffi::c_char,
        MAX_POSS_STEMS - 1 as ::core::ffi::c_int,
    );
    if rval != 0 {
        rval = StemsWork(
            Gkword,
            &raw mut poss_stems as *mut *mut gk_string,
            &raw mut poss_keys as *mut *mut ::core::ffi::c_char,
            rval,
        );
        rval != 0;
    }
    if rval != 0 {
        return 1 as ::core::ffi::c_int
    } else {
        return 0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn StemsWork(
    mut Gkword: *mut gk_word,
    mut poss_stems_0: *mut *mut gk_string,
    mut poss_keys_0: *mut *mut ::core::ffi::c_char,
    mut stem_num: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut savestem: [::core::ffi::c_char; 60] = [0; 60];
    let mut i: ::core::ffi::c_int = 0;
    let mut rval: ::core::ffi::c_int = 0;
    let mut result: ::core::ffi::c_int = 0;
    result = 0 as ::core::ffi::c_int;
    Xstrncpy(
        &raw mut savestem as *mut ::core::ffi::c_char,
        &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as ::core::ffi::c_int
            as size_t,
    );
    i = 0 as ::core::ffi::c_int;
    while i < stem_num {
        rval = StemWorks(
            Gkword,
            *poss_keys_0.offset(i as isize),
            *poss_stems_0.offset(i as isize),
        );
        if rval != 0 {
            result += rval;
        }
        i += 1;
    }
    if result == 0 {
        Xstrncpy(
            &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
            &raw mut savestem as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn StemWorks(
    mut Gkword: *mut gk_word,
    mut posskey: *mut ::core::ffi::c_char,
    mut possstem: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut curval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut workkey: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut stemkeys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut curkey: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut savestemstr: gk_string = gk_string {
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
    workkey = malloc((BUFSIZ as size_t).wrapping_mul(2 as size_t))
        as *mut ::core::ffi::c_char;
    stemkeys = malloc((BUFSIZ as size_t).wrapping_mul(2 as size_t))
        as *mut ::core::ffi::c_char;
    curkey = malloc((BUFSIZ as size_t).wrapping_mul(2 as size_t))
        as *mut ::core::ffi::c_char;
    Xstrncpy(workkey, posskey, (BUFSIZ * 2 as ::core::ffi::c_int) as size_t);
    while nextkey(workkey, curkey) != 0 {
        curval = checkdict(Gkword, possstem, curkey);
        rval += curval;
    }
    xFree(
        workkey,
        b"workkey\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    xFree(
        stemkeys,
        b"stemkeys\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    xFree(
        curkey,
        b"curkey\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    stemkeys = ::core::ptr::null_mut::<::core::ffi::c_char>();
    workkey = stemkeys;
    curkey = workkey;
    return rval;
}
