use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, InitializeAccount, Token};
use solana_program::program_pack::Pack;

declare_id!("DHdbzP1fvKzcCVmyseygBmWjQL3NTWqcEK87LUrLbUf9");

#[program]
pub mod rew {
    use solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_token_account_if_not_exists(ctx: Context<CreateTokenAccount>, user_pubkey: Pubkey) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(init, payer = user, token::mint = token_mint, token::authority = user)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    // If you are using Bumps, include it
    // pub bumps: Bumps,
}


#[derive(Accounts)]
pub struct Initialize {}
