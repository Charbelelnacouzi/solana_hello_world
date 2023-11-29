use anchor_lang::prelude::*;

declare_id!("vpjKtkApgucL79kUryJKJKdggJ36jj6dLbdas7h61as");

#[program]
pub mod solana_hello_world {
    use super::*;
    #[account]
pub struct Message {
    pub author: Pubkey,
    pub timestamp: i64,
    pub content: String,
}
#[derive(Accounts)]
pub struct CreateMessage<'info> {
		#[account(init, payer = author, space = 1000)]
    pub message: Account<'info, Message>,
		#[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct UpdateMessage<'info> {
		#[account(mut)]
    pub message: Account<'info, Message>,
		#[account(mut)]
    pub author: Signer<'info>,
}

pub fn create_message(ctx: Context<CreateMessage>, content: String) -> Result<()> {
    let message: &mut Account<Message> = &mut ctx.accounts.message;
    let author: &Signer = &ctx.accounts.author;
    let clock: Clock = Clock::get().unwrap();
    
    message.author = *author.key;
    message.timestamp = clock.unix_timestamp;
    message.content = content;
    
    Ok(())
  }
  pub fn update_message(ctx: Context<UpdateMessage>, content: String) -> Result<()> {
    let message: &mut Account<Message> = &mut ctx.accounts.message;
    let author: &Signer = &ctx.accounts.author;
    let clock: Clock = Clock::get().unwrap();
    
    message.author = *author.key;
    message.timestamp = clock.unix_timestamp;
    message.content = content;
    
    Ok(())
  }
}
