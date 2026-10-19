# Additional evaluation student batch 05

**Subsequent repair accepted (2026-10-18).** The [CR-001-B05 repair](cr-001-b05-parity.md)
closes the printed-total/date-footer rejection with fresh original/Rust PDFs from `0e6476a`.
All 3,962 painted glyphs and 28 ratings match within unchanged limits; all six pages reviewed.
Five prior accepted Rust PDFs remain byte-identical, including CR-003/005-B05. Coverage
at that repair: **18 accepted / 0 rejected / 80 untested**. The batch findings below preserve
the original rejection; the repair supersedes that case's status.

**Final result (2026-10-18): two accepted, one rejected.** CR-003-B05 and CR-005-B05
pass all-page review; CR-001-B05 exposes wrong printed page totals, missing date blocks
and raised footers. No renderer repair was claimed at this batch checkpoint. Coverage then was **17 accepted /
1 rejected / 80 untested**, with eighteen distinct students compared. Earlier checkpoint
statements below retain the state at that time.

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


## Checkpoint 15b: CR-001-B05 rejected for page-total/date-footer differences

All capture/native-live/native-frozen/rust-frozen stages pass from clean `c66e672`.
The three PDFs pass qpdf and contain three equal physical page boxes. Native live/frozen
page PNGs are identical. All 28 source ratings match in engine order, allocated 15 / 13 / 0,
and decoded image RGB matches exactly. These successes do not establish visual parity.

Coordinator and independent all-six-page reviews reject this case:

- Native printed page labels are **1 of 2, 2 of 2, 1 of 1**; Rust prints **1 of 3,
  2 of 3, 1 of 3**. The sole page-one outline difference is the page-total digit, not
  the Arabic name. Physical PDF page count alone does not capture reset semantics.
- Rust omits the entire **Print Date block on pages two and three**, 38 painted glyphs
  each. Native date rows are retained in both live and frozen exports.
- The legends and page footers on those pages are **16 pt higher** in Rust: native
  page-number baseline is y=12 pt, Rust y=28 pt in PDF coordinates. Body, headings,
  Arabic-name shapes, signatures and final note align; this is not the B04 body/signature
  displacement and not a newly observed Arabic shaping failure.

Painted glyph inventories are 1,789/1,789, 1,944/1,906 and 229/191 (native/Rust), totaling
3,962/3,886. Geometry and per-page date gates fail as intended; matching ratings/page
counts cannot override rejection. Root cause is not established. Retain the frozen pair
for focused reset-aware page-total and conditional-footer investigation; do not change
source data, templates or tolerances to force a pass. No renderer repair is claimed.
The previous CR-001-B04 acceptance remains valid for its own captured input.

Private `review/visual-review.json` explicitly records rejection. The comparison runner
exits nonzero; the other two students remain pending at this checkpoint. Initial review
considered a possible name-outline issue, but all-page/operator inspection identified
the differing glyph as the page-total digit and ruled out that visual interpretation.


## Checkpoint 15c: complete sequential batch and acceptance

| Case | Native / Rust physical pages | Ratings per page | Native / Rust painted glyphs | Final result |
| --- | --- | --- | --- | --- |
| CR-001-B05 | 3 / 3 | 15 / 13 / 0 | 3,962 / 3,886 | Rejected; page totals/date footers above |
| CR-003-B05 | 3 / 3 | 18 / 21 / 13 | 6,456 / 6,456 | Accepted within existing limits |
| CR-005-B05 | 3 / 3 | 16 / 12 / 7 | 4,927 / 4,927 | Accepted within existing limits |

All three capture/native-live/native-frozen/rust-frozen stages pass. All nine exported
PDFs pass qpdf; native live/frozen page PNGs are identical in each case. Each pair has
equal page boxes and decoded image RGB. All **115 ratings** match engine order and source
multiplicity, including the rejected case. Dates agree in the accepted two cases only.
The six final paired PDFs and all eighteen page PNGs are retained and reviewed; intermediate
native-live PDFs establish the replay checks. Each case has an explicit visual-review record.

CR-003-B05 matches all 6,456 painted glyphs (2,052 / 2,187 / 2,217), max X/Y
**0.06008 / 0.03016 pt**. Page one ends with an English heading and empty continuation
space in both engines. Page two repeats English/Reading Comprehension and progresses
through Mathematics/Rounding Numbers. Page three resumes Mathematics at Differentiating
between Numbers, then Sciences and all final signature/stamp/note/date content. Arabic
name and repeated footers match; no missing heading or clipped text was observed.

