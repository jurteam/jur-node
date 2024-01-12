use codec::{Decode, Encode};
use frame_support::{pallet_prelude::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(NameLimit, DescriptionLimit, BadgeNameLimit,))]
pub struct EventDetails<
	NameLimit: Get<u32>,
	DescriptionLimit: Get<u32>,
	BadgeNameLimit: Get<u32>,
	AccountId,
> {
	pub creator: AccountId,
	pub name: BoundedVec<u8, NameLimit>,
	pub description: BoundedVec<u8, DescriptionLimit>,
	pub start_time: u64,
	pub end_time: u64,
	pub event_type: EventType,
	pub venue: Option<BoundedVec<u8, NameLimit>>,
	pub badge: BoundedVec<u8, BadgeNameLimit>,
	pub attendees_list: Vec<AccountId>,
}

/// Types of Event.
#[derive(Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, Encode, Decode)]
pub enum EventType {
	/// Event is on physical venue.
	Physical,
	/// Event is online.
	Virtual,
}
