FROM rust:1.61.0 AS builder

WORKDIR /src

COPY ./ .

RUN cargo build --release

FROM rust:latest

WORKDIR /app

COPY --from=builder /src/Rocket.toml /app
COPY --from=builder /src/Pantry.toml /app
COPY --from=builder /src/target/release/* /app/

EXPOSE 8000

ENTRYPOINT  ["./pantry-manager-api"]