# syntax=docker.io/docker/dockerfile:1.7-labs

# ================================================================================
#  Build crate
# ================================================================================
FROM rust:1.83-bullseye AS build

ARG CRATE=goodbye-world

RUN mkdir -p /build
WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY .cargo .cargo
RUN cargo init crates/$CRATE
COPY crates/$CRATE/Cargo.toml crates/$CRATE/

RUN cargo build --release -p $CRATE

COPY crates/$CRATE/src crates/$CRATE/src
RUN touch crates/$CRATE/src/main.rs

RUN cargo build --release -p $CRATE

RUN mv target/release/$CRATE service

# ================================================================================
#  Runtime configuration
# ================================================================================
FROM debian:bullseye-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates curl

COPY --from=build /build/service /root/service

WORKDIR /root

ENTRYPOINT ["/root/service"]
