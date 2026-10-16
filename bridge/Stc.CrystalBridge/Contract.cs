// Ported from the current STC bridge contract. Production bridge files are unchanged.
using System.Data;
using System.Globalization;
using System.Text;
using System.Xml;
using Microsoft.Data.SqlClient;
namespace WCF.Service
{
    internal static class Contract
    {
        internal static BridgeRequest ReadRequest(string[] args, JsonCodec serializer, ref bool typedProtocol)
        {
            if (args == null || args.Length != 1)
                throw new InvalidOperationException("Crystal bridge requires a single request JSON file path.");
            serializer.MaxJsonLength = Int32.MaxValue;
            string requestJson;
            using (var file = new FileStream(args[0], FileMode.Open, FileAccess.Read, FileShare.ReadWrite | FileShare.Delete))
            using (var reader = new StreamReader(file, new UTF8Encoding(false, true), true))
                requestJson = reader.ReadToEnd();
            var json = serializer.Deserialize<Dictionary<string, object>>(requestJson);
            // Set the error response shape before validation, including ambiguous mixed requests.
            typedProtocol = json == null || HasTypedMarker(json) || !HasLegacyMarker(json);
            bool detected = DetectRequestProtocol(json);
            typedProtocol = detected;
            return detected ? ReadTypedRequest(args[0], json) : ReadLegacyRequest(json);
        }

        private static bool DetectRequestProtocol(Dictionary<string, object> json)
        {
            if (json == null) throw new InvalidOperationException("Crystal request must be a JSON object.");
            bool typed = HasTypedMarker(json);
            bool legacy = HasLegacyMarker(json);
            if (typed == legacy)
                throw new InvalidOperationException(typed ? "Crystal request mixes typed and legacy protocol fields." : "Crystal request protocol is not recognized.");
            return typed;
        }

        private static bool HasTypedMarker(Dictionary<string, object> json)
        {
            return json.Keys.Any(key => String.Equals(key, "outputPath", StringComparison.OrdinalIgnoreCase)
                || String.Equals(key, "storedProcedure", StringComparison.OrdinalIgnoreCase));
        }

        private static bool HasLegacyMarker(Dictionary<string, object> json)
        {
            return json.Keys.Any(key => String.Equals(key, "OutputFilePath", StringComparison.OrdinalIgnoreCase)
                || String.Equals(key, "Query", StringComparison.OrdinalIgnoreCase)
                || String.Equals(key, "DataSetXml", StringComparison.OrdinalIgnoreCase));
        }

