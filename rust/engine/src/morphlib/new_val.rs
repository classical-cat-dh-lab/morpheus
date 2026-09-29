use ::c2rust_bitfields;
extern "C" {
    fn add_domain(_: *mut gk_string, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn add_morphflag(_: *mut MorphFlags, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type int32 = ::core::ffi::c_uint;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C, align(4))]
pub struct word_form {
    #[bitfield(name = "f_voice", ty = "::core::ffi::c_uint", bits = "0..=2")]
    #[bitfield(name = "f_mood", ty = "::core::ffi::c_uint", bits = "3..=6")]
    #[bitfield(name = "f_tense", ty = "::core::ffi::c_uint", bits = "7..=10")]
    #[bitfield(name = "f_person", ty = "::core::ffi::c_uint", bits = "11..=13")]
    #[bitfield(name = "f_number", ty = "::core::ffi::c_uint", bits = "14..=16")]
    #[bitfield(name = "f_case", ty = "::core::ffi::c_uint", bits = "17..=22")]
    #[bitfield(name = "f_degree", ty = "::core::ffi::c_uint", bits = "23..=24")]
    #[bitfield(name = "f_gender", ty = "::core::ffi::c_uint", bits = "25..=28")]
    pub f_voice_f_mood_f_tense_f_person_f_number_f_case_f_degree_f_gender: [u8; 4],
}
pub type Dialect = ::core::ffi::c_short;
pub type GeogRegion = int32;
pub type Stemtype = ::core::ffi::c_uint;
pub type Derivtype = ::core::ffi::c_uint;
pub type MorphFlags = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gk_string {
    pub gs_forminfo: word_form,
    pub gs_steminfo: Stemtype,
    pub gs_derivtype: Derivtype,
    pub gs_dialect: Dialect,
    pub gs_geogregion: GeogRegion,
    pub gs_morphflags: [MorphFlags; 12],
    pub st_domains: [::core::ffi::c_char; 21],
    pub gs_gkstring: [::core::ffi::c_char; 60],
}
pub const ACTIVE: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const MIDDLE: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const PASSIVE: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const MEDIO_PASS: ::core::ffi::c_int = MIDDLE | PASSIVE;
#[no_mangle]
pub unsafe extern "C" fn new_person(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    (*gstr).gs_forminfo.set_f_person(val as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn new_number(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    (*gstr).gs_forminfo.set_f_number(val as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn new_case(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    (*gstr)
        .gs_forminfo
        .set_f_case((*gstr).gs_forminfo.f_case() | val as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn new_tense(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    (*gstr).gs_forminfo.set_f_tense(val as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn new_voice(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    if val == ACTIVE as ::core::ffi::c_ulong
        && (*gstr).gs_forminfo.f_voice() as ::core::ffi::c_int & (MIDDLE | PASSIVE) != 0
    {
        (*gstr)
            .gs_forminfo
            .set_f_voice(val as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else if val & (MIDDLE | PASSIVE) as ::core::ffi::c_ulong != 0
        && (*gstr).gs_forminfo.f_voice() as ::core::ffi::c_int == ACTIVE
    {
        (*gstr)
            .gs_forminfo
            .set_f_voice(val as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        (*gstr)
            .gs_forminfo
            .set_f_voice((*gstr).gs_forminfo.f_voice() | val as ::core::ffi::c_uint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn new_mood(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    (*gstr).gs_forminfo.set_f_mood(val as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn new_degree(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    (*gstr).gs_forminfo.set_f_degree(val as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn new_gender(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    (*gstr)
        .gs_forminfo
        .set_f_gender((*gstr).gs_forminfo.f_gender() | val as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn new_dialect(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    (*gstr).gs_dialect = ((*gstr).gs_dialect as ::core::ffi::c_int
        | val as Dialect as ::core::ffi::c_int) as Dialect;
}
#[no_mangle]
pub unsafe extern "C" fn new_region(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    (*gstr).gs_geogregion |= val as GeogRegion;
}
#[no_mangle]
pub unsafe extern "C" fn new_morphflags(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    add_morphflag(
        &raw mut (*gstr).gs_morphflags as *mut MorphFlags,
        val as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn new_stemtype(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    (*gstr).gs_steminfo = val as Stemtype;
}
#[no_mangle]
pub unsafe extern "C" fn new_domain(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    add_domain(gstr, val as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn new_derivtype(
    mut gstr: *mut gk_string,
    mut val: ::core::ffi::c_ulong,
) {
    (*gstr).gs_derivtype = val as Derivtype;
}
