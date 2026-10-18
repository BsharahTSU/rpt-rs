# Docker Rust target cache on D:

## Completed 2026-10-18

The Rust target cache now lives on D:. The retired `stc-rpt-rs-target` volume is
removed. All three captured reports print from the D: bind mount before removal,
after removal and after Docker disk compaction/restart, with identical PDF and Page
IR bytes to their accepted Rust references. Administrator-approved compaction
recovered **27.18 GiB of actual C: space**; Docker is running normally.

## Migration and ownership

The user requested moving the approximately 26.6 GB `stc-rpt-rs-target` Docker volume
out of C: while preserving work and proving printing still works. Source code, local
Git history, original templates, frozen inputs and prior PDFs remain in their existing
locations. Only the Rust target cache was relocated; unrelated Docker images,
BuildKit cache, NuGet and Cargo download volumes are not selected for deletion.

The old volume lived inside the Docker Desktop WSL data disk on C:. Inventory found
47,982 regular files, 2,389 directories including the root, no symlinks, 28,534,344,866
logical bytes and 27,106,483,279 unique file bytes. The difference is shared hardlinks.
Most logical bytes are dependency/test artifacts (18.28 GB) and incremental data
(9.63 GB). The original volume was mounted read-only during copying.

Destination: `D:\tmpppppp123\docker-cache\rpt-rs-target`.
Container path remains `/target`, so compiled worker lookup and Cargo paths stay stable.
Temporary container files use `D:\tmpppppp123\docker-cache\tmp` at `/tmp`.
This folder is now persistent build storage despite its parent's temporary-looking name;
do not remove it as routine scratch cleanup while it is in use.

The migration preserved file contents, modes and hardlinks. Original timestamps were
requested, but the Windows bind mount rounded them by less than one second (maximum
999,911,034 ns). Every copied file was reopened on D: and its SHA-256 checked against
the source manifest; every file path, size, mode and hardlink destination was accounted
for. The source volume remained intact until verification and build/print checks passed.
Private inventories,
copy/hash results and PDF replay evidence live in ignored
`research/stc-verification/docker-cache-migration/`.

## Launcher contract

`DockerStorage.ps1` is the shared path resolver for:

- `Invoke-DockerCargo.ps1`: writable target and temporary bind mounts, one Cargo/test
  thread, two CPUs and 2 GiB memory/swap-total cap.
- `Invoke-DockerBridge.ps1`: read-only target for printing, D: temporary mount, matching
  resource caps, unchanged worker path, culture and replay-clock behavior.
- `Invoke-ParentCrystalCase.ps1`: the same target for artifact probes and actual exports.

All eight existing private build/replay helpers were also updated so rerunning an old
comparison does not silently recreate the retired named volume. Existing PDF evidence
and captured inputs are not modified.

Default cache root is `D:\tmpppppp123\docker-cache`. For a different machine, supply
`-CacheRoot <absolute-directory>` or set `STC_RPT_DOCKER_CACHE_ROOT`; the explicit argument
wins. The resolver rejects relative paths and mount-string delimiters. Printing uses
`--mount type=bind`, which fails for a missing destination instead of creating an empty
named volume. The build launcher creates the required host directories.

