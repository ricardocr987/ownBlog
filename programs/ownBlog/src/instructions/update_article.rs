use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct UpdateArticle<'info> {
    #[account(
        mut,
        seeds = [ 
            b"article".as_ref(),
            author_account.authority.key().as_ref(),
            author_account.post_counter.to_le_bytes().as_ref()
        ], 
        bump = article_account.article_bump, 
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
}

pub fn handler(
    ctx: Context<UpdateArticle>,
    name: String,
    free: bool
) -> Result<()> {

    let article = &mut ctx.accounts.article_account;
    let author = &mut ctx.accounts.author_account;

    article.authority = author.authority;
    article.free = free; // true: accesible to everyone
    article.name = name;
   
    Ok(())
}