using System.Data;
using System.Globalization;
using System.Text;
using Microsoft.Data.SqlClient;
namespace WCF.Service
{
    internal static class DataAccess
    {
        internal static DataSet Load(BridgeRequest request)
        {
            if (request.UseTemplateData)
                throw new NotSupportedException("Embedded template SQL is not yet compatible; no saved-data fallback is permitted.");
            if (!String.IsNullOrWhiteSpace(request.DataSetXml))
            {
                DataSet data = Contract.DeserializeDataSet(request.DataSetXml);
                try { ValidateDataSet(data, request); return data; }
                catch { data.Dispose(); throw; }
            }
            using var connection = Utils.GetConnection(request.ConnectionString);
            using var command = new SqlCommand(request.IsTypedProtocol ? request.StoredProcedure : request.Query, connection);
            command.CommandTimeout = 0;
            command.CommandType = request.IsTypedProtocol || String.Equals(request.QueryType, "stored_procedure", StringComparison.OrdinalIgnoreCase)
                ? CommandType.StoredProcedure : CommandType.Text;
            connection.Open();
            if (request.IsTypedProtocol) { SqlCommandBuilder.DeriveParameters(command); BindDeclaredParameters(command, request.QueryParams); }
            else ApplySqlCommandParameters(command, request.QueryParams);
            DataSet result = FillSqlDataSet(command, "report query result");
            try
            {
                ValidateDataSet(result, request);
                if (request.ExportFormat == "pdf") NormalizeText(result.Tables[0], request.IsTypedProtocol);
                return result;
            }
            catch { result.Dispose(); throw; }
        }
        internal static void ValidateDataSet(DataSet data, BridgeRequest request)
        {
            if (data == null || data.Tables.Count == 0)
                throw new InvalidOperationException("The report query or dataset did not supply a data table.");
            if (request.IsTypedProtocol)
                foreach (DataTable table in data.Tables) ValidateResultScope(table, request.QueryParams);
        }

        internal static void ValidateResultScope(DataTable table, Dictionary<string, object> supplied)
        {
            foreach (string name in new[] { "StudentID", "SchoolID", "ClassID", "SectionID", "TermID", "ReportID" })
            {
                object expected;
                if ((!supplied.TryGetValue(name, out expected) && !supplied.TryGetValue("@" + name, out expected)) ||
                    !table.Columns.Contains(name)) continue;
                int expectedId = Convert.ToInt32(expected, CultureInfo.InvariantCulture);
                if (expectedId <= 0) throw new InvalidOperationException("Crystal report scope must be positive: " + name);
                foreach (DataRow row in table.Rows)
                    if (row.IsNull(name) || Convert.ToInt32(row[name], CultureInfo.InvariantCulture) != expectedId)
                        throw new InvalidOperationException("Crystal procedure returned data outside the requested scope: " + name);
            }
        }

        internal static void BindDeclaredParameters(SqlCommand command, Dictionary<string, object> supplied)
        {
            Dictionary<string, object> remaining = new Dictionary<string, object>(StringComparer.OrdinalIgnoreCase);
            foreach (KeyValuePair<string, object> pair in supplied)
            {
                string name = pair.Key.StartsWith("@", StringComparison.Ordinal) ? pair.Key : "@" + pair.Key;
                if (remaining.ContainsKey(name))
                {
                    throw new InvalidOperationException();
                }

                remaining.Add(name, pair.Value);
            }

            bool studentBound = false;
            foreach (SqlParameter parameter in command.Parameters)
            {
                if (parameter.Direction == ParameterDirection.ReturnValue || parameter.Direction == ParameterDirection.Output)
                {
                    continue;
                }

                object value;
                if (!remaining.TryGetValue(parameter.ParameterName, out value))
                {
                    throw new InvalidOperationException("Missing Crystal SQL parameter: " + parameter.ParameterName);
                }

                parameter.Value = value ?? DBNull.Value;
                if (String.Equals(parameter.ParameterName, "@StudentID", StringComparison.OrdinalIgnoreCase))
                {
                    if (value == null || Convert.ToInt32(value, CultureInfo.InvariantCulture) <= 0)
                        throw new InvalidOperationException("Crystal SQL student scope must be positive.");
                    studentBound = true;
                }
                remaining.Remove(parameter.ParameterName);
            }

            // The review flag was added after the original eight-parameter evaluation procedure.
            // All other supplied selectors must be declared so schema drift fails closed.
            remaining.Remove("@ForReview");
            if (remaining.Count != 0)
                throw new InvalidOperationException("Crystal SQL procedure does not declare a supplied parameter.");
            if (!studentBound) throw new InvalidOperationException("Crystal SQL procedure must declare StudentID.");
        }

