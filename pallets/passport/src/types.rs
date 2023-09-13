use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use frame_support::{pallet_prelude::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
#[scale_info(skip_type_params(AddressLimit))]
pub struct PassportDetails<PassportId, AddressLimit: Get<u32>> {
	pub id: PassportId,
	pub address: Option<BoundedVec<u8, AddressLimit>>,
	pub badges: Vec<Vec<u8>>,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
#[scale_info(skip_type_params(BadgeNameLimit, DescriptionLimit, AddressLimit))]
pub struct BadgeDetails<DescriptionLimit: Get<u32>, AddressLimit: Get<u32>> {
	pub badge_type: BadgesType,
	pub description: BoundedVec<u8, DescriptionLimit>,
	pub address: BoundedVec<u8, AddressLimit>,
}

#[derive(Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, Encode, Decode)]
pub enum BadgesType {
	/// Participation badge.
	Participation,
	/// visa type badge.
	Visa,
	/// credential badge type.
	Credentials,
	/// Badge as rewards.
	Rewards,
}
