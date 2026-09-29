extern "C" {
    fn printf(_: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
pub type bool_0 = ::core::ffi::c_int;
#[no_mangle]
pub static mut BinLookPrnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn binlook(
    mut table: *mut ::core::ffi::c_char,
    mut tag: *mut ::core::ffi::c_char,
    mut nelems: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
    mut exact_match: bool_0,
    mut compare: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    let mut high: ::core::ffi::c_int = 0;
    let mut low: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    let mut comp: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_long = 0;
    high = nelems - 1 as ::core::ffi::c_int;
    low = 0 as ::core::ffi::c_int;
    while low <= high {
        mid = (low + high) / 2 as ::core::ffi::c_int;
        offset = mid as ::core::ffi::c_long * size as ::core::ffi::c_long;
        comp = ::core::mem::transmute::<
            _,
            unsafe extern "C" fn(_, _) -> ::core::ffi::c_int,
        >(
            Some(compare.expect("non-null function pointer"))
                .expect("non-null function pointer"),
        )(tag, table.offset(offset as isize));
        if BinLookPrnt != 0 {
            printf(
                b"off %ld comparing [%s] and [%s]\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                offset,
                tag,
                table.offset(offset as isize),
            );
        }
        if comp < 0 as ::core::ffi::c_int {
            high = mid - 1 as ::core::ffi::c_int;
        } else if comp > 0 as ::core::ffi::c_int {
            low = mid + 1 as ::core::ffi::c_int;
        } else {
            if BinLookPrnt != 0 {
                printf(
                    b"A returning with tag [%s] tagstring [%s]\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    tag,
                    table.offset(offset as isize),
                );
            }
            return mid;
        }
    }
    if BinLookPrnt != 0 {
        printf(
            b"B mid %d nelems %d with em %d tag [%s] tagstring [%s]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            mid,
            nelems,
            exact_match,
            tag,
            table.offset((size * mid) as isize),
        );
    }
    if exact_match != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if mid > 0 as ::core::ffi::c_int {
        i = mid - 1 as ::core::ffi::c_int;
        while i < nelems {
            offset = size as ::core::ffi::c_long * i as ::core::ffi::c_long;
            if ::core::mem::transmute::<
                _,
                unsafe extern "C" fn(_, _) -> ::core::ffi::c_int,
            >(
                Some(compare.expect("non-null function pointer"))
                    .expect("non-null function pointer"),
            )(tag, table.offset(offset as isize)) < 0 as ::core::ffi::c_int
            {
                break;
            }
            i += 1;
        }
        if i > 0 as ::core::ffi::c_int {
            i -= 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
    }
    if BinLookPrnt != 0 {
        printf(
            b"C returning with i %d tag [%s] tagstring [%s]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            i,
            tag,
            table.offset(offset as isize),
        );
    }
    return i;
}
