FROM rust:1.79-bookworm AS rust-builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release -p echo-server

FROM debian:bookworm-slim AS debian-runtime

RUN mkdir -p /app
WORKDIR /app

COPY --from=rust-builder /usr/src/app/target/release/echo-server /app/echo-server