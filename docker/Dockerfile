# syntax=docker.io/docker/dockerfile:1-labs

FROM rust:1.83-bullseye AS chef
RUN cargo install --locked cargo-chef
WORKDIR /build
ARG BIN

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin $BIN

# ==================================================================================================
#  Build binary
# ==================================================================================================
FROM chef AS build
COPY --from=planner /build/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --no-default-features --release --recipe-path recipe.json --bin $BIN

# Build application
COPY . .
RUN cargo build --release --bin $BIN

RUN mv target/release/$BIN service

# ================================================================================
#  goodbye-world
# ================================================================================
FROM debian:bullseye-slim AS goodbye-world

RUN apt-get update && apt-get install -y ca-certificates curl

COPY --from=build /build/service /root/service

WORKDIR /root

ENTRYPOINT ["/root/service"]

# ================================================================================
#  hasta-la-vista
# ================================================================================
FROM debian:bullseye-slim AS hasta-la-vista

COPY --from=build /build/service /root/service

WORKDIR /root

ENTRYPOINT ["/root/service"]

# ================================================================================
#  hello-world
# ================================================================================
FROM debian:bullseye-slim AS hello-world

RUN apt-get update && apt-get install -y ca-certificates curl

COPY --from=build /build/service /root/service

WORKDIR /root

ENTRYPOINT ["/root/service"]

RUN --mount=type=secret,id=MY_SECRET cat /run/secrets/MY_SECRET > file.txt

CMD ["file.txt"]
