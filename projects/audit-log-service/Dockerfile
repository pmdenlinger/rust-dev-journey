
# Simple container build (developer convenience)
FROM rust:1 AS builder
WORKDIR /app
COPY ../../Cargo.toml ../../Cargo.lock* ./
# Workaround: create minimal workspace to cache deps (adjust in real build)
RUN mkdir -p projects/audit-log-service/src && echo "fn main(){}" > projects/audit-log-service/src/main.rs
COPY . .
RUN cargo build -p audit-log-service --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/audit-log-service /usr/local/bin/audit-log-service
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/audit-log-service"]
