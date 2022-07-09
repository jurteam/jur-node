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

	for (_i, k) in key.iter().enumerate() {
		nibbles.push(k >> 4);
		nibbles.push(k % 16);
	}

	let mut nibbles_iter = nibbles.iter();

	for proof_step in proof.iter() {
		let blake2_256_hash: VechainHash = blake2_256(proof_step);

		ensure!(blake2_256_hash == root, ErrorMessage::InvalidProofData);

		let rlp = rlp::Rlp::new(proof_step);
		match rlp.item_count()? {
			2 => {
				let mut node = rlp.iter();
				let prefix: Vec<u8> = match node.next() {
					Some(n) => n.as_val()?,
					None => return Err(ErrorMessage::InvalidRLP),
				};

				let value: Vec<u8> = match node.next() {
					Some(n) => n.as_val()?,
					None => return Err(ErrorMessage::InvalidRLP),
				};

				let odd = prefix[0] & 16 != 0;
				let terminal = prefix[0] & 32 != 0;

				let mut prefix_nibbles = vec![];

				for (i, p) in prefix.iter().enumerate() {
					if i != 0 {
						prefix_nibbles.push(p >> 4);
					}

					if i != 0 || odd {
						prefix_nibbles.push(p % 16);
					}
				}

				let prefix_nibbles_len = prefix_nibbles.len();

				let n = nibbles_iter.by_ref().take(prefix_nibbles_len);

				if !n.eq(&prefix_nibbles) {
					return Err(ErrorMessage::InvalidKey);
				}

				if terminal {
					if nibbles_iter.count() != 0 {
						return Err(ErrorMessage::ProofTooShort);
					} else {
						return Ok(value);
					}
				}
				root = convert(value)?;
			},
			17 => {
				let mut node = rlp.iter();
				let key_nibble = match nibbles_iter.next() {
					None => match node.nth(16 as usize) {
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

	match rlp.item_count()? {
		6 => {
			let mut node = rlp.iter();

			match node.nth(5 as usize) {
				None => Err(ErrorMessage::InvalidRLP),
				Some(value) => Ok(value.as_val()?),
			}
		},
		_ => Err(ErrorMessage::InvalidAccount),
	}
}

pub fn compute_storage_key_for_depositor(eth_address: EthereumAddress) -> Vec<u8> {
	let mut x: Vec<u8> = vec![];
	x.extend_from_slice(&[0; 12]);
	x.extend_from_slice(&eth_address.0);
	x.extend_from_slice(&[0; 32]);

	let kec_256 = keccak_256(x.as_slice());
	blake2_256(&kec_256).to_vec()
}
