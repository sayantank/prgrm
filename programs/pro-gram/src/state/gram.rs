use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Gram {
    pub is_initialized: bool,
    pub prog_key: Pubkey,
    pub gram_authority: Pubkey, // PDA of prog_key
    pub num_likes: u64,
    pub name: String, // 30 chars
    pub description: String, // 256 chars
    pub repo: String // 128 chars
}

impl Gram {
    pub const LEN: usize = 8 + 1 + 32 + 8 + (4 + (30*4)) + (4 + (256*4)) + (4 + (128*4)); 
}