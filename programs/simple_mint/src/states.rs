use anchor_lang::prelude::*;
use std::ops::Deref;

#[account]
pub struct Treasure {
    pub admin: Pubkey,
    pub name:String,
    pub uri:String,
    pub symbol:String,
    pub supply: u64,
    pub mints: u64
}
#[derive(Clone, Debug, PartialEq)]
pub struct CollectionAuthorityRecordAccount(anchor_spl::metadata::mpl_token_metadata::accounts::CollectionAuthorityRecord);

impl anchor_lang::AccountDeserialize for CollectionAuthorityRecordAccount {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let me = Self::try_deserialize_unchecked(buf)?;
        if me.key != anchor_spl::metadata::mpl_token_metadata::types::Key::CollectionAuthorityRecord {
            return Err(ErrorCode::AccountNotInitialized.into());
        }
        Ok(me)
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let result = anchor_spl::metadata::mpl_token_metadata::accounts::CollectionAuthorityRecord::safe_deserialize(buf)?;
        Ok(Self(result))
    }
}

impl Deref for CollectionAuthorityRecordAccount {
    type Target = anchor_spl::metadata::mpl_token_metadata::accounts::CollectionAuthorityRecord;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl anchor_lang::AccountSerialize for CollectionAuthorityRecordAccount {}

impl anchor_lang::Owner for CollectionAuthorityRecordAccount {
    fn owner() -> Pubkey {
        anchor_spl::metadata::mpl_token_metadata::ID
    }
}