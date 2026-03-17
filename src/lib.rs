use anchor_lang::prelude::*;

declare_id!("2wufVp4QcwKpdG9xv2EqaxFYwBzBmTdke8mqVYKRUAv7");

#[program]
pub mod mancer_eggplan {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Eggplan terdeploy di markas lokal Sora!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}