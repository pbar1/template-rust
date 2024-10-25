# https://github.com/LukeMathWalker/cargo-chef#running-the-binary-in-alpine

# Using the `rust-musl-builder` as base image, instead of
# the official Rust toolchain
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
ARG ARCH
RUN cargo chef cook --release --target $ARCH-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target $ARCH-unknown-linux-musl --bin template-rust-cli

FROM alpine AS runtime
ARG ARCH
RUN addgroup -S user && adduser -S user -G user
COPY --from=builder /app/target/$ARCH-unknown-linux-musl/release/template-rust-cli /usr/local/bin/
USER user
ENTRYPOINT ["/usr/local/bin/template-rust-cli"]
LABEL org.opencontainers.image.source="https://github.com/pbar1/template-rust"
