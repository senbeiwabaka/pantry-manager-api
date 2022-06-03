FROM rust:1.61.0 AS builder

WORKDIR /src

COPY ./ .

RUN cargo build --release

FROM rust:1.61.0-slim

WORKDIR /src

COPY --from=builder /src/Rocket.toml /app
COPY --from=builder /src/target/release/* /app/

WORKDIR /app

EXPOSE 8000

CMD ["pantry-manager-api.exe"]