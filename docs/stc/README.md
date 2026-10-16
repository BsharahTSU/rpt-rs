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

- The final full Docker workspace run reported **1,572 passed, zero failed, zero ignored**, across
  78 test groups. Its database cases requiring an explicit connection were not configured. The
  separate authorized SQL Server comparison is described below. Log: ignored
  `research/stc-verification/docker-workspace-after-bridge.log`.
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

## Experimental PDF adapter checkpoint

`bridge/Stc.CrystalBridge` is a portable .NET 10 adapter using Microsoft.Data.SqlClient; it does not
load CrystalDecisions. It ports the inspected current typed/legacy request validation, typed owned
paths, declared SQL parameter rules, ForReview exception, first-table DataSet binding, result-scope
validation and PDF text normalization. No connection/command/application data limits were added.
The credential-bearing request stays in the adapter; the Rust worker receives only prepared rows,
column types, selectors and display variables over stdin. Stdout is one response JSON, diagnostics
are on stderr, and a successful scratch export replaces the owned output only after rendering.

`apps/stc-render-worker` adapts first-table rows to report table aliases/types, supplies that table
to subreports, binds declared report parameter aliases/current/default values, and renders with
system fonts. It rejects coercion failures, non-font fidelity diagnostics, missing image assets and
empty output. Null text remains Null; SQL binary values remain bytes. This is an experimental
compatibility layer, not a claim that every Crystal template or legacy input is supported.

The first real AAJ evaluation comparison now runs end to end against the local SQL Server 2016
reporting procedure. Both the unchanged Windows Crystal bridge and Linux Rust pipeline produced
three-page structurally valid PDFs. All 35 extracted rating values match in order across pages.
Visual inspection of every page still fails parity: Science moves from page 3 to page 2, the
continuation group heading is absent, some borders extend into later bands, and footer conditions
and date formatting differ. Therefore matching page counts/values is not an acceptance gate.
The actual student selectors, source model, IR, logs and PDFs remain in ignored local research.

That comparison exposed an OLE picture gap: the school logo is a native bitmap in `Ole10Native`,
not `CONTENTS`. The reader now extracts only a recognized image inside the declared, bounded
MS-OLEDS native-data envelope, keeps CONTENTS precedence and preserves embedding/subreport scope.
Truncation, non-image native content, precedence and scoping have synthetic regression coverage.
The improved comparison includes the logo. Existing Windows fonts were mounted read-only for
that comparison; they are not copied into an image/repository. A default Linux-font render was
also retained and shows larger Arabic metrics, so required production font packaging remains open.

Verification additionally includes 28 portable bridge contract checks (typed/legacy JSON, scoped
paths and rows, SQL argument discovery/defaults, XXE rejection, and Chinese/null/binary/multiple-table
DataSet transport) and five worker tests. Strict Clippy passed for the worker, reader and data crates; formatting
also passed. The .NET build finished with zero warnings/errors. No persistent database DML was executed for the comparison;
the selected procedure was inspected for side effects first. This was an offline engine comparison
using the Parent recipe; it did not exercise the authenticated Parent API or change its configuration.

Build/test the experimental harness separately from invoking a request:

```powershell
./scripts/stc/Invoke-DockerBridge.ps1 -Build -Test
```

Invoke it with an existing request whose template/output paths are `/reports/...` and `/work/...`:

```powershell
./scripts/stc/Invoke-DockerBridge.ps1 -RequestFile <owned-request-file> -WorkDirectory <private-working-folder> -TemplateRoot <report-root>
```

`-FontDirectory` optionally mounts an existing caller-provided font folder read-only. `-DebugPageIr`
writes private report contents into the work folder; use it only in a private ignored verification
folder. Request creation/removal and host/container SQL endpoint selection belong to the caller.
Template-owned SQL (`queryType: template`) and BIFF/XLS currently return an explicit failure.
The development image mounts sources/build outputs; it is not a standalone production package.

The adapter pins `System.Security.Cryptography.Pkcs` 9.0.21 because SQL Client's older transitive
version is covered by Microsoft's [October 13 advisory](https://github.com/advisories/GHSA-846f-f764-76mx).
The production backend dependency files were not changed.

## Parent evaluation trace inspected for this comparison

1. Active `src/app/pages/grades/grades.component.ts` checks the evaluation option and status, then
   calls `ParentReportsService.reportCardPdf` with the selected student, term, UI language and
   `certificateType: evaluation`. It opens the returned PDF blob through the Parent PDF flow.
2. `parent-reports.service.ts` sends the typed `api/parent-reports/report-card/status` and `/pdf`
   requests. The compatibility Stimulsoft `/reports-viewer` component is not this rendering path.
3. Backend `ParentReportRepository` derives school/class/section from the current student course,
   reads the matching SchoolClasses evaluation flag/name, prefixes SysParam.WebSchoolReports,
   selects printable SchoolReports metadata and requires the matching published StudentMarksMaster
   slot. Parent ownership, HideMarks and the school printing policy remain backend gates.
4. The AAJ Crystal branch in `ParentReportRules.EvaluationReportCard` selects
   `web_GetStudentsIndicatorPreview` with StudentID, SchoolID, ClassID, SectionID, ReportID, TermID,
   CategoryID=0, ForSpecialAid=false and ForReview=false; display variables are CompanyName,
   PrintUser=stc and ReportYear. Only the absent declared ForReview argument is tolerated.
5. The comparison uses exactly that database-selected RPT and recipe. No school/class/report
   selectors or publication settings were edited.

## Remaining compatibility gates

| Area | Required behavior | Status |
| --- | --- | --- |
| Protocols | Exact typed/legacy JSON and diagnostic channels across callers | Port and contract checks; full caller integration pending |
| SQL | SQL Server text/procedures, strict selectors and legacy defaults | One live evaluation procedure passes; other templates/procedures pending |
| DataSet | Schema/multiple-table transport; first-table main/subreport binding | Chinese/null/binary transport checks pass; broad subreport/schema parity pending |
| Parameters | All RPT aliases, linked/range/default values and precedence | Initial declared binding; complex and embedded-query cases pending |
| PDF | Content, pagination, repeated headers, borders, footers, dates and fonts | Real comparison values match; visual parity fails |
| Excel | Actual BIFF/XLS preserving current callers | Not implemented; rejected before executing SQL |
| Lifecycle | Owned paths, cleanup, cancellation and caller logging | Owned paths/scratch cleanup covered; cancellation/integration pending |
| Deployment | Standalone Docker runtime and explicit opt-in bridge selection | Local development harness only; production unchanged |

The canonical existing contracts remain in the backend repository's `docs/reports/README.md`.
Keep all required access/publication/HideMarks/student-scope rules in the current backend. Real
report-content/layout comparisons and unsupported-feature failures remain required before the
experimental adapter can replace the current bridge.
