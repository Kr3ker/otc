use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;

const COMP_DEF_OFFSET_ADD_TOGETHER: u32 = comp_def_offset("add_together");
const COMP_DEF_OFFSET_INIT_COUNTER: u32 = comp_def_offset("init_counter");
const COMP_DEF_OFFSET_INCREMENT_COUNTER: u32 = comp_def_offset("increment_counter");
const COMP_DEF_OFFSET_GET_COUNTER: u32 = comp_def_offset("get_counter");

// Counter account data layout (after 8-byte discriminator):
// - nonce: [u8; 16] at offset 8
// - state: [[u8; 32]; 1] at offset 24
// For account references, we pass just the ciphertext portion
const COUNTER_CIPHERTEXT_OFFSET: u32 = 24; // Skip discriminator (8) + nonce (16)
const COUNTER_CIPHERTEXT_LENGTH: u32 = 32; // Just the ciphertext, 1 x 32 bytes

declare_id!("DvLY17y6UfZSiNPKnavbW2aJBVWo8N6vbNXgf7ym95mE");

#[arcium_program]
pub mod otc {
    use arcium_client::idl::arcium::types::CallbackAccount;

    use super::*;

    pub fn init_add_together_comp_def(ctx: Context<InitAddTogetherCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, None, None)?;
        Ok(())
    }

    pub fn add_together(
        ctx: Context<AddTogether>,
        computation_offset: u64,
        ciphertext_0: [u8; 32],
        ciphertext_1: [u8; 32],
        pubkey: [u8; 32],
        nonce: u128,
    ) -> Result<()> {
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;
        let args = ArgBuilder::new()
            .x25519_pubkey(pubkey)
            .plaintext_u128(nonce)
            .encrypted_u8(ciphertext_0)
            .encrypted_u8(ciphertext_1)
            .build();

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            None,
            vec![AddTogetherCallback::callback_ix(
                computation_offset,
                &ctx.accounts.mxe_account,
                &[],
            )?],
            1,
            0,
        )?;
        Ok(())
    }

    #[arcium_callback(encrypted_ix = "add_together")]
    pub fn add_together_callback(
        ctx: Context<AddTogetherCallback>,
        output: SignedComputationOutputs<AddTogetherOutput>,
    ) -> Result<()> {
        let o = match output.verify_output(
            &ctx.accounts.cluster_account,
            &ctx.accounts.computation_account,
        ) {
            Ok(AddTogetherOutput { field_0 }) => field_0,
            Err(_) => return Err(ErrorCode::AbortedComputation.into()),
        };

        emit!(SumEvent {
            sum: o.ciphertexts[0],
            nonce: o.nonce.to_le_bytes(),
        });
        Ok(())
    }

    ///////////////////////////
    /// Init Counter
    ///////////////////////////

    pub fn init_init_counter_comp_def(
        ctx: Context<InitInitCounterCompDef>,
    ) -> Result<()> {
        init_comp_def(ctx.accounts, None, None)?;
        Ok(())
    }

    pub fn init_counter(
        ctx: Context<InitCounter>,
        computation_offset: u64,
        nonce: u128,
    ) -> Result<()> {
        // Mxe marker type requires a nonce for the output encryption
        let args = ArgBuilder::new().plaintext_u128(nonce).build();

        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            None,
            vec![InitCounterCallback::callback_ix(
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

    #[arcium_callback(encrypted_ix = "init_counter")]
    pub fn init_counter_callback(
        ctx: Context<InitCounterCallback>,
        output: SignedComputationOutputs<InitCounterOutput>,
    ) -> Result<()> {
        let o = match output.verify_output(
            &ctx.accounts.cluster_account,
            &ctx.accounts.computation_account,
        ) {
            Ok(InitCounterOutput { field_0 }) => field_0,
            Err(_) => return Err(ErrorCode::AbortedComputation.into()),
        };

        let counter = &mut ctx.accounts.counter;
        counter.nonce = o.nonce.to_le_bytes();
        counter.state = o.ciphertexts;

        Ok(())
    }

    ///////////////////////////
    /// Increment Counter
    ///////////////////////////

    pub fn init_increment_counter_comp_def(
        ctx: Context<InitIncrementCounterCompDef>,
    ) -> Result<()> {
        init_comp_def(ctx.accounts, None, None)?;
        Ok(())
    }

    pub fn increment_counter(
        ctx: Context<IncrementCounter>,
        computation_offset: u64,
    ) -> Result<()> {
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

    #[arcium_callback(encrypted_ix = "increment_counter")]
    pub fn increment_counter_callback(
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

    ///////////////////////////
    /// Get Counter (read and re-encrypt for user)
    ///////////////////////////

    pub fn init_get_counter_comp_def(ctx: Context<InitGetCounterCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, None, None)?;
        Ok(())
    }

    /// Queue a get_counter computation to read and re-encrypt the counter for the caller.
    /// The result will be encrypted with a shared secret so the caller can decrypt it.
    pub fn get_counter(
        ctx: Context<GetCounter>,
        computation_offset: u64,
        recipient_pubkey: [u8; 32],
        recipient_nonce: u128,
    ) -> Result<()> {
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        // For Enc<Mxe, &T> by reference:
        // - Pass nonce from account as plaintext
        // - Pass account reference to just the ciphertext portion
        // For Shared recipient marker: pass x25519 public key and nonce for output encryption
        let mxe_nonce = u128::from_le_bytes(ctx.accounts.counter.nonce);
        let args = ArgBuilder::new()
            .plaintext_u128(mxe_nonce)
            .account(
                ctx.accounts.counter.key(),
                COUNTER_CIPHERTEXT_OFFSET,
                COUNTER_CIPHERTEXT_LENGTH,
            )
            .x25519_pubkey(recipient_pubkey)
            .plaintext_u128(recipient_nonce)
            .build();

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            None,
            vec![GetCounterCallback::callback_ix(
                computation_offset,
                &ctx.accounts.mxe_account,
                &[],
            )?],
            1,
            0,
        )?;

        Ok(())
    }

    #[arcium_callback(encrypted_ix = "get_counter")]
    pub fn get_counter_callback(
        ctx: Context<GetCounterCallback>,
        output: SignedComputationOutputs<GetCounterOutput>,
    ) -> Result<()> {
        let o = match output.verify_output(
            &ctx.accounts.cluster_account,
            &ctx.accounts.computation_account,
        ) {
            Ok(GetCounterOutput { field_0 }) => field_0,
            Err(_) => return Err(ErrorCode::AbortedComputation.into()),
        };

        // Emit an event with the re-encrypted counter value
        // The caller can decrypt this with their private key
        emit!(CounterValueEvent {
            encryption_key: o.encryption_key,
            nonce: o.nonce.to_le_bytes(),
            ciphertext: o.ciphertexts[0],
        });

        Ok(())
    }
}

#[queue_computation_accounts("add_together", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct AddTogether<'info> {
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
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_ADD_TOGETHER)
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
}

