# Telemetry

Common `tracing` layers preconfigured, as well as global subscriber
initialization. Enable/disable the layers with Cargo features.

| Cargo Feature | Enabled by default? | Description                           |
| ------------- | ------------------- | ------------------------------------- |
| `lines`       | 🟢                  | Logging lines to a file/stdout/stderr |
| `otel`        |                     | OpenTelemetry support                 |
| `console`     |                     | `tokio-console` support               |
