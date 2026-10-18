# CR-001-B03 continuation-header and Arabic shaping repair

**Accepted after repair (2026-10-18).** Both final PDFs were regenerated from clean
`8f9fd5e`. All 36 ratings and 4,748 painted glyphs match within the unchanged tolerances;
all six pages were reviewed. The original failure and intermediate checkpoints below
remain historical evidence. Coverage at that checkpoint: twelve accepted, none rejected, 86 untested.
See [batch 04](additional-evaluation-batch-04.md) for the later comparison and repair queue.

## Scope and retained failure

The [batch 03 rejection](additional-evaluation-batch-03.md) is preserved as the
before-repair evidence: three pages, 36 ratings allocated 15 / 16 / 5, a missing
page-two bilingual group heading, all sixteen subsequent rows 21.5 pt too high,
and different Arabic name outlines. Fix the renderer against the retained typed
snapshot; do not change templates, source text or acceptance thresholds. This repair
executes no SQL. Private data and PDFs remain under ignored
`research/stc-verification/cr-001-b03-repair/` on D:.

## Checkpoint 12a: multipart group-header continuation

`Formatter::emit_group` previously registered the group for repetition only after
all header sections completed. A page break between those sections therefore omitted
the already printed prefix on the continuation page. Native Crystal repeats that
prefix before continuing the suspended section. Its height accounts for the entire
21.5 pt body displacement in this case.

The active group is now registered before its header walk, with a count of completed
sections. `begin_page` repeats only that prefix. A break before the first section
repeats nothing; an ordinary detail-page continuation repeats the complete header.
The section count advances after emission, so deferred New Page After also includes
the section that just completed. Existing repeat/suppression and group scope rules
remain in effect. This is generic pagination behavior without report identifiers or
student-specific constants.

Two synthetic renderer regressions cover:

- A break between two header sections, with repetition enabled/disabled, for overflow,
  New Page Before and deferred New Page After; row order and exact positions checked.
- A whole header moved before its first section: no duplicate heading or missing rows.

The corrected red reproduction failed at the missing continuation heading; all **374
layout library tests** now pass in Docker. Rustfmt passed for changed layout files.
The initial fixture mistakenly treated numeric display formatting as plain text; it
was corrected to a string field before confirming the intended missing-header failure.

## Arabic shaping investigation and next checkpoint

Native paints an Arial Bold discretionary Arabic ligature where Rust paints four
component letters. Font GSUB lookup and actual embedded outline evidence identify
the `dlig` feature, currently disabled by the shaping defaults. Logical source text
and formula concatenation are correct. A scoped legacy option must agree in layout
measurement and PDF shaping, retain logical Unicode mappings, and guard ordinary,
Latin and Chinese output. Implementation/tests and final native/Rust regeneration
are pending at checkpoint 12a; this case remains rejected until all-page verification.

## Reproduction and resource policy

```powershell
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('test','--locked','--offline','-p','rpt-layout','--lib')
```

Builds use the existing D: target cache, one Cargo job/test thread, two CPUs and 2 GiB.
Private replay scratch is `D:\tmpppppp123\rpt-rs-cr001-b03`. Resource guard before
exports requires 3 GiB available RAM, 30 GiB C: free and 10 GiB D: free. Initial samples:
5.92 GiB RAM available, C: 313.34 GiB and D: 535.79 GiB free; after layout validation
5.82 GiB RAM, C: 313.37 GiB and D: 535.78 GiB free. These are sampled values, not peaks.
Local commits only; existing branch; no worktrees, push, production or database changes.

## Checkpoint 12b: Arabic discretionary ligatures

The name mismatch is a shaping feature difference, not incorrect source data or an
Arabic font fallback. The native embedded outline equals installed Arial Bold glyph
3869 (presentation-form cmap U+FDF4). Its GSUB `dlig` lookup 21 invokes lookup 75 to
replace four component glyphs with that one ligature. The old Rust output paints
three additional glyphs at each of the three name occurrences. Native outline identity,
font hashes and lookup evidence are retained privately in
`arabic-ligature-font-evidence.json`. Ordinary Uniscribe and HarfRust defaults do not
enable this optional feature; this is a measured legacy compatibility policy.

New `with_legacy_arabic_ligatures()` measurement methods and
`PdfOptions::legacy_arabic_ligatures` enable font-defined `dlig` on Arabic-script byte
ranges only. The STC worker enables it consistently for printer fitting, wrapping,
final alignment measurement and PDF shaping. The ordinary CLI/default options leave
it off. Following inherited marks/join controls stay in an Arabic span; preceding
Latin combining accents, punctuation, spaces, Latin, Hebrew and CJK are excluded.
Original logical strings and UTF-8 cluster mappings are preserved. No name, report
identifier, replacement presentation-form text or proprietary font is hardcoded.

Seven new synthetic tests exercise an in-memory GSUB table built from bundled open
DejaVu. It deliberately offers both Arabic and Latin discretionary substitutions,
so a mistakenly global `dlig` switch fails the guards. Tests cover actual changed
glyph counts/advances, alignment width, cluster ranges after multibyte prefixes,
per-scalar spacing, mixed direction, non-Arabic/default behavior and serialized PDF
ToUnicode mappings. The actual Chinese printer-profile tests enable the new option.

