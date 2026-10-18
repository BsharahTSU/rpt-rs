# CR-001-B03 continuation-header and Arabic shaping repair

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
