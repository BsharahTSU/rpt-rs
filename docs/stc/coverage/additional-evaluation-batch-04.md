# Additional evaluation student batch 04

**Subsequent repair accepted (2026-10-18).** The [CR-001-B04 repair](cr-001-b04-parity.md)
closes the stale-heading/body/signature rejection with fresh paired PDFs from `85ff5ce`.
All 4,256 painted glyphs and 33 ratings match within unchanged limits; all six pages were
reviewed. CR-003-B04 and CR-005-B04 Rust replays remain byte-identical. Coverage at that
repair: **15 accepted / 0 rejected / 83 untested**. See [batch 05](additional-evaluation-batch-05.md)
for subsequent coverage and its separate rejection. The sections below preserve this
batch's original two passes and one rejection; the repair supersedes that case's status.

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

## Checkpoint 13b: first case rejected for a new boundary layout difference

CR-001-B04 exported from clean `723921d`; capture, native-live, native-frozen and
rust-frozen all passed. All three PDFs pass qpdf, have three pages and equal boxes/dates.
Native live/frozen raster pages are identical. All 33 source ratings match in the same
17 / 16 / 0 page allocation, and decoded image RGB agrees exactly.

All six original-frozen/Rust page images were reviewed by the coordinator and an
independent reviewer. Page one, including the Arabic student name, agrees. On page two,
Rust inserts the previous Arabic Language group heading and an empty blue column-header
block before the English Language section; Crystal starts directly with English Language
below the carry-over horizontal line. The English body is **42.2 pt lower** in Rust
(raw PDF Y delta -42.20004 to -42.19999 pt for all sixteen rating baselines).
This pushes Division Principal/Homeroom Teacher signatures from native page two onto
Rust page three and moves the final principal/stamp/note area downward on page three.

The case is explicitly **rejected** despite matching rating data and page count.
Painting-glyph inventories are 1,856/1,856 on page one, 1,948/1,943 on page two and
452/545 on page three. Page one's geometry passes; pages two/three fail inventory and
position gates. Small residuals among a subset of matched glyphs do not imply acceptance.
No new Arabic-name outline defect is visible. The previous CR-001-B03 repair remains
accepted for its own captured inputs; this new boundary is separate evidence.

The precise root cause and renderer repair have not been established. Preserve the
frozen snapshot and failed pair for a focused completed-group/footer-transition
investigation; do not change data/templates/tolerances to pass. The comparison runner
exits nonzero as intended and private `review/visual-review.json` records the rejection.
The other two students remain pending. Resource sample after export: RAM 4.91 GiB,
C: 312.74 GiB and D: 534.40 GiB free. No renderer changes in this checkpoint.

## Batch results and review before the subsequent repair

| Case | Pages each | Ratings per page | Native/Rust painted glyphs | Result | Export source |
| --- | ---: | --- | --- | --- | --- |
| CR-001-B04 | 3 | 17 / 16 / 0 | 4,256 / 4,344 | Rejected: stale heading/body/signature placement | clean `723921d` |
| CR-003-B04 | 3 | 16 / 18 / 0 | 4,788 / 4,788 | Accepted; max X/Y 0.06008 / 0.03016 pt | clean `ddb5bdb` |
| CR-005-B04 | 3 | 16 / 12 / 7 | 4,943 / 4,943 | Accepted; max X/Y 0.05894 / 0.03016 pt | clean `ddb5bdb` |

All capture/native-live/native-frozen/rust-frozen stages pass for all three cases.
All nine execution PDFs pass qpdf without repair warnings. Native live/frozen raster
pages are identical, page boxes/dates agree, all **102 ratings** match in engine order
and against captured-source multiplicity, and every decoded image RGB value matches.
All **18** original-frozen/Rust page images were reviewed by the coordinator and an
independent reviewer. The failed layout remains rejected despite those data successes.

