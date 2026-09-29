use ::c2rust_bitfields;
extern "C" {
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn calloc(_: size_t, _: size_t) -> *mut ::core::ffi::c_void;
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn CreatGkword(_: ::core::ffi::c_int) -> *mut gk_word;
    fn MorphFopen(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> *mut FILE;
    fn ScanAsciiKeys(
        _: *mut ::core::ffi::c_char,
        _: *mut gk_word,
        _: *mut gk_string,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn FreeGkword(_: *mut gk_word) -> ::core::ffi::c_int;
    fn NextEndTable(_: *mut ::core::ffi::c_int, _: Stemtype) -> *mut ::core::ffi::c_char;
    fn NextSuffTable(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ReadEnding(
        _: *mut FILE,
        _: *mut gk_string,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn get_endheader(_: *mut FILE, _: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn nextkey(
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
pub const ADJSTEM: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const NOUNSTEM: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const REG_DERIV: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const DIRCHAR: ::core::ffi::c_int = '/' as i32;
pub const MAX_END_TABLE: ::core::ffi::c_int = 12000 as ::core::ffi::c_int;
pub static mut endlines: *mut *mut ::core::ffi::c_char = ::core::ptr::null::<
    *mut ::core::ffi::c_char,
>() as *mut *mut ::core::ffi::c_char;
pub static mut Gstr: gk_string = gk_string {
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
pub unsafe extern "C" fn countendtables(
    mut stype: Stemtype,
    mut is_deriv: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut nends: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut totends: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut curtable: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut basen: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut dirp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut shortname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut curderivname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tmp: [::core::ffi::c_char; 8192] = [0; 8192];
    let mut prevtag: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut prevkey: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut curtag: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut savestr: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut markedstr: [::core::ffi::c_char; 60] = [0; 60];
    let mut Gstr_0: gk_string = gk_string {
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
    let mut finput: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut foutput: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut maxstring: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ntypes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut typestr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if is_deriv != 0 {
        dirp = DERIVTABLEDIR.as_ptr() as *mut ::core::ffi::c_char;
    } else {
        dirp = ENDTABLEDIR.as_ptr() as *mut ::core::ffi::c_char;
    }
    endlines = calloc(
        MAX_END_TABLE as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
    ) as *mut *mut ::core::ffi::c_char;
    loop {
        if is_deriv != 0 {
            let mut gstring: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
            let mut tmpGkword: *mut gk_word = ::core::ptr::null_mut::<gk_word>();
            let mut rconj: ::core::ffi::c_int = 0;
            curtable = NextSuffTable(&raw mut tmp as *mut ::core::ffi::c_char);
            nextkey(
                &raw mut tmp as *mut ::core::ffi::c_char,
                &raw mut curderivname as *mut ::core::ffi::c_char,
            );
            if curtable.is_null() {
                break;
            }
            curtable = &raw mut curderivname as *mut ::core::ffi::c_char;
            gstring = CreatGkString(1 as ::core::ffi::c_int);
            tmpGkword = CreatGkword(1 as ::core::ffi::c_int);
            ScanAsciiKeys(
                curtable,
                tmpGkword,
                gstring,
                ::core::ptr::null_mut::<gk_string>(),
            );
            rconj = ((*gstring).gs_derivtype & REG_DERIV as Derivtype)
                as ::core::ffi::c_int;
            FreeGkString(gstring);
            FreeGkword(tmpGkword);
            if rconj == 0 {
                printf(
                    b"[%s] not a regular conj\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    curtable,
                );
                continue;
            }
        } else {
            curtable = NextEndTable(&raw mut index, stype);
        }
        if curtable.is_null() {
            break;
        }
        sprintf(
            &raw mut shortname as *mut ::core::ffi::c_char,
            b"%s%cout%c%s.out\0" as *const u8 as *const ::core::ffi::c_char,
            dirp,
            DIRCHAR,
            DIRCHAR,
            curtable,
        );
        finput = MorphFopen(
            &raw mut shortname as *mut ::core::ffi::c_char,
            b"rb\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        if finput.is_null() {
            continue;
        }
        Gstr_0 = Blnk;
        nends = get_endheader(finput, &raw mut maxstring);
        totends += nends;
        ntypes += 1;
        ReadEnding(finput, &raw mut Gstr_0, maxstring);
        if Gstr_0.gs_steminfo & NOUNSTEM as Stemtype != 0 {
            typestr = b"noun\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        } else if Gstr_0.gs_steminfo & ADJSTEM as Stemtype != 0 {
            typestr = b"adj\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        } else {
            typestr = b"verb\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        printf(
            b"%s\t%s\t%d\t%d\n\0" as *const u8 as *const ::core::ffi::c_char,
            curtable,
            typestr,
            nends,
            totends,
        );
        fclose(finput);
    }
    printf(
        b"grand total: %d types %d endings\n\0" as *const u8
            as *const ::core::ffi::c_char,
        ntypes,
        totends,
    );
    return 0;
}
pub const ENDTABLEDIR: [::core::ffi::c_char; 10] = unsafe {
    ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"endtables\0")
};
pub const DERIVTABLEDIR: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"derivs\0")
};

unsafe extern "C" fn xstrcmp(
    mut p1: *mut *mut ::core::ffi::c_char,
    mut p2: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0;
    rval = morphstrcmp(*p1, *p2);
    return rval;
}

extern "C" { fn morphstrcmp(a: *mut core::ffi::c_char,b: *mut core::ffi::c_char) -> core::ffi::c_int; }
