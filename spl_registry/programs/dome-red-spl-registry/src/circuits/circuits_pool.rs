use anchor_lang::prelude::*;
use crate::errors::DomeError;
use super::Circuit;

const REF_CIRCUITS_NUM: usize = 4; //32;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace, Default)]
pub struct CircuitsPool {
    #[max_len(REF_CIRCUITS_NUM, Circuit::INIT_SPACE)]
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

    pub fn get_circuit_index(&self, circuit_id: u32) -> Result<usize> {
        self.circuits.iter()
            .position(|x| x.get_id().eq(&circuit_id))
            .ok_or(DomeError::CircuitNotFound.into())
    }

    pub fn find_circuit(&mut self, circuit_id: u32) -> Result<&mut Circuit> {
        let circuit_index = self.get_circuit_index(circuit_id)?;
        self.circuits.get_mut(circuit_index).ok_or(DomeError::CircuitNotFound.into())
    }

    pub fn add_circuit(&mut self, circuit: &Circuit) -> Result<()> {
        self.next_circuit_id().map(|circuit_id| {
            let circuit = circuit.clone().set_id(circuit_id).0.to_owned();
            self.circuits.push(circuit);
        })
    }

    pub fn remove_circuit(&mut self, circuit_id: u32) -> Result<()> {
        let circuit_index = self.get_circuit_index(circuit_id)?;
        self.circuits.remove(circuit_index);
        Ok(())
    }

    pub fn set_enabled(&mut self, circuit_id: u32, enabled: bool) -> Result<()> {
        self.find_circuit(circuit_id)
            .map(|x| {
                x.set_enabled(enabled);
            })
    }
}

