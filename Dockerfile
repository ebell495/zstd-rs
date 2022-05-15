# FROM fuzzers/cargo-fuzz:0.10.0
# rustup toolchain install nightly
# cargo +nightly fuzz build
FROM rust:latest as builder
RUN rustup toolchain install nightly && cargo install cargo-fuzz
# WORKDIR /zstd-rs
COPY . /zstd-rs/
WORKDIR /zstd-rs/fuzz
RUN cargo +nightly fuzz build

FROM debian:bullseye-slim
COPY --from=builder /zstd-rs/fuzz/target/x86_64-unknown-linux-gnu/release/ .