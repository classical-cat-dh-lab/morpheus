use ::c2rust_bitfields;
extern "C" {
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn NameOfTense(vf: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfMood(vf: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfVoice(vf: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfPerson(vf: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfNumber(vf: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfGender(af: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfCase(af: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfDegree(af: word_form) -> *mut ::core::ffi::c_char;
    fn NameOfStemtype(st: Stemtype) -> *mut ::core::ffi::c_char;
    fn NameOfDerivtype(st: Derivtype) -> *mut ::core::ffi::c_char;
    fn DialectNames(
        _: Dialect,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn DomainNames(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn GeogRegionNames(
        _: GeogRegion,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn MorphNames(
        _: *mut MorphFlags,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type int32 = ::core::ffi::c_uint;
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
#[no_mangle]
pub unsafe extern "C" fn JakeSprintGkFlags(
    mut gstr: *mut gk_string,
    mut buf: *mut ::core::ffi::c_char,
    mut dels: *mut ::core::ffi::c_char,
    mut more_dels: ::core::ffi::c_int,
    mut pretty: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dialbuf: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wf: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    wf = (*gstr).gs_forminfo;
    s = NameOfStemtype((*gstr).gs_steminfo);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, NameOfStemtype((*gstr).gs_steminfo));
    }
    s = NameOfDerivtype((*gstr).gs_derivtype);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, NameOfDerivtype((*gstr).gs_derivtype));
    }
    s = NameOfTense(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfMood(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfVoice(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfGender(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfCase(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfDegree(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfPerson(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfNumber(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, NameOfNumber(wf));
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    DialectNames((*gstr).gs_dialect, &raw mut dialbuf as *mut ::core::ffi::c_char, dels);
    if dialbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        || *dels as ::core::ffi::c_int == '\t' as i32
    {
        strcat(buf, dels);
    }
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    GeogRegionNames(
        (*gstr).gs_geogregion,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
        dels,
    );
    if dialbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        || *dels as ::core::ffi::c_int == '\t' as i32
    {
        strcat(buf, dels);
    }
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    DomainNames(
        &raw mut (*gstr).st_domains as *mut ::core::ffi::c_char,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
        dels,
    );
    if dialbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        || *dels as ::core::ffi::c_int == '\t' as i32
    {
        strcat(buf, dels);
    }
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    MorphNames(
        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
        dels,
        pretty,
    );
    if dialbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        || *dels as ::core::ffi::c_int == '\t' as i32
    {
        strcat(buf, dels);
    }
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn GregSprintGkFlags(
    mut gstr: *mut gk_string,
    mut buf: *mut ::core::ffi::c_char,
    mut dels: *mut ::core::ffi::c_char,
    mut more_dels: ::core::ffi::c_int,
    mut pretty: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dialbuf: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wf: word_form = word_form {
        f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [0; 4],
    };
    wf = (*gstr).gs_forminfo;
    s = NameOfTense(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfMood(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfVoice(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfGender(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfCase(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfDegree(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfPerson(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, s);
    }
    s = NameOfNumber(wf);
    if *s as ::core::ffi::c_int != 0 || *dels as ::core::ffi::c_int == '\t' as i32 {
        strcat(buf, dels);
    }
    if *s != 0 {
        strcat(buf, NameOfNumber(wf));
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    DialectNames((*gstr).gs_dialect, &raw mut dialbuf as *mut ::core::ffi::c_char, dels);
    strcat(buf, b"\t\0" as *const u8 as *const ::core::ffi::c_char);
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    GeogRegionNames(
        (*gstr).gs_geogregion,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
        dels,
    );
    if dialbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        || *dels as ::core::ffi::c_int == '\t' as i32
    {
        strcat(buf, dels);
    }
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    DomainNames(
        &raw mut (*gstr).st_domains as *mut ::core::ffi::c_char,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
        dels,
    );
    if dialbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
        || *dels as ::core::ffi::c_int == '\t' as i32
    {
        strcat(buf, dels);
    }
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    dialbuf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    MorphNames(
        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        &raw mut dialbuf as *mut ::core::ffi::c_char,
        dels,
        pretty,
    );
    strcat(buf, b"\t\0" as *const u8 as *const ::core::ffi::c_char);
    if dialbuf[0 as ::core::ffi::c_int as usize] != 0 {
        strcat(buf, &raw mut dialbuf as *mut ::core::ffi::c_char);
    }
    s = NameOfStemtype((*gstr).gs_steminfo);
    strcat(buf, b"\t\0" as *const u8 as *const ::core::ffi::c_char);
    if *s != 0 {
        strcat(buf, NameOfStemtype((*gstr).gs_steminfo));
    }
    s = NameOfDerivtype((*gstr).gs_derivtype);
    if *s != 0 {
        strcat(buf, b",\0" as *const u8 as *const ::core::ffi::c_char);
        strcat(buf, NameOfDerivtype((*gstr).gs_derivtype));
    }
    return 0;
}
