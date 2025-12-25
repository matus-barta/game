# https://github.com/LukeMathWalker/cargo-chef
# Leveraging the pre-built Docker images with 
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-slim-trixie AS chef
WORKDIR /build
RUN sed -i s#http://deb.debian.org/debian-security#http://debcache-komj.home.anonymus09.com/debsec#g /etc/apt/sources.list.d/debian.sources
RUN sed -i s#http://deb.debian.org/debian#http://debcache-komj.home.anonymus09.com/debian#g /etc/apt/sources.list.d/debian.sources
RUN apt-get update
RUN apt-get install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM builder AS final
COPY . .
RUN cargo build --bin asset_server --release

FROM gcr.io/distroless/cc-debian13

COPY --from=final /build/target/release/asset_server /app/asset_server
ENTRYPOINT ["./app/asset_server"]