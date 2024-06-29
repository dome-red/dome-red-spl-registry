import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js'
import { DomeRedSplRegistry } from "../target/types/dome_red_spl_registry";
import * as assert from "assert";

describe("dome-red-spl-registry", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.DomeRedSplRegistry as Program<DomeRedSplRegistry>;

    /*
    it("Get instruction data", async() => {
        const [oracleAccountPDA, _] = await PublicKey.findProgramAddress(
            [
                anchor.utils.bytes.utf8.encode('oracle'),
                provider.wallet.publicKey.toBuffer(),
            ],
            program.programId
        )

        let instruction = await program.methods.registerOracle('oracle-1').accounts({
            oracle: provider.wallet.publicKey,
            oracleAccount: oracleAccountPDA,
        }).instruction();
        console.log(instruction);
    });
     */

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

        let register_circuit = async function(program, circuit, oracle, oracleAccount) {
            let register_circuit_tx = await program.methods.registerCircuit(circuit)
                .accounts({
                    oracle: oracle,
                    oracleAccount: oracleAccount,
                })
                .rpc();
            console.log("register_circuit tx", circuit.name, register_circuit_tx);
        }

        let circuit_1 = {
            name: 'Circuit #1',
            program: 'Circuit 1 - Code',
            signalNames: ['pubk', 'shdk']
        };
        let circuit_2 = {
            name: 'Circuit #2',
            program: 'Circuit 2 - Code',
            signalNames: ['pubk', 'shdk']
        };
        let circuit_3 = {
            name: 'Circuit #3',
            program: 'Circuit 3 - Code',
            signalNames: ['pubk', 'shdk']
        };
        let circuit_4 = {
            name: 'Circuit #4',
            program: 'Circuit 4 - Code',
            signalNames: ['pubk', 'shdk']
        };

        await register_circuit(program, circuit_1, provider.wallet.publicKey, oracleAccountPDA);
        await register_circuit(program, circuit_2, provider.wallet.publicKey, oracleAccountPDA);
        await register_circuit(program, circuit_3, provider.wallet.publicKey, oracleAccountPDA);
        await register_circuit(program, circuit_4, provider.wallet.publicKey, oracleAccountPDA);

        // Fetch the account details of the created tweet.
        let oracleAccount = await program.account.oracleAccount.fetch(oracleAccountPDA);

        // Ensure it has the right number of circuits.
        assert.equal(oracleAccount.circuitsPool.circuitItems.length, 4);
        console.log(oracleAccount.circuitsPool.circuitItems);

        // Increase oracle account size.
        const increase_account_size_tx = await program.methods.increaseAccountSize(4096)
            .accounts({
                oracle: provider.wallet.publicKey,
                oracleAccount: oracleAccountPDA,
            })
            .rpc();
        console.log("increase_account_size tx", increase_account_size_tx);

        // And try to register extra circuit.
        let circuit_5 = {
            name: 'Circuit #5',
            program: 'Circuit 5 - Code',
            signalNames: ['pubk', 'shdk']
        };
        await register_circuit(program, circuit_5, provider.wallet.publicKey, oracleAccountPDA);

        // Fetch the account details of the created tweet.
        oracleAccount = await program.account.oracleAccount.fetch(oracleAccountPDA);

        // Ensure it has the right number of circuits.
        assert.equal(oracleAccount.circuitsPool.circuitItems.length, 5);
        console.log(oracleAccount.circuitsPool.circuitItems);
    });
});
