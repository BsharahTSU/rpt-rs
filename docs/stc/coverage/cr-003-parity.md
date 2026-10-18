# CR-003 repair and PDF comparison

## Scope and current checkpoint

CR-003 is the second representative evaluation template in the local Parent Crystal
inventory. This repair uses its unchanged original template and previously captured,
schema-bearing DataSet. No SQL is executed. Native Crystal runs on Windows; Rust
builds, tests and exports run in Linux Docker. Parent code, publication, configuration
and deployment are unchanged. All work stays on the existing branch with local commits.

**Accepted for this captured case within the measured tolerances below.** Both
engines were freshly regenerated from clean local implementation commit `899ca64`;
final independent all-page/operator review passes without blockers. [Whitespace findings and fixes](cr-003-whitespace.md)
owns the controlled
native experiments and exact implementation scope.

## Findings and implementation

- The English question's leading U+0020 did not consume Rust wrapping width. Native
  zero/one/two-prefix controls prove that it must. The complete LTR paragraph now
  participates in fitting; the existing RTL prefix policy is retained using actual
  paragraph base direction, rather than the presence of any RTL glyph.
- Close English fit boundaries require font-provided hinted advances. Native GDI
  matches 570 hdmx entries exactly. The portable path reuses Cosmic line layout with
  those advances only for unkerned printable ASCII and an exact available ppem record;
  all other paths fall back. PDF/nominal metrics remain unchanged.
- A terminal LRM/space sequence created an extra line in the first candidate. A native
  one-cell control proves unchanged visible output when that suffix is removed. LTR
  fitting now separates and restores the suffix without adding a phantom line.
- Two centered RTL headings were displaced about 1.10 pt by a terminal U+0020.
  Native zero/one/two-suffix controls produce identical visible glyphs and no terminal
  space. Centered RTL fields now use the established opt-in centered-field preparation,
  including late page-count/currency rewrites. Other alignments and static text retain
  their previous policies. Unknown initial direction is guarded; the older Boolean-only
  late numeric fixup behavior is not newly guaranteed.
- A new mixed-language test exposed pre-existing character loss in wrapped text.
  Reconstruction now uses the logical extent of every glyph cluster instead of only
  the first/last visual glyph. This changes neither shaping nor break decisions.

No report-specific string, object-name check, arbitrary font scaling, width fudge or
hard-coded translation was added. Controlled copies and raw field data remain private.

## Verification

Final source verification passes **479 focused Docker tests**: 39 text, 371 layout,
57 render (including six documentation tests) and 12 worker tests. Thirteen
minimal-feature text tests, strict all-target/all-feature workspace Clippy,
format checking and the committed-source worker build also pass. Independent
source review found no blocking issue. No public baseline was updated and no
live database was configured.

The render checks include six Chinese tests: Simplified/Traditional leading-prefix
coverage plus existing Chinese PDF extraction/fallback checks. The unchanged public
PDF reference net also passes. Four additional integration tests construct synthetic
hdmx records in a bundled open font entirely in memory, covering actual glyph-ID
fitting, unchanged nominal metrics/caches, opt-in/script/ppem guards and malformed
record fallback. No proprietary font fixture is added.

Both original compatibility regressions failed before their fixes. The first
implementation passed 473 focused and 12 minimal tests, but its all-page candidate
audit still caught two wrapping regressions. That rejected evidence is retained in
`candidate/review`. Native GDI/table and terminal-tail probes established the final
correction; final tests and fresh-pair comparison pass as recorded here.

The earlier CR-001 full 1,630-test checkpoint remains historical evidence; it is not
added to these counts or represented as a full workspace rerun for CR-003.

## Evidence and reproduction

Use the isolated-case contract in [case-runner.md](case-runner.md). The original
case snapshot remains under ignored `research/stc-verification/parent-cases/cr-003`.
Private replay scripts, native controls, failed/passing test logs, PDF operator
measurements, comparisons and PNGs are under `research/stc-verification/cr-003-parity`.
The baseline was regenerated from clean `339404c` on 2026-10-17. Native/Rust inputs
share the frozen snapshot; output dates are held consistent by the existing adapter.

Keep private manifests, report copies, PDFs, personal data and credentials out of Git.
Public documentation uses opaque case IDs. Raw extracted-word equality, glyph/geometry
agreement and visual acceptance must be assessed separately because the engines can
encode identical glyph outlines with different Unicode mappings.

## Resource controls

Temporary files use `D:\tmpppppp123\rpt-rs-cr003`; ignored evidence also resides on D:.
Containers run sequentially with 2 GiB RAM, no additional swap, two CPUs, one Cargo
build job and one Rust test thread. The runner checks at least 30 GiB free on C:,
10 GiB on D: and 2 GiB available host RAM before starting a heavy job. Existing Docker
images, registry and target volumes are reused; their managed storage was not moved.

