use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self, Token};
use std::mem::size_of;
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("11111111111111111111111111111111");

#[program]
mod nft_music {
    use super::*;

    pub fn accept_payment(ctx: Context<PayerContext>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data); // Message will show up in the tx logs
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PayerContext<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(init, seeds = [b"prayer".as_ref(), authority.key().as_ref()], bump  payer = authority, space = size_of::<PayerAccount> + 8)]
    pub payer_wallet: Account<'info, PayerAccount>,

    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct NewAccount {
    data: u64,
}
