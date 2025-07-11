use borsh::{BorshDeserialize,BorshSerialize};
use solana_program::{
    account_info::{next_account_info,AccountInfo}, entrypoint::ProgramResult, msg, program::invoke, 
    program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,entrypoint
};

#[derive(BorshDeserialize,BorshSerialize)]
struct CounterState{
    count:u32
}

#[derive(BorshDeserialize,BorshSerialize)]
enum Instruction{
    Init,
    Double,
    Half
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
)->ProgramResult{

    let instruction=Instruction::try_from_slice(instruction_data)?;

    match instruction {
        
        Instruction::Init=>{

            msg!("INIT");
            let mut iter=accounts.iter();
            let data_acc=next_account_info(&mut iter)?;
            let payer=next_account_info(&mut iter)?;
            let system_program=next_account_info(&mut iter)?;

            if !payer.is_signer{
                return Err(ProgramError::MissingRequiredSignature);
            }
            let space = 4;

            let rent=Rent::get()?;
            let lamports=rent.minimum_balance(space);

            let create_acc_ix=system_instruction::create_account(payer.key, data_acc.key,
                 lamports, space as u64, program_id);
            invoke(&create_acc_ix, &[
                    payer.clone(),
                    data_acc.clone(),
                    system_program.clone()
            ])?;
            let counter_state=CounterState{count:1};
            counter_state.serialize(&mut *data_acc.data.borrow_mut())?;
        }
        Instruction::Double=>{
            msg!("Double");

            let mut iter= accounts.iter();
            let data_acc=next_account_info(&mut iter)?;

            if data_acc.owner!=program_id{
                return Err(ProgramError::IncorrectProgramId);
            }

            let mut counter_state=CounterState::try_from_slice(&data_acc.data.borrow())?;
            counter_state.count*=2;
            counter_state.serialize(&mut *data_acc.data.borrow_mut())?;            
        }
        Instruction::Half=>{
            msg!("Half");

            let mut iter= accounts.iter();
            let data_acc=next_account_info(&mut iter)?;

            if data_acc.owner!=program_id{
                return Err(ProgramError::IncorrectProgramId);
            }

            let mut counter_state=CounterState::try_from_slice(&data_acc.data.borrow())?;
            counter_state.count/=2;
            counter_state.serialize(&mut *data_acc.data.borrow_mut())?; 
        }
    }

    Ok(())
}

