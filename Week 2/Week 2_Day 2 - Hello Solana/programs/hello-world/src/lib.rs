use anchor_lang::prelude::*;

declare_id!("CuzcS4f582LJLDgxyzoy1yxH2Qt7cU8uTP64AsfAD5q2");

#[program]
pub mod hello_world {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