#[callback_accounts("add_together")]
#[derive(Accounts)]
pub struct AddTogetherCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_ADD_TOGETHER)
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
}

#[init_computation_definition_accounts("add_together", payer)]
#[derive(Accounts)]
pub struct InitAddTogetherCompDef<'info> {
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

////////////////////////
/// Init Counter
///////////////////////

#[init_computation_definition_accounts("init_counter", payer)]
#[derive(Accounts)]
pub struct InitInitCounterCompDef<'info> {
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

#[queue_computation_accounts("init_counter", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct InitCounter<'info> {
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
    /// CHECK: mempool_account, checked by the arcium program
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!(mxe_account, ErrorCode::ClusterNotSet)
    )]
    /// CHECK: executing_pool, checked by the arcium program
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset, mxe_account, ErrorCode::ClusterNotSet)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_INIT_COUNTER)
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
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS,
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(init,
        payer = payer,
        space = 8 + Counter::INIT_SPACE,
        seeds = [b"counter", payer.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,
}

#[callback_accounts("init_counter")]
#[derive(Accounts)]
pub struct InitCounterCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_INIT_COUNTER)
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


////////////////////////
/// Increment Counter
///////////////////////

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
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_INCREMENT_COUNTER)
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
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_INCREMENT_COUNTER)
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

/// Counter account stores MXE-encrypted state.
/// Layout matches MXEEncryptedStruct: nonce first, then ciphertexts.
#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub nonce: [u8; 16],
    pub state: [[u8; 32]; 1],
}

#[event]
pub struct SumEvent {
    pub sum: [u8; 32],
    pub nonce: [u8; 16],
}

#[event]
pub struct CounterValueEvent {
    pub encryption_key: [u8; 32],
    pub nonce: [u8; 16],
    pub ciphertext: [u8; 32],
}

#[error_code]
pub enum ErrorCode {
    #[msg("The computation was aborted")]
    AbortedComputation,
    #[msg("Cluster not set")]
    ClusterNotSet,
}

////////////////////////
/// Get Counter
///////////////////////

#[init_computation_definition_accounts("get_counter", payer)]
#[derive(Accounts)]
pub struct InitGetCounterCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[queue_computation_accounts("get_counter", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct GetCounter<'info> {
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
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_GET_COUNTER)
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
    /// The counter account to read from
    pub counter: Account<'info, Counter>,
}

#[callback_accounts("get_counter")]
#[derive(Accounts)]
pub struct GetCounterCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_GET_COUNTER)
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
}