        private static BridgeRequest ReadTypedRequest(string requestFile, Dictionary<string, object> json)
        {
            string[] required =
            {
                "reportPath", "outputPath", "connectionString", "queryType", "storedProcedure",
                "queryParams", "reportVariables", "exportFormat"
            };
            string[] obsoleteLimits = { "commandTimeoutSeconds", "maxDataRows", "maxPdfBytes", "maxPageCount" };
            if (json.Keys.Any(key => !required.Contains(key, StringComparer.Ordinal) && !obsoleteLimits.Contains(key, StringComparer.Ordinal))
                || required.Any(key => !json.ContainsKey(key)))
                throw new InvalidOperationException("Crystal typed request fields are invalid.");

            string tempRoot = BridgePathPolicy.ValidateRoot(Environment.GetEnvironmentVariable("PARENT_REPORTS_CRYSTAL_TEMP_ROOT"));
            string templateRoot = BridgePathPolicy.ValidateRoot(Environment.GetEnvironmentVariable("PARENT_REPORTS_CRYSTAL_TEMPLATE_ROOT"));
            string requestPath = BridgePathPolicy.ValidateRequestFile(tempRoot, requestFile);
            string reportPath = BridgePathPolicy.ValidateTemplateFile(templateRoot, RequiredString(json, "reportPath"));
            string exportFormat = RequiredString(json, "exportFormat");
            if (exportFormat != "pdf" && exportFormat != "excel")
                throw new InvalidOperationException("Crystal typed export format must be pdf or excel.");
            string outputPath = BridgePathPolicy.ValidateOutputPath(requestPath, RequiredString(json, "outputPath"), exportFormat);
            string queryType = RequiredString(json, "queryType");
            bool template = String.Equals(queryType, "template", StringComparison.Ordinal);
            string procedure = template ? json["storedProcedure"] as string : RequiredString(json, "storedProcedure");
            if ((!template && queryType != "stored_procedure")
                || (template ? procedure != String.Empty : !IsSafeStoredProcedure(procedure)))
                throw new InvalidOperationException("Crystal typed query or export format is invalid.");

            Dictionary<string, object> parameters = RequiredDictionary(json, "queryParams");
            Dictionary<string, object> variables = RequiredDictionary(json, "reportVariables");
            ValidateBindings(parameters);
            ValidateBindings(variables);
            parameters = new Dictionary<string, object>(parameters, StringComparer.OrdinalIgnoreCase);
            variables = new Dictionary<string, object>(variables, StringComparer.OrdinalIgnoreCase);
            object student;
            int studentId;
            if ((!parameters.TryGetValue("StudentID", out student) && !parameters.TryGetValue("@StudentID", out student))
                || !Int32.TryParse(Convert.ToString(student, CultureInfo.InvariantCulture), NumberStyles.Integer,
                    CultureInfo.InvariantCulture, out studentId) || studentId <= 0)
                throw new InvalidOperationException("Crystal parent reports require a positive StudentID.");

            return new BridgeRequest
            {
                IsTypedProtocol = true, ReportPath = reportPath, OutputPath = outputPath,
                ConnectionString = ValidateConnectionString(RequiredString(json, "connectionString")),
                Query = procedure, StoredProcedure = procedure, QueryType = queryType,
                UseTemplateData = template, QueryParams = parameters, ReportVariables = variables,
                ExportFormat = exportFormat
            };
        }

        private static BridgeRequest ReadLegacyRequest(Dictionary<string, object> json)
        {
            var values = new Dictionary<string, object>(json, StringComparer.OrdinalIgnoreCase);
            string dataset = OptionalString(values, "DataSetXml");
            string format = OptionalString(values, "ExportFormat");
            format = String.IsNullOrWhiteSpace(format) ? "pdf" : format.Trim().ToLowerInvariant();
            if (format != "pdf" && format != "excel")
                throw new InvalidOperationException("Crystal export format must be pdf or excel.");
            string queryType = OptionalString(values, "QueryType");
            queryType = String.IsNullOrWhiteSpace(queryType) ? "query" : queryType.Trim();
            bool hasDataSet = !String.IsNullOrWhiteSpace(dataset);
            string query = hasDataSet ? OptionalString(values, "Query") : RequiredString(values, "Query");
            return new BridgeRequest
            {
                IsTypedProtocol = false,
                ReportPath = RequiredString(values, "ReportPath"),
                OutputPath = RequiredString(values, "OutputFilePath"),
                ConnectionString = hasDataSet ? OptionalString(values, "ConnectionString")
                    : Utils.NormalizeConnectionString(RequiredString(values, "ConnectionString")),
                Query = query, StoredProcedure = String.Equals(queryType, "stored_procedure", StringComparison.OrdinalIgnoreCase) ? query : String.Empty,
                QueryType = queryType, QueryParams = OptionalDictionary(values, "QueryParams"),
                ReportVariables = OptionalDictionary(values, "ReportVariables"), DataSetXml = dataset,
                ExportFormat = format
            };
        }

        private static string ValidateConnectionString(string connectionString)
        {
            var connection = new SqlConnectionStringBuilder(Utils.NormalizeConnectionString(connectionString));
            if (String.IsNullOrWhiteSpace(connection.DataSource) || String.IsNullOrWhiteSpace(connection.InitialCatalog))
                throw new InvalidOperationException("Crystal reports require a SQL server and database.");
            connection.PersistSecurityInfo = false;
            return connection.ConnectionString;
        }

