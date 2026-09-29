//! Logical source checkpoints; compiled only with the trace feature.
use crate::src::anal::checkword::{gk_word,gk_string,word_form};
use std::{fmt::Write as _,io::Write as _};
fn h(s:&mut String,b:&[core::ffi::c_char]){s.push('|');for &c in b.iter().take_while(|&&c|c!=0){write!(s,"{:02x}",c as u8).unwrap();}}
fn form(s:&mut String,w:word_form){write!(s,"|{}",w.f_voice()).unwrap();write!(s,"|{}",w.f_mood()).unwrap();write!(s,"|{}",w.f_tense()).unwrap();write!(s,"|{}",w.f_person()).unwrap();write!(s,"|{}",w.f_number()).unwrap();write!(s,"|{}",w.f_case()).unwrap();write!(s,"|{}",w.f_degree()).unwrap();write!(s,"|{}",w.f_gender()).unwrap();}
macro_rules! meta {($s:expr,$v:expr)=>{{let v=$v;form($s,v.gs_forminfo);write!($s,"|{}",v.gs_steminfo).unwrap();write!($s,"|{}",v.gs_derivtype).unwrap();write!($s,"|{}",v.gs_dialect).unwrap();write!($s,"|{}",v.gs_geogregion).unwrap();for &b in &v.gs_morphflags{write!($s,"|{}",b as u8).unwrap();}h($s,&v.st_domains);}};}
fn gs(s:&mut String,v:&gk_string){meta!(s,v);h(s,&v.gs_gkstring);}
pub unsafe fn word(tag:&str,p:*const core::ffi::c_void){let Some(path)=std::env::var_os("MORPH_TRACE_FILE") else{return};let mut s=tag.to_owned();if p.is_null(){s.push_str("|null\n");}else{let v=&*(p as *const gk_word);meta!(&mut s,v);write!(s,"|{}|{}",v.gs_prntflags,v.gw_totanal).unwrap();
h(&mut s,&v.st_lemma);
h(&mut s,&v.st_rawprvb);
h(&mut s,&v.st_rawword);
h(&mut s,&v.st_workword);
h(&mut s,&v.st_crasis);
gs(&mut s,&v.gs_preverb);
gs(&mut s,&v.gs_aug1);
gs(&mut s,&v.gs_stem);
gs(&mut s,&v.gs_suffix);
gs(&mut s,&v.gs_endstring);
for j in 0..v.gw_totanal {let a=&*v.gw_analysis.offset(j as isize);s.push_str("|analysis");meta!(&mut s,a);
h(&mut s,&a.st_lemma);
h(&mut s,&a.st_rawprvb);
h(&mut s,&a.st_rawword);
h(&mut s,&a.st_workword);
h(&mut s,&a.st_crasis);
h(&mut s,&a.st_dictform);
h(&mut s,&a.st_engform);
gs(&mut s,&a.gs_preverb);
gs(&mut s,&a.gs_aug1);
gs(&mut s,&a.gs_stem);
gs(&mut s,&a.gs_suffix);
gs(&mut s,&a.gs_endstring);
}s.push('\n');}std::fs::OpenOptions::new().create(true).append(true).open(path).unwrap().write_all(s.as_bytes()).unwrap();}
