# https://github.com/LukeMathWalker/cargo-chef
# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-slim-trixie AS chef
WORKDIR /build
RUN apt-get update
RUN apt-get install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM builder AS final
RUN cargo build -p asset_server --release

FROM gcr.io/distroless/cc-debian12

COPY --from=final /build/target/release/asset_server /app/asset_server
ENTRYPOINT ["./app/asset_server"]