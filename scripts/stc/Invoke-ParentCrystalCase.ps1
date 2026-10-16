[CmdletBinding()]
param(
    [Parameter(Mandatory=$true)][string]$CaseFile,
    [Parameter(Mandatory=$true)][string]$TemplateRoot,
    [Parameter(Mandatory=$true)][string]$NativeBridge,
    [string]$Server='localhost\SQLSERVER2016',
    [string]$FontDirectory='C:\Windows\Fonts',
    [string]$Image='stc-rpt-rs:bridge-dev'
)
Set-StrictMode -Version Latest
$ErrorActionPreference='Stop'
. (Join-Path $PSScriptRoot 'CaseTools.ps1')
$repo = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
$private = Join-Path $repo 'research/stc-verification'
$CaseFile = Assert-CaseContained $private $CaseFile
$case = Get-Content -Encoding UTF8 -LiteralPath $CaseFile -Raw | ConvertFrom-Json
if ($case.caseId -notmatch '^[a-z0-9][a-z0-9-]{0,63}$') { throw 'Invalid opaque case ID.' }
if ($case.procedure -notmatch '^dbo\.[A-Za-z_][A-Za-z0-9_]*$') { throw 'Only a named dbo stored procedure is allowed.' }
if ($case.procedureReadOnly -ne $true -or $case.reviewedProcedureSha256 -notmatch '^[a-f0-9]{64}$') { throw 'A reviewed read-only procedure hash is required.' }
$template = Assert-CaseContained $TemplateRoot (Join-Path $TemplateRoot $case.relativeTemplate)
if ([IO.Path]::GetExtension($template) -ne '.rpt' -or -not (Test-Path -LiteralPath $template)) { throw 'RPT template is missing.' }
if ((& docker info --format '{{.OSType}}') -ne 'linux') { throw 'Docker Linux engine required.' }
$run = Join-Path (Split-Path -Parent $CaseFile) ('run-' + (Get-Date -Format 'yyyyMMdd-HHmmss') + '-' + [Guid]::NewGuid().ToString('N').Substring(0,8))
[void](New-Item -ItemType Directory -Path $run)
$selectors = ConvertTo-CaseMap $case.queryParams
$variables = ConvertTo-CaseMap $case.reportVariables
$requests = [Collections.Generic.List[string]]::new()
$culture = Get-Culture
$clock = [DateTime]::Now.ToString('yyyy-MM-ddTHH:mm:ss',[Globalization.CultureInfo]::InvariantCulture)
$evidence = [ordered]@{
    caseId=$case.caseId; startedLocal=$clock; sourceCommit=(& git -C $repo rev-parse HEAD)
    sourceDirty=([bool](& git -C $repo status --porcelain)); caseManifestSha256=(Get-FileHash -LiteralPath $CaseFile -Algorithm SHA256).Hash.ToLowerInvariant()
    nativeBridgeSha256=(Get-FileHash -LiteralPath $NativeBridge -Algorithm SHA256).Hash.ToLowerInvariant()
    templateSha256=(Get-FileHash -LiteralPath $template -Algorithm SHA256).Hash.ToLowerInvariant()
    dockerImage=(& docker image inspect $Image --format '{{.Id}}')
    culture=$culture.Name; shortDate=$culture.DateTimeFormat.ShortDatePattern
    clockNote='Rust uses captured host local time; unchanged native bridge uses its live local clock.'
    fontFiles=@(); stages=[ordered]@{}; authenticatedApi='not-tested'; visual='not-reviewed'
}
foreach ($font in @('arial.ttf','arialbd.ttf','arialbi.ttf','ariali.ttf','tahoma.ttf','tahomabd.ttf')) {
    $path=Join-Path $FontDirectory $font
    if (Test-Path -LiteralPath $path) { $evidence.fontFiles += @{name=$font;sha256=(Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()} }
}
$artifactProbe=Invoke-CaseProcess 'docker.exe' @('run','--rm','--network','none','--mount',"type=bind,source=$repo,target=/src,readonly",'--mount','type=volume,source=stc-rpt-rs-target,target=/target,readonly',$Image,'sha256sum','/target/debug/stc-rpt-worker','/src/bridge/Stc.CrystalBridge/bin/Debug/net10.0/stc-crystal-bridge.dll') $run 'build-artifacts'
if ($artifactProbe.exitCode -ne 0) { throw 'Built worker/bridge artifacts are missing.' }
$evidence.builtArtifacts=@(Get-Content -Encoding UTF8 -LiteralPath $artifactProbe.response)
$evidence.nativeCrystalVersion=[Diagnostics.FileVersionInfo]::GetVersionInfo((Join-Path (Split-Path -Parent $NativeBridge) 'CrystalDecisions.CrystalReports.Engine.dll')).FileVersion
$evidence.frozenReplayLimit='DataSet mode has no native SQL schema fallback; assess live/frozen failures separately.'
$oldTemp=$env:PARENT_REPORTS_CRYSTAL_TEMP_ROOT
$oldRoot=$env:PARENT_REPORTS_CRYSTAL_TEMPLATE_ROOT
try {
    $connectionString=New-CaseConnectionString $Server $case.database
    $connection=[System.Data.SqlClient.SqlConnection]::new($connectionString)
    $connection.Open()
    try {
        $metadata=$connection.CreateCommand()
        $metadata.CommandText='SELECT OBJECT_DEFINITION(OBJECT_ID(@name))'
        [void]$metadata.Parameters.Add('@name',[Data.SqlDbType]::NVarChar,256)
        $metadata.Parameters['@name'].Value=$case.procedure
        $definition=[string]$metadata.ExecuteScalar(); $metadata.Dispose()
        if ((Get-CaseTextHash $definition) -ne $case.reviewedProcedureSha256) { throw 'Procedure definition changed since read-only review.' }
        $evidence.procedureSha256=$case.reviewedProcedureSha256
        $command=$connection.CreateCommand(); $command.CommandText=$case.procedure
        $command.CommandType=[Data.CommandType]::StoredProcedure; $command.CommandTimeout=120
        [Data.SqlClient.SqlCommandBuilder]::DeriveParameters($command)
        $remaining=@{}
        foreach ($key in $selectors.Keys) {
            $normalized='@'+$key.TrimStart('@')
            if ($remaining.ContainsKey($normalized)) { throw 'Duplicate SQL parameter alias.' }
            $remaining[$normalized]=$selectors[$key]
        }
        foreach ($parameter in $command.Parameters) {
            if ($parameter.Direction -eq [Data.ParameterDirection]::ReturnValue -or $parameter.Direction -eq [Data.ParameterDirection]::Output) { continue }
            if (-not $remaining.ContainsKey($parameter.ParameterName)) { throw 'A declared SQL input is missing.' }
            $value=$remaining[$parameter.ParameterName]; $parameter.Value=if ($null -eq $value) { [DBNull]::Value } else { $value }; $remaining.Remove($parameter.ParameterName)
        }
        $remaining.Remove('@ForReview')
        if ($remaining.Count -ne 0 -or -not $selectors.ContainsKey('StudentID') -or $selectors.StudentID -le 0) { throw 'Unexpected or missing student selector.' }
        $data=[Data.DataSet]::new(); $adapter=[Data.SqlClient.SqlDataAdapter]::new($command)
        try { [void]$adapter.Fill($data) } finally { $adapter.Dispose(); $command.Dispose() }
        Assert-CaseScope $data $selectors
        $raw=Join-Path $run 'snapshot-raw.xml'; $data.WriteXml($raw,[Data.XmlWriteMode]::WriteSchema)
        ConvertTo-TypedPdfText $data.Tables[0]
        $snapshot=Join-Path $run 'snapshot.xml'; $data.WriteXml($snapshot,[Data.XmlWriteMode]::WriteSchema)
        $evidence.snapshotRawSha256=(Get-FileHash -LiteralPath $raw -Algorithm SHA256).Hash.ToLowerInvariant()
        $evidence.snapshotSha256=(Get-FileHash -LiteralPath $snapshot -Algorithm SHA256).Hash.ToLowerInvariant()
        $evidence.tables=@($data.Tables | ForEach-Object { @{name=$_.TableName;rows=$_.Rows.Count;columns=$_.Columns.Count;verifiedScopeColumns=@($_.Columns | Where-Object { $selectors.ContainsKey($_.ColumnName) -and $_.ColumnName -in @('StudentID','SchoolID','ClassID','SectionID','TermID','ReportID') } | ForEach-Object ColumnName)} })
        $data.Dispose(); $evidence.stages['capture']='passed'
    } finally { $connection.Dispose() }
    $env:PARENT_REPORTS_CRYSTAL_TEMP_ROOT=$run; $env:PARENT_REPORTS_CRYSTAL_TEMPLATE_ROOT=$TemplateRoot
    $nonce=[Guid]::NewGuid().ToString('N')
    $liveRequest=Join-Path $run ('crystal-'+$nonce+'.request.json')
    $liveOutput=Join-Path $run ('crystal-'+$nonce+'.output.pdf')
    [IO.File]::WriteAllBytes($liveOutput,[byte[]]@())
    $requests.Add($liveRequest)
    @{reportPath=$template;outputPath=$liveOutput;connectionString=$connectionString;queryType='stored_procedure';storedProcedure=$case.procedure;queryParams=$selectors;reportVariables=$variables;exportFormat='pdf'} | ConvertTo-Json -Depth 30 | Set-Content -LiteralPath $liveRequest -Encoding utf8
    try {
        $process=Invoke-CaseProcess $NativeBridge @($liveRequest) $run 'native-live'
        Assert-CasePdfResult $process $liveOutput -Typed
        Move-Item -LiteralPath $liveOutput -Destination (Join-Path $run 'native-live.pdf')
        $evidence.stages['native-live']='passed'
    } catch { $evidence.stages['native-live']='failed'; $_ | Out-String | Set-Content -LiteralPath (Join-Path $run 'native-live.failure.txt') }
    $xml=[IO.File]::ReadAllText($snapshot)
    foreach ($engine in @('native-frozen','rust-frozen')) {
        $containerName=$null
        $request=Join-Path $run ($engine+'.request.json'); $requests.Add($request)
        $output=Join-Path $run ($engine+'.pdf')
        $requestOutput=if ($engine -eq 'native-frozen') { $output } else { '/work/'+$engine+'.pdf' }
        $requestTemplate=if ($engine -eq 'native-frozen') { $template } else { '/reports/'+$case.relativeTemplate.Replace('\','/') }
        @{ReportPath=$requestTemplate;OutputFilePath=$requestOutput;DataSetXml=$xml;QueryParams=$selectors;ReportVariables=$variables;ExportFormat='pdf'} | ConvertTo-Json -Depth 30 | Set-Content -LiteralPath $request -Encoding utf8
        try {
            if ($engine -eq 'native-frozen') { $process=Invoke-CaseProcess $NativeBridge @($request) $run $engine }
            else {
                $containerName='stc-case-'+[Guid]::NewGuid().ToString('N')
                $arguments=@('run','--rm','--name',$containerName,'--network','none','--mount',"type=bind,source=$repo,target=/src,readonly",'--mount',"type=bind,source=$run,target=/work",'--mount',"type=bind,source=$TemplateRoot,target=/reports,readonly",'--mount',"type=bind,source=$FontDirectory,target=/usr/local/share/fonts/windows,readonly",'--mount','type=volume,source=stc-rpt-rs-target,target=/target,readonly','-e','STC_RPT_WORKER=/target/debug/stc-rpt-worker','-e',('STC_RPT_LOCALE='+$culture.Name),'-e',('STC_RPT_SHORT_DATE='+$culture.DateTimeFormat.ShortDatePattern),'-e',('STC_RPT_LOCAL_DATETIME='+$clock),$Image,'dotnet','/src/bridge/Stc.CrystalBridge/bin/Debug/net10.0/stc-crystal-bridge.dll',('/work/'+[IO.Path]::GetFileName($request)))
                $process=Invoke-CaseProcess 'docker.exe' $arguments $run $engine
            }
            # The Rust response uses its container path, while the artifact is checked on the host.
            if ($engine -eq 'rust-frozen') {
                if ($null -eq $process.exitCode -or $process.exitCode -ne 0) { throw 'Rust bridge did not exit successfully.' }
                $response=Get-Content -Encoding UTF8 -LiteralPath $process.response -Raw | ConvertFrom-Json
                if ($response.Error -ne $false -or $response.Result -ne $requestOutput -or -not (Test-Path -LiteralPath $output) -or (Get-Item -LiteralPath $output).Length -le 0) { throw 'Rust bridge did not produce the expected PDF.' }
            } else { Assert-CasePdfResult $process $output }
            $evidence.stages[$engine]='passed'
        } catch { $evidence.stages[$engine]='failed'; $_ | Out-String | Set-Content -LiteralPath (Join-Path $run ($engine+'.failure.txt')) }
        finally {
            if ($containerName) {
                # Stop the owned container even if its Docker client timed out. Normal --rm completion means no container remains.
                $cleanup=Invoke-CaseProcess 'docker.exe' @('rm','-f',$containerName) $run 'rust-container-cleanup' 30
                $evidence.containerCleanupAttempted=$true
            }
        }
    }
    $evidence.stages['visual']='pending'
} catch {
    $evidence.stages['pipeline']='failed'
    $_ | Out-String | Set-Content -LiteralPath (Join-Path $run 'pipeline.failure.txt')
} finally {
    $env:PARENT_REPORTS_CRYSTAL_TEMP_ROOT=$oldTemp; $env:PARENT_REPORTS_CRYSTAL_TEMPLATE_ROOT=$oldRoot
    foreach ($request in $requests) { if (Test-Path -LiteralPath $request) { Remove-Item -LiteralPath $request -Force } }
    $evidence.finishedLocal=[DateTime]::Now.ToString('s')
    $evidence | ConvertTo-Json -Depth 30 | Set-Content -LiteralPath (Join-Path $run 'evidence.json') -Encoding utf8
}
[pscustomobject]@{caseId=$case.caseId;runDirectory=$run;stages=$evidence.stages} | ConvertTo-Json -Depth 10
if ($evidence.stages.Values -contains 'failed') { exit 1 }
