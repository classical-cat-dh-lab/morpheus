# Engineering reference and compatibility

The release identifies source, runtime, data and protocol separately. It preserves
the original analyser and original morphology records. It does not identify the
current Tufts production binary or database and does not make that equivalence claim.

## Portable runtime

The initial unchanged source on Apple libc trapped on overlapping `strcpy` calls.
The builder records the minimal `memmove` adaptations in the standalone conjugation
builder and `checkcomderiv2`. It also removes 16 surplus arguments at the original
`getsyll`/`getsyll2` call sites for the Wasm ABI. Native and Wasm analyse the same
generated morphology files. No rule, lexical record or candidate ranking criterion
is corrected by these adaptations.

The expanded Latin probe also exposed overlapping copy in `strippreverb` for
`esse`. The same narrow `memmove` adaptation is applied and recorded on both
targets. The original `-e` mode passes an entire `word_form` struct to a `%d`
format; native and Wasm ABIs print different values. This undefined diagnostic
mode is explicitly unavailable in 0.0.1, not normalized into a false parity pass.
Its disposition remains in the full preservation inventory for 1.0.

The original candidate comparator compares only lemma strings. Equal-key order
therefore depends on the host sorting library. The portable native reference pins
the musl sorting implementation supplied with Emscripten 6.0.6; the corresponding
system-libc binary remains a separate witness. This fixes the environment rather
than sorting output in the test harness. It does not assert that musl order was
historically used by Tufts. Future 1.x qualification must retain this distinction.

The same compiler/data-source versions and flags are recorded by the build tool.
Runtime-library adapters do not authorize an unrecorded new morphology algorithm.

## Data reconstruction

Greek indexes are rebuilt from the original curated stems and original tables.
Latin's distributed recipe lists five verb files, but `stemsrc/vbs.mpi` is absent
from the selected tree/history. Its shell pipeline can continue using the four
present files. This release explicitly records those four inputs and applies the
original Perl substitution and conjugation/index tools. It does not claim recovery
of the absent records or reproduction of the earlier lexicon-import process.

The source package excludes one unused historical file, `src/play/y.tab.c`, whose
OSF/1 generated-parser notice differs from the repository's default license.
It is not part of these build targets. `source.lock.json` records the exclusion
and distinguishes the public source archive from the retained original archive.
All required engine and data-build sources are included unchanged before patches.

## Protocol and delivery

`morph-engine/1` accepts an explicit profile, implementation, data identity,
operation, original argument list and byte-array stdin. It returns the same
identity, raw stdout/stderr, named generated files and termination.
`cruncher` uses stdin; `cruncher-file` stages the supplied bytes as `input.words`
and returns the original `input.morph`, `.failed` and `.stats` files. This bounded
file operation requires explicit `-T`; arbitrary host paths are not accepted.

The UI uses `-T` explicitly. Original timing modes are not normalized into byte
parity. Each job starts with fresh process state. Unsupported operations/options,
non-ASCII original input and declared size-limit violations fail before execution.
Host failures and Wasm traps are distinct from original successful exit status.

Unicode conversion belongs outside the engine. Execution exports include the
original user text, converted records and delivered bytes; the raw result is
never replaced by the rendered view. No silent C fallback will hide later Rust
failures. Later 2.x/3.x profiles may change results while keeping the protocol.

## Validation boundary

The first 92 selected-original Greek probes became byte-identical after the
recorded runtime alignment. A passing fixture set is not exhaustive correctness.
The final option/input matrix records 456 exact comparisons. The separately
selected Kaikki sample records 2,000 forms tested in both stdin and file modes
(160 batch/mode comparisons), including raw streams, generated files and exit
status. The wider sample identified the original runtime dependency on
`stemsrc/vbs.cmp.ml`; the unchanged Greek/Latin compound-lemma maps are included
in the runtime data. `evidence/` retains the receipts; only their recorded scope
is claimed.
