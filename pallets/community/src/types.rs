use super::*;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::{CloneNoBound, PartialEqNoBound, RuntimeDebugNoBound};
use frame_support::{pallet_prelude::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_std::{prelude::*, vec::Vec};

pub type CommunityMetaDataFor<T> = CommunityMetaData<<T as Config>::StringLimit>;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, Default)]
#[scale_info(skip_type_params(
	NameLimit,
	DescriptionLimit,
	TagLimit,
	ColorLimit,
	StringLimit,
	LogoLimit
))]
pub struct Community<
	AccountId,
	NameLimit: Get<u32>,
	DescriptionLimit: Get<u32>,
	TagLimit: Get<u32>,
	ColorLimit: Get<u32>,
	StringLimit: Get<u32>,
	LogoLimit: Get<u32>,
> {
	pub founder: AccountId,
	pub logo: BoundedVec<u8, LogoLimit>,
	pub name: BoundedVec<u8, NameLimit>,
	pub description: BoundedVec<u8, DescriptionLimit>,
	pub members: Vec<AccountId>,
	pub metadata: Option<CommunityMetaData<StringLimit>>,
	pub reference_id: [u8; 16],
	pub category: Category,
	pub tag: BoundedVec<u8, TagLimit>,
	pub primary_color: BoundedVec<u8, ColorLimit>,
	pub secondary_color: BoundedVec<u8, ColorLimit>,
	pub community_type: Option<CommunityType<AccountId>>,
}

#[derive(
	PartialEqNoBound, Eq, CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, Default,
)]
#[scale_info(skip_type_params(StringLimit))]
pub struct CommunityMetaData<StringLimit: Get<u32>> {
	pub customs: Option<Vec<Customs<StringLimit>>>,
	pub languages: Option<Vec<Languages<StringLimit>>>,
	pub norms: Option<Vec<Norms<StringLimit>>>,
	pub religions: Option<Vec<Religions<StringLimit>>>,
	pub territories: Option<Vec<Territories<StringLimit>>>,
	pub traditions: Option<Vec<Traditions<StringLimit>>>,
	pub values: Option<Vec<Values<StringLimit>>>,
}

#[derive(PartialEqNoBound, Eq, CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(StringLimit))]
pub struct Customs<StringLimit: Get<u32>>(pub BoundedVec<u8, StringLimit>);

#[derive(PartialEqNoBound, Eq, CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(StringLimit))]
pub struct Languages<StringLimit: Get<u32>>(pub BoundedVec<u8, StringLimit>);

#[derive(PartialEqNoBound, Eq, CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(StringLimit))]
pub struct Norms<StringLimit: Get<u32>>(pub BoundedVec<u8, StringLimit>);

#[derive(PartialEqNoBound, Eq, CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(StringLimit))]
pub struct Religions<StringLimit: Get<u32>>(pub BoundedVec<u8, StringLimit>);

#[derive(PartialEqNoBound, Eq, CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(StringLimit))]
pub struct Territories<StringLimit: Get<u32>>(pub BoundedVec<u8, StringLimit>);

#[derive(PartialEqNoBound, Eq, CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(StringLimit))]
pub struct Traditions<StringLimit: Get<u32>>(pub BoundedVec<u8, StringLimit>);

#[derive(PartialEqNoBound, Eq, CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(StringLimit))]
pub struct Values<StringLimit: Get<u32>>(pub BoundedVec<u8, StringLimit>);

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
