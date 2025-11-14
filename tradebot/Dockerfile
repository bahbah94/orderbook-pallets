FROM rust:1.83-slim AS builder
WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./
COPY tradebot/Cargo.toml tradebot/Cargo.toml

COPY . .

RUN cargo build --release --locked --manifest-path tradebot/Cargo.toml

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /workspace/target/release/tradebot /usr/local/bin/tradebot

ENV RUST_LOG=info

CMD ["tradebot"]
