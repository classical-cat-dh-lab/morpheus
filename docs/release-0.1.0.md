# Morph 0.1.0 — Rust beta

The application now uses the accepted Rust preservation core in the browser.
Its source and compiled artifacts are unchanged from the qualified baseline:
`perseus-b1b33c5-rust-candidate.2`. The retained implementation name is a provenance
identifier, not the application's release status. This is the first Rust **beta**;
frontend refinement and student feedback remain on the path to 1.0.

The engine preserves the declared original Greek/Latin analysis boundary,
including raw bytes, ordering, duplicate analyses, internal state and the original
retry-memory defect. The selected original source is PerseusDL `b1b33c5`, with the
documented portable runtime and reconstructed data. Equivalence to today's Tufts
server is not claimed. The source contains the frozen baseline, source mapping,
causal investigation and qualification receipts. There is no silent C fallback.

All analysis runs on the user's device, even when opening the website without
installing. The application recommends complete offline saving and PWA installation
for regular use. After saving, the large prompt becomes a compact update control.
Updates reuse verified unchanged data, check the saved Greek/Latin analyzer and
wait for explicit reload. Interrupted downloads retain the previous complete copy.
Both complete, uncollated LSJ and Lewis & Short dictionaries remain available
offline, with the accepted passage/word-selection interface and original IDs.

The release includes a reconstructible source archive, a ready-to-host static
application, and an additional native Rust CLI/data package for Apple Silicon
macOS. Native CLI refinement and other native platform builds remain later work.
The browser application requires no compiler, Node.js, remote analysis service,
user account or periodic online authorization. Browser storage can still be
cleared; independently retained archives provide an additional holding.

The app retains `morph-engine/1`, morphology data and the preservation profile.
Frontend integration does not reopen the accepted backend or authorize original
engineering/scholarly repairs. Code/core: CC BY-SA 3.0 US; dictionary source/display
packs: CC BY-SA 4.0; runtime/font components retain their own notices.
