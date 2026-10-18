# Initial eligible evaluation PDF comparisons

Verified 2026-10-16 with the unchanged Windows Crystal bridge and rebuilt Docker Linux Rust worker.
These are three representative students from the 98 SQL-ready candidates in the [inventory](sql-eligibility.md).
They establish actual PDF generation for these representatives, not all students or authenticated
Parent API/UI delivery. At that checkpoint, the remaining 95 candidates had eligibility evidence only.

Current status: [CR-001](cr-001-parity.md) and [CR-003](cr-003-parity.md) have since
completed their individual repairs and fresh paired acceptance. [CR-005](cr-005-parity.md) now passes text/layout checks under those shared fixes.
The subsequent [logo-color repair](cr-005-logo.md) closes the shared image residual:
all 9,600 decoded logo RGB pixels match in fresh CR-005 and CR-001/003 replays.
Whole-page pixel identity is still not claimed; documented layout tolerances remain.
After [batch 01](additional-evaluation-batch-01.md), [batch 02](additional-evaluation-batch-02.md) and [batch 03](additional-evaluation-batch-03.md), eleven students are accepted, one is rejected for heading/name differences, and 86 eligible candidates remain untested.
The table and observations below retain the original October 16 results.

## Results

| Case | Template | Native live | Native snapshot | Rust snapshot | Pages (each) | Ordered ratings per page | Visual parity |
| --- | --- | --- | --- | --- | ---: | --- | --- |
| CR-001 | aaj/rptStudentMark01.rpt | Pass | Pass | Pass | 3 | 17 / 17 / 0 (34 total), exact match | Open: page-boundary heading and metrics |
| CR-003 | aaj/rptStudentMark02.rpt | Pass | Pass | Pass | 3 | 18 / 21 / 13 (52 total), exact match | Open: wrapping and metrics |
| CR-005 | aaj/rptStudentMark03.rpt | Pass | Pass | Pass | 3 | 16 / 12 / 7 (35 total), exact match | Open: small text/row metrics |

All nine PDFs pass qpdf without errors/repair warnings. Each has 595 x 841 point page boxes.
Native-live and native-snapshot ordered text match, and all nine corresponding rendered PNG pairs
have identical SHA-256 hashes at 96 DPI. This verifies that replay reproduced the observed native
outputs for these cases. The Rust engine consumed the same normalized schema-bearing XML as the
native snapshot run, with Docker networking disabled.

A second agent independently verified the rating sequences and native image equality. Ratings were
extracted from the central column at x=300..320 points, y<760 points, excluding the footer legend.
All nine native/Rust page pairs were visually reviewed. Dates agree and appear in the expected
final-page footer. Logos, signatures, legends and report values are present. Rust emitted no stderr
warnings for these mounted-font runs. Matching values/page counts do not establish visual parity.

## Historical differences and follow-up

CR-001 follow-up: [printer metrics investigation and corrected replay](cr-001-metrics.md).
The missing boundary heading and cumulative row drift are repaired there. The original bbox-based
baseline estimate below includes a font-descriptor artifact; actual first baselines already matched.
Retain these original observations as history, not the current acceptance result.

1. **CR-001, page 1:** native prints an orphan `Songs and Recitals` subgroup heading at y=751.041pt;
   Rust defers it until page 2. The native label rectangle contains 348 dark pixels versus zero
   in Rust. Investigate group-header keep-together/page-boundary behavior with a synthetic fixture
   before changing it; preserve this native behavior for compatibility. Review CR-001 first.
2. **CR-003, page 2:** the writing sentence wraps after `people,` in native; Rust includes `places`
   on that first line, moving only `and objects` to the next line. This is confirmed by bbox lines.
   Investigate measured width/word-fit alongside the inherited nominal/font-cell metric policy.
3. **All three:** Rust rating baselines are about 1.58-3.48pt lower in the sampled rows, with small
   cumulative row/border placement changes. Arabic rich-text and Latin glyph positioning differ.
   CR-005 preserves the established page allocation, including Science beginning page 3, but
   remains close visual parity rather than pixel identity.

A preliminary visual suspicion about missing continuation/subgroup labels was rechecked against
both bbox text and raster pixels and withdrawn. Arabic Language, Listening and Life Science labels
are present. Do not turn that discarded suspicion into a renderer fix or regression expectation.

## Reproduction and private evidence

Use the [case runner](case-runner.md). Current private runs are beneath
`research/stc-verification/parent-cases/`:

| Case | Run directory |
| --- | --- |
| CR-001 | `cr-001/run-20261016-225147-fa0b19cd` |
| CR-003 | `cr-003/run-20261016-225245-88c4417c` |
| CR-005 | `cr-005/run-20261016-225330-76344ec5` |

Each directory holds raw/normalized snapshots, engine responses, actual binary/template/input
hashes, live/frozen PDFs, qpdf logs, bbox XML, all page PNGs, `comparison.json`, `comparison.html`
and `visual-review.json`. The private `parent-cases/index.html` links all cases and PDFs. Temporary
requests and partial outputs are absent, and no owned case container remains running.

The source baseline was `01c855c`; the local connection-builder correction used by these runs is
committed in `d318b79`. The worker and adapter already included clock fix `3c579b5`; their actual
hashes are in each evidence manifest. Fonts were mounted from the existing Windows installation
without copying them into an image or repository. Native used its live local clock; Rust used the
captured host-local clock. These same-evening date comparisons do not freeze native time specials.

## Remaining coverage gates

- The remaining 95 eligible students and other content shapes have not been exported by this batch.
- Templates 04/05 and second-term evaluation have no currently published eligible cases. Do not
  alter publication to manufacture evidence.
- All standard cases are currently unpublished; their procedure also mutates marks/averages.
  Prepare an isolated writable database before attempting faithful standard exports.
- GOPS/BAS/TerraSanta RPT policies need an authoritative eligible database and template-owned SQL
  support in Rust. Local restored-copy metadata does not establish current deployment behavior.
- External Gradebook printing, authenticated Parent API/UI delivery, BIFF/XLS and broad legacy/
  multi-table/subreport parity remain open as documented in the source inventory and fork status.

No Parent deployment, report template, publication, grade/student data or SQL module was modified.
