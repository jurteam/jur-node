use crate::RootHash;
use sp_io::hashing::blake2_256;
use sp_std::{vec, vec::Vec};
use rlp::DecoderError;
use frame_support::pallet_prelude::*;
use parity_scale_codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;

#[derive(Clone, PartialEq, Eq, Encode, Decode, Copy, RuntimeDebug)]
pub enum InvalidProof {
    /// Invalid Proof Call
    Call,
    /// TODO Need to be removed
    NotImplemented
}

impl From<rlp::DecoderError> for InvalidProof {
    fn from(_: DecoderError) -> Self {
        InvalidProof::Call
    }
}

pub fn verify_proof(
    mut root: RootHash,
    proof: Vec<Vec<u8>>,
    key: Vec<u8>,
) -> Result<Vec<u8>, InvalidProof> {
    let mut nibbles = vec![];

    for (_i, k) in key.iter().enumerate() {
        nibbles.push(k >> 4);
        nibbles.push(k % 16);
    }

    let mut nibbles_iter = nibbles.iter();

    for proof_step in proof.iter() {
        let blake2_256_hash: RootHash = blake2_256(proof_step);

        ensure!(blake2_256_hash == root, InvalidProof::Call);

        let rlp = rlp::Rlp::new(proof_step);
        match rlp.item_count()? {
            2 => {
                let mut node = rlp.iter();
                let prefix: Vec<u8> = match node.next() {
                    Some(n) => n.as_val()?,
                    None => return Err(InvalidProof::Call),
                };

                let value: Vec<u8> = match node.next() {
                    Some(n) => n.as_val()?,
                    None => return Err(InvalidProof::Call),
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
                    return Err(InvalidProof::Call);
                }

                if terminal {
                   if nibbles_iter.count()!=0 {
                       return Err(InvalidProof::Call);
                   } else {
                       return Ok(value);
                   }
                }
                //assert_eq!(Some(prefix_nibbles), None);
                /// TODO This is to removed
                return Err(InvalidProof::NotImplemented);
            },
            17 => {
                let key_nibble = match nibbles_iter.next() {
                     None => return Err(InvalidProof::NotImplemented),
                    Some(value) => *value
                };

                let mut node = rlp.iter();
                let branch: Vec<u8> = match node.nth(key_nibble as usize) {
                    None => return Err(InvalidProof::Call),
                    Some(value) => value.as_val()?
                };


                root = convert(branch);

            },
            _ => return Err(InvalidProof::Call),
        };
    }

    Err(InvalidProof::Call)
}

fn convert<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a valid proof {}", v.len()))
}