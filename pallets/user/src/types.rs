use super::*;
use codec::{Decode, Encode};
use frame_support::{pallet_prelude::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_std::prelude::*;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
#[scale_info(skip_type_params(NameLimit, AddressLimit))]
pub struct User< NameLimit: Get<u32>, AddressLimit: Get<u32>> {
	pub username: Option<BoundedVec<u8, NameLimit>>,
	pub avatar: Option<BoundedVec<u8, AddressLimit>>,
}
