use anchor_lang::prelude::*;
use super::Proof;

#[account]
#[derive(InitSpace)]
pub struct ProofAccount {
    proof: Proof,
    pub(crate) bump: u8,
}

impl ProofAccount {
    pub fn setup(&mut self, proof: &Proof) -> &mut Self {
        self.proof = proof.clone();
        self
    }

    pub fn set_bump(&mut self, bump: u8) -> (&mut Self, u8) {
        let prev_value = self.bump;
        self.bump = bump;
        (self, prev_value)
    }
}
