# Morph 0.0.1 — engineering edition

The original Perseus Morpheus C engine now runs locally in a browser for Ancient
Greek and Latin. A complete, verified offline download keeps analysis available
without a working central morphology service. The static package can be hosted
independently; the source package includes the original build inputs and data.

This edition establishes the replaceable `morph-engine/1` byte interface, a fresh
Worker/process per job, transparent Unicode conversion, readable analyses, raw
stdout/stderr and execution-record downloads, and bounded CLI/file-mode access.
The original driver, retries, candidate order and linguistic rules remain inside
C. Later Rust/Wasm work will replace this implementation behind the same interface.

## Evidence and limits

- 456 selected Greek/Latin option and input cases agree byte-for-byte with the
  declared portable native reference.
- 2,000 deterministically selected Kaikki forms (1,000 per language), tested in
  stdin and original file modes, agree across all 160 batch/mode comparisons.
  Output, diagnostics, generated files and termination are compared separately.
- Independent native-data rebuilds produce the same data identity and Wasm binary.
- Chromium, WebKit and Firefox exercise real analysis and complete offline use,
  including full browser-process exit/restart with the origin unreachable.
  Failure, cancellation, damaged-cache and quota-recovery tests retain a previous
  complete offline edition. This is automated desktop-browser evidence, not a
  claim of physical iPhone/Android device acceptance.

The original pin is `PerseusDL/morpheus@b1b33c5`. Necessary portability adaptations
and the musl sort reference are documented; historical Tufts equal-key order and
the current live-server binary/data are not identified. Latin explicitly uses the
four retained verb inputs because `vbs.mpi` is absent. The original undefined `-e`
diagnostic, arbitrary output paths and standalone companion/generation operations
are not exposed by this engineering edition. See `docs/operations.md`.

This is not the 1.0 full preservation qualification or a scholarly revision.
Dictionary definitions/adapters are deferred. Original errors and missing analyses
remain possible. Tests establish only their declared coverage, not all possible I/O.

CC BY-SA 3.0 United States governs the inherited core/data and this adaptation;
runtime/font components retain their separate notices. Original Morpheus credit
belongs to Gregory Crane and contributors, Perseus Digital Library, Tufts University.
This independent adaptation does not imply their endorsement.
