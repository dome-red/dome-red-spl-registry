use anchor_lang::prelude::*;

use crate::errors::DomeError;

const MAX_NAME_LENGTH: usize = 128;
const MAX_CODE_LENGTH: usize = 2048;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub struct Circuit {
    pub(crate) id: u32,
    pub(crate) enabled: bool,
    #[max_len(MAX_NAME_LENGTH)]
    name: String,
    #[max_len(MAX_CODE_LENGTH)]
    code: String,
}

impl Circuit {
    pub fn new(circuit_id: u32, circuit_name: &str, circuit_code: &str) -> Result<Self> {
        if circuit_name.len() > MAX_NAME_LENGTH {
            return Err(DomeError::CircuitNameTooLong.into());
        }
        if circuit_code.len() > MAX_CODE_LENGTH {
            return Err(DomeError::CircuitCodeTooLong.into());
        }
        Ok(Self {
            id: circuit_id,
            enabled: true,
            name: circuit_name.to_owned(),
            code: circuit_code.to_owned(),
        })
    }

    pub fn set_enabled(&mut self, enabled: bool) -> Result<bool> {
        let prev_value = self.enabled;
        self.enabled = enabled;
        Ok(prev_value)
    }
}
