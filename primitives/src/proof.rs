use super::*;
use crate::{EthereumAddress, VechainHash};
use frame_support::pallet_prelude::*;
use parity_scale_codec::{Decode, Encode};
use rlp::DecoderError;
use sp_io::hashing::{blake2_256, keccak_256};
use sp_runtime::RuntimeDebug;
use sp_std::{vec, vec::Vec};

#[derive(Clone, PartialEq, Eq, Encode, Decode, Copy, RuntimeDebug)]
pub enum ErrorMessage {
	/// Invalid Proof RLP
	InvalidRLP,
	/// Invalid Proof Data
	InvalidProofData,
	/// Invalid Key
	InvalidKey,
	/// Proof Too Short
	ProofTooShort,
	/// Invalid Node
	InvalidNode,
	/// Invalid Account
	InvalidAccount,
	/// Invalid Input
	InvalidInput,
}

impl From<rlp::DecoderError> for ErrorMessage {
	fn from(_: DecoderError) -> Self {
		ErrorMessage::InvalidRLP
	}
}

pub fn decode_rlp(value: Vec<u8>) -> Result<Balance, ErrorMessage> {
	let rlp = rlp::Rlp::new(&value);
	let balance: Balance = rlp.as_val()?;
	Ok(balance)
}

pub fn verify_proof(
	mut root: VechainHash,
	proof: Vec<Vec<u8>>,
	key: Vec<u8>,
) -> Result<Vec<u8>, ErrorMessage> {
	let mut nibbles = vec![];

	for k in key.iter() {
		nibbles.push(k >> NIBBLES_RIGHT_SHIFT_INDEX);
		nibbles.push(k % NIBBLES_PATH_LEN);
	}

	let mut nibbles_iter = nibbles.iter();

	for proof_step in proof.iter() {
		let blake2_256_hash: VechainHash = blake2_256(proof_step);

		ensure!(blake2_256_hash == root, ErrorMessage::InvalidProofData);

		let rlp = rlp::Rlp::new(proof_step);
		match rlp.item_count()? as u8 {
			SHORT_NODE_INDEX => {
				let mut node = rlp.iter();
				let prefix: Vec<u8> = match node.next() {
					Some(n) => n.as_val()?,
					None => return Err(ErrorMessage::InvalidRLP),
				};

				let value: Vec<u8> = match node.next() {
					Some(n) => n.as_val()?,
					None => return Err(ErrorMessage::InvalidRLP),
				};

				ensure!(!prefix.is_empty(), ErrorMessage::InvalidProofData);
				let odd = prefix[INITIAL_INDEX] & ODD_NODE_INDEX != INITIAL_NODE_INDEX;
				let terminal = prefix[INITIAL_INDEX] & TERMINAL_NODE_INDEX != INITIAL_NODE_INDEX;

				let mut prefix_nibbles = vec![];

				for (i, p) in prefix.iter().enumerate() {
					if i != INITIAL_INDEX {
						prefix_nibbles.push(p >> NIBBLES_RIGHT_SHIFT_INDEX);
					}

					if i != INITIAL_INDEX || odd {
						prefix_nibbles.push(p % NIBBLES_PATH_LEN);
					}
				}

				let prefix_nibbles_len = prefix_nibbles.len();

				let n = nibbles_iter.by_ref().take(prefix_nibbles_len);

				if !n.eq(&prefix_nibbles) {
					return Err(ErrorMessage::InvalidKey);
				}

				if terminal {
					if nibbles_iter.count() != INITIAL_INDEX {
						return Err(ErrorMessage::ProofTooShort);
					} else {
						return Ok(value);
					}
				}
				root = convert(value)?;
			},
			RLP_FULL_NODE_INDEX => {
				let mut node = rlp.iter();
				let key_nibble = match nibbles_iter.next() {
					None => match node.nth(FULL_NODE_INDEX as usize) {
						None => return Err(ErrorMessage::InvalidRLP),
						Some(value) => return Ok(value.as_val()?),
					},
					Some(value) => *value,
				};

				let branch: Vec<u8> = match node.nth(key_nibble as usize) {
					None => return Err(ErrorMessage::InvalidNode),
					Some(value) => value.as_val()?,
				};

				root = convert(branch)?;
			},
			_ => return Err(ErrorMessage::ProofTooShort),
		};
	}

	Err(ErrorMessage::InvalidRLP)
}

pub fn convert<T, const N: usize>(v: Vec<T>) -> Result<[T; N], ErrorMessage> {
	v.try_into().ok().ok_or(ErrorMessage::InvalidInput)
}

pub fn extract_storage_root(account_rlp: Vec<u8>) -> Result<Vec<u8>, ErrorMessage> {
	let rlp = rlp::Rlp::new(&account_rlp);

	match rlp.item_count()? as u8 {
		RLP_ROOT_ITEM_INDEX => {
			let mut node = rlp.iter();

			match node.nth(NODE_ROOT_INDEX) {
				None => Err(ErrorMessage::InvalidRLP),
				Some(value) => Ok(value.as_val()?),
			}
		},
		_ => Err(ErrorMessage::InvalidAccount),
	}
}

pub fn compute_storage_key_for_depositor(eth_address: EthereumAddress) -> Vec<u8> {
	let mut key = [0u8; MAX_KEY_SIZE];
	key[FROM_INDEX..TO_INDEX].copy_from_slice(&eth_address.0);

	let kec_256 = keccak_256(&key);
	blake2_256(&kec_256).to_vec()
}
