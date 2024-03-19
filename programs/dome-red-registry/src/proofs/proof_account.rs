use anchor_lang::prelude::*;

use crate::errors::DomeError;

const MAX_PROOF_LENGTH: usize = 4096;
const MAX_VERIFICATION_KEY_LENGTH: usize = 2048;

#[account]
#[derive(InitSpace)]
pub struct ProofAccount {
    #[max_len(MAX_PROOF_LENGTH)]
    proof: String,
    pub bump: u8,
    #[max_len(MAX_VERIFICATION_KEY_LENGTH)]
    verification_key: String,
}

impl ProofAccount {
    pub fn initialize(&mut self, proof: &str, verification_key: &str) -> Result<()> {
        if proof.len() > MAX_PROOF_LENGTH {
            return Err(DomeError::ProofTooLong.into());
        }
        if verification_key.len() > MAX_VERIFICATION_KEY_LENGTH {
            return Err(DomeError::InvalidProofVerificationKey.into());
        }
        self.proof = proof.to_owned();
        self.verification_key = verification_key.to_owned();
        Ok(())
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump = bump;
    }
}
