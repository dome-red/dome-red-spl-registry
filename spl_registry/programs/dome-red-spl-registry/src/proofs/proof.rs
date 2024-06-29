use anchor_lang::prelude::*;
use serde::{Serialize, Deserialize};

// Maximum PDA size in one iteration is 10240 bytes.
const REF_PROOF_LEN: usize = 4096;
const REF_PUBLIC_SIGNALS_NUM: usize = 16;
const REF_PUBLIC_SIGNAL_LEN: usize = 16;
const REF_VERIFICATION_KEY_LEN: usize = 2048;

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone, Serialize, Deserialize)]
pub struct Proof {
    #[max_len(REF_PROOF_LEN)]
    pub proof: String,
    #[max_len(REF_PUBLIC_SIGNALS_NUM, REF_PUBLIC_SIGNAL_LEN)]
    pub public_signals: Vec<String>,
    #[max_len(REF_VERIFICATION_KEY_LEN)]
    pub verification_key: String,
}

impl Proof {
    pub fn new(proof: &str, public_signals: &Vec<String>, verification_key: &str) -> Self {
        Self {
            proof: proof.to_string(),
            public_signals: public_signals.clone(),
            verification_key: verification_key.to_string()
        }
    }
}
