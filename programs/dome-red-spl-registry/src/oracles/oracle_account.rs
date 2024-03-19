use anchor_lang::prelude::*;

use crate::circuits::CircuitsPool;
use crate::errors::DomeError;

// Maximum PDA size in one iteration is 10240 bytes.ы
const MAX_NAME_LENGTH: usize = 32;

#[account]
#[derive(InitSpace)]
pub struct OracleAccount {
    enabled: bool,
    #[max_len(MAX_NAME_LENGTH)]
    name: String,
    circuits_pool: CircuitsPool,
    pub bump: u8,
}

impl OracleAccount {
    pub fn initialize(&mut self, oracle_name: &str) -> Result<()> {
        if oracle_name.len() > MAX_NAME_LENGTH {
            return Err(DomeError::OracleNameTooLong.into());
        }
        self.enabled = true;
        self.name = oracle_name.to_owned();
        self.circuits_pool = CircuitsPool::default();
        Ok(())
    }

    pub fn set_enabled(&mut self, enabled: bool) -> Result<bool> {
        let prev_value = self.enabled;
        self.enabled = enabled;
        Ok(prev_value)
    }

    pub fn set_name(&mut self, oracle_name: &str) -> Result<String> {
        if oracle_name.len() > MAX_NAME_LENGTH {
            return Err(DomeError::OracleNameTooLong.into());
        }
        let prev_value = self.name.clone();
        self.name = oracle_name.to_owned();
        Ok(prev_value)
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump = bump;
    }

    pub fn circuits_pool(&mut self) -> &mut CircuitsPool {
        &mut self.circuits_pool
    }
}
