use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct UpdateGramAuth<'info> {
    #[account(mut, seeds=[b"gram".as_ref(), prog.key.as_ref()], bump, has_one = gram_authority)]
    pub gram: Account<'info, Gram>,

    /// The program for which a GRAM is being created
    #[account(executable)]
    pub prog: AccountInfo<'info>,

    pub new_authority: Signer<'info>,

    pub gram_authority: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateGramAuth>) -> ProgramResult {
    let gram = &mut ctx.accounts.gram;
    gram.gram_authority = ctx.accounts.new_authority.key();
    Ok(())
}