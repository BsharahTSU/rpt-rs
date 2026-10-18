[CmdletBinding()]
param(
    [switch]$Build,
    [string]$Image = 'stc-rpt-rs:dev',
    [string[]]$CargoArgs = @('test', '--locked', '--workspace'),
    [string]$CacheRoot
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
. (Join-Path $PSScriptRoot 'DockerStorage.ps1')
$storage = Get-StcDockerStorage -CacheRoot $CacheRoot
if ((& docker info --format '{{.OSType}}') -ne 'linux') {
    throw 'The Docker Linux engine is required.'
}
if ($Build) {
    & docker build --progress plain -f (Join-Path $repoRoot 'Dockerfile.stc-dev') -t $Image $repoRoot
    if ($LASTEXITCODE -ne 0) { throw 'Linux development image build failed.' }
}
foreach ($directory in @($storage.TargetDirectory, $storage.TempDirectory)) {
    [void][IO.Directory]::CreateDirectory($directory)
}
$dockerArgs = @(
    'run', '--rm', '--memory', '2g', '--memory-swap', '2g', '--cpus', '2',
    '--mount', "type=bind,source=$repoRoot,target=/src,readonly",
    '--mount', $storage.TargetMount,
    '--mount', 'type=volume,source=stc-rpt-rs-cargo-registry,target=/usr/local/cargo/registry',
    '--mount', $storage.TempMount,
    '-e', 'CARGO_TARGET_DIR=/target', '-e', 'CARGO_BUILD_JOBS=1', '-e', 'RUST_TEST_THREADS=1',
    $Image, 'cargo'
) + $CargoArgs
& docker @dockerArgs
exit $LASTEXITCODE
