import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js'
import { DomeRedSplRegistry } from "../target/types/dome_red_spl_registry";
import * as assert from "assert";

describe("dome-red-spl-registry", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.DomeRedSplRegistry as Program<DomeRedSplRegistry>;

    it("Sets and changes Oracle name", async () => {
        const [oracleAccountPDA, _] = await PublicKey.findProgramAddress(
            [
                anchor.utils.bytes.utf8.encode('oracle'),
                provider.wallet.publicKey.toBuffer(),
            ],
            program.programId
        )

        const register_tx = await program.methods.registerOracle('Alice')
            .accounts({
                oracle: provider.wallet.publicKey,
                oracleAccount: oracleAccountPDA,
            })
            .rpc();
        console.log("register_oracle tx", register_tx);

        // Fetch the account details of the created tweet.
        const oracleAccount = await program.account.oracleAccount.fetch(oracleAccountPDA);

        // Ensure it has the right data.
        assert.equal(oracleAccount.name, 'Alice');
        console.log(oracleAccount);

        // Change oracle data.
        const change_oracle_tx = await program.methods.changeOracle('Bob')
            .accounts({
                oracle: provider.wallet.publicKey,
                oracleAccount: oracleAccountPDA,
            })
            .rpc();
        console.log("change_oracle tx", change_oracle_tx);

        // Fetch the account details of the created tweet.
        const changedOracleAccount = await program.account.oracleAccount.fetch(oracleAccountPDA);

        // Ensure it has the right data.
        assert.equal(changedOracleAccount.name, 'Bob');
        console.log(changedOracleAccount);

    });

    it("Add circuits to oracle", async () => {
        const [oracleAccountPDA, _] = await PublicKey.findProgramAddress(
            [
                anchor.utils.bytes.utf8.encode('oracle'),
                provider.wallet.publicKey.toBuffer(),
            ],
            program.programId
        )

        const register_tx = await program.methods.registerOracle('Alice')
            .accounts({
                oracle: provider.wallet.publicKey,
                oracleAccount: oracleAccountPDA,
            })
            .rpc();
        console.log("register_oracle tx", register_tx);

        let register_circuit = async function(program, name, code, oracle, oracleAccount) {
            let register_circuit_tx = await program.methods.registerCircuit(name, code)
                .accounts({
                    oracle: oracle,
                    oracleAccount: oracleAccount,
                })
                .rpc();
            console.log("register_circuit tx", name, register_circuit_tx);
        }

        await register_circuit(program, 'Circuit #1', 'Circuit 1 - code', provider.wallet.publicKey, oracleAccountPDA);
        await register_circuit(program, 'Circuit #2', 'Circuit 2 - code', provider.wallet.publicKey, oracleAccountPDA);
        await register_circuit(program, 'Circuit #3', 'Circuit 3 - code', provider.wallet.publicKey, oracleAccountPDA);
        await register_circuit(program, 'Circuit #4', 'Circuit 4 - code', provider.wallet.publicKey, oracleAccountPDA);

        // Fetch the account details of the created tweet.
        let oracleAccount = await program.account.oracleAccount.fetch(oracleAccountPDA);

        // Ensure it has the right number of circuits.
        assert.equal(oracleAccount.circuitsPool.circuits.length, 4);
        console.log(oracleAccount.circuitsPool.circuits);

        // Increase oracle account size.
        const increase_account_size_tx = await program.methods.increaseAccountSize(4096)
            .accounts({
                oracle: provider.wallet.publicKey,
                oracleAccount: oracleAccountPDA,
            })
            .rpc();
        console.log("increase_account_size tx", increase_account_size_tx);

        // And try to register extra circuit.
        await register_circuit(program, 'Circuit #5', 'Circuit 5 - code', provider.wallet.publicKey, oracleAccountPDA);

        // Fetch the account details of the created tweet.
        oracleAccount = await program.account.oracleAccount.fetch(oracleAccountPDA);

        // Ensure it has the right number of circuits.
        assert.equal(oracleAccount.circuitsPool.circuits.length, 5);
        console.log(oracleAccount.circuitsPool.circuits);
    });
});
