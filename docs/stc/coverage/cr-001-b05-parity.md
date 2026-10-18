# CR-001-B05 reset-scoped page totals and date footers

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

- Unequal 2/1/3/1-page numbering sequences, consecutive resets, no-reset totals, ordered
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
