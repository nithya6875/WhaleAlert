use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;


declare_id!("83e8hA9WzwexTCEkEL35f5GsRvj7oo59pD74z9Ewj5PF");

#[program]
pub mod whale_monitor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, threshold: u64) -> ProgramResult {
        let data_account = &mut ctx.accounts.data_account;
        data_account.threshold = threshold;
        Ok(())
    }

    pub fn update_threshold(ctx: Context<UpdateThreshold>, new_threshold: u64) -> ProgramResult {
        let data_account = &mut ctx.accounts.data_account;
        data_account.threshold = new_threshold;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)] // Space for storing the threshold
    pub data_account: Account<'info, ThresholdData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateThreshold<'info> {
    #[account(mut)]
    pub data_account: Account<'info, ThresholdData>,
}

#[account]
pub struct ThresholdData {
    pub threshold: u64,
}
