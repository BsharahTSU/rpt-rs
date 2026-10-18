# Additional evaluation student batch 04

## Checkpoint 13a: bounded selection and test preparation (2026-10-18)

Refresh SELECT-only eligibility and compare three new students sequentially, one per
existing evaluation template, through original Crystal and Docker Rust. The current
inventory still has 98 ready students; all twelve previously captured students are
excluded, including the historically rejected and subsequently repaired CR-001-B03.
No new case is accepted merely because it is eligible. Current pre-batch coverage is
12 accepted / 0 rejected / 86 untested.

| Case | Rows | Subjects | English/Arabic source name length | Arabic characters in source |
| --- | ---: | ---: | --- | --- |
| CR-001-B04 | 33 | 14 | 24 / 22 | Yes |
| CR-003-B04 | 34 | 15 | 31 / 30 | Yes |
| CR-005-B04 | 35 | 12 | 32 / 28 | Yes |

Selection first prefers an untested row count for that template, then larger reports,
actual Arabic-name characters, source-name lengths and a stable ID tie-break. The first
two cases add new row counts. CR-005 has no unseen row count; its name now contains
Arabic, unlike B03's Latin text in the Arabic-name source field. Formulas can shorten
source names, so actual displayed-name overflow remains a separate check. Comments
are empty. Full selectors and data remain private; public records use opaque case IDs.

Added pure `scripts/stc/select_batch.py` and six synthetic regressions: exclude every
captured student regardless of prior outcome/template; keep row novelty template-local;
prefer actual Arabic over long Latin source text at equal row count; reject ineligible,
wrong-template, ambiguous duplicate and zero-row candidates; deterministic ties.
All 30 Python selection/glyph/comparison/rating checks pass in capped Docker.
No renderer changes have been made for this batch. Chinese and actual paired exports
remain pending at this checkpoint.

The evaluation procedure definition still matches the reviewed SELECT-only hash.
Standard/mutating procedures are not called. Read-only metadata discovery completed
in about nine seconds; report execution is bounded per case by the existing runner.
Authenticated Parent access/delivery and other-template availability retain their gaps.

Private evidence: `research/stc-verification/additional-evaluation-batch-04/` on D:.
Scratch: `D:\tmpppppp123\rpt-rs-additional-batch-04`; target/temp cache remains on D:.
At initial inspection RAM was only 2.86 GiB, so exports were held until it recovered.
At 17:13:06: RAM 4.88 GiB available, C: 312.73 GiB and D: 534.41 GiB free. Require
3 GiB RAM, 30 GiB C: and 10 GiB D: before each export; use sequential two-CPU/2-GiB
containers and 512-MiB comparisons. No unrelated application is closed or reconfigured.
Local commits only on existing main; no branches/worktrees, push or deployment.
