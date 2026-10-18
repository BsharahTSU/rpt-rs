# Shared local Docker storage. Resolves paths only; the build launcher creates directories.
function Get-StcDockerStorage {
    [CmdletBinding()]
    param([string]$CacheRoot)
    if ([string]::IsNullOrWhiteSpace($CacheRoot)) { $CacheRoot = $env:STC_RPT_DOCKER_CACHE_ROOT }
    if ([string]::IsNullOrWhiteSpace($CacheRoot)) { $CacheRoot = 'D:\tmpppppp123\docker-cache' }
    if ($CacheRoot -notmatch '^(?:[A-Za-z]:[\\/]|\\\\[^\\/]+\\[^\\/]+(?:\\|$))' -or $CacheRoot -match '[,\r\n"]') {
        throw 'Docker cache root must be an absolute path without commas, quotes or line breaks.'
    }
    $root = [IO.Path]::GetFullPath($CacheRoot)
    $target = Join-Path $root 'rpt-rs-target'
    $temp = Join-Path $root 'tmp'
    [pscustomobject]@{
        Root = $root
        TargetDirectory = $target
        TempDirectory = $temp
        TargetMount = "type=bind,source=$target,target=/target"
        TargetReadOnlyMount = "type=bind,source=$target,target=/target,readonly"
        TempMount = "type=bind,source=$temp,target=/tmp"
    }
}