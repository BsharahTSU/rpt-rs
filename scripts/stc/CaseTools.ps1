# Helpers shared by the private-case runner and synthetic verification.
Set-StrictMode -Version Latest
function ConvertTo-CaseMap($Value) {
    $map = @{}
    foreach ($property in $Value.PSObject.Properties) { $map[$property.Name] = $property.Value }
    return $map
}
function Assert-CaseScope([System.Data.DataSet]$Data, [hashtable]$Selectors) {
    if ($Data.Tables.Count -eq 0 -or $Data.Tables[0].Rows.Count -eq 0) { throw 'No report rows returned.' }
    foreach ($table in $Data.Tables) {
        foreach ($name in @('StudentID','SchoolID','ClassID','SectionID','TermID','ReportID')) {
            if (-not $Selectors.ContainsKey($name) -or -not $table.Columns.Contains($name)) { continue }
            $expected = [Convert]::ToInt32($Selectors[$name], [Globalization.CultureInfo]::InvariantCulture)
            if ($expected -le 0) { throw "Nonpositive scope selector: $name" }
            foreach ($row in $table.Rows) {
                if ($row.IsNull($name) -or [Convert]::ToInt32($row[$name], [Globalization.CultureInfo]::InvariantCulture) -ne $expected) {
                    throw "Result scope mismatch: $name"
                }
            }
        }
    }
}
function ConvertTo-TypedPdfText([System.Data.DataTable]$Table) {
    foreach ($row in $Table.Rows) {
        foreach ($column in $Table.Columns) {
            if ($row[$column] -isnot [string]) { continue }
            $normalized = $row[$column].Normalize([Text.NormalizationForm]::FormD)
            $builder = [Text.StringBuilder]::new()
            foreach ($character in $normalized.ToCharArray()) {
                if ([Globalization.CharUnicodeInfo]::GetUnicodeCategory($character) -ne [Globalization.UnicodeCategory]::NonSpacingMark) {
                    [void]$builder.Append($character)
                }
            }
            $row[$column] = $builder.ToString().Normalize([Text.NormalizationForm]::FormC)
        }
    }
}
function Get-CaseTextHash([string]$Text) {
    $sha = [Security.Cryptography.SHA256]::Create()
    try { return ([BitConverter]::ToString($sha.ComputeHash([Text.Encoding]::UTF8.GetBytes($Text)))).Replace('-','').ToLowerInvariant() }
    finally { $sha.Dispose() }
}
function Assert-CaseContained([string]$Root, [string]$Path) {
    $fullRoot = [IO.Path]::GetFullPath($Root).TrimEnd('\','/') + [IO.Path]::DirectorySeparatorChar
    $fullPath = [IO.Path]::GetFullPath($Path)
    if (-not $fullPath.StartsWith($fullRoot, [StringComparison]::OrdinalIgnoreCase)) { throw 'Path is outside its owned root.' }
    $current = $fullPath
    while ($current) {
        if (Test-Path -LiteralPath $current) {
            if (((Get-Item -LiteralPath $current -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw 'Reparse paths are not supported.' }
        }
        $current = [IO.Path]::GetDirectoryName($current)
    }
    return $fullPath
}
function Invoke-CaseProcess([string]$Executable, [string[]]$Arguments, [string]$Directory, [string]$Prefix, [int]$TimeoutSeconds=120) {
    # CommandLineToArgvW quoting; arguments are passed to an executable, never a shell.
    $quoted = foreach ($argument in $Arguments) { '"' + [regex]::Replace([regex]::Replace($argument, '(\\*)"', '$1$1\"'), '(\\+)$', '$1$1') + '"' }
    $stdout = Join-Path $Directory ($Prefix + '.response.json')
    $stderr = Join-Path $Directory ($Prefix + '.stderr.log')
    $process = Start-Process -FilePath $Executable -ArgumentList ($quoted -join ' ') -WorkingDirectory $Directory -RedirectStandardOutput $stdout -RedirectStandardError $stderr -WindowStyle Hidden -PassThru
    $null = $process.Handle
    try {
        if (-not $process.WaitForExit($TimeoutSeconds * 1000)) {
            Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
            throw "Process timed out: $Prefix"
        }
        $process.WaitForExit()
        # Start-Process on Windows PowerShell can expose a null ExitCode after WaitForExit.
        $process.Refresh()
        $code = $process.ExitCode
        return [pscustomobject]@{ exitCode=$code; response=$stdout; stderr=$stderr }
    } finally { $process.Dispose() }
}
function Assert-CasePdfResult($Process, [string]$Output, [switch]$Typed) {
    if ($null -eq $Process.exitCode -or $Process.exitCode -ne 0) { throw 'Bridge process did not exit successfully; inspect the private log.' }
    $response = Get-Content -Encoding UTF8 -LiteralPath $Process.response -Raw | ConvertFrom-Json
    if ($Typed) {
        if ($response.error -ne $false -or $response.code -ne 'ok' -or $response.pageCount -le 0) { throw 'Typed bridge rejected the case.' }
    } elseif ($response.Error -ne $false -or [string]$response.Result -ne $Output) { throw 'DataSet bridge rejected the case.' }
    if (-not (Test-Path -LiteralPath $Output) -or (Get-Item -LiteralPath $Output).Length -le 0) { throw 'Bridge produced no fresh PDF.' }
}
