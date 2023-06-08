use codec::{Decode, Encode};
use frame_support::{pallet_prelude::Get, BoundedVec, RuntimeDebug};
use scale_info::TypeInfo;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
#[scale_info(skip_type_params(DescriptionLimit, AddressLimit, AccountLimit))]
pub struct Proposal<DescriptionLimit: Get<u32>, AddressLimit: Get<u32>, AccountId, AccountLimit: Get<u32>> {
	pub proposer: AccountId,
	pub address: BoundedVec<u8, AddressLimit>,
	pub description: BoundedVec<u8, DescriptionLimit>,
	pub historical: bool,
	pub status: bool,
	pub voter_accounts: BoundedVec<AccountId, AccountLimit>,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
#[scale_info(skip_type_params(LabelLimit))]
pub struct Choice<ChoiceId, LabelLimit: Get<u32>> {
	pub id: ChoiceId,
	pub label: BoundedVec<u8, LabelLimit>,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
#[scale_info(skip_type_params(AccountLimit))]
pub struct Vote<BlockNumber, AccountId, AccountLimit: Get<u32>> {
	pub who: BoundedVec<AccountId, AccountLimit>,
	pub vote_count: u64,
	pub last_voted: BlockNumber,
}