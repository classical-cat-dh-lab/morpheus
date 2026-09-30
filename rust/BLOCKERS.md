# Resolved preservation blocker: residual C stack state

Candidate 1 failed internal preservation checks. Candidate 2 resolves those
failures without changing the frozen original or waiving internal comparison.
[CAUSALITY.md](CAUSALITY.md) contains the causal evidence and
[qualification.json](evidence/qualification.json) records current acceptance.
The facts below retain the failed candidate's history.

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

## Initial inference and its correction

The initial local frame adapters did not reproduce the entire frozen executable's
memory reuse. Adding output filters, exceptions for individual words, ignoring
extra trace events, or silently fixing the original C would change the approved
preservation objective. None is accepted here. No online C service is involved,
and no C engine is invoked by the Rust candidate.

The initial report prematurely framed this as a choice between complete executable
memory emulation and a changed reference profile. That inference is withdrawn.
Direct observation of the linked original identifies missing verb/nominal string
writes and two omitted 1,024-byte frame reservations. Extending the existing
explicit model resolves the recorded failures while retaining source-level logic.
There is no required owner decision to relax the preservation objective.

All currently recorded candidate-2 gates pass. This is finite measured acceptance,
not proof for every possible input, compiler or historical Tufts deployment.
Any new discrepancy must receive causal analysis and an ordinary failing test.
