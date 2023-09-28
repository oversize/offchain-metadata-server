FROM rust:1.72 AS builder
WORKDIR /app

RUN apt-get -y update && apt-get -y install lld clang
COPY . .
RUN cargo build --release

# Copy bin from builder
FROM rust:1.72-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/tokenapi tokenapi
USER 1000
EXPOSE 8080
ENV LISTEN="0.0.0.0:8080"
ENV MAPPINGS="/registry"
CMD ["./tokenapi"]