Validation: **125 text/PDF library tests, seven Chinese PDF tests, twelve worker
checks and 24 Python audit checks pass**, in addition to the 374 layout tests (542
total focused checks). The candidate frozen-data pair passes all gates: three pages,
36 ordered ratings (15 / 16 / 5), 4,748/4,748 painted glyphs, no unmatched outlines,
exact decoded image RGB and equal boxes/dates. Maximum X/Y displacement is
0.05708 / 0.03016 pt, below existing 0.061 / 0.031 limits. Independent review of all
six page images accepts the restored heading, body position, name and final content.
Final clean-source pair and prior-case guards are the next checkpoint; candidate
results alone do not replace the published batch acceptance ledger.

Build dependency changes add only direct/dev edges to already pinned unicode-script
and ttf-parser packages; no package versions or downloads changed. A full offline
lock regeneration was refused by an unrelated cached yanked dependency and made no
changes. The missing dev dependency edge was corrected in the existing lock and all
subsequent builds/tests pass with `--locked --offline`; preserve the existing lock.

```powershell
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('test','--locked','--offline','-p','rpt-text','-p','rpt-render-pdf','--lib')
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('test','--locked','--offline','-p','rpt-render','--test','chinese')
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('test','--locked','--offline','-p','stc-render-worker')
```
Strict `cargo clippy --locked --offline --workspace --all-targets -- -D warnings`,
workspace formatting check and final worker build also pass in bounded Docker.
Independent source review of the pagination commit found no blocking issue.

## Checkpoint 12c: final paired acceptance and regression replays

Final native Crystal and Docker Rust PDFs were regenerated from the retained typed
snapshot with clean source `8f9fd5e` (pagination fix `de290a0`). No SQL was executed.
The private `final/evidence.json` records source cleanliness, snapshot/template/native
bridge/worker hashes, clock and execution times; `comparison.json` records PDF hashes.
The original failed snapshot and outputs remain unchanged in the batch03 folder.

| Final CR-001-B03 check | Result |
| --- | --- |
| Pages and boxes | 3 each, equal |
| PDF integrity | qpdf passes without repair warnings |
| Ordered/source ratings | 36, allocated 15 / 16 / 5, all accounted for |
| Painted glyphs per page | 1,778 / 1,852 / 1,118 in each engine |
| Missing/extra painted outlines | 0 / 0 |
| Maximum glyph X / Y residual | 0.05708 / 0.03016 pt |
| Page-two rating baseline delta | -0.00004 to +0.00001 pt, previously about -21.5 pt |
| All decoded image RGB | Exact equality, including the logo |
| Printed date and final signatures | Match |
| Visual review | All six final PNGs reviewed; accepted |

The repeated bilingual heading, table/body origins, Arabic name field and both repeated
name footers now agree. No clipping, missing row or new wrapping discrepancy is visible.
Independent candidate all-page review also passed; every final page PNG is byte-identical
to its same-engine candidate page, and the coordinator reviewed all six final pages again.
The visual acceptance is explicit in `final/review/visual-review.json`, distinct from
original execution-time pending fields. Forty-seven known outline-equivalent Unicode
mapping differences remain; raw extracted text, native/Rust PDF byte equality and
whole-page pixel identity are not claimed. The existing 0.061 pt X / 0.031 pt Y limits
were not widened. Synthetic ToUnicode tests separately protect the logical new ligature.

Previously accepted cases were replayed sequentially using their own frozen data and
retained original Crystal PDFs. All gates passed, and each new Rust PDF is byte-for-byte
identical to its previously accepted Rust export, preserving the earlier all-page review:

| Guard | Pages | Ratings | Painted glyphs | Rust PDF |
| --- | ---: | ---: | ---: | --- |
| CR-001-B02 sparse continuation | 2 | 16 | 2,725 | Byte-identical |
| CR-003-B03 continuation/Arabic | 3 | 50 | 6,474 | Byte-identical |
| CR-005-B03 Latin name/logo | 3 | 35 | 4,960 | Byte-identical |

These add no new students. Coverage is **12 accepted / 0 rejected / 86 untested** of
98 eligible candidates. Populated comments, overflowing displayed names, blocked
standard/second-term/other-school reports, BIFF/XLS, wider legacy/subreport behavior
and authenticated Parent API/UI delivery retain their earlier gaps. This is captured-case
acceptance, not a production deployment or full bridge-compatibility claim.

Private reproduction uses `research/stc-verification/cr-001-b03-repair/render.ps1`
with a unique `-Tag`, `-Native` for a fresh pair, then `verify.ps1 -Tag <tag>`. Its default
input is the retained CR-001-B03 snapshot. Prior-case guards pass `-SourceRun` and a
retained `-NativeReference`; they do not claim fresh native or live database execution.
`guard-summary.json` records byte comparisons. Request files are removed after each
export. `final/comparison.html` provides the PDF links and per-page visual comparison.

Final sampled resources at **17:04:41**: C: **313.14 GiB free**, D: **534.41 GiB free**,
RAM **6.24 GiB available**. Lowest observed RAM in this repair was 5.21 GiB; these are
samples rather than continuous peaks. D: target/temp retained, no running containers,
old C-backed target volume absent. No source-data/template/production configuration
changes; local commits on existing main only, no branch/worktree/push/deployment.