A Windows bind mount may have more metadata overhead than a Linux filesystem, as
[Docker's WSL guidance](https://docs.docker.com/desktop/features/wsl/best-practices/)
explains. This choice moves only the requested cache and leaves Docker's other storage
and running environment alone. See [Docker bind mount semantics](https://docs.docker.com/engine/storage/bind-mounts/).

## Verification and completion gates

- Storage contracts test D: defaults, explicit/environment precedence, six invalid path
  forms, read-only printing, D: temp, worker lookup and resource limits. Existing clock
  and printer contract checks remain passing.
- Complete copy SHA-256 verification before invoking Cargo on D:.
- Build the worker in Docker using the shared launcher and exercise focused regressions.
- Print frozen CR-001/003/005 with the new target and compare PDF/Page IR hashes to the
  accepted previous outputs. No live SQL is required for this storage-only change.
- Remove only the exact old target volume after those checks pass, then print again
  without it and verify no launcher recreated it.
- Record C:/D: free space, Docker storage and RAM. Deleting a Docker volume frees space
  inside its disk; Windows free-space recovery must be measured, not inferred from the
  volume's logical size. No blanket prune or deletion of unrelated Docker resources.

The initial checkpoint (`c2e9221`) intentionally retained the old volume until the
following completion evidence was available.

## Verification results

- Copied 47,982 files / 35,251 unique inodes, preserving 12,731 extra hardlinks and
  27,106,483,279 unique bytes. Independent verification found zero content, size,
  mode or hardlink mismatches. The full copy took 583 seconds; an independent
  eight-reader, read-only verification took 438 seconds under a 512 MiB cap.
- The first sequential verifier was too slow for the Windows mount. Its owned
  container was stopped only after the complete copy and manifest existed. The
  replacement verifier checked the complete destination; no files were skipped.
- The actual shared Cargo launcher built the worker offline in 5.50 seconds, using
  the copied cache. The worker SHA-256 matches the previously accepted build.
- **50 focused Docker tests passed:** 29 render-utility, 12 worker, seven Chinese
  PDF tests and two legacy-bitmap PDF tests. Some test binaries relinked; no Rust
  source changed. Storage contracts passed in PowerShell 7 and Windows PowerShell
  5.1, as did seven replay-clock and five printer contracts.
- The actual shared bridge launcher printed CR-001/003/005 three times: before
  removing the volume, after removing it, and after compaction/restart. All nine
  PDF/Page IR pairs match the accepted Rust references byte-for-byte. A live
  container inspection confirmed `/target` is the D: bind mount with `RW=false`.
- Final qpdf checks pass for all three three-page PDFs. All three CR-005 PNGs also
  equal the accepted raster bytes, and every page was visually inspected. Native
  Crystal references and captured inputs were unchanged; no live SQL was needed.
  This verifies storage migration, not additional template or Parent API coverage.
- A private replay helper initially encountered PowerShell 7's JSON date coercion
  before starting an export. It now formats the replay clock explicitly using
  invariant ISO local time. That rejected attempt is retained separately.
- Only the exact old target volume was removed after verification and successful
  prints, with no attached container. It was not recreated by any later print.
  Cargo download and NuGet volumes remain. No blanket Docker prune was used.

## Windows space recovery and resources

Deleting the volume freed space inside Docker's existing virtual disk but did not
immediately return it to C:. After `sync` and `fstrim` of Docker's own data mount,
Windows free space was still about 284.8 GiB. The 62.5 GiB reported as trimmed was
an internal discard measurement, not a claim of Windows disk space recovered.

The user explicitly approved an administrator prompt for a brief Docker stop,
compaction and restart. The first elevated helper safely rejected its verification
guard before changing the disk: Windows PowerShell 5.1 wrapped the converted JSON
array differently from PowerShell 7. Assigning the converted value before wrapping
it fixed the guard; an independent Windows PowerShell 5.1 check confirmed all three
successful print records before retrying. Docker restarted after both attempts.

The retry verified an unexpired gate, the full-copy result, three post-removal print
results, and that the exact Docker data disk was detached and dynamic. It ran
`Optimize-VHD -Mode Quick` on the existing Docker data disk only, then restarted
Docker in the parent script's `finally` block. On a detached disk Quick uses the
pretrimmed behavior documented by [Microsoft](https://learn.microsoft.com/en-us/powershell/module/hyper-v/optimize-vhd?view=windowsserver2022-ps).

| Measurement | Before compaction | After compaction |
| --- | ---: | ---: |
| Docker VHD file bytes | 91,886,714,880 | 62,701,699,072 |
| Windows C: available bytes | 305,873,059,840 | 335,058,079,744 |
| Actual recovered C: space | | 29,185,019,904 bytes / 27.18 GiB |

Immediately after compaction C: had about 312.0 GiB free. The final health snapshot
at 12:32:59 recorded C: 313.80 GiB, D: 535.88 GiB free and RAM 5.32 GiB available.
The later system-wide change is separate from the measured compaction recovery. During the migration/tests the lowest observed available
host RAM was 3.63 GiB. Copy/check containers were capped at 1 GiB/512 MiB; builds
and printing at 2 GiB, two CPUs and one Cargo/test thread. No global WSL memory
configuration was changed. Docker images and BuildKit storage still use Docker's
C: disk; future Rust target and Cargo/print container temporary writes use the D: folders.

## Reuse and recovery

Keep `D:\tmpppppp123\docker-cache` as persistent storage. Printing requires the built
worker and bridge artifacts at the expected target paths. To rebuild the Rust worker
from the preserved source using this cache:

```powershell
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('build', '--locked', '-p', 'stc-render-worker')
```

For a full bridge rebuild use `./scripts/stc/Invoke-DockerBridge.ps1 -Build -Test`.
If the cache is intentionally replaced, rebuild before printing; there is no need to
recreate the retired named volume. On another machine use the explicit `-CacheRoot`
option or `STC_RPT_DOCKER_CACHE_ROOT` environment variable consistently for builds
and printing. Source, local commits, templates, frozen inputs and prior outputs are
independent of this cache and remain in their existing locations.

Private evidence includes `copy-result.json`, before/after-removal print records,
`live-print-mounts.json`, `removal.json`, both compaction results and
`after-compaction/integrity.json`. Failed attempts are retained alongside successful
results. No private reports, inputs, credentials or student details are committed.
