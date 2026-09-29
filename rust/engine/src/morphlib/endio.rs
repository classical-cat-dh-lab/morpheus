use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fseek(
        _: *mut FILE,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(_: *mut FILE) -> ::core::ffi::c_long;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vax_fread(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn vax_fwrite(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: ::core::ffi::c_int,
        _: *mut FILE,
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
pub const MORPH_VERSION: ::core::ffi::c_int = 44440001 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn WriteEnding(
    mut f: *mut FILE,
    mut gstr: *mut gk_string,
    mut maxend: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut max: int32 = 0;
    max = maxend as int32;
    localtrimwhite(&raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char, maxend);
    if !(vax_fwrite(
        &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        maxend,
        f,
    ) < 0 as ::core::ffi::c_int)
    {
        if !(vax_fwrite(
            &raw mut (*gstr).gs_forminfo as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<word_form>() as size_t,
            1 as ::core::ffi::c_int,
            f,
        ) < 0 as ::core::ffi::c_int)
        {
            if !(vax_fwrite(
                &raw mut (*gstr).gs_dialect as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<Dialect>() as size_t,
                1 as ::core::ffi::c_int,
                f,
            ) < 0 as ::core::ffi::c_int)
            {
                if !(vax_fwrite(
                    &raw mut (*gstr).gs_geogregion as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<GeogRegion>() as size_t,
                    1 as ::core::ffi::c_int,
                    f,
                ) < 0 as ::core::ffi::c_int)
                {
                    if !(vax_fwrite(
                        &raw mut (*gstr).gs_steminfo as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<Stemtype>() as size_t,
                        1 as ::core::ffi::c_int,
                        f,
                    ) < 0 as ::core::ffi::c_int)
                    {
                        if !(vax_fwrite(
                            &raw mut (*gstr).gs_derivtype as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<Derivtype>() as size_t,
                            1 as ::core::ffi::c_int,
                            f,
                        ) < 0 as ::core::ffi::c_int)
                        {
                            if !(vax_fwrite(
                                &raw mut (*gstr).gs_morphflags as *mut MorphFlags
                                    as *mut ::core::ffi::c_char,
                                1 as size_t,
                                ::core::mem::size_of::<[MorphFlags; 12]>()
                                    as ::core::ffi::c_int,
                                f,
                            ) < 0 as ::core::ffi::c_int)
                            {
                                if !(vax_fwrite(
                                    &raw mut (*gstr).st_domains as *mut ::core::ffi::c_char,
                                    1 as size_t,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 21]>()
                                        as ::core::ffi::c_int,
                                    f,
                                ) < 0 as ::core::ffi::c_int)
                                {
                                    return 1 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fprintf(stderr, b"output error!\n\0" as *const u8 as *const ::core::ffi::c_char);
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ReadEnding(
    mut f: *mut FILE,
    mut gstr: *mut gk_string,
    mut maxend: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nread: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    nread = vax_fread(
        &raw mut (*gstr).gs_gkstring as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        maxend,
        f,
    );
    if !(nread <= 0 as ::core::ffi::c_int) {
        nread = vax_fread(
            &raw mut (*gstr).gs_forminfo as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<word_form>() as size_t,
            1 as ::core::ffi::c_int,
            f,
        );
        if !(nread <= 0 as ::core::ffi::c_int) {
            nread = vax_fread(
                &raw mut (*gstr).gs_dialect as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<Dialect>() as size_t,
                1 as ::core::ffi::c_int,
                f,
            );
            if !(nread <= 0 as ::core::ffi::c_int) {
                nread = vax_fread(
                    &raw mut (*gstr).gs_geogregion as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<GeogRegion>() as size_t,
                    1 as ::core::ffi::c_int,
                    f,
                );
                if !(nread <= 0 as ::core::ffi::c_int) {
                    nread = vax_fread(
                        &raw mut (*gstr).gs_steminfo as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<Stemtype>() as size_t,
                        1 as ::core::ffi::c_int,
                        f,
                    );
                    if !(nread <= 0 as ::core::ffi::c_int) {
                        nread = vax_fread(
                            &raw mut (*gstr).gs_derivtype as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<Derivtype>() as size_t,
                            1 as ::core::ffi::c_int,
                            f,
                        );
                        if !(nread <= 0 as ::core::ffi::c_int) {
                            nread = vax_fread(
                                &raw mut (*gstr).gs_morphflags as *mut MorphFlags
                                    as *mut ::core::ffi::c_char,
                                1 as size_t,
                                ::core::mem::size_of::<[MorphFlags; 12]>()
                                    as ::core::ffi::c_int,
                                f,
                            );
                            if !(nread <= 0 as ::core::ffi::c_int) {
                                nread = vax_fread(
                                    &raw mut (*gstr).st_domains as *mut ::core::ffi::c_char,
                                    1 as size_t,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 21]>()
                                        as ::core::ffi::c_int,
                                    f,
                                );
                                if !(nread <= 0 as ::core::ffi::c_int) {
                                    return 1 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if nread < 0 as ::core::ffi::c_int {
        fprintf(stderr, b"input error!\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return nread;
}
#[no_mangle]
pub unsafe extern "C" fn set_endheader(
    mut f: *mut FILE,
    mut maxstring: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut morph_version: ::core::ffi::c_uint = 0;
    let mut len: ::core::ffi::c_int = 0;
    morph_version = MORPH_VERSION as ::core::ffi::c_uint;
    len = maxstring;
    if vax_fwrite(
        &raw mut morph_version as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<::core::ffi::c_uint>() as size_t,
        1 as ::core::ffi::c_int,
        f,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if vax_fwrite(
        &raw mut len as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as ::core::ffi::c_int,
        f,
    ) != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_endheader(
    mut f: *mut FILE,
    mut maxp: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut morph_version: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut curpos: ::core::ffi::c_int = 0;
    let mut filelen: ::core::ffi::c_int = 0;
    let mut endlen: ::core::ffi::c_int = 0;
    let mut gstrsize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nendings: ::core::ffi::c_int = 0;
    let mut unitsize: ::core::ffi::c_int = 0;
    let mut fred: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    gstrsize = (::core::mem::size_of::<word_form>() as usize)
        .wrapping_add(::core::mem::size_of::<Stemtype>() as usize)
        .wrapping_add(::core::mem::size_of::<Dialect>() as usize)
        .wrapping_add(::core::mem::size_of::<Derivtype>() as usize)
        .wrapping_add(::core::mem::size_of::<GeogRegion>() as usize)
        .wrapping_add(::core::mem::size_of::<[MorphFlags; 12]>() as usize)
        .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 21]>() as usize)
        .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 60]>() as usize)
        as ::core::ffi::c_int;
    if vax_fread(
        &raw mut morph_version as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as ::core::ffi::c_int,
        f,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if morph_version != MORPH_VERSION {
        fprintf(
            stderr,
            b"Hey! new version of Morpheus!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if vax_fread(
        &raw mut len as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        1 as ::core::ffi::c_int,
        f,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    *maxp = len;
    curpos = ftell(f) as ::core::ffi::c_int;
    fseek(f, 0 as ::core::ffi::c_long, 2 as ::core::ffi::c_int);
    filelen = ftell(f) as ::core::ffi::c_int;
    fseek(f, curpos as ::core::ffi::c_long, 0 as ::core::ffi::c_int);
    unitsize = (gstrsize as usize)
        .wrapping_sub(::core::mem::size_of::<[::core::ffi::c_char; 60]>() as usize)
        .wrapping_add(*maxp as usize) as ::core::ffi::c_int;
    endlen = (filelen as usize)
        .wrapping_sub(
            (::core::mem::size_of::<::core::ffi::c_int>() as usize)
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as usize),
        ) as ::core::ffi::c_int;
    nendings = endlen / unitsize;
    if endlen % unitsize != 0 {
        printf(
            b"gstrsize %d endlen %d unitsize %d, mod %d, nendings %d filelen %ld\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            gstrsize,
            endlen,
            unitsize,
            endlen % unitsize,
            nendings,
            filelen,
        );
        fprintf(
            stderr,
            b"Error in endio!\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    return nendings;
}
#[no_mangle]
pub unsafe extern "C" fn localtrimwhite(
    mut s: *mut ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut sdone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        if *s == 0 {
            sdone = 1 as ::core::ffi::c_int;
        }
        if sdone != 0 {
            *s = 0 as ::core::ffi::c_char;
        }
        s = s.offset(1);
        i += 1;
    }
    return 0;
}
