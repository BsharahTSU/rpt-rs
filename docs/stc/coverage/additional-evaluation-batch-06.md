# Additional evaluation student batch 06

**Completed (2026-10-19): all three new students accepted.** Original Crystal and Docker
Rust PDFs from clean `4607ed4` match all 113 ratings and 15,033 painted glyphs within
unchanged limits. All sixteen final page images were reviewed by the coordinator and an
independent reviewer. All 44 focused checks pass. Current coverage: **21 accepted /
0 rejected / 77 untested**. No renderer repair was necessary. Earlier checkpoint pending
statements below preserve the state at that time.

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

## Checkpoint 17b: sequential exports and acceptance

| Case | Native / Rust pages | Ratings per page | Painted glyphs per engine | Max X / Y difference (pt) | Result |
| --- | --- | --- | ---: | --- | --- |
| CR-001-B06 | 2 / 2 | 14 / 12 | 3,640 | 0.05708 / 0.03016 | Accepted |
| CR-003-B06 | 3 / 3 | 18 / 21 / 13 | 6,449 | 0.06008 / 0.03016 | Accepted |
| CR-005-B06 | 3 / 3 | 16 / 12 / 7 | 4,944 | 0.05894 / 0.03016 | Accepted |

Every capture/native-live/native-frozen/rust-frozen stage passes from clean `4607ed4`.
All nine PDFs pass qpdf. Native live/frozen page PNGs are identical; frozen snapshots
establish the comparison input. All 113 ratings match source multiplicity and engine order.
Decoded image RGB, dates and page boxes agree in each case. Printed labels are complete
and equal: 1/2, 2/2 for CR-001; 1/3, 2/3, 3/3 for CR-003/005. No tolerance was widened.

The coordinator and independent reviewer examined all sixteen final page PNGs, recording
explicit acceptance for each case before the next export:

- CR-001-B06 ends page one after the first English Oral row; page two repeats English/Oral,
  continues the remaining subsections and retains complete signatures, principal/stamp,
  bilingual note and date. Both page labels and Arabic names match. No body shift observed.
- CR-003-B06 retains the same English heading/empty continuation area at page one's end.
  Page two resumes Reading Comprehension and reaches Mathematics/Rounding Numbers. Page
  three resumes Differentiating between Numbers, then Sciences and complete final content.
- CR-005-B06 resumes English/Grammar and Punctuation on page two, then Mathematics. The same
  blank remainder precedes Sciences/Biology on page three. All seven science rows and final
  signatures/stamp/note/date match. Logo RGB is exact and Arabic names/footers agree.

No missing headings, dates or clipping were found. Glyph inventory/shape/size/position
checks cover all 15,033 painted glyphs. The three cases retain respectively 37 / 92 / 63
Unicode mapping differences for equivalent outlines. Cross-engine extracted-text, PDF-byte
and whole-page pixel identity are not claimed; acceptance uses existing 0.061/0.031-pt limits.

## Tests, provenance and evidence

All **44 focused checks** pass: nine selection tests, 28 glyph/comparison/rating checks
(including four new page-label tests), and seven actual Chinese PDF regressions. The latter
cover embedding/extraction, font coverage/fallback, spaces, LRM and Chinese wrapping. The
new label gate rejects the retained B05 failure and accepts its repaired sequence. Independent
read-only review of the parser, tests, acceptance wiring and pre-capture rerun guard found
no blocking issue. The English caption-format limitation remains explicit.

No renderer change or rebuild was needed. The reused B05-repaired worker SHA256 is
`f09c89325eea0e8de90b68c1b78e9c51da113ee23d8900c72900c263d6019206`.
The source checkpoint changes comparison tests/audits and documentation only. Previous
400-check renderer validation and five byte-identical replays remain the dated B05 repair
evidence; those unchanged broad checks were not repeated for this bounded batch.

Ignored `research/stc-verification/additional-evaluation-batch-06/` retains SELECT inventory,
procedure hash, feature/prior coverage rows, case manifests, frozen snapshots, PDF/checksums,
qpdf/PNG/glyph/rating/image/date/page-label evidence, visual-review records, test/resource logs,
`final-summary.json` and the side-by-side `comparison.html`. Finalization verifies eighteen
prior plus three new distinct students, clean-source stage success, delivered PDF hashes,
all expected PNGs and agreement of automatic and two-reviewer visual acceptance. It never
publishes student IDs/names or private reports to Git.

Run from the fork root, using the existing private manifests:

```powershell
# Recheck a retained pair without another database capture.
./research/stc-verification/additional-evaluation-batch-06/verify-case.ps1 -CaseId cr-001-b06

# Historical export; the completed case is now rejected BEFORE recapture by the wrapper.
./research/stc-verification/additional-evaluation-batch-06/run-case.ps1 -CaseId cr-005-b06

./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @(
  'test','--locked','--offline','-p','rpt-render','--test','chinese')
```

Python tests run `python3 -m unittest test_select_batch.py test_glyph_audit.py
 test_compare_case.py test_rating_audit.py` in `/src/scripts/stc`, with source read-only,
`PYTHONDONTWRITEBYTECODE=1`, D: scratch `/tmp`, no network, two CPUs and 512 MiB memory/swap.
No dependency/image rebuild was needed. To test new inputs, prepare a separate opaque case
and run directory, rather than bypassing the completed-case guard or overwriting evidence.

## Resources and remaining coverage

Final sample at **2026-10-19T09:22:11**: **C: 311.44 GiB free, D: 533.81 GiB free, RAM 3.08 GiB available**. Pre-export available RAM was
3.38 / 4.14 / 4.40 GiB, each above the 3-GiB start guard. Available RAM briefly fell to
**1.55 GiB** during the first pair as host memory pressure changed, recovering to 3.99 GiB
before moving on. The guard is a start check, not a continuous host memory limit. The Chinese
test start was held at 2.78 GiB; it ran after recovery to 3.23 GiB. These are sampled values,
not a continuously measured peak. No unrelated process was stopped and no limit was weakened.

D: target/cache temp and task scratch were retained. No containers remain and the retired
`stc-rpt-rs-target` volume is absent. No SQL/template/production runtime changes were made;
local commits on existing main only, no branches/worktrees/push or deployment. An unrelated
backend appsettings edit was left untouched and excluded from this task's documentation commit.

Coverage is now **21 accepted / 0 rejected / 77 untested** of 98 SQL-ready students. Next
is another bounded batch with the same guards and per-case review. Populated comments,
demonstrated displayed-name overflow, blocked standard/second-term/other-school reports,
BIFF/XLS, broader legacy/subreports and authenticated Parent API/UI delivery retain their
previous gaps. These three additional students establish no new template or API coverage.
