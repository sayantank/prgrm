use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct UpdateGramMeta<'info> {
    #[account(mut, seeds=[b"gram".as_ref(), prog.key.as_ref()], bump, has_one = gram_authority)]
    pub gram: Account<'info, Gram>,

    /// The program for which a GRAM is being created
    #[account(executable)]
    pub prog: AccountInfo<'info>,

    pub gram_authority: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateGramMeta>, name: String, description: String, repo: String) -> ProgramResult {
    let gram = &mut ctx.accounts.gram;

    if name.len() > 30 || description.len() > 256 || repo.len() > 128 {
        return Err(ProgramError::InvalidArgument)
    }

    gram.name = name;
    gram.description = description;
    gram.repo = repo;
    
    Ok(())
}