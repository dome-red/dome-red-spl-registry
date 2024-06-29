use anchor_lang::prelude::*;
use crate::circuits::CircuitsPool;

// Maximum PDA size in one iteration is 10240 bytes.ы
const REF_NAME_LEN: usize = 32;

#[account]
#[derive(InitSpace)]
pub struct OracleAccount {
    enabled: bool,
    #[max_len(REF_NAME_LEN)]
    name: String,
    circuits_pool: CircuitsPool,
    pub(crate) bump: u8,
}

impl OracleAccount {
    pub fn setup(&mut self, name: &str) -> &mut Self {
        self.enabled = true;
        self.name = name.to_string();
        self 
    }

    pub fn set_enabled(&mut self, enabled: bool) -> (&mut Self, bool) {
        let prev_value = self.enabled;
        self.enabled = enabled;
        (self, prev_value)
    }

    pub fn set_name(&mut self, oracle_name: &str) -> (&mut Self, String) {
        let prev_value = self.name.clone();
        self.name = oracle_name.to_owned();
        (self, prev_value)
    }

    pub fn set_bump(&mut self, bump: u8) -> (&mut Self, u8) {
        let prev_value = self.bump;
        self.bump = bump;
        (self, prev_value)
    }

    pub fn circuits_pool(&mut self) -> &mut CircuitsPool {
        &mut self.circuits_pool
    }
}
