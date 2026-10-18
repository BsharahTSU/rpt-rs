[CmdletBinding()]
param(
    [switch]$Build,
    [switch]$Test,
    [string]$RequestFile,
    [string]$WorkDirectory,
    [string]$TemplateRoot,
    [string]$FontDirectory,
    [switch]$DebugPageIr,
    [string]$AsOfLocal,
    [ValidateRange(72, 9600)][int]$PrinterDpi,
    [string]$Image = 'stc-rpt-rs:bridge-dev',
    [string]$CacheRoot
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
if ($RequestFile -and ($Build -or $Test)) { throw 'Run build/test separately so request stdout stays JSON only.' }
# Reject an explicit malformed replay clock before Docker or SQL work. The normal request
# captures DateTime.Now in the caller's zone, independently of the container's clock zone.
$clockFormat = "yyyy-MM-dd'T'HH:mm:ss"
$hasClockOverride = $PSBoundParameters.ContainsKey('AsOfLocal')
if ($hasClockOverride) {
    $parsedClock = [DateTime]::MinValue
    if (-not [DateTime]::TryParseExact($AsOfLocal, $clockFormat, [Globalization.CultureInfo]::InvariantCulture,
            [Globalization.DateTimeStyles]::None, [ref]$parsedClock) -or
        $parsedClock.ToString($clockFormat, [Globalization.CultureInfo]::InvariantCulture) -cne $AsOfLocal) {
        throw 'Invalid AsOfLocal; expected yyyy-MM-ddTHH:mm:ss without an offset.'
    }
}
. (Join-Path $PSScriptRoot 'DockerStorage.ps1')
$storage = Get-StcDockerStorage -CacheRoot $CacheRoot
if ((& docker info --format '{{.OSType}}') -ne 'linux') { throw 'The Docker Linux engine is required.' }
if ($Build) {
    & (Join-Path $PSScriptRoot 'Invoke-DockerCargo.ps1') -CacheRoot $storage.Root -Build -CargoArgs @('build', '--locked', '-p', 'stc-render-worker')
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    & docker run --rm --mount "type=bind,source=$repoRoot,target=/src" --mount 'type=volume,source=stc-rpt-bridge-nuget,target=/root/.nuget/packages' -w /src/bridge/Stc.CrystalBridge mcr.microsoft.com/dotnet/sdk:10.0 dotnet build --locked-mode --nologo
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    & docker build --progress plain -f (Join-Path $repoRoot 'Dockerfile.stc-bridge-dev') -t $Image $repoRoot
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
if ($Test) {
    & (Join-Path $PSScriptRoot 'Test-DockerStorageContract.ps1')
    & (Join-Path $PSScriptRoot 'Test-ClockContract.ps1')
    & (Join-Path $PSScriptRoot 'Test-PrinterMetricsContract.ps1')
    & docker run --rm --mount "type=bind,source=$repoRoot,target=/src" --mount 'type=volume,source=stc-rpt-bridge-nuget,target=/root/.nuget/packages' -w /src/bridge/Stc.CrystalBridge.Tests mcr.microsoft.com/dotnet/sdk:10.0 dotnet run --nologo
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    & (Join-Path $PSScriptRoot 'Invoke-DockerCargo.ps1') -CacheRoot $storage.Root -CargoArgs @('test', '--locked', '-p', 'stc-render-worker')
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
if ($RequestFile) {
    if (-not $WorkDirectory -or -not $TemplateRoot) { throw 'WorkDirectory and TemplateRoot are required for a request.' }
    $workRoot = [IO.Path]::GetFullPath($WorkDirectory)
    $requestPath = [IO.Path]::GetFullPath($RequestFile)
    if ([IO.Path]::GetDirectoryName($requestPath) -ne $workRoot) { throw 'The request must be directly inside WorkDirectory.' }
    $dockerArgs = @('run', '--rm', '--memory', '2g', '--memory-swap', '2g', '--cpus', '2',
        '--mount', $storage.TempMount,
        '--mount', "type=bind,source=$repoRoot,target=/src,readonly",
        '--mount', "type=bind,source=$workRoot,target=/work",
        '--mount', "type=bind,source=$([IO.Path]::GetFullPath($TemplateRoot)),target=/reports,readonly",
        '--mount', $storage.TargetReadOnlyMount,
        '-e', 'PARENT_REPORTS_CRYSTAL_TEMP_ROOT=/work', '-e', 'PARENT_REPORTS_CRYSTAL_TEMPLATE_ROOT=/reports',
        '-e', 'STC_RPT_WORKER=/target/debug/stc-rpt-worker')
    $localClock = if ($hasClockOverride) { $AsOfLocal } else { [DateTime]::Now.ToString($clockFormat, [Globalization.CultureInfo]::InvariantCulture) }
    $dockerArgs += @('-e', ('STC_RPT_LOCAL_DATETIME=' + $localClock))
    if ($PSBoundParameters.ContainsKey('PrinterDpi')) { $dockerArgs += @('-e', ('STC_RPT_PRINTER_DPI=' + $PrinterDpi)) }
    $culture = Get-Culture
    $dockerArgs += @('-e', ('STC_RPT_LOCALE=' + $culture.Name), '-e', ('STC_RPT_SHORT_DATE=' + $culture.DateTimeFormat.ShortDatePattern))
    if ($FontDirectory) { $dockerArgs += @('--mount', "type=bind,source=$([IO.Path]::GetFullPath($FontDirectory)),target=/usr/local/share/fonts/windows,readonly") }
    if ($DebugPageIr) { $dockerArgs += @('-e', 'STC_RPT_DEBUG_IR=/work/rust-page-ir.json') }
    $dockerArgs += @($Image, 'dotnet', '/src/bridge/Stc.CrystalBridge/bin/Debug/net10.0/stc-crystal-bridge.dll', ('/work/' + [IO.Path]::GetFileName($requestPath)))
    & docker @dockerArgs
    exit $LASTEXITCODE
}