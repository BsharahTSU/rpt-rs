# Additional evaluation student batch 03

## Scope and selection (2026-10-18)

Run three new students sequentially through original Windows Crystal and Docker Linux
Rust, using the same captured schema-bearing data. Refreshed SELECT-only inventory
still has 98 ready candidates. Nine previously accepted students are excluded, leaving
89 untested before this batch. Selection chooses the largest untested row count when
available, then Arabic/English source-name lengths and stable student-ID tie-breaking.
CR-005 has no new row counts available, so it uses the longest remaining Arabic source
name. Full source names can be shortened by formulas; this is not overflow coverage.

| Case | Rows | Subjects | English/Arabic source name length | Status |
| --- | ---: | ---: | --- | --- |
| CR-001-B03 | 36 | 15 | 29 / 22 | Rejected: missing repeated heading and name-outline differences |
| CR-003-B03 | 50 | 22 | 36 / 30 | Pending |
| CR-005-B03 | 35 | 12 | 29 / 29 | Pending |

Comments are empty across the eligible candidates. Standard/second-term/other-school
availability and authenticated delivery are not changed by this rehearsal. The evaluation
procedure hash still matches the manually reviewed SELECT-only definition; no mutating
standard-report procedures are called. A preparatory scan encountered an incomplete old
run with no capture status under PowerShell strict mode; the exclusion scan was corrected
to skip incomplete runs before selecting/exporting. All nine prior IDs remain excluded.

Private inputs/evidence live under ignored
`research/stc-verification/additional-evaluation-batch-03/` on D:. Temporary storage is
`D:\tmpppppp123\rpt-rs-additional-batch-03`; target/temp cache mounts remain on D:.
Exports use two CPUs and 2 GiB, comparison containers 512 MiB. Check available RAM
before each export, pausing below 3 GiB. Initial C: free 313.35 GiB, D: 535.82 GiB,
RAM 5.88 GiB available; no containers were already running.

## First case checkpoint: confirmed rejection

CR-001-B03 exported from clean `dd1c81e`. Capture, native-live, native-frozen and
rust-frozen passed; all three PDFs pass qpdf, have three pages and equal boxes/dates.
Native live/frozen raster pages are identical. All 36 source aliases are `O` and match
in the same 15 / 16 / 5 page allocation; decoded image RGB also matches exactly.

Visual review of all six page images confirms that both engines print the English
Language heading at the bottom of page one. Native repeats that heading at the top
of page two; Rust omits it and starts the following table higher. The painted Arabic
student-name outlines also differ in the first-page name field and repeated footers.
The geometry audit fails and the case is explicitly rejected. These differences are
not covered by the accepted small-rounding tolerances or Unicode mapping exceptions.
No root cause or renderer repair is claimed at this checkpoint. Preserve this snapshot
and both engine PDFs for a focused continuation-header/name-shaping repair.

Two new synthetic audit regressions ensure that a boundary heading cannot replace a
required repeat on the next page and identical Arabic text cannot hide a different
painted outline. They verify rejection, not a renderer repair. All 24 Python audit
checks pass in capped Docker (16 glyph, four comparison, four rating); Chinese PDF
checks and the next two cases remain pending. Existing acceptance is not widened.
