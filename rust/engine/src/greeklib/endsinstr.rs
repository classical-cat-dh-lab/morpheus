extern "C" {
    fn Xstrlen(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn Xstrncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripdiaer(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub const LOCBUF: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ends_in(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut p2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut n: ::core::ffi::c_int = 0;
    let mut tmp1: [::core::ffi::c_char; 256] = [0; 256];
    let mut tmp2: [::core::ffi::c_char; 256] = [0; 256];
    let mut b1: [::core::ffi::c_char; 256] = [0; 256];
    let mut b2: [::core::ffi::c_char; 256] = [0; 256];
    let mut i: ::core::ffi::c_int = 0;
    strcpy(&raw mut tmp1 as *mut ::core::ffi::c_char, s1);
    s1 = &raw mut tmp1 as *mut ::core::ffi::c_char;
    strcpy(&raw mut tmp2 as *mut ::core::ffi::c_char, s2);
    s2 = &raw mut tmp2 as *mut ::core::ffi::c_char;
    stripacc(&raw mut tmp1 as *mut ::core::ffi::c_char);
    stripacc(&raw mut tmp2 as *mut ::core::ffi::c_char);
    stripdiaer(&raw mut tmp1 as *mut ::core::ffi::c_char);
    stripdiaer(&raw mut tmp2 as *mut ::core::ffi::c_char);
    n = Xstrlen(s1);
    if n > LOCBUF {
        p1 = s1.offset(LOCBUF as isize).offset(-(1 as ::core::ffi::c_int as isize));
    } else {
        p1 = s1.offset(n as isize).offset(-(1 as ::core::ffi::c_int as isize));
    }
    i = 0 as ::core::ffi::c_int;
    while !(p1 < s1) {
        let fresh0 = p1;
        p1 = p1.offset(-1);
        b1[i as usize] = *fresh0;
        i += 1;
    }
    b1[i as usize] = 0 as ::core::ffi::c_char;
    n = Xstrlen(s2);
    if n > LOCBUF {
        p2 = s2.offset(LOCBUF as isize).offset(-(1 as ::core::ffi::c_int as isize));
    } else {
        p2 = s2.offset(n as isize).offset(-(1 as ::core::ffi::c_int as isize));
    }
    i = 0 as ::core::ffi::c_int;
    while !(p2 < s2) {
        let fresh1 = p2;
        p2 = p2.offset(-1);
        b2[i as usize] = *fresh1;
        i += 1;
    }
    b2[i as usize] = 0 as ::core::ffi::c_char;
    return if Xstrncmp(
        &raw mut b2 as *mut ::core::ffi::c_char,
        &raw mut b1 as *mut ::core::ffi::c_char,
        Xstrlen(&raw mut b2 as *mut ::core::ffi::c_char) as size_t,
    ) != 0
    {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
}
