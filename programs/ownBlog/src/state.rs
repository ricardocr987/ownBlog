use anchor_lang::prelude::*;

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const U64: usize = 8;
const U32: usize = 4;
const U16: usize = 2;
const U8: usize = 1;

#[account]
pub struct AuthorAccount {
    pub authority: Pubkey,
    pub price_sub: u8,
    pub author_bump: u8,
    pub post_counter: u16,
    pub name: String
}

impl AuthorAccount{
    pub fn space(name: String) -> usize {
        DISCRIMINATOR_LENGTH +
        PUBLIC_KEY_LENGTH +
        U8 +
        U8 +
        U16 +
        U32 + name.len()
    }
}

#[account]
pub struct ArticleAccount {
    pub authority: Pubkey,
    pub free: bool,
    pub article_bump: u8,
    pub id: u16,
    pub name: String
}

impl ArticleAccount{
    pub fn space(name: String) -> usize {
        DISCRIMINATOR_LENGTH +
        PUBLIC_KEY_LENGTH +
        U8 +
        U8 +
        U16 +
        U32 + name.len()
    }
}

#[account]
pub struct ReaderAccount {
    pub authority: Pubkey,
    pub reader_bump: u8,
    pub name: String
}

impl ReaderAccount {
    pub fn space(name: String) -> usize {
        DISCRIMINATOR_LENGTH +
        PUBLIC_KEY_LENGTH +
        U8 +
        U32 + name.len()
    }
}

#[account]
pub struct SubscriberAccount {
    pub authority: Pubkey,
    pub author_subscribed: Pubkey,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub sub_bump: u8
}

impl SubscriberAccount {
    pub const LEN: usize = DISCRIMINATOR_LENGTH 
        + PUBLIC_KEY_LENGTH 
        + PUBLIC_KEY_LENGTH 
        + U64 + U64 + U8;
}