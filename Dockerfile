### package stage
FROM debian as target
# metadata
ARG VCS_REF
ARG BUILD_DATE
# show backtraces
ENV RUST_BACKTRACE 1
# install tools and dependencies
SHELL ["/bin/bash", "-c"]
RUN apt-get update 
RUN apt-get upgrade -y 
RUN apt-get install -y ca-certificates  curl git clang curl libssl-dev llvm libudev-dev
RUN apt-get install build-essential -y
# apt cleanup
RUN apt-get autoremove -y
RUN apt-get clean 
RUN find /var/lib/apt/lists/ -type f -not -name lock -delete;
RUN useradd -m -u 1000 -U -s /bin/sh -d /metaverse mvs
RUN curl https://getsubstrate.io -sSf | bash -s -- --fast
RUN source $HOME/.cargo/env
RUN source ~/.profile
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup toolchain install nightly-2021-05-18-x86_64-unknown-linux-gnu
RUN rustup +nightly-2021-05-18 target add wasm32-unknown-unknown
RUN git clone https://github.com/mvs-org/new-frontiers.git
WORKDIR "new-frontiers"
RUN cargo +nightly-2021-05-18 build --release 
RUN cp target/release/metaversevm /usr/local/bin
RUN ls /usr/local/bin
USER mvs
COPY ./testnet1.json ./testnet
COPY ./testnet1.json .
EXPOSE 30333 9933 9944 9615
VOLUME ["/metaversevm"]
ENTRYPOINT ["/usr/local/bin/metaversevm", "--unsafe-rpc-external", "--unsafe-ws-external"]