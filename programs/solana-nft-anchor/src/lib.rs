use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, MintTo, mint_to},
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata, MetadataAccount
    }
};

use mpl_token_metadata::{ 
    // instructions::{find_master_edition_account, find_metadata_account},
    types::DataV2
};

declare_id!("G4xeu1UsPpAhV9QnamPPftyLuxRVp18uTBp4vFh8ZgcC");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn init_nft(
        ctx: Context<IntNFT>,
        name: String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        //create mint account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo{
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info()
            }
        );
        mint_to(cpi_context, 1)?;

        //create metedata account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info()   
            }
        );
        let data_v2 = DataV2{
            name: name,
            symbol: symbol,
            uri: uri,    
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None
        };
        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None );

        //create master edition account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3{
                edition: ctx.accounts.master_addition_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info()
            }
        );    

        create_master_edition_v3(cpi_context, None)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IntNFT<'info> {
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    #[account(init, 
        payer = signer, 
        mint::decimals = 0, 
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    #[account(
         mut, 
        // address =  find_metadata_account(&mint.key()).0,
    
    )]
    pub metadata_account: AccountInfo<'info>,
    #[account(
        mut,
        // address=find_master_edition_account(&mint.key()).0,
    )]
    pub master_addition_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

