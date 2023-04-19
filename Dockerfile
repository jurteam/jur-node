# This is the build stage for JUR. Here we create the binary.
FROM docker.io/library/ubuntu:latest as builder

RUN apt install -y git clang curl libssl-dev llvm libudev-dev protobuf-compiler

#Install Rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 
RUN source ~/.cargo/env
RUN rustup default stable
RUN rustup update nightly
RUN rustup update stable
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

# copy the source
WORKDIR /jur-node
COPY . /jur-node

# Build JUR Node
RUN SKIP_WASM_BUILD= cargo build --all-targets --features runtime-benchmarks --locked --release

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