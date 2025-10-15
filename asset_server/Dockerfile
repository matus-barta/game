# https://github.com/LukeMathWalker/cargo-chef
# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1.90.0-slim-bookworm AS chef
WORKDIR /build

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /build/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /build/target/release/asset_server /app/asset_server
# COPY --from=builder /build/assets /app/assets

ENTRYPOINT ["./app/asset_server"]