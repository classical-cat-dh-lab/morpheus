fn main() {
    let args: Vec<std::ffi::CString> = std::env::args().map(|a| std::ffi::CString::new(a).unwrap()).collect();
    let mut pointers: Vec<*mut core::ffi::c_char> = args.iter().map(|a| a.as_ptr() as *mut _).collect();
    pointers.push(core::ptr::null_mut());
    unsafe { std::process::exit(morpheus_preservation::src::anal::stdiomorph::morph_main(args.len() as i32, pointers.as_mut_ptr())); }
}
