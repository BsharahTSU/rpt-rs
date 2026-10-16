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

PDF support is experimental. One real evaluation template has close visual parity, with residual
text/row metrics. Template-owned SQL and BIFF/XLS are explicitly unsupported. Multiple-table
transport exists, but main/subreport binding currently uses the first table. Full caller integration,
production packaging and broad template parity remain open. Production Parent configuration is unchanged.

## Source map

| Area | Location |
| --- | --- |
| Portable bridge and contract checks | `bridge/Stc.CrystalBridge`, `bridge/Stc.CrystalBridge.Tests` |
| Prepared-data Rust worker | `apps/stc-render-worker` |
| Linux build/run entry points | `scripts/stc/Invoke-DockerCargo.ps1`, `scripts/stc/Invoke-DockerBridge.ps1` |
| Renderer implementation history and limitations | [README](README.md) |
| Checkpoint ledger | [WORKLOG](WORKLOG.md) |
| Parent case discovery and comparison protocol | [coverage/README](coverage/README.md) |
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
