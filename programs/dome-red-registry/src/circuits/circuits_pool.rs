use anchor_lang::__private::bytemuck::Contiguous;
use anchor_lang::prelude::*;

use crate::errors::DomeError;

use super::Circuit;

const MAX_CIRCUITS_NUM: usize = 1024;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace, Default)]
pub struct CircuitsPool {
    #[max_len(MAX_CIRCUITS_NUM, Circuit::INIT_SPACE)]
    circuits: Vec<Circuit>,
}

impl CircuitsPool {
    pub fn next_circuit_id(&self) -> Result<u32> {
        let last_id = self.circuits.last().map(|x| x.id).unwrap_or(0);
        match last_id {
            u32::MAX_VALUE => Err(DomeError::MaxCircuitsNumReached.into()),
            _ => Ok(last_id + 1)
        }
    }

    pub fn get_circuit_index(&self, circuit_id: u32) -> Option<usize> {
        self.circuits.iter()
            .filter(|x| x.enabled)
            .position(|x| x.id == circuit_id)
    }

    pub fn find_circuit(&mut self, circuit_id: u32) -> Option<&mut Circuit> {
        self.get_circuit_index(circuit_id)
            .and_then(|index| self.circuits.get_mut(index))
    }

    pub fn add_circuit(&mut self, circuit_name: &str, circuit_program: &str) -> Result<()> {
        if self.circuits.len() >= MAX_CIRCUITS_NUM {
            return Err(DomeError::MaxCircuitsNumReached.into());
        }
        self.circuits.push(
            self.next_circuit_id()
                .and_then(|circuit_id| Circuit::new(circuit_id, circuit_name, circuit_program))?
        );
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

