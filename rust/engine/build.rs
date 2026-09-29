use std::{env, fs, path::PathBuf, process::Command};
fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "emscripten" { return; }
    // This is the pinned platform sorting runtime, not original morphology C.
    let runtime = PathBuf::from("../../vendor/runtime");
    println!("cargo:rerun-if-changed={}", runtime.display());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let source = fs::read_to_string(runtime.join("qsort.c")).unwrap()
        .replace("#include \"atomic.h\"", "#define a_ctz_l(x) __builtin_ctzl(x)")
        .replace("weak_alias(__qsort_r, qsort_r);", "");
    let wrapper = fs::read_to_string(runtime.join("qsort_nr.c")).unwrap().replace("cmpfun", "cmpfun2");
    fs::write(out.join("runtime-sort.c"), format!("{source}\n{wrapper}")).unwrap();
    assert!(Command::new("clang").args(["-O2", "-c"]).arg(out.join("runtime-sort.c")).arg("-o").arg(out.join("runtime-sort.o")).status().unwrap().success());
    assert!(Command::new("ar").arg("crs").arg(out.join("libmorphsort.a")).arg(out.join("runtime-sort.o")).status().unwrap().success());
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=morphsort");
}
