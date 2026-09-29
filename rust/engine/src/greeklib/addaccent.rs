extern "C" {
    fn is_diphth(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char) -> bool_0;
    fn strsqz(_: *mut ::core::ffi::c_char, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cinsert(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type bool_0 = ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn addaccent(
    mut word: *mut ::core::ffi::c_char,
    mut accent: ::core::ffi::c_int,
    mut p: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut t: *mut ::core::ffi::c_char = p.offset(-(1 as ::core::ffi::c_int as isize));
    if *word as ::core::ffi::c_int == '*' as i32
        && (*word.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ROUGHBR
            || *word.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == SMOOTHBR)
    {
        if p == word.offset(2 as ::core::ffi::c_int as isize) {
            cinsert(accent, word.offset(2 as ::core::ffi::c_int as isize));
            return 0;
        }
        if p == word.offset(3 as ::core::ffi::c_int as isize)
            && is_diphth(p, word) == YES
        {
            cinsert(accent, word.offset(2 as ::core::ffi::c_int as isize));
            return 0;
        }
    }
    loop {
        p = p.offset(1);
        if !(*p as ::core::ffi::c_int == ROUGHBR || *p as ::core::ffi::c_int == SMOOTHBR
            || (*p as ::core::ffi::c_int == HARDLONG
                || *p as ::core::ffi::c_int == HARDSHORT))
        {
            break;
        }
    }
    if accent == CIRCUMFLEX
        && *p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == HARDLONG
    {
        strsqz(p.offset(-(1 as ::core::ffi::c_int as isize)), 1 as ::core::ffi::c_int);
        p = p.offset(-1);
    }
    cinsert(accent, p);
    return 0;
}
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CIRCUMFLEX: ::core::ffi::c_int = '=' as i32;
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
pub const HARDSHORT: ::core::ffi::c_int = '^' as i32;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
