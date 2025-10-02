#![allow(unexpected_cfgs)]

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

#[derive(borsh::BorshDeserialize)]
#[cfg_attr(test, derive(borsh::BorshSerialize))]
struct Data {
    hash: String,
    signature: String,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let data: Data = match borsh::from_slice(instruction_data) {
        Ok(data) => data,
        Err(_) => {
            return Err(solana_program::program_error::ProgramError::BorshIoError(
                "failed to decode instruction data".to_string(),
            ));
        }
    };
    msg!("Hash: {}", data.hash);
    msg!("Signature: {}", data.signature);
    Ok(())
}

#[cfg(test)]
mod test {
    use solana_program_test::*;
    use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
    use solana_signer::Signer;
    use solana_transaction::Transaction;

    use crate::Data;

    #[tokio::test]
    async fn test_hello_world() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::default();
        program_test.add_program("tc_solana_program", program_id, None);
        let (banks_client, payer, recent_blockhash) = program_test.start().await;

        let instruction = Instruction {
            program_id,
            accounts: vec![],
            data: borsh::to_vec(&Data {
                hash: "test_hash".to_string(),
                signature: "test_signature".to_string(),
            })
            .unwrap(),
        };
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        transaction.sign(&[&payer], recent_blockhash);

        let transaction_result = banks_client.process_transaction(transaction).await;
        assert!(transaction_result.is_ok());
    }
}
