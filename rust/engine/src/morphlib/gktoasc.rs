#[no_mangle]
pub static mut gktoasc: [::core::ffi::c_char; 54] = [
    'a' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o1 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'b' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o2 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'g' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o3 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'd' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o4 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'e' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o5 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'z' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o6 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'h' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o7 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'q' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o10 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'i' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o11 as ::core::ffi::c_int) as ::core::ffi::c_char,
    '|' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o12 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'k' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o13 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'l' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o14 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'm' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o15 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'n' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o16 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'c' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o17 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'o' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o20 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'p' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o21 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'r' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o22 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'v' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o23 as ::core::ffi::c_int) as ::core::ffi::c_char,
    's' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o24 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'j' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o25 as ::core::ffi::c_int) as ::core::ffi::c_char,
    't' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o26 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'u' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o27 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'f' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o30 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'x' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o31 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'y' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o32 as ::core::ffi::c_int) as ::core::ffi::c_char,
    'w' as i32 as ::core::ffi::c_char,
    ('a' as i32 + 0o33 as ::core::ffi::c_int) as ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn set_gkorder(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as usize) < ::core::mem::size_of::<[::core::ffi::c_char; 54]>() as usize {
        *s.offset(gktoasc[i as usize] as isize) = gktoasc[(i + 1 as ::core::ffi::c_int)
            as usize];
        i += 2 as ::core::ffi::c_int;
    }
    return 0;
}
