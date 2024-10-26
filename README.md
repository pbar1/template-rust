# template-rust

Opinionated template for Rust projects.

### Usage

Replace all references with your own:

```
rg template-rust -l | xargs sed -i 's/template-rust/myproject/g'
rg pbar -l | xargs sed -i 's/pbar/myproject/g'
```

### [CLI](./cli/README.md)

Starting point for a CLI program using `clap`.

### [Telemetry](./telemetry/README.md)

Ready-to-use default config for `tracing`.
