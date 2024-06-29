use anchor_lang::prelude::*;

use super::Circuit;

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone)]
pub struct CircuitItem {
    id: u32,
    enabled: bool,
    circuit: Circuit,
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
