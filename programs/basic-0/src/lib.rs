use anchor_lang::prelude::*;

// Define the program's instruction handlers.


//This is a slighly modified example from the tutorial you can find in the anchor github.
//Idea is to replicate this tuto but with anchor
//https://smith-mcf.medium.com/a-simple-solana-dapp-tutorial-6dedbdf65444
//Really doing this as an exercise, the weighting for the votes are not impemented
#[program]
mod ftr_voter {
    use super::*;


    pub fn create(ctx: Context<Create>, authority: Pubkey) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.authority = authority;
        counter.count = 0;
        Ok(())
    }

    pub fn vote_yes(ctx: Context<vote_yesr>) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count = 1;
        Ok(())
    }

    pub fn vote_no(ctx: Context<vote_nor>) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count = 2;
        Ok(())
    }
}

// Define the validated accounts for each handler.

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init)]
    pub counter: ProgramAccount<'info, Counter>,
}

#[derive(Accounts)]
pub struct vote_yesr<'info> {
    #[account(mut, has_one = authority)]
    pub counter: ProgramAccount<'info, Counter>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct vote_nor<'info> {
    #[account(mut, has_one = authority)]
    pub counter: ProgramAccount<'info, Counter>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

// Define the program owned accounts.

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
