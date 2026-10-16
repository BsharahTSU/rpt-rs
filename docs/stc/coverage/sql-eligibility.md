# Parent Crystal SQL eligibility inventory

Inspected 2026-10-16. Read-only discovery covered the current local AAJ database and metadata in other accessible school copies. No publication flags, tuition restrictions, procedures, schema or application configuration were changed. SQL eligibility is separate from an actual PDF export and an authenticated Parent App request.

## Reproduce

Run [Get-ParentCrystalInventory.ps1](../../../scripts/stc/Get-ParentCrystalInventory.ps1) with Windows integrated SQL authentication:

    .\scripts\stc\Get-ParentCrystalInventory.ps1 -Server '<server,port>' -Database 'AAJ_Live' -BackendRoot '<backend checkout>' -TemplateRoot '<LMS reports folder>'

The default private output is research/stc-verification/parent-inventory. Output is restricted to the checkout's ignored research tree; reparse paths are rejected for output and templates. All text reads explicitly use UTF-8 for Windows PowerShell 5.1. The script saves candidate selectors, case representatives, the SELECT queries, procedure text/hashes, database/profile coverage and summary. Console output omits student/parent IDs, names, credentials and connection strings. Do not commit generated evidence.

cases.json supplies one representative per ready group; candidates.json retains all positive and negative candidates. Case IDs identify groups within an inventory run, not permanent student identities. Group keys include report kind, template, class, term, selected report/exam, language and grading mode. Evaluation uses shared language because the AAJ recipe ignores the requested UI language. Ordinary cases retain Arabic and English variants.

All database commands are SELECT statements/read-only CTEs, with a 60-second inventory statement timeout. No stored procedure is executed. The first complete local run took about eight seconds. Full candidate selection currently supports AAJ only; other databases receive metadata inspection and are never automatically promoted from restored copies to current deployments.

## Current AAJ result

Eight Crystal templates exist in the inspected Parent paths: five evaluation RPTs and three ordinary certificates. The inventory covered 246 distinct current students in these cohorts and produced 38 groups: ten evaluation cases and 28 ordinary class/term/language cases.

| Case | Evaluation template | Class | Term/report | Cohort | SQL eligible | Nonempty scoped candidates |
| --- | --- | --- | --- | ---: | ---: | ---: |
| CR-001 | aaj/rptStudentMark01.rpt | 3 | 1/1 | 27 | 27 | 27 |
| CR-003 | aaj/rptStudentMark02.rpt | 4 | 1/1 | 34 | 33 | 33 |
| CR-005 | aaj/rptStudentMark03.rpt | 5 | 1/1 | 40 | 39 | 38 |
| CR-007 | aaj/rptStudentMark04.rpt | 6 | 1/1 | 31 | 0 | 0 |
| CR-009 | aaj/rptStudentMark05.rpt | 8 | 1/1 | 32 | 0 | 0 |

There are 98 ready student candidates across three first-term evaluation templates. Class 4 has one hidden-marks candidate. Class 5 has one unpublished candidate and another passing publication with no printable indicator rows. Classes 6 and 8 have no published candidates; class 8 also has one hidden-marks candidate. All second-term evaluation cases are unpublished.

Ordinary cases cover classes 12–15 (British), 16–17 (American), and 18 (9AHSD), both terms and Arabic/English requests. All are unpublished under the current Parent query. They also carry blocked-live-mutating-procedure and must not be exported directly against the current database for read-only comparison.

## Application gates

Source authority is the sibling backend's current Dotnet10/WebApi/Features/ParentReports files: ParentReportRepository.cs, ParentReportService.cs, ParentReportRules.cs, ParentReportEvaluation.cs, ParentReportTenantResolver.cs and GradeAccessPolicy.cs. Begin canonical documentation at that repository's docs/reports/README.md.

