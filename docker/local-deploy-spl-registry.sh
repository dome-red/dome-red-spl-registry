#!/bin/bash
# Airdrop sols for default deploy account (id.json) and oracles.
solana airdrop 100000 -k ~/.config/solana/id.json
solana airdrop 100000 -k ./assets/oracle_1-keypair.json
solana airdrop 100000 -k ./assets/oracle_2-keypair.json
solana airdrop 100000 -k ./assets/oracle_3-keypair.json
solana airdrop 100000 -k ./assets/oracle_4-keypair.json
solana airdrop 100000 -k ./assets/oracle_5-keypair.json

# Deploy registry.
cd ./spl_registry
anchor deploy
# anchor test
cd ../
