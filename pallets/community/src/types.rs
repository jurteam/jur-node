use super::*;
use codec::{Decode, Encode};
use frame_support::{pallet_prelude::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_std::{prelude::*, vec::Vec};

pub type CommunityMetaDataFor<T> =
	CommunityMetaData<<T as frame_system::Config>::AccountId, <T as frame_system::Config>::Hash>;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
#[scale_info(skip_type_params(NameLimit, DescriptionLimit))]
pub struct Community<AccountId, Hash, NameLimit: Get<u32>, DescriptionLimit: Get<u32>> {
	pub founder: AccountId,
	pub logo: Option<Vec<u8>>,
	pub name: BoundedVec<u8, NameLimit>,
	pub description: BoundedVec<u8, DescriptionLimit>,
	pub members: Vec<AccountId>,
	pub metadata: Option<CommunityMetaData<AccountId, Hash>>,
	pub reference_id: Hash,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
pub struct CommunityMetaData<AccountId, Hash> {
	pub community_type: Option<CommunityType<AccountId, Hash>>,
	pub customs: Option<Vec<Vec<u8>>>,
	pub languages: Option<Vec<Vec<u8>>>,
	pub norms: Option<Vec<Hash>>,
	pub religions: Option<Vec<Vec<u8>>>,
	pub territories: Option<Vec<Vec<u8>>>,
	pub traditions: Option<Vec<Vec<u8>>>,
	pub values: Option<Vec<Vec<u8>>>,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
pub struct State<AccountId, Hash> {
	pub constitution: Vec<Hash>,
	pub government: Vec<AccountId>,
	pub citizens: Vec<AccountId>,
}
/// Different types of Communities.
#[derive(Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, Encode, Decode)]
pub enum CommunityType<AccountId, Hash> {
	/// Community Type.
	Community,
	/// A community of entities and people united by a commonality.
	Nation,
	/// A State is the next step of a Nation.
	State(State<AccountId, Hash>),
}

impl<AccountId, Hash> Default for CommunityType<AccountId, Hash> {
	fn default() -> Self {
		Self::Community
	}
}
