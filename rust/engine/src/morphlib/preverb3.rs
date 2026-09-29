use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn load_euph_tab(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut gk_string;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Xstrncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn add_morphflags(_: *mut gk_string, _: *mut MorphFlags) -> ::core::ffi::c_int;
    fn has_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn no_morphflag(_: *mut MorphFlags) -> ::core::ffi::c_int;
    fn overlap_morphflags(_: *mut MorphFlags, _: *mut MorphFlags) -> ::core::ffi::c_int;
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
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RAW_PREVERB: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const MAXSUBSTRING: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut previndex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut prevprevb: [::core::ffi::c_char; 60] = [0; 60];
static mut PrevbTable: *mut gk_string = ::core::ptr::null::<gk_string>()
    as *mut gk_string;
static mut numprevb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn nextpreverb(
    mut word: *mut ::core::ffi::c_char,
    mut oldprevb: *mut ::core::ffi::c_char,
    mut pblemma: *mut ::core::ffi::c_char,
    mut gstr: *mut gk_string,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if PrevbTable.is_null() {
        PrevbTable = load_euph_tab(
            RAWPBLIST.as_ptr() as *mut ::core::ffi::c_char,
            &raw mut numprevb,
            NO,
        );
        if PrevbTable.is_null() {
            fprintf(
                stderr,
                b"Could not create numpreverbs!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < numprevb {
        if *oldprevb == 0 {
            break;
        }
        if has_rawpreverb(oldprevb, PrevbTable.offset(i as isize)) != 0 {
            i += 1;
            break;
        } else {
            i += 1;
        }
    }
    while i < numprevb {
        if has_rawpreverb(word, PrevbTable.offset(i as isize)) != 0 {
            let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
            Xstrncpy(
                oldprevb,
                &raw mut (*PrevbTable.offset(i as isize)).gs_gkstring
                    as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            Xstrncpy(
                &raw mut prevprevb as *mut ::core::ffi::c_char,
                &raw mut (*PrevbTable.offset(i as isize)).gs_gkstring
                    as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            previndex = i + 1 as ::core::ffi::c_int;
            Xstrncpy(
                pblemma,
                (&raw mut (*PrevbTable.offset(i as isize)).gs_gkstring
                    as *mut ::core::ffi::c_char)
                    .offset(MAXSUBSTRING as isize),
                MAXWORDSIZE as size_t,
            );
            Xstrncpy(
                &raw mut tmp as *mut ::core::ffi::c_char,
                word
                    .offset(
                        Xstrlen(
                            &raw mut (*PrevbTable.offset(i as isize)).gs_gkstring
                                as *mut ::core::ffi::c_char,
                        ) as isize,
                    ),
                MAXWORDSIZE as size_t,
            );
            Xstrncpy(
                word,
                &raw mut tmp as *mut ::core::ffi::c_char,
                MAXWORDSIZE as size_t,
            );
            if !(no_morphflag(&raw mut (*gstr).gs_morphflags as *mut MorphFlags) == 0
                && overlap_morphflags(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    &raw mut (*PrevbTable.offset(i as isize)).gs_morphflags
                        as *mut MorphFlags,
                ) != 0
                && has_morphflag(
                    &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
                    RAW_PREVERB,
                ) != 0)
            {
                add_morphflags(
                    gstr,
                    &raw mut (*PrevbTable.offset(i as isize)).gs_morphflags
                        as *mut MorphFlags,
                );
                if (*PrevbTable.offset(i as isize)).gs_dialect != 0 {
                    (*gstr).gs_dialect = (*PrevbTable.offset(i as isize)).gs_dialect;
                }
                return 1 as ::core::ffi::c_int;
            }
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn has_rawpreverb(
    mut curpb: *mut ::core::ffi::c_char,
    mut pbentry: *mut gk_string,
) -> ::core::ffi::c_int {
    return (Xstrncmp(
        curpb,
        &raw mut (*pbentry).gs_gkstring as *mut ::core::ffi::c_char,
        Xstrlen(&raw mut (*pbentry).gs_gkstring as *mut ::core::ffi::c_char) as size_t,
    ) == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_rawpreverb(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < numprevb {
        if strcmp(
            s,
            &raw mut (*PrevbTable.offset(i as isize)).gs_gkstring
                as *mut ::core::ffi::c_char,
        ) == 0
        {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
pub const RAWPBLIST: [::core::ffi::c_char; 30] = unsafe {
    ::core::mem::transmute::<
        [u8; 30],
        [::core::ffi::c_char; 30],
    >(*b"rule_files/raw_preverbs.table\0")
};
