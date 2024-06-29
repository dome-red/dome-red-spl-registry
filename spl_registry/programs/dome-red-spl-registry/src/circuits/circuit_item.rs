use anchor_lang::prelude::*;
use serde::{Serialize, Deserialize};
use super::Circuit;

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone, Serialize, Deserialize)]
pub struct CircuitItem {
    pub id: u32,
    pub enabled: bool,
    pub circuit: Circuit,
}

impl CircuitItem {
    pub fn new(circuit: &Circuit) -> Self {
        Self {
            id: 0,
            enabled: true,
            circuit: circuit.clone(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_id(&mut self, id: u32) -> (&mut Self, u32) {
        self.id = id;
        (self, id)
    }

    pub fn set_enabled(&mut self, enabled: bool) -> (&mut Self, bool) {
        let prev_value = self.enabled;
        self.enabled = enabled;
        (self, prev_value)
    }
}
