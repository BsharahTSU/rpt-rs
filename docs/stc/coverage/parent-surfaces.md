# Parent Crystal report surface inventory

Inspected 2026-10-16 from current Parent client and backend source. This is a source and local-file inventory, not a live SQL, authenticated API, export or PDF parity test. No configuration, publication, student data or report-selection behavior was changed by this inspection. See [coverage gates](README.md) and [fork limitations](../FORK.md).

## Current Crystal cases

The typed Parent report-card path has three fixed Crystal templates, all AAJ standard certificates, plus database-selected evaluation RPTs in four supported families. The dynamic rows below represent policies, not a finite list of filenames: SQL discovery must enumerate their actual resolved templates.

| Case family | Server selection | Template and data contract |
| --- | --- | --- |
| AAJ standard American | Class 16 or 17; standard certificate enabled | `aaj/SchoolReportCardAmerican.rpt`; `web_Get_Std_MarksWithChartStc` |
| AAJ standard British | Classes 12 through 15; standard enabled | `aaj/SchoolReportCardBritish.rpt`; same procedure |
| AAJ standard AHSD | Class 18; standard enabled | `aaj/SchoolReportCard9AHSD.rpt`; same procedure |
| AAJ evaluation | Evaluation enabled; database-selected `.rpt` | `<WebSchoolReports>/<EvaReportName>`; `web_GetStudentsIndicatorPreview` |
| GOPS evaluation | Evaluation enabled; database-selected `.rpt` | Same dynamic path rule; template-owned SQL |
| BAS evaluation | Evaluation enabled; database-selected `.rpt` | Same dynamic path rule; template-owned SQL |
| TerraSanta evaluation | Evaluation enabled; database-selected `.rpt` | Same dynamic path rule; template-owned SQL |

All three fixed AAJ files were present under the configured local Desktop LMS `reports` root. This check establishes file presence only. Other RPT files on disk are not automatically Parent report cases.

Authoritative backend evidence:

- [ParentReportRules.cs](<../../../../Mobile Parent Web Service/Dotnet10/WebApi/Features/ParentReports/ParentReportRules.cs>): lines 30-42 select engines and the three fixed RPTs; 116-145 select AAJ recipes; 503-519 define standard Crystal parameters; 551-580 define evaluation recipes; 614-644 define the class-specific Crystal variables.
- [ParentReportEvaluation.cs](<../../../../Mobile Parent Web Service/Dotnet10/WebApi/Features/ParentReports/ParentReportEvaluation.cs>): supported families, explicit/legacy selection, safe relative paths, school-folder prefix and extension dispatch.
- [ParentReportCatalogDefinition.cs](<../../../../Mobile Parent Web Service/Dotnet10/WebApi/Features/ParentReports/Configuration/ParentReportCatalogDefinition.cs>): lines 57-86 declare the three fixed Crystal templates for both terms and both accepted languages.
- [ParentReportCatalog.cs](<../../../../Mobile Parent Web Service/Dotnet10/WebApi/Features/ParentReports/Configuration/ParentReportCatalog.cs>): evaluation authorizes the exact database-owned path through a compiled policy; it does not require one literal catalog row per evaluation filename.
- [CrystalRenderer.cs](<../../../../Mobile Parent Web Service/Dotnet10/WebApi/Features/ParentReports/Rendering/CrystalRenderer.cs>): lines 90-98 use `stored_procedure` when a recipe supplies a procedure and `template` otherwise.

The current Rust adapter explicitly rejects template-owned SQL. GOPS/BAS/TerraSanta evaluation RPT cases therefore require separate readiness work; they cannot be treated as the already exercised AAJ procedure path. BIFF/XLS and broad multiple-table/subreport fidelity also remain open, as described in the fork status.

## Parent entry points and eligibility

The Grades screen exposes separate standard and evaluation actions. Each sends an explicit certificate type, student, term and language; performs a fresh status request; and requests the PDF only when status permits printing. Current requests accept terms 1/2 and languages `ar`/`en`. The rendered PDF opens through the shared native/embedded PDF viewer.

Client evidence:

