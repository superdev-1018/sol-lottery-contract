use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_lang::solana_program::system_instruction;
pub use crate::{account::*, constant::*, error::*};

#[derive(Accounts)]
pub struct PrizeDistribute<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK:this is unsafe
    #[account(mut)]
    pub pool_token_account: AccountInfo<'info>,

    #[account(mut)]
    pub lottery: Box<Account<'info, Lottery>>,

    /// CHECK:this is unsafe
    #[account(mut)]
    pub winner1_token_account: AccountInfo<'info>,

    /// CHECK:this is unsafe
    #[account(mut)]
    pub winner2_token_account: AccountInfo<'info>,

    /// CHECK:this is unsafe
    #[account(mut)]
    pub winner3_token_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn send_prize(ctx: Context<PrizeDistribute>) -> Result<()> {

    let lottery = &mut ctx.accounts.lottery;

    let winner1_prize = lottery.winner_prize[0];
    let winner2_prize = lottery.winner_prize[1];
    let winner3_prize = lottery.winner_prize[2];

    let ix1 = system_instruction::transfer(
        ctx.accounts.pool_token_account.key,
        ctx.accounts.winner1_token_account.key, 
        winner1_prize
    );

    let ix2 = system_instruction::transfer(
        ctx.accounts.pool_token_account.key,
        ctx.accounts.winner2_token_account.key, 
        winner2_prize
    );

    let ix3 = system_instruction::transfer(
        ctx.accounts.pool_token_account.key,
        ctx.accounts.winner3_token_account.key, 
        winner3_prize
    );

    msg!("Transferring {} lamports from {} to {}", winner1_prize, ctx.accounts.pool_token_account.key, ctx.accounts.winner1_token_account.key);
    msg!("Transferring {} lamports from {} to {}", winner2_prize, ctx.accounts.pool_token_account.key, ctx.accounts.winner2_token_account.key);
    msg!("Transferring {} lamports from {} to {}", winner3_prize, ctx.accounts.pool_token_account.key, ctx.accounts.winner3_token_account.key);


    anchor_lang::solana_program::program::invoke(
        &ix1,
        &[ctx.accounts.pool_token_account.clone(), ctx.accounts.winner1_token_account.clone()],
    )?;

    anchor_lang::solana_program::program::invoke(
        &ix2,
        &[ctx.accounts.pool_token_account.clone(), ctx.accounts.winner2_token_account.clone()],
    )?;

    anchor_lang::solana_program::program::invoke(
        &ix3,
        &[ctx.accounts.pool_token_account.clone(), ctx.accounts.winner3_token_account.clone()],
    )?;

    Ok(())
}