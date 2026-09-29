#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[allow(unused_imports)]

extern "C" {
    #[cfg_attr(target_os = "macos", link_name = "__stdinp")]
    static stdin: *mut FILE;
    #[cfg_attr(target_os = "macos", link_name = "__stdoutp")]
    static stdout: *mut FILE;
    #[cfg_attr(target_os = "macos", link_name = "__stderrp")]
    static stderr: *mut FILE;
    fn fopen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(_: *mut FILE) -> ::core::ffi::c_int;
    fn fgets(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn getopt(
        _: ::core::ffi::c_int,
        _: *const *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn anal_buf() -> *mut ::core::ffi::c_char;
    fn addbreath(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn checkstring(
        _: *mut ::core::ffi::c_char,
        _: PrntFlags,
        _: *mut FILE,
    ) -> ::core::ffi::c_int;
    fn cur_lang() -> ::core::ffi::c_int;
    fn set_lang(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn show_totanals() -> ::core::ffi::c_int;
    fn show_totlems() -> ::core::ffi::c_int;
    fn stripbreath(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn trimwhite(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn clock() -> clock_t;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub __x: ::core::ffi::c_char,
}
pub type FILE = _IO_FILE;
pub type PrntFlags = ::core::ffi::c_int;
pub type time_t = ::core::ffi::c_longlong;
#[cfg(target_os = "emscripten")]
pub type clock_t = ::core::ffi::c_int;
#[cfg(target_os = "macos")]
pub type clock_t = ::core::ffi::c_ulong;
#[inline]
unsafe extern "C" fn __isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c == ' ' as i32
        || (_c as ::core::ffi::c_uint).wrapping_sub('\t' as i32 as ::core::ffi::c_uint)
            < 5 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
pub const SHOW_ANAL: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const SHOW_LEMMA: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const SHOW_MISSES: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const BUFFER_ANALS: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const CHECK_PREVERB: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const KEEP_BETA: ::core::ffi::c_int = 0o40 as ::core::ffi::c_int;
pub const SHOW_FULL_INFO: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const DBASEFORMAT: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const DBASESHORT: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int | DBASEFORMAT;
pub const STRICT_CASE: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const PARSE_FORMAT: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const PERSEUS_FORMAT: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const ENDING_INDEX: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const IGNORE_ACCENTS: ::core::ffi::c_int = 0o20000 as ::core::ffi::c_int;
pub const LEXICON_OUTPUT: ::core::ffi::c_int = 0o40000 as ::core::ffi::c_int;
pub const LATIN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const VERBS_ONLY: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const ITALIAN: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
#[no_mangle]
pub static mut quickflag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut prevmemory: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
#[no_mangle]
pub static mut start_time: time_t = 0 as time_t;
#[no_mangle]
pub static mut prev_time: time_t = 0 as time_t;
#[no_mangle]
pub static mut end_time: time_t = 0 as time_t;
#[no_mangle]
pub static mut avg_time: ::core::ffi::c_double = 0 as ::core::ffi::c_int
    as ::core::ffi::c_double;
#[no_mangle]
pub static mut long_time: ::core::ffi::c_double = 0 as ::core::ffi::c_int
    as ::core::ffi::c_double;
#[no_mangle]
pub static mut string_time: ::core::ffi::c_double = 0 as ::core::ffi::c_int
    as ::core::ffi::c_double;
#[no_mangle]
pub static mut long_string: [::core::ffi::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut timeit: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ARGS: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<
        [u8; 22],
        [::core::ffi::c_char; 22],
    >(*b"ILalmnbckidsxSVpPeTo:\0")
};
pub const PATH_SEP: ::core::ffi::c_int = '/' as i32;
#[no_mangle]
pub unsafe extern "C" fn morph_main(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut finput: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut foutput: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut ffailed: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut fstats: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fstats = ::core::ptr::null_mut::<FILE>();
    ffailed = fstats;
    foutput = ffailed;
    finput = foutput;
    let mut line: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut fname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut inpname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut outname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut failedname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut statsname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut destPath: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut flags: PrntFlags = PERSEUS_FORMAT | STRICT_CASE;
    let mut rval: ::core::ffi::c_int = 0;
    let mut freemem: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut nwords: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut nhits: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_int = 0;
    let mut errflg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    outname[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    while errflg == 0
        && {
            c = getopt(argc, argv as *const *mut ::core::ffi::c_char, ARGS.as_ptr());
            c != -(1 as ::core::ffi::c_int)
        }
    {
        match c {
            97 => {
                flags |= SHOW_ANAL;
            }
            108 => {
                flags |= SHOW_LEMMA;
            }
            109 => {
                flags |= SHOW_MISSES;
            }
            98 => {
                flags |= BUFFER_ANALS;
            }
            99 => {
                flags |= CHECK_PREVERB;
            }
            73 => {
                set_lang(ITALIAN);
            }
            76 => {
                set_lang(LATIN);
            }
            107 => {
                flags |= KEEP_BETA;
            }
            105 => {
                flags |= SHOW_FULL_INFO;
            }
            100 => {
                flags &= !(0o4000 as ::core::ffi::c_int);
                flags &= !(0o40000 as ::core::ffi::c_int);
                flags |= DBASEFORMAT;
            }
            115 => {
                flags |= DBASESHORT;
            }
            110 => {
                flags |= IGNORE_ACCENTS;
            }
            120 => {
                flags |= LEXICON_OUTPUT;
            }
            86 => {
                flags |= VERBS_ONLY;
            }
            83 => {
                flags &= !(0o1000 as ::core::ffi::c_int);
            }
            112 => {
                flags |= PARSE_FORMAT;
            }
            80 => {
                flags &= !(0o4000 as ::core::ffi::c_int);
            }
            101 => {
                flags |= ENDING_INDEX;
            }
            84 => {
                timeit = 0 as ::core::ffi::c_int;
            }
            111 => {
                if strcmp(optarg, b"-\0" as *const u8 as *const ::core::ffi::c_char) == 0
                {
                    foutput = stdout;
                    ffailed = stderr;
                    fstats = ffailed;
                } else {
                    strcpy(&raw mut outname as *mut ::core::ffi::c_char, optarg);
                    sprintf(
                        &raw mut failedname as *mut ::core::ffi::c_char,
                        b"%s.failed\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut outname as *mut ::core::ffi::c_char,
                    );
                    sprintf(
                        &raw mut statsname as *mut ::core::ffi::c_char,
                        b"%s.stats\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut outname as *mut ::core::ffi::c_char,
                    );
                    printf(
                        b"outname [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut outname as *mut ::core::ffi::c_char,
                    );
                }
            }
            _ => {
                errflg += 1;
            }
        }
    }
    if optind >= argc {
        finput = stdin;
        foutput = stdout;
        ffailed = stderr;
        fstats = ffailed;
    } else {
        let fresh0 = optind;
        optind = optind + 1;
        strcpy(
            &raw mut fname as *mut ::core::ffi::c_char,
            *argv.offset(fresh0 as isize),
        );
        strcpy(
            &raw mut inpname as *mut ::core::ffi::c_char,
            &raw mut fname as *mut ::core::ffi::c_char,
        );
        strcat(
            &raw mut inpname as *mut ::core::ffi::c_char,
            b".words\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if optind >= argc {
            if outname[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == '\0' as i32
            {
                sprintf(
                    &raw mut outname as *mut ::core::ffi::c_char,
                    b"%s.morph\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut fname as *mut ::core::ffi::c_char,
                );
                sprintf(
                    &raw mut failedname as *mut ::core::ffi::c_char,
                    b"%s.failed\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut fname as *mut ::core::ffi::c_char,
                );
                sprintf(
                    &raw mut statsname as *mut ::core::ffi::c_char,
                    b"%s.stats\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut fname as *mut ::core::ffi::c_char,
                );
            }
            fprintf(
                stdout,
                b"files: [%s] [%s]\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut outname as *mut ::core::ffi::c_char,
                &raw mut failedname as *mut ::core::ffi::c_char,
            );
        } else {
            strcpy(
                &raw mut destPath as *mut ::core::ffi::c_char,
                *argv.offset(optind as isize),
            );
            sprintf(
                &raw mut outname as *mut ::core::ffi::c_char,
                b"%s%c%s.morph\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut destPath as *mut ::core::ffi::c_char,
                PATH_SEP,
                &raw mut fname as *mut ::core::ffi::c_char,
            );
        }
        fprintf(
            stderr,
            b"Input: %s\nOutput: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut inpname as *mut ::core::ffi::c_char,
            &raw mut outname as *mut ::core::ffi::c_char,
        );
        finput = fopen(
            &raw mut inpname as *mut ::core::ffi::c_char,
            b"r\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if finput.is_null() {
            fprintf(
                stderr,
                b"cannot find [%s]!\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut inpname as *mut ::core::ffi::c_char,
            );
            exit(-(1 as ::core::ffi::c_int));
        }
    }
    if foutput != stdout
        && {
            foutput = fopen(
                &raw mut outname as *mut ::core::ffi::c_char,
                b"w\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            foutput.is_null()
        }
    {
        fprintf(
            stderr,
            b"cannot find [%s]!\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut outname as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    if ffailed != stderr
        && {
            ffailed = fopen(
                &raw mut failedname as *mut ::core::ffi::c_char,
                b"w\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            ffailed.is_null()
        }
    {
        fprintf(
            stderr,
            b"cannot open [%s]!\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut failedname as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    if fstats != stderr
        && {
            fstats = fopen(
                &raw mut statsname as *mut ::core::ffi::c_char,
                b"w\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            fstats.is_null()
        }
    {
        fprintf(
            stderr,
            b"cannot open [%s]!\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut statsname as *mut ::core::ffi::c_char,
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int,
            finput,
        )
        .is_null()
    {
        trimwhite(&raw mut line as *mut ::core::ffi::c_char);
        if __isspace(line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int) != 0
            && line[0 as ::core::ffi::c_int as usize] == 0
        {
            continue;
        }
        if line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32 {
            fprintf(
                foutput,
                b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut line as *mut ::core::ffi::c_char,
            );
        } else {
            trimdigit(&raw mut line as *mut ::core::ffi::c_char);
            p = &raw mut line as *mut ::core::ffi::c_char;
            while *p as ::core::ffi::c_int != 0
                && __isspace(*p as ::core::ffi::c_int) == 0
            {
                p = p.offset(1);
            }
            if p == &raw mut line as *mut ::core::ffi::c_char {
                continue;
            }
            if *p != 0 {
                *p = 0 as ::core::ffi::c_char;
            }
            if timeit != 0 {
                prev_time = clock() as time_t;
                if start_time == 0 as time_t {
                    start_time = prev_time;
                }
            }
            rval = checkstring(
                &raw mut line as *mut ::core::ffi::c_char,
                flags,
                foutput,
            );
            if cur_lang() != LATIN && rval == 0
                && flags as ::core::ffi::c_int & IGNORE_ACCENTS != 0
            {
                let mut tmpform: [::core::ffi::c_char; 1024] = [0; 1024];
                strcpy(
                    &raw mut tmpform as *mut ::core::ffi::c_char,
                    &raw mut line as *mut ::core::ffi::c_char,
                );
                stripbreath(&raw mut tmpform as *mut ::core::ffi::c_char);
                addbreath(&raw mut tmpform as *mut ::core::ffi::c_char, ')' as i32);
                rval = checkstring(
                    &raw mut tmpform as *mut ::core::ffi::c_char,
                    flags,
                    foutput,
                );
                if rval == 0 {
                    stripbreath(&raw mut tmpform as *mut ::core::ffi::c_char);
                    addbreath(&raw mut tmpform as *mut ::core::ffi::c_char, '(' as i32);
                    rval = checkstring(
                        &raw mut tmpform as *mut ::core::ffi::c_char,
                        flags,
                        foutput,
                    );
                }
            }
            if timeit != 0 {
                end_time = clock() as time_t;
                string_time = (end_time - prev_time) as ::core::ffi::c_double
                    / CLOCKS_PER_SEC as ::core::ffi::c_double;
                if string_time >= long_time && nwords > 0 as ::core::ffi::c_long
                    && rval != 0
                {
                    long_time = string_time;
                    strcpy(
                        &raw mut long_string as *mut ::core::ffi::c_char,
                        &raw mut line as *mut ::core::ffi::c_char,
                    );
                    fprintf(
                        stderr,
                        b":longtime\t%.2f\t%s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        long_time,
                        &raw mut long_string as *mut ::core::ffi::c_char,
                    );
                }
            }
            nwords += 1;
            if rval != 0 {
                nhits += 1;
                fprintf(
                    foutput,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    anal_buf(),
                );
            } else {
                if flags as ::core::ffi::c_int & SHOW_LEMMA != 0
                    && flags as ::core::ffi::c_int & IGNORE_ACCENTS != 0
                {
                    fprintf(
                        ffailed,
                        b"form:\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut line as *mut ::core::ffi::c_char,
                    );
                }
                fprintf(
                    ffailed,
                    b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut line as *mut ::core::ffi::c_char,
                );
                fflush(ffailed);
            }
            if nwords % 1000 as ::core::ffi::c_long == 0 {
                if timeit != 0 {
                    avg_time = (end_time - start_time) as ::core::ffi::c_double
                        / (CLOCKS_PER_SEC as ::core::ffi::c_double
                            * nwords as ::core::ffi::c_double);
                    fprintf(
                        stderr,
                        b":time %.2f %.2f\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        avg_time,
                        string_time,
                    );
                }
                fprintf(
                    stderr,
                    b"%ld %ld %0.2f %s %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    nwords,
                    nhits,
                    (100 as ::core::ffi::c_int as ::core::ffi::c_float
                        * (nhits as ::core::ffi::c_float
                            / nwords as ::core::ffi::c_float)) as ::core::ffi::c_double,
                    &raw mut line as *mut ::core::ffi::c_char,
                    rval,
                );
            }
        }
    }
    if finput != stdin {
        fclose(finput);
    }
    fclose(foutput);
    fclose(ffailed);
    if nwords != 0 {
        fprintf(
            fstats,
            b"FINAL:  words %ld, analyzed %ld (%0.2f pct), %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            nwords,
            nhits,
            (100 as ::core::ffi::c_int as ::core::ffi::c_float
                * (nhits as ::core::ffi::c_float / nwords as ::core::ffi::c_float))
                as ::core::ffi::c_double,
            rval,
        );
        fprintf(
            stderr,
            b"FINAL:  words %ld, analyzed %ld (%0.2f pct), %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            nwords,
            nhits,
            (100 as ::core::ffi::c_int as ::core::ffi::c_float
                * (nhits as ::core::ffi::c_float / nwords as ::core::ffi::c_float))
                as ::core::ffi::c_double,
            rval,
        );
    }
    if nhits != 0 {
        fprintf(
            fstats,
            b":nhits %ld anals %ld anals/hit %0.2f lems %d lems/hit %0.2f\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            nhits,
            show_totanals(),
            (show_totanals() as ::core::ffi::c_float / nhits as ::core::ffi::c_float)
                as ::core::ffi::c_double,
            show_totlems(),
            (show_totlems() as ::core::ffi::c_float / nhits as ::core::ffi::c_float)
                as ::core::ffi::c_double,
        );
    }
    if timeit != 0 {
        fprintf(
            fstats,
            b":avg time %.2f; long time [%.2f] for [%s]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            avg_time,
            long_time,
            &raw mut long_string as *mut ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b":avg time %.2f; long time [%.2f] for [%s]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            avg_time,
            long_time,
            &raw mut long_string as *mut ::core::ffi::c_char,
        );
    }
    fclose(fstats);
    exit(0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn trimdigit(
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = s;
    while *s != 0 {
        s = s.offset(1);
    }
    s = s.offset(-1);
    while (if 0 as ::core::ffi::c_int != 0 {
        isdigit(*s as ::core::ffi::c_int)
    } else {
        ((*s as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint)
            < 10 as ::core::ffi::c_uint) as ::core::ffi::c_int
    }) != 0 && s > p
    {
        let fresh1 = s;
        s = s.offset(-1);
        *fresh1 = 0 as ::core::ffi::c_char;
    }
    return 0;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const CLOCKS_PER_SEC: ::core::ffi::c_long = 1000000 as ::core::ffi::c_long;

#[cfg(target_os = "emscripten")]
#[export_name = "main"]
pub unsafe extern "C" fn wasm_main(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int { morph_main(argc, argv) }