- [grades.component.html](<C:/Projects stc/Parent App/Client/src/app/pages/grades/grades.component.html>): lines 15-23 expose Gradebook actions; 52-63 expose separate certificate print/view actions.
- [grades.component.ts](<C:/Projects stc/Parent App/Client/src/app/pages/grades/grades.component.ts>): lines 393-426 open Gradebook; 438-500 perform certificate validation, status and PDF generation.
- [parent-reports.service.ts](<C:/Projects stc/Parent App/Client/src/app/core/STC/services/parent-reports.service.ts>): typed PDF calls, including report-card at line 269.
- [ParentReportsController.cs](<../../../../Mobile Parent Web Service/Dotnet10/WebApi/Features/ParentReports/ParentReportsController.cs>): authenticated report-card PDF/status/Excel and IG PDF/Excel routes.

The status and export path share [ParentReportService.cs](<../../../../Mobile Parent Web Service/Dotnet10/WebApi/Features/ParentReports/ParentReportService.cs>) resolution, starting at line 215. The checks include authorized current student context, the application feature switch, `HideMarks`, `AllowPrintFinalCertificate`, the independently selected SchoolClasses flag, publication, deployment/database family agreement, safe template/assets and runtime dependencies. A missing evaluation filename or invalid extension does not fall back to a standard certificate.

[ParentReportRepository.cs](<../../../../Mobile Parent Web Service/Dotnet10/WebApi/Features/ParentReports/ParentReportRepository.cs>) loads current school/class/section from `vwStudentCourses`, with matching SchoolClasses options. Standard publication uses `Std_MarKS` and the report view. Evaluation uses a printable `SchoolReports` row and matching `StudentMarksMaster` publication. GOPS classes 1/2 retain preferred report IDs 1 for term 1 and 5 for term 2. The SQL eligibility investigation must reproduce the actual publication resolution and policy; a student/class match alone is insufficient.

Explicit standard selection requires `IsStandardCertificate`; explicit evaluation requires `EvaluationSystem`. Evaluation is supported for GOPS/BAS/AAJ/TerraSanta and does not require `IsFoundationStage`. Older requests omitting the selector retain automatic evaluation/standard choice, with the selected flag still enforced.

## Coverage variants to discover and exercise

| Axis | Required distinction |
| --- | --- |
| Fixed Crystal cohorts | All seven class IDs across the three RPTs; shared filename does not prove equal data or variables |
| Terms | Both printable terms; report IDs and final-exam handling remain part of each case |
| Language | Arabic and English accepted requests; standard SQL language changes even where the RPT filename does not |
| Grading mode | Available numeric/letter variants; the American template also receives `LettersGrade` as a display variable |
| AAJ exam mapping | Published term-two exam 4 becomes certificate exam 5; selected exams 6/7 also use effective exam 5 |
| Evaluation path | Every distinct database-selected safe RPT path, school/class cohort and printable report/term slot |
| Data shape | Representative short/long text, empty/filled comments, repeated groups, varying indicator counts, images, optional sections and pagination |
| Caller protocol | Typed Parent recipe versus legacy query/procedure/DataSet compatibility; separate from public Parent UI coverage |
| Language regression | Keep synthetic Chinese coverage; Chinese is not an accepted current Parent report-request language |

Build representative cases from eligible SQL rows without changing access, flags or publication. Record no eligible rows, unpublished reports, missing templates/assets, unsupported modes and inaccessible databases separately. Do not count these as renderer passes or renderer regressions.

For AAJ standard reports, `withAvg` is removed from the common parameters and `CalculateClassAvg=0` is added. American variables include current year, grading mode, two exam bindings and two semester bindings. British has current year and two exam/semester bindings; AHSD has current year and one exam binding. Evaluation AAJ uses its indicator procedure and CompanyName/PrintUser/ReportYear variables. These distinctions belong in the per-case manifest rather than reconstructing all cases from one sample.

Use the gates in [README](README.md): source discovery, SQL eligibility, native export, Rust export, content, visual review and acceptance. Record authenticated API/UI proof separately. Identical filenames and parameters do not prove identical data when the two engines independently query a changing database; prefer captured-data replay or an established stable snapshot.

## Other surfaces and explicit exclusions

| Surface | Current evidence and treatment |
| --- | --- |
| Other ordinary certificates | Stimulsoft, including AAJ classes outside the three fixed Crystal overrides |
| Evaluation `.mrt`/`.mtr` | Stimulsoft; maintain in overall reporting inventory, exclude from Crystal parity |
| Evaluation `.cshtml` | Razor/browser PDF; separate engine |
| IG registration | Stimulsoft only, NOS/GOPS; `igStudentRegistrationSubjects.mrt` |
| `/reports-viewer` | Active client Stimulsoft component, not the sibling old viewer and not a Crystal selector |
| Gradebook / Gradebook Details | Parent opens an external teacher application after visibility checks; print engine and controls remain unverified |
| Generic legacy Crystal PDF/Excel | Compatibility endpoints exist; migrated backend blocks authenticated parent/student roles from caller-owned query/template generation |
| Teacher/admin RPT inventory | Template presence or a teacher-only controller branch does not activate a Parent feature |

