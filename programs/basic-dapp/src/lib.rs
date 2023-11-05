use anchor_lang::prelude::*;

declare_id!("BtbH9LjA9Y6sfiP5bzKEzPpGhX9MyXEjf99zkDnKpPDz");

#[program]
pub mod basic_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
