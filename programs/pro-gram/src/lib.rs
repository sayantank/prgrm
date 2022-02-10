use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;
pub use state::*;

declare_id!("EEs91kXyAGSMAajLKejmBeqxBxHEM7ZhZtGikwsNoEwa");

#[program]
pub mod pro_gram {
    use super::*;
    pub fn init_gram(ctx: Context<InitGram>, name: String, description: String, repo: String) -> ProgramResult {
        instructions::init_gram::handler(ctx, name, description, repo)
    }

    pub fn update_gram_auth(ctx: Context<UpdateGramAuth>) -> ProgramResult {
        instructions::update_gram_auth::handler(ctx)
    }

    pub fn update_gram_meta(ctx: Context<UpdateGramMeta>, name: String, description: String, repo: String) -> ProgramResult {
        instructions::update_gram_meta::handler(ctx, name, description, repo)
    }

    pub fn like_gram(ctx: Context<LikeGram>, comment: String) -> ProgramResult {
        instructions::like_gram::handler(ctx, comment)
    }
}
