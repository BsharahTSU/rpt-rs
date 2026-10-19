# CR-001-B05 reset-scoped page totals and date footers

**Accepted after repair (2026-10-18).** Fresh original Crystal and Docker Rust PDFs from
clean `0e6476a` match all 28 ratings and 3,962 painted glyphs within unchanged limits.
Page totals/date footers are repaired; all six final pages were reviewed. Five prior
accepted Rust PDFs remain byte-identical. Coverage at that repair checkpoint: **18 accepted / 0 rejected /
80 untested**. Earlier checkpoints below retain their original pending status.

## Checkpoint 16a: cause, fix and synthetic validation (2026-10-18)

Repair the retained [batch 05 rejection](additional-evaluation-batch-05.md) against its
frozen typed snapshot. Original Crystal prints three physical pages numbered 1 of 2,
2 of 2 and 1 of 1. Rust instead printed 1 of 3, 2 of 3 and 1 of 3, omitted the Print Date
blocks on pages two/three and raised following legends/page footers by 16 pt. Body,
headings, Arabic names and signatures already matched; all 28 ratings were retained.

The formatter honored ResetPageNumberAfter for the displayed page number, but both
`emit_final_footers` and `resolve_page_totals` used the whole-document physical count.
The retained template's 320-twip date section is suppressed by
`PageNumber < TotalPageCount`. Incorrect totals therefore hide that leading section on
the last page of each numbering sequence and move the following footer stack upward.
The reset is deferred until a real page opens; it need not coincide with an explicit
New Page After. The correct per-physical-page totals for this case are 2 / 2 / 1.

`Formatter::page_sequence_totals` now partitions finalized physical pages at actual
reset-to-one boundaries. Both deferred footer visibility and earlier placed/embedded
page-total fixups use the same vector indexed by physical page. Consecutive single-page
sequences work; an unused pending trailing reset does not create a new segment or page.
No footer-coordinate correction or case/student/template-specific value is introduced.
Body execution, footer origin, deferred reset timing and the B04 completed-group header
retirement remain intact. This does not implement general reevaluation of arbitrary body
formulas containing forward page totals; the scope is the existing special-field fixups
and final-total-dependent page footers.

The new tests also reproduced a deferred embedded-value defect: a footer already planned
with its final total was registered again with a provisional physical-page value. Later
replacement could alter a matching literal, e.g. `literal 1.00; total 7.00` became
`literal 7.00; total 7.00`. Discard only page-count fixups appended while emitting the
already-final footers, preserving every earlier header/body fixup and currency fixups.

Four synthetic regressions cover:

- Unequal 2/1/3/1-page numbering sequences, consecutive resets, no-reset totals,
  row allocation and an unused trailing reset. Placed and embedded captions/totals agree.
- Conditional date sections before and after other footer content, with exact date/footer
  positions, unchanged row positions and literal-safe embedded totals, with/without resets.
- A reset flag with no explicit page break: the next group shares the current physical
  page until natural overflow consumes the reset. Both actual pages are numbered one.
- A footer-only final page: labels 1 of 2, 2 of 2, 1 of 1; dates on the last two pages,
  no lost body rows, no blank-page creation and exact final-section placement.

All four fail before the fix and pass afterward. The initial test fixture assumed integer
formatting for an embedded total; its expected text was corrected to the existing default
two-decimal format before recording the meaningful red run. All **400 focused Docker
checks** pass: 381 layout, seven actual Chinese PDF and twelve worker tests. Independent
read-only source review found no blocking issue. Fresh native/Rust PDF verification and
clean-source acceptance remain pending at this checkpoint.

## Evidence, resources and reproduction

Private inputs, snapshots, PDFs, raw logs, manifests and selectors stay in ignored
`research/stc-verification/cr-001-b05-repair/`. Retain the original batch evidence unchanged.
Scratch is `D:\tmpppppp123\rpt-rs-cr001-b05`; Cargo target and container temp use the existing
D: Docker bind mounts. No SQL is needed. Sequential builds/exports use two CPUs/2 GiB,
one Cargo job and one test thread; comparisons use 512 MiB. Before exports require at
least 3 GiB available RAM, 30 GiB C: free and 10 GiB D: free. At the saved build-start
sample RAM was 5.15 GiB available, C: 311.83 GiB free and D: 533.96 GiB free. These are
samples, not continuous peak measurements. No unrelated application is stopped.

```powershell
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('test','--locked','--offline','-p','rpt-layout','--lib')
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('test','--locked','--offline','-p','rpt-render','--test','chinese')
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('test','--locked','--offline','-p','stc-render-worker')
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('build','--locked','--offline','-p','stc-render-worker')
```

