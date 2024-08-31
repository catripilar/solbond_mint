use anchor_lang::prelude::*;

/// instructions
pub mod instructions;
pub mod constants;
pub mod errors;
pub mod states;
use crate::instructions::*;

declare_id!("4jNNoXXRZA8ctduQWkea3TPxN2TLYCfJNxyjEPs5Bu7W");

#[program]
pub mod simple_mint {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        supply:u64,
        name:String,
        uri:String,
        symbol:String,
    ) -> Result<()>{
        Initialize::validate(ctx,supply,name,uri,symbol)?;
        Ok(())
    }

    pub fn mint_collection(
        ctx: Context<MintCollection>,
        _bump:u8
    ) -> Result<()>{
        create_collection::mint_collection(ctx,_bump)?;
        Ok(())
    }
    pub fn mint(
        ctx: Context<MintNFT>,
        _bump:u8
    ) -> Result<()>{
    create_collection::mint(ctx,_bump)?;
    Ok(())
}
}