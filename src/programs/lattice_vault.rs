use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use pqcrypto_traits::sign::{DetachedSignature, PublicKey};
use pqcrypto_dilithium::dilithium3;
use crate::vaults::lattice_schnorr::LatticeSchnorrVault;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let vault_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;

    // Parse the signature and message from instruction data
    if instruction_data.len() < dilithium3::DETACHED_SIGNATURE_BYTES {
        return Err(ProgramError::InvalidInstructionData);
    }
    let signature = DetachedSignature::from_bytes(&instruction_data[..dilithium3::DETACHED_SIGNATURE_BYTES])
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let message = &instruction_data[dilithium3::DETACHED_SIGNATURE_BYTES..];

    // Parse the public key from the owner account
    let pk = PublicKey::from_bytes(owner_account.try_borrow_data()?)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Verify the lattice-based signature
    LatticeSchnorrVault::verify(&pk, message, &signature)
        .map_err(|_| ProgramError::InvalidArgument)?;

    // Perform vault operations (e.g., transfer tokens) here
    msg!("Lattice-based signature verified successfully!");

    Ok(())
} 
