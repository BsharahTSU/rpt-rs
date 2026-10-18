# Additional evaluation student batch 05

## Checkpoint 15a: bounded selection and regression preparation (2026-10-18)

Refresh SELECT-only eligibility and compare three new students sequentially, one per
current evaluation template. After the [CR-001-B04 repair](cr-001-b04-parity.md), prior
coverage is 15 accepted / 0 rejected / 83 untested. The refreshed inventory still has
98 ready candidates and the same reviewed read-only evaluation procedure hash. All
fifteen previously captured students are excluded, including the two repaired cases.

| Case | Rows | Subjects | English/Arabic source name length | New row count for template |
| --- | ---: | ---: | --- | --- |
| CR-001-B05 | 28 | 13 | 33 / 27 | Yes |
| CR-003-B05 | 52 | 22 | 36 / 32 | No; unseen row counts exhausted |
| CR-005-B05 | 35 | 12 | 23 / 27 | No; unseen row counts exhausted |

All three have actual Arabic characters in the source name; comments are empty.
Name formulas can shorten the source value, so source length does not establish
visible overflow coverage. Selection prefers an unseen row count per template, then
larger reports, actual Arabic names, source-name lengths and a stable private ID tie.
A selected student is now reserved before selecting the next template; historical
exclusion count stays separate from current-batch reservations. Remaining pools before
selection are 22 / 28 / 33 students across these templates, totaling 83 untested.

Three new synthetic selection tests protect exhausted row-count novelty, duplicate
rejected/repaired history, and excluding the first selection from a different template
in the same batch. All **33 Python selection/glyph/comparison/rating checks** pass in
capped Docker. No shared selector or renderer change was needed. Chinese PDF checks and
all actual exports/visual acceptance remain pending at this checkpoint.

The native runner rechecks the reviewed procedure definition hash before capture.
Only SELECT discovery and the manually reviewed read-only evaluation procedure are
allowed. Mutating ordinary-report procedures remain excluded; neither publication/access
settings nor database/template/production configuration is changed. SQL eligibility,
actual native export, Rust export and visual parity remain separate gates.

Private evidence lives in `research/stc-verification/additional-evaluation-batch-05/`
on D:. Scratch is `D:\tmpppppp123\rpt-rs-additional-batch-05`; target/cache temp remain
under `D:\tmpppppp123\docker-cache`. Each export requires clean source, the accepted
CR-001-B04 worker hash, 3 GiB available RAM, 30 GiB C: free and 10 GiB D: free. Containers
run sequentially with two CPUs/2 GiB; comparisons use 512 MiB. At the initial 17:49:20
sample RAM was 5.22 GiB available, C: 312.09 GiB free and D: 534.01 GiB free. No task
containers were running. Do not stop unrelated applications or widen tolerances to pass.

Discovery took about ten seconds and remains bounded by statement timeouts. Verify and
review each pair before starting the next export. Preserve a rejected pair for a focused
repair rather than accepting it from matching ratings/page counts alone. No credentials,
student names/IDs, RPTs or report PDFs are committed. Local commits on existing main only;
no new branches, worktrees, push or deployment.
