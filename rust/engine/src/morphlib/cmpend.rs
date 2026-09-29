extern "C" {
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn cmpend(
    mut word: *mut ::core::ffi::c_char,
    mut ending: *mut ::core::ffi::c_char,
    mut stem: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s1: *mut ::core::ffi::c_char = word;
    let mut s2: *mut ::core::ffi::c_char = ending;
    if *s1 == 0 || *s2 == 0 {
        return 0 as ::core::ffi::c_int;
    }
    while *s1 != 0 {
        s1 = s1.offset(1);
    }
    s1 = s1.offset(-1);
    while *s2 != 0 {
        s2 = s2.offset(1);
    }
    s2 = s2.offset(-1);
    while s2 >= ending {
        if s1 <= word {
            return 0 as ::core::ffi::c_int;
        }
        if *s1 as ::core::ffi::c_int != *s2 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if s2 == ending {
            break;
        }
        s1 = s1.offset(-1);
        s2 = s2.offset(-1);
    }
    if s2 == ending {
        strcpy(stem, word);
        *stem.offset(s1.offset_from(word) as ::core::ffi::c_long as isize) = 0
            as ::core::ffi::c_char;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
