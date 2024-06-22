#!/bin/bash
# Airdrop sols for default deploy account (id.json) and oracles.
solana airdrop 100000 -k /root/.config/solana/id.json
solana airdrop 100000 -k /opt/assets/oracle_1-keypair.json
solana airdrop 100000 -k /opt/assets/oracle_2-keypair.json
solana airdrop 100000 -k /opt/assets/oracle_3-keypair.json
solana airdrop 100000 -k /opt/assets/oracle_4-keypair.json
solana airdrop 100000 -k /opt/assets/oracle_5-keypair.json

# Deploy registry.
cd ./spl_registry
anchor deploy
# anchor test
cd ../
