use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_lang::solana_program::system_instruction;
pub use crate::error::*;
pub use crate::account::*;
pub use crate::constant::*;

#[derive(Accounts)]
pub struct SetReferralLink<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub user: Box<Account<'info, User>>,
}

#[derive(Accounts)]
// #[instruction(id: u8, admin_key:Pubkey)]
pub struct BuyTicketWithReferral<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub global_account: Box<Account<'info, GlobalAccount>>,

    /// CHECK:this is unsafe
    #[account(mut)]
    pub pool_token_account: AccountInfo<'info>,

    /// CHECK:this is unsafe
    #[account(mut)]
    pub buyer_token_account: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = buyer, 
        seeds = [USER_INFO, buyer.key().as_ref()], 
        bump, 
        space = 8 + std::mem::size_of::<User>()
    )]
    pub user: Box<Account<'info, User>>,

    #[account(mut)]
    pub referrer: Box<Account<'info, User>>,

    // #[account(mut, seeds = [LOTTERY_INFO, admin_key.as_ref(), &id.to_le_bytes()],bump,)]
    #[account(mut)]
    pub lottery: Box<Account<'info, Lottery>>,

    #[account(mut)]
    pub deposite_ticker: Box<Account<'info, DepositeTicker>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}



pub fn setreferral(ctx: Context<SetReferralLink>, referral_link: String) -> Result<()> {
    let user = &mut ctx.accounts.user;
    require!(user.id == ctx.accounts.signer.key(), ContractError::InvalidUserAccount);
    if user.referral_link == referral_link {
        return Err(ContractError::ReferralLinkAlreadyExist.into());
    } else {
        user.referral_link = referral_link;
        Ok(())
    }
}

pub fn buy_with_referral(ctx: Context<BuyTicketWithReferral>, count:u8) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let buyer = &ctx.accounts.buyer;
    let user =&mut ctx.accounts.user;
    let transfer_amount = (lottery.ticket_price as u64) * (count as u64); 

    msg!("transfer token amount {}", transfer_amount);
    msg!("Buyer token account owner: {:?}", ctx.accounts.buyer_token_account.owner);
    msg!("Authority for transfer: {:?}", ctx.accounts.buyer.key);

    let ix = system_instruction::transfer(
        ctx.accounts.pool_token_account.key, 
        ctx.accounts.buyer_token_account.key, 
        transfer_amount);
    
    msg!("Transferring {} lamports from {} to {}", transfer_amount, ctx.accounts.pool_token_account.key, ctx.accounts.buyer_token_account.key);

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[ctx.accounts.pool_token_account.clone(), ctx.accounts.buyer_token_account.clone()],
    )?;

    lottery.real_pool_amount += lottery.ticket_price as u64; 
    user.id = buyer.key();
    let lottery_timeframe = lottery.time_frame;

    let time_frames = [1, 3, 6, 12, 24, 168, 720, 2160, 4320, 8640];
    if let Some(index) = time_frames.iter().position(|&timeframe| timeframe == lottery_timeframe) {
        user.spot[index] += count;
    }

    let deposite_ticker = &mut ctx.accounts.deposite_ticker;
    deposite_ticker.depositer = buyer.key();
    deposite_ticker.time_frame = lottery.time_frame;
    deposite_ticker.spots = count;
    deposite_ticker.amount = lottery.ticket_price * (count as u64);

    let current_referrer = user.referrer;

    let referrer =&mut ctx.accounts.referrer;

    if current_referrer == Pubkey::default() && user.key() != referrer.id {
        user.referrer = referrer.id;
        referrer.spot[0] += 1;
    } else {
        msg!("Already added this referrer {}", referrer.id);
    }


    Ok(())
}
