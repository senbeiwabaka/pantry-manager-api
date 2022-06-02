FROM rust:1.61.0 AS builder

WORKDIR /src

COPY ./ .

RUN ls

RUN ls /src/src
RUN ls src

#RUN cargo build --release

#FROM rust:1.61.0-slim

#COPY --from=builder ./target/release /app
#COPY --from=builder ./Rocket.toml /app

#WORKDIR /app

#EXPOSE 8000

#CMD ["pantry-manager-api.exe"]