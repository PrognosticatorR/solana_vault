// !Error

use solana_program::{msg, program_error::ProgramError};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum VaultError {
    #[error("Failed to parse the pubkey")]
    PublicKeyParseError,
    #[error("Only Admin can perform this action!")]
    UnAutorizeActionError,
    #[error("Not enough tokens in account")]
    NotEnoughTokens,
    #[error("Cant withdraw before withdraw-time")]
    InvalidTimeError,
    #[error("Receiver does not own enough tokens")]
    WithdrawError,
    #[error("Unlock timestamp is not in seconds!")]
    IncorrectTimeStampError,
    #[error("Unlock time is not in the future!")]
    BadUnlockTimeError,
    #[error("Tokens are still locked!")]
    TokensAreLockedError,
    #[error("Tokens are already withdrawn!")]
    TokensAlreadyWithdrawnError,
    #[error("Admin account invalid")]
    AdminAccountInvalid,
    #[error("Not enough lamports")]
    NotEnoughLamports,
    #[error("Address is not a valid sp20 token")]
    InvalidSPL20Address,
}

impl From<VaultError> for ProgramError {
    fn from(e: VaultError) -> Self {
        msg!("{}", e);
        ProgramError::Custom(e as u32)
    }
}
