use anchor_lang::prelude::*;

use crate::errors::DomeError;

const MAX_PROOF_LENGTH: usize = 4096;
const MAX_PUBLIC_LENGTH: usize = 4096;
const MAX_VERIFICATION_KEY_LENGTH: usize = 2048;

#[account]
#[derive(InitSpace)]
pub struct ProofAccount {
    #[max_len(MAX_PROOF_LENGTH)]
    proof: String,
    #[max_len(MAX_PUBLIC_LENGTH)]
    public: String,
    #[max_len(MAX_VERIFICATION_KEY_LENGTH)]
    verification_key: String,
    pub bump: u8,
}

impl ProofAccount {
    pub fn initialize(&mut self, proof: &str, public: &str, verification_key: &str) -> Result<()> {
        if proof.len() > MAX_PROOF_LENGTH {
            return Err(DomeError::ProofTooLong.into());
        }
        if public.len() > MAX_PUBLIC_LENGTH {
            return Err(DomeError::ProofPublicTooLong.into());
        }
        if verification_key.len() > MAX_VERIFICATION_KEY_LENGTH {
            return Err(DomeError::ProofVerificationKeytooLong.into());
        }
        self.proof = proof.to_owned();
        self.public = public.to_owned();
        self.verification_key = verification_key.to_owned();
        Ok(())
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump = bump;
    }
}
