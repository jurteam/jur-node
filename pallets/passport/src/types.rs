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
	pub stamps: Option<Vec<BoundedVec<u8, AddressLimit>>>,
	pub avatar: Option<BoundedVec<u8, AddressLimit>>,
}
