use ::c2rust_bitfields;
extern "C" {
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn CheckGenWords(_: *mut gk_word, _: *mut gk_word) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn GenIrregForm(
        _: *mut gk_word,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut gk_word;
    fn chckindecl(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn parsefield(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn subchar(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
pub const INDECL: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const VERBS_ONLY: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn checkindecl(mut Gkword: *mut gk_word) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0;
    let mut hits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nstems: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut sawstems: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut keys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut keybuf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut workword: [::core::ffi::c_char; 60] = [0; 60];
    let mut stemkeys: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tmpword: [::core::ffi::c_char; 60] = [0; 60];
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if (*Gkword).gs_prntflags as ::core::ffi::c_int & VERBS_ONLY != 0 {
        return 0 as ::core::ffi::c_int;
    }
    Xstrncpy(
        &raw mut tmpword as *mut ::core::ffi::c_char,
        &raw mut (*Gkword).st_workword as *mut ::core::ffi::c_char,
        MAXWORDSIZE as size_t,
    );
    keys = &raw mut keybuf as *mut ::core::ffi::c_char;
    rval = chckindecl(&raw mut tmpword as *mut ::core::ffi::c_char, keys);
    if !(rval == 0) {
        while *keys != 0 {
            sp = keys;
            sp = parsefield(
                sp,
                &raw mut workword as *mut ::core::ffi::c_char,
                ':' as i32,
                MAXWORDSIZE,
            );
            if workword[0 as ::core::ffi::c_int as usize] == 0 {
                Xstrncpy(
                    &raw mut workword as *mut ::core::ffi::c_char,
                    &raw mut tmpword as *mut ::core::ffi::c_char,
                    MAXWORDSIZE as size_t,
                );
            }
            sp = parsefield(
                sp,
                &raw mut (*Gkword).st_lemma as *mut ::core::ffi::c_char,
                ':' as i32,
                MAXWORDSIZE,
            );
            sp = parsefield(
                sp,
                &raw mut stemkeys as *mut ::core::ffi::c_char,
                ' ' as i32,
                LONGSTRING,
            );
            subchar(
                &raw mut stemkeys as *mut ::core::ffi::c_char,
                ':' as i32,
                ' ' as i32,
            );
            Xstrncpy(
                &raw mut (*Gkword).gs_stem.gs_gkstring as *mut ::core::ffi::c_char,
                &raw mut workword as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 60]>() as size_t,
            );
            hits += IndeclWorks(Gkword, &raw mut stemkeys as *mut ::core::ffi::c_char);
            while *keys as ::core::ffi::c_int != 0
                && __isspace(*keys as ::core::ffi::c_int) == 0
            {
                keys = keys.offset(1);
            }
            while __isspace(*keys as ::core::ffi::c_int) != 0 {
                keys = keys.offset(1);
            }
        }
    }
    return hits;
}
unsafe extern "C" fn IndeclWorks(
    mut Gkword: *mut gk_word,
    mut keys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut Forms: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    Forms = GenIrregForm(Gkword, keys, INDECL);
    if !Forms.is_null() {
        rval += CheckGenWords(Gkword, Forms);
        FreeGkString(Forms as *mut gk_string);
    }
    return rval;
}
