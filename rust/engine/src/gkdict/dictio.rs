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
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn MorphFopen(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> *mut FILE;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn ChckFullIndex(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_long,
        _: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    ) -> ::core::ffi::c_int;
    fn ChckPreIndex(
        _: *mut endtags,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    ) -> ::core::ffi::c_long;
    fn ErrorMess(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    fn chckvstem(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn checkforderiv(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn is_blank(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn nextkey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn rstprevb(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut gk_string,
    ) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripdiaer(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripmetachars(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripquant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn trimwhite(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn xFclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn xFree(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn init_preind(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> *mut endtags;
    fn morphstrcmp(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn morphstrncmp(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: size_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct endtags {
    pub tagstring: [::core::ffi::c_char; 9],
    pub tagoffset: ::core::ffi::c_long,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
#[no_mangle]
pub static mut VbTags: *mut endtags = ::core::ptr::null::<endtags>() as *mut endtags;
#[no_mangle]
pub static mut NomTags: *mut endtags = ::core::ptr::null::<endtags>() as *mut endtags;
static mut num_of_ntags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut num_of_vtags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut vbindex: *mut ::core::ffi::c_char = VBINDEX.as_ptr()
    as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut nomindex: *mut ::core::ffi::c_char = NOMINDEX.as_ptr()
    as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut Use_hqdict: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn init_dict(
    mut fname: *mut ::core::ffi::c_char,
    mut ntags: *mut ::core::ffi::c_int,
) -> *mut endtags {
    if Use_hqdict != 0 {
        fname = STEMLIST.as_ptr() as *mut ::core::ffi::c_char;
        vbindex = STEMLIST.as_ptr() as *mut ::core::ffi::c_char;
        nomindex = STEMLIST.as_ptr() as *mut ::core::ffi::c_char;
    }
    return init_preind(fname, ntags);
}
#[no_mangle]
pub unsafe extern "C" fn chckirrverb(
    mut irregstr: *mut ::core::ffi::c_char,
    mut lemmas: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut workstem: [::core::ffi::c_char; 60] = [0; 60];
    let mut rval: ::core::ffi::c_int = 0;
    let mut startoff: ::core::ffi::c_long = 0;
    workstem[0 as ::core::ffi::c_int as usize] = '1' as i32 as ::core::ffi::c_char;
    Xstrncpy(
        (&raw mut workstem as *mut ::core::ffi::c_char)
            .offset(1 as ::core::ffi::c_int as isize),
        irregstr,
        MAXWORDSIZE as size_t,
    );
    stripmetachars(&raw mut workstem as *mut ::core::ffi::c_char);
    stripacc(&raw mut workstem as *mut ::core::ffi::c_char);
    rval = chckvstem(&raw mut workstem as *mut ::core::ffi::c_char, lemmas);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn chckindecl(
    mut indeclstr: *mut ::core::ffi::c_char,
    mut lemmas: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut startoff: ::core::ffi::c_long = 0;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmpindecl: [::core::ffi::c_char; 60] = [0; 60];
    *lemmas = 0 as ::core::ffi::c_char;
    if NomTags.is_null() {
        NomTags = init_dict(nomindex, &raw mut num_of_ntags);
    }
    tmpindecl[0 as ::core::ffi::c_int as usize] = '2' as i32 as ::core::ffi::c_char;
    Xstrncpy(
        (&raw mut tmpindecl as *mut ::core::ffi::c_char)
            .offset(1 as ::core::ffi::c_int as isize),
        indeclstr,
        MAXWORDSIZE as size_t,
    );
    stripquant(&raw mut tmpindecl as *mut ::core::ffi::c_char);
    stripdiaer(&raw mut tmpindecl as *mut ::core::ffi::c_char);
    stripacc(&raw mut tmpindecl as *mut ::core::ffi::c_char);
    startoff = ChckPreIndex(
        NomTags,
        &raw mut tmpindecl as *mut ::core::ffi::c_char,
        num_of_ntags,
        NO,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                morphstrcmp
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
    if startoff < 0 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if ChckFullIndex(
        &raw mut tmpindecl as *mut ::core::ffi::c_char,
        lemmas,
        nomindex,
        startoff,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                    size_t,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                morphstrncmp
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    ) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn chckderiv(
    mut derivstr: *mut ::core::ffi::c_char,
    mut derivkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut startoff: ::core::ffi::c_long = 0;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmpderivstr: [::core::ffi::c_char; 60] = [0; 60];
    tmpderivstr[0 as ::core::ffi::c_int as usize] = '3' as i32 as ::core::ffi::c_char;
    Xstrncpy(
        (&raw mut tmpderivstr as *mut ::core::ffi::c_char)
            .offset(1 as ::core::ffi::c_int as isize),
        derivstr,
        MAXWORDSIZE as size_t,
    );
    stripquant(&raw mut tmpderivstr as *mut ::core::ffi::c_char);
    stripdiaer(&raw mut tmpderivstr as *mut ::core::ffi::c_char);
    rval = chckvstem(&raw mut tmpderivstr as *mut ::core::ffi::c_char, derivkeys);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn chckstem(
    mut stemstr: *mut ::core::ffi::c_char,
    mut stemkeys: *mut ::core::ffi::c_char,
    mut is_nom: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut startoff: ::core::ffi::c_long = 0;
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rval2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut taglen: ::core::ffi::c_int = 0;
    let mut curntags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmpkeys: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut indfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut CurTags: *mut endtags = ::core::ptr::null_mut::<endtags>();
    stripquant(stemstr);
    stripdiaer(stemstr);
    tmpkeys[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    *stemkeys = tmpkeys[0 as ::core::ffi::c_int as usize];
    if is_nom != 0 {
        indfile = nomindex;
    } else {
        rval = chckvstem(stemstr, stemkeys);
        rval2 = checkforderiv(stemstr, &raw mut tmpkeys as *mut ::core::ffi::c_char);
        if rval2 != 0 {
            if rval != 0 {
                Xstrncat(
                    stemkeys,
                    b" \0" as *const u8 as *const ::core::ffi::c_char,
                    LONGSTRING as size_t,
                );
            }
            Xstrncat(
                stemkeys,
                &raw mut tmpkeys as *mut ::core::ffi::c_char,
                LONGSTRING as size_t,
            );
        }
        return rval + rval2;
    }
    if *stemstr == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if VbTags.is_null() {
        VbTags = init_dict(vbindex, &raw mut num_of_vtags);
    }
    if NomTags.is_null() {
        NomTags = init_dict(nomindex, &raw mut num_of_ntags);
    }
    if is_nom != 0 {
        CurTags = NomTags;
        curntags = num_of_ntags;
    } else {
        CurTags = VbTags;
        curntags = num_of_vtags;
    }
    taglen = Xstrlen(stemstr);
    startoff = ChckPreIndex(
        CurTags,
        stemstr,
        curntags,
        if is_nom != 0 { NO } else { YES },
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                morphstrcmp
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
    if startoff >= 0 as ::core::ffi::c_long {
        rval = ChckFullIndex(
            stemstr,
            stemkeys,
            indfile,
            startoff,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                        size_t,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
            >(
                Some(
                    morphstrncmp
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_char,
                            *mut ::core::ffi::c_char,
                            size_t,
                        ) -> ::core::ffi::c_int,
                ),
            ),
        );
    }
    return rval;
}
#[no_mangle]
pub static mut LemmTags: *mut endtags = ::core::ptr::null::<endtags>() as *mut endtags;
static mut num_of_ltags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn prntlemmentry(
    mut lemma: *mut ::core::ffi::c_char,
    mut preverb: *mut ::core::ffi::c_char,
    mut f: *mut FILE,
) -> ::core::ffi::c_int {
    let mut startoff: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut lemmfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut fword: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fword = getlemmstart(lemma, lemmfile, &raw mut startoff);
    if fword.is_null() {
        sprintf(
            line,
            b"No Lemma found under [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
            lemma,
        );
        ErrorMess(line);
        return -(1 as ::core::ffi::c_int);
    }
    lemmfile = malloc(LONGSTRING as size_t) as *mut ::core::ffi::c_char;
    line = malloc(LONGSTRING as size_t) as *mut ::core::ffi::c_char;
    *lemmfile = 0 as ::core::ffi::c_char;
    *line = *lemmfile;
    while !fgets(line, LONGSTRING, fword).is_null() {
        if is_blank(line) != 0 {
            fprintf(f, b"\n\n\0" as *const u8 as *const ::core::ffi::c_char);
            break;
        } else {
            trimwhite(line);
            if *preverb as ::core::ffi::c_int != 0
                && Xstrncmp(line, LEMMTAG.as_ptr(), Xstrlen(LEMMTAG.as_ptr()) as size_t)
                    == 0
            {
                rstprevb(
                    line.offset(Xstrlen(LEMMTAG.as_ptr()) as isize),
                    preverb,
                    ::core::ptr::null_mut::<gk_string>(),
                );
                fprintf(f, b"%s\n\0" as *const u8 as *const ::core::ffi::c_char, line);
            } else if !preverb.is_null() && *preverb as ::core::ffi::c_int != 0 {
                fprintf(
                    f,
                    b"%s\tpb:%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    line,
                    preverb,
                );
            } else {
                fprintf(f, b"%s\n\0" as *const u8 as *const ::core::ffi::c_char, line);
            }
        }
    }
    xFclose(fword);
    fword = ::core::ptr::null_mut::<FILE>();
    xFree(
        line,
        b"line prntlem\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    xFree(
        lemmfile,
        b"lemmfile prntlem\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    lemmfile = ::core::ptr::null_mut::<::core::ffi::c_char>();
    line = lemmfile;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getlemmstart(
    mut lemma: *mut ::core::ffi::c_char,
    mut lemmfile: *mut ::core::ffi::c_char,
    mut lemmoff: *mut ::core::ffi::c_long,
) -> *mut FILE {
    let mut curtarget: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut curoff: ::core::ffi::c_long = 0;
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut startoff: ::core::ffi::c_long = 0;
    let mut comp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut shorttag: [::core::ffi::c_char; 60] = [0; 60];
    if LemmTags.is_null() {
        LemmTags = init_preind(
            WORDLIST.as_ptr() as *mut ::core::ffi::c_char,
            &raw mut num_of_ltags,
        );
    }
    Xstrncpy(
        &raw mut shorttag as *mut ::core::ffi::c_char,
        lemma,
        MAXWORDSIZE as size_t,
    );
    stripquant(&raw mut shorttag as *mut ::core::ffi::c_char);
    shorttag[6 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    startoff = ChckPreIndex(
        LemmTags,
        &raw mut shorttag as *mut ::core::ffi::c_char,
        num_of_ltags,
        NO,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_char,
                    *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                morphstrcmp
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ),
    );
    f = MorphFopen(
        WORDLIST.as_ptr() as *mut ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"getlemmstart: could not find %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut line as *mut ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<FILE>();
    }
    fseek(f, startoff, 0 as ::core::ffi::c_int);
    Xstrncpy(
        &raw mut shorttag as *mut ::core::ffi::c_char,
        lemma,
        MAXWORDSIZE as size_t,
    );
    stripquant(&raw mut shorttag as *mut ::core::ffi::c_char);
    sprintf(
        &raw mut curtarget as *mut ::core::ffi::c_char,
        b":le:%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut shorttag as *mut ::core::ffi::c_char,
    );
    loop {
        let mut curlemm: [::core::ffi::c_char; 60] = [0; 60];
        curoff = ftell(f);
        if fgets(
                &raw mut line as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                    as ::core::ffi::c_int,
                f,
            )
            .is_null()
        {
            *lemmoff = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
            break;
        } else {
            if Xstrncmp(
                &raw mut line as *mut ::core::ffi::c_char,
                LEMMTAG.as_ptr(),
                4 as size_t,
            ) != 0
            {
                continue;
            }
            trimwhite(&raw mut line as *mut ::core::ffi::c_char);
            nextkey(
                &raw mut line as *mut ::core::ffi::c_char,
                &raw mut curlemm as *mut ::core::ffi::c_char,
            );
            comp = morphstrcmp(
                &raw mut curtarget as *mut ::core::ffi::c_char,
                &raw mut curlemm as *mut ::core::ffi::c_char,
            );
            if comp == 0 {
                *lemmoff = curoff;
                break;
            } else {
                if !(comp < 0 as ::core::ffi::c_int) {
                    continue;
                }
                *lemmoff = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
                break;
            }
        }
    }
    if *lemmoff < 0 as ::core::ffi::c_long {
        *lemmfile = 0 as ::core::ffi::c_char;
        xFclose(f);
        f = ::core::ptr::null_mut::<FILE>();
        return ::core::ptr::null_mut::<FILE>();
    }
    Xstrncpy(lemmfile, WORDLIST.as_ptr(), MAXWORDSIZE as size_t);
    fseek(f, *lemmoff, 0 as ::core::ffi::c_int);
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn lemma_exists(
    mut lemma: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut flemm: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut lemmfile: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut lemmoff: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    flemm = getlemmstart(
        lemma,
        &raw mut lemmfile as *mut ::core::ffi::c_char,
        &raw mut lemmoff,
    );
    if flemm.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    xFclose(flemm);
    flemm = ::core::ptr::null_mut::<FILE>();
    return 1 as ::core::ffi::c_int;
}
pub const WORDLIST: [::core::ffi::c_char; 14] = unsafe {
    ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"hqdict/hqdict\0")
};
pub const NOMINDEX: [::core::ffi::c_char; 16] = unsafe {
    ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"steminds/nomind\0")
};
pub const VBINDEX: [::core::ffi::c_char; 15] = unsafe {
    ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"steminds/vbind\0")
};
pub const STEMLIST: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<
        [u8; 23],
        [::core::ffi::c_char; 23],
    >(*b"hqdict/indices/stindex\0")
};
pub const LEMMTAG: [::core::ffi::c_char; 5] = unsafe {
    ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b":le:\0")
};
