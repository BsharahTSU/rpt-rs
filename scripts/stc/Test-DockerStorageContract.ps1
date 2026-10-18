# Pure storage/launcher contracts: Docker is mocked and no cache directory is created.
[CmdletBinding()]
param()
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'DockerStorage.ps1')
$oldRoot = $env:STC_RPT_DOCKER_CACHE_ROOT
$previousProbe = Get-Variable -Name StcStorageProbe -Scope Global -ErrorAction SilentlyContinue
$global:StcStorageProbe = [pscustomobject]@{ Captured = @() }
function docker {
    if ($args[0] -eq 'info') { return 'linux' }
    $global:StcStorageProbe.Captured = @($args)
    $global:LASTEXITCODE = 0
}
try {
    $env:STC_RPT_DOCKER_CACHE_ROOT = $null
    $storage = Get-StcDockerStorage
    if ($storage.TargetDirectory -cne 'D:\tmpppppp123\docker-cache\rpt-rs-target') { throw 'Default cache escaped the D: destination.' }
    if ($storage.TempDirectory -cne 'D:\tmpppppp123\docker-cache\tmp') { throw 'Default temp escaped the D: destination.' }
    $env:STC_RPT_DOCKER_CACHE_ROOT = 'D:\cache contract\environment'
    if ((Get-StcDockerStorage).Root -cne $env:STC_RPT_DOCKER_CACHE_ROOT) { throw 'Environment override lost.' }
    $explicit = 'D:\cache contract\explicit'
    $storage = Get-StcDockerStorage -CacheRoot $explicit
    if ($storage.Root -cne $explicit) { throw 'Explicit override must win.' }
    foreach ($root in @('relative', 'D:relative', '\rooted', 'D:\comma,path', 'D:\quote"path', "D:\line`nbreak")) {
        $rejected = $false
        try { $null = Get-StcDockerStorage -CacheRoot $root } catch { $rejected = $_.Exception.Message -like 'Docker cache root must*' }
        if (-not $rejected) { throw 'Unsafe or nonabsolute mount path accepted.' }
    }
    $launcher = Join-Path $PSScriptRoot 'Invoke-DockerBridge.ps1'
    & $launcher -CacheRoot $explicit -RequestFile 'D:\storage contract\request.json' -WorkDirectory 'D:\storage contract' -TemplateRoot 'D:\reports' -AsOfLocal '2026-10-18T11:00:00'
    $captured = $global:StcStorageProbe.Captured
    if ($captured -notcontains $storage.TargetReadOnlyMount -or $captured -notcontains $storage.TempMount) {
        throw 'Printing must mount the configured target read-only and temp on the same disk.'
    }
    if (@($captured | Where-Object { $_ -like '*type=volume,source=stc-rpt-rs-target*' }).Count) {
        throw 'Printing must never recreate the retired named target volume.'
    }
    if ($captured -notcontains 'STC_RPT_WORKER=/target/debug/stc-rpt-worker') { throw 'Worker lookup changed.' }
    $runLimits = $captured -join '|'
    if ($runLimits -notlike '*--memory|2g|--memory-swap|2g|--cpus|2*') { throw 'Printing resource limits missing.' }
    Write-Output 'Docker storage contracts: passed (D defaults, overrides, six invalid roots, read-only print mount, D temp, worker path and resource limits).'
} finally {
    $env:STC_RPT_DOCKER_CACHE_ROOT = $oldRoot
    if ($null -ne $previousProbe) { $global:StcStorageProbe = $previousProbe.Value }
    else { Remove-Variable -Name StcStorageProbe -Scope Global }
}