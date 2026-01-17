use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::types::CallbackAccount;

use crate::error::ErrorCode;

const COMP_DEF_OFFSET: u32 = comp_def_offset("increment_counter");
use crate::state::{Counter, COUNTER_CIPHERTEXT_LENGTH, COUNTER_CIPHERTEXT_OFFSET};
use crate::{SignerAccount, ID, ID_CONST};

pub fn handler(ctx: Context<IncrementCounter>, computation_offset: u64) -> Result<()> {
    ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

    // For Enc<Mxe, &T> by reference:
    // - Pass nonce from account as plaintext (like Enc<Mxe, T> by value)
    // - Pass account reference to just the ciphertext portion
    let nonce = u128::from_le_bytes(ctx.accounts.counter.nonce);
    let args = ArgBuilder::new()
        .plaintext_u128(nonce)
        .account(
            ctx.accounts.counter.key(),
            COUNTER_CIPHERTEXT_OFFSET,
            COUNTER_CIPHERTEXT_LENGTH,
        )
        .build();

    queue_computation(
        ctx.accounts,
        computation_offset,
        args,
        None,
        vec![IncrementCounterCallback::callback_ix(
            computation_offset,
            &ctx.accounts.mxe_account,
            &[CallbackAccount {
                pubkey: ctx.accounts.counter.key(),
                is_writable: true,
            }],
        )?],
        1,
        0,
    )?;

    Ok(())
}

pub fn callback_handler(
    ctx: Context<IncrementCounterCallback>,
    output: SignedComputationOutputs<IncrementCounterOutput>,
) -> Result<()> {
    let o = match output.verify_output(
        &ctx.accounts.cluster_account,
        &ctx.accounts.computation_account,
    ) {
        Ok(IncrementCounterOutput { field_0 }) => field_0,
        Err(_) => return Err(ErrorCode::AbortedComputation.into()),
    };

    let counter = &mut ctx.accounts.counter;
    counter.nonce = o.nonce.to_le_bytes();
    counter.state = o.ciphertexts;

    Ok(())
}

pub fn init_comp_def_handler(ctx: Context<InitIncrementCounterCompDef>) -> Result<()> {
    init_comp_def(ctx.accounts, None, None)?;
    Ok(())
}

#[queue_computation_accounts("increment_counter", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct IncrementCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        space = 9,
        payer = payer,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(
        mut,
        address = derive_mempool_pda!(mxe_account, ErrorCode::ClusterNotSet)
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!(mxe_account, ErrorCode::ClusterNotSet)
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset, mxe_account, ErrorCode::ClusterNotSet)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[callback_accounts("increment_counter")]
#[derive(Accounts)]
pub struct IncrementCounterCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    /// CHECK: computation_account, checked by arcium program via constraints in the callback context.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_cluster_pda!(mxe_account, ErrorCode::ClusterNotSet)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[init_computation_definition_accounts("increment_counter", payer)]
#[derive(Accounts)]
pub struct InitIncrementCounterCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    /// Can't check it here as it's not initialized yet.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}
