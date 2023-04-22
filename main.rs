use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::processor::{process_instruction, Instruction};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = match Instruction::unpack(instruction_data) {
        Ok(instruction) => instruction,
        Err(_) => return Err(ProgramError::InvalidInstructionData),
    };
    match instruction {
        Instruction::CreateVideoNFT => {
            msg!("Instruction: CreateVideoNFT");
            crate::processor::process_create_video_nft(program_id, accounts)
        }
        Instruction::TransferVideoNFT => {
            msg!("Instruction: TransferVideoNFT");
            crate::processor::process_transfer_video_nft(program_id, accounts)
        }
    }
}
