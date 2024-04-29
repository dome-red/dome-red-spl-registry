# Dome.Red SPL Registry

## Instructions set

**Important:** all instructions can be called only by oracles itself. **There is NO ADMIN control.**

| Instruction           | Description                                                                                                                                                                                                                                                                                                                                                               |
|-----------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| register_oracle       | Creates OracleAccount PDA (if not exists). Called by Oracle itself.                                                                                                                                                                                                                                                                                                       |
| enable_oracle         | Each Oracle can enable or disable itself. This instruction is sets Oracle as disabled. Called only by Oracle itself.                                                                                                                                                                                                                                                      |
| disable_oracle        | Disables the Oracle. Called only by Oracle itself. Oracle can enable itself any time.                                                                                                                                                                                                                                                                                     |
| change_oracle         | Also, Oracles can change their name or other meta data (in future) with this instruction.                                                                                                                                                                                                                                                                                 |
| register_circuit      | Oracles have their own pool of circuits. With this instruction, Oracle can add new circuit into its circuits pool.                                                                                                                                                                                                                                                        |
| remove_circuit        | With this instruction, Oracles can remove the circuit from their circuits pool.                                                                                                                                                                                                                                                                                           |
| enable_circuit        | Oracles can disable and enable the specified circuits in their circuits pool. This instruction enables the circuit.                                                                                                                                                                                                                                                       |
| disable_circuit       | And this instruction disables the specified circuit (see above).                                                                                                                                                                                                                                                                                                          | 
| register_proof        | With this instruction, Proving Oracle creates ProofAccount PDA (if not exists) for user proof.<br><br>Proof PDA address is calculated on Pubkey of Proving Oracle, hash of the Pubkey of Target Oracle (for which this proof is created) and hash of the User Pubkey (for whom this proof is created).<br><br>We use hashes to hide association between User and Oracles. |
| increase_account_size | PDA accounts size is limited to 10K. So, we create PDA with Circuits Pool, that can contain 4 circuits. If Oracle needs to store more circuits, it can increase the space with this instruction.                                                                                                                                                                          |

## Testing

1. Build spl code: `anchor build`
2. Install dependencies for tests: `yarn install`
3. Start Solana test validator: `solana-test-validator -r`
4. Deploy program library into solana: `anchor deploy`
5. Run tests: `anchor test`

Program ID can be obtained by command: `solana address -k ./target/deploy/dome_red_spl_registry-keypair.json`

## Using Docker for Testing

1. Build docker image: `docker build -t dome-red/spl-registry:latest .`
2. Run tests inside docker-compose: `docker-compose -f ./docker/docker-compose.yaml up`

## Program Scheme

![](https://www.plantuml.com/plantuml/png/ZLR1Rjim3BthAuIUscQ-R9V1C8mMtM4xBLYtNGL5PbnXjXH8SjTqsBzFiUN2J1WjfqHyZ-GJAKg-yq6wCFGTspaRhB8TRsmFma4Z_AwBlwtsmUsh2BTEzZKZOLxo5PyMu8HrKdLmVPFXWf6R3YvXDjfVIbLFqhGP9Y5LQQS67Vv3xkqpPAbeMpagsbUEIykDNR3RB5KRvK1w458fEvWWl7w5wC5Oi2s49t3y3uliGGN-BONzEXQikfyyVCqmCCQTYbhoHzqQSESPSzwIhp_TLnES0fDn_JuQUf6EL_IQ_bIFexrcizR9vS6Qu6ota8hib59Ck53eSONMXxeliY8xaPKWZAt3IKSm1QLYNbgRBwddATIZVbqsaq0fnNbQjbgawmNw_-BkplvLJ54pOmFmZGt1zdpCMlAhISiLNoK7lXusls3Fa7ja5saGcp96i1s16zaVjXwSDYq1fw7qOXVTI_wGbcigs8flXdvNyk4Z0McZWvRTD8Edi8SWajIpU9HwtlcGcWBn3YA367-qg-K2bQ3aFq5L2QwFUIjku2M8H1OwRkEKqiA2OHfueKZRYCmi2fTD3aQNDCt0jJugGgAdfZKb7pwyHpPqe1O8Fae8IEV-64jL-_scaJWAYz6OBudyP8otoybWnFNdw-_IjH0mS3CYFpousS2MUPByQ9Z5loFmOTDfbI4yWzDRhMJGreYOWwRY2PeHUe0mYaVWhPX5SPivseMBC1gNurVfjYWm0JyX18vlxeYZfh4phtLHr1odpX2H19fNblWpOP4WIJldHtup7DofbuMnEp3Dq7V_0000)

<!--
//www.plantuml.com/plantuml/png/ZLR1Rjim3BthAuIUscQ-R9V1C8mMtM4xBLYtNGL5PbnXjXH8SjTqsBzFiUN2J1WjfqHyZ-GJAKg-yq6wCFGTspaRhB8TRsmFma4Z_AwBlwtsmUsh2BTEzZKZOLxo5PyMu8HrKdLmVPFXWf6R3YvXDjfVIbLFqhGP9Y5LQQS67Vv3xkqpPAbeMpagsbUEIykDNR3RB5KRvK1w458fEvWWl7w5wC5Oi2s49t3y3uliGGN-BONzEXQikfyyVCqmCCQTYbhoHzqQSESPSzwIhp_TLnES0fDn_JuQUf6EL_IQ_bIFexrcizR9vS6Qu6ota8hib59Ck53eSONMXxeliY8xaPKWZAt3IKSm1QLYNbgRBwddATIZVbqsaq0fnNbQjbgawmNw_-BkplvLJ54pOmFmZGt1zdpCMlAhISiLNoK7lXusls3Fa7ja5saGcp96i1s16zaVjXwSDYq1fw7qOXVTI_wGbcigs8flXdvNyk4Z0McZWvRTD8Edi8SWajIpU9HwtlcGcWBn3YA367-qg-K2bQ3aFq5L2QwFUIjku2M8H1OwRkEKqiA2OHfueKZRYCmi2fTD3aQNDCt0jJugGgAdfZKb7pwyHpPqe1O8Fae8IEV-64jL-_scaJWAYz6OBudyP8otoybWnFNdw-_IjH0mS3CYFpousS2MUPByQ9Z5loFmOTDfbI4yWzDRhMJGreYOWwRY2PeHUe0mYaVWhPX5SPivseMBC1gNurVfjYWm0JyX18vlxeYZfh4phtLHr1odpX2H19fNblWpOP4WIJldHtup7DofbuMnEp3Dq7V_0000

-->
