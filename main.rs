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
// The process_instruction function is where the bulk of the program logic will go. This
// function takes in the
// program ID, accounts, and instruction data, and returns a ProgramResult indicating
// whether the instruction was processed successfully or not.

// Within the process_instruction function, you would typically parse the instruction
// data and perform any necessary checks or validations to ensure that the instruction
// is valid and authorized.
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Parse the instruction data to get the video NFT file
    let video_nft = parse_video_nft(instruction_data)?;

    // Create a new account to hold the video NFT data
    let account = create_account(program_id, accounts)?;

    // Serialize the video NFT data and store it in the account
    store_video_nft(&video_nft, &account)?;

    Ok(())
}

fn parse_video_nft(instruction_data: &[u8]) -> Result<VideoNFT, ProgramError> {
    // Parse the instruction data to get the video NFT file
    // ...
}

fn create_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<AccountInfo, ProgramError> {
    // Create a new account to hold the video NFT data
    // ...
}

fn store_video_nft(video_nft: &VideoNFT, account: &AccountInfo) -> Result<(), ProgramError> {
    // Serialize the video NFT data and store it in the account
    // ...
}
