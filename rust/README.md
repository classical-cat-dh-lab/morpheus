# Rust preservation candidate

**Status: builds and runs; preservation acceptance is blocked. Do not replace the
released C/Wasm engine with this candidate.** See [BLOCKERS.md](BLOCKERS.md).
The candidate is a source translation, not a wrapper around original morphology
C objects. All 119 selected compilation units and 470 source function definitions
have Rust counterparts. The historical utilities outside the `cruncher` process
boundary are not advertised as supported executables.

The retained C source pin is PerseusDL/morpheus
`b1b33c56ef2338fe0dcd1893628ed638f00c0986`. `translation-units.json` records the
portable translation input; `c-source-map.json` and `function-correspondence.json`
connect original functions to Rust. The original notices and source remain in
`vendor/morpheus-original.tar.gz`. This translation retains the original branch
structure, globals, bitfields, pointer operations, candidate order and formatting.
It intentionally uses `unsafe` where C memory operations require it. It is not a
safe/idiomatic redesign, a scholarly correction, or a completed 1.0 release.

## Build and run

Use Rust 1.98.1 with the `wasm32-unknown-emscripten` standard library, Emscripten
6.0.6, Python 3.14, and Apple Clang. Supply explicit paths; no global toolchain or
OS configuration change is required. Cargo dependencies are vendored and locked.
A completed original reference build supplies the **unchanged morphology data**.

```sh
python3 scripts/build-engine.py --emsdk /path/to/emsdk --output build/reference
python3 scripts/build-rust.py --toolchain /path/to/rust --emsdk /path/to/emsdk \
  --reference build/reference --output build/rust-candidate
MORPHLIB="$PWD/build/reference/runtime" build/rust-candidate/native/cruncher -T
printf 'lo/gos\n' | node build/rust-candidate/artifact/cli.mjs -T
printf 'amo\n' | node build/rust-candidate/artifact/cli.mjs -T -L
```

The builder emits a native macOS executable, Rust/Wasm and loader, the data pack,
the unchanged `morph-engine/1` wrapper, an explicit candidate implementation
identity, a CLI, logs and a build manifest. No original morphology C object enters
the Rust link. The native executable links the separately retained musl sorting
runtime; Wasm uses Emscripten's platform runtime. libc is not the morphology engine.
The builder remaps local source paths out of binaries.

Each job requires fresh process/instance state. The native executable expects an
explicit `MORPHLIB` directory; the Wasm pack mounts it at `/morphlib`. Native timing
uses the host clock ABI. Timing-disabled Wasm is the canonical comparison target;
indeterminate native stack contents are not assumed to match Wasm.

## Translation record

C2Rust 0.22.1 provided an initial mechanical translation. It is a build tool, not
an oracle. `toolchain.lock.json` pins it and the bootstrap downloads. Two small
tool patches suppress host include injection for a Wasm cross-target and classify
an unused LLVM 20 Wasm builtin. Neither changes Morpheus logic.

1. `scripts/prepare-rust-port.py` verifies the original archive and the three
   existing portability patches, then emits the 119-unit compilation database.
2. `scripts/inventory-rust-source.py` records Clang AST definitions and call edges.
3. `scripts/resolve-rust-prototypes.py` resolves old implicit declarations from
   original definitions. The translator then receives explicit cross-unit types.
4. C2Rust is run with `--binary stdiomorph --fail-on-error --disable-rustfmt`.
   Three unused static functions are also retained with
   `--preserve-unused-functions`; none is silently treated as runtime coverage.
5. Recorded Rust adaptations resolve discarded implicit returns, `size_t`, C ABI
   function pointers, Darwin stdio symbols, bitfield alignment, symbol collisions
   in unused archive members, the driver entry point and standalone build layout.
   `translation-adjustments.patch` records the edits from the initial translation.

The original `low_bit_of` omits a final return. Its frozen optimized Wasm returns
zero on that path; `probes/low-bit-of-frozen.ll` records the compiler evidence.
This is preserved, not replaced by a mathematically corrected bit operation.
`engine/return-audit.json` records caller use of the other missing returns.
Unreachable missing-argument helpers and the undefined allocation-failure return
explicitly refuse execution; the candidate does not invent their values.

`legacy_stack.rs` is an **incomplete compatibility experiment**. Original Latin
retry code reads after a terminator and shifts a string without moving its final
NUL. Zero-initialized Rust arrays change observed results. The experiment retains
bytes shared by the original 128-byte and 192-byte retry frames, whose offsets are
recorded in `probes/retry-frames.s`. This recovers observed I/O differences without
changing spelling rules, but it does not emulate every retired C frame. Residual
internal mismatches remain, so this adapter is not accepted as a complete solution.

## Verify

```sh
(cd rust/engine && cargo test --offline --frozen)
python3 scripts/build-rust-probes.py --reference build/reference \
  --emsdk /path/to/emsdk --output build/c-probes
python3 scripts/build-rust.py --toolchain /path/to/rust --emsdk /path/to/emsdk \
  --reference build/reference --output build/rust-trace --trace
node scripts/verify-rust-fixtures.mjs build/reference/artifact \
  build/rust-candidate/artifact build/c-probes/wasm build/rust-trace/artifact \
  rust/fixtures/source-and-order.json build/source-and-order.json
node scripts/verify-rust-fixtures.mjs build/reference/artifact \
  build/rust-candidate/artifact build/c-probes/wasm build/rust-trace/artifact \
  rust/fixtures/options-and-boundaries.json build/options-and-boundaries.json
node scripts/verify-rust.mjs build/reference/artifact build/rust-candidate/artifact \
  /path/to/frozen-corpus-input.json build/corpus.json \
  build/c-probes/wasm build/rust-trace/artifact
```

Eight logical checkpoints cover word/noun/verb entry, generation checks, candidate
addition (both arguments), and analysis state before/after original sorting.
Serialization includes grammatical fields, flags, string components and ordered
analyses, excluding pointers/padding. Wasm C probes read linear memory in JavaScript
without a C logging stack frame; native witnesses also exist. Traced outputs are
compared with uninstrumented outputs. Failed comparisons remain failures: there
is no normalization, ignored candidate order, ignored extra branch, or C fallback.

Known-failing probes are in `fixtures/known-divergence.json`. Running them must
remain a failed acceptance gate until the preservation blocker is resolved. Passing
external I/O tests alone does not satisfy this project's internal fidelity rule.

## Licenses

The translated morphology code remains under the original CC BY-SA 3.0 US terms
with original attribution retained. C2Rust bitfields and its derive helper use
BSD-3-Clause; see `../licenses/C2Rust-BSD-3-Clause.txt`. Vendored proc-macro2, quote
and syn retain MIT/Apache-2.0 notices; unicode-ident additionally retains its
Unicode notice. Rust's standard library notices are retained separately. These
component terms are not replaced by the morphology adaptation's license.
