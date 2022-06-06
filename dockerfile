FROM rust:1.61.0 AS builder

WORKDIR /src

COPY ./ .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /src/Rocket.toml /app
COPY --from=builder /src/target/release/* /app/

RUN ls /app

EXPOSE 8000

ENTRYPOINT  ["/pantry-manager-api"]