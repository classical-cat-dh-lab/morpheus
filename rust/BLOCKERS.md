# Preservation acceptance blocker: residual C stack state

The current Rust implementation is **not qualified for engine replacement**.
It builds, executes Greek/Latin queries and passes broad raw-output tests, but the
required internal behavior comparison still has known failures.

## Reproducible facts

- Direct translation with zero-initialized Rust arrays passed the original 456
  fixtures and 2,000-form sample. It then failed 26 of 3,200 batch/mode groups in a
  40,000-form sample. `subiectus\nexto\n` is a small external-output witness:
  Rust produced an additional `exsto_` analysis that frozen C omitted.
- Retaining only each recursion depth's `checkstring3` work buffer recovered all
  40,000-form I/O groups, but failed 18 internal traces in 584 order/option probes.
- Sharing the two compiled retry frames recovered those 584 probes and a further
  1,508 option-combination and byte-length probes. It still failed 12 trace groups
  (six distinct batches, both stdin/file modes) in the 40,000-form trace run.
  All 3,200 raw-output groups continued to match.
- The failure was retested with read-only JavaScript probes instead of C logging.
  It remains. A minimized input is `dictatoriis\nobiectivis\n`, with `-T -L`, in
  original file mode. Rust takes extra `objectj` retry branches; final raw output
  agrees. Passing output cannot waive this internal difference.

The original source at `anal/checkstring.c` reads `*(a+2)` even when `a+1` is the
terminating NUL; `strchr("aeiou", 0)` also succeeds. Its `ex` retry calls `memmove`
with `strlen(p_word)`, omitting the NUL. Its `Xstrncpy` actually calls `strcpy` and
does not clear the remaining buffer. Consequently, previous inputs, recursion and
other functions' retired stack frames can influence subsequent analysis paths.

## Why the gate remains closed

The tested local frame adapters do not reproduce the entire frozen executable's
memory reuse. Adding output filters, exceptions for individual words, ignoring
extra trace events, or silently fixing the original C would change the approved
preservation objective. None is accepted here. No online C service is involved,
and no C engine is invoked by the Rust candidate.

A complete executable-memory compatibility model is possible in principle. It
would be substantially different from a source-led Rust baseline: frame layout,
compiler transformations and potentially addresses become maintained semantics.
Alternatively, a narrowly documented portability adaptation could establish a
new defined reference for these indeterminate reads, while retaining the old
C/Wasm edition as an explicit historical profile. That changes the preservation
boundary and needs an owner decision. This task has not made that decision or
claimed either route is already authorized.

The remaining independent work—source correspondence, vendored offline build,
protocol packaging, differential tools and browser integration verification—is
retained so the decision can resume construction rather than restart translation.
