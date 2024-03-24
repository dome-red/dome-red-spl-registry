use anchor_lang::prelude::*;

use oracles::OracleAccount;
use proofs::ProofAccount;

mod errors;
mod circuits;
mod oracles;
mod proofs;

declare_id!("DWPQtKoCh7daDAqJa5LAw2QT9aZtgXk38QBSpTz6yoVX");

#[program]
mod dome_red_spl_registry {
    use super::*;

    pub fn register_oracle(ctx: Context<RegisterOracle>, oracle_name: String) -> Result<()> {
        let oracle_account = &mut ctx.accounts.oracle_account;
        oracle_account.initialize(&oracle_name)?;
        oracle_account.set_bump(ctx.bumps.oracle_account);
        Ok(())
    }

    pub fn enable_oracle(ctx: Context<OracleControl>) -> Result<()> {
        let oracle_account = &mut ctx.accounts.oracle_account;
        oracle_account.set_enabled(true).map(|_| ())
    }

    pub fn disable_oracle(ctx: Context<OracleControl>) -> Result<()> {
        let oracle_account = &mut ctx.accounts.oracle_account;
        oracle_account.set_enabled(false).map(|_| ())
    }

    pub fn change_oracle(
        ctx: Context<OracleControl>,
        oracle_name: String,
    ) -> Result<()> {
        let oracle_account = &mut ctx.accounts.oracle_account;
        oracle_account.set_name(&oracle_name)?;
        Ok(())
    }

    // -----

    pub fn register_circuit(ctx: Context<OracleControl>, circuit_name: String, circuit_program: String) -> Result<()> {
        let oracle_account = &mut ctx.accounts.oracle_account;
        oracle_account.circuits_pool().add_circuit(&circuit_name, &circuit_program)
    }

    pub fn remove_circuit(ctx: Context<OracleControl>, circuit_id: u32) -> Result<()> {
        let oracle_account = &mut ctx.accounts.oracle_account;
        oracle_account.circuits_pool().remove_circuit(circuit_id)
    }

    pub fn enable_circuit(ctx: Context<OracleControl>, circuit_id: u32) -> Result<()> {
        let oracle_account = &mut ctx.accounts.oracle_account;
        oracle_account.circuits_pool().set_enabled(circuit_id, true)
    }

    pub fn disable_circuit(ctx: Context<OracleControl>, circuit_id: u32) -> Result<()> {
        let oracle_account = &mut ctx.accounts.oracle_account;
        oracle_account.circuits_pool().set_enabled(circuit_id, false)
    }

    // -----

    pub fn register_proof(ctx: Context<RegisterProof>, _target_oracle_hash: String, _circuit_id: u32, _user_pubkey_hash: String, proof: String, public: String, verification_key: String) -> Result<()> {
        let proof_account = &mut ctx.accounts.proof_account;
        proof_account.initialize(&proof, &public, &verification_key)?;
        proof_account.set_bump(ctx.bumps.proof_account);
        Ok(())
    }

    pub fn increase_account_size(_ctx: Context<IncreaseAccoutSize>, _len: u32) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct RegisterOracle<'info> {
    #[account(mut)]
    pub oracle: Signer<'info>,
    #[account(
        init_if_needed,
        payer = oracle,
        space = 8 + OracleAccount::INIT_SPACE,
        seeds = [b"oracle", oracle.key().as_ref()],
        bump
    )]
    pub oracle_account: Account<'info, OracleAccount>,
    pub system_program: Program<'info, System>,
}

// validation struct
#[derive(Accounts)]
pub struct OracleControl<'info> {
    pub oracle: Signer<'info>,
    #[account(mut, seeds = [b"oracle", oracle.key().as_ref()], bump = oracle_account.bump)]
    pub oracle_account: Account<'info, OracleAccount>,
}

#[derive(Accounts)]
#[instruction(
    target_oracle_hash: String,
    circuit_id: u32,
    user_pubkey_hash: String,
    proof: String,
    public: String,
    verification_key: String
)]
pub struct RegisterProof<'info> {
    #[account(mut)]
    pub oracle: Signer<'info>,
    #[account(
        init_if_needed,
        payer = oracle,
        space = 8 + ProofAccount::INIT_SPACE,
        seeds = [
            b"proof",
            oracle.key().as_ref(),
            target_oracle_hash.to_lowercase().as_bytes(),
            circuit_id.to_le_bytes().as_slice(),
            user_pubkey_hash.to_lowercase().as_bytes()
        ],
        bump
    )]
    pub proof_account: Account<'info, ProofAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(len: u32)]
pub struct IncreaseAccoutSize<'info> {
    #[account(mut)]
    pub oracle: Signer<'info>,
    #[account(
        mut,
        realloc = len as usize,
        realloc::zero = true,
        realloc::payer=oracle,
        seeds = [b"oracle", oracle.key().as_ref()],
        bump = oracle_account.bump
    )]
    pub oracle_account: Account<'info, OracleAccount>,
    pub system_program: Program<'info, System>
}