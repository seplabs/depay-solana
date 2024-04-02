use anchor_lang::prelude::*;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;


declare_id!("Do7Ta4RFCwrBhbESGcMg6W7sLNqXMcq5E7YV5CAEZhij");

const SEEDSTATE:&[u8] = b"escrow_state";
const SEEDWALLET:&[u8] = b"escrow_wallet";

#[program]
pub mod wu_pay_solana {

    use super::*;

    pub fn deposite_grant(_ctx: Context<DepositeGrant>, _deposite_idx:u64, _state_bump:u8, _wallet_bump:u8, _amount:u64) -> Result<()> {
        let escrow_state = &mut _ctx.accounts.escrow_state;
        escrow_state.sender = _ctx.accounts.sender.key();
        escrow_state.receiver = _ctx.accounts.receiver.key();
        escrow_state.amount = _amount;
        escrow_state.waller_bump = _wallet_bump;
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

    pub fn complete_grant(_ctx: Context<DepositeGrant>, _deposite_idx:u64, _state_bump:u8, _wallet_bump:u8, _amount:u64) -> Result<()> {
        //let escrow_state = &mut _ctx.accounts.escrow_state;
        let escrow_wallet = &mut _ctx.accounts.escrow_wallet;
        let system_program = &mut _ctx.accounts.system_program;
        let sender = &mut _ctx.accounts.sender;
        let receiver = &mut _ctx.accounts.receiver;
        let _deposite_idx_bytes = _deposite_idx.to_le_bytes();
        let _wallet_bump_bytes = _wallet_bump.to_le_bytes();

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

        anchor_lang::system_program::transfer(cpictx, _amount)?;
        Ok(())
    }

}


#[derive(Accounts)]
#[instruction(deposite_idx:u64, state_bump:u8, wallet_bump:u8)]
pub struct CompleteGrant<'info> {
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
        has_one = sender,
        has_one = receiver,
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
    escrow_wallet: SystemAccount<'info>,

    system_program: Program<'info, System>,
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
        space = 8 + 8 + 8 + 1,
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
        space = 8 + 8 + 1,
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

#[account]
#[derive(Default)]
pub struct EscrowState {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub amount: u64,
    waller_bump: u8,
}

#[account]
#[derive(Default)]
pub struct EscrowWallet {
}


