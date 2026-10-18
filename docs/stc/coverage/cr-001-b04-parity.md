# CR-001-B04 completed-group footer pagination repair

**Accepted after repair (2026-10-18).** Final native Crystal and Docker Rust PDFs from
clean `85ff5ce` match all 33 ratings and 4,256 painted glyphs within unchanged limits.
All six final pages were reviewed; four prior-case Rust PDFs remain byte-identical.
Coverage at this repair: **15 accepted / 0 rejected / 83 untested**.
See [batch 05](additional-evaluation-batch-05.md) for subsequent coverage and its separate rejection. Earlier checkpoints below
retain the original failure and intermediate validation; final acceptance follows them.

## Scope and retained failure

Repair the [batch 04 rejection](additional-evaluation-batch-04.md) against its retained
frozen dataset. The original three-page pair has 33 ratings allocated 17 / 16 / 0.
Rust wrongly repeats the completed Arabic Language heading and empty column-header
block at page two, shifts the English body down 42.2 pt and displaces the Division
Principal/Homeroom Teacher signatures to page three. All data matched, but layout failed.
No template, source row, acceptance tolerance or production configuration is changed.

Private evidence is under `research/stc-verification/cr-001-b04-repair/` on D:; the
original batch evidence remains unchanged. Scratch is `D:\tmpppppp123\rpt-rs-cr001-b04`;
Cargo target/temp remain under the existing D: Docker cache. No SQL is needed for repair.

## Checkpoint 14a: completed-group repetition lifetime

`Formatter::emit_group` retained its repeating-header entry throughout its footer walk.
A footer that overflows or requests a page break therefore caused `begin_page` to replay
a heading whose detail/child content had already finished. In the retained CR-001 model,
the two repeating category-header sections measure 430 + 414 twips, exactly 42.2 pt.
The actual regenerated pair must verify the relationship before final acceptance.

Retire only this group's repetition entry after both child walks and before its footer
begins. Ancestor groups remain eligible for continuation headings. Keep the completed
group's resolution scope, summaries and last-record context until all footer sections
finish. The prior multipart-header prefix fix remains unchanged. There are no report,
student, category or displacement constants in the renderer change.

Three new synthetic regressions cover:

- Footer overflow, explicit New Page Before and deferred last-detail New Page After,
  with repetition enabled/disabled, ordered rows and exact signature positions. Forced
  variants have room for the footer without the explicit break, isolating each trigger.
- Multipart footers with a suppressed first section and an overflow/forced break between
  visible sections: no stale heading or reserved space from the suppressed band.
- Nested child-footer continuation: repeat the live ancestor, retire the completed child,
  preserve both group-name scopes and the following child's row/header positions.

The initial reproduction failed as intended: repetition produced three pages instead of
two and displaced signatures. After the fix, the first run passed all 377 layout tests;
forced-break variants were then strengthened for independent coverage. Final validation,
native/Rust regeneration and prior-case guards are pending at this checkpoint.
Independent read-only source review found no blocking issue and suggested that test
strengthening. No renderer behavior outside completed-group footer repetition is intended.

## Resource and reproducibility policy

Use sequential Docker Linux builds/exports, two CPUs/2 GiB, one Cargo build job and one
test thread; PDF comparison containers use 512 MiB. Before every export require 3 GiB
available RAM, 30 GiB C: free and 10 GiB D: free. At 17:38:06 RAM was 5.05 GiB available,
C: 312.10 GiB free and D: 534.11 GiB free. These are samples, not continuous peak values.
No unrelated application is stopped. Work remains on existing main with local commits
only; no branches, worktrees, push, database changes or deployment.

```powershell
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('test','--locked','--offline','-p','rpt-layout','--lib')
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('build','--locked','--offline','-p','stc-render-worker')
```

Private reproduction uses `render.ps1 -Tag <unique-tag> -Native`, then
`verify.ps1 -Tag <unique-tag>` in the repair directory. It reuses the captured typed
snapshot and current unchanged template; a tag may not overwrite existing evidence.
Prior-case guards accept `-SourceRun` and `-NativeReference`. Those guards replay Rust
against a retained original PDF and do not claim fresh native or live SQL execution.


## Checkpoint 14b: candidate acceptance and focused validation

The fresh candidate original Crystal and Docker Rust exports verify the inferred cause.
Both have three equal page boxes and all 33 ordered/source ratings allocated 17 / 16 / 0.
All 4,256 painted glyphs match (1,856 / 1,948 / 452 per page), with no missing or extra
outlines. Maximum X/Y residual is 0.05708 / 0.03016 pt, below the existing 0.061 / 0.031
limits. Decoded image RGB and printed dates agree. Both PDFs pass qpdf without warnings.

