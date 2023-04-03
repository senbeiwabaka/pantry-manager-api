FROM rust:1.68.2 AS builder

WORKDIR /src

COPY ./ .

RUN cargo build --release

FROM rust:latest

WORKDIR /app

COPY --from=builder /src/Rocket.toml /app
COPY --from=builder /src/Pantry.toml /app
COPY --from=builder /src/target/release/pantry-manager-api /app/
COPY --from=builder /src/target/release/.cargo-lock /app/
COPY --from=builder /src/target/release/pantry-manager-api.d /app/

EXPOSE 8000

ENTRYPOINT  ["./pantry-manager-api"]