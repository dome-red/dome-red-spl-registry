# Dome Red SPL Registry

## Instructions set

**Important:** all instructions can be called only by oracles itself. **There is NO ADMIN control.**

| Instruction | Description                                                                                                                                                                                                                                                                                                                                                               |
| ---- |---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| register_oracle | Creates OracleAccount PDA (if not exists). Called by Oracle itself.                                                                                                                                                                                                                                                                                                       |
| enable_oracle | Each Oracle can enable or disable itself. This instruction is sets Oracle as disabled. Called only by Oracle itself.                                                                                                                                                                                                                                                      |
| disable_oracle | Disables the Oracle. Called only by Oracle itself. Oracle can enable itself any time.                                                                                                                                                                                                                                                                                     |
| change_oracle | Also, Oracles can change their name or RPC URL with this instruction.                                                                                                                                                                                                                                                                                                     |
| register_circuit | Oracles have their own pool of circuits. With this instruction, Oracle can add new circuit into its circuits pool.                                                                                                                                                                                                                                                        |
| remove_circuit | With this instruction, Oracles can remove the circuit from their circuits pool.                                                                                                                                                                                                                                                                                           |
| enable_circuit  | Oracles can disable and enable the specified circuits in their circuits pool. This instruction enables the circuit.                                                                                                                                                                                                                                                       |
| disable_circuit | And this instruction disables the specified circuit (see above).                                                                                                                                                                                                                                                                                                          | 
| register_proof | With this instruction, Proving Oracle creates ProofAccount PDA (if not exists) for user proof.<br><br>Proof PDA address is calculated on Pubkey of Proving Oracle, hash of the Pubkey of Target Oracle (for which this proof is created) and hash of the User Pubkey (for whom this proof is created).<br><br>We use hashes to hide association between User and Oracles. | 

## Program Scheme

![](https://www.plantuml.com/plantuml/png/XPNDRjGm4CVlVWghfxPBBd11LXIrAWTkh2XmZRpEDBL8x9LZGnV4k-DDPY4JE-bjC_-VvsCyuxqRl7G-z1qxEkkjiXrlR0-rWrOFtftAQ6wTx2j6p5pm7Py-mDNMITN1zKs60ICF7MmHZHvUGzIpD6r2P5DHsgcW_GhKsn-mYM2wAKBpfSn6Eh5xzYaYZ7dRWdy6nt-pY0Nb-MU4FerbbVTAsM3yK34gZ0wdqWL_rAq1jqxKSl8Y-Fx3VJd9w3ZhTpX5Gr_Z9STJqu_oSRHNx9BhTEJ16kziDqkLs3EPhWARYIuxj3ztSHQLs7DHKs2y21fq5D5fwlFrtlnPT0TTbT5tnOprm0_MUzlpCPpWLrEILto71_W-7Bx3YP7ZaIvm31iHFTWkenlPdulrJfis8xkZgeFhDWWStg4-MXkv1_pSvyBj-27qHy73-uoaZVPQTlfNtCkCF82liPXLFHQpVXXBMTNFgUL4Mc4aqWMaLSy_I8i4_mQgHBbQSZjkuCNN2DSwbfaYBSm8qy1B3dgAoeNAwRBPal5nsoBeNIoQW3XUQYDuUFUMsEX8ef8C8n5mSvOQTktd_npJz6RBTLdJxTqQzpjPhVZzyVgBT2ruAjoCojS1tCN0vd4m_MYu9F-6w6GhY98k1LNfsFvhG57SRenZuQ9CHdpvbjyM1GNemvlHwNiNTT9ejlXJAugAqv0PW8IakX3qBv4u70CpTWUc2Ntt5m00)

<!--

@startuml
protocol dome_registry<program>
dome_registry : + register_oracle()
dome_registry : + enable_oracle()
dome_registry : + disable_oracle()
dome_registry : + change_oracle()
dome_registry : + register_circuit()
dome_registry : + remove_circuit()
dome_registry : + enable_circuit()
dome_registry : + disable_circuit()
dome_registry : + register_proof()


together {
struct RegisterOracle<Accounts>
RegisterOracle : + oracle: Signer
RegisterOracle : + oracle_account: PDA<OracleAccount>
RegisterOracle : + system_program: Program<System>

struct OracleControl<Accounts>
OracleControl : + oracle: Signer
OracleControl : + oracle_account: PDA<OracleAccount>

struct RegisterProof<Accounts>
RegisterProof : + oracle: Signer
RegisterProof : + proof_account: PDA<ProofAccount>
RegisterProof : + system_program: Program<System>
}

struct OracleAccount<PDA>
note bottom : PDA: "oracle" + Oracle Pubkey
OracleAccount : - enabled: bool
OracleAccount : - name: String
OracleAccount : - rpc_url: String
OracleAccount : - circuits_pool: CircuitsPool
OracleAccount : + bump: u8
OracleAccount : + initialize()
OracleAccount : + set_enabled()
OracleAccount : + set_name()
OracleAccount : + set_rpc_url()
OracleAccount : + set_bump()
OracleAccount : + circuits_pool()

together {
struct CircuitsPool
CircuitsPool : - circuits: Vec<Circuit>
CircuitsPool : + next_circuit_id()
CircuitsPool : + get_circuit_index()
CircuitsPool : + find_circuit()
CircuitsPool : + add_circuit()
CircuitsPool : + remove_circuit()
CircuitsPool : + set_enabled()

struct Circuit
Circuit : + id: u32
Circuit : + enabled: bool
Circuit : - name: String
Circuit : - program: String
Circuit : + new()
Circuit : + set_enabled()
}

struct ProofAccount<PDA>
note bottom : PDA: "proof" + Oracle Pubkey + H(Target Oracle Pubkey) + H(User Pubkey)
ProofAccount : - proof: String,
ProofAccount : + bump: u8
ProofAccount : + initialize()
ProofAccount : + set_bump()

Circuit --* CircuitsPool
CircuitsPool -r-* OracleAccount
RegisterOracle *-- OracleAccount
OracleControl *-- OracleAccount
RegisterProof *-- ProofAccount

dome_registry --\> RegisterOracle
dome_registry --\> OracleControl
dome_registry::register_proof --\> RegisterProof

@enduml

-->