[ParentReportLegacyEndpointPolicy.cs](<../../../../Mobile Parent Web Service/Dotnet10/WebApi/Features/ParentReports/Legacy/ParentReportLegacyEndpointPolicy.cs>) blocks `web_generateReport`, `generateCrystalReport`, `generateCrystalReportExcel` and `RenderHtmlToPdf` for authenticated parent/student identities. [LegacyWcfController.cs](<../../../../Mobile Parent Web Service/Dotnet10/WebApi/Controllers/LegacyWcfController.cs>) invokes that policy. This describes the migrated backend; it is not a claim that every historical WCF deployment has this guard. The original WCF Crystal class remains a compatibility baseline, not proof of an active current Parent UI call.

No current non-backup client TypeScript caller of `generateCrystalReport` or an explicit `.rpt` selection was found. The routed [Stimulsoft component](<C:/Projects stc/Parent App/Client/src/app/core/STC/components/reports/stimulosoft/stimulosoft.component.ts>) and [PDF service](<C:/Projects stc/Parent App/Client/src/app/core/STC/services/stimulosoft-report-pdf.service.ts>) own `/reports-viewer` behavior.

Gradebook URLs target `${teacherUrl}/accounts/web/#!/gradebook/` and `gradebookDetails/`. The [shared iframe component](<C:/Projects stc/Parent App/Client/src/app/core/STC/components/iframe-page/iframe-page.component.ts>) rechecks options and HideMarks. The inspected Desktop LMS copy does not contain that `/accounts/web` application. Its similarly named teacher/student Gradebook controllers cannot establish the external Parent iframe's print capabilities. Full Parent-associated print coverage must keep this as an open inspection item.

## Profile and documentation observations

Current source has **19** `appsettingsOverrides/<profile>/appsettings.json` files, observed on 2026-10-16:

`AAJ`, `BAS`, `catholic`, `duroub`, `GOPS`, `IECJA`, `IECJUB`, `nazareneASH`, `nazareneZQ`, `NOS`, `rcIRBID`, `rcJA`, `rcMARJ`, `rcMISDAR`, `rcRamtha`, `rcSHMEISANI`, `ROSARY_BETHLEHEM`, `terrasanta`, `whitman`.

The dated [school-validation guide](<../../../../Mobile Parent Web Service/docs/reports/current/school-validation.md>) still describes 18 profiles. `rcRamtha` is present in the current tree but not recognized by [ParentReportTenantResolver.cs](<../../../../Mobile Parent Web Service/Dotnet10/WebApi/Features/ParentReports/ParentReportTenantResolver.cs>). The canonical reports README already records an earlier profile-inventory test mismatch. This inspection neither changed profile configuration nor reran that suite.

None of these 19 override files defines its own root `ConfigPublishName` or `ParentReports`. Effective shared runtime configuration and database `WebSchoolReports` must be resolved before declaring a profile supported. A filename alone is not a verified deployment family or reachable database. The current client environment selects AAJ.

The resolver implements AAJ, GOPS, NOS/NOSTest, BAS, Catholic, TerraSanta and Rosary aliases. Whitman resolves as a family but has no current report-card recipe. Other unmatched identifiers have no implemented typed Parent recipe. NOS, Catholic and Rosary have no evaluation product in the current Parent rules.

The Catholic Arabic-only Parent requirement remains documented while current code still selects an English T1 Stimulsoft template. This existing discrepancy is outside Crystal parity; no behavior was changed. See the [canonical language contract](<../../../../Mobile Parent Web Service/docs/reports/current/school-rules.md#catholic-parent-language-contract>).

## Inspection result and remaining evidence

This checkpoint establishes source branches, local fixed-template presence and a coverage plan. It does not establish live eligible student counts, native/Rust export success for the inventory, authenticated Parent access or rendered parity. Those results must be recorded by subsequent bounded case runs with private selectors/data/PDFs only under ignored `research/`.
