extern "C" {
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stripzeroend(
    mut word: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    n = Xstrlen(word);
    if *word.offset(n as isize).offset(-(1 as ::core::ffi::c_int as isize))
        as ::core::ffi::c_int == '*' as i32
    {
        *word.offset(n as isize).offset(-(1 as ::core::ffi::c_int as isize)) = 0
            as ::core::ffi::c_char;
    }
    return 0;
}
