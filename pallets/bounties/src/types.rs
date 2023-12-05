use crate::Vec;
use codec::{Decode, Encode};
use frame_support::{pallet_prelude::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(
	NameLimit,
	CategoryLimit,
	BadgeNameLimit,
	DescriptionLimit,
	AccountLimit
))]
pub struct Bounty<
	NameLimit: Get<u32>,
	CategoryLimit: Get<u32>,
	BadgeNameLimit: Get<u32>,
	DescriptionLimit: Get<u32>,
	AccountId,
	AccountLimit: Get<u32>,
	BlockNumber,
> {
	pub creator: AccountId,
	pub name: BoundedVec<u8, NameLimit>,
	pub category: Vec<BoundedVec<u8, CategoryLimit>>,
	pub badge: BoundedVec<u8, BadgeNameLimit>,
	pub description: BoundedVec<u8, DescriptionLimit>,
	pub status: BountyStatus,
	pub participants: BoundedVec<AccountId, AccountLimit>,
	pub contributors: Vec<AccountId>,
	pub duration: u32,
	pub deadline_block: BlockNumber,
}

/// Status of bounty.
#[derive(Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, Encode, Decode)]
pub enum BountyStatus {
	/// Bounty is ongoing.
	Ongoing,
	/// Bounty's work is in progress.
	WorkInProgress,
	/// Bounty is completed.
	Completed,
}