The private repair wrapper uses `render.ps1 -Tag <unique-tag> -Native`, followed by
`verify.ps1 -Tag <unique-tag>`. A tag cannot overwrite retained evidence. It reuses the
captured dataset and unchanged template; the native bridge runs on Windows and Rust in
Docker. Prior-case guards use `-SourceRun` and `-NativeReference`, replaying Rust against
retained original PDFs at their recorded local clocks; those do not claim fresh native
exports or SQL execution. Final review must cover every page and retain the existing
0.061 pt X / 0.031 pt Y limits. No template/data/font/dependency/production configuration
change, branch, worktree, push or deployment; local commits on existing main only.

## Checkpoint 16b: fresh candidate comparison accepted

Fresh original Crystal and repaired Docker Rust exports both pass qpdf and have three
identical page boxes. All **3,962 painted glyphs** match (1,789 / 1,944 / 229), maximum
X/Y **0.05708 / 0.03016 pt**, within unchanged limits. All 28 ordered/source ratings match,
allocated 15 / 13 / 0. Decoded image RGB and per-page printed dates agree exactly.

Coordinator and independent all-six-page review confirm labels **1 of 2, 2 of 2, 1 of 1**,
date blocks restored on physical pages two/three and aligned legends/page footers. Body,
headings, Arabic names, logo, signatures, stamp and note remain aligned. Physical page
three remains the same sparse footer-only page as Crystal. No template or data change
is needed. Thirty-nine equivalent-outline Unicode mappings remain; native/Rust PDF-byte,
extracted-text and whole-page pixel identity are not claimed.

Strict Clippy for affected layout/worker all targets and workspace formatting also pass.
The candidate intentionally records dirty source; final clean-source regeneration and
five prior accepted-case replays are still required before final acceptance.


## Checkpoint 16c: final clean-source acceptance and prior-case guards

Both original Crystal and Docker Rust PDFs were regenerated from clean local commit
`0e6476a`. Native bridge, template and snapshot hashes match the retained batch; no SQL
was executed. Final worker SHA-256 is
`f09c89325eea0e8de90b68c1b78e9c51da113ee23d8900c72900c263d6019206`.
All three native page rasters reproduce the original batch exactly. All six final page
rasters reproduce their independently reviewed same-engine candidate pages, and the
coordinator also inspected every final page directly. Private `visual-review.json` records
acceptance and `final-summary.json` verifies source/input provenance and all guard results.

| Final paired check | Result |
| --- | --- |
| PDF integrity and physical pages | Both qpdf checks pass; three equal page boxes |
| Printed page labels | 1 of 2, 2 of 2, 1 of 1 in both engines |
| Print Date block | Present and aligned on physical pages two and three |
| Legends and page footers | Aligned; former 16 pt displacement closed |
| Ordered/source ratings | 28, allocated 15 / 13 / 0 |
| Painted glyphs | 3,962 / 3,962, allocated 1,789 / 1,944 / 229 |
| Maximum X / Y residual | 0.05708 / 0.03016 pt under existing limits |
| Decoded image RGB and dates | Equal |
| Body, names, headings, signatures/stamp/note | Match; sparse footer-only page retained |

Five sequential frozen-input Rust replays pass every audit and preserve each accepted
Rust PDF byte-for-byte. They reuse the retained native PDFs and recorded local clocks;
these guards add no students and do not claim freshly regenerated native exports.

| Prior accepted case | Ratings | Protection |
| --- | ---: | --- |
| CR-001-B04 | 33 | Completed-group footer transition, body/signatures |
| CR-001-B03 | 36 | Repeated heading and Arabic ligature repair |
| CR-001-B02 | 16 | Sparse two-page report |
| CR-003-B05 | 52 | Category/subsection continuations and final content |
| CR-005-B05 | 35 | Blank remainder/Science continuation and final content |

Final evidence is `research/stc-verification/cr-001-b05-repair/final/`, including
`native.pdf`, `rust.pdf`, six page PNGs, qpdf logs, comparison HTML, audits and provenance.
The original rejected batch stays unchanged as a historical reproduction. Its acceptance
status is superseded by this record. The retained 39 equivalent-outline Unicode mapping
differences and small raster residuals do not establish raw extracted-text, PDF-byte or
whole-page native/Rust pixel identity. No tolerances were widened.

Final resources at **18:16:23**: C: **311.40 GiB free**, D: **533.84 GiB free**, RAM
**5.96 GiB available**. Lowest observed RAM during validation was approximately 3.66 GiB;
all exports passed their start guard. These are samples, not continuous peak measurements.
D: cache/temp and sequential container caps were retained; no task containers remain and
the retired C-backed target volume is absent. No unrelated application was stopped.

Coverage at that repair: **18 accepted / 0 rejected / 80 untested** of 98 eligible students.
[Batch 06](additional-evaluation-batch-06.md) subsequently raises acceptance to 21, with 77 untested.
At that checkpoint another bounded batch was next. Populated comments, demonstrated displayed-name
overflow, blocked standard/second-term/other-school reports, BIFF/XLS, wider legacy/subreport
behavior and authenticated Parent API/UI delivery remain open. No production rendering,
configuration, database or template changes; local commits only, no branches/worktrees/push.
