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
	glibc \
	curl && \
	# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
	# add user
	useradd -m -u 1000 -U -s /bin/sh -d /metaverse mvs
# add binary to docker image
COPY ./target/release/metaversevm /usr/local/bin/metaversevm
COPY ./testnet1.json ./testnet
COPY ./testnet1.json .
USER mvs
# check if executable works in this container
RUN /usr/local/bin/metaversevm --version
# 30333 p2p
# 9933 http rpc
# 9944 ws rpc
# 9615 prometheus
EXPOSE 30333 9933 9944 9615
VOLUME ["/metaversevm"]
ENTRYPOINT ["/usr/local/bin/metaversevm", "--unsafe-rpc-external", "--unsafe-ws-external"]