use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_supply: u64) -> Result<()> {
        let token = &mut ctx.accounts.token;
        let admin_ownership = &mut ctx.accounts.admin_ownership;
        token.owner = ctx.accounts.signer.key();
        token.total_supply = initial_supply;
        admin_ownership.holder = ctx.accounts.signer.key();
        admin_ownership.balance = initial_supply;
        Ok(())
    }

    pub fn transfer_to_empty(ctx: Context<TransferToEmpty>, to: Pubkey, amount: u64) -> Result<()> {
        let from_account = &mut ctx.accounts.from;
        require!(from_account.holder == ctx.accounts.signer.key(), TokenErrors::SenderHasNoAccess);
        let to_account = &mut ctx.accounts.to;
        require!(to_account.holder == Pubkey::default(), TokenErrors::RecipientAccountNotEmpty);
        require!(from_account.balance >= amount, TokenErrors::InsufficientBalance);
        from_account.balance -= amount;
        to_account.balance = amount;
        to_account.holder = to;
        Ok(())
    }

    pub fn transfer_to_existing(ctx: Context<TransferToExisting>, to: Pubkey, amount: u64) -> Result<()> {
        let from_account = &mut ctx.accounts.from;
        require!(from_account.holder == ctx.accounts.signer.key(), TokenErrors::SenderHasNoAccess);
        let to_account = &mut ctx.accounts.to;
        require!(to_account.holder == to, TokenErrors::RecipientAccountNotEmpty);
        require!(from_account.balance >= amount, TokenErrors::InsufficientBalance);
        from_account.balance -= amount;
        to_account.balance += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 256)]
    pub token: Account<'info, Token>,
    #[account(init, payer = signer, space = 256)]
    pub admin_ownership: Account<'info, TokenOwnership>,
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
pub struct TokenOwnership {
    pub holder: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct TransferToEmpty<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenOwnership>,
    #[account(init, payer = signer, space = 256)]
    pub to: Account<'info, TokenOwnership>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferToExisting<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenOwnership>,
    #[account(mut)]
    pub to: Account<'info, TokenOwnership>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum TokenErrors {
    SenderHasNoAccess,
    RecipientAccountNotEmpty,
    InsufficientBalance,
}