FROM rust:1.89.0-slim AS build

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
RUN cargo build --release

FROM rust:1.89.0-slim
WORKDIR /app
COPY --from=build /app/target/release/ftp2webdav-rs ./
CMD ["./ftp2webdav-rs"]
