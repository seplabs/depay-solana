use anchor_lang::prelude::*;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;


declare_id!("Do7Ta4RFCwrBhbESGcMg6W7sLNqXMcq5E7YV5CAEZhij");

const SEEDSTATE:&[u8] = b"escrow_state";
const SEEDWALLET:&[u8] = b"escrow_wallet";

#[error_code]
pub enum ErrorCode {
    #[msg("The amount to withdrwa does not match the amount in the escrow state")]
    InvalidAmount,
    #[msg("The bump seed for the wallet does not match the escrow state's wallet bump seed")]
    InvalidWalletBump,
}

#[program]
pub mod wu_pay_solana {

    use super::*;

    pub fn deposite_grant(_ctx: Context<DepositeGrant>, _deposite_idx:u64, _state_bump:u8, _wallet_bump:u8, _amount:u64) -> Result<()> {
        let escrow_state = &mut _ctx.accounts.escrow_state;
        escrow_state.sender = _ctx.accounts.sender.key();
        escrow_state.receiver = _ctx.accounts.receiver.key();
        escrow_state.amount = _amount;
        escrow_state.wallet_bump = _wallet_bump;
        let system_program = &mut _ctx.accounts.system_program;

        let transfer_instruction = anchor_lang::system_program::Transfer {
            from: _ctx.accounts.sender.to_account_info(),
            to: _ctx.accounts.escrow_wallet.to_account_info(),
        };
        
        let cpictx = CpiContext::new(
            system_program.to_account_info(),
            transfer_instruction);
        
        anchor_lang::system_program::transfer(cpictx, _amount)?;
        Ok(())
    }

    pub fn complete_grant(_ctx: Context<CompleteGrant>, _deposite_idx:u64, _state_bump:u8, _wallet_bump:u8, _amount:u64) -> Result<()> {
        let escrow_state = &mut _ctx.accounts.escrow_state;
        let escrow_wallet = &mut _ctx.accounts.escrow_wallet;
        let system_program = &mut _ctx.accounts.system_program;
        let sender = &_ctx.accounts.sender;
        let receiver = &mut _ctx.accounts.receiver;
        let _deposite_idx_bytes = _deposite_idx.to_le_bytes();
        let _wallet_bump_bytes = _wallet_bump.to_le_bytes();

        if escrow_state.amount != _amount {
            return Err(ErrorCode::InvalidAmount.into());
        }

        if escrow_state.wallet_bump != _wallet_bump {
            return Err(ErrorCode::InvalidWalletBump.into());
        }

        msg!("Transfering amount to receiver...");
        **escrow_wallet.to_account_info().try_borrow_mut_lamports()? -= _amount;
        **receiver.to_account_info().try_borrow_mut_lamports()? += _amount;

        /* 
        msg!("Creating seeds for escrow wallet and state...");
        let inner = vec![
            SEEDWALLET.as_ref(),
            sender.key.as_ref(),
            receiver.key.as_ref(),
            &_deposite_idx_bytes.as_ref(),
            &_wallet_bump_bytes.as_ref(),
        ];
        let outer = vec![inner.as_slice()];


        let transfer_instruction = anchor_lang::system_program::Transfer {
            from: escrow_wallet.to_account_info(),
            to: receiver.to_account_info(),
        };

        let cpictx = CpiContext::new_with_signer(
            system_program.to_account_info(),
            transfer_instruction,
            outer.as_slice(),
        );
        msg!("Transferring amount to receiver...");
        anchor_lang::system_program::transfer(cpictx, _amount)?;
        */
        Ok(())
    }

    pub fn withdraw_grant(_ctx: Context<WithdrwaGrant>, _deposite_idx:u64, _state_bump:u8, _wallet_bump:u8, _amount:u64) -> Result<()> {
        let escrow_state = &mut _ctx.accounts.escrow_state;
        let escrow_wallet = &mut _ctx.accounts.escrow_wallet;
        let system_program = &mut _ctx.accounts.system_program;
        let sender = &_ctx.accounts.sender;
        let receiver = &mut _ctx.accounts.receiver;
        let _deposite_idx_bytes = _deposite_idx.to_le_bytes();
        let _wallet_bump_bytes = _wallet_bump.to_le_bytes();

        if escrow_state.amount != _amount {
            return Err(ErrorCode::InvalidAmount.into());
        }

        if escrow_state.wallet_bump != _wallet_bump {
            return Err(ErrorCode::InvalidWalletBump.into());
        }

        msg!("Withdraw amount to sender...");
        **escrow_wallet.to_account_info().try_borrow_mut_lamports()? -= _amount;
        **sender.to_account_info().try_borrow_mut_lamports()? += _amount;
        Ok(())
    }


