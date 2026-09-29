# Engine protocol `morph-engine/1`

Import `runEngine`, `identity`, `protocol` and `limits` from `engine/runtime.mjs`.
The interface surrounds one original `cruncher` process and its morphology data.
Each call creates a fresh Wasm instance. A multi-line job retains original state
within that process; later jobs do not inherit it.

The request is an object with these fields:

| Field | Meaning |
|---|---|
| `protocol` | Exact `morph-engine/1` |
| `id` | Caller-supplied job identity, echoed unchanged |
| `profile` | Explicit installed behavior profile from `identity.profile` |
| `implementation` | Explicit installed implementation from `identity.implementation` |
| `data` | Exact generated morphology-file-set SHA-256 from `identity.data` |
| `operation` | `cruncher` or `cruncher-file` |
| `argv` | Original ordered argument array; see `operations.md` |
| `stdin` | `Uint8Array`, validated before execution |

The result echoes the effective identity and operation, and contains `stdout` and
`stderr` byte arrays, `files` (a name-to-byte-array object) and `termination`.
A returned normal exit is `{kind:'exit', code:number}`; a Wasm execution trap is
`{kind:'trap', message:string}` with the raw bytes already produced. Validation,
asset loading and host failures reject the promise and are not fake original exit
codes. The browser owns cancellation/timeouts by terminating its dedicated Worker;
no partial result is presented as successful completion.

The internal filesystem is `/job`, with `MORPHLIB=/morphlib`, locale `C` and TZ
`UTC`. File mode writes request bytes to `input.words`, invokes the original
basename `input`, and returns `input.morph`, `input.failed`, `input.stats`.
No arbitrary host paths or network service are exposed to C.

Current host limits are 64 KiB, 1,000 lines and 48 ASCII bytes per line; stdout and
stderr together are bounded at 16 MiB. Original output is never sorted, deduplicated
or normalized by the wrapper. Bytes are compared under the explicitly selected
reference/runtime contract. No cross-stream write-interleaving order is promised.

`runEngine` accepts host resource options `locateFile`, `wasmBinary` and `data`.
The browser Worker loads and verifies exact asset sizes and hashes before execution.
Node CLI/tests use the shipped local files. These options change transport, not
engine selection or behavior. No alternative implementation is selected on failure.

The browser execution export stores byte arrays as base64 and also records original
user text, source spans and the `morph-input/1` conversion. Unicode input and rendered
output are outside the engine boundary; raw original-mode bytes are not rewritten.
Later C/Rust implementations and 1.x/2.x/3.x profiles retain existing protocol-field
meanings. New behavior is an explicit profile/implementation/data selection; a new
capability must be additive or use an explicit protocol adapter.