        private static void NormalizeText(DataTable table, bool decompose)
        {
            foreach (DataRow row in table.Rows)
            {
                foreach (DataColumn column in table.Columns)
                {
                    string value = row[column] as string;
                    if (value != null)
                    {
                        row[column] = RemoveDiacritics(value, decompose);
                    }
                }
            }
        }

        private static string RemoveDiacritics(string text, bool decompose)
        {
            string normalized = text.Normalize(decompose ? NormalizationForm.FormD : NormalizationForm.FormC);
            StringBuilder result = new StringBuilder(normalized.Length);
            foreach (char character in normalized)
            {
                if (CharUnicodeInfo.GetUnicodeCategory(character) != UnicodeCategory.NonSpacingMark)
                {
                    result.Append(character);
                }
            }

            return result.ToString().Normalize(NormalizationForm.FormC);
        }

        private static DataSet FillSqlDataSet(SqlCommand cmd, string operation)
        {
            DataSet dataSet = new DataSet();
            try
            {
                using (SqlDataAdapter adapter = new SqlDataAdapter())
                {
                    adapter.SelectCommand = cmd;
                    adapter.Fill(dataSet);
                    return dataSet;
                }
            }
            catch (Exception ex)
            {
                dataSet.Dispose();
                StringBuilder builder = new StringBuilder("Crystal report SQL execution failed.");
                AppendDetail(builder, "Operation", operation);
                AppendSqlCommandContext(builder, cmd);
                AppendExceptionDetails(builder, ex);
                string error = builder.ToString();
                Utils.writeToErrorLog(error + Environment.NewLine + ex);
                throw new ApplicationException(error, ex);
            }
        }

        private static void ApplySqlCommandParameters(SqlCommand cmd, Dictionary<string, object> queryParams)
        {
            if (cmd.CommandType == CommandType.StoredProcedure)
            {
                ApplyStoredProcedureParameters(cmd, queryParams);
                return;
            }

            ApplySqlTextParameters(cmd, queryParams);
        }

        private static void ApplySqlTextParameters(SqlCommand cmd, Dictionary<string, object> queryParams)
        {
            List<string> parameterNames = GetSqlTextParameterNames(cmd.CommandText);
            if (parameterNames.Count == 0)
            {
                return;
            }

            Dictionary<string, object> values = BuildSqlParameterValueMap(queryParams);
            List<string> missingParameters = new List<string>();
            foreach (string parameterName in parameterNames)
            {
                object value;
                if (TryGetSqlParameterValue(values, parameterName, out value))
                {
                    cmd.Parameters.Add(new SqlParameter(parameterName, value ?? DBNull.Value));
                    continue;
                }

                missingParameters.Add(parameterName);
            }

            if (missingParameters.Count > 0)
            {
                string error = BuildMissingSqlTextParameterError(cmd, missingParameters, parameterNames, queryParams);
                Utils.writeToErrorLog(error);
                throw new ApplicationException(error);
            }
        }

        private static void ApplyStoredProcedureParameters(SqlCommand cmd, Dictionary<string, object> queryParams)
        {
            try
            {
                SqlCommandBuilder.DeriveParameters(cmd);
            }
            catch (Exception ex)
            {
                string error = BuildStoredProcedureParameterDiscoveryError(cmd, ex);
                Utils.writeToErrorLog(error + Environment.NewLine + ex);
                throw new ApplicationException(error, ex);
            }

            BindLegacyDeclaredParameters(cmd, queryParams);
        }

