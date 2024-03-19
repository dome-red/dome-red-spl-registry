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

![](https://www.plantuml.com/plantuml/png/XPNFRjGm4CRlVWghf_uX5xYWA8fQbGCtLXIuHbvdcbeazignInV4kpFhE2MpTjBRPhxVpdmpzYItenFM-Q5d1skSaQRdhHcWiT2fqTbZXU7EYg5c9CnBViEd1x2DiKBsS7cLOK2BVGzRHAl6jn3v97ItIX0hKbdfbLk11lCCUOJQJH7YTvCXTd1wvX4Hnfpfm3s1vNyOObuw_ZL2Nq9RrPsKncitremA8U7KUiaVLAV1hXEDcBAKVFVfhfhac3YRTpnYO6ZY8UE_fX_LGuZNRFOw_UNUQ6TDlx1AuXcdgy2cqVC9xKvJN5GbyLpL50W7GOi6CIPDSxuzcx-lFedz2dFNJ1i7V6-SCmCFvKf-CPcymDJn17P-_nEEZFmTwI9UmxR43ARFw5eCfsQTLRhBoFOW6s_x3I9Uth4v68FSVNpSvShTyBqV3YNt7pEIqiefqQlVk9SPUGJNn6PMzLCpgs9iPLK_MSk9j4CaqWMaNIyVo8XA_WDa5UMQSA6GXXVNHBXHs6QATB0WT0ilEUWHbPdAwQBTbEdBBKVGipWRGiJZeRObzn_UanYvab59BoCHukQaMfZQh_yUKtkBvPhNT7ktmdudguNFdo-_2Tk1ey9LKBwFOES0MzQ9zZ4mct-NqVs-Lt83U0Qh7fKKJXdTO8qiIZUCAic2KPMkmEimY-9wuseN5cMoBkVVXUkYe01zYQSwVNS6VMcKqGya-givBPa1I4cgboNzl0QUM63i5dJhX_uV)

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
ProofAccount : - public: String,
ProofAccount : - verification_key: String,
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
