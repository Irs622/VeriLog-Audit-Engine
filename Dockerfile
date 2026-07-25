# Multi-stage Dockerfile for building a release binary of VeriLog Audit Engine Agent
FROM rust:1.78-bookworm as builder
WORKDIR /usr/src/verilog
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev clang libclang-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --workspace --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates sqlite3 libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/verilog/target/release/agent /app/agent
COPY --from=builder /usr/src/verilog/dashboard /app/dashboard

ENV DATABASE_URL=/app/data/verilog.db
ENV DASHBOARD_PATH=/app/dashboard
ENV APP_ENV=production

RUN mkdir -p /app/data
EXPOSE 3000
CMD ["/app/agent"]
