# Frozen Rust preservation core

Accepted on **2026-09-30** for the declared Greek/Latin `cruncher` boundary.
The qualified implementation is `perseus-b1b33c5-rust-candidate.2`, from source
commit `4f1ba0ae3a842bfaacaf89546bbff2efdf5f400e`. Its retained name does not imply
an unresolved blocker. [The baseline manifest](preservation-baseline.json) binds
the source/build inputs, reference, qualification receipts and artifacts.

This is a core acceptance and freeze. The released application remains C/Wasm
0.0.2; the next planned application milestone is Rust/Wasm 0.1.0, followed by
frontend qualification toward 1.0. The engine can remain unchanged through those
application releases. No stable 1.0 product or production replacement is asserted.

## Acceptance basis

[The qualification receipt](evidence/qualification.json) and its hashed receipts
record source correspondence for 119 selected C units / 470 functions, exact
Greek/Latin I/O, internal checkpoints and independent linked-original observation.
The 40,000-form corpus passes 3,200 batch/mode comparisons in Rust/Wasm and native
Rust; source probes compare 1,223,194 events, and linked-original probes compare
170,374 `checkword` entries without changing raw I/O. All recorded option, source,
boundary and retained-memory order fixtures pass. Three browser engines pass
offline cold restart; independent clean builds reproduce all seven artifacts.

On acceptance, the current 124 Rust source files, all 16 receipt hashes, the seven
delivery artifacts and the independently held delivery archive were checked
against those records. This identifies the tested implementation rather than
assuming that an old passing report describes the current working tree.

The earlier divergence is causally resolved in [CAUSALITY.md](CAUSALITY.md).
The original reads residual bytes beyond a string terminator. The port explicitly
models the evidenced writes and reads, preserving the original retry behavior.
It does not correct that historical defect, filter results, special-case words or
fall back to the original C engine. Platform libc remains distinct from the
translated morphology program; no original morphology C object is linked.

## Exact scope

- Reference: the retained PerseusDL source and fixed portable C/Wasm executable,
  data and environment, not an unverified live Tufts deployment.
- Operations: Greek/Latin `cruncher` and bounded `cruncher-file` under
  [the existing protocol](../docs/api.md) and [operation inventory](../docs/operations.md).
  Deterministic byte comparisons explicitly use `-T`; they do not delete timing
  output after execution. Fresh instances isolate jobs; words within a job retain
  original batch state. Candidate order, multiplicity, raw channels, files and
  termination remain within the comparison.
- Internal fidelity: source/control-flow correspondence and the recorded logical
  checkpoints, including the historical defect's causal state. This is not a
  claim of identical physical addresses, instruction streams or exhaustive proof
  over every possible input byte sequence.
- Host admission limits and explicit unsupported operations remain visible.
  Historical companion executables, `-e`'s undefined formatting, Italian mode,
  timing equality, arbitrary host files and unqualified allocation-failure paths
  are not accepted operations by implication. The retained unreachable helpers
  are not advertised as completed standalone tools. See [README.md](README.md)
  and `engine/return-audit.json` for translation dispositions.
- Native acceptance currently covers the recorded Apple Silicon build. Browser
  evidence covers the recorded Chromium, WebKit and Firefox versions; it is not
  an assertion of testing every physical device or operating system.
- Unicode conversion, passage selection and dictionary presentation remain outside
  the engine. They must retain the original input/result record and cannot
  silently change this profile's bytes or data.

These limits define the accepted core. They do not waive an unexplained difference
inside it. There are no known unresolved in-scope mismatches in the recorded
qualification. Broader historical-tool support remains a separately named scope.

## Maintaining the freeze

`npm test` checks protected source/build inputs and receipts before application
tests. To verify the already built delivery as well:

```sh
node scripts/check-rust-baseline.mjs --artifacts build/rust-delivery-002
```

Keep `preservation-baseline.json` and its evidence immutable. Do not refresh hashes
to make a changed core pass. A necessary preservation repair requires a causal
record, requalification and a linked successor manifest. A frontend-only release
keeps the existing engine identity and bytes. A toolchain/runtime change requires
requalification rather than reliance on the source language alone.

The manifest is a drift guard, not proof of correctness by itself. The core's
source revision, qualification records and archived artifact hashes supply the
reproducibility chain; future publication must bind the same chain to release
assets and an exact version archive.
