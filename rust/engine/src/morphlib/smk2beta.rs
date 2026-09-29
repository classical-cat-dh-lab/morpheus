extern "C" {
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn strcpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn islower(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tolower(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Xlit {
    pub keystring: [::core::ffi::c_char; 12],
    pub keycode: ::core::ffi::c_int,
}
static mut Beta_SMK: [Xlit; 145] = unsafe {
    [
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o220 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o222 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o224 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o221 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o223 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o225 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a/\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o213 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a\\\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o214 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a=\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o215 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o216 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o217 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a|\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o46 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a/|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o226 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a\\|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o227 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a=|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o230 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o231 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o232 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)/|\0\0\0\0\0\0\0\0"),
            keycode: 0o233 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(/|\0\0\0\0\0\0\0\0"),
            keycode: 0o234 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\\|\0\0\0\0\0\0\0\0"),
            keycode: 0o236 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(\\|\0\0\0\0\0\0\0\0"),
            keycode: 0o236 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)=|\0\0\0\0\0\0\0\0"),
            keycode: 0o237 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(=|\0\0\0\0\0\0\0\0"),
            keycode: 0o240 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*b\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'B' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*=\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o202 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*)\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o203 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*(\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o204 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*)/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o205 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*(/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o206 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*)\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o207 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*(\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o210 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*)=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o211 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*(=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o212 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*g\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'G' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*d\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'D' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o246 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o250 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o247 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o251 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e/\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o241 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e\\\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o242 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e)\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o244 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"e(\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o245 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h)/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o263 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h)\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o265 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h)=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o267 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o264 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o266 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o270 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h/\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o256 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h\\\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o257 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h=\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o260 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h)\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o261 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o262 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h|\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o372 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h/|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o271 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h\\|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o272 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h=|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o273 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h)|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o274 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o275 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h)/|\0\0\0\0\0\0\0\0"),
            keycode: 0o276 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(/|\0\0\0\0\0\0\0\0"),
            keycode: 0o277 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h)\\|\0\0\0\0\0\0\0\0"),
            keycode: 0o300 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(\\|\0\0\0\0\0\0\0\0"),
            keycode: 0o301 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h)=|\0\0\0\0\0\0\0\0"),
            keycode: 0o302 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"h(=|\0\0\0\0\0\0\0\0"),
            keycode: 0o303 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*z\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'Z' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*q\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'Y' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i)/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o340 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i)\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o342 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i)=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o344 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i(/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o341 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i(\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o343 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i(=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o345 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i+\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o363 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i/+\0\0\0\0\0\0\0\0\0"),
            keycode: 0o375 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i\\+\0\0\0\0\0\0\0\0\0"),
            keycode: 0o376 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i/\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o333 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i\\\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o334 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i=\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o335 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i)\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o336 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i(\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o337 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*k\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'K' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*l\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'L' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*m\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'M' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*n\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'N' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*c\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'C' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o)/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o366 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o)\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o370 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o367 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o371 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o/\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o361 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o\\\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o362 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o)\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o364 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"o(\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o365 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*p\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'P' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"r(\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o75 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*s\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'S' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*t\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'T' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u)/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o353 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u)\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o355 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u)=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o357 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u(/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o354 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u(\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o356 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u(=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o360 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u+\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o43 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u/+\0\0\0\0\0\0\0\0\0"),
            keycode: 0o100 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u\\+\0\0\0\0\0\0\0\0\0"),
            keycode: 0o347 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u/\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o346 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u\\\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o347 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u=\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o350 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u)\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o351 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u(\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o352 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*f\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'F' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*y\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'C' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"*x\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'X' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o312 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o314 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o316 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(/\0\0\0\0\0\0\0\0\0"),
            keycode: 0o313 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(\\\0\0\0\0\0\0\0\0\0"),
            keycode: 0o315 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o317 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w/\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o305 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w\\\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o306 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w=\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o307 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o310 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o311 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w|\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o304 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w/|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o320 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w\\|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o321 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w=|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o322 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o323 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(|\0\0\0\0\0\0\0\0\0"),
            keycode: 0o324 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)/|\0\0\0\0\0\0\0\0"),
            keycode: 0o325 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(/|\0\0\0\0\0\0\0\0"),
            keycode: 0o326 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)\\|\0\0\0\0\0\0\0\0"),
            keycode: 0o327 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(\\|\0\0\0\0\0\0\0\0"),
            keycode: 0o330 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w)=|\0\0\0\0\0\0\0\0"),
            keycode: 0o331 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w(=|\0\0\0\0\0\0\0\0"),
            keycode: 0o332 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"q\0\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'y' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"y\0\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'c' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"c\0\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'j' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"w\0\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'v' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"s\0\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'w' as i32,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"v\0\0\0\0\0\0\0\0\0\0\0"),
            keycode: 'W' as i32,
        },
    ]
};
static mut Beta_Smarta: [Xlit; 38] = unsafe {
    [
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a_\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o46 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a/_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o226 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a\\_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o227 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a=\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o230 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o231 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o232 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)/_\0\0\0\0\0\0\0\0"),
            keycode: 0o233 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(/_\0\0\0\0\0\0\0\0"),
            keycode: 0o234 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)\\_\0\0\0\0\0\0\0\0"),
            keycode: 0o236 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(\\_\0\0\0\0\0\0\0\0"),
            keycode: 0o236 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a)=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o237 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"a(=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o240 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u_\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o304 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u/_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o320 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u\\_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o321 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u=\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o322 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u)_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o323 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u(_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o324 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u)/_\0\0\0\0\0\0\0\0"),
            keycode: 0o325 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u(/_\0\0\0\0\0\0\0\0"),
            keycode: 0o326 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u)\\_\0\0\0\0\0\0\0\0"),
            keycode: 0o327 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u(\\_\0\0\0\0\0\0\0\0"),
            keycode: 0o330 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"u)=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o331 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"uw(=\0\0\0\0\0\0\0\0"),
            keycode: 0o332 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i_\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o372 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i/_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o271 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i\\_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o272 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i=\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o273 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i)_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o274 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i(_\0\0\0\0\0\0\0\0\0"),
            keycode: 0o275 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i)/_\0\0\0\0\0\0\0\0"),
            keycode: 0o276 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i(/_\0\0\0\0\0\0\0\0"),
            keycode: 0o277 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i)\\_\0\0\0\0\0\0\0\0"),
            keycode: 0o300 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i(\\_\0\0\0\0\0\0\0\0"),
            keycode: 0o301 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i)=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o302 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"i(=\0\0\0\0\0\0\0\0\0"),
            keycode: 0o303 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"r(\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o373 as ::core::ffi::c_int,
        },
        Xlit {
            keystring: ::core::mem::transmute::<
                [u8; 12],
                [::core::ffi::c_char; 12],
            >(*b"%6\0\0\0\0\0\0\0\0\0\0"),
            keycode: 0o75 as ::core::ffi::c_int,
        },
    ]
};
pub const MAXCHAR: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAXSUBSTRING: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ROMAN: ::core::ffi::c_int = 1;
pub const GREEK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ITALIC: ::core::ffi::c_int = 3;
static mut smkinited: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut Xlit_table_smk: [*mut ::core::ffi::c_char; 257] = [::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char; 257];
#[no_mangle]
pub static mut Xlit_table_smarta: [*mut ::core::ffi::c_char; 257] = [::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char; 257];
#[no_mangle]
pub static mut Xlit_table: *mut *mut ::core::ffi::c_char = ::core::ptr::null::<
    *mut ::core::ffi::c_char,