The coordinator and independent reviewer inspected all six page images: the old Arabic
heading/empty blue column block is gone, the English body aligns, and both Division
Principal and Homeroom Teacher signatures are back on page two. The school principal,
stamp, note and date on page three match, as do the student name and repeated footers.
Thirty-nine known outline-equivalent Unicode mappings remain. PDF bytes, extracted text
and whole-page native/Rust pixels are not claimed equal; tolerances were not widened.
`candidate/review/visual-review.json` records acceptance separately from execution fields.

All 377 final layout tests, seven actual Chinese PDF tests and twelve worker tests pass
in capped Docker: **396 focused checks**, including three new regressions. Source review
also passes; the final clean-source pair and prior accepted-case guards remain pending.
Strict Clippy for layout/worker all targets and workspace formatting also pass.
No dependency, font, template, source-data or bridge changes were required.


## Checkpoint 14c: final clean-source pair and regression replays

Both original Crystal and Docker Rust PDFs were regenerated from clean local `85ff5ce`
using the retained typed snapshot. `final/evidence.json` records source cleanliness,
snapshot/template/worker/native bridge hashes and the local render clock; SQL was not
executed. The native page PNGs exactly reproduce the original batch reference, and the
snapshot hash is unchanged. No baseline, row, report or acceptance tolerance was altered.

| Final comparison gate | Result |
| --- | --- |
| Pages and media boxes | 3 each; equal |
| PDF integrity | Both pass qpdf without repair warnings |
| Ordered/source ratings | 33; 17 / 16 / 0; all accounted for |
| Painted glyphs per page | 1,856 / 1,948 / 452 in both engines |
| Missing/extra painted outlines | 0 / 0 |
| Maximum glyph X / Y residual | 0.05708 / 0.03016 pt |
| Stale category heading/empty columns | Removed from page two |
| Body and Division Principal/Homeroom Teacher signatures | Aligned; signatures on page two |
| School principal/stamp/note/date | Matching page-three placement |
| Decoded image RGB and printed date | Exact equality |
| Visual review | All six final PNGs reviewed; accepted |

The final page PNGs are byte-identical to their same-engine reviewed candidate pages;
independent all-page candidate acceptance therefore remains applicable. The coordinator
also inspected all six final pages. `final/review/visual-review.json` makes acceptance
explicit; initial execution-time pending fields remain historical. The 39 outline-
equivalent Unicode mappings and small raster residuals remain as documented above.

Four accepted cases were then replayed sequentially with their own frozen input and
recorded clock, against their retained original Crystal PDFs. All automated glyph,
rating, image, page/date and PDF checks pass, and the Rust files remain byte-identical
to their previously accepted exports, preserving their earlier visual review:

| Prior-case guard | Pages | Ratings | Painted glyphs | Rust PDF |
| --- | ---: | ---: | ---: | --- |
| CR-001-B03 multipart header/Arabic | 3 | 36 | 4,748 | Byte-identical |
| CR-001-B02 sparse continuation | 2 | 16 | 2,725 | Byte-identical |
| CR-003-B04 boundary/sparse final page | 3 | 34 | 4,788 | Byte-identical |
| CR-005-B04 Arabic name/final signatures | 3 | 35 | 4,943 | Byte-identical |

These are regression replays, not new students. `guard-summary.json` records all four
results. One initial replay was rejected before rendering because PowerShell converted
the JSON timestamp to a DateTime and then a culture-specific string. The private runner
now explicitly formats local ISO time before passing it to the bridge. Its failed attempt
is retained; the retry uses a new tag. The production clock validation was unchanged.
The first private edit also introduced unintended source encoding/formatting changes;
diff review removed them before the final 396-check run and commit. Only the intended
repeat-entry lifetime and three new tests are in the implementation diff.

The candidate retains diagnostic Page IR. Later exports omit that optional debug output,
which is unnecessary for glyph/PDF verification and adds slow writes through the host
mount. Final rendered pixels reproduce the candidate exactly. All request files are
removed after execution; full report data and selectors remain private and ignored.

Final sampled resources at **17:45:21**: C: **312.09 GiB free**, D: **534.01 GiB free**,
RAM **5.40 GiB available**. Lowest observed RAM during this repair was 4.02 GiB. Sequential
caps and D: cache/temp were retained; no task containers remain and the retired C-backed
target volume is absent. These are sampled values, not continuous peak measurements.

Coverage at this repair was **15 accepted / 0 rejected / 83 untested** of 98 eligible students.
The subsequent [batch 05](additional-evaluation-batch-05.md) records the next bounded batch. Populated comments, demonstrated
displayed-name overflow, blocked standard/second-term/other-school reports, BIFF/XLS,
wider legacy/subreport behavior and authenticated Parent API/UI delivery retain their
earlier gaps. No production rendering/configuration or database changes were made.
