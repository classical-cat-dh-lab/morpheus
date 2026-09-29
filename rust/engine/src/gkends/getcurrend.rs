use ::c2rust_bitfields;
extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn CreatGkString(_: ::core::ffi::c_int) -> *mut gk_string;
    fn MorphFopen(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> *mut FILE;
    fn NameOfStemtype(st: Stemtype) -> *mut ::core::ffi::c_char;
    fn FreeGkString(_: *mut gk_string) -> ::core::ffi::c_int;
    fn ReadEnding(
        _: *mut FILE,
        _: *mut gk_string,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn get_endheader(_: *mut FILE, _: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn xFclose(_: *mut FILE) -> ::core::ffi::c_int;
}
pub type int32 = ::core::ffi::c_uint;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const ENDCACHESIZE: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
static mut curecache: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut prevcache: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut EndingCache: [*mut gk_string; 46] = [::core::ptr::null::<gk_string>()
    as *mut gk_string; 46];
#[no_mangle]
pub unsafe extern "C" fn GetCurrentEndList(
    mut gstr: *mut gk_string,
    mut lnump: *mut ::core::ffi::c_int,
) -> *mut gk_string {
    let mut fname: [::core::ffi::c_char; 512] = [0; 512];
    let mut CurEndList: *mut gk_string = ::core::ptr::null_mut::<gk_string>();
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut maxend: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rval: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut lno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    CurEndList = CheckEndCache(gstr);
    if !CurEndList.is_null() {
        i = 0 as ::core::ffi::c_int;
        while !((*CurEndList.offset(i as isize))
            .gs_gkstring[0 as ::core::ffi::c_int as usize] == 0)
        {
            i += 1;
        }
        *lnump = i;
        return CurEndList;
    }
    sprintf(
        &raw mut fname as *mut ::core::ffi::c_char,
        b"%s/out/%s.out\0" as *const u8 as *const ::core::ffi::c_char,
        ENDTABLEDIR.as_ptr(),
        NameOfStemtype((*gstr).gs_steminfo),
    );
    f = MorphFopen(
        &raw mut fname as *mut ::core::ffi::c_char,
        b"rb\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"stemtype %o, could not open %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*gstr).gs_steminfo,
            &raw mut fname as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gk_string>();
    }
    lno = get_endheader(f, &raw mut maxend);
    if lno < 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"problem with endfile [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
            NameOfStemtype((*gstr).gs_steminfo),
        );
        xFclose(f);
        f = ::core::ptr::null_mut::<FILE>();
        return ::core::ptr::null_mut::<gk_string>();
    }
    *lnump = lno;
    CurEndList = CreatGkString(lno + 2 as ::core::ffi::c_int);
    if CurEndList.is_null() {
        fprintf(
            stderr,
            b"Out of memory loading in %d new endings!\n\0" as *const u8
                as *const ::core::ffi::c_char,
            lno + 2 as ::core::ffi::c_int,
        );
        xFclose(f);
        f = ::core::ptr::null_mut::<FILE>();
        exit(-(21 as ::core::ffi::c_int));
    }
    i = 0 as ::core::ffi::c_int;
    while i < lno {
        rval = ReadEnding(f, CurEndList.offset(i as isize), maxend);
        (*CurEndList.offset(i as isize)).gs_steminfo = (*gstr).gs_steminfo;
        if rval <= 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"hey! fname [%s] wanted [%d] endings got [%d]!\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                &raw mut fname as *mut ::core::ffi::c_char,
                lno,
                i,
            );
            xFclose(f);
            f = ::core::ptr::null_mut::<FILE>();
            return ::core::ptr::null_mut::<gk_string>();
        }
        i += 1;
    }
    xFclose(f);
    f = ::core::ptr::null_mut::<FILE>();
    InsertEndCache(CurEndList);
    return CurEndList;
}
static mut csize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CheckEndCache(mut gstr: *mut gk_string) -> *mut gk_string {
    let mut i: ::core::ffi::c_int = 0;
    cacheconsistent();
    i = 0 as ::core::ffi::c_int;
    while i < ENDCACHESIZE {
        if EndingCache[i as usize].is_null() {
            break;
        }
        if (*EndingCache[i as usize]).gs_steminfo == (*gstr).gs_steminfo {
            return EndingCache[i as usize];
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<gk_string>();
}
#[no_mangle]
pub unsafe extern "C" fn cacheconsistent() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < ENDCACHESIZE {
        if EndingCache[i as usize].is_null() {
            break;
        }
        if NameOfStemtype((*EndingCache[i as usize]).gs_steminfo).is_null() {
            if i > 1 as ::core::ffi::c_int {
                printf(
                    b"%d) prev type %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    i,
                    NameOfStemtype(
                        (*EndingCache[(i - 1 as ::core::ffi::c_int) as usize])
                            .gs_steminfo,
                    ),
                );
            } else {
                printf(
                    b"first stemtype has been zapped!\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            break;
        } else {
            i += 1;
        }
    }
    if i < csize {
        printf(
            b"saw only %d of %d ending tables\n\0" as *const u8
                as *const ::core::ffi::c_char,
            i,
            csize,
        );
    } else {
        csize = i;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn InsertEndCache(mut gstr: *mut gk_string) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if !EndingCache[curecache as usize].is_null() {
        FreeGkString(EndingCache[curecache as usize]);
        EndingCache[curecache as usize] = ::core::ptr::null_mut::<gk_string>();
    }
    EndingCache[curecache as usize] = gstr;
    prevcache = curecache;
    curecache += 1;
    if curecache >= ENDCACHESIZE {
        curecache = 0 as ::core::ffi::c_int;
    }
    return 0;
}
pub const ENDTABLEDIR: [::core::ffi::c_char; 10] = unsafe {
    ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"endtables\0")
};