CR-005-B05 matches all 4,927 painted glyphs (2,113 / 1,548 / 1,266), max X/Y
**0.05894 / 0.03016 pt**. Page two resumes English/Grammar and Punctuation then Mathematics;
both engines retain the same blank remainder before Sciences/Biology begins page three.
All seven science rows, Arabic names/footers, signatures, stamp, bilingual note and date
match. Coordinator and independent reviewer inspected all six page images for each case.

Existing acceptance limits remain **0.061 pt X / 0.031 pt Y**; none was widened. The two
accepted cases contain respectively 92 and 65 Unicode mapping differences for equivalent
painted outlines. Raw extracted-text, native/Rust PDF-byte and whole-page pixel identity
are not claimed. All 11,383 painted glyphs across the accepted pair of cases match.

No renderer change was needed for the accepted cases, and no fix is claimed for CR-001-B05.
Selection preparation now reserves each chosen ID before the next template; three synthetic
regressions protect this and later-batch history/novelty behavior. All **40 focused checks**
pass: nine selection, 24 glyph/comparison/rating audits, and seven actual Chinese PDF tests.
The reused worker hash matches the accepted CR-001-B04 build:
`695e77b2ba433ec29fb9b32894ac82a06618cef14b4a978d95df044f142152af`.
CR-001-B05 was exported from clean `c66e672`; CR-003/005-B05 from clean `d9ddb1b`.
Those checkpoints change tests/documentation only, not renderer source or the accepted worker.

## Private evidence and reproduction

The ignored batch directory contains SELECT inventory/procedure hash, feature rows,
prior coverage, manifests, frozen snapshots, checksums, engine responses, qpdf logs,
page renders, glyph/rating/image/date audits, independent visual records, resource samples,
test logs, `final-summary.json` and a side-by-side `comparison.html`. Finalization verifies
fifteen prior and three new distinct student IDs, clean-source stage success and agreement
between automatic and visual outcomes. Student data/templates/PDFs remain untracked on D:.

Run commands from the fork root; existing manifests contain private selectors. Do not
rerun discovery or create a second run in a completed case directory without explicitly
preparing a fresh wrapper and manifest destination: this fixed-case wrapper has no new-run-path
option and its ambiguity guard executes after export.

```powershell
# Review a retained pair without another SQL capture (CR-001 returns failure as expected).
./research/stc-verification/additional-evaluation-batch-05/verify-case.ps1 -CaseId cr-003-b05

# Historical capture command used for this batch; do not rerun this completed case.
./research/stc-verification/additional-evaluation-batch-05/run-case.ps1 -CaseId cr-005-b05

# Existing D: target and temp mounts, one build/test worker and capped Docker.
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @(
  'test','--locked','--offline','-p','rpt-render','--test','chinese')
```

The Python audit command runs `python3 -m unittest test_select_batch.py test_glyph_audit.py
 test_compare_case.py test_rating_audit.py` from `/src/scripts/stc`, with source read-only,
`PYTHONDONTWRITEBYTECODE=1`, D: scratch mounted at `/tmp`, `--network none`, two CPUs and
512 MiB memory/swap caps. No dependency/image rebuild was required.

## Resources and next repair

Final sample at **17:59:30**: C: **312.10 GiB free**, D: **533.98 GiB free**, RAM
**4.69 GiB available**. Saved pre-export samples were 4.56 / 4.22 / 4.71 GiB available;
all passed the 3-GiB start guard. These are samples, not continuous peak measurements.
Target/cache temp and task scratch remain under `D:\tmpppppp123`. No task containers remain;
only the small registry/NuGet volumes remain, and the retired C-backed target volume is absent.
No unrelated application was stopped. Local commits on existing main only; no branches,
worktrees, push, database/template/runtime/configuration changes or deployment.

Coverage at this batch checkpoint was **17 accepted / 1 rejected / 80 untested** of 98 eligible students.
The next action then was to repair **CR-001-B05 printed page totals and conditional date-footer behavior** against
its retained snapshot, add a renderer-level synthetic regression, regenerate both PDFs and
recheck accepted cases. Root cause was still unproven at that checkpoint. B04's accepted repair is not reopened
by a different captured input. Populated comments, demonstrated displayed-name overflow,
blocked standard/second-term/other-school reports, BIFF/XLS, broader legacy/subreports and
authenticated Parent API/UI delivery retain their earlier gaps.
