use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalcResult {
    /// Result of the addition operation
    pub add_result: u32,
    /// Result of the subtraction operation
    pub sub_result: u32,
}

// Declare and export the program's entrypoint
entrypoint!(handle_instruction);

// Program entrypoint's implementation
pub fn handle_instruction(
    program_id: &Pubkey, // Public key of the account the calculator program was loaded into
    accounts: &[AccountInfo], // Accounts used by the program
    instruction_data: &[u8], // Input data containing two numbers and operation choice
) -> ProgramResult {
    msg!("Calculator program entrypoint");

    // Ensure the instruction data is the correct size
    if instruction_data.len() != 12 {
        msg!("Invalid instruction data size");
        return Err(ProgramError::InvalidInstructionData);
    }

    // Parse the input data
    let num1 = u32::from_le_bytes(instruction_data[0..4].try_into().unwrap());
    let num2 = u32::from_le_bytes(instruction_data[4..8].try_into().unwrap());
    let operation = u32::from_le_bytes(instruction_data[8..12].try_into().unwrap());

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the calculator account to store the results
    let calc_account = next_account_info(accounts_iter)?;

    // The calculator account must be owned by the program
    if calc_account.owner != program_id {
        msg!("Calculator account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Perform the requested operation
    let mut calc_data = CalcResult::try_from_slice(&calc_account.data.borrow())?;

    match operation {
        0 => {
            // Calculate the addition
            calc_data.add_result = num1 + num2;
            msg!("Addition result: {}", calc_data.add_result);
        }
        1 => {
            // Calculate the subtraction
            if num1 >= num2 {
                calc_data.sub_result = num1 - num2;
                msg!("Subtraction result: {}", calc_data.sub_result);
            } else {
                msg!("Invalid subtraction operation: num1 is less than num2");
                return Err(ProgramError::InvalidArgument);
            }
        }
        _ => {
            msg!("Invalid operation choice");
            return Err(ProgramError::InvalidArgument);
        }
    }

    // Serialize and store the updated calculator data
    calc_data.serialize(&mut &mut calc_account.data.borrow_mut()[..])?;

    Ok(())
}

// Tests for the calculator program
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_calculator_operations() {
        let program_id = Pubkey::default();
        let calc_key = Pubkey::default();
        let mut lamports = 0;
        let mut calc_data = vec![0; mem::size_of::<CalcResult>()];
        let owner = Pubkey::default();
        let calc_account = AccountInfo::new(
            &calc_key,
            false,
            true,
            &mut lamports,
            &mut calc_data,
            &owner,
            false,
            Epoch::default(),
        );

        let num1: u32= 100;
        let num2: u32 = 30;
        let add_operation: u32 = 0; // 0 for addition
        let add_instruction_data = [num1.to_le_bytes(), num2.to_le_bytes(), add_operation.to_le_bytes()]
            .concat();

        let accounts = vec![calc_account];

        assert_eq!(
            CalcResult::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .add_result,
            0
        );

        handle_instruction(&program_id, &accounts, &add_instruction_data).unwrap();

        assert_eq!(
            CalcResult::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .add_result,
            num1 + num2
        );

        // Test the subtraction operation
        let sub_operation: u32 = 1; // 1 for subtraction
        let sub_instruction_data = [num1.to_le_bytes(), num2.to_le_bytes(), sub_operation.to_le_bytes()]
            .concat();

        handle_instruction(&program_id, &accounts, &sub_instruction_data).unwrap();

        assert_eq!(
            CalcResult::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .sub_result,
            num1 - num2
        );
    }
}
