// !Instruction types
use crate::state::{CreateVaultInput, TransactionInput};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

/// Instructions supported by the vault program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum VaultInstructions {
    /// Create a vault with a escrow account created and funded by sender
    /// account should have a total_lamport=admin_cut+program_rent_account+amount_to_send.
    ///
    /// Accounts expected:
    ///
    /// `[writable]` escrow account, it will hold all necessary info about the trade.
    /// `[signer]` sender account
    /// `[]` receiver account
    /// `[]` Admin account
    CreateVault(CreateVaultInput),

    /// Withdraw from a vault
    ///
    /// Accounts expected:
    ///
    /// `[writable]` escrow account, it will hold all necessary info about the trade.
    /// `[signer]` receiver account
    WithdrawFromVault(TransactionInput),

    /// Deposite to a vault
    ///
    /// Accounts expected:
    ///
    /// `[writable]` escrow account, it will hold all necessary info about the trade.
    /// `[signer]` receiver account
    DepositeToVault(TransactionInput),

    /// Close a stream and transfer tokens between sender and receiver.
    ///
    /// Accounts expected:
    ///
    /// `[writable]` escrow account, it will hold all necessary info about the trade.
    /// `[signer]` sender account
    /// `[]` receiver account
    CloseVault,
}

impl VaultInstructions {
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let (tag, data) = instruction_data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        match tag {
            1 => Ok(VaultInstructions::CreateVault(
                CreateVaultInput::try_from_slice(data)?,
            )),
            2 => Ok(VaultInstructions::WithdrawFromVault(
                TransactionInput::try_from_slice(data)?,
            )),
            3 => Ok(VaultInstructions::DepositeToVault(
                TransactionInput::try_from_slice(data)?,
            )),
            4 => Ok(VaultInstructions::CloseVault),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
