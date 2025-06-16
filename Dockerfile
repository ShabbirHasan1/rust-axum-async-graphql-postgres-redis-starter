FROM rust:latest as build

RUN apt-get update && apt-get install -y \
    cmake \
    libssl-dev \
    build-essential

WORKDIR /usr/src/backend

COPY . .

RUN cargo build --release

FROM ubuntu:22.04

RUN apt-get update -y && apt-get install -y \
    libssl-dev \
    build-essential \
    cmake \
    ca-certificates \
    && update-ca-certificates

COPY --from=build /usr/src/backend/target/release/rust-axum-async-graphql-postgres-redis-starter /usr/local/bin/backend
COPY --from=build /usr/src/backend/.env /usr/local/bin/.env

WORKDIR /usr/local/bin

CMD ["backend"]
