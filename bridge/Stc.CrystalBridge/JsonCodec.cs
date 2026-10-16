using System.Text.Json;
namespace WCF.Service;
internal sealed class JsonCodec
{
    public int MaxJsonLength { get; set; }
    public T Deserialize<T>(string json)
    {
        using var document = JsonDocument.Parse(json);
        return (T)Read(document.RootElement);
    }
    private static object Read(JsonElement e) => e.ValueKind switch
    {
        JsonValueKind.Object => e.EnumerateObject().ToDictionary(p => p.Name, p => Read(p.Value)),
        JsonValueKind.Array => e.EnumerateArray().Select(Read).ToArray(),
        JsonValueKind.String => e.GetString(),
        JsonValueKind.Number => e.TryGetInt32(out var i) ? (object)i : e.TryGetInt64(out var l) ? (object)l : e.TryGetDecimal(out var d) ? (object)d : e.GetDouble(),
        JsonValueKind.True => true, JsonValueKind.False => false, JsonValueKind.Null => null,
        _ => throw new InvalidOperationException("Unsupported JSON value.")
    };
}
