### build stage
FROM rust:1.51-slim as builder
ENV USER root
ENV CI_PROJECT_NAME docker
RUN apt-get update && apt-get install -y git cmake pkg-config libssl-dev git clang libclang-dev
RUN rustup default nightly && rustup target add wasm32-unknown-unknown
COPY . new-frontiers
WORKDIR "new-frontiers"
RUN cargo build --release

### package stage
FROM debian:bullseye-slim
# metadata
ARG VCS_REF
ARG BUILD_DATE
# show backtraces
ENV RUST_BACKTRACE 1
# install tools and dependencies
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
        libssl1.1 \
        ca-certificates \
        curl && \
# apt cleanup
    apt-get autoremove -y && \
    apt-get clean && \
    find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user
    useradd -m -u 1000 -U -s /bin/sh -d /new-frontiers mvs
# add binary to docker image
COPY --from=builder /new-frontiers/target/release/metaversevm /usr/local/bin/

RUN apt-get update && apt-get install -y libc6

# check if executable works in this container
USER mvs
RUN /usr/local/bin/metaversevm --version
# 30333 p2p
# 9933 http rpc
# 9944 ws rpc
# 9615 prometheus
EXPOSE 30333 9933 9944 9615
VOLUME ["/metaverse"]
CMD metaversevm
