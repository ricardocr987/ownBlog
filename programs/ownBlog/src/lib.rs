use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod state;
pub mod instructions;

#[program]
pub mod own_blog {
    use super::*;

    pub fn initialize_author(
        ctx: Context<InitializeAuthor>, 
        name: String,
        price_sub: u8
    ) -> Result<()> {
        instructions::init_author::handler(ctx, name, price_sub)
    }

    pub fn initialize_article(
        ctx: Context<InitializeArticle>, 
        name: String,
        free: bool
    ) -> Result<()> {
        instructions::init_article::handler(ctx, name, free)
    }

    pub fn update_article(
        ctx: Context<UpdateArticle>, 
        name: String,
        free: bool
    ) -> Result<()> {
        instructions::update_article::handler(ctx, name, free)
    }
    
    pub fn initialize_reader(
        ctx: Context<InitializeReader>, 
        name: String,
    ) -> Result<()> {
        instructions::init_reader::handler(ctx, name)
    }
    
    pub fn initialize_subscriber(
        ctx: Context<InitializeSubscriber>, 
        author_subscribed: Pubkey,
        start_timestamp: i64,
        end_timestamp: i64
    ) -> Result<()> {
        instructions::init_subscriber::handler(ctx, author_subscribed, start_timestamp, end_timestamp)
    }
}
