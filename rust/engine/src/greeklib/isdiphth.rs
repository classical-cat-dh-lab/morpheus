extern "C" {
    fn isupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tolower(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type bool_0 = ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn is_diphth(
    mut p: *mut ::core::ffi::c_char,
    mut word: *mut ::core::ffi::c_char,
) -> bool_0 {
    let mut c1: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    if p.offset(-(1 as ::core::ffi::c_int as isize)) < word {
        return 0 as bool_0;
    }
    if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == DIAERESIS {
        return 0 as bool_0;
    }
    c2 = *p as ::core::ffi::c_int;
    if if 0 as ::core::ffi::c_int != 0 {
        isupper(c2)
    } else {
        ((c2 as ::core::ffi::c_uint).wrapping_sub('A' as i32 as ::core::ffi::c_uint)
            < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
    } != 0
    {
        c2 = tolower(c2);
    }
    if c2 != 'i' as i32 && c2 != 'u' as i32 {
        return 0 as bool_0;
    }
    c1 = *p.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
    if if 0 as ::core::ffi::c_int != 0 {
        isupper(c1)
    } else {
        ((c1 as ::core::ffi::c_uint).wrapping_sub('A' as i32 as ::core::ffi::c_uint)
            < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
    } != 0
    {
        c1 = tolower(c1);
    }
    if !(c1 == 'a' as i32 || c1 == 'e' as i32 || c1 == 'i' as i32 || c1 == 'o' as i32
        || c1 == 'u' as i32 || c1 == 'A' as i32 || c1 == 'E' as i32 || c1 == 'I' as i32
        || c1 == 'O' as i32 || c1 == 'U' as i32
        || (c1 == 'h' as i32 || c1 == 'w' as i32 || c1 == 'H' as i32
            || c1 == 'W' as i32))
    {
        return 0 as bool_0;
    }
    if c1 == 'a' as i32 || c1 == 'e' as i32 || c1 == 'o' as i32 {
        return 1 as bool_0;
    }
    if c2 == 'i' as i32 {
        return (c1 == 'u' as i32) as ::core::ffi::c_int;
    }
    if c2 == 'u' as i32 {
        return (c1 == 'h' as i32) as ::core::ffi::c_int;
    }
    return 0 as bool_0;
}
#[no_mangle]
pub unsafe extern "C" fn starts_w_diphth(
    mut stem: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return is_diphth(stem.offset(1 as ::core::ffi::c_int as isize), stem)
        as ::core::ffi::c_int;
}
pub const YES: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DIAERESIS: ::core::ffi::c_int = '+' as i32;
