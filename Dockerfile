# Build Stage
FROM rust:1.63.0 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release

# Run Stage
FROM debian:bullseye-slim AS runtime

RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	# Clean up
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY config config
ENV RUN_TYPE prod
ENTRYPOINT ["./zero2prod"]
