# Isolated report-case runner

Use [parent-surfaces](parent-surfaces.md) and [sql-eligibility](sql-eligibility.md) before running a case.
The runner does not grant Parent access or reimplement an authenticated status request. It accepts
only a private case manifest under this checkout's ignored `research/stc-verification/` folder.

## Discovery and selection

Run `scripts/stc/Get-ParentCrystalInventory.ps1` with the local SQL instance, backend and template
roots. It uses Windows integrated authentication and SELECT/metadata only. Review the actual
procedure definition, not just the heuristic audit flags. Do not run mutating report procedures
against live school data. The current standard AAJ procedure needs an isolated writable database
before a faithful execution can be attempted.

Choose one ready candidate per case. Copy its selectors and display variables to a private JSON
manifest. The shape below uses illustrative values, not an actual student. The reviewed hash is
SHA-256 of the UTF-8 SQL module definition (without a file-added newline).

```json
{
  "caseId": "cr-example",
  "database": "LOCAL_TEST_DATABASE",
  "relativeTemplate": "aaj/rptStudentMark01.rpt",
  "procedure": "dbo.web_GetStudentsIndicatorPreview",
  "procedureReadOnly": true,
  "reviewedProcedureSha256": "<64 lowercase hex characters from reviewed definition>",
  "queryParams": {
    "StudentID": 1, "SchoolID": 1, "ClassID": 3, "SectionID": 1,
    "ReportID": 1, "TermID": 1, "CategoryID": 0,
    "ForSpecialAid": false, "ForReview": false
  },
  "reportVariables": {"CompanyName": "Example", "PrintUser": "stc", "ReportYear": "2026/2027"}
}
```

## Execution

Build the current adapter and worker in Docker using `Invoke-DockerBridge.ps1 -Build` before cases.
Then run one case:

```powershell
./scripts/stc/Invoke-ParentCrystalCase.ps1 -CaseFile <private-case.json> -Server <local-instance> -TemplateRoot <reports-root> -NativeBridge <existing-CrystalReportBridge.exe>
```

Each invocation creates a fresh run directory and records evidence, with three independent stages:

- `native-live`: unchanged original typed bridge executes the reviewed SQL recipe.
- `native-frozen`: original legacy DataSet bridge renders a captured schema-bearing XML snapshot.
- `rust-frozen`: portable legacy DataSet bridge renders the exact same snapshot inside Docker Linux
  with `--network none`. No database connection or credential is sent to this container.

Capture executes the reviewed procedure once, validates all returned tables against present scope
columns, preserves raw XML, normalizes only the first table exactly like typed native PDF, and writes
schema-bearing replay XML. Nulls, types, binary values, table order and row order are retained.
A separate native-live query is not the same snapshot: compare its PDF to native-frozen to detect
changes or replay limitations. The native frozen path has no SQL schema-adaptation fallback.

The procedure hash is rechecked before capture. A caller's reviewed hash is an explicit prerequisite,
not a general SQL safety parser. The current runner is for trusted local verification manifests;
it is not a production service endpoint. A report with scope columns absent from its result still
needs the reviewed SQL's predicates to establish scope, just as the typed native contract does.

The evidence captures template, snapshot, request-manifest, actual worker/adapter binary and native
executable hashes, source HEAD/dirty state, Docker image ID, selected font hashes, culture and local
clock. See [clock-parity](clock-parity.md) for override/fallback behavior. Native uses its live local
clock; it cannot be time-frozen by this runner. Avoid crossing a date boundary and inspect any time
specials separately. Mounted Windows fonts stay on the host and are not redistributed.

Process/SQL time bounds belong to this test harness, not product reporting limits. A timed-out Docker
client is followed by removal of its uniquely named owned container. Temporary requests are deleted
in `finally`. Failures retain private logs and never reuse an earlier run's PDF.

## PDF verification

Mount the completed run as `/work` and this repository as read-only `/src` in the existing Linux
image, then run `python3 /src/scripts/stc/compare_case.py /work` with Docker networking disabled.
It requires qpdf integrity, extracts per-page words/boxes/dates, renders every page to PNG, records
native-live/frozen and native-frozen/Rust differences and builds a local comparison HTML page.
It only reads PDFs whose execution stage passed. Text equality is diagnostic and never automatic
visual acceptance. Record an explicit review of all pages and case-specific values/headings/images.

## Focused checks

- `scripts/stc/Test-CaseTools.ps1`: cross-table scope rejection, Chinese/Arabic normalization, schema,
  decimal/binary/null fidelity, path containment, subprocess argument/exit handling and stale PDF rejection.
- Docker `python3 -m unittest test_compare_case.py` in `scripts/stc`: duplicate value loss, moved content,
  Chinese/Arabic text, paper/date differences and refusal to infer visual acceptance.
- Clock, bridge and worker checks are described in [clock-parity](clock-parity.md).

Keep real artifacts private. Commit only scripts, synthetic checks, sanitized findings and Markdown
review outcomes. A readiness count is not a successful PDF count; one representative does not prove
all students or every report content shape.