        internal static void BindLegacyDeclaredParameters(SqlCommand cmd, Dictionary<string, object> queryParams)
        {
            Dictionary<string, object> values = BuildSqlParameterValueMap(queryParams);
            List<string> missingParameters = new List<string>();
            List<SqlParameter> parametersToRemove = new List<SqlParameter>();
            foreach (SqlParameter parameter in cmd.Parameters)
            {
                if (!IsSqlInputParameter(parameter))
                {
                    continue;
                }

                object value;
                if (TryGetSqlParameterValue(values, parameter.ParameterName, out value))
                {
                    parameter.Value = value ?? DBNull.Value;
                    continue;
                }

                if (parameter.Direction == ParameterDirection.Input)
                {
                    parametersToRemove.Add(parameter);
                    continue;
                }

                missingParameters.Add(parameter.ParameterName);
            }

            foreach (SqlParameter parameter in parametersToRemove)
            {
                cmd.Parameters.Remove(parameter);
            }

            if (missingParameters.Count > 0)
            {
                string error = BuildMissingStoredProcedureParameterError(cmd, missingParameters, queryParams);
                Utils.writeToErrorLog(error);
                throw new ApplicationException(error);
            }
        }

        private static bool IsSqlInputParameter(SqlParameter parameter)
        {
            return parameter.Direction == ParameterDirection.Input
                || parameter.Direction == ParameterDirection.InputOutput;
        }

        private static Dictionary<string, object> BuildSqlParameterValueMap(Dictionary<string, object> queryParams)
        {
            Dictionary<string, object> values = new Dictionary<string, object>(StringComparer.OrdinalIgnoreCase);
            if (queryParams == null)
            {
                return values;
            }

            foreach (string key in queryParams.Keys)
            {
                AddSqlParameterValue(values, key, queryParams[key]);
            }

            return values;
        }

        private static void AddSqlParameterValue(Dictionary<string, object> values, string parameterName, object value)
        {
            if (String.IsNullOrEmpty(parameterName))
            {
                return;
            }

            string normalizedName = NormalizeSqlParameterName(parameterName);
            if (!String.IsNullOrEmpty(normalizedName))
            {
                object existing;
                if (values.TryGetValue(normalizedName, out existing) && !Object.Equals(existing, value))
                    throw new InvalidOperationException("Conflicting Crystal SQL parameter aliases: " + normalizedName);
                values[parameterName] = value;
                values[normalizedName] = value;
                values["@" + normalizedName] = value;
            }
        }

        private static bool TryGetSqlParameterValue(Dictionary<string, object> values, string parameterName, out object value)
        {
            if (values.TryGetValue(parameterName, out value))
            {
                return true;
            }

            string normalizedName = NormalizeSqlParameterName(parameterName);
            if (values.TryGetValue(normalizedName, out value))
            {
                return true;
            }

            if (!String.IsNullOrEmpty(normalizedName) && values.TryGetValue("@" + normalizedName, out value))
            {
                return true;
            }

            value = null;
            return false;
        }

        private static string NormalizeSqlParameterName(string parameterName)
        {
            if (String.IsNullOrEmpty(parameterName))
            {
                return parameterName;
            }

            return parameterName.StartsWith("@", StringComparison.Ordinal)
                ? parameterName.Substring(1)
                : parameterName;
        }

