use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token}, associated_token::AssociatedToken};
use anchor_spl::token::{MintTo ,mint_to};
use anchor_spl::metadata::{
    Metadata,
    mpl_token_metadata::types::{Creator,DataV2,Collection,CollectionDetails},
    create_master_edition_v3,approve_collection_authority, create_metadata_accounts_v3,update_metadata_accounts_v2,
    CreateMasterEditionV3,CreateMetadataAccountsV3,UpdateMetadataAccountsV2,ApproveCollectionAuthority
};

use crate::{constants::*, states::*,errors::*};

pub fn mint_collection(
    ctx: Context<MintCollection>,
    _bump:u8
) -> Result<()> {
    let accts: &mut MintCollection<'_> = ctx.accounts;
    let binding: [u8; 1] = _bump.to_le_bytes();
    let authority: &mut Box<Account<'_, Treasure>> = &mut accts.treasure;

    let seeds: &[&[&[u8]]] = &[&[
        TRESURE_SEED,
        binding.as_ref()
    ]];
    if authority.admin != accts.admin.to_account_info().key() {
        return Err(CustomError::Unauthorized.into());
    }
    let mut _collection_details: Option<CollectionDetails> = None;

    let cpi_ctx = CpiContext::new_with_signer(
        accts.token_program.to_account_info(), 
            MintTo {
                mint: accts.mint.to_account_info(),
                to: accts.token_account.to_account_info(),
                authority: authority.to_account_info(),
            },
        seeds
    );
    //msg!("CPI Context Assigned");
    mint_to(cpi_ctx, 1)?;
    let createmeta = CpiContext::new_with_signer(
        accts.metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            payer: accts.admin.to_account_info(),
            mint: accts.mint.to_account_info(),
            metadata: accts.nft_metadata.to_account_info(),
            mint_authority: authority.to_account_info(),
            update_authority: authority.to_account_info(),
            system_program: accts.system_program.to_account_info(),
            rent: accts.rent.to_account_info(),
        }, 
        seeds
    );
    //msg!("Account Info Assigned");
    let _ = create_metadata_accounts_v3(
        createmeta,
        DataV2{
            name: format!("Genesis Seternia NFT"),
            uri: format!("https://aquamarine-elaborate-cuckoo-625.mypinata.cloud/ipfs/Qmey3dBM5tnJfGUoAuLDThYKvkbmttbUZR6zgzRzGnvfuD"),
            symbol: format!("STR"),
            seller_fee_basis_points:0,
            creators:None,
            collection:None,
            uses:None
        },
        true,
        true,
        _collection_details
    )?;
    //msg!("{:?} Metadata Account Created !!!",meta);
    let createmaster = CpiContext::new_with_signer(
        accts.metadata_program.to_account_info(),
        CreateMasterEditionV3 {
            edition: accts.master_edition_account.to_account_info(),
            payer: accts.admin.to_account_info(),
            mint: accts.mint.to_account_info(),
            metadata: accts.nft_metadata.to_account_info(),
            mint_authority: authority.to_account_info(),
            update_authority: authority.to_account_info(),
            system_program: accts.system_program.to_account_info(),
            token_program: accts.token_program.to_account_info(),
            rent: accts.rent.to_account_info(),
        }, 
        seeds
    );
    let _ = create_master_edition_v3(createmaster,Some(0))?;

    let cpi_apv = CpiContext::new_with_signer(
        accts.metadata_program.to_account_info(),
        ApproveCollectionAuthority{
            collection_authority_record:accts.delegate.to_account_info(),
            metadata:accts.nft_metadata.to_account_info(),
            payer: accts.admin.to_account_info(),
            mint: accts.mint.to_account_info(),
            update_authority: authority.to_account_info(),
            new_collection_authority:authority.to_account_info()
        }
        ,seeds
    );
    let _= approve_collection_authority(cpi_apv)?;
    Ok(())
}
pub fn mint(
    ctx: Context<MintNFT>,
    _bump:u8
) -> Result<()> {
    let accts: &mut MintNFT<'_> = ctx.accounts;
    let binding: [u8; 1] = _bump.to_le_bytes();

    let seeds: &[&[&[u8]]] = &[&[
        TRESURE_SEED,
        binding.as_ref()
    ]];

    accts.treasure.mints += 1;

    let authority: &mut Box<Account<'_, Treasure>> = &mut accts.treasure;
    let id: u64 = authority.mints;
    let name: String = authority.name.clone();
    let uri: String = authority.uri.clone();
    let symbol: String = authority.symbol.clone();
    let supply: u64 = authority.supply;

    if id > supply {
        return Err(CustomError::IncorrectSupply.into());
    }

    let mut _collection_details: Option<CollectionDetails> = None;
    let mint_ctx = CpiContext::new_with_signer(
        accts.token_program.to_account_info(), 
            MintTo {
                mint: accts.mint.to_account_info(),
                to: accts.token_account.to_account_info(),
                authority: authority.to_account_info(),
            },
        seeds
    );
    mint_to(mint_ctx, 1)?;
    let createmeta = CpiContext::new_with_signer(
        accts.metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            payer: accts.payer.to_account_info(),
            mint: accts.mint.to_account_info(),
            metadata: accts.nft_metadata.to_account_info(),
            mint_authority: authority.to_account_info(),
            update_authority: authority.to_account_info(),
            system_program: accts.system_program.to_account_info(),
            rent: accts.rent.to_account_info(),
        }, 
        seeds
    );
    //msg!("Account Info Assigned");
    let _ = create_metadata_accounts_v3(
        createmeta,
        DataV2{
            name: format!("{name} #{id}"),
            uri: uri,
            symbol: symbol,
            seller_fee_basis_points:500,
            creators:Some(vec![
                Creator{
                    address: authority.to_account_info().key(),
                    verified:true,
                    share: 100
                }
            ]),
            collection:Some(Collection{
                verified:false,
                key: accts.collection_mint.key()
            }),
            uses:None
        },
        true,
        true,
        None
    )?;
    //msg!("{:?} Metadata Account Created !!!",meta);
    let createmaster = CpiContext::new_with_signer(
        accts.metadata_program.to_account_info(),
        CreateMasterEditionV3 {
            edition: accts.master_edition_account.to_account_info(),
            payer: accts.payer.to_account_info(),
            mint: accts.mint.to_account_info(),
            metadata: accts.nft_metadata.to_account_info(),
            mint_authority: authority.to_account_info(),
            update_authority: authority.to_account_info(),
            system_program: accts.system_program.to_account_info(),
            token_program: accts.token_program.to_account_info(),
            rent: accts.rent.to_account_info(),
        }, 
        seeds
    );
    let _ = create_master_edition_v3(createmaster,Some(0))?;
    let cpi_up = CpiContext::new_with_signer(
        accts.metadata_program.to_account_info(),
        UpdateMetadataAccountsV2{
            metadata:accts.nft_metadata.to_account_info(),
            update_authority: authority.to_account_info()
        }
        ,seeds
    );
    let _= update_metadata_accounts_v2(cpi_up,None,None,Some(true),None)?;

    let cpi_ver = CpiContext::new_with_signer(
        accts.metadata_program.to_account_info(),
        NewVerifyCollection{
            collection_authority_record:accts.delegate.to_account_info(),
            payer: accts.payer.to_account_info(),
            metadata:accts.nft_metadata.to_account_info(),
            collection_authority:authority.to_account_info(),
            collection_mint:accts.collection_mint.to_account_info(),
            collection_metadata:accts.collection_metadata.to_account_info(),
            collection_master_edition:accts.collection_master_edition.to_account_info()
        }  
        ,seeds
    );
    let _= corm(cpi_ver)?;
    Ok(())
}
#[derive(Accounts)]
pub struct MintCollection<'info> {

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [TRESURE_SEED],
        bump,
    )]
    pub treasure: Box<Account<'info, Treasure>>,
    #[account( 
        init,
        payer = admin, 
        mint::decimals = 0,
        mint::authority = treasure,
        mint::freeze_authority = treasure,
        seeds = [COLLECTION], 
        bump,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = mint,
        associated_token::authority = admin,
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [
            METADATA,
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            EDITION.as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub master_edition_account: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            METADATA,
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub nft_metadata: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            METADATA,
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            COLLECTION_AUTHORITY,
            treasure.key().as_ref()
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub delegate: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}
#[derive(Accounts)]
pub struct MintNFT<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub treasure: Box<Account<'info, Treasure>>,

    #[account( 
        init,
        payer = payer, 
        mint::decimals = 0,
        mint::authority = treasure,
        mint::freeze_authority = treasure,
        seeds = [MINT,payer.key().as_ref()], 
        bump,
    )]
    pub mint: Box<Account<'info, Mint>>,
    /// CHECK:
    pub collection_mint: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            METADATA,
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            EDITION.as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub master_edition_account: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_master_edition: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            METADATA,
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub nft_metadata: UncheckedAccount<'info>,
    /// CHECK:
    pub collection_metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            METADATA,
            metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            COLLECTION_AUTHORITY,
            treasure.key().as_ref()
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub delegate: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}
pub fn corm<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, NewVerifyCollection<'info>>,
) -> Result<()> {
    let ix = anchor_spl::metadata::mpl_token_metadata::instructions::VerifyCollection {
        collection_authority_record:Some(*ctx.accounts.collection_authority_record.key),
        collection: *ctx.accounts.collection_metadata.key,
        collection_authority: *ctx.accounts.collection_authority.key,
        collection_master_edition_account: *ctx.accounts.collection_master_edition.key,
        collection_mint: *ctx.accounts.collection_mint.key,
        metadata: *ctx.accounts.metadata.key,
        payer: *ctx.accounts.payer.key
    }
    .instruction();
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}
#[derive(Accounts)]
pub struct NewVerifyCollection<'info> {
    /// CHECK:
    pub collection_authority_record: AccountInfo<'info>,
    /// CHECK:
    pub payer: AccountInfo<'info>,
    /// CHECK:
    pub metadata: AccountInfo<'info>,
    /// CHECK:
    pub collection_authority: AccountInfo<'info>,
    /// CHECK:
    pub collection_mint: AccountInfo<'info>,
    /// CHECK:
    pub collection_metadata: AccountInfo<'info>,
    /// CHECK:
    pub collection_master_edition: AccountInfo<'info>,
}