>() as *mut *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut smarta_char: [::core::ffi::c_char; 257] = [0; 257];
static mut fromsmk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut cur_font: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn smarta2beta(
    mut start: *mut ::core::ffi::c_char,
    mut result: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    Xlit_table = &raw mut Xlit_table_smarta as *mut *mut ::core::ffi::c_char;
    fromsmk = 0 as ::core::ffi::c_int;
    conv(start, result);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn smk2beta(
    mut start: *mut ::core::ffi::c_char,
    mut result: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    Xlit_table = &raw mut Xlit_table_smk as *mut *mut ::core::ffi::c_char;
    fromsmk = 1 as ::core::ffi::c_int;
    conv(start, result);
    return 0;
}
unsafe extern "C" fn conv(
    mut start: *mut ::core::ffi::c_char,
    mut result: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut s: *mut ::core::ffi::c_char = start;
    cur_font = 0 as ::core::ffi::c_int;
    if smkinited == 0 {
        smkinited += 1;
        init_smk();
    }
    *result = 0 as ::core::ffi::c_char;
    if (if 0 as ::core::ffi::c_int != 0 {
        isupper(*s as ::core::ffi::c_int)
    } else {
        ((*s as ::core::ffi::c_uint).wrapping_sub('A' as i32 as ::core::ffi::c_uint)
            < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
    }) != 0 && fromsmk != 0
    {
        strcpy(
            &raw mut tmp as *mut ::core::ffi::c_char,
            s.offset(1 as ::core::ffi::c_int as isize),
        );
        *s.offset(1 as ::core::ffi::c_int as isize) = tolower(*s as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        *s = '*' as i32 as ::core::ffi::c_char;
        strcpy(
            s.offset(2 as ::core::ffi::c_int as isize),
            &raw mut tmp as *mut ::core::ffi::c_char,
        );
    }
    while *s != 0 {
        if *s as ::core::ffi::c_int == '^' as i32 && fromsmk == 0 {
            s = s.offset(1);
            trap_upper(result, s);
            s = s.offset(1);
        } else if (if 0 as ::core::ffi::c_int != 0 {
            isupper(*s as ::core::ffi::c_int)
        } else {
            ((*s as ::core::ffi::c_uint).wrapping_sub('A' as i32 as ::core::ffi::c_uint)
                < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
        }) != 0 && fromsmk == 0
        {
            if cur_font == GREEK || cur_font == 0 {
                set_cur_font(ROMAN, result);
            }
            tmp[0 as ::core::ffi::c_int as usize] = tolower(*s as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            tmp[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            strcat(result, &raw mut tmp as *mut ::core::ffi::c_char);
            s = s.offset(1);
        } else if *s as ::core::ffi::c_int == '_' as i32 && fromsmk == 0 {
            set_cur_font(ITALIC, result);
            s = s.offset(1);
        } else if *s as ::core::ffi::c_int == -85i32 && fromsmk == 0 {
            set_cur_font(ROMAN, result);
            s = s.offset(1);
        } else if *s as ::core::ffi::c_int == '`' as i32 && fromsmk == 0 {
            if cur_font == 0 || cur_font == GREEK {
                set_cur_font(ROMAN, result);
            }
            strcat(result, b":\0" as *const u8 as *const ::core::ffi::c_char);
            s = s.offset(1);
        } else if *s as ::core::ffi::c_int & 0o377 as ::core::ffi::c_int
            >= 0o202 as ::core::ffi::c_int
            && *s as ::core::ffi::c_int & 0o377 as ::core::ffi::c_int
                <= 0o212 as ::core::ffi::c_int
        {
            set_cur_font(GREEK, result);
            let fresh0 = s;
            s = s.offset(1);
            strcat(
                result,
                *Xlit_table
                    .offset(
                        (*fresh0 as ::core::ffi::c_int & 0o377 as ::core::ffi::c_int)
                            as isize,
                    ),
            );
            if if 0 as ::core::ffi::c_int != 0 {
                isupper(*s as ::core::ffi::c_int)
            } else {
                ((*s as ::core::ffi::c_uint)
                    .wrapping_sub('A' as i32 as ::core::ffi::c_uint)
                    < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
            } != 0
            {
                tmp[0 as ::core::ffi::c_int as usize] = tolower(*s as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            } else {
                tmp[0 as ::core::ffi::c_int as usize] = *s;
            }
            tmp[0 as ::core::ffi::c_int as usize] = smk2betachar(
                tmp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            ) as ::core::ffi::c_char;
            tmp[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            strcat(result, &raw mut tmp as *mut ::core::ffi::c_char);
            s = s.offset(1);
        } else {
            if fromsmk == 0 {
                if (smarta_char[(*s as ::core::ffi::c_int & 0o377 as ::core::ffi::c_int)
                    as usize] as ::core::ffi::c_int != 0
                    || (if 0 as ::core::ffi::c_int != 0 {
                        islower(*s as ::core::ffi::c_int)
                    } else {
                        ((*s as ::core::ffi::c_uint)
                            .wrapping_sub('a' as i32 as ::core::ffi::c_uint)
                            < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
                    }) != 0)
                    && (cur_font == ROMAN || cur_font == ITALIC || cur_font == 0)
                {
                    set_cur_font(GREEK, result);
                }
            }
            let fresh1 = s;
            s = s.offset(1);
            strcat(
                result,
                *Xlit_table
                    .offset(
                        (*fresh1 as ::core::ffi::c_int & 0o377 as ::core::ffi::c_int)
                            as isize,
                    ),
            );
        }
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn smk2betachar(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if c == 'v' as i32 {
        return 'w' as i32;
    }
    if c == 'y' as i32 {
        return 'q' as i32;
    }
    if c == 'c' as i32 {
        return 'y' as i32;
    }
    if c == 'j' as i32 {
        return 'c' as i32;
    }
    if c == 'W' as i32 {
        return 'v' as i32;
    }
    crate::unavailable("smk2betachar: undefined original return");
}
#[no_mangle]
pub unsafe extern "C" fn init_smk() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut tmp: [::core::ffi::c_char; 80] = [0; 80];
    i = 0 as ::core::ffi::c_int;
    while i < MAXCHAR {
        Xlit_table_smk[i as usize] = malloc(MAXSUBSTRING as size_t)
            as *mut ::core::ffi::c_char;
        *Xlit_table_smk[i as usize] = 0 as ::core::ffi::c_char;
        Xlit_table_smarta[i as usize] = malloc(MAXSUBSTRING as size_t)
            as *mut ::core::ffi::c_char;
        *Xlit_table_smarta[i as usize] = 0 as ::core::ffi::c_char;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[Xlit; 145]>() as usize)
            .wrapping_div(::core::mem::size_of::<Xlit>() as usize)
    {
        strncpy(
            Xlit_table_smk[Beta_SMK[i as usize].keycode as usize],
            &raw mut (*(&raw mut Beta_SMK as *mut Xlit).offset(i as isize)).keystring
                as *mut ::core::ffi::c_char,
            MAXSUBSTRING as size_t,
        );
        strncpy(
            Xlit_table_smarta[Beta_SMK[i as usize].keycode as usize],
            &raw mut (*(&raw mut Beta_SMK as *mut Xlit).offset(i as isize)).keystring
                as *mut ::core::ffi::c_char,
            MAXSUBSTRING as size_t,
        );
        smarta_char[Beta_SMK[i as usize].keycode as usize] = 1 as ::core::ffi::c_char;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[Xlit; 38]>() as usize)
            .wrapping_div(::core::mem::size_of::<Xlit>() as usize)
    {
        strncpy(
            Xlit_table_smarta[Beta_Smarta[i as usize].keycode as usize],
            &raw mut (*(&raw mut Beta_Smarta as *mut Xlit).offset(i as isize)).keystring
                as *mut ::core::ffi::c_char,
            MAXSUBSTRING as size_t,
        );
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        if *Xlit_table_smk[i as usize] == 0 {
            sprintf(
                Xlit_table_smk[i as usize],
                b"%c\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            );
        }
        if *Xlit_table_smarta[i as usize] == 0 {
            sprintf(
                Xlit_table_smarta[i as usize],
                b"%c\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            );
        }
        i += 1;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_cur_font(
    mut n: ::core::ffi::c_int,
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if fromsmk != 0 {
        return 0;
    }
    if n != cur_font {
        match n {
            GREEK => {
                strcat(s, b"$\0" as *const u8 as *const ::core::ffi::c_char);
            }
            ROMAN => {
                strcat(s, b"&\0" as *const u8 as *const ::core::ffi::c_char);
            }
            ITALIC => {
                strcat(s, b"&3\0" as *const u8 as *const ::core::ffi::c_char);
            }
            _ => {
                strcat(s, b"?Font?\0" as *const u8 as *const ::core::ffi::c_char);
            }
        }
        cur_font = n;
    }
    return 0;
}
pub const SPACE_ACUTE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const ALPHA_ACUTE: ::core::ffi::c_int = 0o213 as ::core::ffi::c_int;
pub const EPSILON_ACUTE: ::core::ffi::c_int = 0o241 as ::core::ffi::c_int;
pub const IOTA_ACUTE: ::core::ffi::c_int = 0o333 as ::core::ffi::c_int;
pub const OMICRON_ACUTE: ::core::ffi::c_int = 0o361 as ::core::ffi::c_int;
pub const UPSILON_ACUTE: ::core::ffi::c_int = 0o346 as ::core::ffi::c_int;
pub const ETA_ACUTE: ::core::ffi::c_int = 0o256 as ::core::ffi::c_int;
pub const WMEGA_ACUTE: ::core::ffi::c_int = 0o305 as ::core::ffi::c_int;
pub const AISUB_ACUTE: ::core::ffi::c_int = 0o226 as ::core::ffi::c_int;
pub const EISUB_ACUTE: ::core::ffi::c_int = 0o372 as ::core::ffi::c_int;
pub const WISUB_ACUTE: ::core::ffi::c_int = 0o304 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn trap_upper(
    mut res: *mut ::core::ffi::c_char,
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    if if 0 as ::core::ffi::c_int != 0 {
        isupper(*s as ::core::ffi::c_int)
    } else {
        ((*s as ::core::ffi::c_uint).wrapping_sub('A' as i32 as ::core::ffi::c_uint)
            < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
    } != 0
    {
        if cur_font == 0 || cur_font == GREEK {
            set_cur_font(ROMAN, res);
        }
        tmp[0 as ::core::ffi::c_int as usize] = '*' as i32 as ::core::ffi::c_char;
        tmp[1 as ::core::ffi::c_int as usize] = tolower(*s as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        tmp[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        strcat(res, &raw mut tmp as *mut ::core::ffi::c_char);
        return 0;
    }
    if if 0 as ::core::ffi::c_int != 0 {
        islower(*s as ::core::ffi::c_int)
    } else {
        ((*s as ::core::ffi::c_uint).wrapping_sub('a' as i32 as ::core::ffi::c_uint)
            < 26 as ::core::ffi::c_uint) as ::core::ffi::c_int
    } != 0
    {
        if cur_font == 0 || cur_font == ROMAN || cur_font == ITALIC {
            set_cur_font(GREEK, res);
        }
        tmp[0 as ::core::ffi::c_int as usize] = '*' as i32 as ::core::ffi::c_char;
        tmp[1 as ::core::ffi::c_int as usize] = *s;
        tmp[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        strcat(res, &raw mut tmp as *mut ::core::ffi::c_char);
        return 0;
    }
    tmp[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    if *s as ::core::ffi::c_int == 'a' as i32 || *s as ::core::ffi::c_int == 'A' as i32
        || *s as ::core::ffi::c_int >= 0o213 as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= 0o225 as ::core::ffi::c_int
    {
        add_acc(
            &raw mut tmp as *mut ::core::ffi::c_char,
            *s as ::core::ffi::c_int - ALPHA_ACUTE + SPACE_ACUTE,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if *s as ::core::ffi::c_int == 'e' as i32
        || *s as ::core::ffi::c_int == 'E' as i32
        || *s as ::core::ffi::c_int >= 0o241 as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= 0o251 as ::core::ffi::c_int
    {
        add_acc(
            &raw mut tmp as *mut ::core::ffi::c_char,
            *s as ::core::ffi::c_int - EPSILON_ACUTE + SPACE_ACUTE,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"e\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if *s as ::core::ffi::c_int == 'i' as i32
        || *s as ::core::ffi::c_int == 'I' as i32
        || *s as ::core::ffi::c_int >= 0o333 as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= 0o345 as ::core::ffi::c_int
    {
        add_acc(
            &raw mut tmp as *mut ::core::ffi::c_char,
            *s as ::core::ffi::c_int - IOTA_ACUTE + SPACE_ACUTE,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"i\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if *s as ::core::ffi::c_int == 'o' as i32
        || *s as ::core::ffi::c_int == 'O' as i32
        || *s as ::core::ffi::c_int >= 0o361 as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= 0o371 as ::core::ffi::c_int
    {
        add_acc(
            &raw mut tmp as *mut ::core::ffi::c_char,
            *s as ::core::ffi::c_int - OMICRON_ACUTE + SPACE_ACUTE,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"o\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if *s as ::core::ffi::c_int == 'u' as i32
        || *s as ::core::ffi::c_int == 'U' as i32
        || *s as ::core::ffi::c_int >= 0o346 as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= 0o360 as ::core::ffi::c_int
    {
        add_acc(
            &raw mut tmp as *mut ::core::ffi::c_char,
            *s as ::core::ffi::c_int - UPSILON_ACUTE + SPACE_ACUTE,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"u\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if *s as ::core::ffi::c_int == 'h' as i32
        || *s as ::core::ffi::c_int == 'H' as i32
        || *s as ::core::ffi::c_int >= 0o256 as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= 0o270 as ::core::ffi::c_int
    {
        add_acc(
            &raw mut tmp as *mut ::core::ffi::c_char,
            *s as ::core::ffi::c_int - ETA_ACUTE + SPACE_ACUTE,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"h\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if *s as ::core::ffi::c_int == 'v' as i32
        || *s as ::core::ffi::c_int == 'V' as i32
        || *s as ::core::ffi::c_int >= 0o305 as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= 0o317 as ::core::ffi::c_int
    {
        add_acc(
            &raw mut tmp as *mut ::core::ffi::c_char,
            *s as ::core::ffi::c_int - WMEGA_ACUTE + SPACE_ACUTE,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"w\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if *s as ::core::ffi::c_int == 0o46 as ::core::ffi::c_int
        || *s as ::core::ffi::c_int >= 0o226 as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= 0o240 as ::core::ffi::c_int
    {
        add_acc(
            &raw mut tmp as *mut ::core::ffi::c_char,
            *s as ::core::ffi::c_int - AISUB_ACUTE + SPACE_ACUTE,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"_\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if *s as ::core::ffi::c_int == 0o372 as ::core::ffi::c_int
        || *s as ::core::ffi::c_int >= 0o271 as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= 0o303 as ::core::ffi::c_int
    {
        add_acc(
            &raw mut tmp as *mut ::core::ffi::c_char,
            *s as ::core::ffi::c_int - EISUB_ACUTE + SPACE_ACUTE,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"_\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"h\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if *s as ::core::ffi::c_int == 0o304 as ::core::ffi::c_int
        || *s as ::core::ffi::c_int >= 0o320 as ::core::ffi::c_int
            && *s as ::core::ffi::c_int <= 0o332 as ::core::ffi::c_int
    {
        add_acc(
            &raw mut tmp as *mut ::core::ffi::c_char,
            *s as ::core::ffi::c_int - WISUB_ACUTE + SPACE_ACUTE,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"_\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcat(
            &raw mut tmp as *mut ::core::ffi::c_char,
            b"w\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if tmp[0 as ::core::ffi::c_int as usize] != 0 {
        if cur_font == 0 || cur_font == ROMAN || cur_font == ITALIC {
            set_cur_font(GREEK, res);
        }
        strcat(res, &raw mut tmp as *mut ::core::ffi::c_char);
    }
    return 0;
}
unsafe extern "C" fn add_acc(
    mut s: *mut ::core::ffi::c_char,
    mut anum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    strcpy(s, *Xlit_table.offset((anum & 0o377 as ::core::ffi::c_int) as isize));
    return 0;
}
