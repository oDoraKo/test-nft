use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
    mpl_token_metadata::{
        ID as TOKEN_METADATA_ID,
        instruction as token_instruction,
    },
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mint_nft_test {
    use super::*;

    pub fn mint(
        ctx: Context<MintNft>,
        metadata_title: String, 
        metadata_symbol: String, 
        metadata_uri: String,
    ) -> Result<()> {
        
        msg!("Creating mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.mint.to_account_info(),
                },
            ),
            10000000, // lamport 1 sol
            82, // space
            &ctx.accounts.token_program.key(), //owner
        )?;


        msg!("Initializing mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            0, // decimals
            &ctx.accounts.mint_authority.key(), //authority
            Some(&ctx.accounts.mint_authority.key()), //freeze_authority
        )?;

        msg!("Creating token account...");
        msg!("Mint: {}", &ctx.accounts.token_account.key());
        associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create {
                    payer: ctx.accounts.mint_authority.to_account_info(),
                    associated_token: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
        )?;

        msg!("Minting token to token account...");
        msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key()); 
        msg!("Token Address: {}", &ctx.accounts.token_account.key());

        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1,
        )?;

        msg!("Creating metadata account...");
        msg!("Metadata account address: {}", &ctx.accounts.metadata.to_account_info().key());
        invoke(
            &token_instruction::create_metadata_accounts_v2(
                TOKEN_METADATA_ID, // program_id
                ctx.accounts.metadata.key(), // metadata_account
                ctx.accounts.mint.key(), // mint
                ctx.accounts.mint_authority.key(), // mint_authority
                ctx.accounts.mint_authority.key(), // payer
                ctx.accounts.mint_authority.key(), // update_authority
                metadata_title, // name
                metadata_symbol, // symbol
                metadata_uri, // uri
                None, // creators
                1, // seller_fee_basis_points
                true, // update_authority_is_signer
                false, // is_mutable
                None, // collection
                None, // uses
            ),
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Creating master edition metadata account...");
        msg!("Master edition metadata account address: {}", &ctx.accounts.master_edition.to_account_info().key());
        invoke(
            &token_instruction::create_master_edition_v3(
                TOKEN_METADATA_ID, // program_id
                ctx.accounts.master_edition.key(), // edition
                ctx.accounts.mint.key(), //mint
                ctx.accounts.mint_authority.key(), // update_authority
                ctx.accounts.mint_authority.key(), // mint_authority
                ctx.accounts.metadata.key(), // metadata
                ctx.accounts.mint_authority.key(), // payer
                Some(0), // max_supply
            ),
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Token mint process completed successfully.");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNft<'info> { // add lifetime specifier here anchor use info
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>, // add first account: metadata account, uncheck account means we don't have to try to tell anchor what type of account
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: We're about to create this with anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,

}