1. Parent/student claims must authorize the student; parent claims must match current ParentID. Inventory has no token and does not prove authenticated ownership.
2. Current school/class/section comes from vwStudentCourses, ordered by school/class/section, with matching Students, Schools and SchoolClasses rows. Duplicate school/class configuration is ambiguous because the application uses unordered TOP (1); inventory blocks duplicates. None occurred in AAJ.
3. Any FatherTuitionMaster.HideMarks row for the current parent with integer 1 or textual true blocks grade reports. Certificate printing also requires SysParam.AllowPrintFinalCertificate.
4. Evaluation requires EvaluationSystem; standard requires IsStandardCertificate. IsGradeBook is independent. Evaluation supports AAJ/GOPS/BAS/TerraSanta. Deployment and database WebSchoolReports must resolve to the same supported family.
5. Evaluation paths join WebSchoolReports and EvaReportName, retaining an existing prefix once. Missing/invalid files do not fall back. Engine follows extension.
6. Evaluation chooses the highest SchoolReports.ID with AllowPrint=1 for the term. GOPS classes 1/2 force report 1/5 for term 1/2. Term one accepts IDs 1–4; term two accepts 5–8. Semester and exam metadata are required.
7. Every StudentMarksMaster row for student/report/term must publish the selected slot, and at least one row is required. IDs 1/5 use MarkPublish, 2/6 Mark2Publish, 3/7 Mark3Publish, 4/8 Mark4Publish. The application does not add year/school/class predicates to this master query; current course scope is separate.
8. Ordinary publication checks current-year Std_MarKS rows having a matching scoped web_vwStudentMarksReport row with nonzero Mark1+Mark2+Mark3+Mark4. It selects the highest slot 1–8 published on every qualifying row. At least one row is required. Slots 5–8 normalize to exam 1–4. Ordinary state does not require a printable SchoolReports row. AAJ term-two exam 4 becomes certificate exam 5.
9. Global feature settings, catalog/asset availability, dependencies, trusted paths and returned-data checks must still pass. Inventory is a preflight and does not substitute for application status or actual export.

The reusable SQL implements the inspected AAJ rules. Other families' rules above were source-inspected; candidate execution on their copies needs an authoritative selected database with current schema.

## Procedure side effects and scope

AAJ web_Get_Std_MarksWithChartStc is not read-only. It unconditionally updates Std_Marks.HideOnCertificate for the class range, without student, school or year predicates; invokes CalculateGPA_Term for American classes; and deletes/reinserts StudentsAverage values for selected students. CalculateAvg=0 and a positive StudentID do not disable these writes. Future ordinary comparisons need an explicitly prepared disposable database or independently reviewed immutable dataset path.

AAJ web_GetStudentsIndicatorPreview was manually reviewed as SELECT-only. Its reviewed SHA-256 is 1bcd37818cd5706ec611777ad0883022b5f7eb03929586d16e6a86a7eafb2a92. Inventory requires this hash for ready candidates; a changed definition yields evaluation-procedure-review-required. Keyword scans in procedure-audit.json are hints, not proof of safety.

The evaluation procedure's normal branch hardcodes ReportID=1, does not filter its TermID argument, excludes blocked-email/hidden/extra indicator rows and applies the main-category rule. Inventory mirrors these predicates and checks nonempty data plus selected term/report identity. Actual bridge scope validation remains required. First-term/report-one evidence does not establish other report support.

The separate web_getStudentsMarks definition in AAJ contains a persistent StudentMarksMaster.ReportID update. Current AAJ RPTs do not use it. The GOPS repair marker applies only to the specifically repaired procedure/database; do not assume all copies or production have received it.

## Local coverage

The override directory contains 19 profiles; older canonical references describe 18. None of the 14 literal override database names matched a local database name exactly. Five profiles have no literal sCon override and need inherited/runtime configuration resolution. The user-selected/current base test target is local AAJ. No remote profile servers were contacted.

| Database family | Local copies |
| --- | ---: |
| AAJ | 7 |
| BAS | 6 |
| Catholic | 6 |
| GOPS | 2 |
| IEC | 3 |
| JIEC | 5 |
| JIS | 1 |
| NOS | 6 |
| Rosary | 10 |
| TerraSanta | 2 |
| Whitman Academy | 3 |

Only current AAJ had configured .rpt evaluation rows among the 51 local school copies surveyed. The inspected latest BAS copy uses MRT evaluations. Inspected GOPS and TerraSanta copies lack SchoolClasses.IsStandardCertificate, so current Parent student loading fails before rendering; no migration was applied. IEC/JIEC/JIS and literal Whitman Academy are unsupported by current Parent Crystal rules. Dated/restored copies remain discovery evidence.

## Verification

The PowerShell file parsed successfully and the complete read-only SQL Server 2016 inventory succeeded, including metadata inspection of compatibility-100 copies. The run saved private evidence and created no PDFs. Actual native/Rust exports and visual comparisons belong in the per-case rendering ledger.
