extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn putchar(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn comstemtypes(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn stripacc(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn stripquant(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
pub const COMPHEADS: [::core::ffi::c_char; 15] = unsafe {
    ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"/tmp/nom.heads\0")
};
pub const ROUGHBR: ::core::ffi::c_int = '(' as i32;
pub const SMOOTHBR: ::core::ffi::c_int = ')' as i32;
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkforcompnoun(
    mut curstem: *mut ::core::ffi::c_char,
    mut endkeys: *mut ::core::ffi::c_char,
    mut stemkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = curstem;
    let mut headkeys: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut firsth: [::core::ffi::c_char; 1024] = [0; 1024];
    if *s != 0 {
        s = s.offset(1);
    }
    while *s != 0 {
        if strlen(s) < 3 as size_t {
            break;
        }
        if (*s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
            == 'a' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'e' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'i' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'o' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'u' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'A' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'E' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'I' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'O' as i32
            || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'U' as i32
            || (*s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == 'h' as i32
                || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'w' as i32
                || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'H' as i32
                || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == 'W' as i32)
            || (*s as ::core::ffi::c_int == 'a' as i32
                || *s as ::core::ffi::c_int == 'e' as i32
                || *s as ::core::ffi::c_int == 'i' as i32
                || *s as ::core::ffi::c_int == 'o' as i32
                || *s as ::core::ffi::c_int == 'u' as i32
                || *s as ::core::ffi::c_int == 'A' as i32
                || *s as ::core::ffi::c_int == 'E' as i32
                || *s as ::core::ffi::c_int == 'I' as i32
                || *s as ::core::ffi::c_int == 'O' as i32
                || *s as ::core::ffi::c_int == 'U' as i32
                || (*s as ::core::ffi::c_int == 'h' as i32
                    || *s as ::core::ffi::c_int == 'w' as i32
                    || *s as ::core::ffi::c_int == 'H' as i32
                    || *s as ::core::ffi::c_int == 'W' as i32))
            || (*s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                == ROUGHBR
                || *s.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    == SMOOTHBR))
            && is_nomhead(s, &raw mut headkeys as *mut ::core::ffi::c_char) != 0
        {
            printf(
                b"[%s] [%s] [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
                curstem,
                &raw mut headkeys as *mut ::core::ffi::c_char,
                endkeys,
            );
            n = comstemtypes(
                curstem,
                &raw mut headkeys as *mut ::core::ffi::c_char,
                endkeys,
            );
            strcpy(&raw mut firsth as *mut ::core::ffi::c_char, curstem);
            firsth[strlen(curstem).wrapping_sub(strlen(s)) as usize] = 0
                as ::core::ffi::c_char;
            if n != 0 {
                let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                p = &raw mut headkeys as *mut ::core::ffi::c_char;
                while *p != 0 {
                    while __isspace(*p as ::core::ffi::c_int) != 0 {
                        p = p.offset(1);
                    }
                    if *p == 0 {
                        break;
                    }
                    printf(
                        b"%s-\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut firsth as *mut ::core::ffi::c_char,
                    );
                    if *p as ::core::ffi::c_int == ':' as i32 {
                        p = p.offset(1);
                    }
                    while *p as ::core::ffi::c_int != 0
                        && __isspace(*p as ::core::ffi::c_int) == 0
                    {
                        let fresh2 = p;
                        p = p.offset(1);
                        putchar(*fresh2 as ::core::ffi::c_int);
                    }
                    putchar('\n' as i32);
                }
            }
        }
        s = s.offset(1);
    }
    return 0;
}
#[no_mangle]
pub static mut headtab: [*mut ::core::ffi::c_char; 10000] = [::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char; 10000];
static mut init_headtab: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut nheads: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn setup_headtab() -> ::core::ffi::c_int {
    let mut fheads: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut line: [::core::ffi::c_char; 1000] = [0; 1000];
    if init_headtab != 0 {
        return 1 as ::core::ffi::c_int;
    }
    fheads = fopen(COMPHEADS.as_ptr(), b"r\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut FILE;
    if fheads.is_null() {
        fprintf(
            stderr,
            b"could not open [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
            COMPHEADS.as_ptr(),
        );
        return 0 as ::core::ffi::c_int;
    }
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1000]>() as ::core::ffi::c_int,
            fheads,
        )
        .is_null()
    {
        if line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '#' as i32 {
            continue;
        }
        headtab[nheads as usize] = malloc(
            strlen(&raw mut line as *mut ::core::ffi::c_char),
        ) as *mut ::core::ffi::c_char;
        line[strlen(&raw mut line as *mut ::core::ffi::c_char).wrapping_sub(1 as size_t)
            as usize] = 0 as ::core::ffi::c_char;
        strcpy(
            headtab[nheads as usize],
            (&raw mut line as *mut ::core::ffi::c_char)
                .offset(1 as ::core::ffi::c_int as isize),
        );
        nheads += 1;
    }
    fclose(fheads);
    init_headtab = 1 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_nomhead(
    mut heads: *mut ::core::ffi::c_char,
    mut headkeys: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmphead: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tmptab: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut i: ::core::ffi::c_int = 0;
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if init_headtab == 0 {
        setup_headtab();
    }
    strcpy(&raw mut tmphead as *mut ::core::ffi::c_char, heads);
    stripacc(&raw mut tmphead as *mut ::core::ffi::c_char);
    strcat(
        &raw mut tmphead as *mut ::core::ffi::c_char,
        b"\t\0" as *const u8 as *const ::core::ffi::c_char,
    );
    *headkeys.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < nheads {
        strcpy(&raw mut tmptab as *mut ::core::ffi::c_char, headtab[i as usize]);
        stripquant(&raw mut tmptab as *mut ::core::ffi::c_char);
        if strncmp(
            &raw mut tmptab as *mut ::core::ffi::c_char,
            &raw mut tmphead as *mut ::core::ffi::c_char,
            strlen(&raw mut tmphead as *mut ::core::ffi::c_char),
        ) == 0
        {
            s = headtab[i as usize]
                .offset(strlen(&raw mut tmphead as *mut ::core::ffi::c_char) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize));
            while __isspace(*s as ::core::ffi::c_int) != 0 {
                let fresh0 = s;
                s = s.offset(1);
                *fresh0 = ':' as i32 as ::core::ffi::c_char;
            }
            while *s as ::core::ffi::c_int != 0
                && __isspace(*s as ::core::ffi::c_int) == 0
            {
                s = s.offset(1);
            }
            while __isspace(*s as ::core::ffi::c_int) != 0 {
                let fresh1 = s;
                s = s.offset(1);
                *fresh1 = ':' as i32 as ::core::ffi::c_char;
            }
            strcat(headkeys, headtab[i as usize]);
            strcat(headkeys, b" \0" as *const u8 as *const ::core::ffi::c_char);
            rval = 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return rval;
}
