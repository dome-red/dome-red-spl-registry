use anchor_lang::prelude::*;

// Maximum PDA size in one iteration is 10240 bytes.
const REF_PROOF_LEN: usize = 4096;
const REF_PUBLIC_SIGNALS_NUM: usize = 16;
const REF_PUBLIC_SIGNAL_LEN: usize = 16;
const REF_VERIFICATION_KEY_LEN: usize = 2048;

#[account]
#[derive(InitSpace, Default)]
pub struct ProofAccount {
    #[max_len(REF_PROOF_LEN)]
    proof: String,
    #[max_len(REF_PUBLIC_SIGNALS_NUM, REF_PUBLIC_SIGNAL_LEN)]
    public_signals: Vec<String>,
    #[max_len(REF_VERIFICATION_KEY_LEN)]
    verification_key: String,
    pub(crate) bump: u8,
}

impl ProofAccount {
    pub fn setup(&mut self, proof: &str, public_signals: &Vec<String>, verification_key: &str) -> &mut Self {
        self.proof = proof.to_string();
        self.public_signals = public_signals.clone();
        self.verification_key = verification_key.to_string();
        self
    }

    pub fn set_bump(&mut self, bump: u8) -> (&mut Self, u8) {
        let prev_value = self.bump;
        self.bump = bump;
        (self, prev_value)
    }
}
