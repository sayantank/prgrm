use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct LikeGram<'info> {
    #[account(init, seeds=[b"like".as_ref(), liker.key.as_ref(), gram.key().as_ref()], bump, payer=liker, space=Like::LEN)]
    pub like: Account<'info, Like>,

    #[account(mut)]
    pub liker: Signer<'info>,

    #[account(mut, seeds=[b"gram".as_ref(), prog.key.as_ref()], bump)]
    pub gram: Account<'info, Gram>,

    #[account(executable)]
    pub prog: AccountInfo<'info>,

    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<LikeGram>, comment: String) -> ProgramResult {
    if comment.len() > 256 {
        return Err(ProgramError::InvalidArgument)
    }
    let like = &mut ctx.accounts.like;
    like.liker = ctx.accounts.liker.key();
    like.prog_key = ctx.accounts.prog.key();
    like.comment = comment;

    let gram = &mut ctx.accounts.gram;
    gram.num_likes += 1;
    
    Ok(())
}