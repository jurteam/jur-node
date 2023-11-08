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
//!    - if it’s historical or not
//!    - the ask/question to the other Members
//! * A member can vote on an existing proposal
//!
//! ## Interface
//!
//! * `create_proposal`
//! * `cast_vote`
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod types;
use crate::types::{Choice, Proposal, ProposalResultStatus, Vote};
use frame_support::{dispatch::DispatchResultWithPostInfo, BoundedVec};
use primitives::{Incrementable, BLOCKS_PER_DAY, PROPOSAL_DURATION_LIMIT};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub mod migration;

pub use weights::WeightInfo;

const LOG_TARGET: &str = "runtime::proposal";

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

		/// The maximum length of proposal name/title.
		#[pallet::constant]
		type NameLimit: Get<u32>;

		/// The maximum length of proposal description.
		#[pallet::constant]
		type DescriptionLimit: Get<u32>;

		/// The maximum length of choice label.
		#[pallet::constant]
		type LabelLimit: Get<u32>;

		/// The maximum length of address.
		#[pallet::constant]
		type AccountLimit: Get<u32>;

		#[cfg(feature = "runtime-benchmarks")]
		/// A set of helper functions for benchmarking.
		type Helper: BenchmarkHelper<Self::ProposalId, Self::ChoiceId>;

		/// Weight information
		type WeightInfo: WeightInfo;
	}

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::storage_version(STORAGE_VERSION)]
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
		Proposal<
			<T as Config>::DescriptionLimit,
			<T as pallet::Config>::NameLimit,
			T::AccountId,
			T::AccountLimit,
		>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn proposal_expire)]
	pub type ProposalExpireTime<T: Config> =
		StorageMap<_, Identity, BlockNumberFor<T>, (T::ProposalId, T::CommunityId), OptionQuery>;

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
	pub type Votes<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::ChoiceId,
		Vote<BlockNumberFor<T>, T::AccountId, T::AccountLimit>,
		OptionQuery,
	>;

	/// Stores the `ProposalId` that is going to be used for the next proposal.
	/// This gets incremented whenever a new proposal is created.
	#[pallet::storage]
	pub type NextProposalId<T: Config> =
		StorageMap<_, Twox64Concat, T::CommunityId, T::ProposalId, OptionQuery>;

	/// Stores the `ChoiceId` that is going to be used for the next choice.
	/// This gets incremented whenever a new choice is created.
	#[pallet::storage]
	pub(super) type NextChoiceId<T: Config> = StorageValue<_, T::ChoiceId, OptionQuery>;

	/// Store the `Proposal Result`
	#[pallet::storage]
	pub(super) type ProposalResult<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::ProposalId,
		(ProposalResultStatus, Vote<BlockNumberFor<T>, T::AccountId, T::AccountLimit>),
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Created Proposals [Proposal Id]
		CreatedProposal(T::ProposalId),
		/// Submitted Proposal [Proposal Id]
		VoteCasted(T::ProposalId),
		/// Proposal state changed [Proposal Id]
		ProposalStateChanged(T::ProposalId),
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
		/// Proposal got inactive.
		ProposalNotActive,
		/// Duplicate vote.
		DuplicateVote,
		/// Vote Not found for given choice Id.
		VotesNotFound,
		/// New account can't be added due to account limit.
		AccountLimitReached,
		/// Invalid Proposal duration.
		InvalidProposalDuration,
		/// Invalid Choices given during creating proposal.
		InvalidChoicesGiven,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
			let option_proposal_expire = ProposalExpireTime::<T>::get(block_number);

			if let Some((proposal_id, community_id)) = option_proposal_expire {
				Proposals::<T>::try_mutate(
					community_id,
					proposal_id,
					|proposal_detail| -> DispatchResult {
						let proposal_data = proposal_detail
							.as_mut()
							.ok_or(Error::<T>::ProposalDoesNotExist)?;

						// Add the proposalResult storage to add the result after the deadline of proposal voting.
						let voters_count = &proposal_data.voter_accounts.len();

						// find all the choice id's for the current proposal.
						// iterate for all the choice id's and get the total no of votes for it.
						let choice_ids =
							Choices::<T>::get(proposal_id).ok_or(Error::<T>::ChoiceDoesNotExist)?;

						// fetching the vote information for both choices.
						let yes_vote_info =
							Votes::<T>::get(choice_ids[0].id).ok_or(Error::<T>::VotesNotFound)?;
						let no_vote_info =
							Votes::<T>::get(choice_ids[1].id).ok_or(Error::<T>::VotesNotFound)?;

						// Inserting the proposal result according to the voting.
						// If 51% or more then from all voters voted in favour of proposal
						// then proposal is Accepted.
						// Otherwise the proposal is rejected.
						if yes_vote_info.vote_count > (*voters_count as u64) / 2 {
							ProposalResult::<T>::insert(
								proposal_id,
								(ProposalResultStatus::Accepted, yes_vote_info),
							);
						} else {
							ProposalResult::<T>::insert(
								proposal_id,
								(ProposalResultStatus::Rejected, no_vote_info),
							);
						}

						proposal_data.status = false;

						Self::deposit_event(Event::<T>::ProposalStateChanged(proposal_id));

						Ok(())
					},
				)
				.expect("Proposal not found");
			}
			Weight::zero()
		}
	}

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
		/// - `name`: name/title of the proposal.
		/// - `description`: description of the proposal.
		/// - `choices`: Choices for a given proposal.
		/// - `is_historical`: A Proposal can be marked as historical.
		/// - `proposal_duration`: Voting duration of the proposal.
		/// 			In case it is flagged as such, the proposal becomes part of the History.
		///
		/// Emits `CreatedProposal` event when successful.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::create_proposal())]
		pub fn create_proposal(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			name: BoundedVec<u8, <T as pallet::Config>::NameLimit>,
			description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit>,
			choices: Vec<Vec<u8>>,
			is_historical: bool,
			proposal_duration: u32,
		) -> DispatchResultWithPostInfo {
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;

			let origin = ensure_signed(origin)?;
			ensure!(origin == community.founder, Error::<T>::NotAllowed);

			ensure!(choices.len() >= 2, Error::<T>::InvalidChoicesGiven);
			ensure!(
				(1..=PROPOSAL_DURATION_LIMIT).contains(&proposal_duration),
				Error::<T>::InvalidProposalDuration
			);

			Self::do_create_proposal(
				origin,
				community_id,
				name,
				description,
				choices,
				is_historical,
				proposal_duration,
			)
		}

		/// cast a vote for a proposal.
		///
		/// The origin must be Signed and the member of the community.
		///
		/// Parameters:
		/// - `community_id`: Id of the community.
		/// - `proposal_id`: Id of the proposal.
		/// - `choice_id`: Id of the choice.
		///
		/// Emits `cast_vote` event when successful.
		///
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::cast_vote())]
		pub fn cast_vote(
			origin: OriginFor<T>,
			community_id: T::CommunityId,
			proposal_id: T::ProposalId,
			choice: BoundedVec<u8, T::LabelLimit>,
		) -> DispatchResultWithPostInfo {
			let community = pallet_community::Communities::<T>::get(community_id)
				.ok_or(Error::<T>::CommunityDoesNotExist)?;
			let origin = ensure_signed(origin)?;

			ensure!(community.members.contains(&origin), Error::<T>::NotAllowed);

			let proposal = Proposals::<T>::get(community_id, proposal_id)
				.ok_or(Error::<T>::ProposalDoesNotExist)?;

			ensure!(Choices::<T>::contains_key(proposal_id), Error::<T>::NoChoiceAvailable);

			ensure!(proposal.status, Error::<T>::ProposalNotActive);

			ensure!(!(proposal.voter_accounts).contains(&origin), Error::<T>::DuplicateVote);

			// Get all the choices id from the current proposal and
			// check if current choice_id is already present or not?
			let proposal_choices =
				Choices::<T>::get(proposal_id).ok_or(Error::<T>::NoChoiceAvailable)?;

			proposal_choices
				.into_iter()
				.find(|choices| choices.label == choice)
				.ok_or(Error::<T>::ChoiceDoesNotExist)?;

			let proposal_choices =
				Choices::<T>::get(proposal_id).ok_or(Error::<T>::NoChoiceAvailable)?;

			let mut choice_id = proposal_choices[0].id;

			if proposal_choices[1].label == choice {
				choice_id = proposal_choices[1].id;
			}

			// Adding the vote to the storage.
			Votes::<T>::mutate(choice_id, |optional_vote| -> DispatchResult {
				let vote = optional_vote.as_mut().ok_or(Error::<T>::VotesNotFound)?;
				vote.who
					.try_push(origin.clone())
					.ok()
					.ok_or(Error::<T>::AccountLimitReached)?;
				*optional_vote = Some(Vote {
					who: vote.who.clone(),
					vote_count: vote.vote_count + 1,
					last_voted: <frame_system::Pallet<T>>::block_number(),
				});
				Ok(())
			})?;

			// Add this account in voter_accounts list.
			Proposals::<T>::mutate(
				community_id,
				proposal_id,
				|proposal_details| -> DispatchResult {
					let proposal_info = proposal_details
						.as_mut()
						.ok_or(Error::<T>::ProposalDoesNotExist)?;

					proposal_info
						.voter_accounts
						.try_push(origin.clone())
						.ok()
						.ok_or(Error::<T>::AccountLimitReached)?;

					Ok(())
				},
			)?;

			Self::deposit_event(Event::VoteCasted(proposal_id));
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn do_create_proposal(
		proposer_account: T::AccountId,
		community_id: T::CommunityId,
		name: BoundedVec<u8, <T as pallet::Config>::NameLimit>,
		description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit>,
		choices: Vec<Vec<u8>>,
		is_historical: bool,
		proposal_duration: u32,
	) -> DispatchResultWithPostInfo {
		let bounded_account: BoundedVec<T::AccountId, <T as Config>::AccountLimit> = Vec::new()
			.clone()
			.try_into()
			.map_err(|_| Error::<T>::AccountLimitReached)?;

		let new_proposal = Proposal {
			proposer: proposer_account.clone(),
			name,
			description,
			historical: is_historical,
			status: true,
			voter_accounts: bounded_account.clone(),
		};

		let proposal_id = NextProposalId::<T>::get(community_id).unwrap_or(T::ProposalId::initial_value());

		let new_choices: Vec<Choice<T::ChoiceId, <T as Config>::LabelLimit>> = choices
			.clone()
			.into_iter()
			.map(|choice| {
				let bounded_choice: BoundedVec<u8, <T as Config>::LabelLimit> =
					choice.try_into().expect("Choice Label Limit Exceeded.");

				let choice_id: T::ChoiceId =
					NextChoiceId::<T>::get().unwrap_or(T::ChoiceId::initial_value());
				let vote = Vote {
					who: bounded_account.clone(),
					vote_count: 0,
					last_voted: <frame_system::Pallet<T>>::block_number(),
				};
				<Votes<T>>::insert(choice_id, vote);

				let next_choice_id = choice_id.increment();
				NextChoiceId::<T>::set(Some(next_choice_id));
				Choice { id: choice_id, label: bounded_choice }
			})
			.collect::<Vec<_>>();

		// Storing the proposal
		<Proposals<T>>::insert(community_id, proposal_id, &new_proposal);

		// Set up the expire time of a particular proposal with community id.
		let total_block: u32 = BLOCKS_PER_DAY * proposal_duration;

		let expire_block = frame_system::Pallet::<T>::block_number() + total_block.into();
		ProposalExpireTime::<T>::insert(expire_block, (proposal_id, community_id));

		let next_proposal_id = proposal_id.increment();
		NextProposalId::<T>::insert(community_id, next_proposal_id);

		// Storing choices
		if !choices.is_empty() {
			<Choices<T>>::insert(proposal_id, new_choices);
		}
		Self::deposit_event(Event::CreatedProposal(proposal_id));

		Ok(().into())
	}
}
