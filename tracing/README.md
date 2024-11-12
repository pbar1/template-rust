# `tracing-pbar`

Common `tracing` layers preconfigured.

Enable/disable the layers with Cargo features.

| Cargo Feature | Description                           | Enabled by default? |
| ------------- | ------------------------------------- | ------------------- |
| `lines`       | Logging lines to a file/stdout/stderr | 🟢                  |
| `otel`        | OpenTelemetry support                 |                     |
| `console`     | `tokio-console` support               |                     |
