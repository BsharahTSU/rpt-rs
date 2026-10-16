[CmdletBinding()]
param(
    [switch]$Build,
    [switch]$Test,
    [string]$RequestFile,
    [string]$WorkDirectory,
    [string]$TemplateRoot,
    [string]$FontDirectory,
    [switch]$DebugPageIr,
    [string]$Image = 'stc-rpt-rs:bridge-dev'
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
if ($RequestFile -and ($Build -or $Test)) { throw 'Run build/test separately so request stdout stays JSON only.' }
if ((& docker info --format '{{.OSType}}') -ne 'linux') { throw 'The Docker Linux engine is required.' }
if ($Build) {
    & (Join-Path $PSScriptRoot 'Invoke-DockerCargo.ps1') -Build -CargoArgs @('build', '--locked', '-p', 'stc-render-worker')
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    & docker run --rm --mount "type=bind,source=$repoRoot,target=/src" --mount 'type=volume,source=stc-rpt-bridge-nuget,target=/root/.nuget/packages' -w /src/bridge/Stc.CrystalBridge mcr.microsoft.com/dotnet/sdk:10.0 dotnet build --locked-mode --nologo
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    & docker build --progress plain -f (Join-Path $repoRoot 'Dockerfile.stc-bridge-dev') -t $Image $repoRoot
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
if ($Test) {
    & docker run --rm --mount "type=bind,source=$repoRoot,target=/src" --mount 'type=volume,source=stc-rpt-bridge-nuget,target=/root/.nuget/packages' -w /src/bridge/Stc.CrystalBridge.Tests mcr.microsoft.com/dotnet/sdk:10.0 dotnet run --nologo
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    & (Join-Path $PSScriptRoot 'Invoke-DockerCargo.ps1') -CargoArgs @('test', '--locked', '-p', 'stc-render-worker')
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
if ($RequestFile) {
    if (-not $WorkDirectory -or -not $TemplateRoot) { throw 'WorkDirectory and TemplateRoot are required for a request.' }
    $workRoot = [IO.Path]::GetFullPath($WorkDirectory)
    $requestPath = [IO.Path]::GetFullPath($RequestFile)
    if ([IO.Path]::GetDirectoryName($requestPath) -ne $workRoot) { throw 'The request must be directly inside WorkDirectory.' }
    $dockerArgs = @('run', '--rm',
        '--mount', "type=bind,source=$repoRoot,target=/src,readonly",
        '--mount', "type=bind,source=$workRoot,target=/work",
        '--mount', "type=bind,source=$([IO.Path]::GetFullPath($TemplateRoot)),target=/reports,readonly",
        '--mount', 'type=volume,source=stc-rpt-rs-target,target=/target,readonly',
        '-e', 'PARENT_REPORTS_CRYSTAL_TEMP_ROOT=/work', '-e', 'PARENT_REPORTS_CRYSTAL_TEMPLATE_ROOT=/reports',
        '-e', 'STC_RPT_WORKER=/target/debug/stc-rpt-worker')
    $culture = Get-Culture
    $dockerArgs += @('-e', ('STC_RPT_LOCALE=' + $culture.Name), '-e', ('STC_RPT_SHORT_DATE=' + $culture.DateTimeFormat.ShortDatePattern))
    if ($FontDirectory) { $dockerArgs += @('--mount', "type=bind,source=$([IO.Path]::GetFullPath($FontDirectory)),target=/usr/local/share/fonts/windows,readonly") }
    if ($DebugPageIr) { $dockerArgs += @('-e', 'STC_RPT_DEBUG_IR=/work/rust-page-ir.json') }
    $dockerArgs += @($Image, 'dotnet', '/src/bridge/Stc.CrystalBridge/bin/Debug/net10.0/stc-crystal-bridge.dll', ('/work/' + [IO.Path]::GetFileName($requestPath)))
    & docker @dockerArgs
    exit $LASTEXITCODE
}