        private static void ValidateBindings(Dictionary<string, object> values)
        {
            var names = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
            foreach (var value in values)
            {
                string name = value.Key.StartsWith("@", StringComparison.Ordinal) ? value.Key.Substring(1) : value.Key;
                if (!IsSafeIdentifier(name) || !names.Add(name) || !IsScalar(value.Value))
                    throw new InvalidOperationException("Crystal binding names or values are invalid.");
            }
        }

        private static bool IsScalar(object value)
        {
            return value == null || value is string || value is bool || value is byte || value is short
                || value is int || value is long || value is float || value is double || value is decimal;
        }

        private static bool IsSafeStoredProcedure(string value)
        {
            string[] parts = value.Split('.');
            return (parts.Length == 1 || parts.Length == 2) && parts.All(IsSafeIdentifier);
        }

        private static bool IsSafeIdentifier(string value)
        {
            return !String.IsNullOrEmpty(value) && value.Length <= 128
                && (IsAsciiLetter(value[0]) || value[0] == '_')
                && value.All(character => IsAsciiLetter(character) || (character >= '0' && character <= '9') || character == '_');
        }

        private static bool IsAsciiLetter(char value)
        {
            return (value >= 'A' && value <= 'Z') || (value >= 'a' && value <= 'z');
        }

        private static string RequiredString(Dictionary<string, object> values, string name)
        {
            string value = OptionalString(values, name);
            if (String.IsNullOrWhiteSpace(value)) throw new InvalidOperationException("Crystal request requires " + name + ".");
            return value;
        }

        private static string OptionalString(Dictionary<string, object> values, string name)
        {
            object raw;
            if (!values.TryGetValue(name, out raw) || raw == null) return String.Empty;
            string value = raw as string;
            if (value == null) throw new InvalidOperationException("Crystal request field must be a string: " + name);
            return value;
        }

        private static Dictionary<string, object> RequiredDictionary(Dictionary<string, object> values, string name)
        {
            object raw;
            Dictionary<string, object> result;
            if (!values.TryGetValue(name, out raw) || (result = raw as Dictionary<string, object>) == null)
                throw new InvalidOperationException("Crystal request field must be an object: " + name);
            return result;
        }

        private static Dictionary<string, object> OptionalDictionary(Dictionary<string, object> values, string name)
        {
            object raw;
            return !values.TryGetValue(name, out raw) || raw == null
                ? new Dictionary<string, object>() : RequiredDictionary(values, name);
        }

        internal static DataSet DeserializeDataSet(string dataSetXml)
        {
            var dataSet = new DataSet();
            try
            {
                using (var text = new StringReader(dataSetXml))
                using (XmlReader reader = XmlReader.Create(text, new XmlReaderSettings { XmlResolver = null, DtdProcessing = DtdProcessing.Prohibit }))
                    dataSet.ReadXml(reader, XmlReadMode.ReadSchema);
                return dataSet;
            }
            catch { dataSet.Dispose(); throw; }
        }

    }

    internal sealed class BridgeRequest
    {
        public bool IsTypedProtocol { get; set; }
        public string ReportPath { get; set; }
        public string OutputPath { get; set; }
        public string ConnectionString { get; set; }
        public string Query { get; set; }
        public string StoredProcedure { get; set; }
        public string QueryType { get; set; }
        public bool UseTemplateData { get; set; }
        public Dictionary<string, object> QueryParams { get; set; }
        public Dictionary<string, object> ReportVariables { get; set; }
        public string DataSetXml { get; set; }
        public string ExportFormat { get; set; }
    }

