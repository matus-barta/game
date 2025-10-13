FROM rust:1.90.0-slim-bookworm AS builder

WORKDIR /build

COPY . .

WORKDIR /build

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /build/target/release/asset_server /app/asset_server
# COPY --from=builder /build/assets /app/assets

ENTRYPOINT ["./app/asset_server"]