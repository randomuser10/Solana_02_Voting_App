use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>,
                           name: String,
                           description: String,
                           start_time: u64,
                           end_time: u64,
                        ) -> Result<()>{
        
        ctx.accounts.poll_account.poll_name = name;
        ctx.accounts.poll_account.description = description;
        ctx.accounts.poll_account.poll_start = start_time;
        ctx.accounts.poll_account.poll_end = end_time;
        Ok(())


    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info>{
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll",poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount{
    
    #[max_len(50)]
    pub poll_name: String,

    #[max_len(300)]
    pub description: String,

    pub poll_start: u64,

    pub poll_end: u64,
}