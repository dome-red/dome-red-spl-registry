use anchor_lang::prelude::*;
use crate::errors::DomeError;
use super::{Circuit, CircuitItem};

const REF_CIRCUITS_NUM: usize = 4; //32;

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone)]
pub struct CircuitsPool {
    #[max_len(REF_CIRCUITS_NUM, Circuit::INIT_SPACE)]
    pub circuit_items: Vec<CircuitItem>,
    pub last_used_id: u32,
}

impl CircuitsPool {
    pub fn next_circuit_item_id(&mut self) -> Result<u32> {
        match self.last_used_id.checked_add(1) {
            None => Err(DomeError::MaxCircuitsNumReached.into()),
            Some(last_user_id) => {
                self.last_used_id = last_user_id;
                Ok(self.last_used_id)
            }
        }
    }

    pub fn get_circuit_item_index(&self, circuit_item_id: u32) -> Result<usize> {
        self.circuit_items.iter()
            .position(|x| x.get_id().eq(&circuit_item_id))
            .ok_or(DomeError::CircuitNotFound.into())
    }

    pub fn find_circuit_item(&mut self, circuit_item_id: u32) -> Result<&mut CircuitItem> {
        let circuit_item_index = self.get_circuit_item_index(circuit_item_id)?;
        self.circuit_items.get_mut(circuit_item_index).ok_or(DomeError::CircuitNotFound.into())
    }

    pub fn add_circuit_item(&mut self, circuit: &Circuit) -> Result<()> {
        self.next_circuit_item_id().map(|circuit_id| {
            let circuit_item = CircuitItem::new(circuit).set_id(circuit_id).0.to_owned();
            self.circuit_items.push(circuit_item);
        })
    }

    pub fn remove_circuit_item(&mut self, circuit_item_id: u32) -> Result<()> {
        let circuit_item_index = self.get_circuit_item_index(circuit_item_id)?;
        self.circuit_items.remove(circuit_item_index);
        Ok(())
    }

    pub fn set_circuit_item_enabled(&mut self, circuit_item_id: u32, enabled: bool) -> Result<()> {
        self.find_circuit_item(circuit_item_id)
            .map(|x| {
                x.set_enabled(enabled);
            })
    }
}

