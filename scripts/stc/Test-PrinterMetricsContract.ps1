# Pure printer-resolution transport checks. Docker is mocked; no SQL or PDF export.
[CmdletBinding()]
param()
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$launcher = Join-Path $PSScriptRoot 'Invoke-DockerBridge.ps1'
$repoRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
$previousProbe = Get-Variable -Name StcPrinterMetricsProbe -Scope Global -ErrorAction SilentlyContinue
$global:StcPrinterMetricsProbe = [pscustomobject]@{ Calls = 0; Captured = @() }
function docker {
    $global:StcPrinterMetricsProbe.Calls++
    if ($args[0] -eq 'info') { return 'linux' }
    $global:StcPrinterMetricsProbe.Captured = @($args | Where-Object { $_ -like 'STC_RPT_PRINTER_DPI=*' })
    $global:LASTEXITCODE = 0
}
try {
    foreach ($dpi in @(0, 71, 9601)) {
        $rejected = $false
        try { & $launcher -PrinterDpi $dpi } catch { $rejected = $_.Exception.Message -like '*PrinterDpi*' }
        if (-not $rejected) { throw 'Expected invalid printer DPI rejection.' }
    }
    if ($global:StcPrinterMetricsProbe.Calls -ne 0) { throw 'An invalid DPI reached Docker.' }
    $work = Join-Path $repoRoot 'research/stc-verification/printer-test-placeholder'
    & $launcher -RequestFile (Join-Path $work 'request.json') -WorkDirectory $work -TemplateRoot $repoRoot
    if ($global:StcPrinterMetricsProbe.Captured.Count -ne 0) {
        throw 'The launcher must preserve the template DPI when no override is supplied.'
    }
    & $launcher -RequestFile (Join-Path $work 'request.json') -WorkDirectory $work -TemplateRoot $repoRoot -PrinterDpi 1200
    if ($global:StcPrinterMetricsProbe.Captured.Count -ne 1 -or
        $global:StcPrinterMetricsProbe.Captured[0] -cne 'STC_RPT_PRINTER_DPI=1200') {
        throw 'The explicit printer DPI was not forwarded.'
    }
    Write-Output 'Launcher printer metrics: 5 checks passed (3 invalid, default, override).'
} finally {
    if ($null -ne $previousProbe) { $global:StcPrinterMetricsProbe = $previousProbe.Value }
    else { Remove-Variable -Name StcPrinterMetricsProbe -Scope Global }
}
