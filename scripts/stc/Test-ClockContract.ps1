# Pure launcher contract checks. Docker is mocked; no files, containers or SQL are created.
[CmdletBinding()]
param()
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$launcher = Join-Path $PSScriptRoot 'Invoke-DockerBridge.ps1'
$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
$previousProbe = Get-Variable -Name StcClockContractProbe -Scope Global -ErrorAction SilentlyContinue
$global:StcClockContractProbe = [pscustomobject]@{ Calls = 0; Captured = @() }
function docker {
    $global:StcClockContractProbe.Calls++
    if ($args[0] -eq 'info') { return 'linux' }
    $global:StcClockContractProbe.Captured = @($args | Where-Object { $_ -like 'STC_RPT_LOCAL_DATETIME=*' })
    $global:LASTEXITCODE = 0
}
try {
    foreach ($clock in @('', '2026-02-29T00:00:00', '2026-10-16T24:00:00', '2026-10-16T00:00:00Z', '2026-10-16T00:00:00+03:00')) {
        $rejected = $false
        try { & $launcher -AsOfLocal $clock } catch { $rejected = $_.Exception.Message -like 'Invalid AsOfLocal*' }
        if (-not $rejected) { throw 'Expected malformed clock rejection.' }
    }
    if ($global:StcClockContractProbe.Calls -ne 0) { throw 'A malformed clock reached Docker.' }
    # The mock intercepts these paths; they need not exist and no files are written.
    $work = Join-Path $repoRoot 'research/stc-verification/clock-test-placeholder'
    & $launcher -RequestFile (Join-Path $work 'request.json') -WorkDirectory $work -TemplateRoot $repoRoot -AsOfLocal '2026-10-16T00:15:07'
    if ($global:StcClockContractProbe.Captured.Count -ne 1 -or
        $global:StcClockContractProbe.Captured[0] -cne 'STC_RPT_LOCAL_DATETIME=2026-10-16T00:15:07') {
        throw 'The explicit clock was not forwarded exactly.'
    }
    $before = [DateTime]::Now.AddSeconds(-1)
    & $launcher -RequestFile (Join-Path $work 'request.json') -WorkDirectory $work -TemplateRoot $repoRoot
    $after = [DateTime]::Now
    $captured = [DateTime]::ParseExact(
        $global:StcClockContractProbe.Captured[0].Substring('STC_RPT_LOCAL_DATETIME='.Length),
        "yyyy-MM-dd'T'HH:mm:ss", [Globalization.CultureInfo]::InvariantCulture)
    if ($captured -lt $before -or $captured -gt $after) { throw 'The default clock was not host-local DateTime.Now.' }
    Write-Output 'Launcher clock contract: 7 checks passed (5 malformed, exact override, host-local default).'
} finally {
    if ($null -ne $previousProbe) { $global:StcClockContractProbe = $previousProbe.Value }
    else { Remove-Variable -Name StcClockContractProbe -Scope Global }
}
