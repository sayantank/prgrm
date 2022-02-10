use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct InitGram<'info> {
    /// PDA derived from MY program, but has prog's key as seed
    #[account(init, seeds=[b"gram".as_ref(), prog.key.as_ref()], bump, payer=payer, space=Gram::LEN)]
    pub gram: Account<'info, Gram>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// The program for which a GRAM is being created
    #[account(executable)]
    pub prog: AccountInfo<'info>,

    // /// The CALLING program's PDA that acts as a signer
    // /// Has to be derived from the calling program, with seeds = ["gram_authority"]
    // pub gram_authority: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<InitGram>, name: String, description: String, repo: String) -> ProgramResult {
    // check if gram_authority is valid
    let prog = &ctx.accounts.prog;
    
    if name.len() > 30 || description.len() > 256 || repo.len() > 128 {
        return Err(ProgramError::InvalidArgument)
    }

    let (derived_gram_auth, _derived_auth_bump) = Pubkey::find_program_address(
        &[b"gram_authority"], 
        prog.key
    );
    // if ctx.accounts.gram_authority.key() != derived_gram_auth {
    //     msg!("Invalid gram_authority");
    //     return Err(ProgramError::InvalidAccountData)
    // }

    let gram = &mut ctx.accounts.gram;
    gram.is_initialized = true;
    gram.name = name;
    gram.description = description;
    gram.repo = repo;
    gram.gram_authority = derived_gram_auth;
    gram.prog_key = prog.key();
    gram.num_likes = 0;

    Ok(())
}
