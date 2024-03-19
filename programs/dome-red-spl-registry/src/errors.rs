use anchor_lang::error_code;

#[error_code]
pub enum DomeError {
    #[msg("Circuit not found in Circuits Pool")]
    CircuitNotFound,
    #[msg("Circuit name is too long")]
    CircuitNameTooLong,
    #[msg("Circuit program is too big")]
    CircuitProgramTooLong,
    #[msg("Maximum circuits number has been reached")]
    MaxCircuitsNumReached,

    #[msg("Oracle name is too long")]
    OracleNameTooLong,

    #[msg("Proof data is too long")]
    ProofTooLong,
    #[msg("Proof public data is too long")]
    ProofPublicTooLong,
    #[msg("Proof verification key is too big")]
    ProofVerificationKeytooLong,
}