        internal static List<string> GetSqlTextParameterNames(string commandText)
        {
            List<string> parameterNames = new List<string>();
            HashSet<string> foundNames = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
            if (String.IsNullOrEmpty(commandText))
            {
                return parameterNames;
            }

            for (int index = 0; index < commandText.Length; index++)
            {
                char value = commandText[index];
                if (value == '\'' || value == '[' || value == '"')
                {
                    index = SkipSqlDelimitedValue(commandText, index, value == '[' ? ']' : value);
                    continue;
                }

                if (value == '-' && index + 1 < commandText.Length && commandText[index + 1] == '-')
                {
                    index = SkipSqlLineComment(commandText, index + 2);
                    continue;
                }

                if (value == '/' && index + 1 < commandText.Length && commandText[index + 1] == '*')
                {
                    index = SkipSqlBlockComment(commandText, index + 2);
                    continue;
                }

                if (value != '@')
                {
                    continue;
                }

                if (index + 1 < commandText.Length && commandText[index + 1] == '@')
                {
                    index++;
                    continue;
                }
                if (index + 1 >= commandText.Length
                    || !IsSqlParameterNameStart(commandText[index + 1]))
                {
                    continue;
                }

                int end = index + 2;
                while (end < commandText.Length && IsSqlParameterNamePart(commandText[end]))
                {
                    end++;
                }

                string parameterName = commandText.Substring(index, end - index);
                if (!foundNames.Contains(parameterName))
                {
                    foundNames.Add(parameterName);
                    parameterNames.Add(parameterName);
                }

                index = end - 1;
            }

            return parameterNames;
        }

        private static int SkipSqlDelimitedValue(string commandText, int startIndex, char closingDelimiter)
        {
            for (int index = startIndex + 1; index < commandText.Length; index++)
            {
                if (commandText[index] != closingDelimiter)
                {
                    continue;
                }

                if (index + 1 < commandText.Length && commandText[index + 1] == closingDelimiter)
                {
                    index++;
                    continue;
                }

                return index;
            }

            return commandText.Length - 1;
        }

        private static int SkipSqlLineComment(string commandText, int startIndex)
        {
            for (int index = startIndex; index < commandText.Length; index++)
            {
                if (commandText[index] == '\r' || commandText[index] == '\n')
                {
                    return index;
                }
            }

            return commandText.Length - 1;
        }

        private static int SkipSqlBlockComment(string commandText, int startIndex)
        {
            int depth = 1;
            for (int index = startIndex; index + 1 < commandText.Length; index++)
            {
                if (commandText[index] == '/' && commandText[index + 1] == '*')
                {
                    depth++;
                    index++;
                }
                else if (commandText[index] == '*' && commandText[index + 1] == '/')
                {
                    if (--depth == 0) return index + 1;
                    index++;
                }
            }

            return commandText.Length - 1;
        }

        private static bool IsSqlParameterNameStart(char value)
        {
            return Char.IsLetter(value) || value == '_';
        }

        private static bool IsSqlParameterNamePart(char value)
        {
            return Char.IsLetterOrDigit(value) || value == '_' || value == '$' || value == '#';
        }

        private static string BuildStoredProcedureParameterDiscoveryError(SqlCommand cmd, Exception ex)
        {
            StringBuilder builder = new StringBuilder("Stored procedure parameter discovery failed.");
            AppendDetail(builder, "Procedure", cmd.CommandText);
            AppendExceptionDetails(builder, ex);
            return builder.ToString();
        }

        private static string BuildMissingStoredProcedureParameterError(SqlCommand cmd, List<string> missingParameters, Dictionary<string, object> queryParams)
        {
            StringBuilder builder = new StringBuilder("Missing required stored procedure parameter.");
            AppendDetail(builder, "Procedure", cmd.CommandText);
            AppendDetail(builder, "MissingParameters", String.Join(", ", missingParameters.ToArray()));
            AppendDetail(builder, "AvailableProcedureParameters", GetSqlCommandParameterNames(cmd));
            AppendDetail(builder, "ProvidedParameterNames", GetProvidedParameterNames(queryParams));
            return builder.ToString();
        }

