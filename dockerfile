FROM rust:latest AS builder

WORKDIR /src

COPY ./ .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /src/Rocket.toml /app
COPY --from=builder /src/Pantry.toml /app
COPY --from=builder /src/target/release/pantry-manager-api /app/
COPY --from=builder /src/target/release/pantry-manager-api.d /app/

EXPOSE 8000

ENTRYPOINT  ["./pantry-manager-api"]