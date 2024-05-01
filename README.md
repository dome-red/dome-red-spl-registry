# Dome.Red SPL Registry

## Instructions set

**Important:** all instructions can be called only by oracles itself. **There is NO ADMIN control.**

| Instruction           | Description                                                                                                                                                                                                                                                                                                                                                               |
|-----------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| register_oracle       | Creates OracleAccount PDA (if not exists). Called by Oracle itself.                                                                                                                                                                                                                                                                                                       |
| enable_oracle         | Each Oracle can enable or disable itself. This instruction is sets Oracle as enabled. Called only by Oracle itself.                                                                                                                                                                                                                                                       |
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

![](https://www.plantuml.com/plantuml/png/ZPRFRjmW4CRlVWgKK_xKb_PIMPQLA3sqfwwQjbV4uec3QiCAS9fDrNSlYt7hMS_IFINm_ORv61YyjyuBwyUXBtRMU2DDprep0BVGShVhmzzEEM_tTP0xAuQc86LMiHis3S1oOuNiuV8gmu8MsnxE8LlbpaNbaz1TXYIiIcNbgFn_uC4yGnx5sygZQ5yPbh0RJi4yPb6bfGNXW0ifpQWzT-eLGaJXJGV-2IpxLGHub9vzIQj-ZlMgxwO0rnHOYCjEDQtOW-eqsDF4dBPYcmztzIIdXSbrtJvC33pThX0r_LC_nFcccBrE8VT6UslwXLKqJpd5m9b6ZokqEPHzaHND4ra989uSJXf5N1mKScvjFgKZJqizgDVbOH8gvJYFPQj5XfxX_pS-tJc-3YcQGXiFR6kyDmEBMIjsCNcvO3SfW6t6xG_O5oWyq6NgmxOAAvYUqBKO3bltLkcEa5DFEhuBuHMxJyCDjTWDsux3hcBZUq9IMdab-jY2XEp0ysJrf7wmUb8yfAP4l0Eo_V6kbWDKWefz1radkK5SJAJXnVC4SnMsiKOwM12wXHSAUWpAJ56wQBCoVgCf0jVwg0W9JuVMLcnyznRDuGkLbDLLGaAwzsijLktdFuzhUulMc1-9V6V4rsNL667yyVAhi1ruB5n5vPi3EqyKopp9VfYOpRyXz77RAva1di6gHoM5LqRpa8D6KGTXPTL0MCKjyBUOPNcTkTgb3J9gb-ElqdLPOW1_GWWTlTnHNnejw8zryDamd3e38WiqLrNuLqACGzPEnP4_6M8uyR8Njw3RSUZ_0000)

<!--
https://www.plantuml.com/plantuml/uml/ZPRFRjmW4CRlVWgKK_xKb_PIMPQLA3sqfwwQjbV4uec3QiCAS9fDrNSlYt7hMS_IFINm_ORv61YyjyuBwyUXBtRMU2DDprep0BVGShVhmzzEEM_tTP0xAuQc86LMiHis3S1oOuNiuV8gmu8MsnxE8LlbpaNbaz1TXYIiIcNbgFn_uC4yGnx5sygZQ5yPbh0RJi4yPb6bfGNXW0ifpQWzT-eLGaJXJGV-2IpxLGHub9vzIQj-ZlMgxwO0rnHOYCjEDQtOW-eqsDF4dBPYcmztzIIdXSbrtJvC33pThX0r_LC_nFcccBrE8VT6UslwXLKqJpd5m9b6ZokqEPHzaHND4ra989uSJXf5N1mKScvjFgKZJqizgDVbOH8gvJYFPQj5XfxX_pS-tJc-3YcQGXiFR6kyDmEBMIjsCNcvO3SfW6t6xG_O5oWyq6NgmxOAAvYUqBKO3bltLkcEa5DFEhuBuHMxJyCDjTWDsux3hcBZUq9IMdab-jY2XEp0ysJrf7wmUb8yfAP4l0Eo_V6kbWDKWefz1radkK5SJAJXnVC4SnMsiKOwM12wXHSAUWpAJ56wQBCoVgCf0jVwg0W9JuVMLcnyznRDuGkLbDLLGaAwzsijLktdFuzhUulMc1-9V6V4rsNL667yyVAhi1ruB5n5vPi3EqyKopp9VfYOpRyXz77RAva1di6gHoM5LqRpa8D6KGTXPTL0MCKjyBUOPNcTkTgb3J9gb-ElqdLPOW1_GWWTlTnHNnejw8zryDamd3e38WiqLrNuLqACGzPEnP4_6M8uyR8Njw3RSUZ_0000

-->
