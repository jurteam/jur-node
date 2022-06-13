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
    root: RootHash,
    proof: Vec<Vec<u8>>,
    key: Vec<u8>,
) -> Result<Vec<u8>, InvalidProof> {
    let mut nibbles = vec![];

    for (i, k) in key.iter().enumerate() {
        nibbles.push(k >> 4);
        nibbles.push(k % 16);
    }

    let nibbles_iter = nibbles.iter();

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

                let n = nibbles_iter.take(prefix_nibbles_len);

                //assert_eq!(Some(prefix_nibbles), None);
                /// TODO This is to removed
                return Ok(vec![]);
            },
            17 => return Err(InvalidProof::NotImplemented),
            _ => return Err(InvalidProof::Call),
        }

        return Err(InvalidProof::NotImplemented)
    }

    Err(InvalidProof::Call)
}
