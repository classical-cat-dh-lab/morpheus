use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fseek(
        _: *mut FILE,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn CreatGkword(_: ::core::ffi::c_int) -> *mut gk_word;
    fn MorphFopen(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> *mut FILE;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn GetTableLine(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn ScanAsciiKeys(
        _: *mut ::core::ffi::c_char,
        _: *mut gk_word,
        _: *mut gk_string,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn FreeGkword(_: *mut gk_word) -> ::core::ffi::c_int;
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
    fn add_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn nextkey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn set_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn xFclose(_: *mut FILE) -> ::core::ffi::c_int;
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
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
pub static mut Euph_table: *mut gk_string = ::core::ptr::null::<gk_string>()
    as *mut gk_string;
#[no_mangle]
pub unsafe extern "C" fn load_euph_tab(
    mut filename: *mut ::core::ffi::c_char,
    mut gotno: *mut ::core::ffi::c_int,
    mut is_contr: ::core::ffi::c_int,
) -> *mut gk_string {
    let mut nunits: ::core::ffi::c_int = 0;
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut i: ::core::ffi::c_int = 0;
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut raw: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cooked: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tmpa: [::core::ffi::c_char; 60] = [0; 60];
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    let mut CurStr: gk_string = gk_string {
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
    let mut TmpGkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
    extern "C" {
        #[link_name = "RevCompByStr"]
        fn RevCompByStr_0(_: *mut gk_string, _: *mut gk_string) -> ::core::ffi::c_int;
    }
    f = MorphFopen(
        filename,
        b"r\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"Could not open [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        return ::core::ptr::null_mut::<gk_string>();
    }
    TmpGkword = CreatGkword(1 as ::core::ffi::c_int);
    nunits = count_rlines(f);
    Euph_table = CreatGkString(nunits + 1 as ::core::ffi::c_int);
    if Euph_table.is_null() {
        fprintf(
            stderr,
            b"no memory for %d big Euph_table\n\0" as *const u8
                as *const ::core::ffi::c_char,
            nunits + 1 as ::core::ffi::c_int,
        );
        xFclose(f);
        FreeGkword(TmpGkword);
        return ::core::ptr::null_mut::<gk_string>();
    }
    i = 0 as ::core::ffi::c_int;
    while GetTableLine(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
        f,
    ) != 0
    {
        if i >= nunits {
            printf(
                b"hey! more than %d contracts!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                nunits,
            );
            xFclose(f);
            FreeGkword(TmpGkword);
            return ::core::ptr::null_mut::<gk_string>();
        }
        CurStr = Blnk;
        s = &raw mut CurStr.gs_gkstring as *mut ::core::ffi::c_char;
        nextkey(&raw mut line as *mut ::core::ffi::c_char, s);
        nextkey(
            &raw mut line as *mut ::core::ffi::c_char,
            s.offset(MAXSUBSTRING as isize),
        );
        raw = s;
        cooked = s.offset(MAXSUBSTRING as isize);
        if Xstrncmp(raw, cooked, Xstrlen(raw) as size_t) == 0
            && Xstrlen(raw) > 1 as ::core::ffi::c_int && is_contr == YES
        {
            tmp[0 as ::core::ffi::c_int as usize] = *cooked;
            tmp[1 as ::core::ffi::c_int as usize] = '+' as i32 as ::core::ffi::c_char;
            tmp[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            Xstrncat(
                &raw mut tmp as *mut ::core::ffi::c_char,
                cooked.offset(1 as ::core::ffi::c_int as isize),
                MAXWORDSIZE as size_t,
            );
            Xstrncpy(
                cooked,
                &raw mut tmp as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
        }
        set_morphflag(
            &raw mut (*TmpGkword).gs_preverb.gs_morphflags as *mut MorphFlags,
            0 as ::core::ffi::c_int,
        );
        ScanAsciiKeys(
            &raw mut line as *mut ::core::ffi::c_char,
            TmpGkword,
            &raw mut CurStr,
            ::core::ptr::null_mut::<gk_string>(),
        );
        add_morphflags(
            &raw mut CurStr,
            &raw mut (*TmpGkword).gs_preverb.gs_morphflags as *mut MorphFlags,
        );
        *Euph_table.offset(i as isize) = CurStr;
        i += 1;
    }
    *gotno = i;
    xFclose(f);
    FreeGkword(TmpGkword);
    return Euph_table;
}
#[no_mangle]
pub unsafe extern "C" fn count_rlines(mut f: *mut FILE) -> ::core::ffi::c_int {
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut nlines: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while GetTableLine(
        &raw mut line as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
        f,
    ) != 0
    {
        nlines += 1;
    }
    fseek(f, 0 as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    return nlines;
}