Initial readings were C: 283.01 GiB free, D: 561.48 GiB free and available RAM 5.06 GiB.
After compilation RAM reached 2.52 GiB. With no running container, releasing idle
Linux filesystem cache recovered it to 5.07 GiB; WSL working set fell from 3.47 to
0.66 GiB. C: remained 282.98 GiB free. No files, volumes or user processes were removed,
and no global WSL settings were changed. Final resource readings follow acceptance.

## Final comparison and CR-001 regression

The corrected candidate and fresh final pair pass independent review of all six page
PNGs, all 6,449 nonblank glyph outlines/sizes, 377 text runs (279 aligned runs),
72 English lines (23/26/23) and 52 ordered ratings (18/21/13). No run is excluded.
The target English break, previously regressed short word and terminal-LRM row
height now agree with native. Both RTL headings' maximum X errors fall from about
1.11 pt to 0.02212 and 0.01148 pt. The other previously matched runs do not regress.

| Final measurement | Maximum native/Rust difference |
| --- | ---: |
| All glyph X origins | 0.06008 pt |
| English glyph X origins | 0.03265 pt |
| Ordinary text baselines | 0.00005 pt |
| Rating baselines | 0.00004 pt |
| Arabic mark Y rounding | 0.03016 pt |
| Seven stroked rounded-frame bounds | 0.00005 pt |
| Six filled rounded-header bounds | Exact |
| 180 horizontal path endpoints | 0.05004 pt |

Eighty-nine glyph pairs have different Unicode mappings but identical embedded
outlines and sizes. Raw extracted-word equality remains false; these are measured
geometry/visual results, not byte or pixel identity. The captured-case envelope is
below 0.061 pt X, 0.031 pt mark Y and 0.051 pt path rounding. CR-001's narrower
historical envelope is not silently broadened.

The latest worker also replays CR-001 with its original frozen input/date. Rust PDF,
Page IR and all three PNGs are byte-identical to the accepted CR-001 final outputs.
Private `cr-one-hinted-regression/` records that compatibility check. This does not
establish other students, CR-005, standard reports, Excel, template-owned SQL,
subreports, authenticated API/UI delivery or production readiness.

## Final regeneration and local commits

Both original Windows Crystal and Linux Docker Rust were freshly generated at
19:02:21-19:02:47 local on 2026-10-17 from clean `899ca64`. Both pass PDF integrity
checks, contain three 595 x 841 pt pages, share the print date and preserve all
52 ordered ratings. Original template and frozen snapshot SHA-256 values are
unchanged; source was clean and no SQL was executed.

The coordinator and independent reviewer inspected all six fresh PNGs. Full
operator/outline analysis without exclusions reproduces every measurement above.
Each engine's fresh PDF and PNG hashes equal its own reviewed candidate; Rust IR,
run geometry and path audit also match. This is not equality between the two
engines' PDF bytes or raster pixels. There are no unresolved layout defects in
this captured pair within the documented envelope.

Final private evidence beneath `research/stc-verification/cr-003-parity/final/`:

- `native.pdf`, `rust.pdf`, `comparison.html`, `comparison.json` and six page PNGs.
- `evidence.json`: clean source commit, timestamps, snapshot hash and no-SQL replay.
- `engine-hashes.json`: source/PDF/template/bridge/worker/image hashes.
- `review/final-review.md`, `final-review-summary.json`, `candidate-identity.json`,
  all-glyph positions and run/English/rating/path measurements.
- `resources.json`: final resource checkpoint and absence of running containers.

At 19:03:43, C: had 282.98 GiB free, D: 561.41 GiB free, available host RAM was
4.28 GiB and WSL working set was 2.21 GiB. No container remained. Initial C: free
space was 283.01 GiB. Idle cache was released only between completed stages;
temporary files/evidence remained on D: and existing Docker storage was reused.

Implementation and initial findings are committed locally as `899ca64`. The final
acceptance ledger/index updates form a separate local documentation checkpoint;
canonical backend changes contain documentation only. No branch, worktree, hosted
fork, push, application deployment, report change or database mutation was made.

## Image residual update (2026-10-18)

The [CR-005 image investigation](cr-005-parity.md#shared-logo-residual) finds the
same native/Rust decoded logo-color difference in this accepted pair. Rust exactly
preserves the stored RGB bitmap; native Crystal reduces it to an indexed palette.
The earlier text/layout and geometry measurements remain valid. This newly measured
residual extends the acceptance record; exact image-color parity remains open.


## Shared image repair replay (2026-10-18)

The subsequent [generic legacy bitmap fix](cr-005-logo.md), from clean `eb4656d`,
closes the decoded-logo color residual in this captured case. A fresh Docker Rust
replay against the original accepted native reference matches all 9,600 logo RGB
pixels exactly. Rust drawing streams, font resources, other raster pixels and
Page IR remain identical to the accepted pre-fix output. PDF integrity, page boxes,
ratings and date checks pass. Prior text/layout tolerances remain unchanged.
No SQL or new native export was needed for this historical replay.
