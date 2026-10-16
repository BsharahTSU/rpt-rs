# Local clock parity for Docker comparisons

## Finding and correction (2026-10-16)

The experimental worker previously called the upstream `rpt_render::default_as_of()`, which
captures UTC. The native Windows Crystal runtime uses local calendar values for `CurrentDate`,
`Today`, `CurrentTime` and `CurrentDateTime`. Passing only locale and a short-date pattern did
not align their clocks. At 00:15 in UTC+03, the old worker could print the preceding date even
when both engines had identical report rows.

The STC launcher now captures the Windows host's `DateTime.Now` once for each request and
passes its naive local calendar fields through `STC_RPT_LOCAL_DATETIME`. The portable adapter
validates this value before loading data and puts it in prepared worker input as `as_of_local`.
The worker passes the same date/time specials to record selection, formulas and layout. No
UTC conversion or timezone offset is applied. The upstream renderer clock is unchanged.

Direct adapter callers that omit the environment variable use that process's `DateTime.Now`.
If they run inside a UTC container without the launcher, their process-local time is UTC;
callers needing Windows-host semantics must supply the captured host value.

## Input contract and frozen replays

The exact format is `yyyy-MM-ddTHH:mm:ss`, using ASCII digits, a valid Gregorian date in years
0001 through 9999, and whole seconds. Offsets, `Z`, fractional seconds, whitespace, impossible
dates, 24-hour rollover and leap seconds are rejected. Malformed explicit values never silently
fall back to the current clock. The launcher's optional argument is validated before Docker is
invoked; the adapter validates before data/SQL loading; the worker validates before RPT loading.

For normal bridge requests, use `Invoke-DockerBridge.ps1` as before. To replay a documented
local clock, add:

```powershell
-AsOfLocal '2026-10-16T00:15:07'
```

This is a verification override, not a business report parameter. It does not change the system
clock or the native Crystal process. Record the native execution window and clock-dependent
fields when judging a comparison. A historical Rust replay cannot by itself freeze a freshly
executed native report's clock.

A prepared worker JSON can include:

```json
"as_of_local": "2026-10-16T00:15:07"
```

Clock precedence for direct worker calls is:

1. The prepared `as_of_local` value, which is authoritative for a frozen replay.
2. `STC_RPT_LOCAL_DATETIME`, if the prepared value is absent/null.
3. The previous upstream UTC system-clock fallback, for backward compatibility when both are
   absent. Such old prepared inputs remain readable but are not deterministic clock captures.

Capture the local clock in every new private case manifest and prepared input. Do not rely on
step 3 for a native-versus-Rust acceptance comparison. The existing typed and legacy bridge JSON
schemas are unchanged; the additional field exists only in the internal prepared-data protocol.

## Source locations

- `scripts/stc/Invoke-DockerBridge.ps1`: optional `AsOfLocal`, strict validation and host capture.
- `bridge/Stc.CrystalBridge/Program.cs`: `CaptureLocalClock`, validation before data access and
  prepared `as_of_local` transport.
- `apps/stc-render-worker/src/main.rs`: optional prepared field, strict local-calendar parsing,
  override precedence and a single clock supplied to data/formula/layout contexts.

## Verification

Focused verification on 2026-10-16:

- Docker worker suite: **10 passed**, including three new regressions. Formula evaluation at
  local 00:15 resolves all four date/time specials to the local next day instead of the prior
  UTC day. Replays preserve their prepared clock, older inputs remain accepted, and malformed
  dates/times/offsets are rejected; Gregorian leap-century boundaries are covered.
- Strict Docker Clippy for `stc-render-worker --all-targets`: passed with warnings denied.
- Portable bridge contract suite in Docker: **39 passed**, including 11 added clock checks.
  Synthetic schema-bearing XML and an intentionally failing mock worker verify that two exact
  local clocks reach prepared input unchanged. Direct clock checks cover valid values and the
  process-local `DateTime.Now` default; six malformed overrides fail before deliberately invalid
  XML can be loaded. These regressions are in `bridge/Stc.CrystalBridge.Tests/Program.cs`.
- Tracked Windows launcher suite `scripts/stc/Test-ClockContract.ps1`, with Docker mocked:
  **7 checks passed**. Five malformed overrides were
  rejected before Docker, an explicit near-midnight value was forwarded unchanged, and the normal
  value fell within the host-local invocation window.

The tests used no database connections or school data and did not render PDFs. The normal worker
binary and portable adapter were rebuilt after the changes for subsequent report comparisons. These checks
establish clock transport and formula behavior; each real report still requires its own content,
pagination and visual comparison.

Reproduce the committed suites with (the bridge test launcher also runs the clock launcher checks):

```powershell
./scripts/stc/Invoke-DockerBridge.ps1 -Test
./scripts/stc/Invoke-DockerCargo.ps1 -CargoArgs @('clippy', '--locked', '-p', 'stc-render-worker', '--all-targets', '--', '-D', 'warnings')
```