    internal static class BridgePathPolicy
    {
        private static StringComparison PathComparison => OperatingSystem.IsWindows() ? StringComparison.OrdinalIgnoreCase : StringComparison.Ordinal;
        private const string FilePrefix = "crystal-";
        private const string RequestSuffix = ".request.json";

        internal static string ValidateRoot(string path)
        {
            if (String.IsNullOrWhiteSpace(path))
            {
                throw new InvalidOperationException();
            }

            string full = TrimEndingSeparator(Path.GetFullPath(path));
            if (!Directory.Exists(full))
            {
                throw new InvalidOperationException();
            }

            RejectReparseAncestors(full);
            return full;
        }

        internal static string ValidateRequestFile(string tempRoot, string requestPath)
        {
            string full = Path.GetFullPath(requestPath);
            string nonce;
            if (!String.Equals(Path.GetDirectoryName(full), tempRoot, PathComparison) ||
                !TryParseOwnedFileName(Path.GetFileName(full), RequestSuffix, out nonce) ||
                !File.Exists(full))
            {
                throw new InvalidOperationException();
            }

            RejectReparseAncestors(full);
            return full;
        }

        internal static string ValidateTemplateFile(string templateRoot, string reportPath)
        {
            string full = Path.GetFullPath(reportPath);
            if (!IsContained(templateRoot, full) ||
                !String.Equals(Path.GetExtension(full), ".rpt", StringComparison.OrdinalIgnoreCase) ||
                !File.Exists(full))
            {
                throw new InvalidOperationException();
            }

            RejectReparseAncestors(full);
            return full;
        }

        internal static string ValidateOutputPath(string requestPath, string outputPath, string exportFormat = "pdf")
        {
            if (exportFormat != "pdf" && exportFormat != "excel") throw new InvalidOperationException();
            string outputSuffix = exportFormat == "excel" ? ".output.xls" : ".output.pdf";
            string request = Path.GetFullPath(requestPath);
            string full = Path.GetFullPath(outputPath);
            string requestNonce;
            string outputNonce;
            if (!TryParseOwnedFileName(Path.GetFileName(request), RequestSuffix, out requestNonce) ||
                !TryParseOwnedFileName(Path.GetFileName(full), outputSuffix, out outputNonce) ||
                !String.Equals(requestNonce, outputNonce, StringComparison.Ordinal) ||
                !String.Equals(Path.GetDirectoryName(full), Path.GetDirectoryName(request), PathComparison) ||
                !File.Exists(full))
            {
                throw new InvalidOperationException();
            }

            RejectReparseAncestors(full);
            return full;
        }

        private static bool TryParseOwnedFileName(string name, string suffix, out string nonce)
        {
            nonce = null;
            if (name == null ||
                name.Length != FilePrefix.Length + 32 + suffix.Length ||
                !name.StartsWith(FilePrefix, StringComparison.Ordinal) ||
                !name.EndsWith(suffix, StringComparison.Ordinal))
            {
                return false;
            }

            nonce = name.Substring(FilePrefix.Length, 32);
            return nonce.All(character =>
                (character >= '0' && character <= '9') ||
                (character >= 'a' && character <= 'f'));
        }

        private static bool IsContained(string root, string path)
        {
            return path.StartsWith(root + Path.DirectorySeparatorChar, PathComparison);
        }

        private static void RejectReparseAncestors(string path)
        {
            FileSystemInfo current = File.Exists(path)
                ? (FileSystemInfo)new FileInfo(path)
                : new DirectoryInfo(path);
            while (current != null)
            {
                if ((current.Attributes & FileAttributes.ReparsePoint) != 0)
                {
                    throw new InvalidOperationException();
                }

                current = current is FileInfo ? ((FileInfo)current).Directory : ((DirectoryInfo)current).Parent;
            }
        }

        private static string TrimEndingSeparator(string path)
        {
            string root = Path.GetPathRoot(path);
            string trimmed = path.TrimEnd(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar);
            return trimmed.Length < root.Length ? root : trimmed;
        }
    }
}
