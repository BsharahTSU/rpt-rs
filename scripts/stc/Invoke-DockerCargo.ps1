[CmdletBinding()]
param(
    [switch]$Build,
    [string]$Image = 'stc-rpt-rs:dev',
    [string[]]$CargoArgs = @('test', '--locked', '--workspace')
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
if ((& docker info --format '{{.OSType}}') -ne 'linux') {
    throw 'The Docker Linux engine is required.'
}
if ($Build) {
    & docker build --progress plain -f (Join-Path $repoRoot 'Dockerfile.stc-dev') -t $Image $repoRoot
    if ($LASTEXITCODE -ne 0) { throw 'Linux development image build failed.' }
}
$dockerArgs = @(
    'run', '--rm',
    '--mount', "type=bind,source=$repoRoot,target=/src,readonly",
    '--mount', 'type=volume,source=stc-rpt-rs-target,target=/target',
    '--mount', 'type=volume,source=stc-rpt-rs-cargo-registry,target=/usr/local/cargo/registry',
    $Image, 'cargo'
) + $CargoArgs
& docker @dockerArgs
exit $LASTEXITCODE
