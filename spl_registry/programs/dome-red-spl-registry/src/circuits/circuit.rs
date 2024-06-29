use anchor_lang::prelude::*;

const REF_NAME_LEN: usize = 32;
const REF_PROGRAM_LEN: usize = 1024;
const REF_SIGNAL_NAMES_NUM: usize = 16;
const REF_SIGNAL_NAME_LEN: usize = 16;

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone)]
pub struct Circuit {
    #[max_len(REF_NAME_LEN)]
    pub name: String,
    #[max_len(REF_PROGRAM_LEN)]
    pub program: String,
    #[max_len(REF_SIGNAL_NAMES_NUM, REF_SIGNAL_NAME_LEN)]
    pub signal_names: Vec<String>,
}

impl Circuit {
    pub fn new(name: &str, program: &str, signal_names: &Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            program: program.to_string(),
            signal_names: signal_names.clone()
        }
    }
}
