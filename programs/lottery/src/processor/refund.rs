use anchor_lang::prelude::*;
use anchor_spl::token::Token;
pub use crate::{account::*, constant::*, error::*};
use anchor_lang::solana_program::system_instruction;

#[derive(Accounts)]
pub struct ReFund<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub lottery: Box<Account<'info, Lottery>>,

    /// CHECK:this is unsafe
    #[account(mut)]
    pub pool_token_account: AccountInfo<'info>,

    /// CHECK:this is unsafe
    #[account(mut)]
    pub participant_token_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn refund_ticket_price(ctx: Context<ReFund>) -> Result<()> {

    let lottery = &mut ctx.accounts.lottery;

    let ticket_price = lottery.ticket_price as u64;


    let ix = system_instruction::transfer(
        ctx.accounts.pool_token_account.key, 
        ctx.accounts.participant_token_account.key, 
        ticket_price);
    
    msg!("Transferring {} lamports from {} to {}", ticket_price, ctx.accounts.pool_token_account.key, ctx.accounts.participant_token_account.key);

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[ctx.accounts.pool_token_account.clone(), ctx.accounts.participant_token_account.clone()],
    )?;

    lottery.state = 0;
    lottery.participants = [].to_vec();

    Ok(())
}