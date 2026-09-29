extern "C" {
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn set_gkorder(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
static mut comptab: [::core::ffi::c_char; 128] = [0; 128];
static mut betatab: [::core::ffi::c_char; 128] = [0; 128];
static mut tabinited: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut betatabinited: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn morphstrcmp(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if tabinited == 0 {
        init_comptab();
    }
    loop {
        if comptab[*s1 as usize] as ::core::ffi::c_int
            != comptab[*s2 as usize] as ::core::ffi::c_int
        {
            return comptab[*s1 as usize] as ::core::ffi::c_int
                - comptab[*s2 as usize] as ::core::ffi::c_int;
        }
        if *s1 as ::core::ffi::c_int == '\0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn betastrcmp(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if betatabinited == 0 {
        init_betatab();
    }
    loop {
        while *s1 as ::core::ffi::c_int != 0 && betatab[*s1 as usize] == 0 {
            s1 = s1.offset(1);
        }
        while *s2 as ::core::ffi::c_int != 0 && betatab[*s2 as usize] == 0 {
            s2 = s2.offset(1);
        }
        if betatab[*s1 as usize] as ::core::ffi::c_int
            != betatab[*s2 as usize] as ::core::ffi::c_int
        {
            return betatab[*s1 as usize] as ::core::ffi::c_int
                - betatab[*s2 as usize] as ::core::ffi::c_int;
        }
        if *s1 as ::core::ffi::c_int == '\0' as i32 {
            return strcmp(s1, s2);
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn morphstrncmp(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *mut ::core::ffi::c_char,
    mut n: size_t,
) -> ::core::ffi::c_int {
    if tabinited == 0 {
        init_comptab();
    }
    if n <= 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        n = n.wrapping_sub(1);
        if !(n != 0
            && comptab[*s1 as usize] as ::core::ffi::c_int
                == comptab[*s2 as usize] as ::core::ffi::c_int)
        {
            break;
        }
        if *s1 == 0 {
            break;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    }
    return comptab[*s1 as usize] as ::core::ffi::c_int
        - comptab[*s2 as usize] as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dictstrcmp(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut t1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut t2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    t1 = s1;
    t2 = s2;
    if tabinited == 0 {
        init_comptab();
    }
    loop {
        while (if 0 as ::core::ffi::c_int != 0 {
            isalpha(*s1 as ::core::ffi::c_int)
        } else {
            ((*s1 as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) == 0 && *s1 as ::core::ffi::c_int != '|' as i32
            && *s1 as ::core::ffi::c_int != HARDLONG && *s1 as ::core::ffi::c_int != 0
        {
            s1 = s1.offset(1);
        }
        while (if 0 as ::core::ffi::c_int != 0 {
            isalpha(*s2 as ::core::ffi::c_int)
        } else {
            ((*s2 as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) == 0 && *s2 as ::core::ffi::c_int != '|' as i32
            && *s2 as ::core::ffi::c_int != HARDLONG && *s2 as ::core::ffi::c_int != 0
        {
            s2 = s2.offset(1);
        }
        if comptab[*s1 as usize] as ::core::ffi::c_int
            != comptab[*s2 as usize] as ::core::ffi::c_int
        {
            return comptab[*s1 as usize] as ::core::ffi::c_int
                - comptab[*s2 as usize] as ::core::ffi::c_int;
        }
        if *s1 as ::core::ffi::c_int == '\0' as i32
            || __isspace(*s1 as ::core::ffi::c_int) != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn dictstrncmp(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *mut ::core::ffi::c_char,
    mut n: size_t,
) -> ::core::ffi::c_int {
    let mut b1: [::core::ffi::c_char; 512] = [0; 512];
    let mut b2: [::core::ffi::c_char; 512] = [0; 512];
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    s = &raw mut b1 as *mut ::core::ffi::c_char;
    while *s1 != 0 {
        if if 0 as ::core::ffi::c_int != 0 {
            isalpha(*s1 as ::core::ffi::c_int)
        } else {
            ((*s1 as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        } == 0
        {
            s1 = s1.offset(1);
        } else {
            let fresh0 = s1;
            s1 = s1.offset(1);
            let fresh1 = s;
            s = s.offset(1);
            *fresh1 = *fresh0;
        }
    }
    *s = 0 as ::core::ffi::c_char;
    s = &raw mut b2 as *mut ::core::ffi::c_char;
    while *s1 != 0 {
        if if 0 as ::core::ffi::c_int != 0 {
            isalpha(*s2 as ::core::ffi::c_int)
        } else {
            ((*s2 as ::core::ffi::c_uint | 32 as ::core::ffi::c_uint)
                .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        } == 0
        {
            s2 = s2.offset(1);
        } else {
            let fresh2 = s2;
            s2 = s2.offset(1);
            let fresh3 = s;
            s = s.offset(1);
            *fresh3 = *fresh2;
        }
    }
    *s = 0 as ::core::ffi::c_char;
    return morphstrncmp(
        &raw mut b1 as *mut ::core::ffi::c_char,
        &raw mut b2 as *mut ::core::ffi::c_char,
        n,
    );
}
#[no_mangle]
pub unsafe extern "C" fn init_comptab() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    tabinited += 1;
    i = 0 as ::core::ffi::c_int;
    while i < 128 as ::core::ffi::c_int {
        comptab[i as usize] = i as ::core::ffi::c_char;
        i += 1;
    }
    comptab['|' as i32 as usize] = 'i' as i32 as ::core::ffi::c_char;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn init_betatab() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    betatabinited += 1;
    i = 0 as ::core::ffi::c_int;
    while i < 128 as ::core::ffi::c_int {
        betatab[i as usize] = i as ::core::ffi::c_char;
        i += 1;
    }
    betatab['|' as i32 as usize] = 'i' as i32 as ::core::ffi::c_char;
    set_gkorder(&raw mut betatab as *mut ::core::ffi::c_char);
    return 0;
}
pub const HARDLONG: ::core::ffi::c_int = '_' as i32;
