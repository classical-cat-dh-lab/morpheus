# Original operation inventory and 0.0.1 disposition

The source pin is `PerseusDL/morpheus@b1b33c56ef2338fe0dcd1893628ed638f00c0986`.
This table accounts for the 26 default executable targets in its four component
Makefiles. It separates the delivered engineering application from later complete
preservation qualification. The archived source includes non-default targets and
historical scripts; these are not implied supported browser interfaces.

| Original component/targets | Disposition in 0.0.1 |
|---|---|
| `anal/cruncher` | Delivered browser/CLI/API, Greek and Latin; stdin and fixed-basename file modes |
| `anal/pname`, `findbase`, `deverbal` | Original sources retained; not browser operations; qualify separately toward 1.0 |
| `gener/do_conj` | Native build tool used to reconstruct morphology data |
| `gener/gener`, `checkstype` | Sources retained; original generation/control-record grammar still needs standalone operation qualification |
| `gkdict/indexnoms`, `indexvbs` | Native data indexers used by the reproducible build |
| `gkdict/indexcomps`, `newlems`, `newlems2`, `setquant`, `splitlems`, `splitlat`, `latvb`, `conj1`, `combitype`, `latnom`, `fixhesc`, `fixgend` | Original data-import/maintenance sources retained; not application operations or recreated dictionary imports |
| `gkends/buildend`, `indendtables`, `buildderiv`, `indderivtables` | Native build tools used to compile original rules/tables |
| `gkends/buildword` | Original source retained; not a browser operation |

The analyser's internal generation, retries and sorting remain inside the compiled
C boundary. Standalone generation is a different input grammar and is not exposed
as an invented lemma-to-paradigm API.

## Options and bounded transport

The API accepts original flags `-T -L -a -l -m -b -c -k -i -d -s -n -x -S -V -p -P`.
Greek is the original default and `-L` selects Latin. The UI explicitly supplies
`-T`, optional `-n` and `-S`, and `-L` for Latin. There are no extra host retries.

- `-I`: historical Italian selector; outside the Greek/Latin product and not packaged.
- `-e`: unavailable because the source passes a struct to `%d`, producing
  ABI-dependent undefined output. Preserve and resolve separately for 1.0.
- `-o` and arbitrary positional paths: unavailable through this bounded host
  interface. `cruncher-file` provides the original `input.words` to `.morph`,
  `.failed`, `.stats` workflow; it requires explicit `-T`.
- Timing output without `-T`: retained where stdin mode requests it; no timing
  byte-equality claim. The declared differential tests all supply `-T` explicitly.
- Control/NUL/non-ASCII bytes and inputs exceeding the declared limits: rejected
  before entering the C program. This is a host capability boundary, not silent
  truncation or a claim to emulate every malformed-input native failure.

Comments, CRLF, trailing fields, missing final newline and repeated words are
passed unchanged in original mode. A “no analysis” result is distinct from a trap,
timeout, cancellation or a rejected request. Original output channels are retained
independently of the readable UI and its hit counter.
