// !Program State

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{clock::UnixTimestamp, pubkey::Pubkey};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CreateVaultInput {
    pub token_address: Pubkey,
    pub amount: u64,
    pub withdrawer: Pubkey,
    pub deposite_time_stamp: UnixTimestamp,
    pub withdrawn: bool,
    pub deposited: bool,
    pub unlock_time_stamp: UnixTimestamp,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SP20TokenData {
    pub depositor: Pubkey,
    pub amount: u64,
    pub token_address: Pubkey,
    pub black_listed: bool,
    pub amount_withdrawn: u64,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct User {
    pub user_type: UserType,
    pub address: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum UserType {
    Admin,
    Investor,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TransactionInput {
    pub amount: u64,
    pub token: Pubkey,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct VaultData {
    pub withdrawer: Pubkey,
    pub deposite_time_stamp: UnixTimestamp,
    pub unlock_time_stamp: UnixTimestamp,
    pub withdrawn: bool,
    pub deposited: bool,
    pub token: SP20TokenData,
}

impl VaultData {
    pub fn new(data: CreateVaultInput, sender: Pubkey) -> Self {
        Self {
            deposite_time_stamp: data.deposite_time_stamp,
            unlock_time_stamp: data.unlock_time_stamp,
            withdrawn: data.withdrawn,
            deposited: data.deposited,
            token: SP20TokenData {
                depositor: sender,
                amount: data.amount,
                token_address: data.token_address,
                black_listed: false,
                amount_withdrawn: 0,
            },
            withdrawer: data.withdrawer,
        }
    }
}

// Added this test to check the size of the VaultData
#[test]
fn check_size() {
    let vault = VaultData {
        deposite_time_stamp: 0,
        unlock_time_stamp: 0,
        withdrawn: true,
        deposited: false,
        token: SP20TokenData {
            depositor: Pubkey::new_unique(),
            amount: 10,
            token_address: Pubkey::new_unique(),
            black_listed: false,
            amount_withdrawn: 0,
        },
        withdrawer: Pubkey::new_unique(),
    };
    let compress = vault.try_to_vec().expect("something went wrong!");
    println!("{}", compress.len())
}
