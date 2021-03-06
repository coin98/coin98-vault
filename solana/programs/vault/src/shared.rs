use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::{
  hashv,
};
use std::convert::TryInto;

static TOKEN_PROGRAM_ID: Pubkey = Pubkey::new_from_array([6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169]);

pub fn is_system_program<'a>(account: &AccountInfo<'a>) -> bool {
  *account.key == anchor_lang::system_program::ID
}

pub fn is_token_program<'a>(account: &AccountInfo<'a>) -> bool {
  *account.key == TOKEN_PROGRAM_ID
}

pub fn derive_event_id(event_id: u64) -> [u8; 8] {
  let data = DeriveEventIdParam {
    event_id: event_id,
  };
  let vec = data.try_to_vec().unwrap();
  let arr: [u8; 8] = vec.try_into().unwrap();
  arr
}

pub fn transfer_lamports<'info>(
  from_pubkey: &AccountInfo<'info>,
  to_pubkey: &AccountInfo<'info>,
  amount: u64,
  signer_seeds: &[&[&[u8]]],
) -> std::result::Result<(), ProgramError> {
  let instruction = &solana_program::system_instruction::transfer(from_pubkey.key, to_pubkey.key, amount);
  if signer_seeds.len() == 0 {
    solana_program::program::invoke(&instruction, &[from_pubkey.clone(), to_pubkey.clone()])
  }
  else {
    solana_program::program::invoke_signed(&instruction, &[from_pubkey.clone(), to_pubkey.clone()], &signer_seeds)
  }
}

pub fn transfer_token<'info>(
  owner: &AccountInfo<'info>,
  from_pubkey: &AccountInfo<'info>,
  to_pubkey: &AccountInfo<'info>,
  amount: u64,
  signer_seeds: &[&[&[u8]]],
) -> std::result::Result<(), ProgramError> {
  let data = TransferTokenParams {
    instruction: 3,
    amount: amount,
  };
  let instruction = solana_program::instruction::Instruction {
    program_id: TOKEN_PROGRAM_ID,
    accounts: vec![
      solana_program::instruction::AccountMeta::new(*from_pubkey.key, false),
      solana_program::instruction::AccountMeta::new(*to_pubkey.key, false),
      solana_program::instruction::AccountMeta::new_readonly(*owner.key, true),
    ],
    data: data.try_to_vec().unwrap(),
  };
  if signer_seeds.len() == 0 {
    solana_program::program::invoke(&instruction, &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()])
  }
  else {
    solana_program::program::invoke_signed(&instruction, &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()], &signer_seeds)
  }
}

/// Returns true if a `leaf` can be proved to be a part of a Merkle tree
/// defined by `root`. For this, a `proof` must be provided, containing
/// sibling hashes on the branch from the leaf to the root of the tree. Each
/// pair of leaves and each pair of pre-images are assumed to be sorted.
pub fn verify_proof(proofs: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
  let mut computed_hash = leaf;
  for proof in proofs.into_iter() {
    if computed_hash < proof {
      // Hash(current computed hash + current element of the proof)
      computed_hash = hashv(&[&computed_hash, &proof]).to_bytes();
    } else {
      // Hash(current element of the proof + current computed hash)
      computed_hash = hashv(&[&proof, &computed_hash]).to_bytes();
    }
  }
  // Check if the computed hash (root) is equal to the provided root
  computed_hash == root
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct DeriveEventIdParam {
  pub event_id: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct TransferTokenParams {
  pub instruction: u8,
  pub amount: u64,
}
