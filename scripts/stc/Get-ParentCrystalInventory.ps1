[CmdletBinding()]
param(
    [string]$Server = 'localhost,61821',
    [string]$Database = 'AAJ_Live',
    [string]$BackendRoot = 'D:\projects 2025\schools merge 2025 june\Mobile Parent Web Service',
    [string]$TemplateRoot = 'C:\Users\user6\Desktop\tmp\LMS Frontend\reports',
    [string]$OutputDirectory = (Join-Path $PSScriptRoot '../../research/stc-verification/parent-inventory')
)
$ErrorActionPreference = 'Stop'
# SELECT-only inventory; no report procedures are invoked.
function Read-Sql([string]$Db, [string]$Sql) {
    if ($Sql -notmatch '^\s*(SELECT|WITH)\b') { throw 'Only SELECT/CTE inventory statements are allowed.' }
    $builder = New-Object System.Data.SqlClient.SqlConnectionStringBuilder 'Integrated Security=True;TrustServerCertificate=True;Connect Timeout=15'
    $builder['Data Source'] = $Server
    $builder['Initial Catalog'] = $Db
    $connection = New-Object System.Data.SqlClient.SqlConnection $builder.ConnectionString
    try {
        $connection.Open()
        $command = $connection.CreateCommand()
        $command.CommandTimeout = 60
        $command.CommandText = $Sql
        $adapter = New-Object System.Data.SqlClient.SqlDataAdapter $command
        $table = New-Object System.Data.DataTable
        [void]$adapter.Fill($table)
        foreach ($row in $table.Rows) {
            $record = [ordered]@{}
            foreach ($column in $table.Columns) {
                $record[$column.ColumnName] = if ($row[$column] -is [DBNull]) { $null } else { $row[$column] }
            }
            [pscustomobject]$record
        }
    } finally { $connection.Dispose() }
}
function Assert-NoReparse([string]$Path) {
    $current = [IO.Path]::GetFullPath($Path)
    while ($current) {
        if (Test-Path -LiteralPath $current) {
            if (((Get-Item -LiteralPath $current -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
                throw 'Reparse paths are not supported for inventory inputs/outputs.'
            }
        }
        $current = [IO.Path]::GetDirectoryName($current)
    }
}
function Write-PrivateText([string]$Name, [string]$Value) {
    $path = Join-Path $OutputDirectory $Name
    Assert-NoReparse $path
    Set-Content -LiteralPath $path -Value $Value -Encoding utf8
}
function Write-PrivateJson([string]$Name, $Value) {
    Write-PrivateText $Name (ConvertTo-Json -InputObject $Value -Depth 15)
}
function Get-TextHash([string]$Value) {
    $sha = [Security.Cryptography.SHA256]::Create()
    try { ([BitConverter]::ToString($sha.ComputeHash([Text.Encoding]::UTF8.GetBytes($Value)))).Replace('-','').ToLowerInvariant() }
    finally { $sha.Dispose() }
}
function Resolve-Template([string]$Folder, [string]$Name) {
    $folderValue = $Folder.Trim().Replace('\','/').TrimEnd('/')
    $pathValue = $Name.Trim().Replace('\','/')
    foreach ($part in @($folderValue,$pathValue)) {
        if ($part.Contains(':') -or $part.StartsWith('/') -or ($part -split '/') -contains '..' -or
            ($part -split '/') -contains '.' -or $part -match '[\x00-\x1f]') { return $null }
    }
    if ([string]::IsNullOrWhiteSpace($pathValue)) { return $null }
    if ($folderValue -and -not $pathValue.StartsWith($folderValue+'/',[StringComparison]::OrdinalIgnoreCase)) {
        return $folderValue+'/'+$pathValue
    }
    return $pathValue
}
$OutputDirectory = [IO.Path]::GetFullPath($OutputDirectory)
$repository = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
$privateRoot = [IO.Path]::GetFullPath((Join-Path $repository 'research')) + [IO.Path]::DirectorySeparatorChar
if (-not $OutputDirectory.StartsWith($privateRoot,[StringComparison]::OrdinalIgnoreCase)) {
    throw 'Private inventory output must stay beneath this checkout research/ directory.'
}
Assert-NoReparse $OutputDirectory
[void](New-Item -ItemType Directory -Force -Path $OutputDirectory)
$databases = @(Read-Sql 'master' 'SELECT name,state_desc,compatibility_level,HAS_DBACCESS(name) AS HasAccess FROM sys.databases WHERE database_id>4 ORDER BY name')
$availableNames = @($databases | Where-Object { $_.state_desc -eq 'ONLINE' -and $_.HasAccess -eq 1 } | ForEach-Object name)
$profileRows = @()
foreach ($folder in Get-ChildItem (Join-Path $BackendRoot 'Dotnet10/WebApi/appsettingsOverrides') -Directory | Sort-Object Name) {
    $settingsPath = Join-Path $folder.FullName 'appsettings.json'
    if (-not (Test-Path -LiteralPath $settingsPath)) { continue }
    $settings = Get-Content -Encoding UTF8 -Raw -LiteralPath $settingsPath | ConvertFrom-Json
    $configuredDatabase = ''
    if ($settings.ConnectionStrings.sCon) {
        $profileBuilder = New-Object System.Data.SqlClient.SqlConnectionStringBuilder $settings.ConnectionStrings.sCon
        $configuredDatabase = $profileBuilder.InitialCatalog
    }
    $profileRows += [pscustomobject]@{
        Profile=$folder.Name; ConfiguredDatabase=$configuredDatabase
        ExactDatabaseOnLocalServer=[bool]($configuredDatabase -and $availableNames -contains $configuredDatabase)
        Note=if ($configuredDatabase) {'Exact name only; restored copies are not deployment evidence.'} else {'No literal sCon override; resolve inherited/runtime configuration.'}
    }
}
$localFamilies = @()
foreach ($db in $databases | Where-Object { $_.state_desc -eq 'ONLINE' -and $_.HasAccess -eq 1 }) {
    try {
        $schema = @(Read-Sql $db.name "SELECT OBJECT_ID('dbo.SysParam') AS SysParamId,OBJECT_ID('dbo.SchoolClasses') AS SchoolClassesId,COL_LENGTH('dbo.SchoolClasses','IsStandardCertificate') AS StandardFlag,COL_LENGTH('dbo.SchoolClasses','IsGradeBook') AS GradeBookFlag,COL_LENGTH('dbo.SysParam','WebSchoolReports') AS FamilyColumn")
        if (-not $schema[0].SysParamId -or -not $schema[0].SchoolClassesId -or -not $schema[0].FamilyColumn) { continue }
        $family = @(Read-Sql $db.name 'SELECT TOP(1) WebSchoolReports FROM dbo.SysParam')
        $templates = @(Read-Sql $db.name "SELECT SchoolID,ClassID,EvaluationSystem,EvaReportName FROM dbo.SchoolClasses WHERE LOWER(RIGHT(RTRIM(EvaReportName),4))='.rpt'")
        $localFamilies += [pscustomobject]@{
            Database=$db.name;Family=$family[0].WebSchoolReports;CurrentTarget=($db.name -eq $Database)
            StandardFlagPresent=($null -ne $schema[0].StandardFlag);GradeBookFlagPresent=($null -ne $schema[0].GradeBookFlag)
            CrystalEvaluationRows=$templates
        }
    } catch {
        $localFamilies += [pscustomobject]@{Database=$db.name;MetadataUnavailable=$true;Reason='Schema differs from current report metadata contract.'}
    }
}
Write-PrivateJson 'database-coverage.json' ([pscustomobject]@{CurrentDatabase=$Database;Profiles=$profileRows;LocalCopies=$localFamilies})
$policySql = @'
SELECT TOP(1) WebSchoolReports,CurrentYear,CurrentTermID,ReportYear,ReportYearAr,
CASE WHEN TRY_CAST(AllowPrintFinalCertificate AS int)=1 OR LOWER(LTRIM(RTRIM(CONVERT(nvarchar(20),AllowPrintFinalCertificate))))=N'true' THEN 1 ELSE 0 END AS AllowPrintFinalCertificate
FROM dbo.SysParam
'@
$policy = @(Read-Sql $Database $policySql)[0]
if ($policy.WebSchoolReports.Trim().ToLowerInvariant() -ne 'aaj') {
    throw 'Full candidate selection is validated for AAJ only; other copies are metadata coverage only.'
}
$yearMatch = [regex]::Match([string]$policy.CurrentYear,'^\s*(\d+)(?:[-_/]|$)')
$year = if ($yearMatch.Success) { [int]$yearMatch.Groups[1].Value } else { 0 }
$procedures = @(Read-Sql $Database "SELECT o.name,m.definition FROM sys.objects o JOIN sys.sql_modules m ON m.object_id=o.object_id WHERE o.name IN ('web_GetStudentsIndicatorPreview','web_Get_Std_MarksWithChartStc','web_getStudentsMarks','CalculateGPA_Term')")
$procedureAudit = @()
foreach ($procedure in $procedures) {
    $definition = [string]$procedure.definition
    $safeName = $procedure.name -replace '[^A-Za-z0-9_-]','_'
    Write-PrivateText ($safeName+'.sql') $definition
    $procedureAudit += [pscustomobject]@{
        Procedure=$procedure.name;Sha256=Get-TextHash $definition
        ContainsPersistentMarksUpdate=($definition -match '(?is)UPDATE\s+M\s+SET.{0,80}HideOnCertificate')
        ContainsAverageDelete=($definition -match '(?i)DELETE\s+(?:FROM\s+)?StudentsAverage')
        ContainsGpaCall=($definition -match '(?i)EXEC(?:UTE)?\s+CalculateGPA_Term')
        ContainsMasterReportUpdate=($definition -match '(?i)UPDATE\s+StudentMarksMaster\s+SET\s+ReportID')
        HasGopsRepairMarker=$definition.Contains('Keep requested report metadata local; do not update StudentMarksMaster.')
    }
}
$evaluationProc = @($procedures | Where-Object name -eq 'web_GetStudentsIndicatorPreview')
if ($evaluationProc.Count -ne 1) { throw 'AAJ evaluation procedure text unavailable.' }
# This hash was manually reviewed as SELECT-only on 2026-10-16. Pattern scans above
# are investigation aids, not proof that an arbitrary procedure is side-effect free.
$reviewedEvaluationHash = '1bcd37818cd5706ec611777ad0883022b5f7eb03929586d16e6a86a7eafb2a92'
$evaluationDefinitionReviewed = (Get-TextHash ([string]$evaluationProc[0].definition)) -eq $reviewedEvaluationHash
$base = @'
WITH CourseRows AS (
 SELECT DISTINCT c.ParentID,c.StudentID,c.SchoolID,c.ClassID,c.SectionID
 FROM dbo.vwStudentCourses c INNER JOIN dbo.Students s ON s.ID=c.StudentID
), Ranked AS (
 SELECT c.*,ROW_NUMBER() OVER(PARTITION BY StudentID ORDER BY SchoolID,ClassID,SectionID) AS rn
 FROM CourseRows c
), Context AS (
 SELECT c.*,s.Name AS SchoolName,sc.EvaluationSystem,sc.IsStandardCertificate,sc.EvaReportName,sc.ISNumericMark,
 CASE WHEN EXISTS(SELECT 1 FROM dbo.FatherTuitionMaster f WHERE f.FatherID=c.ParentID AND
 (TRY_CAST(f.HideMarks AS int)=1 OR LOWER(LTRIM(RTRIM(CONVERT(nvarchar(20),f.HideMarks))))=N'true')) THEN 1 ELSE 0 END AS HideMarks
 FROM Ranked c INNER JOIN dbo.Schools s ON s.ID=c.SchoolID
 CROSS APPLY(SELECT TOP(1) EvaluationSystem,IsStandardCertificate,EvaReportName,ISNumericMark
 FROM dbo.SchoolClasses sc WHERE sc.SchoolID=c.SchoolID AND sc.ClassID=c.ClassID) sc
 WHERE c.rn=1
)
'@
$evaluationSql = $base + @'
SELECT c.*,t.TermID,r.ID AS ReportID,
 CASE WHEN t.TermID=1 AND r.ID BETWEEN 1 AND 4 THEN r.ID WHEN t.TermID=2 AND r.ID BETWEEN 5 AND 8 THEN r.ID-4 ELSE 0 END AS EffectiveExam,
 pub.MasterRows,CASE WHEN r.ID IN(1,5) THEN pub.p1 WHEN r.ID IN(2,6) THEN pub.p2 WHEN r.ID IN(3,7) THEN pub.p3 WHEN r.ID IN(4,8) THEN pub.p4 END AS AllPublished,
 CASE WHEN EXISTS(SELECT 1 FROM dbo.Semesters WHERE ID=t.TermID) THEN 1 ELSE 0 END AS SemesterExists,
 CASE WHEN EXISTS(SELECT 1 FROM dbo.examsName e WHERE e.Code=CASE WHEN r.ID>4 THEN r.ID-4 ELSE r.ID END) THEN 1 ELSE 0 END AS ExamExists,
 dat.DataRows,CASE WHEN dat.MinTerm<>t.TermID OR dat.MaxTerm<>t.TermID OR dat.MinReport<>r.ID OR dat.MaxReport<>r.ID THEN 1 ELSE 0 END AS ScopeMismatches
FROM Context c CROSS JOIN(VALUES(1),(2)) t(TermID)
OUTER APPLY(SELECT TOP(1) ID FROM dbo.SchoolReports WHERE TermID=t.TermID AND AllowPrint=1 ORDER BY ID DESC) r
OUTER APPLY(
 SELECT COUNT(*) AS MasterRows,
 MIN(CASE WHEN MarkPublish=1 THEN 1 ELSE 0 END) AS p1,MIN(CASE WHEN Mark2Publish=1 THEN 1 ELSE 0 END) AS p2,
 MIN(CASE WHEN Mark3Publish=1 THEN 1 ELSE 0 END) AS p3,MIN(CASE WHEN Mark4Publish=1 THEN 1 ELSE 0 END) AS p4
 FROM dbo.StudentMarksMaster m WHERE m.StudentID=c.StudentID AND m.TermID=t.TermID AND m.ReportID=r.ID
) pub
OUTER APPLY(
 SELECT COUNT(*) AS DataRows,MIN(v.TermID) AS MinTerm,MAX(v.TermID) AS MaxTerm,MIN(v.ReportID) AS MinReport,MAX(v.ReportID) AS MaxReport
 FROM dbo.web_vwStudentMarksList v
 WHERE v.ReportID=1 AND ISNULL(v.BlockStdMarkEmail,0)=0 AND ISNULL(v.HideInReport,0)=0
 AND v.ClassID=c.ClassID AND v.SectionID=c.SectionID AND v.SchoolID=c.SchoolID AND v.StudentID=c.StudentID AND v.IsExtra=0
 AND (v.CategoryIsMain=1 OR ISNULL((SELECT MainCategory FROM dbo.SchoolReportsClasses WHERE ClassID=v.ClassID AND ReportID=v.ReportID),0)=0)
) dat
WHERE LOWER(RIGHT(RTRIM(c.EvaReportName),4))='.rpt'
ORDER BY c.ClassID,t.TermID,c.StudentID
'@
$standardSql = $base + @"
SELECT c.*,t.TermID,r.ID AS ReportID,pub.MarkRows,slot.PublicationSlot,
 CASE WHEN slot.PublicationSlot>4 THEN slot.PublicationSlot-4 ELSE slot.PublicationSlot END AS EffectiveExam,
 CASE WHEN EXISTS(SELECT 1 FROM dbo.Semesters WHERE ID=t.TermID) THEN 1 ELSE 0 END AS SemesterExists,
 CASE WHEN EXISTS(SELECT 1 FROM dbo.examsName e WHERE e.Code=CASE WHEN slot.PublicationSlot>4 THEN slot.PublicationSlot-4 ELSE slot.PublicationSlot END) THEN 1 ELSE 0 END AS ExamExists
FROM Context c CROSS JOIN(VALUES(1),(2)) t(TermID)
OUTER APPLY(SELECT TOP(1) ID FROM dbo.SchoolReports WHERE TermID=t.TermID AND AllowPrint=1 ORDER BY ID DESC) r
OUTER APPLY(
 SELECT COUNT(*) AS MarkRows,
 MIN(CASE WHEN m.Mark1Publish=1 THEN 1 ELSE 0 END) AS p1,MIN(CASE WHEN m.Mark2Publish=1 THEN 1 ELSE 0 END) AS p2,
 MIN(CASE WHEN m.Mark3Publish=1 THEN 1 ELSE 0 END) AS p3,MIN(CASE WHEN m.Mark4Publish=1 THEN 1 ELSE 0 END) AS p4,
 MIN(CASE WHEN m.Mark5Publish=1 THEN 1 ELSE 0 END) AS p5,MIN(CASE WHEN m.Mark6Publish=1 THEN 1 ELSE 0 END) AS p6,
 MIN(CASE WHEN m.Mark7Publish=1 THEN 1 ELSE 0 END) AS p7,MIN(CASE WHEN m.Mark8Publish=1 THEN 1 ELSE 0 END) AS p8
 FROM dbo.Std_MarKS m WHERE m.StudentID=c.StudentID AND m.Term=t.TermID AND m.TermYear=N'$year'
 AND EXISTS(SELECT 1 FROM dbo.web_vwStudentMarksReport v WHERE v.ID=m.ID AND v.StudentID=c.StudentID
 AND v.Term=t.TermID AND v.TermYear=m.TermYear AND v.SchoolID=c.SchoolID AND v.ClassID=c.ClassID
 AND v.SectionID=c.SectionID AND v.Mark1+v.Mark2+v.Mark3+v.Mark4<>0)
) pub
OUTER APPLY(SELECT MAX(n) AS PublicationSlot FROM(VALUES(1,pub.p1),(2,pub.p2),(3,pub.p3),(4,pub.p4),(5,pub.p5),(6,pub.p6),(7,pub.p7),(8,pub.p8)) v(n,p) WHERE p=1) slot
WHERE c.ClassID BETWEEN 12 AND 18
ORDER BY c.ClassID,t.TermID,c.StudentID
"@
Write-PrivateText 'evaluation-inventory.sql' $evaluationSql
Write-PrivateText 'standard-inventory.sql' $standardSql
$evaluation = @(Read-Sql $Database $evaluationSql)
$standard = @(Read-Sql $Database $standardSql)
$duplicates = @(Read-Sql $Database 'SELECT SchoolID,ClassID,COUNT(*) AS DuplicateCount FROM dbo.SchoolClasses GROUP BY SchoolID,ClassID HAVING COUNT(*)>1')
$candidates = @()
foreach ($kind in @('evaluation','standard')) {
    $rows = if ($kind -eq 'evaluation') { $evaluation } else { $standard }
    foreach ($row in $rows) {
        $template = if ($kind -eq 'evaluation') { Resolve-Template $policy.WebSchoolReports $row.EvaReportName } elseif ($row.ClassID -in 16,17) {'aaj/SchoolReportCardAmerican.rpt'} elseif ($row.ClassID -eq 18) {'aaj/SchoolReportCard9AHSD.rpt'} else {'aaj/SchoolReportCardBritish.rpt'}
        $blockers = @()
        if (-not $policy.AllowPrintFinalCertificate) {$blockers += 'school-print-disabled'}
        if ($row.HideMarks -eq 1) {$blockers += 'marks-hidden'}
        if (($kind -eq 'evaluation' -and -not $row.EvaluationSystem) -or ($kind -eq 'standard' -and -not $row.IsStandardCertificate)) {$blockers += 'certificate-disabled'}
        if (-not $row.SemesterExists -or -not $row.ExamExists) {$blockers += 'metadata-missing'}
        if ($kind -eq 'evaluation' -and (-not $row.ReportID -or -not $row.EffectiveExam -or $row.MasterRows -eq 0 -or $row.AllPublished -ne 1)) {$blockers += 'evaluation-unpublished'}
        if ($kind -eq 'standard' -and ($row.MarkRows -eq 0 -or -not $row.PublicationSlot)) {$blockers += 'standard-unpublished'}
        if (-not $template -or -not (Test-Path -LiteralPath (Join-Path $TemplateRoot $template) -PathType Leaf)) {$blockers += 'template-missing-or-invalid'}
        elseif ($template) {
            try { Assert-NoReparse (Join-Path $TemplateRoot $template) }
            catch { $blockers += 'template-reparse-path' }
        }
        if (@($duplicates | Where-Object {$_.SchoolID -eq $row.SchoolID -and $_.ClassID -eq $row.ClassID}).Count -gt 0) {$blockers += 'ambiguous-school-class'}
        $parentEligible = $blockers.Count -eq 0
        if ($kind -eq 'evaluation' -and $row.DataRows -eq 0) {$blockers += 'evaluation-data-empty'}
        if ($kind -eq 'evaluation' -and $row.ScopeMismatches -gt 0) {$blockers += 'evaluation-data-scope-mismatch'}
        if ($kind -eq 'evaluation' -and -not $evaluationDefinitionReviewed) {$blockers += 'evaluation-procedure-review-required'}
        if ($kind -eq 'standard') {$blockers += 'blocked-live-mutating-procedure'}
        $languages = if ($kind -eq 'standard') {@('ar','en')} else {@('shared')}
        foreach ($language in $languages) {
            $candidates += [pscustomobject]@{
                CaseId='';Database=$Database;Kind=$kind;Template=$template;Language=$language
                StudentID=$row.StudentID;ParentID=$row.ParentID;SchoolID=$row.SchoolID;SchoolName=$row.SchoolName
                ClassID=$row.ClassID;SectionID=$row.SectionID;TermID=$row.TermID;ReportID=$row.ReportID
                EffectiveExam=$row.EffectiveExam;PublicationSlot=$row.PublicationSlot
                LettersGrade=if ($row.ISNumericMark -eq 1) {0} else {1}
                ReportYear=$policy.ReportYear;ParentSqlEligible=$parentEligible;ReadyForNative=($blockers.Count -eq 0)
                HideMarks=($row.HideMarks -eq 1);DataRows=$row.DataRows;Blockers=$blockers
            }
        }
    }
}
$cases = @();$caseNumber=0
$groups = $candidates | Group-Object Kind,Template,ClassID,TermID,ReportID,EffectiveExam,Language,LettersGrade | Sort-Object Name
foreach ($group in $groups) {
    $caseNumber++;$caseId='CR-{0:D3}' -f $caseNumber
    foreach ($candidate in $group.Group) {$candidate.CaseId=$caseId}
    $first=$group.Group[0]
    $ready=@($group.Group | Where-Object ReadyForNative | Sort-Object StudentID)
    $sample=if ($ready.Count) {$ready[0]} else {$null}
    $cases += [pscustomobject]@{
        CaseId=$caseId;Kind=$first.Kind;Template=$first.Template;ClassID=$first.ClassID;TermID=$first.TermID
        ReportID=$first.ReportID;EffectiveExam=$first.EffectiveExam;Language=$first.Language;LettersGrade=$first.LettersGrade
        CandidateCount=$group.Count;SqlEligibleCount=@($group.Group | Where-Object ParentSqlEligible).Count
        ReadyCount=$ready.Count;HiddenCount=@($group.Group | Where-Object HideMarks).Count
        Blockers=@($group.Group.Blockers | Sort-Object -Unique);Representative=$sample
    }
}
Write-PrivateJson 'candidates.json' $candidates
Write-PrivateJson 'cases.json' $cases
Write-PrivateJson 'procedure-audit.json' $procedureAudit
Write-PrivateJson 'summary.json' ([pscustomobject]@{
    InspectedAt=(Get-Date).ToString('o');Server=$Server;CurrentDatabase=$Database
    ProfileCount=$profileRows.Count;LocalCopyCount=$localFamilies.Count;CaseCount=$cases.Count
    ReadyCaseCount=@($cases | Where-Object {$_.ReadyCount -gt 0}).Count
    EvaluationCandidateCount=$evaluation.Count;StandardCandidateCount=$standard.Count
    DistinctStudentCount=@($candidates.StudentID | Sort-Object -Unique).Count
    Policy=$policy;DuplicateSchoolClasses=$duplicates
    Boundary='Read-only SQL eligibility/local template discovery; no authenticated API, dependency, contained-path or actual PDF proof. Other copies are metadata only.'
})
$cases | Select-Object CaseId,Kind,Template,ClassID,TermID,Language,CandidateCount,SqlEligibleCount,ReadyCount,HiddenCount,Blockers | ConvertTo-Json -Depth 5
Write-Host ("Private inventory saved: {0} cases; {1} ready for native evaluation; {2} profiles." -f $cases.Count,@($cases | Where-Object {$_.ReadyCount -gt 0}).Count,$profileRows.Count)
