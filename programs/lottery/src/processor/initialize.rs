use anchor_lang::prelude::*;
use anchor_spl::token::Token;
pub use crate::{account::*, constant::*, error::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        init, 
        payer = initializer,
        seeds = [GLOBAL_SETTING, initializer.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<GlobalAccount>()
    )]
    pub global_account: Box<Account<'info, GlobalAccount>>,

    #[account(
        init,
        payer = initializer,
        seeds = [LOTTERY_PDAKEY_INFO],
        bump,
        space = 8 + std::mem::size_of::<LotteryPdaInfo>()
    )]
    pub lottery_pdakey_info: Box<Account<'info, LotteryPdaInfo>>,

    #[account(
        init,
        payer= initializer, 
        seeds=[WINNER_TICKER],
        bump,
        space= 8 + std::mem::size_of::<WinnerTicker>()
    )]
    pub winner_ticker: Box<Account<'info, WinnerTicker>>,

    #[account(
        init, 
        payer= initializer, 
        seeds=[DEPOSITE_TICKER], 
        bump, 
        space= 8 + std::mem::size_of::<DepositeTicker>()
    )]
    pub deposite_ticker: Box<Account<'info, DepositeTicker>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

// impl <'info>Initialize<'info> {
//     pub fn validate(&self) -> Result<()> {
//         if self.global_account.is_initialized == 1 {
//             require!(
//                 self.global_account.withdrawer.key() == self.initializer.key(),
//                 ContractError::InvalidAddress
//             );
//         }
//         Ok(())
//     }
// }

// #[access_control(ctx.accounts.validate())]
pub fn init(ctx: Context<Initialize>) -> Result<()> {
    msg!("This is {} function", "init");

    ctx.accounts.global_account.initializer = ctx.accounts.initializer.key();
    ctx.accounts.global_account.is_initialized = 1;
    ctx.accounts.lottery_pdakey_info.count = 0;
    ctx.accounts.lottery_pdakey_info.rounds = [0;10];
    ctx.accounts.winner_ticker.winner = Pubkey::default();
    ctx.accounts.winner_ticker.time_frame = 0;
    ctx.accounts.winner_ticker.prize = 0;
    ctx.accounts.deposite_ticker.depositer = Pubkey::default();
    ctx.accounts.deposite_ticker.time_frame = 0;
    ctx.accounts.deposite_ticker.spots = 0;
    ctx.accounts.deposite_ticker.amount = 0;
    Ok(())
}