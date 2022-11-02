use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self, Token};
use std::mem::size_of;
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("3Fyk9Hi8T9XF998uBW1tJKLAXUTSz4xtZQHzb6L3htbF");

const TEXT_LENGTH: usize = 255;
const MUSIC_URL_LENGTH: usize = 255;

#[program]
mod nft_music {
    use super::*;

    pub fn accept_payment(ctx: Context<PayerContext>) -> ProgramResult {
        let payer_wallet = &mut ctx.accounts.payer_wallet;
        payer_wallet.wallet = ctx.accounts.authority.key();

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.authority.key(),
            &ctx.accounts.receiver.key(),
            100000000,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
            ],
        )
    }

    pub fn create_music(
        ctx: Context<CreateMusic>,
        title: String,
        music_url: String,
    ) -> ProgramResult {
        let music = &mut ctx.accounts.music;

        music.authority = ctx.accounts.authority.key();
        music.title = title;
        music.music_url = music_url;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct PayerContext<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(init, seeds = [b"prayer".as_ref(), authority.key().as_ref()], bump,  payer = authority, space = size_of::<PayerAccount>() + 8)]
    pub payer_wallet: Account<'info, PayerAccount>,

    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: UncheckedAccount<'info>,

    // Token Program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct PayerAccount {
    pub wallet: Pubkey,
}

#[derive(Accounts)]
pub struct CreateMusic<'info> {
    #[account(
        init,
        seeds=[b"music".as_ref(), randomkey.key().as_ref()],
        bump,
        payer = authority,
        space = size_of::<MusicAccount>() + TEXT_LENGTH + MUSIC_URL_LENGTH + 8
    )]
    pub music: Account<'info, MusicAccount>,

    #[account(mut)]
    pub randomkey: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: UncheckedAccount<'info>,

    // Token Program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,

    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct MusicAccount {
    pub authority: Pubkey,
    pub title: String,
    pub music_url: String,
}
