//! # Jur Proposal Pallet
//!
//! A pallet allow Members to make and vote on proposals that can shape the identity and
//! values of the community
//!
//! ## Overview
//!
//! A Proposal is a way for the Community to propose a change in any of the core values that
//! make the Community stick together.
//!
//! A Proposal can be of different types e.g. custom, language, etc. and the outcome sets
//! a different value within that property of the Community data structure.
//!
//! ## Functionalities
//!
//! * A founder can create a new proposal for a particular community and specify:
//! 		- if it’s historical or not
//! 		- the ask/question to the other Members
//! * A member can vote on an existing proposal
//!
//! ## Interface
//!
//! * `create_proposal`
//! * `submit_choice`
//!


#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod types;
use crate::types::{Choice, Proposal, Vote};
use frame_support::{dispatch::DispatchResultWithPostInfo, BoundedVec};
use primitives::Incrementable;
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[cfg(feature = "runtime-benchmarks")]
	pub trait BenchmarkHelper<ProposalId, ChoiceId> {
		fn proposal(i: u32) -> ProposalId;
		fn choice(i: u32) -> ChoiceId;
	}
	#[cfg(feature = "runtime-benchmarks")]
	impl<ProposalId: From<u32>, ChoiceId: From<u32>> BenchmarkHelper<ProposalId, ChoiceId> for () {
		fn proposal(i: u32) -> ProposalId {
			i.into()
		}
		fn choice(i: u32) -> ChoiceId {
			i.into()
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it
	/// depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_community::Config {
		/// Because this pallet emits events, it depends on the runtime's
		/// definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Identifier for the Proposal.
		type ProposalId: Member + Parameter + MaxEncodedLen + Copy + Incrementable;

		/// Identifier for the Choice.
		type ChoiceId: Member + Parameter + MaxEncodedLen + Copy + Incrementable;

		/// The maximum length of any description.
		#[pallet::constant]
		type DescriptionLimit: Get<u32>;

		/// The maximum length of any label.
		#[pallet::constant]
		type LabelLimit: Get<u32>;

		#[cfg(feature = "runtime-benchmarks")]
		/// A set of helper functions for benchmarking.
		type Helper: BenchmarkHelper<Self::ProposalId, Self::ChoiceId>;

		/// Weight information
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Store new proposal with a unique proposal id for a particular community
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type Proposals<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CommunityId,
		Blake2_128Concat,
		T::ProposalId,
		Proposal<<T as Config>::DescriptionLimit>,
		OptionQuery,
	>;

	/// Store Choices for a particular proposal
	#[pallet::storage]
	#[pallet::getter(fn choices)]
	pub type Choices<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::ProposalId,
		Vec<Choice<T::ChoiceId, <T as Config>::LabelLimit>>,
		OptionQuery,
	>;

	/// Store votes submitted for a choice
	#[pallet::storage]
	#[pallet::getter(fn votes)]
	pub type Votes<T: Config> =
		StorageMap<_, Blake2_128Concat, T::ChoiceId, Vote<T::BlockNumber>, ValueQuery>;

	/// Stores the `ProposalId` that is going to be used for the next proposal.
	/// This gets incremented whenever a new proposal is created.
	#[pallet::storage]
	pub(super) type NextProposalId<T: Config> = StorageValue<_, T::ProposalId, OptionQuery>;

	/// Stores the `ChoiceId` that is going to be used for the next choice.
	/// This gets incremented whenever a new choice is created.
	#[pallet::storage]
	pub(super) type NextChoiceId<T: Config> = StorageValue<_, T::ChoiceId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Created Proposals [description]
		CreatedProposal(Vec<u8>),
		/// Submitted Proposal
		SubmittedChoice,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Proposal Does Not Exist.
		ProposalDoesNotExist,
		/// No Choice Available
		NoChoiceAvailable,
		/// Choice Does Not Exist
		ChoiceDoesNotExist,
		/// Community Does Not Exist.
		CommunityDoesNotExist,
		/// NotAllowed
		NotAllowed,
		/// Invalid description given.
		BadDescription,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new proposal for a particular community from a origin.
		///
		/// This new proposal has choices with zero votes.
		///
		/// The origin must be Signed and the community founder.
		///
		/// Parameters:
		/// - `community_id`: Id of the community.
		/// - `proposal`: A proposal like `Which language should we speak within the Community?`.
		/// - `choices`: Choices for a given proposal.
		/// - `is_historical`: A Proposal can be marked as historical.
		/// 			In case it is flagged as such, the proposal becomes part of the History.
		///
		/// Emits `CreatedProposal` event when successful.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::create_proposal())]
		pub fn create_proposal(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			proposal: Vec<u8>,
			choices: Vec<Vec<u8>>,
			is_historical: bool,
		) -> DispatchResultWithPostInfo {
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;
			let origin = ensure_signed(origin)?;
			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			Self::do_create_proposal(community_id, proposal, choices, is_historical)
		}

		/// Submit a choice for a proposal.
		///
		/// The origin must be Signed and the member of the community.
		///
		/// Parameters:
		/// - `community_id`: Id of the community.
		/// - `proposal_id`: Id of the proposal.
		/// - `choice_id`: Id of the coice.
		///
		/// Emits `SubmittedChoice` event when successful.
		///
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::submit_choice())]
		pub fn submit_choice(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			proposal_id: T::ProposalId,
			choice_id: T::ChoiceId,
		) -> DispatchResultWithPostInfo {
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;
			let origin = ensure_signed(origin)?;

			ensure!(community.members.contains(&origin), Error::<T>::NotAllowed);

			ensure!(
				Proposals::<T>::contains_key(community_id, proposal_id),
				Error::<T>::ProposalDoesNotExist
			);
			ensure!(Choices::<T>::contains_key(proposal_id), Error::<T>::NoChoiceAvailable);
			ensure!(Votes::<T>::contains_key(choice_id), Error::<T>::ChoiceDoesNotExist);

			Votes::<T>::try_mutate(choice_id, |vote| -> DispatchResult {
				let new_count = vote.vote_count + 1;
				*vote = Vote {
					vote_count: new_count,
					last_voted: <frame_system::Pallet<T>>::block_number(),
				};
				Ok(())
			})?;

			Self::deposit_event(Event::SubmittedChoice);
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn do_create_proposal(
		community_id: T::CommunityId,
		proposal: Vec<u8>,
		choices: Vec<Vec<u8>>,
		is_historical: bool,
	) -> DispatchResultWithPostInfo {
		let bounded_proposal: BoundedVec<u8, <T as Config>::DescriptionLimit> = proposal
			.clone()
			.try_into()
			.map_err(|_| Error::<T>::BadDescription)?;

		let new_proposal =
			Proposal { description: bounded_proposal.clone(), historical: is_historical };

		let proposal_id = NextProposalId::<T>::get().unwrap_or(T::ProposalId::initial_value());

		let new_choices: Vec<Choice<T::ChoiceId, <T as Config>::LabelLimit>> = choices
			.clone()
			.into_iter()
			.map(|choice| {
				let bounded_choice: BoundedVec<u8, <T as Config>::LabelLimit> =
					choice.try_into().expect("Invalid choice given.");

				let choice_id: T::ChoiceId =
					NextChoiceId::<T>::get().unwrap_or(T::ChoiceId::initial_value());
				let vote =
					Vote { vote_count: 0, last_voted: <frame_system::Pallet<T>>::block_number() };
				<Votes<T>>::insert(choice_id, vote);

				let next_choice_id = choice_id.increment();
				NextChoiceId::<T>::set(Some(next_choice_id));
				Choice { id: choice_id, label: bounded_choice }
			})
			.collect::<Vec<_>>();

		// Storing the proposal
		<Proposals<T>>::insert(community_id, proposal_id, new_proposal);

		let next_proposal_id = proposal_id.increment();
		NextProposalId::<T>::set(Some(next_proposal_id));
		// Storing choices
		if !choices.is_empty() {
			<Choices<T>>::insert(proposal_id, new_choices);
		}
		Self::deposit_event(Event::CreatedProposal(proposal));

		Ok(().into())
	}
}
