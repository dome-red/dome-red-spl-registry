use anchor_lang::error_code;

#[error_code]
pub enum DomeError {
    NotPermitted,

    CircuitNotFound,
    CircuitNameTooLong,
    CircuitProgramTooLong,
    MaxCircuitsNumReached,

    OracleNotFound,
    OracleAlreadyExists,
    OracleNameTooLong,
    OracleRpcUrlTooLong,
    MaxOraclesNumReached,

    ProofTooLong,
    InvalidProofVerificationKey,
}
