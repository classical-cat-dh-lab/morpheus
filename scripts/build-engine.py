#!/usr/bin/env python3
"""Build Morpheus 0.0.1 from the locked original and explicit portability patches.

This engineering reconstruction does not claim current Tufts-server equivalence.
Use a fresh output directory. Requires Apple Clang, make, flex, Perl, Emscripten.
"""

import argparse
import difflib
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile

PROJECT = Path(__file__).resolve().parents[1]
NATIVE_FLAGS = "-O2 -std=gnu89 -I../includes -Wno-return-type -Wno-implicit-function-declaration -Wno-error=incompatible-function-pointer-types"
WASM_FLAGS = "-O2 -I../includes -std=gnu89 -Wno-return-type -Wno-implicit-function-declaration -Wno-int-conversion -Wno-incompatible-function-pointer-types -fno-common"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--emsdk", type=Path, required=True)
    args = parser.parse_args()
    output, sdk = args.output.resolve(), args.emsdk.resolve()
    if output.exists():
        parser.error("Output must not exist; retain previous evidence and use a fresh directory.")
    emcc = sdk / "upstream/emscripten/emcc"
    if not emcc.is_file():
        parser.error("The specified SDK does not contain emcc.")
    source = json.loads((PROJECT / "source.lock.json").read_text())
    assert source["repository"] == "PerseusDL/morpheus"
    lock = source["localSnapshot"]
    archive = PROJECT / lock["archive"]
    if digest(archive) != lock["archiveSha256"]:
        raise RuntimeError("Locked source archive does not match")
    output.mkdir(parents=True)
    logs = output / "logs"
    logs.mkdir()
    environment = dict(os.environ, LC_ALL="C", TZ="UTC", EMSDK_PYTHON=sys.executable)
    commands = []

    def run(command, cwd, name, env=None, stdin=None, stdout=None):
        command = [str(value) for value in command]
        commands.append({"argv": command, "cwd": str(cwd), "log": name})
        with (logs / name).open("ab") as log:
            completed = subprocess.run(command, cwd=cwd, env=env or environment,
                                       input=stdin, stdout=stdout or log, stderr=log,
                                       timeout=600)
        if completed.returncode:
            raise RuntimeError(f"Build failed ({completed.returncode}); inspect {logs / name}")

    patches = []

    def patch(path, new, name):
        old = path.read_text()
        if old == new:
            raise RuntimeError(f"Expected an actual adaptation: {path}")
        path.write_text(new)
        diff = "".join(difflib.unified_diff(old.splitlines(True), new.splitlines(True),
                                           fromfile="a/" + str(path.relative_to(path.parents[2])),
                                           tofile="b/" + str(path.relative_to(path.parents[2]))))
        (output / name).write_text(diff)
        patches.append({"patch": name, "sha256": digest(output / name)})

    print("Extracting the locked original source", flush=True)
    with tarfile.open(archive) as bundle:
        bundle.extractall(output / "extract", filter="data")
    original = output / "extract" / lock["archivePrefix"].rstrip("/")
    native, wasm = output / "native", output / "wasm"
    shutil.copytree(original, native)
    shutil.copytree(original, wasm)
    # A defined overlapping copy is a portability adaptation; linguistic logic
    # and data remain original. Apply the same change to both execution targets.
    for tree in (native, wasm):
        file = tree / "src/gkdict/derivio.c"
        content = file.read_text()
        assert content.count("strcpy(derivsuff,s);") == 1
        patch(file, content.replace("strcpy(derivsuff,s);", "memmove(derivsuff,s,strlen(s)+1);"),
              tree.name + "-derivation-overlap.patch")
        file = tree / "src/anal/prvb.c"
        content = file.read_text()
        old = "strcpy(stem_of(&WorkGkword),stem_of(&WorkGkword)+1);"
        assert content.count(old) == 1
        patch(file, content.replace(old, "memmove(stem_of(&WorkGkword),stem_of(&WorkGkword)+1,strlen(stem_of(&WorkGkword)+1)+1);"),
              tree.name + "-preverb-overlap.patch")
    (native / "bin").mkdir(exist_ok=True)
    print("Building original native tools", flush=True)
    run(["make", "all", "CC=clang", "CFLAGS=" + NATIVE_FLAGS, "LOADLIBES=-ll"],
        native / "src", "native.log")

    # Only standalone indexers receive isolated temporary-file paths. The original
    # analyser has already been built and is not relinked with either adaptation.
    header = native / "src/includes/gkdict.h"
    header_original = header.read_text()
    isolated = header_original
    for filename in ("nommorph", "vbmorph"):
        literal = '"' + str(output / filename).replace("\\", "\\\\").replace('"', '\\"') + '"'
        assert '"/tmp/' + filename + '"' in isolated
        isolated = isolated.replace('"/tmp/' + filename + '"', literal)
    patch(header, isolated, "indexer-paths.patch")
    try:
        cwd = native / "src/gkdict"
        run(["clang", *NATIVE_FLAGS.split(), "-c", "indexstems.c", "-o", "indexstems-isolated.o"], cwd, "indexers.log")
        for name in ("indexnoms", "indexvbs"):
            run(["clang", "-o", name + "-isolated", name + ".main.o", "indexstems-isolated.o",
                 "../morphlib/morphlib.a", "../greeklib/greeklib.a"], cwd, "indexers.log")
    finally:
        header.write_text(header_original)

    print("Rebuilding Greek morphology data from original checked-in sources", flush=True)
    greek = native / "stemlib/Greek"
    data_env = dict(environment, MORPHLIB="..")
    nouns = sorted(set(greek.glob("stemsrc/nom.*")) | set(greek.glob("stemsrc/nom[0-9]*")))
    nouns = [p.relative_to(greek) for p in nouns if not p.name.endswith("~")]
    with (output / "nommorph").open("wb") as target:
        run(["/usr/bin/perl", "addconstraints.pl", *nouns, "stemsrc/lsj.nom", "stemsrc/lsj.byhand"],
            greek, "data.log", env=data_env, stdout=target)
    with (greek / "steminds/entitylist.txt").open("wb") as target:
        run(["/usr/bin/perl", "getentities.pl", output / "nommorph"], greek, "data.log", env=data_env, stdout=target)
    run([native / "src/gkdict/indexnoms-isolated"], greek, "data.log", env=data_env)
    # The original all-target prerequisites omit this required dependency order.
    for executable, arguments in (("buildend", ["nom"]), ("indendtables", ["nom"]),
                                  ("buildend", ["verb"]), ("indendtables", ["verb"]),
                                  ("buildderiv", ["all"]), ("indderivtables", [])):
        run([native / "src/gkends" / executable, *arguments], greek, "data.log", env=data_env)
    with (greek / "conjfile").open("wb") as target:
        for filename in ("vbs.irreg", "vbs.simp.ml", "vbs.simp.02.new", "lsj.vbs"):
            target.write((greek / "stemsrc" / filename).read_bytes())
    conjugations = native / "src/gener/conjsys.c"
    before = conjugations.read_text()
    assert before.count("strcpy(stembuf,stembuf+4);") == 1
    patch(conjugations, before.replace("strcpy(stembuf,stembuf+4);",
                                      "memmove(stembuf,stembuf+4,strlen(stembuf+4)+1);"),
          "native-data-overlap.patch")
    run(["make", "do_conj", "CC=clang", "CFLAGS=" + NATIVE_FLAGS], native / "src/gener", "conjugations.log")
    run([native / "src/gener/do_conj"], greek, "conjugations.log", env=data_env)
    shutil.copyfile(greek / "conjfile.short", output / "vbmorph")
    run([native / "src/gkdict/indexvbs-isolated"], greek, "data.log", env=data_env)

    print("Rebuilding Latin from the retained original inputs", flush=True)
    latin = native / "stemlib/Latin"
    for directory in ("steminds", "derivs/ascii", "derivs/indices", "derivs/out"):
        (latin / directory).mkdir(parents=True, exist_ok=True)
    with (output / "nommorph").open("wb") as target:
        for file in [*sorted(latin.glob("stemsrc/nom.*")), latin / "stemsrc/ls.nom"]:
            target.write(file.read_bytes())
    with (latin / "steminds/entitylist.txt").open("wb") as target:
        run(["/usr/bin/perl", "../Greek/getentities.pl", output / "nommorph"], latin, "latin.log", env=data_env, stdout=target)
    run([native / "src/gkdict/indexnoms-isolated", "-L"], latin, "latin.log", env=data_env)
    for executable, arguments in (("buildend", ["nom"]), ("indendtables", ["nom"]),
                                  ("buildend", ["verb"]), ("indendtables", ["verb"]),
                                  ("buildderiv", ["all"]), ("indderivtables", [])):
        run([native / "src/gkends" / executable, "-L", *arguments], latin, "latin.log", env=data_env)
    # The distributed recipe names vbs.mpi, absent from the entire selected Git
    # history. Its shell pipeline continues with the four retained files. Make
    # that reconstruction explicit rather than inventing a replacement input.
    verbs = b"".join((latin / "stemsrc" / name).read_bytes() for name in
                     ("vbs.latin.bas", "vbs.latin.irreg", "vbs.latin", "vbs.irreg"))
    with (latin / "conjfile").open("wb") as target:
        run(["/usr/bin/perl", "-pe", "s/([a-z])([aei])_v[ \\t]+perfstem/$1\\t$2vperf/g;"],
            latin, "latin.log", env=data_env, stdin=verbs, stdout=target)
    run([native / "src/gener/do_conj", "-L"], latin, "latin.log", env=data_env)
    shutil.copyfile(latin / "conjfile.short", output / "vbmorph")
    run([native / "src/gkdict/indexvbs-isolated", "-L"], latin, "latin.log", env=data_env)

    runtime = output / "runtime"
    for language in ("Greek", "Latin"):
        for directory in ("rule_files", "endtables", "derivs", "steminds"):
            shutil.copytree(native / "stemlib" / language / directory, runtime / language / directory,
                            ignore=shutil.ignore_patterns(".cvsignore"))
        # Lemma resolution opens this source-side compound map at analysis time.
        (runtime / language / "stemsrc").mkdir()
        shutil.copyfile(native / "stemlib" / language / "stemsrc/vbs.cmp.ml",
                        runtime / language / "stemsrc/vbs.cmp.ml")
    runtime_files = [{"path": p.relative_to(runtime).as_posix(), "bytes": p.stat().st_size,
                      "sha256": digest(p)} for p in sorted(runtime.rglob("*")) if p.is_file()]
    runtime_identity = hashlib.sha256(json.dumps(runtime_files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()

    print("Building original C/Wasm with the recorded call-arity adaptation", flush=True)
    fixacc = wasm / "src/morphlib/fixacc.c"
    adapted, count = re.subn(r"\b(getsyll2?)\(([^,\n]+),([^,\n]+),(?:0|is_ending)\)",
                             r"\1(\2,\3)", fixacc.read_text())
    assert count == 16
    patch(fixacc, adapted, "wasm-call-arity.patch")
    helpers = output / "wasm-tools"
    helpers.mkdir()
    for command in ("ar", "ranlib"):
        (helpers / command).symlink_to(sdk / "upstream/bin" / ("llvm-" + command))
    wasm_env = dict(environment, PATH=str(helpers) + os.pathsep + environment["PATH"], EM_CACHE=str(output / "em-cache"))
    for library in ("greeklib", "morphlib", "gkends", "gkdict", "gener", "anal"):
        run(["make", library + ".a", "CC=" + str(emcc), "CFLAGS=" + WASM_FLAGS],
            wasm / "src" / library, "wasm.log", env=wasm_env)
    artifact = output / "artifact"
    artifact.mkdir()
    run([emcc, *WASM_FLAGS.split(), "stdiomorph.c", "../gener/genwd.o", "anal.a", "../gener/gener.a",
         "../gkends/gkends.a", "../gkdict/gkdict.a", "../morphlib/morphlib.a", "../greeklib/greeklib.a",
         "-o", os.path.relpath(artifact / "morpheus.mjs", wasm / "src/anal"), "-sMODULARIZE=1", "-sEXPORT_ES6=1", "-sINVOKE_RUN=0",
         "-sALLOW_MEMORY_GROWTH=1", "-sEXPORTED_RUNTIME_METHODS=callMain,FS,ENV", "-sENVIRONMENT=web,worker,node",
         "--preload-file", str(runtime) + "@/morphlib"], wasm / "src/anal", "wasm-link.log", env=wasm_env)

    # Pin the native reference's sorting library to the implementation shipped
    # with this Wasm SDK. Equal-key ordering is library-dependent in original C.
    # Preserve the system-libc executable as a separate comparison witness.
    native_binary = native / "src/anal/cruncher"
    shutil.copyfile(native_binary, native_binary.with_name("cruncher-system-sort"))
    sorting = output / "runtime-sort.c"
    sort_original = (PROJECT / "vendor/runtime/qsort.c").read_text()
    sort_code = sort_original.replace('#include "atomic.h"', '#define a_ctz_l(x) __builtin_ctzl(x)')
    sort_code = sort_code.replace("weak_alias(__qsort_r, qsort_r);", "")
    sorting.write_text(sort_code + "\n" + (PROJECT / "vendor/runtime/qsort_nr.c").read_text().replace("cmpfun", "cmpfun2"))
    run(["clang", *NATIVE_FLAGS.split(), "stdiomorph.c", sorting, "../gener/genwd.o", "anal.a",
         "../gener/gener.a", "../gkends/gkends.a", "../gkdict/gkdict.a", "../morphlib/morphlib.a",
         "../greeklib/greeklib.a", "-o", "cruncher"], native / "src/anal", "native-runtime.log")
    tools = {}
    for name, command in (("clang", ["clang", "--version"]), ("emscripten", [str(emcc), "--version"]),
                          ("perl", ["/usr/bin/perl", "-e", "print $^V"]), ("system", ["uname", "-srm"])):
        tools[name] = subprocess.check_output(command, env=environment, text=True).strip()
    manifest = {"schemaVersion": 1, "status": "0.0.1 retained-source engineering reconstruction",
                "source": {"repository": source["repository"], "revision": source["revision"],
                           "archiveSha256": lock["archiveSha256"]},
                "runtimeDataIdentity": runtime_identity, "runtimeFiles": runtime_files,
                "patches": patches, "tools": tools, "commands": commands,
                "artifacts": [{"file": p.name, "bytes": p.stat().st_size, "sha256": digest(p)}
                              for p in sorted(artifact.iterdir()) if p.is_file()],
                "limitations": ["Latin reconstruction explicitly uses the four retained verb sources; vbs.mpi is absent",
                                "Original checked-in morphology sources are inputs, not regenerated from lexica",
                                "No claim of current Tufts production equivalence or complete I/O coverage"]}
    (output / "build-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"Build complete: {output / 'build-manifest.json'}", flush=True)


if __name__ == "__main__":
    main()
