use anchor_lang::prelude::*;

use crate::errors::DomeError;

use super::Circuit;

const MAX_CIRCUITS_NUM: usize = 4; //32;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace, Default)]
pub struct CircuitsPool {
    #[max_len(MAX_CIRCUITS_NUM, Circuit::INIT_SPACE)]
    circuits: Vec<Circuit>,
    last_used_id: u32,
}

impl CircuitsPool {
    pub fn next_circuit_id(&mut self) -> Result<u32> {
        match self.last_used_id.checked_add(1) {
            None => Err(DomeError::MaxCircuitsNumReached.into()),
            Some(last_user_id) => {
                self.last_used_id = last_user_id;
                Ok(self.last_used_id)
            }
        }
    }

    pub fn get_circuit_index(&self, circuit_id: u32) -> Option<usize> {
        self.circuits.iter()
            .position(|x| x.id == circuit_id)
    }

    pub fn find_circuit(&mut self, circuit_id: u32) -> Option<&mut Circuit> {
        self.get_circuit_index(circuit_id)
            .and_then(|index| self.circuits.get_mut(index))
    }

    pub fn add_circuit(&mut self, circuit_name: &str, circuit_program: &str, circuit_signals: &Vec<String>) -> Result<()> {
        let circuit = self.next_circuit_id()
            .and_then(|circuit_id| Circuit::new(circuit_id, circuit_name, circuit_program, circuit_signals))?;
        self.circuits.push(circuit);
        Ok(())
    }

    pub fn remove_circuit(&mut self, circuit_id: u32) -> Result<()> {
        self.circuits.remove(
            self.get_circuit_index(circuit_id)
                .ok_or(DomeError::CircuitNotFound)?
        );
        Ok(())
    }

    pub fn set_enabled(&mut self, circuit_id: u32, enabled: bool) -> Result<()> {
        self.find_circuit(circuit_id)
            .ok_or(DomeError::CircuitNotFound)
            .map_err(|e| e.into())
            .and_then(|x| x.set_enabled(enabled))
            .map(|_prev| ())
    }
}

