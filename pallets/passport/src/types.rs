use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use frame_support::{pallet_prelude::Get, BoundedVec};
use scale_info::TypeInfo;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
#[scale_info(skip_type_params(AddressLimit))]
pub struct PassportDetails<PassportId, AddressLimit: Get<u32>> {
	pub id: PassportId,
	pub address: Option<BoundedVec<u8, AddressLimit>>,
}
