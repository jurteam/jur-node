use super::*;
use crate::{EthereumAddress, RootHash};
use sp_io::hashing::{blake2_256, keccak_256};
use sp_std::{vec, vec::Vec};
use rlp::DecoderError;
use frame_support::pallet_prelude::*;
use parity_scale_codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;

#[derive(Clone, PartialEq, Eq, Encode, Decode, Copy, RuntimeDebug)]
pub enum ErrorMessage {
    /// Invalid Proof Call
    InvalidProof,
}

impl From<rlp::DecoderError> for ErrorMessage {
    fn from(_: DecoderError) -> Self {
        ErrorMessage::InvalidProof
    }
}

pub fn decode_rlp(value: Vec<u8>) -> Result<Balance, ErrorMessage>{
    let rlp = rlp::Rlp::new(&value);
    let balance: Balance= rlp.as_val()?;
    Ok(balance)
}

pub fn verify_proof(
    mut root: RootHash,
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
        let blake2_256_hash: RootHash = blake2_256(proof_step);

        ensure!(blake2_256_hash == root, ErrorMessage::InvalidProof);

        let rlp = rlp::Rlp::new(proof_step);
        match rlp.item_count()? {
            2 => {
                let mut node = rlp.iter();
                let prefix: Vec<u8> = match node.next() {
                    Some(n) => n.as_val()?,
                    None => return Err(ErrorMessage::InvalidProof),
                };

                let value: Vec<u8> = match node.next() {
                    Some(n) => n.as_val()?,
                    None => return Err(ErrorMessage::InvalidProof),
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
                    return Err(ErrorMessage::InvalidProof);
                }

                if terminal {
                   if nibbles_iter.count()!=0 {
                       return Err(ErrorMessage::InvalidProof);
                   } else {
                       return Ok(value);
                   }
                }
                root = convert(value);
            },
            17 => {
                let mut node = rlp.iter();
                let key_nibble = match nibbles_iter.next() {
                     None => {
                         match node.nth(16 as usize) {
                             None => return Err(ErrorMessage::InvalidProof),
                             Some(value) => return Ok(value.as_val()?)
                         }
                     },
                    Some(value) => *value
                };


                let branch: Vec<u8> = match node.nth(key_nibble as usize) {
                    None => return Err(ErrorMessage::InvalidProof),
                    Some(value) => value.as_val()?
                };

                root = convert(branch);

            },
            _ => return Err(ErrorMessage::InvalidProof),
        };
    }

    Err(ErrorMessage::InvalidProof)
}

pub fn convert<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a valid proof {}", v.len()))
}

pub fn extract_storage_root(account_rlp: Vec<u8>) -> Result<Vec<u8>, ErrorMessage> {
    let rlp = rlp::Rlp::new(&account_rlp);

    match rlp.item_count()? {
        6 => {
            let mut node = rlp.iter();

            match node.nth(5 as usize) {
                None => Err(ErrorMessage::InvalidProof),
                Some(value) => Ok(value.as_val()?)
            }
        },
        _ => Err(ErrorMessage::InvalidProof)
    }
}

pub fn compute_key(eth_address: EthereumAddress) -> Vec<u8>{

    let mut x: Vec<u8> = vec![];
    x.extend_from_slice(&[0; 12]);
    x.extend_from_slice(&eth_address.0);
    x.extend_from_slice(&[0; 32]);

    let kec_256 =  keccak_256(x.as_slice());
    blake2_256(&kec_256).to_vec()
}