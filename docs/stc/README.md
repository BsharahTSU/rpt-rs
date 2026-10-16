# STC Linux Crystal compatibility work

Target agreed on 2026-10-16: full current bridge compatibility, including PDF, BIFF/XLS Excel,
typed Parent Reports and legacy SQL/procedure/DataSet callers. This is experimental work in a
separate checkout. No current Parent App or backend deployment/configuration has been switched.

## Source and build checkpoint

- Upstream: https://github.com/MrSrsen/rpt-rs, commit `3f5beb51dd34d36c4a4326280d25f473b44889d1`.
- Development/build/verification use Docker Linux containers, as requested. No host Rust/C compiler
  was installed. The failed passwordless Ubuntu package attempt made no package changes.
- `Dockerfile.stc-dev` pins Rust 1.92 and includes Noto core/CJK fonts, Poppler and qpdf for verification.
- `scripts/stc/Invoke-DockerCargo.ps1 -Build` builds that image and runs the workspace tests.
  Sources are mounted read-only; compilation and downloaded crates use dedicated Docker volumes.
- Ignored local verification logs/artifacts live under `research/stc-verification/`. No school RPT,
  saved report rows, credentials, student selectors or private PDFs belong in this repository.

## Current implementation

The Page IR now carries a backward-compatible `rtl` flag (false is omitted and is the default
when reading older JSON). Pagination preserves the stored paragraph reading order. PDF rendering
orders bidi runs, intersects them with font fallback runs and shapes each in its actual direction.
RTL glyph clusters use increasing logical UTF-8 boundaries for Unicode mappings and scalar spacing;
the old next-glyph calculation could produce a reversed slice for Arabic.

Chinese rendering now falls back beyond the bundled Latin/symbol faces when those faces lack a
character. The resolver checks actual glyph coverage, prefers a Noto CJK family matching the
requested serif/sans/monospace class and caches coverage lookups. Existing primary/symbol choices
are preserved. `FontSource::System` and installed CJK fonts are required for this coverage; bundled
fonts alone do not cover Chinese. The regression tests first reproduced missing Chinese glyphs in
Arial reports and now cover that case as well as explicit Simplified/Traditional CJK families.

The upstream font-source test was corrected: different installed/bundled revisions of DejaVu can
share their PostScript name. It still requires both renders to embed a font and their deterministic
PDF bytes to differ when different source font bytes were resolved.

Verification on 2026-10-16:

- The final full Docker workspace run reported **1,564 passed, zero failed, zero ignored**, across
  77 test groups. Database cases that require an explicit connection were not exercised; no live
  school SQL was executed. Log: ignored `research/stc-verification/docker-workspace-after-chinese.log`.
- The four Linux Chinese integration tests cover glyph coverage, unspaced wrapping without dropped
  characters or width overflow, Simplified/Traditional and mixed Chinese/Latin/digits PDF embedding
  and actual UTF-8 extraction with Poppler, and Chinese fallback in an Arial report. The intentional
  fallback must produce exactly one `FontSubstituted` warning; unexpected diagnostics fail the test.
- Arabic checks cover logical UTF-8 cluster bounds with spacing, mixed bidi run order, preserving
  stored paragraph direction into the Page IR, and an end-to-end synthetic PDF probe.
- qpdf found no syntax/stream errors in either language probe. Poppler rendered both PDFs to PNG;
  visual inspection confirmed joined Arabic, mixed Latin/digits, direction/spacing/justification,
  and legible Simplified/Traditional Chinese, wrapping and Arial fallback. These are synthetic
  fixtures, not evidence of school-template parity.
- Docker formatting and strict Clippy cover the affected renderer, layout, Page IR and font crates.
- The Chinese integration target is Linux-only. Linux CI declares Noto core/CJK fonts and Poppler
  alongside qpdf; the existing Windows test job does not require those Linux packages.

Reproduce just the Chinese suite from PowerShell:

```powershell
./scripts/stc/Invoke-DockerCargo.ps1 -Build -CargoArgs @('test', '--locked', '-p', 'rpt-render', '--test', 'chinese')
```

To retain a synthetic probe, additionally mount a writable artifact directory and set
`STC_RPT_ARTIFACTS` to its container path. The normal test run uses a temporary PDF and removes it.

Three current ordinary AAJ templates decode in offline Linux containers: American, British and
9AHSD. The AAJ evaluation `rptStudentMark01.rpt` also decodes. This establishes reader compatibility,
not report-content or layout parity. They were mounted from the existing local report root read-only.

## Remaining compatibility gates

| Area | Required behavior | Status |
| --- | --- | --- |
| Protocols | Exact typed and legacy JSON, clean stdout, diagnostics on stderr | Pending adapter |
| SQL | Microsoft SQL Server text/procedure execution; strict typed selectors and legacy defaults | Pending adapter |
| DataSet | Schema/multiple-table transport; first-table main/subreport binding | Pending adapter |
| Parameters | RPT aliases/defaults, query/display precedence, ForReview exception | Pending adapter |
| PDF | Valid structural PDF/page count; Arabic, subreports, formulas and template layout | Arabic/Chinese core tests pass; school parity pending |
| Excel | Actual BIFF/XLS matching current callers; no silent XLSX substitution | Pending export implementation |
| Lifecycle | Owned typed paths, cleanup, cancellation and file diagnostic contracts | Pending integration |
| Deployment | Docker image and explicit opt-in bridge selection | Dev image only; production unchanged |

The canonical existing contracts remain in the backend repository's `docs/reports/README.md`.
Keep all required access/publication/HideMarks/student-scope rules in the current backend. A Rust
renderer must never turn a diagnostic PDF, dropped rows or unsupported formulas into a successful
certificate response. Real template/data/formula/subreport/pagination comparisons are required
before any deployment replacement is considered compatible.
