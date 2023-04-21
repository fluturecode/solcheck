use solana_sdk::{
    commitment::Commitment, pubkey::Pubkey, signer::Signer, transaction::Transaction,
    transaction_instruction::TransactionInstruction,
};

pub struct UploadVideoNFTInstruction {
    pub payer: Pubkey,
    pub video_url: String,
    pub name: String,
    pub description: String,
    pub price: u64,
}

impl UploadVideoNFTInstruction {
    pub fn new(
        payer: Pubkey,
        video_url: String,
        name: String,
        description: String,
        price: u64,
    ) -> Self {
        Self {
            payer,
            video_url,
            name,
            description,
            price,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        solana_sdk::serialize(&self).unwrap()
    }
}

pub fn upload_video_nft(
    connection: &mut solana_sdk::connection::Connection,
    signer: &Signer,
    instruction: &UploadVideoNFTInstruction,
) -> Result<Transaction, solana_sdk::program::ProgramError> {
    let mut transaction =
        Transaction::new_with_recent_blockhash(connection.last_blockhash().unwrap());

    transaction.add_instruction(
        *instruction.serialize(),
        signer.pubkey(),
        vec![instruction.payer],
    );

    connection.send_transaction(&transaction)?;

    Ok(transaction)
}
