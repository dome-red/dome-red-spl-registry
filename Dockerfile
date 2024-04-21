# Install Anchor
FROM solanalabs/rust:1.75.0 AS builder
USER root
WORKDIR /opt
RUN cargo install rustfilt
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.17.31/install)" && \
    /root/.local/share/solana/install/active_release/bin/sdk/sbf/scripts/install.sh \
ENV PATH="${PATH}:/root/.local/share/solana/install/active_release/bin"
RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
RUN avm install latest && avm use latest

# Build Solana Program
FROM builder as spl-builder
RUN solana --version
COPY spl_registry /opt/dome_red/spl_registry
WORKDIR /opt/dome_red/spl_registry
RUN solana-install init 1.18.0
RUN anchor build