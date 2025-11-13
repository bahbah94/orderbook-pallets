FROM rust:1.83-slim AS builder
WORKDIR /workspace

RUN rustup toolchain install nightly && \
    rustup default nightly

COPY Cargo.toml Cargo.lock ./
COPY indexer/Cargo.toml indexer/Cargo.toml
COPY . .

RUN cargo build --release --locked --manifest-path indexer/Cargo.toml

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/local/bin

COPY --from=builder /workspace/target/release/orderbook-indexer .

ENV RUST_LOG=info
ENV DATABASE_URL=postgres://postgres:password@timescaledb:5432/polkadot_clob
ENV ENV=dev

EXPOSE 8081
CMD ["./orderbook-indexer"]
