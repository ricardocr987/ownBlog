use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeAuthor<'info> {
    #[account(
        init, 
        seeds = [ 
            b"author".as_ref(),
            signer.key().as_ref(),
        ], 
        bump, 
        payer = signer,
        space = AuthorAccount::space(name)
    )]
    author_account: Account<'info, AuthorAccount>,

    #[account(mut)]
    signer: Signer<'info>,
    
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeAuthor>,
    name: String,
    price_sub: u8,
) -> Result<()> {

    let author = &mut ctx.accounts.author_account;

    author.authority = *ctx.accounts.signer.to_account_info().key;
    author.price_sub = price_sub;
    author.author_bump = *ctx.bumps.get("author").unwrap(); 
    author.post_counter = 0;
    author.name = name;

    Ok(())
}