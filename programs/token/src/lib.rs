use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_supply: u64) -> Result<()> {
        let token = &mut ctx.accounts.token;
        let admin_balance = &mut ctx.accounts.admin_balance;
        token.owner = ctx.accounts.signer.key();
        token.total_supply = initial_supply;
        admin_balance.holder = ctx.accounts.signer.key();
        admin_balance.balance = initial_supply;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=signer, space=256)]
    pub token: Account<'info, Token>,
    #[account(init, payer=signer, space=256)]
    pub admin_balance: Account<'info, Balance>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Token {
    pub owner: Pubkey,
    pub total_supply: u64,
}

#[account]
pub struct Balance {
    pub holder: Pubkey,
    pub balance: u64,
}
