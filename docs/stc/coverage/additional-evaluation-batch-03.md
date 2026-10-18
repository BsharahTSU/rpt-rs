# Additional evaluation student batch 03

## Follow-up: all three now accepted after CR-001-B03 repair

The subsequent [CR-001-B03 repair](cr-001-b03-parity.md) fixes both causes and regenerates
the original/Rust pair from clean `8f9fd5e`. All 4,748 painted glyphs and 36 ratings agree
within unchanged limits, and all six final pages were reviewed. CR-003-B03 and CR-005-B03
Rust replays remain byte-identical. Cumulative coverage at that repair checkpoint is twelve accepted,
none rejected, 86 untested. The sections below preserve this batch's original rejection
and then-open findings; the linked repair supersedes that case's final status.

## Original batch review: two accepted, one rejected

All three new students printed in both engines. CR-003-B03 and CR-005-B03 are accepted
within existing layout limits; CR-001-B03 is rejected for a missing repeated heading,
a 21.5 pt page-two body displacement and Arabic name-outline differences. No renderer
repair is claimed. The failed frozen pair is the next repair target. Twelve distinct
students have now been compared: eleven accepted, one rejected, and 86 eligible
students remain untested. Passing audit/Chinese tests do not override the rejected PDF.

## Scope and selection (2026-10-18)

Run three new students sequentially through original Windows Crystal and Docker Linux
Rust, using the same captured schema-bearing data. Refreshed SELECT-only inventory
still has 98 ready candidates. Nine previously accepted students are excluded, leaving
89 untested before this batch. Selection chooses the largest untested row count when
available, then Arabic/English source-name lengths and stable student-ID tie-breaking.
CR-005 has no new row counts available, so it uses the longest remaining Arabic-name source
name. Full source names can be shortened by formulas; this is not overflow coverage.

| Case | Rows | Subjects | English/Arabic source name length | Status |
| --- | ---: | ---: | --- | --- |
| CR-001-B03 | 36 | 15 | 29 / 22 | Rejected: missing repeated heading and name-outline differences |
| CR-003-B03 | 50 | 22 | 36 / 30 | Accepted within existing tolerances |
| CR-005-B03 | 35 | 12 | 29 / 29 | Accepted; Arabic-name field contains Latin text |

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
RAM 5.84 GiB available; no containers were already running.

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
checks and the next two cases were pending at this first checkpoint. Existing acceptance is not widened.

## Final case results

| Case | Pages each | Ordered ratings | Native/Rust painting glyphs | Geometry and visual outcome | Export source |
| --- | ---: | --- | --- | --- | --- |
| CR-001-B03 | 3 | 15 / 16 / 5 | 4,748 / 4,728 | Rejected | clean `dd1c81e` |
| CR-003-B03 | 3 | 17 / 20 / 13 | 6,474 / 6,474 | Accepted, max X/Y 0.06008 / 0.03016 pt | clean `3712252` |
| CR-005-B03 | 3 | 16 / 12 / 7 | 4,960 / 4,960 | Accepted, max X/Y 0.05894 / 0.03016 pt | clean `3712252` |

All four execution stages pass for each case. All nine PDFs pass qpdf, page boxes/dates
match and native-live/frozen raster pages are identical. All 121 captured rating aliases
match in order between engines and in multiplicity against source data. Every image's
decoded RGB matches. All 18 original-frozen/Rust page images were inspected separately.
These data/integrity successes do not imply visual acceptance of CR-001-B03.

The two accepted cases match all 85 ratings and 11,434 painting glyphs within the existing
0.061 pt X / 0.031 pt Y limits. CR-003 includes all five letter-rating values and verifies
English/Math continuation headings. CR-005's Arabic-name source field contains Latin
letters, confirmed in its snapshot and both page-one/right-field and footer output.
It therefore adds Latin-in-that-field coverage rather than evidence for a long Arabic
name. Source names can be shortened by formulas; no displayed-name overflow is claimed.
Known outline-equivalent Unicode mappings remain different (94 in CR-003, 63 in CR-005).
PDF bytes, extracted text and whole-page native/Rust pixel identity are not claimed.

## Open repair evidence and next step

CR-001-B03's first English group heading is visible at the bottom of page one in both
outputs. The bilingual repeated group heading is present only in native page two.
All sixteen page-two rating baselines in Rust are higher by 21.49998-21.50001 pt,
measured from actual PDF text coordinates. This is a real displacement, not the known
font-descriptor bbox difference. Arabic student-name outline mismatches also occur on
page one and the repeated footers; they need a separate shaping/font investigation.
Small displacement statistics for only matched glyphs must not be interpreted as an
acceptance envelope when the page inventory and unmatched-glyph gates fail.

Use the retained same-data snapshot to diagnose and repair both issues, add renderer-level
synthetic reproductions, regenerate the native/Rust pair, and recheck previously accepted
continuation/name cases. Do not change source data, templates or acceptance tolerances
to make this case pass. Root causes have not been established in this bounded test batch.
The earlier accepted CR-001 representatives remain historical evidence for their inputs;
they do not establish this new student's parity. No renderer fix was attempted or claimed.

## Tests, reproduction and resource checks

**31 focused checks pass:** 16 glyph-audit, four comparison, four rating-audit and seven
actual Chinese PDF embedding/extraction tests. Two synthetic rejection tests were added
in `3712252`. They protect against false acceptance of missing repeated headings and
different painted Arabic outlines with equal extracted labels/counts. They do not prove
that the renderer handles the failing student. Renderer sources remain unchanged.

Run the Python checks from `scripts/stc` in the existing Docker development image:

```text
python3 -m unittest test_glyph_audit.py test_compare_case.py test_rating_audit.py
```

Use read-only source, D: temporary storage, network disabled, two CPUs/512 MiB and
`PYTHONDONTWRITEBYTECODE=1`. Chinese PDF tests use the D: cache and existing capped launcher:

```powershell
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('test','--locked','--offline','-p','rpt-render','--test','chinese')
```

The batch's private `verify-case.ps1` deliberately exits nonzero for the rejected first
case. Its PDF, qpdf, raster, glyph and data evidence remains intact. Each case has an
explicit `review/visual-review.json`; original execution-time `visual: pending` fields
remain historical. `final-summary.json` distinguishes accepted/rejected from merely
printed. `comparison.html` links all six final comparison PDFs and page galleries and
marks the failed case. `review/heading-offset.json` records the measured displacement.
All manifests were verified to represent twelve distinct students across the four groups.

Final sampled resources at 16:36:36: C: **313.34 GiB free**, D: **535.79 GiB free**,
RAM **6.01 GiB available**. The recorded RAM minimum was 5.84 GiB, above the 3 GiB
start guard; sampled checks are not continuous peak measurements. No containers remain,
and the retired C-backed target volume is absent. D: cache/temp and capped sequential
Docker remain in use. No database, publication/access, template, Parent runtime or
production configuration changes; local commits only, no branch/worktree/push/deployment.

Prioritize the rejected CR-001-B03 repair before a broader acceptance claim. The 86
untested eligible students, populated comments, overflowing displayed names, blocked
standard/second-term/other-school cases, Excel, wider legacy/subreport compatibility and
authenticated Parent API/UI delivery retain their documented gaps.
