#![cfg_attr(not(feature = "std"), no_std)]

pub mod proof;

pub mod macros;
#[cfg(test)]
mod tests;

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use sp_runtime::{traits::Saturating, RuntimeDebug};

/// Balance of an account.
pub type Balance = u128;

pub type CurrencyId = u32;

pub type CommunityId = u32;

pub type PassportId = u32;

pub type ProposalId = u32;

pub type BountyId = u32;

pub type EventId = u32;

pub type ChoiceId = u32;

pub type VechainHash = [u8; 32];

pub type BlockNumber = u32;

// Native Token
pub const JUR: CurrencyId = 0;

/// Number Constants

pub const PROPOSAL_DURATION_LIMIT: u32 = 30;

pub const BOUNTY_DURATION_LIMIT: u32 = 365;

pub const INITIAL_INDEX: usize = 0;

pub const INITIAL_NODE_INDEX: u8 = 0;

pub const ACCOUNT_ID_INITIAL_INDEX: usize = 1;

pub const SHORT_NODE_INDEX: u8 = 2;

pub const OFFSET_INDEX: usize = 2;

pub const NIBBLES_RIGHT_SHIFT_INDEX: u8 = 4;

pub const NODE_ROOT_INDEX: usize = 5;

pub const RLP_ROOT_ITEM_INDEX: u8 = 6;

pub const FROM_INDEX: usize = 12;

pub const NIBBLES_PATH_LEN: u8 = 16;

pub const ODD_NODE_INDEX: u8 = 16;

pub const RLP_FULL_NODE_INDEX: u8 = 17;

pub const FULL_NODE_INDEX: u8 = 16;

pub const ETHEREUM_ADDRESS_SIZE: usize = 20;

pub const TERMINAL_NODE_INDEX: u8 = 32;

pub const TO_INDEX: usize = 32;

pub const VECHAIN_HASH_SIZE: u8 = 32;

pub const MAX_ACCOUNT_ID_INDEX: usize = 33;

pub const ADDRESS_LEN: usize = 35;

pub const ETHEREUM_ADDRESS_LEN: usize = 40;

pub const MAX_KEY_SIZE: usize = 64;

pub const ETHEREUM_SIGNATURE_SIZE: usize = 65;

pub const PRIORITY: u64 = 100;

pub const INVALID_ETHEREUM_SIGNATURE_ERR_CODE: u8 = 0;

pub const INVALID_SUBSTRATE_ADDRESS_ERR_CODE: u8 = 1;

pub const INVALID_PREFIX_ERR_CODE: u8 = 2;

pub const INVALID_CONTENT_ERR_CODE: u8 = 3;

pub const INVALID_JSON_ERR_CODE: u8 = 4;

pub const INVALID_BALANCE_ERR_CODE: u8 = 5;

pub const INVALID_INPUT_ERR_CODE: u8 = 6;

pub const INVALID_PROOF_ERR_CODE: u8 = 7;

/// Blocks per day is a assumption of block generating by chain in 24 hours
/// Assuming chain generating the blocks in every 6 second. 1 Block = 6 second
pub const BLOCKS_PER_DAY: u32 = 14_400;

/// An Ethereum address (i.e. 20 bytes, used to represent an Ethereum account).
///
/// This gets serialized to the 0x-prefixed hex representation.
#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct EthereumAddress(pub [u8; ETHEREUM_ADDRESS_SIZE]);

#[cfg(feature = "std")]
impl Serialize for EthereumAddress {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let hex: String = rustc_hex::ToHex::to_hex(&self.0[..]);
		serializer.serialize_str(&format!("0x{}", hex))
	}
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for EthereumAddress {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let base_string = String::deserialize(deserializer)?;
		let offset = if base_string.starts_with("0x") { OFFSET_INDEX } else { INITIAL_INDEX };
		let s = &base_string[offset..];
		if s.len() != ETHEREUM_ADDRESS_LEN {
			Err(serde::de::Error::custom(
				"Bad length of Ethereum address (should be 42 including '0x')",
			))?;
		}
		let raw: Vec<u8> = rustc_hex::FromHex::from_hex(s)
			.map_err(|e| serde::de::Error::custom(format!("{:?}", e)))?;
		let mut r = Self::default();
		r.0.copy_from_slice(&raw);
		Ok(r)
	}
}

/// Custom validity errors used while validating transactions.
#[repr(u8)]
pub enum ValidityError {
	/// The Ethereum signature is invalid.
	InvalidEthereumSignature = INVALID_ETHEREUM_SIGNATURE_ERR_CODE,
	/// Substarte address is invalid.
	InvalidSubstrateAddress = INVALID_SUBSTRATE_ADDRESS_ERR_CODE,
	/// Prefix does not match.
	PrefixDoesNotMatch = INVALID_PREFIX_ERR_CODE,
	/// Content not found.
	ContentNotFound = INVALID_CONTENT_ERR_CODE,
	/// Invalid JSON.
	InvalidJson = INVALID_JSON_ERR_CODE,
	/// Not Sufficient locked balance
	NotSufficientLockedBalance = INVALID_BALANCE_ERR_CODE,
	/// Invalid input
	InvalidInput = INVALID_INPUT_ERR_CODE,
	/// Invalid proof
	InvalidProof = INVALID_PROOF_ERR_CODE,
}

impl From<ValidityError> for u8 {
	fn from(err: ValidityError) -> Self {
		err as u8
	}
}

pub trait Incrementable {
	fn increment(&self) -> Self;
	fn initial_value() -> Self;
	fn jur_community_reserve_slots() -> Self;
}

impl_incrementable!(u16, u32, u64, u128, i16, i32, i64, i128);
