use anchor_lang::prelude::*;

const REF_NAME_LEN: usize = 32;
const REF_PROGRAM_LEN: usize = 1024;
const REF_SIGNAL_NAMES_NUM: usize = 16;
const REF_SIGNAL_NAME_LEN: usize = 16;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub struct Circuit {
    id: u32,
    enabled: bool,
    #[max_len(REF_NAME_LEN)]
    name: String,
    #[max_len(REF_PROGRAM_LEN)]
    program: String,
    #[max_len(REF_SIGNAL_NAMES_NUM, REF_SIGNAL_NAME_LEN)]
    signal_names: Vec<String>,
}

impl Circuit {
    pub fn new(name: &str, program: &str, signal_names: &Vec<String>) -> Self {
        Self {
            id: 0,
            enabled: true,
            name: name.to_string(),
            program: program.to_string(),
            signal_names: signal_names.clone()
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_id(&mut self, id: u32) -> (&mut Self, u32) {
        self.id = id;
        (self, id)
    }

    pub fn set_enabled(&mut self, enabled: bool) -> (&mut Self, bool) {
        let prev_value = self.enabled;
        self.enabled = enabled;
        (self, prev_value)
    }
}
