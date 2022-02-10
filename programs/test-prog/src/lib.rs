use anchor_lang::prelude::*;
use pro_gram::cpi::accounts::{InitGram, UpdateGramAuth};
use pro_gram::program::ProGram;
use pro_gram::{self, Gram};

declare_id!("E69tkfspJSQJYBNXfk6wmVTtxVUvmtC1fdJwMSZuTejE");

#[program]
pub mod test_prog {
    use super::*;
    const GRAM_AUTH_SEED: &[u8] = b"gram_authority";

    pub fn change_auth(ctx: Context<ChangeAuth>) -> ProgramResult {

        // Not doing the check below because pro-gram does the check

        // let (gram_key, _gram_bump) = Pubkey::find_program_address(&[b"gram".as_ref(), id().as_ref()], pro_gram.key);
        // if gram_key != ctx.accounts.gram.key() {
        //     msg!("Invalid gram account!!");
        //     return Err(ProgramError::InvalidAccountData)
        // }

        let pro_gram = ctx.accounts.pro_gram.to_account_info();

        let (gram_auth_pda, gram_auth_bump) = Pubkey::find_program_address(&[GRAM_AUTH_SEED], ctx.program_id);

        if gram_auth_pda != ctx.accounts.gram_authority.key() {
            msg!("Invalid gram auth!!!!");
            return Err(ProgramError::InvalidAccountData)
        }
        let cpi_accounts = UpdateGramAuth {
            gram: ctx.accounts.gram.to_account_info(),
            prog: ctx.accounts.prog.to_account_info(),
            gram_authority: ctx.accounts.gram_authority.to_account_info(),
            new_authority: ctx.accounts.new_authority.to_account_info(),
        };

        pro_gram::cpi::update_gram_auth(CpiContext::new_with_signer(
            pro_gram,
            cpi_accounts, 
            &[&[b"gram_authority".as_ref(), &[gram_auth_bump]]],
        ))

    }

    pub fn setup_gram(ctx: Context<SetupGram>) -> ProgramResult {
        
        // Not doing the check below because pro-gram does the check

        // let (gram_key, _gram_bump) = Pubkey::find_program_address(&[b"gram".as_ref(), id().as_ref()], pro_gram.key);
        // if gram_key != ctx.accounts.gram.key() {
        //     msg!("Invalid gram account!!");
        //     return Err(ProgramError::InvalidAccountData)
        // }

        let pro_gram = ctx.accounts.pro_gram.to_account_info();

        let (_gram_auth_pda, gram_auth_bump) = Pubkey::find_program_address(&[GRAM_AUTH_SEED], ctx.program_id);

        let cpi_accounts = InitGram {
            gram: ctx.accounts.gram.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            prog: ctx.accounts.prog.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        pro_gram::cpi::init_gram(
            CpiContext::new_with_signer(
                pro_gram,
                cpi_accounts, 
                &[&[b"gram_authority".as_ref(), &[gram_auth_bump]]],
            ),
            "test-prog".to_string(),
            "test-description".to_string(),
            "test-repo".to_string()
        )

    }
}

#[derive(Accounts)]
pub struct SetupGram<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub gram: AccountInfo<'info>,

    #[account(executable)]
    pub prog: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub pro_gram: Program<'info, ProGram>
}

#[derive(Accounts)]
pub struct ChangeAuth<'info> {
    #[account(mut, has_one = gram_authority)]
    pub gram: Account<'info, Gram>,

    #[account(executable)]
    pub prog: AccountInfo<'info>,

    pub new_authority: Signer<'info>,

    #[account(seeds=[b"gram_authority".as_ref()], bump)]
    pub gram_authority: AccountInfo<'info>,

    pub pro_gram: Program<'info, ProGram>
}
