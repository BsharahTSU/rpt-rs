using System.Data;
using System.Diagnostics;
using System.Globalization;
using System.Text.Json;
namespace WCF.Service;
internal static class Program
{
    internal static async Task<int> Main(string[] args)
    {
        bool typed = true;
        string code = "invalid_request";
        string scratch = null;
        try
        {
            var request = Contract.ReadRequest(args, new JsonCodec(), ref typed);
            code = "render_failed";
            if (request.ExportFormat != "pdf") throw new NotSupportedException("BIFF/XLS export is not implemented in this experimental adapter.");
            string localClock = CaptureLocalClock(); // Validate overrides before any SQL work.
            using var data = DataAccess.Load(request);
            var table = data.Tables[0]; // Same first-table binding for main report and subreports.
            scratch = request.OutputPath + "." + Guid.NewGuid().ToString("N") + ".partial";
            var prepared = new {
                report_path = request.ReportPath, output_path = scratch,
                export_format = request.ExportFormat, as_of_local = localClock,
                columns = table.Columns.Cast<DataColumn>().Select(c => new { name = c.ColumnName, value_type = ValueType(c.DataType) }),
                rows = table.Rows.Cast<DataRow>().Select(r => r.ItemArray.Select(Cell)),
                query_params = request.QueryParams, report_variables = request.ReportVariables
            };
            string response = await Run(Environment.GetEnvironmentVariable("STC_RPT_WORKER") ?? "/usr/local/bin/stc-rpt-worker",
                [], JsonSerializer.Serialize(prepared));
            using var result = JsonDocument.Parse(response);
            int count = result.RootElement.GetProperty("pageCount").GetInt32();
            if (count <= 0) throw new InvalidOperationException("Renderer produced no pages.");
            if (!File.Exists(scratch) || new FileInfo(scratch).Length == 0)
                throw new InvalidOperationException("Renderer produced an empty export.");
            File.Move(scratch, request.OutputPath, true);
            Console.Out.Write(typed
                ? JsonSerializer.Serialize(new { error = false, code = "ok", pageCount = count, diagnostic = (string)null })
                : JsonSerializer.Serialize(new { Error = false, Result = request.OutputPath }));
            return 0;
        }
        catch (Exception e)
        {
            Console.Error.WriteLine(e);
            Console.Out.Write(typed
                ? JsonSerializer.Serialize(new { error = true, code, pageCount = 0, diagnostic = e.Message })
                : JsonSerializer.Serialize(new { Error = true, Result = e.Message }));
            return 1;
        }
        finally
        {
            if (scratch != null)
                try { File.Delete(scratch); } catch (Exception e) { Console.Error.WriteLine("Export cleanup failed: " + e.Message); }
        }
    }
    // A naive local calendar value matches Crystal's CurrentDate/CurrentTime semantics. The
    // launcher captures Windows host time; direct adapter callers use their process-local clock.
    internal static string CaptureLocalClock()
    {
        const string format = "yyyy-MM-dd'T'HH:mm:ss";
        string supplied = Environment.GetEnvironmentVariable("STC_RPT_LOCAL_DATETIME");
        if (supplied == null) return DateTime.Now.ToString(format, CultureInfo.InvariantCulture);
        if (!DateTime.TryParseExact(supplied, format, CultureInfo.InvariantCulture, DateTimeStyles.None, out var parsed)
            || parsed.ToString(format, CultureInfo.InvariantCulture) != supplied)
            throw new InvalidOperationException("Invalid local render clock; expected yyyy-MM-ddTHH:mm:ss without an offset.");
        return supplied;
    }
    internal static object Cell(object value) => value switch {
        null or DBNull => null,
        byte[] bytes => new { bytes = bytes.Select(b => (int)b).ToArray() },
        DateTime date => new { text = date.ToString("yyyy-MM-dd HH:mm:ss", CultureInfo.InvariantCulture) },
        TimeSpan time => new { text = time.ToString(@"hh\:mm\:ss", CultureInfo.InvariantCulture) },
        bool flag => new { text = flag ? "true" : "false" },
        _ => new { text = Convert.ToString(value, CultureInfo.InvariantCulture) }
    };
    internal static string ValueType(Type type) => type == typeof(bool) ? "Boolean" : type == typeof(DateTime) ? "DateTime"
        : type == typeof(TimeSpan) ? "Time" : type == typeof(byte[]) ? "Blob"
        : type == typeof(string) || type == typeof(char) || type == typeof(Guid) ? "String" : "Number";
    private static async Task<string> Run(string file, string[] args, string input)
    {
        var start = new ProcessStartInfo(file) { RedirectStandardInput = true, RedirectStandardOutput = true, RedirectStandardError = true, UseShellExecute = false };
        foreach (var arg in args) start.ArgumentList.Add(arg);
        using var process = Process.Start(start) ?? throw new InvalidOperationException("Cannot start renderer.");
        var output = process.StandardOutput.ReadToEndAsync();
        var errors = process.StandardError.ReadToEndAsync();
        try
        {
            await process.StandardInput.WriteAsync(input);
            process.StandardInput.Close();
            await process.WaitForExitAsync();
            string diagnostics = await errors;
            if (!String.IsNullOrWhiteSpace(diagnostics)) Console.Error.Write(diagnostics);
            if (process.ExitCode != 0) throw new InvalidOperationException("Renderer rejected the report. " + diagnostics);
            return await output;
        }
        finally { if (!process.HasExited) process.Kill(entireProcessTree: true); }
    }
}
