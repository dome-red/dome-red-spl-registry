use anchor_lang::prelude::*;

use crate::errors::DomeError;

const MAX_NAME_LENGTH: usize = 32;
const MAX_PROGRAM_LENGTH: usize = 2048;
const MAX_SIGNALS_LENGTH: usize = 2048;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub struct Circuit {
    pub(crate) id: u32,
    pub(crate) enabled: bool,
    #[max_len(MAX_NAME_LENGTH)]
    name: String,
    #[max_len(MAX_PROGRAM_LENGTH)]
    program: String,
    // Comma-separated input signals names.
    #[max_len(MAX_SIGNALS_LENGTH)]
    signals: String,
}

impl Circuit {
    pub fn new(circuit_id: u32, circuit_name: &str, circuit_program: &str, circuit_signals: &str) -> Result<Self> {
        if circuit_name.len() > MAX_NAME_LENGTH {
            return Err(DomeError::CircuitNameTooLong.into());
        }
        if circuit_program.len() > MAX_PROGRAM_LENGTH {
            return Err(DomeError::CircuitProgramTooLong.into());
        }
        Ok(Self {
            id: circuit_id,
            enabled: true,
            name: circuit_name.to_owned(),
            program: circuit_program.to_owned(),
            signals: circuit_signals.to_owned()
        })
    }

    pub fn set_enabled(&mut self, enabled: bool) -> Result<bool> {
        let prev_value = self.enabled;
        self.enabled = enabled;
        Ok(prev_value)
    }
}
