use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeReader<'info> {
    #[account(
        init, 
        seeds = [ 
            b"reader".as_ref(),
            signer.key().as_ref(),
        ], 
        bump, 
        payer = signer,
        space = ReaderAccount::space(name)
    )]
    reader_account: Account<'info, ReaderAccount>,

    #[account(mut)]
    signer: Signer<'info>,
    
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeReader>, 
    name: String,
) -> Result<()> {
    let reader = &mut ctx.accounts.reader_account;

    reader.authority = *ctx.accounts.signer.to_account_info().key;
    reader.reader_bump = *ctx.bumps.get("reader").unwrap();
    reader.name = name;

    Ok(())
}