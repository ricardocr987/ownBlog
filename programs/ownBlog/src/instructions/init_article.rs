use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeArticle<'info> {
    #[account(
        init, 
        seeds = [ 
            b"article".as_ref(),
            author_account.authority.key().as_ref(),
            author_account.post_counter.to_le_bytes().as_ref()
        ], 
        bump, 
        payer = signer,
        space = ArticleAccount::space(name)
    )]
    article_account: Account<'info, ArticleAccount>,

    #[account(
        mut,
        seeds = [
            b"author",
            author_account.authority.key().as_ref()
        ],
        bump = author_account.author_bump,
        constraint = author_account.authority == *signer.key,
    )]
    author_account: Account<'info, AuthorAccount>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeArticle>,
    name: String,
    free: bool
) -> Result<()> {

    let article = &mut ctx.accounts.article_account;
    let author = &mut ctx.accounts.author_account;

    article.authority = author.authority;
    article.free = free; // true: accesible to everyone
    article.article_bump = *ctx.bumps.get("article").unwrap(); 
    article.id = author.post_counter;
    article.name = name;
   
    author.post_counter = author.post_counter + 1;

    Ok(())
}