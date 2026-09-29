extern "C" {
    fn fseek(
        _: *mut FILE,
        _: ::core::ffi::c_long,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn getlemmstart(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_long,
    ) -> *mut FILE;
    fn Xstrncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn is_substring(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn is_blank(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn morphstrcmp(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn morphstrncmp(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn nextkey(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripquant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn xFclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn xFree(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
pub type bool_0 = ::core::ffi::c_int;
pub const BUFSIZ: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const MAXWORDSIZE: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const COMMENT_CHAR: ::core::ffi::c_int = '#' as i32;
pub const LONGSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const LEMMTAG: [::core::ffi::c_char; 5] = unsafe {
    ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b":le:\0")
};
#[no_mangle]
pub unsafe extern "C" fn dictstems(
    mut lemma: *mut ::core::ffi::c_char,
    mut nstems: *mut ::core::ffi::c_int,
    mut wantacc: bool_0,
    mut orgstem: *mut ::core::ffi::c_char,
    mut stemtype: *mut ::core::ffi::c_char,
    mut pparttab: *mut *mut ::core::ffi::c_char,
    mut maxpparts: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut lemmfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cstem: [::core::ffi::c_char; 60] = [0; 60];
    let mut wantstem: [::core::ffi::c_char; 60] = [0; 60];
    let mut curtarget: [::core::ffi::c_char; 120] = [0; 120];
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut slen: ::core::ffi::c_int = 0;
    let mut startoff: ::core::ffi::c_long = 0;
    let mut gotpparts: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut anystem: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    line = malloc(
        ((BUFSIZ * 4 as ::core::ffi::c_int) as size_t).wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    lemmfile = malloc((LONGSTRING as size_t).wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    tmp = malloc(
        ((BUFSIZ * 4 as ::core::ffi::c_int) as size_t).wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    let ref mut fresh0 = *tmp
        .offset((BUFSIZ * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
    *fresh0 = 0 as ::core::ffi::c_char;
    let ref mut fresh1 = *lemmfile
        .offset((LONGSTRING + 1 as ::core::ffi::c_int) as isize);
    *fresh1 = *fresh0;
    *line
        .offset((BUFSIZ * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) = *fresh1;
    *lemmfile.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    startoff = 0 as ::core::ffi::c_long;
    if stemtype.is_null() || *stemtype == 0 {
        anystem += 1;
    }
    Xstrncpy(
        &raw mut wantstem as *mut ::core::ffi::c_char,
        orgstem,
        (BUFSIZ * 4 as ::core::ffi::c_int) as size_t,
    );
    stripquant(&raw mut wantstem as *mut ::core::ffi::c_char);
    if wantacc == NO {
        stripacc(&raw mut wantstem as *mut ::core::ffi::c_char);
    }
    f = getlemmstart(lemma, lemmfile, &raw mut startoff);
    if f.is_null() {
        gotpparts = -(1 as ::core::ffi::c_int);
    } else {
        fseek(f, startoff, 0 as ::core::ffi::c_int);
        slen = Xstrlen(&raw mut wantstem as *mut ::core::ffi::c_char);
        gotpparts = 0 as ::core::ffi::c_int;
        while !fgets(line, BUFSIZ * 4 as ::core::ffi::c_int, f).is_null()
            && gotpparts < maxpparts
        {
            if is_blank(line) != 0 {
                break;
            }
            if *line as ::core::ffi::c_int == COMMENT_CHAR {
                continue;
            }
            if morphstrncmp(
                line,
                LEMMTAG.as_ptr() as *mut ::core::ffi::c_char,
                strlen(LEMMTAG.as_ptr()),
            ) == 0
            {
                continue;
            }
            Xstrncpy(
                tmp,
                line.offset(4 as ::core::ffi::c_int as isize),
                (BUFSIZ * 4 as ::core::ffi::c_int) as size_t,
            );
            nextkey(tmp, &raw mut cstem as *mut ::core::ffi::c_char);
            if wantacc == NO {
                stripacc(&raw mut cstem as *mut ::core::ffi::c_char);
            }
            stripquant(&raw mut cstem as *mut ::core::ffi::c_char);
            *nstems += 1;
            if morphstrcmp(
                &raw mut cstem as *mut ::core::ffi::c_char,
                &raw mut wantstem as *mut ::core::ffi::c_char,
            ) == 0
            {
                if anystem != 0 {
                    let fresh2 = gotpparts;
                    gotpparts = gotpparts + 1;
                    Xstrncpy(
                        *pparttab.offset(fresh2 as isize),
                        line.offset(4 as ::core::ffi::c_int as isize),
                        MAXWORDSIZE as size_t,
                    );
                } else if !is_substring(stemtype, tmp).is_null() {
                    let fresh3 = gotpparts;
                    gotpparts = gotpparts + 1;
                    Xstrncpy(
                        *pparttab.offset(fresh3 as isize),
                        line.offset(4 as ::core::ffi::c_int as isize),
                        MAXWORDSIZE as size_t,
                    );
                }
            }
        }
    }
    if !f.is_null() {
        xFclose(f);
        f = ::core::ptr::null_mut::<FILE>();
    }
    xFree(
        line,
        b"dicts line\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    xFree(
        lemmfile,
        b"dicts lemmfile\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    xFree(
        tmp,
        b"dicts tmp\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    );
    tmp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    lemmfile = tmp;
    line = lemmfile;
    return gotpparts;
}
