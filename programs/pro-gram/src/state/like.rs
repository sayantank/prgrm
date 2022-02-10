use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Like {
    pub prog_key: Pubkey,
    pub liker: Pubkey,
    pub comment: String,
}

impl Like {
    pub const LEN: usize = 8 + 32 + 32 + (4 + (256*4));
}