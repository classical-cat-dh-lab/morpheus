#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables, unused_must_use)]

#[macro_use]
extern crate c2rust_bitfields;

pub mod src {
pub mod anal {
pub mod stdiomorph;
pub mod checkcrasis;
pub mod checkdict;
pub mod checkgenwds;
pub mod checkhalf1;
pub mod checkindecl;
pub mod checkirreg;
pub mod checknom;
pub mod checkpreverb;
pub mod checkstem;
pub mod checkstring;
pub mod checkverb;
pub mod checkword;
pub mod dictstems;
pub mod prntanal;
pub mod prvb;
} // mod anal
pub mod gener {
pub mod genwd;
} // mod gener
pub mod gkdict {
pub mod compnoun;
pub mod derivio;
pub mod dictio;
} // mod gkdict
pub mod gkends {
pub mod acccompos;
pub mod checkforbreath;
pub mod contract;
pub mod countendtables;
pub mod endindex;
pub mod euphend;
pub mod expendtable;
pub mod fixeta;
pub mod getcurrend;
pub mod indexendtables;
pub mod lcontr;
pub mod merge;
pub mod mkend;
pub mod nextsufftab;
pub mod retrends;
pub mod stor;
} // mod gkends
pub mod greeklib {
pub mod Fclose;
pub mod addaccent;
pub mod addbreath;
pub mod aspirate;
pub mod beta_tolower;
pub mod binlook;
pub mod checkaccent;
pub mod cinsert;
pub mod do_dissim;
pub mod endsinstr;
pub mod getaccent;
pub mod getaccp;
pub mod getbreath;
pub mod getquantity;
pub mod getsyll;
pub mod gkstrlen;
pub mod hasaccent;
pub mod hasdiaer;
pub mod hasquant;
pub mod isblank;
pub mod isdiphth;
pub mod issubstring;
pub mod keyio;
pub mod longbyposition;
pub mod naccents;
pub mod normucase;
pub mod nsylls;
pub mod quantprim;
pub mod shortanalog;
pub mod sprntGkflags;
pub mod standalpha;
pub mod standword;
pub mod stripacc;
pub mod stripacute;
pub mod stripbreath;
pub mod stripchar;
pub mod stripdiaer;
pub mod stripmeta;
pub mod stripquant;
pub mod stripstemsep;
pub mod stripzeroend;
pub mod strsqz;
pub mod subchar;
pub mod vaxwords;
pub mod xstrings;
pub mod zap2ndbreath;
} // mod greeklib
pub mod morphlib {
pub mod adddomain;
pub mod addninfix;
pub mod antepenform;
pub mod augment;
pub mod beta2rtf;
pub mod beta2smarta;
pub mod cmpend;
pub mod conjstem;
pub mod endio;
pub mod errormess;
pub mod fixacc;
pub mod gkstring;
pub mod gktoasc;
pub mod indkeys;
pub mod is_thirdmono;
pub mod loadeuph;
pub mod markstem;
pub mod morphflags;
pub mod morphkeys;
pub mod morphpath;
pub mod morphstrcmp;
pub mod new_val;
pub mod nextkey;
pub mod numovable;
pub mod penultform;
pub mod pres_redup;
pub mod preverb;
pub mod preverb2;
pub mod preverb3;
pub mod retrentry;
pub mod setlang;
pub mod smk2beta;
pub mod sprntGkflags;
pub mod standphon;
pub mod trimwhite;
pub mod ultform;
pub mod ulttakescirc;
} // mod morphlib
} // mod src

pub fn unavailable(reason: &str) -> ! { panic!("Unqualified original helper: {reason}") }
#[no_mangle]
pub unsafe extern "C" fn morph_port_unavailable(_reason: *const core::ffi::c_char) -> core::ffi::c_int { unavailable("original missing argument") }

#[cfg(feature = "trace")]
pub mod trace;

pub mod legacy_stack;

#[cfg(test)]
mod abi_tests {
    use super::src::anal::checkword::{word_form,gk_string,gk_word,gk_analysis};
    #[test]
    fn preserve_frozen_c_record_layout() {
        assert_eq!(core::mem::size_of::<word_form>(),4);
        assert_eq!(core::mem::align_of::<word_form>(),4);
        assert_eq!(core::mem::size_of::<gk_string>(),116);
        assert_eq!(core::mem::size_of::<gk_analysis>(),1116);
        assert_eq!(core::mem::offset_of!(gk_word,st_workword),824);
        #[cfg(target_pointer_width="64")]
        { assert_eq!(core::mem::size_of::<gk_word>(),960); assert_eq!(core::mem::offset_of!(gk_word,gw_analysis),952); }
        #[cfg(target_pointer_width="32")]
        { assert_eq!(core::mem::size_of::<gk_word>(),952); assert_eq!(core::mem::offset_of!(gk_word,gw_analysis),948); }
    }
}
