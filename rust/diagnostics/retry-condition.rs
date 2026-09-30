use std::ffi::{c_char,c_int};
unsafe extern "C" { fn strchr(s:*const c_char,c:c_int)->*mut c_char; }
fn main(){
 for tail in [0u8,106] {
  let mut work=[tail;60];work[..8].copy_from_slice(b"objecti\0");let a=&work[6..];
  let nul_member=unsafe{!strchr(c"aeiou".as_ptr(),0).is_null()};
  let retry=a[0]==b'i'&&a[2]!=0&&unsafe{!strchr(c"aeiou".as_ptr(),a[1] as c_int).is_null()};
  println!("tail={} next={} following={} nul_member={} retry={}",tail,a[1],a[2],nul_member as u8,retry as u8);
 }
}