        private static string BuildMissingSqlTextParameterError(SqlCommand cmd, List<string> missingParameters, List<string> queryParameterNames, Dictionary<string, object> queryParams)
        {
            StringBuilder builder = new StringBuilder("Missing required SQL query parameter.");
            AppendDetail(builder, "MissingParameters", String.Join(", ", missingParameters.ToArray()));
            AppendDetail(builder, "SqlQueryParameters", String.Join(", ", queryParameterNames.ToArray()));
            AppendDetail(builder, "ProvidedParameterNames", GetProvidedParameterNames(queryParams));
            AppendDetail(builder, "Query", FormatDiagnosticValue(cmd.CommandText));
            return builder.ToString();
        }

        private static string GetSqlCommandParameterNames(SqlCommand cmd)
        {
            List<string> parameterNames = new List<string>();
            foreach (SqlParameter parameter in cmd.Parameters)
            {
                parameterNames.Add(parameter.ParameterName);
            }

            return parameterNames.Count == 0 ? "none" : String.Join(", ", parameterNames.ToArray());
        }

        private static string GetProvidedParameterNames(Dictionary<string, object> values)
        {
            if (values == null || values.Count == 0)
            {
                return "none";
            }

            return String.Join(", ", values.Keys.ToArray());
        }

        private static void AppendSqlCommandContext(StringBuilder builder, SqlCommand cmd)
        {
            if (cmd == null)
            {
                AppendDetail(builder, "CommandText", "none");
                AppendDetail(builder, "CommandType", "none");
                AppendDetail(builder, "BoundSqlParameters", "none");
                return;
            }

            AppendDetail(builder, "CommandText", FormatDiagnosticValue(cmd.CommandText));
            AppendDetail(builder, "CommandType", cmd.CommandType.ToString());
            AppendDetail(builder, "CommandTimeout", cmd.CommandTimeout.ToString());
            AppendDetail(builder, "BoundSqlParameters", GetSqlCommandParameterDetails(cmd));
        }

        private static string GetSqlCommandParameterDetails(SqlCommand cmd)
        {
            if (cmd.Parameters.Count == 0)
            {
                return "none";
            }

            List<string> parameterDetails = new List<string>();
            foreach (SqlParameter parameter in cmd.Parameters)
            {
                parameterDetails.Add(parameter.ParameterName
                    + "="
                    + FormatDiagnosticValue(parameter.Value)
                    + " Direction="
                    + parameter.Direction);
            }

            return String.Join(", ", parameterDetails.ToArray());
        }

        private static string FormatDiagnosticValue(object value)
        {
            if (value == null)
            {
                return "null";
            }

            try
            {
                string text = Convert.ToString(value, CultureInfo.InvariantCulture);
                if (text == null)
                {
                    return "";
                }

                return text;
            }
            catch (Exception ex)
            {
                return "unavailable: " + FormatSingleException(ex);
            }
        }

        private static void AppendExceptionDetails(StringBuilder builder, Exception ex)
        {
            if (ex == null)
            {
                AppendDetail(builder, "ExceptionType", "none");
                AppendDetail(builder, "HResult", "none");
                AppendDetail(builder, "Message", "none");
                AppendDetail(builder, "InnerException", "none");
                AppendDetail(builder, "BaseException", "none");
                return;
            }

            AppendDetail(builder, "ExceptionType", ex.GetType().FullName);
            AppendDetail(builder, "HResult", "0x" + ex.HResult.ToString("X8"));
            AppendDetail(builder, "Message", ex.Message);
            AppendDetail(builder, "InnerException", FormatSingleException(ex.InnerException));

            Exception baseException = ex.GetBaseException();
            AppendDetail(builder, "BaseException", FormatSingleException(baseException));
        }

        private static string FormatSingleException(Exception ex)
        {
            if (ex == null)
            {
                return "none";
            }

            return ex.GetType().FullName + ": " + ex.Message;
        }

        private static void AppendDetail(StringBuilder builder, string name, string value)
        {
            if (builder.Length > 0)
            {
                builder.Append(" ");
            }

            builder.Append(name);
            builder.Append(": ");
            builder.Append(value ?? "");
            builder.Append(";");
        }
    }
}
