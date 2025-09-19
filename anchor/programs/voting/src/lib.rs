use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod voting {
    use super::*;
    //Instruction to initialize voting/poll
    pub fn initialize_poll(ctx: Context<InitializePoll>,
                              poll_id: u64,
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
    //insturctuin to initialize the candidate
    pub fn initialize_candidate(ctx:Context<InitializeCandidate>,
                                poll_id: u64,
                                candidate: String,
                            ) -> Result<()>{
        ctx.accounts.candidate_account.candidate_name = candidate;
        ctx.accounts.poll_account.vote_count += 1;
        Ok(())
    }

    //instruction for voting
    pub fn vote(ctx: Context<Vote>,
                poll_id: u64,
                candidate_name: String,
            ) -> Result<()>{

      let candidate_account = &mut ctx.accounts.candidate_account;
      let current_time = Clock::get()?.unix_timestamp;

      if current_time <= (ctx.accounts.poll_account.poll_start as i64){
        return Err(VotingError::VotingNotStarted.into());
      };

      if current_time > (ctx.accounts.poll_account.poll_end as i64){
        return Err(VotingError::VotingEnded.into());
      };

      candidate_account.candidate_vote += 1;
      Ok(())
    }
}

//deried account for Initialize poll 
#[derive(Accounts)]
#[instruction(poll_id:u64)]
pub struct InitializePoll<'info>{

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll", poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,

    
}

// derive account for initialize candidate instruction
#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitializeCandidate<'info>{

    #[account(mut)]
    pub user: Signer<'info>,

    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = user,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(),candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>,
}

//accounts for data
#[account]
#[derive(InitSpace)]
pub struct PollAccount{

    #[max_len(50)]
    pub poll_name: String,

    #[max_len(300)]
    pub description: String,

    pub poll_start: u64,

    pub poll_end: u64,

    pub  vote_count: u64,
}

//derive account for vote

#[derive(Accounts)]
#[instruction(poll_id: u64,candidate:String)]
pub struct Vote<'info>{
    
    #[account(mut)]
    pub user: Signer<'info>,

    // pub poll_account: Account<'info,PollAccount>,


    #[account(
        mut,
        seeds = [b"poll",poll_id.to_le_bytes().as_ref()],
        bump
    )] 
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [b"poll",poll_id.to_le_bytes().as_ref(),candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount{

    #[max_len(50)]
    pub candidate_name: String,

    pub candidate_vote: u64,
}

//handling errors
#[error_code]
pub enum VotingError{
    #[msg("Voting has'nt started yet.")]
    VotingNotStarted,

    #[msg("Voting has ended.")]
    VotingEnded,
}