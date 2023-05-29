//! Benchmarking setup for pallet-proposal

use super::*;

#[allow(unused)]
use crate::Pallet as Proposal;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use pallet_community::types::{CommunityMetaData, CommunityType};
use sp_std::vec;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn get_community_metadata<T: Config>() -> CommunityMetaData<T::AccountId, T::Hash> {
	CommunityMetaData {
		community_type: CommunityType::Nation,
		customs: vec![
			"in public transport young people should leave the seat to elderly or pregnant women"
				.into(),
			"name newborns with a name that starts with the letter A".into(),
		],
		languages: vec!["English".into(), "German".into()],
		norms: vec![],
		religions: vec!["Christianity".into(), "Buddhism".into()],
		territories: vec!["Mars".into()],
		traditions: vec![
			"Exchange gifts for Christmas".into(),
			"Organize one charity event every 100 blocks".into(),
		],
		values: vec!["Peace".into(), "No gender discrimination".into()],
	}
}

fn create_community<T: Config>(caller: T::AccountId) -> T::CommunityId {
	let community_id =
		pallet_community::NextCommunityId::<T>::get().unwrap_or(T::CommunityId::initial_value());

	let members = Some(vec![account("sub", 1, SEED)]);

	pallet_community::Pallet::<T>::create_community(
		RawOrigin::Signed(caller).into(),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".as_bytes().to_vec(),
		Some("Jur is the core community of the Jur ecosystem, which includes all the contributors."
			.into()),
		members,
		Some(get_community_metadata::<T>()),
	)
	.unwrap();

	community_id
}

fn add_proposal<T: Config>(caller: T::AccountId) -> (T::CommunityId, T::ProposalId, T::ChoiceId) {
	let proposal_id = NextProposalId::<T>::get().unwrap_or(T::ProposalId::initial_value());

	let community_id = create_community::<T>(caller.clone());
	let proposal_address: Vec<u8> =
		"abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
	let bounded_proposal_address: BoundedVec<u8, <T as pallet::Config>::AddressLimit> =
		proposal_address.try_into().unwrap();

	Proposal::<T>::create_proposal(
		RawOrigin::Signed(caller).into(),
		community_id,
		bounded_proposal_address,
		"Which is your native country".into(),
		vec![
			"India".as_bytes().to_vec(),
			"Germany".as_bytes().to_vec(),
			"England".as_bytes().to_vec(),
		],
		false,
		5,
	)
	.unwrap();

	let choice_id = Choices::<T>::get(proposal_id).unwrap()[0].id;

	(community_id, proposal_id, choice_id)
}

benchmarks! {
	create_proposal {
		let caller: T::AccountId = whitelisted_caller();
		let community_id = create_community::<T>(caller.clone());
		let proposal_address: Vec<u8> = "abcdreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into();
		let bounded_proposal_address: BoundedVec<u8, <T as pallet::Config>::AddressLimit> = proposal_address.try_into().unwrap();
		let proposal: Vec<u8> = "Which language should we speak within the Community?".into();
	}: _(
		RawOrigin::Signed(caller),
		community_id,
		bounded_proposal_address,
		proposal.clone(),
		vec!["English".as_bytes().to_vec(), "Ghukliak".as_bytes().to_vec(), "官话".as_bytes().to_vec(), "Rust".as_bytes().to_vec()],
		false,
		5
	)
	verify {
		assert_last_event::<T>(Event::<T>::CreatedProposal(proposal).into());
	}

	submit_choice {
		let caller: T::AccountId = whitelisted_caller();
		let member = account("sub", 1, SEED);
		let (community_id, proposal_id, choice_id) = add_proposal::<T>(caller.clone());

	}: _(RawOrigin::Signed(member), community_id, proposal_id, choice_id)
	verify {
		assert_last_event::<T>(Event::<T>::SubmittedChoice.into());
	}

	impl_benchmark_test_suite!(Proposal, crate::mock::new_test_ext(), crate::mock::Test);
}
