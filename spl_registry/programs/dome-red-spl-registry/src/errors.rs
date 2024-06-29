use anchor_lang::error_code;

#[error_code]
pub enum DomeError {
    #[msg("Circuit not found in Circuits Pool")]
    CircuitNotFound,
    #[msg("Maximum circuits number has been reached")]
    MaxCircuitsNumReached,
}
