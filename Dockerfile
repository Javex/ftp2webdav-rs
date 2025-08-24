FROM rust:1.89.0-slim AS build

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

# Use two caches, one for cargo downloads and one for "target" where
# compilation results are stored.
# It's necessary to copy the binary out from the cache so the step below can
# find it. If it's in the cache, COPY will fail below.
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:1.89.0-slim,source=/usr/local/cargo \
  --mount=type=cache,target=target \
  cargo build --release && \
  cp target/release/ftp2webdav-rs ./ftp2webdav-rs

FROM debian:trixie-slim
LABEL org.opencontainers.image.source="https://github.com/Javex/ftp2webdav-rs"
WORKDIR /app
COPY --from=build /app/ftp2webdav-rs ./
ENTRYPOINT ["./ftp2webdav-rs"]
