# Multi-stage Dockerfile for building a release binary of the agent
FROM rust:1.70 as builder
WORKDIR /usr/src/verilog
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev clang libclang-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --workspace --release

FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/verilog/target/release/agent ./agent
ENV DATABASE_URL=/data/verilog.db
EXPOSE 3000
CMD ["./agent"]
