extern "C" {
    fn cinsert(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn addbreath(
    mut w: *mut ::core::ffi::c_char,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match *w as ::core::ffi::c_int {
        97 | 101 | 111 => {
            if *w.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != DIAERESIS
                && (*w.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'i' as i32
                    || *w.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'u' as i32)
            {
                w = w.offset(2 as ::core::ffi::c_int as isize);
            } else {
                w = w.offset(1);
            }
        }
        119 | 104 => {
            if *w.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'u' as i32
            {
                w = w.offset(2 as ::core::ffi::c_int as isize);
            } else {
                w = w.offset(1);
            }
        }
        _ => {
            if *w as ::core::ffi::c_int == 'a' as i32
                || *w as ::core::ffi::c_int == 'e' as i32
                || *w as ::core::ffi::c_int == 'i' as i32
                || *w as ::core::ffi::c_int == 'o' as i32
                || *w as ::core::ffi::c_int == 'u' as i32
                || *w as ::core::ffi::c_int == 'A' as i32
                || *w as ::core::ffi::c_int == 'E' as i32
                || *w as ::core::ffi::c_int == 'I' as i32
                || *w as ::core::ffi::c_int == 'O' as i32
                || *w as ::core::ffi::c_int == 'U' as i32
                || (*w as ::core::ffi::c_int == 'h' as i32
                    || *w as ::core::ffi::c_int == 'w' as i32
                    || *w as ::core::ffi::c_int == 'H' as i32
                    || *w as ::core::ffi::c_int == 'W' as i32)
            {
                w = w.offset(1);
            }
        }
    }
    if c == ROUGHBR || c == SMOOTHBR {
        cinsert(c, w);
    }
    return 0;
}
pub const DIAERESIS: ::core::ffi::c_int = '+' as i32;
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
