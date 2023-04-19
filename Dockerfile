# This is the build stage for JUR. Here we create the binary.
FROM docker.io/paritytech/ci-linux:production as builder

# copy the source
WORKDIR /jur-node
COPY . /jur-node

# Build JUR Node
RUN cargo build --release

# This is the 2nd stage: a very small image where we copy the JUR binary."
FROM docker.io/library/ubuntu:latest
LABEL description="Multistage Docker image for JUR"

COPY --from=builder /jur-node/target/release/jur-node /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /jur-node jur-node && \
    mkdir -p /data /jur-node/.local/share && \
    ln -s /data /jur-node/.local/share/jur-node && \
    chown -R jur-node:jur-node /data /jur-node/.local/share && \
    # unclutter and minimize the attack surface
    #	rm -rf /usr/bin /usr/sbin && \
    # Sanity checks
    /usr/local/bin/jur-node --version

USER jur-node
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]