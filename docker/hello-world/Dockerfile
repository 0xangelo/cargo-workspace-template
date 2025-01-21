FROM rust:1.83-bullseye AS build

# ================================================================================
#  Build crate
# ================================================================================
RUN mkdir -p /build
WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY .cargo .cargo
COPY crates crates

RUN cargo build --release -p hello-world

# ================================================================================
#  Runtime configuration
# ================================================================================
FROM debian:bullseye-slim AS runtime

# Use jemalloc as memory allocator
RUN apt-get update && apt-get install -y libjemalloc-dev ca-certificates curl
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so

COPY --from=build /build/target/release/hello-world /root/service

WORKDIR /root

ENTRYPOINT ["/root/service"]

RUN --mount=type=secret,id=MY_SECRET cat /run/secrets/MY_SECRET > file.txt

CMD ["file.txt"]
