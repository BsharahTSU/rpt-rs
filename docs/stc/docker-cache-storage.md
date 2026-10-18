# Docker Rust target cache on D:

## Migration and ownership

The user requested moving the approximately 26.6 GB `stc-rpt-rs-target` Docker volume
out of C: while preserving work and proving printing still works. Source code, local
Git history, original templates, frozen inputs and prior PDFs remain in their existing
locations. Only the Rust target cache is being relocated; unrelated Docker images,
BuildKit cache, NuGet and Cargo download volumes are not selected for deletion.

The old volume lives inside the Docker Desktop WSL data disk on C:. Inventory found
47,982 regular files, 2,389 directories including the root, no symlinks, 28,534,344,866
logical bytes and 27,106,483,279 unique file bytes. The difference is shared hardlinks.
Most logical bytes are dependency/test artifacts (18.28 GB) and incremental data
(9.63 GB). The original volume is mounted read-only during copying.

Destination: `D:\tmpppppp123\docker-cache\rpt-rs-target`.
Container path remains `/target`, so compiled worker lookup and Cargo paths stay stable.
Temporary container files use `D:\tmpppppp123\docker-cache\tmp` at `/tmp`.
This folder is now persistent build storage despite its parent's temporary-looking name;
do not remove it as routine scratch cleanup while it is in use.

The migration preserves file contents, modes, timestamps and hardlinks. Every unique
copied file is reopened on D: and its SHA-256 checked against the source manifest; every
file path, size, mode and hardlink destination is accounted for. The source volume must
remain intact until this verification and build/print checks pass. Private inventories,
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

Status at this checkpoint: copying and checksum verification in progress; the original
volume has not been removed. Final measured results will be appended below.