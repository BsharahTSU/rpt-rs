# Additional evaluation student batch 06

## Checkpoint 17a: bounded selection and page-label regression audit (2026-10-19)

After the [CR-001-B05 repair](cr-001-b05-parity.md), coverage starts at **18 accepted /
0 rejected / 80 untested**. SELECT-only discovery still finds 98 ready candidates.
The reviewed evaluation procedure hash is unchanged:
`1bcd37818cd5706ec611777ad0883022b5f7eb03929586d16e6a86a7eafb2a92`.
All eighteen previously captured students are excluded, counting repaired cases once.
A new choice is reserved before selecting the next template.

| Case | Rows | Subjects | English/Arabic source name length | New row count for template |
| --- | ---: | ---: | --- | --- |
| CR-001-B06 | 26 | 13 | 22 / 23 | Yes |
| CR-003-B06 | 52 | 22 | 35 / 26 | No; unseen row counts exhausted |
| CR-005-B06 | 35 | 12 | 32 / 26 | No; unseen row counts exhausted |

All source names contain Arabic characters; all comments are empty. Source name length
is not proof of rendered overflow coverage. Selection prefers novel row count, larger
reports, actual Arabic names, name lengths and a stable private ID tie. Remaining pools
before selection are 21 / 27 / 32 students. This is a bounded three-case batch.

The comparison tool now records printed page labels per physical page and distinguishes
label equality from complete coverage (exactly one recognized label on every page).
Four synthetic regressions protect reset sequences, equal physical counts with wrong
logical totals, missing/duplicate/unavailable captions, and line boundaries. The parser
recognizes the English `Page N of M` captions used by these templates; other languages
still need visual review. It removes format controls without altering Arabic/Chinese text.
Private batch acceptance requires both label gates in addition to the existing geometry,
ratings, decoded images, dates, page boxes and native replay checks. Existing tolerances
remain 0.061 pt X / 0.031 pt Y. Generic comparison remains diagnostic.

All **37 Python selection/glyph/comparison/rating tests** pass in capped Docker. Applying
the new gate to retained B05 evidence rejects its original wrong totals and accepts its
repair, preserving reset labels 1/2, 2/2, 1/1. No renderer change was needed. Seven Chinese
PDF tests, fresh exports and all-page review remain pending at this checkpoint.

Private evidence is in `research/stc-verification/additional-evaluation-batch-06/` on D:.
Scratch is `D:\tmpppppp123\rpt-rs-additional-batch-06`; target/temp remain under
`D:\tmpppppp123\docker-cache`. Sequential exports use two CPUs/2 GiB; comparison/tests
use 512 MiB where applicable. Each capture requires clean source, the accepted B05 worker
hash, RAM >= 3 GiB, C: free >= 30 GiB and D: free >= 10 GiB. Starting sample: RAM 6.37 GiB,
C: 311.49 GiB free, D: 533.84 GiB free, no task containers. The private wrapper now refuses
recapture before invoking SQL when retained run evidence already exists.

Verify and review each pair before the next export. Only SELECT discovery and the reviewed
read-only evaluation procedure are authorized; mutating ordinary-report procedures remain
excluded. No SQL/template/production settings are changed. Student IDs/names, captured rows,
RPTs, PDFs and logs stay ignored on D:. Existing main, local commits only; no branches,
worktrees, push or deployment. Rejected comparisons remain evidence for a focused repair.
