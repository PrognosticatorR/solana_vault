// !Program Processor
use crate::{
    error::VaultError,
    instruction::VaultInstructions,
    state::{CreateVaultInput, TransactionInput, VaultData},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::{check_program_account, instruction::transfer};
use std::str::FromStr;
pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = VaultInstructions::unpack(instruction_data)?;
        match instruction {
            VaultInstructions::CreateVault(data) => {
                Self::process_create_vault(program_id, accounts, data)
            }
            VaultInstructions::WithdrawFromVault(data) => {
                Self::process_withdraw_from_vault(program_id, accounts, data)
            }
            VaultInstructions::DepositeToVault(data) => {
                Self::process_deposite_to_vault(program_id, accounts, data)
            }
            VaultInstructions::CloseVault => Self::process_close_vault(program_id, accounts),
        }
    }
    fn process_create_vault(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: CreateVaultInput,
    ) -> ProgramResult {
        let admin_pub_key = match Pubkey::from_str("HTphKVwkEABXkfHjGyqnMLiTN2CYQj199s4YDB3XJxtd") {
            Ok(key) => key,
            Err(_) => return Err(VaultError::PublicKeyParseError.into()),
        };
        let account_info_iter = &mut accounts.iter();
        let escrow_account = next_account_info(account_info_iter)?;
        let sender_account = next_account_info(account_info_iter)?;
        let admin_account = next_account_info(account_info_iter)?;

        if *admin_account.key != admin_pub_key {
            return Err(VaultError::AdminAccountInvalid.into());
        }

        // 0.01 sol token admin account fee
        // 10000000 Lamports = 0.01 sol
        **escrow_account.try_borrow_mut_lamports()? -= 10000000;
        **admin_account.try_borrow_mut_lamports()? += 10000000;

        if data.deposite_time_stamp <= data.unlock_time_stamp
            || data.deposite_time_stamp < Clock::get()?.unix_timestamp
        {
            return Err(VaultError::InvalidTimeError.into());
        }

        if data.amount * ((data.unlock_time_stamp - data.deposite_time_stamp) as u64)
            != **escrow_account.lamports.borrow()
                - Rent::get()?.minimum_balance(escrow_account.data_len())
        {
            return Err(VaultError::NotEnoughLamports.into());
        }

        if !sender_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let res = check_program_account(&data.token_address)?;
        let is_valid_token_address = matches!(res, ());
        if is_valid_token_address {
            return Err(VaultError::InvalidSPL20Address.into());
        }
        transfer(
            &data.token_address,
            sender_account.key,
            escrow_account.key,
            sender_account.key,
            &[sender_account.key],
            data.amount,
        )?;
        let escrow_data = VaultData::new(data, *sender_account.key);
        escrow_data.serialize(&mut &mut escrow_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_withdraw_from_vault(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: TransactionInput,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let escrow_account = next_account_info(account_info_iter)?;
        let sender_account = next_account_info(account_info_iter)?;

        let mut escrow_data = VaultData::try_from_slice(&escrow_account.data.borrow())
            .expect("failed to serialize escrow data");
        if *sender_account.key != escrow_data.withdrawer {
            return Err(ProgramError::IllegalOwner);
        }
        if !sender_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if escrow_data.withdrawn {
            return Err(VaultError::WithdrawError.into());
        }

        let time = Clock::get()?.unix_timestamp;
        if data.amount > escrow_data.token.amount {
            return Err(VaultError::WithdrawError.into());
        }

        if time < escrow_data.unlock_time_stamp {
            return Err(VaultError::InvalidTimeError.into());
        }

        if data.amount > escrow_data.token.amount - escrow_data.token.amount_withdrawn {
            return Err(ProgramError::InsufficientFunds);
        }

        transfer(
            &escrow_data.token.token_address,
            escrow_account.key,
            sender_account.key,
            escrow_account.key,
            &[sender_account.key],
            escrow_data.token.amount,
        )?;
        escrow_data.token.amount -= data.amount;
        escrow_data.token.amount_withdrawn += data.amount;

        if escrow_data.token.amount == escrow_data.token.amount_withdrawn {
            escrow_data.withdrawn = true
        }
        escrow_data.serialize(&mut &mut escrow_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_deposite_to_vault(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: TransactionInput,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let escrow_account = next_account_info(account_info_iter)?;
        let sender_account = next_account_info(account_info_iter)?;

        let mut escrow_data = VaultData::try_from_slice(&escrow_account.data.borrow())
            .expect("failed to serialize escrow data");

        if !sender_account.is_signer {
            return Err(ProgramError::IllegalOwner);
        }
        escrow_data.token.amount += data.amount;
        transfer(
            &data.token,
            sender_account.key,
            escrow_account.key,
            sender_account.key,
            &[sender_account.key],
            escrow_data.token.amount,
        )?;
        escrow_data.serialize(&mut &mut escrow_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_close_vault(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let escrow_account = next_account_info(account_info_iter)?;
        let sender_account = next_account_info(account_info_iter)?;
        let mut escrow_data = VaultData::try_from_slice(&escrow_account.data.borrow())
            .expect("failed to serialize escrow data");
        if escrow_data.withdrawer != *sender_account.key {
            return Err(ProgramError::IllegalOwner);
        }
        if !sender_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let time: i64 = Clock::get()?.unix_timestamp;

        if time < escrow_data.unlock_time_stamp {
            return Err(VaultError::InvalidTimeError.into());
        }
        transfer(
            &escrow_data.token.token_address,
            escrow_account.key,
            sender_account.key,
            escrow_account.key,
            &[sender_account.key],
            escrow_data.token.amount,
        )?;
        escrow_data.token.amount = 0;
        Ok(())
    }
}
