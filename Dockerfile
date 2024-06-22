# Install Anchor
FROM solanalabs/rust:1.75.0 as anchor-builder
WORKDIR /opt
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.18.12/install)" && \
    /root/.local/share/solana/install/active_release/bin/sdk/sbf/scripts/install.sh
ENV PATH=/root/.local/share/solana/install/active_release/bin:/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
RUN cargo install --git https://github.com/coral-xyz/anchor --tag v0.30.1 avm --locked --force && \
    avm install 0.30.1 && avm use 0.30.1
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add - && \
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
RUN apt update && apt -y install yarn python3 net-tools
RUN apt clean && rm -rf /var/lib/apt/lists/* /var/lib/apt/cache/*

FROM anchor-builder as spl-builder
RUN solana-keygen new -f -s --no-bip39-passphrase
RUN solana config set --url http://localhost:8899
RUN mkdir -p /opt/spl_registry && mkdir -p /opt/docker
COPY ./spl_registry /opt/spl_registry
COPY ./assets /opt/assets
COPY ./docker/*.sh /opt/docker
RUN chmod +x /opt/docker/*.sh
RUN cd /opt/spl_registry && yarn install --cwd /opt/spl_registry && anchor build && rm -rf target/release/deps
RUN cp /opt/assets/dome_red_spl_registry-keypair.json /opt/spl_registry/target/deploy/dome_red_spl_registry-keypair.json
WORKDIR /opt
ENTRYPOINT ["docker/solana-run-registry.sh"]
