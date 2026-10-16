# Parent Crystal PDF coverage

## Scope and acceptance

Inventory every current Parent App Crystal report path and configuration branch. Discover eligible
student/report/term slots from SQL without changing publication or access rules. Select representative
cases per template/recipe, including meaningful language and pagination variants. Run cases separately,
with an original Windows Crystal baseline and a Docker Linux Rust candidate.

Do not execute an unreviewed procedure. In particular, the documented legacy GOPS marks loader can
update report selectors. Confirm deployed procedure behavior before calling it. Database discovery
uses metadata/SELECT queries only. Record unavailable databases/templates and absent eligible rows as
coverage gaps, not renderer failures or passes.

## Per-case gates

1. `discovered`: current Parent route/catalog/template branch established from source.
2. `eligible`: matching student course, school/class options, report/term metadata and publication
   satisfy the current backend rules; record parent ownership/HideMarks policy separately.
3. `native-pdf`: original bridge returned success and a non-empty structurally valid PDF.
4. `rust-pdf`: Docker bridge returned success without forbidden fidelity diagnostics and a valid PDF.
5. `content`: same template, selectors, parameters and data established; compare extracted text/values,
   page sizes/counts, images, dates, repeated groups and footer conditions.
6. `visual`: inspect rendered pages individually; record each defect. A matching page count alone is insufficient.
7. `accepted`: all applicable checks pass with documented tolerances. Pixel identity is a separate stricter claim.

An offline eligibility/export check never establishes authenticated API ownership or UI behavior.
Do not label native and Rust as using identical data unless a captured-data replay or a verified stable
snapshot establishes that. Flag independent live queries as a comparison limitation.

## Evidence and reproducibility

Shared Markdown uses opaque case IDs. Private manifests under ignored `research/stc-verification/`
map cases to database/student selectors, RPT path/hash, recipe, parameter types, culture/date format,
font inputs, engine commits/versions, timestamps, status and artifact hashes. Never serialize passwords
into a tracked manifest. Remove temporary credential-bearing requests in `finally` blocks.

Run a bounded case, verify it, update the ledger, then proceed. Keep failed output diagnostic logs local.
New renderer fixes require a synthetic regression and a rerun of the affected case; do not repeat an
unchanged full suite on every case. Chinese regression coverage remains part of renderer validation.

## Inventory status

Source and SQL discovery are complete for the documented local scope: 38 AAJ case groups, three
ready evaluation groups and 98 candidate students. Other local copies are metadata-only evidence;
external Gradebook and authenticated API/UI delivery remain open. Read the [surface inventory](parent-surfaces.md),
[SQL findings](sql-eligibility.md), [isolated runner](case-runner.md), and [clock fix](clock-parity.md).
[Initial actual comparisons](initial-comparisons.md) now record three passing native/Rust representative exports, identical native live/replay images, matching ordered ratings and the remaining visual defects. The other 95 eligible students remain SQL-only evidence.
