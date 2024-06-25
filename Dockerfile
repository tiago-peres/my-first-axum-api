FROM rust:latest as builder
WORKDIR /usr/src/axum_wasm_postgres
COPY . .
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install --path .

FROM ubuntu:22.04
RUN apt-get update && apt-get install -y ca-certificates libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/axum_wasm_postgres /usr/local/bin/axum_wasm_postgres
CMD ["axum_wasm_postgres"]
