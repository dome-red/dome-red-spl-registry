#!/bin/bash
solana-test-validator -r > /dev/null &
./docker/wait-for-solana.sh ${SOLANA_WAIT_TIMEOUT:-60}

tail +1f test-ledger/validator.log
