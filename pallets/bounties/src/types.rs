use codec::{Decode, Encode};
use frame_support::{pallet_prelude::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(NameLimit, CategoryLimit, DescriptionLimit, AccountLimit))]
pub struct Bounty<
	NameLimit: Get<u32>,
	CategoryLimit: Get<u32>,
	DescriptionLimit: Get<u32>,
	AccountId,
	AccountLimit: Get<u32>,
> {
	pub creator: AccountId,
	pub name: BoundedVec<u8, NameLimit>,
	pub category: BoundedVec<u8, CategoryLimit>,
	pub description: BoundedVec<u8, DescriptionLimit>,
	pub status: BountyStatus,
	pub participants: BoundedVec<AccountId, AccountLimit>,
	pub contributors: BoundedVec<AccountId, AccountLimit>,
	pub duration: u32,
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
