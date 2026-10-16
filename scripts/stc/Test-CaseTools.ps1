# Synthetic contract tests; no SQL, school reports or real PDFs are used.
Set-StrictMode -Version Latest
$ErrorActionPreference='Stop'
. (Join-Path $PSScriptRoot 'CaseTools.ps1')
$script:checks=0
function Assert($Condition, $Message) { if (-not $Condition) { throw $Message }; $script:checks++ }
function MustReject([scriptblock]$Action, [string]$Message) {
    $rejected=$false
    try { & $Action } catch { $rejected=$true }
    Assert $rejected $Message
}
$connectionString=New-CaseConnectionString 'localhost,1234' 'Synthetic;Database'
$connectionBuilder=[Data.SqlClient.SqlConnectionStringBuilder]::new($connectionString)
Assert ($connectionBuilder['Data Source'] -eq 'localhost,1234' -and $connectionBuilder['Initial Catalog'] -eq 'Synthetic;Database' -and $connectionBuilder['Integrated Security'] -eq $true) 'SQL builder did not preserve canonical keywords/authentication.'
$data=[Data.DataSet]::new()
$table=[Data.DataTable]::new('Report')
[void]$table.Columns.Add('StudentID',[int]); [void]$table.Columns.Add('Text',[string]); [void]$table.Columns.Add('Amount',[decimal]); [void]$table.Columns.Add('Image',[byte[]])
[void]$table.Rows.Add(1,'é 学生 العربية',[decimal]12.50,[byte[]]@(0,1,255))
[void]$table.Rows.Add(1,[DBNull]::Value,[decimal]0,[DBNull]::Value)
$data.Tables.Add($table)
Assert-CaseScope $data @{StudentID=1}
$second=$table.Clone(); $second.TableName='Second'; [void]$second.Rows.Add(2,'other',1,[DBNull]::Value); $data.Tables.Add($second)
MustReject { Assert-CaseScope $data @{StudentID=1} } 'Every table must be checked for foreign students.'
$second.Rows[0]['StudentID']=1
ConvertTo-TypedPdfText $table
Assert ($table.Rows[0]['Text'] -eq 'e 学生 العربية') 'Typed normalization changed Chinese or Arabic letters.'
Assert ($table.Rows[1].IsNull('Text')) 'DBNull became empty text.'
$writer=[IO.StringWriter]::new([Globalization.CultureInfo]::InvariantCulture)
$data.WriteXml($writer,[Data.XmlWriteMode]::WriteSchema)
$copy=[Data.DataSet]::new(); [void]$copy.ReadXml([IO.StringReader]::new($writer.ToString()),[Data.XmlReadMode]::ReadSchema)
Assert ($copy.Tables.Count -eq 2 -and $copy.Tables[0].Columns['Amount'].DataType -eq [decimal]) 'Schema/table loss during frozen replay.'
Assert ($copy.Tables[0].Rows[0]['Image'][2] -eq 255 -and $copy.Tables[0].Rows[1].IsNull('Image')) 'Binary/null data changed.'
Assert ((Get-CaseTextHash 'abc') -eq 'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad') 'Procedure fingerprint differs from SHA-256 UTF-8.'
MustReject { Assert-CaseContained 'C:\owned' 'C:\owned-other\out.pdf' } 'Prefix collision escaped owned root.'
MustReject { Assert-CaseContained 'C:\owned' 'C:\owned\..\out.pdf' } 'Traversal escaped owned root.'
$directory=Join-Path ([IO.Path]::GetTempPath()) ('stc-case-test-'+[Guid]::NewGuid().ToString('N'))
[void](New-Item -ItemType Directory -Path $directory)
try {
    $shell=(Get-Process -Id $PID).Path
    $process=Invoke-CaseProcess $shell @('-NoProfile','-Command','[Console]::Out.Write("value with spaces"); exit 0') $directory 'success' 15
    Assert ($null -ne $process.exitCode -and $process.exitCode -eq 0) 'Process exit code is missing.'
    Assert ((Get-Content -Encoding UTF8 -LiteralPath $process.response -Raw) -eq 'value with spaces') 'Native argument quoting failed.'
    $failed=Invoke-CaseProcess $shell @('-NoProfile','-Command','exit 7') $directory 'failure' 15
    Assert ($failed.exitCode -eq 7) 'Nonzero process exit was lost.'
    $stale=Join-Path $directory 'stale.pdf'; [IO.File]::WriteAllText($stale,'stale')
    MustReject { Assert-CasePdfResult $failed $stale } 'Old PDF masked a process failure.'
} finally {
    # Delete only individually owned synthetic artifacts, never recursively computed directories.
    foreach ($file in @('success.response.json','success.stderr.log','failure.response.json','failure.stderr.log','stale.pdf')) {
        $path=Join-Path $directory $file
        if (Test-Path -LiteralPath $path) { Remove-Item -LiteralPath $path }
    }
    Remove-Item -LiteralPath $directory
    $data.Dispose(); $copy.Dispose(); $writer.Dispose()
}
Write-Output "$script:checks case harness checks passed."
