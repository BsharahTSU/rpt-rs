# CR-001-B04 completed-group footer pagination repair

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
