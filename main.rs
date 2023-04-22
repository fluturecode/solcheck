// The `use` statement imports the necessary types & traits form SolanaSDK. Here we are
// importing `next_account_info` and `AccountInfo` to iterate over the accounts paased to
// the program's entry point and `Pubkey` to handle public keys.
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{Rent, Sysvar},
};

//  Rust use statement that imports the necessary types and functions from the spl_token
// crate for working with token accounts and mints on the Solana blockchain.
use spl_token::{
    instruction::{approve, initialize_account, initialize_mint, mint_to},
    state::{Account, Mint},
};

// #[derive(Debug)] is a Rust attribute that enables the automatic implementation of the
// Debug trait for the struct. The Debug trait provides a convenient way to print out the
// contents of a struct for debugging purposes.
#[derive(Debug)]
struct VideoNFT {
    data: Vec<u8>,
}

// The #[derive(Debug)] macro annotation automatically generates the implementation of the
// Debug trait for the Instruction enum. This allows developers to easily print the
// Instruction enum using the println! macro or the dbg! macro for debugging purposes.

// The Instruction enum defines two variants, CreateVideoNFT and TransferVideoNFT, which
// represent the two possible operations that can be performed on the video NFT. The
// CreateVideoNFT variant is used to create a new video NFT, while the TransferVideoNFT
// variant is used to transfer an existing video NFT to a new owner.
#[derive(Debug)]
enum Instruction {
    CreateVideoNFT,
    TransferVideoNFT,
}

// The parse_instruction function takes in a slice of bytes (instruction_data) and returns
// a Result with either an Instruction or a ProgramError.
// Overall, this function is used to parse the instruction data received by the program to
// determine which operation should be performed on the video NFT.
fn parse_instruction(instruction_data: &[u8]) -> Result<Instruction, ProgramError> {
    // The function matches the first byte of the instruction_data slice to either 0 or 1.
    // If the first byte is 0, the function returns the CreateVideoNFT variant of the
    // Instruction enum. If the first byte is 1, the function returns the TransferVideoNFT
    // variant of the Instruction enum. If the first byte does not match either 0 or 1, the
    // function returns an error of type ProgramError::InvalidInstructionData.
    let instruction = match instruction_data[0] {
        0 => Instruction::CreateVideoNFT,
        1 => Instruction::TransferVideoNFT,
        _ => return Err(ProgramError::InvalidInstructionData),
    };
    Ok(instruction)
}

fn create_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<AccountInfo, ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let account = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

    // Check that the account does not already exist
    if Account::unpack(&account.data.borrow())?.is_initialized {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Calculate the required account size
    let data_len = size_of::<VideoNFT>() + rent.minimum_balance(size_of::<Account>());
    let lamports = rent
        .minimum_balance(data_len)
        .max(1)
        .saturating_sub(account.lamports());

    // Create the account
    **account_info_iter = AccountInfo::new(
        account_info_iter.next().unwrap().key,
        false,
        true,
        lamports,
        data_len as u64,
        program_id,
        false,
        0,
    );
    let mut new_account = Account::unpack_unchecked(&account.data.borrow())?;
    new_account.is_initialized = true;
    new_account.serialize(&mut *account.data.borrow_mut())?;
    Ok(account.clone())
}

// The store_video_nft function takes a reference to a VideoNFT and an AccountInfo as
// arguments, and returns a Result with either Ok(()) or a ProgramError.
// The function retrieves the account data by borrowing it mutably and unpacks it as an
// Account struct. It then calculates the offset to the start of the video NFT data by
// taking the size of the Account struct. Next, it retrieves a mutable reference to the
// video NFT data by slicing the account.data buffer. It unpacks the video NFT data as a
// VideoNFT struct and modifies its data field to the new video NFT data provided. Finally,
// it packs the updated VideoNFT struct back into the video NFT data buffer.
// Overall, this function is used to store the video NFT data in a given account.
fn store_video_nft(video_nft: &VideoNFT, account: &AccountInfo) -> Result<(), ProgramError> {
    let mut data = account.data.borrow_mut();
    let account_data = Account::unpack_unchecked(&data)?;
    let offset = size_of::<Account>();
    let video_nft_data = &mut data[offset..];
    let mut video_nft_account = VideoNFT::unpack_unchecked(video_nft_data)?;
    video_nft_account.data = video_nft.data.clone();
    video_nft_account.pack(video_nft_data)?;
    Ok(())
}
