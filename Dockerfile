FROM ubuntu:22.04 AS builder
USER root
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y \
        build-essential \
        git clang curl \
        pkg-config \
        libudev-dev llvm libclang-dev \
        protobuf-compiler libssl-dev \
        make cmake

# Install latest Rust
FROM builder AS rust-builder
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install rustfilt

# Install Solana
FROM rust-builder as solana-builder
RUN sh -c "$(curl -sSfL https://release.solana.com/stable/install)" && \
    /root/.local/share/solana/install/active_release/bin/sdk/sbf/scripts/install.sh
ENV PATH=${PATH}:/root/.local/share/solana/install/active_release/bin

# Install Anchor
FROM solana-builder as anchor-builder
RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
RUN avm install latest && avm use latest

# Build Solana Program
FROM anchor-builder as spl-builder
COPY spl_registry /opt/dome_red/spl_registry
WORKDIR /opt/dome_red/spl_registry
RUN anchor build