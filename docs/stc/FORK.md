# STC local fork

## Provenance and ownership

This checkout is the STC local compatibility fork of [MrSrsen/rpt-rs](https://github.com/MrSrsen/rpt-rs).
It preserves the upstream Git history and MPL-2.0 LICENSE. The starting upstream revision is
`3f5beb51dd34d36c4a4326280d25f473b44889d1` (v0.4.0). The existing branch remains `main`.
On 2026-10-16 the original `origin` remote was renamed `upstream` to distinguish the source project
from our local development. No hosted fork, new branch, worktree, push or deployment was created.
The user's requested workflow is local commits only. Existing upstream authorship is retained.

## Target and current limits

The target is compatibility with the current STC Crystal bridge: typed Parent Reports, legacy
SQL/procedure/DataSet callers, PDF, and BIFF/XLS. The portable adapter uses .NET 10 for SQL and
request validation and the Rust worker for rendering. Rust builds and candidates run in Docker
Linux; original Crystal baselines require the existing Windows Crystal installation.

PDF support is experimental. Three representative evaluation cases have native/Rust exports.
The [CR-001 repair](coverage/cr-001-parity.md), implemented through clean commit `be022bd`,
addresses saved-printer pitch and scoped ascent, Basic-Latin spacing, trailing-space centering,
shaped alignment-width rounding, ordinary border/rule/rounded-frame geometry, stored paragraph
RTL and native Arabic mark offsets. The growing StandardText ascent path follows actual shaped
RTL glyphs, preserving fixed/English guards without report-specific constants.
Both native/Rust PDFs have been regenerated and pass integrity, page/date and ordered-rating
checks. Independent final all-page/operator review passes, and the captured CR-001 comparison
is accepted below 0.06 pt X / 0.031 pt Y, with remaining differences documented in its acceptance
record. [CR-003](coverage/cr-003-parity.md) is also accepted from clean `899ca64`:
printer device-width fitting, terminal LRM handling, centered RTL field preparation
and mixed-direction reconstruction are repaired and regression-tested. All 6,449
glyphs, 72 English lines and 52 ratings agree; maximum residuals are 0.06008 pt X /
0.03016 pt mark Y. CR-001 replay remains byte-identical. [CR-005](coverage/cr-005-parity.md) now passes text/layout checks for all 4,918
glyphs, 59 English lines and 35 ratings. A newly measured shared logo-color residual
remains open in all three cases; native quantizes the original bitmap while Rust
retains its exact pixels.
These cases do not establish full coverage, pixel identity or authenticated delivery.

Template-owned SQL and BIFF/XLS are explicitly unsupported. Multiple-table transport exists,
but main/subreport binding currently uses the first table. Full caller integration, production
packaging and broad template parity remain open. Production Parent configuration is unchanged.

## Source map

| Area | Location |
| --- | --- |
| Portable bridge and contract checks | `bridge/Stc.CrystalBridge`, `bridge/Stc.CrystalBridge.Tests` |
| Prepared-data Rust worker | `apps/stc-render-worker` |
| Linux build/run entry points | `scripts/stc/Invoke-DockerCargo.ps1`, `scripts/stc/Invoke-DockerBridge.ps1` |
| Renderer implementation history and limitations | [README](README.md) |
| Checkpoint ledger | [WORKLOG](WORKLOG.md) |
| Parent case discovery and comparison protocol | [coverage/README](coverage/README.md) |
| Current CR-001 repair and acceptance evidence | [coverage/cr-001-parity](coverage/cr-001-parity.md) |
| Detailed CR-001 findings and fixes | [metrics](coverage/cr-001-metrics.md), [spacing](coverage/cr-001-spacing.md), [geometry](coverage/cr-001-geometry.md), [Arabic](coverage/cr-001-arabic.md), [whitespace](coverage/cr-001-whitespace.md), [text origins](coverage/cr-001-text-origins.md) |
| Current CR-003 acceptance and native findings | [comparison](coverage/cr-003-parity.md), [fitting and whitespace](coverage/cr-003-whitespace.md) |
| Private inputs, logs and outputs (untracked) | `research/stc-verification/` |

Canonical Parent behavior is documented in the sibling backend repository
`Mobile Parent Web Service/docs/reports/README.md`; current code is authoritative when a historical
note differs. The active Parent grades page and typed Parent Reports endpoints determine user-visible
coverage. Legacy bridge capabilities are tracked separately from exposed Parent report cases.

## Commit and evidence policy

Use focused conventional commit subjects with explanatory bodies and validation. Keep findings and
fixes in Markdown with each checkpoint. Preserve failed experiments as summarized evidence, not
as silently accepted results. Commit scripts and synthetic regression tests; exclude real school
RPTs, private PDF/data artifacts, names, student IDs and credentials. `research/` is ignored.
Record report/template/configuration branches separately from student counts: one representative
student does not prove every student's content or pagination. SQL eligibility is not an authenticated
API test, and successful PDF export is not layout parity.
