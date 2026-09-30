# Retry-memory causality and preservation

Recorded: 2026-09-30. This investigation supersedes the initial assertion that
the remaining trace difference required a new preservation-boundary decision.
The frozen original remains unchanged. Candidate 2 extends the explicit residual
memory model; it does not repair the original linguistic or retry rules.

## Finding

Three distinct layers explain the observed difference:

1. **Original defect:** `anal/checkstring.c` reads `*(a+2)` after `a+1` has become
   the string terminator. The current automatic buffer has not initialized that
   byte. `strchr("aeiou", 0)` correctly returns the position of the terminating
   NUL, so it does not protect the retry from this condition. In the minimized
   witness, the read is within the 60-byte array, beyond the initialized string;
   it is not an array-boundary overrun.
2. **Concrete executable behavior:** the frozen C/Wasm compiler reuses linear
   stack locations. Strings written by completed verb/ending routines determine
   the bytes later seen by the retry. Input order and compiled frame placement
   therefore affect the path. This is not a portable guarantee of C source.
3. **Translation omission:** candidate 1 retained the two retry frames but omitted
   intervening verb and nominal-index buffers. It therefore supplied the wrong
   retained byte. Rust can represent these bytes and transitions explicitly;
   neither different `strchr` semantics nor an inability of Rust to express the
   algorithm explains this witness.

The language standards support the distinction: an uninitialized automatic C
object has an indeterminate value ([N1570, 6.7.9p10](https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf));
the NUL is part of the string for `strchr` (7.24.5.2). Rust likewise cannot use
uninitialized storage as ordinary values ([MaybeUninit documentation](https://doc.rust-lang.org/core/mem/union.MaybeUninit.html)).
The compatibility model uses fully initialized byte storage, not an attempted
recreation of undefined behavior in Rust.

## Controlled witness

Input bytes: `dictatoriis\nobiectivis\n`; argv: `-T -L input`; file operation.
The selected original Wasm SHA-256 is
`c91ddc0e424197e3d3cd4e4d396979367f0d37101f608bdd6e9205b841914273`.
The data, input, loader and runtime settings stay fixed.

At `objecti`, the final `i` is offset 6 in a buffer beginning at linear address
93008. Offset 7 is NUL. The decisive read is offset 8, address 93016:

| Observation | Byte | Consequence |
| --- | ---: | --- |
| Frozen C/Wasm | 0 | No terminal-`i` retry |
| Candidate 1 | 106 (`j`) | Extra `objectj`/`obiectj` retry traversal |
| Same frozen C/Wasm, one byte changed to 106 at this read | 106 | Reproduces those extra retries |
| Same frozen C/Wasm, single-byte zero control | 0 | Retains original traversal |

The last original writer at 93016 is the NUL of `iectivis`, copied into
`analyzed_verb.tmpendstring` at 93008. Candidate 1 incorrectly retained the `j`
from the earlier `dictatorjis` buffer at the overlapping location.

These observations instrument the **already linked frozen binary**, not a
recompiled C program. Additional Wasm imports report reads/writes to JavaScript;
the observer introduces no C/Wasm linear-stack frame. Read-only observation
preserves both raw I/O and the complete final linear-memory hash in paired runs.
The intervention changes exactly one byte once. Raw output remains equal for
this witness, which is why internal comparison is necessary.

Addresses apply only to this pinned executable and invocation. Final-memory
hashes are paired observer-transparency checks, not a cross-environment product
contract. No physical address is included in ordinary internal-trace acceptance.

Reproduce with the pinned Emscripten/Binaryen installation:

```sh
python3 scripts/diagnose-retry-memory.py --reference build/reference/artifact \
  --emsdk /path/to/emsdk --output build/retry-diagnosis
```

The script refuses a different reference hash, preserves existing output
directories, records original and observed memory, and asserts the intervention.
`diagnostics/retry-condition.c` and `.rs` separately demonstrate that fully
specified 0/106 tail bytes produce the same predicate in both languages.

## Additional causal controls

The first verb-buffer correction passed all 40,000 forms and their internal
traces, but the subsequent 512 order probes exposed another omission. For
`cuiusquidem\ncuiusquidem\n`, the last writer of the decisive tail byte is the
nominal ending index's `checkendind.curtag`, inlined into `chcknend`. Restoring
that slot also requires the actual call depth: `u2v` and `chckend` each reserve
1,024 bytes. Omitting these reservations falsely projects the index write into
an outer retired retry frame, as `quaequidem\nIliaci\n` demonstrated.

The model now retains these observed locations:

| Frozen function / source role | Frame bytes | Retained slots |
| --- | ---: | --- |
| 20 / `checkstring3` | 128 | work 0, save 64; each 60 bytes |
| 21 / `checkstring4` | 192 | work 0, accent-stripped 64, save 128; each 60 bytes |
| inlined `analyzed_verb` | 64 below current frame | temporary ending, 60 bytes |
| 16 / `checknom`'s inlined `checkregnom` | 128 | half 0, work 64; each 60 bytes |
| 28 / `chcknend` | 128 | ending 0 (61 bytes), inlined index tag 64 (60 bytes) |
| inlined `u2v` | 1,024 | prefix buffer 0 |
| inlined `chckend` | 1,024 | key buffer 0 |

Linked WAT excerpts are retained in `probes/retry-memory/`. The source's `BUFSIZ`
and `LONGSTRING` both resolve to 1,024 in this selected ABI. This is a projection
of evidenced frame reuse, not an emulation of arbitrary host memory. All ordinary
translation control flow and string operations remain in their original places.

A separate original defect affects the `ex` retry: its `memmove` length excludes
the terminator. The earlier `subiectus\nexto\n` witness therefore loses an analysis
that appears for `exto\n` alone. A C-only control that adds automatic-variable
zero initialization to the retry compilation unit changes the output too,
without any Rust implementation involved. Such controls are diagnostic variants,
never replacement references. Preserving these defects belongs to 1.x; repairing
them requires the later engineering profile and its own regression expectations.

```sh
python3 scripts/diagnose-c-initialization.py --reference build/reference \
  --emsdk /path/to/emsdk --output build/c-initialization-controls
```

## Qualification boundary

Candidate 2 passes 3,200 corpus/mode groups (20,000 forms per language), with
1,223,194 source-checkpoint events. Independent post-link observation of the
original matches 170,374 complete `checkword` entry states over the same jobs.
The 584 source/order, 1,508 option/boundary and 2,560 targeted order fixtures also
pass the source checkpoints. Native raw I/O and three desktop browser engines,
including cold offline restart, pass their respective gates.

Receipts distinguish the historical failed candidate, isolated model experiments,
and the delivered candidate. Passing a finite corpus does not prove every possible
byte stream equivalent. New failures remain ordinary failing gates, and source
correspondence remains required alongside output equality. No word-specific
exception, output filter, trace-event suppression or C fallback is used.

This finding permits continued preservation construction without redefining the
reference. It does not establish equality to the unobserved Tufts deployment or
to every native C compiler/ABI. The released C/Wasm application remains available
independently of candidate qualification and publication.