The two accepted cases account for 69 ratings and 9,731 painted glyphs, all matched
within existing 0.061 pt X / 0.031 pt Y limits. CR-003's pending Mathematics subsection
repeats correctly at the page boundary, while its school-principal/stamp/note content
continues onto a sparse third page with zero rating rows. CR-005 has an actual Arabic
name in the first-page field and repeated footers; its English continuation, Mathematics,
Science, final signatures/stamp/note/date all agree. Neither adds demonstrated displayed-
name overflow or populated-comment coverage. Known outline-equivalent Unicode mapping
differences remain (70 for CR-003 and 63 for CR-005); native/Rust PDF bytes, extracted
text and whole-page pixels are not claimed equal. Acceptance tolerances were not widened.

The worker artifact hash matches the final accepted CR-001-B03 build from `8f9fd5e`;
this batch changes selection safeguards/tests/docs only. The newly observed CR-001-B04
failure is not declared a regression or repaired merely by associating it with an earlier
commit. Its root cause needs the retained snapshot and an isolated renderer reproduction.
Earlier accepted cases, including CR-001-B03, retain their own recorded evidence.

## Tests and reproducibility

Six new selection tests plus 24 existing glyph/comparison/rating checks pass in capped
Docker: **30 Python checks**. All **seven actual Chinese PDF checks** also pass under the
current compatibility profile, for **37 focused checks** total. Test success is separate
from per-student visual parity; it does not override CR-001-B04's rejection.

```text
# In the development container, working directory /src/scripts/stc:
python3 -m unittest test_select_batch.py test_glyph_audit.py test_compare_case.py test_rating_audit.py
```

```powershell
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('test','--locked','--offline','-p','rpt-render','--test','chinese')
```

Python checks use read-only source, D: scratch, network disabled, two CPUs/512 MiB and
`PYTHONDONTWRITEBYTECODE=1`. Chinese uses the standard D: cache launcher, one Cargo job
and one test thread, two CPUs/2 GiB. No new packages or container images were downloaded.

Private `prepare.py` normalizes the refreshed inventory, checks feature/candidate row
counts and excludes twelve unique captured students before creating three distinct new
manifests. `run-case.ps1` requires clean source, available RAM/disk and the known worker
hash; it runs one case and verifies it before the next export. Capture checks the reviewed
procedure hash again. `verify-case.ps1` correctly exits nonzero for the first case.
No credentials, student names/IDs, RPTs or report PDFs are committed.

`final-summary.json` records all three statuses and verifies fifteen distinct students.
Each run has explicit `review/visual-review.json`; original execution-time pending fields
are historical. `comparison.html` links all six comparison PDFs and per-page galleries,
clearly marks the rejected pair, and retains original source hashes/evidence. Native live
exports are additional reproducibility evidence, not extra students.

## Resources, current coverage and next repair

Final samples at **17:22:29**: C: **312.72 GiB free**, D: **534.36 GiB free**, RAM
**4.16 GiB available**. Initial 2.86 GiB availability held exports until memory recovered;
every export passed the 3 GiB start guard. These are sampled checks, not continuous peaks.
No task containers remain and the retired C-backed target volume is absent. Cache and
temporary output remain on D:. No unrelated application was stopped or reconfigured.

Coverage at this batch checkpoint was **14 accepted / 1 rejected / 83 untested**, with fifteen distinct
students compared. Prioritize the retained **CR-001-B04 stale group-heading/footer-
transition defect** next: add a renderer-level synthetic reproduction, fix it, regenerate
both frozen PDFs and guard prior accepted boundary/signature cases. No such renderer fix
was attempted in this bounded batch. Standard/second-term/other-school, BIFF/XLS, wider
legacy/subreport and authenticated Parent API/UI delivery gaps remain.

SELECT-only discovery and reviewed read-only evaluation calls only; no database,
publication/access, template or Parent configuration changes. Local commits only on main,
no branches/worktrees, push or deployment. Unrelated backend configuration edits preserved.
