# syntax=docker/dockerfile:1.3
FROM rust:1.54-alpine3.14 as base

WORKDIR /app

COPY ./Cargo.toml /app/
COPY ./src/ /app/src/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
  cargo build --release && \
  mv ./target/release/segfaulter /usr/local/bin

FROM alpine:3.14

RUN adduser -D app

COPY --from=base  /usr/local/bin/segfaulter /usr/local/bin/

CMD ["segfaulter"]