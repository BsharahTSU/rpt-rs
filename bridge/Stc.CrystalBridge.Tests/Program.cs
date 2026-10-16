using System.Data;
using System.Text.Json;
using Microsoft.Data.SqlClient;
using WCF.Service;
using BridgeProgram = WCF.Service.Program;
int passed = 0;
void Check(bool condition) { if (!condition) throw new Exception("Assertion failed"); passed++; }
void Reject(Action action) { try { action(); } catch { passed++; return; } throw new Exception("Expected rejection"); }
var root = Path.Combine(Path.GetTempPath(), "stc-bridge-tests-" + Guid.NewGuid().ToString("N"));
Directory.CreateDirectory(root);
try {
 Environment.SetEnvironmentVariable("PARENT_REPORTS_CRYSTAL_TEMP_ROOT", root);
 Environment.SetEnvironmentVariable("PARENT_REPORTS_CRYSTAL_TEMPLATE_ROOT", root);
 var nonce = Guid.NewGuid().ToString("N");
 var input = Path.Combine(root, "crystal-" + nonce + ".request.json");
 var output = Path.Combine(root, "crystal-" + nonce + ".output.pdf");
 var report = Path.Combine(root, "fixture.RPT");
 File.WriteAllText(report, "test"); File.WriteAllText(output, "");
 Dictionary<string, object> Request() => new() {
  ["reportPath"]=report,["outputPath"]=output,["connectionString"]="Server=localhost;Database=fixture;Integrated Security=True",
  ["queryType"]="stored_procedure",["storedProcedure"]="dbo.fixture",["queryParams"]=new Dictionary<string,object>{{"StudentID",1}},
  ["reportVariables"]=new Dictionary<string,object>(),["exportFormat"]="pdf" };
 BridgeRequest Read(Dictionary<string,object> request) { File.WriteAllText(input,JsonSerializer.Serialize(request)); bool typed=true; return Contract.ReadRequest([input],new JsonCodec(),ref typed); }
 Check(Read(Request()).IsTypedProtocol);
 var numericValues=new JsonCodec().Deserialize<Dictionary<string,object>>("{\"large\":1e100,\"integer\":9223372036854775807,\"decimal\":98.5}");
 Check(numericValues["large"] is double && numericValues["integer"] is long && numericValues["decimal"] is decimal);
 var request=Request(); ((Dictionary<string,object>)request["queryParams"])["StudentID"]=0; Reject(()=>Read(request));
 request=Request(); ((Dictionary<string,object>)request["queryParams"])["@StudentID"]=1; Reject(()=>Read(request));
 request=Request();request["Query"]="SELECT 1";Reject(()=>Read(request));
 request=Request();request["unknown"]=true;Reject(()=>Read(request));
 request=Request();request["storedProcedure"]="dbo.fixture; DELETE FROM Students";Reject(()=>Read(request));
 request=Request();request["commandTimeoutSeconds"]=1;Check(Read(request).IsTypedProtocol); // obsolete fields are accepted but ignored
 var other=Path.Combine(root,"crystal-"+Guid.NewGuid().ToString("N")+".output.pdf");File.WriteAllText(other,"");
 request=Request();request["outputPath"]=other;Reject(()=>Read(request));
 if (!OperatingSystem.IsWindows()) {
  var linked=Path.Combine(root,"linked.rpt");File.CreateSymbolicLink(linked,report);
  request=Request();request["reportPath"]=linked;Reject(()=>Read(request));
 }
 using var data = new DataSet("Fixture");
 var table=data.Tables.Add("First");table.Columns.Add("StudentID",typeof(int));table.Columns.Add("Name",typeof(string));table.Columns.Add("Picture",typeof(byte[]));
 table.Rows.Add(1,"张伟 繁體中文",new byte[]{0,255,1}); table.Rows.Add(1,DBNull.Value,DBNull.Value);
 data.Tables.Add("Second").Columns.Add("Other",typeof(string));
 using var xml=new StringWriter();data.WriteXml(xml,XmlWriteMode.WriteSchema);
 using var decoded=Contract.DeserializeDataSet(xml.ToString());
 Check(decoded.Tables.Count==2 && decoded.Tables[0].TableName=="First");
 Check((string)decoded.Tables[0].Rows[0]["Name"]=="张伟 繁體中文");
 Check(decoded.Tables[0].Rows[1].IsNull("Name"));
 Check(((byte[])decoded.Tables[0].Rows[0]["Picture"]).SequenceEqual(new byte[]{0,255,1}));
 Reject(()=>Contract.DeserializeDataSet("<!DOCTYPE x [<!ENTITY external SYSTEM 'file:///etc/passwd'>]><x>&external;</x>"));
 var legacy=Read(new(){{"ReportPath",report},{"OutputFilePath",output},{"DataSetXml",xml.ToString()}});
 Check(!legacy.IsTypedProtocol && legacy.ExportFormat=="pdf" && legacy.QueryType=="query");
 DataAccess.ValidateResultScope(decoded.Tables[0],new(){{"StudentID",1}});passed++;
 Reject(()=>DataAccess.ValidateResultScope(decoded.Tables[0],new(){{"StudentID",2}}));
 var names=DataAccess.GetSqlTextParameterNames("SELECT @StudentID, '@fake', [@quoted], @@ROWCOUNT -- @comment\n /* @block /* @nested */ */ WHERE SchoolID=@SchoolID");
 Check(names.SequenceEqual(new[]{"@StudentID","@SchoolID"}));
 using var typedCommand=new SqlCommand();typedCommand.Parameters.Add("@StudentID",SqlDbType.Int);
 DataAccess.BindDeclaredParameters(typedCommand,new(){{"StudentID",1},{"ForReview",false}}); Check((int)typedCommand.Parameters[0].Value==1);
 Reject(()=>DataAccess.BindDeclaredParameters(typedCommand,new(){{"StudentID",1},{"UndeclaredSelector",1}}));
 using var defaultCommand=new SqlCommand();defaultCommand.Parameters.Add("@StudentID",SqlDbType.Int);defaultCommand.Parameters.Add("@Optional",SqlDbType.Int);
 DataAccess.BindLegacyDeclaredParameters(defaultCommand,new(){{"StudentID",1}});Check(defaultCommand.Parameters.Count==1);
 using var inout=new SqlCommand();inout.Parameters.Add("@RequiredInOut",SqlDbType.Int).Direction=ParameterDirection.InputOutput;
 Reject(()=>DataAccess.BindLegacyDeclaredParameters(inout,new()));
 Check(BridgeProgram.ValueType(typeof(byte[]))=="Blob");
 Check(JsonSerializer.Serialize(BridgeProgram.Cell(TimeSpan.FromHours(1.5))).Contains("01:30:00"));
 Check(JsonSerializer.Serialize(BridgeProgram.Cell("中文學生")).Contains("text"));
 var originalOut=Console.Out; var originalError=Console.Error;
 try {
  using var stdout=new StringWriter();using var stderr=new StringWriter();Console.SetOut(stdout);Console.SetError(stderr);
  int exit=await BridgeProgram.Main([]);
  using var response=JsonDocument.Parse(stdout.ToString());
  Check(exit==1 && response.RootElement.GetProperty("error").GetBoolean() && response.RootElement.GetProperty("code").GetString()=="invalid_request" && stderr.ToString().Length>0);
  stdout.GetStringBuilder().Clear();stderr.GetStringBuilder().Clear();
  File.WriteAllText(input,JsonSerializer.Serialize(new {ReportPath=report,OutputFilePath=output,DataSetXml=42}));
  exit=await BridgeProgram.Main([input]);
  using var legacyResponse=JsonDocument.Parse(stdout.ToString());
  Check(exit==1 && legacyResponse.RootElement.GetProperty("Error").GetBoolean() && legacyResponse.RootElement.GetProperty("Result").ValueKind==JsonValueKind.String);
  string oldClock = Environment.GetEnvironmentVariable("STC_RPT_LOCAL_DATETIME");
  string oldWorker = Environment.GetEnvironmentVariable("STC_RPT_WORKER");
  try {
   Environment.SetEnvironmentVariable("STC_RPT_LOCAL_DATETIME", null);
   DateTime before = DateTime.Now.AddSeconds(-1);
   string localClock = BridgeProgram.CaptureLocalClock();
   DateTime after = DateTime.Now;
   DateTime captured = DateTime.ParseExact(localClock, "yyyy-MM-dd'T'HH:mm:ss", System.Globalization.CultureInfo.InvariantCulture);
   Check(before <= captured && captured <= after);
   foreach (string value in new[] { "2026-10-16T00:15:07", "2024-02-29T23:59:59" }) {
    Environment.SetEnvironmentVariable("STC_RPT_LOCAL_DATETIME", value);
    Check(BridgeProgram.CaptureLocalClock() == value);
   }
   // Deliberately malformed XML distinguishes clock validation from data loading: the clock
   // must fail first, before a request can touch SQL or pass data to the worker.
   File.WriteAllText(input, JsonSerializer.Serialize(new { ReportPath=report, OutputFilePath=output, DataSetXml="<invalid" }));
   foreach (string value in new[] { "", "2026-02-29T00:00:00", "2026-10-16T24:00:00", "2026-10-16T00:00:00Z", "2026-10-16T00:00:00+03:00", "2026-10-16 00:00:00" }) {
    Environment.SetEnvironmentVariable("STC_RPT_LOCAL_DATETIME", value);
    stdout.GetStringBuilder().Clear(); stderr.GetStringBuilder().Clear();
    exit = await BridgeProgram.Main([input]);
    using var clockFailure = JsonDocument.Parse(stdout.ToString());
    Check(exit == 1 && clockFailure.RootElement.GetProperty("Error").GetBoolean()
      && clockFailure.RootElement.GetProperty("Result").GetString().StartsWith("Invalid local render clock", StringComparison.Ordinal));
   }
   if (OperatingSystem.IsLinux()) {
    // Capture the actual adapter-to-worker stdin without requiring Python, Crystal or a PDF.
    // The mock intentionally fails after capture, so no synthetic bytes can pass as a PDF.
    string capture = Path.Combine(root, "prepared-clock.json");
    string mock = Path.Combine(root, "capture-worker.sh");
    string shellPath = "'" + capture.Replace("'", "'\"'\"'") + "'";
    File.WriteAllText(mock, "#!/bin/sh\ncat > " + shellPath + "\nexit 3\n");
    File.SetUnixFileMode(mock, UnixFileMode.UserRead | UnixFileMode.UserWrite | UnixFileMode.UserExecute);
    Environment.SetEnvironmentVariable("STC_RPT_WORKER", mock);
    File.WriteAllText(input, JsonSerializer.Serialize(new { ReportPath=report, OutputFilePath=output, DataSetXml=xml.ToString() }));
    foreach (string value in new[] { "2026-10-16T00:15:07", "2024-02-29T23:59:59" }) {
     Environment.SetEnvironmentVariable("STC_RPT_LOCAL_DATETIME", value);
     stdout.GetStringBuilder().Clear(); stderr.GetStringBuilder().Clear();
     exit = await BridgeProgram.Main([input]);
     using var preparedClock = JsonDocument.Parse(File.ReadAllText(capture));
     Check(exit == 1 && preparedClock.RootElement.GetProperty("as_of_local").GetString() == value);
    }
   }
  } finally {
   Environment.SetEnvironmentVariable("STC_RPT_LOCAL_DATETIME", oldClock);
   Environment.SetEnvironmentVariable("STC_RPT_WORKER", oldWorker);
  }
 } finally { Console.SetOut(originalOut);Console.SetError(originalError); }
 Console.WriteLine($"STC bridge contract checks: {passed} passed");
} finally { Directory.Delete(root,recursive:true); }