    pub fn close_escrow(_ctx: Context<CloseEscrow>, _deposite_idx:u64, _state_bump:u8, _wallet_bump:u8) -> Result<()> {
        Ok(())
    }

}


#[derive(Accounts)]
#[instruction(deposite_idx:u64, state_bump:u8, wallet_bump:u8)]
pub struct DepositeGrant<'info> {
    /// CHECK: this is safe, will not write to this account
    #[account(mut)]
    sender: Signer<'info>,

    /// CHECK: this is safe, will not write to this account
    receiver: AccountInfo<'info>, 

    #[account(
        init,
        payer = sender,
        space = EscrowState::LEN,
        seeds = [
            SEEDSTATE,
            sender.key.as_ref(),
            receiver.key.as_ref(),
            deposite_idx.to_le_bytes().as_ref(),
        ],
        bump 
    )]
    escrow_state: Account<'info, EscrowState>,

    #[account(
        init,
        payer = sender,
        space = EscrowWallet::LEN,
        seeds = [
            SEEDWALLET,
            sender.key.as_ref(),
            receiver.key.as_ref(),
            deposite_idx.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    escrow_wallet: Account<'info,EscrowWallet>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(deposite_idx:u64, state_bump:u8, wallet_bump:u8)]
pub struct CompleteGrant<'info> {
    /// CHECK: this is safe, will not write to this account
    sender: Signer<'info>,
    
    /// CHECK: this is safe, will not write to this account
    #[account(mut)]
    receiver: AccountInfo<'info>, 

    #[account(
        mut,
        seeds = [
            SEEDSTATE,
            sender.key.as_ref(),
            receiver.key.as_ref(),
            deposite_idx.to_le_bytes().as_ref(),
        ],
        bump = state_bump,
    )]
    escrow_state: Account<'info, EscrowState>,

    #[account(
        mut,
        seeds = [
            SEEDWALLET,
            sender.key.as_ref(),
            receiver.key.as_ref(),
            deposite_idx.to_le_bytes().as_ref(),
        ],
        bump = wallet_bump,
    )]
    escrow_wallet: Account<'info,EscrowWallet>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(deposite_idx:u64, state_bump:u8, wallet_bump:u8)]
pub struct WithdrwaGrant<'info> {
    /// CHECK: this is safe, will not write to this account
    #[account(mut)]
    sender: Signer<'info>,
    
    /// CHECK: this is safe, will not write to this account
    receiver: AccountInfo<'info>, 

    #[account(
        mut,
        seeds = [
            SEEDSTATE,
            sender.key.as_ref(),
            receiver.key.as_ref(),
            deposite_idx.to_le_bytes().as_ref(),
        ],
        bump = state_bump,
    )]
    escrow_state: Account<'info, EscrowState>,

    #[account(
        mut,
        seeds = [
            SEEDWALLET,
            sender.key.as_ref(),
            receiver.key.as_ref(),
            deposite_idx.to_le_bytes().as_ref(),
        ],
        bump = wallet_bump,
    )]
    escrow_wallet: Account<'info,EscrowWallet>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(deposite_idx:u64, state_bump:u8, wallet_bump:u8)]
pub struct CloseEscrow<'info> {
    /// CHECK: this is safe, will not write to this account
    #[account(mut)]
    sender: Signer<'info>,
    
    /// CHECK: this is safe, will not write to this account
    receiver: AccountInfo<'info>, 

    #[account(
        mut,
        seeds = [
            SEEDSTATE,
            sender.key.as_ref(),
            receiver.key.as_ref(),
            deposite_idx.to_le_bytes().as_ref(),
        ],
        close=sender,
        bump = state_bump,
    )]
    escrow_state: Account<'info, EscrowState>,

    #[account(
        mut,
        seeds = [
            SEEDWALLET,
            sender.key.as_ref(),
            receiver.key.as_ref(),
            deposite_idx.to_le_bytes().as_ref(),
        ],
        close=sender,
        bump = wallet_bump,
    )]
    escrow_wallet: Account<'info,EscrowWallet>,
    system_program: Program<'info, System>,
}


#[account]
#[derive(Default)]
pub struct EscrowState {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub amount: u64,
    wallet_bump: u8,
}

impl EscrowState {
    pub const LEN:usize = 8 + 32 + 32 + 8 + 1;
}

#[account]
#[derive(Default)]
pub struct EscrowWallet {
}

impl EscrowWallet {
    pub const LEN:usize = 8;
}

