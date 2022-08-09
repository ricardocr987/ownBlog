use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct InitializeSubscriber<'info> {
    #[account(
        init, 
        seeds = [ 
            b"subscriber".as_ref(),
            signer.key().as_ref(),
        ], 
        bump, 
        payer = signer,
        space = SubscriberAccount::LEN
    )]
    subscriber_account: Account<'info, SubscriberAccount>,

    #[account(mut)]
    signer: Signer<'info>,
    
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeSubscriber>, 
    author_subscribed: Pubkey,
    start_timestamp: i64,
    end_timestamp: i64
) -> Result<()> {

    let subscriber = &mut ctx.accounts.subscriber_account;

    subscriber.authority = *ctx.accounts.signer.to_account_info().key;
    subscriber.author_subscribed = author_subscribed;
    subscriber.start_timestamp = start_timestamp;
    subscriber.end_timestamp =end_timestamp;
    subscriber.sub_bump = *ctx.bumps.get("subscriber").unwrap();

    Ok(())
}