use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constants::*, states::*,errors::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [TRESURE_SEED],
        bump,
        space = 8 + size_of::<Treasure>() + 300
    )]
    pub treasure: Box<Account<'info, Treasure>>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn validate(
        ctx: Context<Self>,
        supply:u64,
        name:String,
        uri:String,
        symbol:String,
    ) -> Result<()> {
        let accts: &mut Initialize<'_> = ctx.accounts;
        let admin_key: Pubkey = accts.admin.key();
        if accts.treasure.admin.key() != Pubkey::default() && accts.treasure.admin.key() != admin_key{
            return Err(CustomError::Unauthorized.into());
        }
        accts.treasure.admin = admin_key;
        accts.treasure.admin = admin_key;
        accts.treasure.supply = supply;
        accts.treasure.name = name;
        accts.treasure.uri = uri;
        accts.treasure.symbol = symbol;
        Ok(())
    }
}
