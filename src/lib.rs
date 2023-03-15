use anchor_lang::prelude::*;
const DELAY : i64 = 120;

declare_id!("FZU8B3uZTcUibKhEqiJzmDNCvh5gYPioy3HA5HvKEyTo");

#[program] 
mod ecoflip_voting {
    use super::*;
    pub fn create_vote(ctx: Context<CreateVote>, votes: u64, project: Pubkey) -> Result<()> {
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        //using pointers instead of referencing through ctx for better code readability
        let current_distro_account = &mut ctx.accounts.current_distro_account;
        let distro_account = &mut ctx.accounts.distro_account;
        let vote_account = &mut ctx.accounts.vote_account;
        
      if (current_distro_account.key() != distro_account.current_distribution){
        return err!(PermissionError::NotPermitted);
      }

      if(current_timestamp > distro_account.timestamp + DELAY){    
        return err!(DistributionEndedError::DistributionEnded);
      }
        
      //todo podmienka aby distribucia bola 
      //todo podmienka aby distribucia bola otvorena. unix timestamp < distribution_account.timestamp + delay
      //todo podmienka projekt sa nachadza medzi approved projects.
        vote_account.votes = votes;
        
        vote_account.project = project;
        
        vote_account.timestamp = current_timestamp;
        
        vote_account.distribution = distro_account.key();
        
        distro_account.total_votes += votes;
        
        msg!("New vote for project: {} with {} votes with timestamp: {}", project, votes, current_timestamp); // Message will show up in the tx logs
        Ok(())
    
    }
    
    
    pub fn release_funds(ctx: Context<ReleaseFunds>) -> Result<()>{
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;
        
        //using pointers instead of referencing through ctx for better code readability
        let distro_account = &mut ctx.accounts.distro_account;
        let vote_account = &mut ctx.accounts.vote_account;
        
        if (vote_account.distribution != distro_account.key()){
        return err!(PermissionError::NotPermitted);
        }
        if(vote_account.funds_released == true){
            return err!(FundsReleasedError::FundsAlreadyReleased);
        }
        if (distro_account.timestamp + DELAY >= current_timestamp){ //funds can only be released after the set time period has passed
            return err!(DistributionEndedError::DistributionEnded);
        }
      
        vote_account.funds_released = true;
      
        let vote_account_votes = vote_account.votes as f64;
      
        let distro_account_votes = distro_account.total_votes as f64;
      
        let lamports = distro_account.treasury_lamports as f64;
      
        let amount = vote_account_votes / distro_account_votes * lamports;
      
        msg!("Sending {} lamports to project: {}", amount, ctx.accounts.vote_account.project);
    
    //kolko votov je v d
      
        Ok(())
    }
  
    pub fn new_distribution(ctx: Context<NewDistribution>) -> Result<()> {
        
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        //using pointers instead of referencing through ctx for better code readability
        let distro_account = &mut ctx.accounts.distro_account;
        let current_distro_account = &mut ctx.accounts.current_distro_account; 
        

        
        if current_distro_account.last_timestamp != 0 {
            if current_distro_account.last_timestamp + DELAY > current_timestamp { //you can't create a distribution account if there is an already ongoing distribution
                return err!(DistributionCreationError::DistributionExists);
            }
            
            current_distro_account.last_timestamp += DELAY;
            
            distro_account.timestamp = current_distro_account.last_timestamp;
            
            current_distro_account.epoch += 1; 
            
            distro_account.epoch = current_distro_account.epoch;
            
            distro_account.treasury_lamports = 1000;
            
        } else {
            distro_account.timestamp = current_timestamp;
            
            distro_account.epoch = 1;
            
            current_distro_account.last_timestamp = current_timestamp;
            
            current_distro_account.epoch = 1; 
            
            distro_account.treasury_lamports = 1000;
        }
        distro_account.current_distribution = current_distro_account.key();
        Ok(())
    }
  
    pub fn create_current_distribution(ctx: Context<CreateCurrentDistribution>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateVote<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    //todo space
    #[account(init, payer = signer, space = 8 + 8 + 32 + 8 + 1 + 32)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub distro_account: Account<'info, DistributionAccount>,
    #[account(mut)]
    pub current_distro_account: Account<'info, CurrentDistributionAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct NewDistribution<'info> {
    // todo space
    #[account(init, payer = signer, space = 8 + 8 + 8 + 8 + 8 + 32 + 8)]
    pub distro_account: Account<'info, DistributionAccount>,
    #[account(mut)]
    pub current_distro_account: Account<'info, CurrentDistributionAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateCurrentDistribution<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    // todo space
    #[account(init, payer = signer, space = 8 + 8 + 8)]
    pub current_distro_account: Account<'info, CurrentDistributionAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseFunds<'info> {
     //We must specify the space in order to initialize an account.
     //First 8 bytes are default account discriminator,
     //next 8 bytes come from NewAccount.data being type u64.
     //(u64 = 64 bits unsigned integer = 8 bytes)
     //todo space
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
     //mutable?
    #[account(mut)]
    pub distro_account: Account<'info, DistributionAccount>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct VoteAccount {
    votes: u64,
    project: Pubkey,
    timestamp: i64,
    funds_released: bool,
    distribution: Pubkey
}

#[account]
pub struct DistributionAccount {
  timestamp: i64, 
  total_votes: u64,
  epoch: u64,
  treasury_lamports: u64,
  current_distribution: Pubkey,
}

#[account]
pub struct CurrentDistributionAccount {
    last_timestamp: i64,
    epoch: u64,
}


/*__________________________________________
                ERRORS
------------------------------------------*/


//todo
#[error_code]
pub enum PlaceholderError {
    #[msg("Congratulations! You've successfully encoutered a dummy error! Now while that's amazing, please contact the devs immediatelly after reading this message")]
    PlaceHolder
}


#[error_code]
pub enum PermissionError {
    #[msg("You don't have permission to perform this action")]
    NotPermitted
}


#[error_code]
pub enum DistributionEndedError {
    #[msg("The distribution you tried to vote in has ended")]
    DistributionEnded
}


#[error_code]
pub enum FundsReleasedError {
    #[msg("The funds have already been released")]
    FundsAlreadyReleased
}


#[error_code]
pub enum DistributionCreationError {
    #[msg("There is already an ongoing distribution")]
    DistributionExists
}


