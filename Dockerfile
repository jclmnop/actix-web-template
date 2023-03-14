# TODO: optimise
FROM lukemathwalker/cargo-chef:latest-rust-1.68.0 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build stage
FROM chef AS build
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies separetly for caching purposes
RUN cargo chef cook --release --recipe-path recipe.json
# Build app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin actix-web-template

# Run stage
FROM debian:bullseye-slim AS run
WORKDIR /app
# OpenSSL - Dynamically linked by some dependencies
# ca-certificates - Needed to verify TLS certifications for HTTPS
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/actix-web-template actix-web-template

COPY config config

ENV APP_ENVIRONMENT prod

ENTRYPOINT ["./actix-web-template"]