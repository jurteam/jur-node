use super::*;
use codec::{Decode, Encode};
use frame_support::{pallet_prelude::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_std::{prelude::*, vec::Vec};
use frame_support::pallet_prelude::{CloneNoBound, PartialEqNoBound, RuntimeDebugNoBound};

pub type CommunityMetaDataFor<T> =
	CommunityMetaData<<T as Config>::CustomLimit>;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
#[scale_info(skip_type_params(NameLimit, DescriptionLimit, TagLimit, ColorLimit, CustomLimit))]
pub struct Community<
	AccountId,
	NameLimit: Get<u32>,
	DescriptionLimit: Get<u32>,
	TagLimit: Get<u32>,
	ColorLimit: Get<u32>,
	CustomLimit: Get<u32>
> {
	pub founder: AccountId,
	pub logo: Option<Vec<u8>>,
	pub name: BoundedVec<u8, NameLimit>,
	pub description: BoundedVec<u8, DescriptionLimit>,
	pub members: Vec<AccountId>,
	pub metadata: Option<CommunityMetaData<CustomLimit>>,
	pub reference_id: [u8; 16],
	pub category: Category,
	pub tag: BoundedVec<u8, TagLimit>,
	pub primary_color: BoundedVec<u8, ColorLimit>,
	pub secondary_color: BoundedVec<u8, ColorLimit>,
	pub community_type: Option<CommunityType<AccountId>>,

}

#[derive(PartialEqNoBound, Eq, CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, Default)]
#[scale_info(skip_type_params(CustomLimit))]
pub struct CommunityMetaData<CustomLimit: Get<u32>> {
	pub customs: Option<Vec<Customs<CustomLimit>>>,
	pub languages: Option<Vec<Vec<u8>>>,
	pub norms: Option<Vec<Vec<u8>>>,
	pub religions: Option<Vec<Vec<u8>>>,
	pub territories: Option<Vec<Vec<u8>>>,
	pub traditions: Option<Vec<Vec<u8>>>,
	pub values: Option<Vec<Vec<u8>>>,
}

#[derive(PartialEqNoBound, Eq, CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(CustomLimit))]
pub struct Customs<CustomLimit: Get<u32>>(pub BoundedVec<u8, CustomLimit>);

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
pub struct State<AccountId> {
	pub constitution: Vec<Vec<u8>>,
	pub government: Vec<AccountId>,
	pub citizens: Vec<AccountId>,
}
/// Different types of Communities.
#[derive(Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, Encode, Decode)]
pub enum CommunityType<AccountId> {
	/// Community Type.
	Community,
	/// A community of entities and people united by a commonality.
	Nation,
	/// A State is the next step of a Nation.
	State(State<AccountId>),
}

#[derive(Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, Encode, Decode)]
pub enum Category {
	/// public.
	Public,
	/// A NFT Gated community.
	NFTGated,
}

impl<AccountId> Default for CommunityType<AccountId> {
	fn default() -> Self {
		Self::Community
	}
}

impl Default for Category {
	fn default() -> Self {
		Self::Public
	}
}
