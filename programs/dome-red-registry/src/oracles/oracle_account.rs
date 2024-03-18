use anchor_lang::prelude::*;

use crate::circuits::CircuitsPool;
use crate::errors::DomeError;

const MAX_NAME_LENGTH: usize = 128;
const MAX_RPC_URL_LENGTH: usize = 128;

#[account]
#[derive(InitSpace)]
pub struct OracleAccount {
    enabled: bool,
    #[max_len(MAX_NAME_LENGTH)]
    name: String,
    #[max_len(MAX_RPC_URL_LENGTH)]
    rpc_url: String,
    circuits_pool: CircuitsPool,
    pub bump: u8,
}

impl OracleAccount {
    pub fn initialize(&mut self, oracle_name: &str, oracle_rpc_url: &str) -> Result<()> {
        if oracle_name.len() > MAX_NAME_LENGTH {
            return Err(DomeError::OracleNameTooLong.into());
        }
        if oracle_rpc_url.len() > MAX_RPC_URL_LENGTH {
            return Err(DomeError::OracleRpcUrlTooLong.into());
        }
        self.enabled = true;
        self.name = oracle_name.to_owned();
        self.rpc_url = oracle_rpc_url.to_owned();
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

    pub fn set_rpc_url(&mut self, oracle_rpc_url: &str) -> Result<String> {
        if oracle_rpc_url.len() > MAX_RPC_URL_LENGTH {
            return Err(DomeError::OracleRpcUrlTooLong.into());
        }
        let prev_value = self.rpc_url.clone();
        self.rpc_url = oracle_rpc_url.to_owned();
        Ok(prev_value)
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump = bump;
    }

    pub fn circuits_pool(&mut self) -> &mut CircuitsPool {
        &mut self.circuits_pool
    }
}
