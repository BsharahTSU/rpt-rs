using Microsoft.Data.SqlClient;
using System.Data.Common;
using System;
using System.Collections.Generic;
using System.Text;

namespace WCF.Service
{
    internal static class Utils
    {
        internal static SqlConnection GetConnection(string connectionString)
        {
            return new SqlConnection(NormalizeConnectionString(connectionString));
        }

        internal static string NormalizeConnectionString(string connectionString)
        {
            // Match the Framework provider's own keywords, ignoring spelling-only whitespace.
            var incoming = new DbConnectionStringBuilder { ConnectionString = connectionString };
            var compatible = new SqlConnectionStringBuilder();
            var supported = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
            foreach (string key in compatible.Keys) supported[KeywordIdentity(key)] = key;
            foreach (string key in incoming.Keys)
            {
                string targetKey;
                if (!supported.TryGetValue(KeywordIdentity(key), out targetKey)) targetKey = key;
                compatible[targetKey] = incoming[key];
            }
            // Reporting jobs have no application-imposed SQL connection deadline.
            if (!incoming.Keys.Cast<string>().Any(k => KeywordIdentity(k).Equals("encrypt", StringComparison.OrdinalIgnoreCase))) compatible.Encrypt = SqlConnectionEncryptOption.Optional;
            compatible.ConnectTimeout = 0;
            return compatible.ConnectionString;
        }

        private static string KeywordIdentity(string key)
        {
            var value = new StringBuilder(key.Length);
            foreach (char character in key) if (!Char.IsWhiteSpace(character)) value.Append(character);
            return value.ToString();
        }

        internal static void writeToErrorLog(string message)
        {
            // Transport only: both WebApi callers persist all stderr in the existing report
            // file logger, including warnings from successful exports. Stdout is JSON only.
            Console.Error.WriteLine(message);
        }
    }
}
