# STAGE 1: Build Stage
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

# STAGE 2: Runtime Stage
FROM debian:bookworm-slim
WORKDIR /usr/local/bin
RUN apt-get update && apt-get install -y python3 python3-pip clang build-essential ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/omni-runtime /usr/local/bin/omni
WORKDIR /app
ENTRYPOINT ["omni"]
