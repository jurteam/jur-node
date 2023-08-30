//! Benchmarking setup for pallet-proposal

use super::*;

#[allow(unused)]
use crate::Pallet as Proposal;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use pallet_community::types::{Category, CommunityMetaData, CommunityType};
use sp_std::vec;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn get_community_metadata<T: Config>() -> CommunityMetaData<T::AccountId> {
	CommunityMetaData {
		community_type: Some(CommunityType::Nation),
		customs: Some(vec![
			"in public transport young people should leave the seat to elderly or pregnant women"
				.into(),
			"name newborns with a name that starts with the letter A".into(),
		]),
		languages: Some(vec!["English".into(), "German".into()]),
		norms: Some(vec![]),
		religions: Some(vec!["Christianity".into(), "Buddhism".into()]),
		territories: Some(vec!["Mars".into()]),
		traditions: Some(vec![
			"Exchange gifts for Christmas".into(),
			"Organize one charity event every 100 blocks".into(),
		]),
		values: Some(vec!["Peace".into(), "No gender discrimination".into()]),
	}
}

pub fn add_founder<T: Config>(caller: T::AccountId) {
	pallet_whitelist::Pallet::<T>::add_founder(RawOrigin::Root.into(), caller).unwrap();
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
		Some(
			"Jur is the core community of the Jur ecosystem, which includes all the contributors."
				.into(),
		),
		members,
		Some(get_community_metadata::<T>()),
		Category::Public,
		Some("tag".into()),
		Some("#222307".into()),
		Some("#E76080".into()),
	)
	.unwrap();

	community_id
}

fn add_proposal<T: Config>(caller: T::AccountId) -> (T::CommunityId, T::ProposalId, T::ChoiceId) {
	let proposal_id = NextProposalId::<T>::get().unwrap_or(T::ProposalId::initial_value());

	add_founder::<T>(caller.clone());
	let community_id = create_community::<T>(caller.clone());

	let proposal_name: Vec<u8> = "Jur community Language proposal".into();
	let bounded_proposal_name: BoundedVec<u8, <T as pallet::Config>::NameLimit> =
		proposal_name.try_into().unwrap();

	let proposal_description: Vec<u8> = "Description of Jur community Language proposal".into();
	let bounded_proposal_description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit> =
		proposal_description.try_into().unwrap();

	Proposal::<T>::create_proposal(
		RawOrigin::Signed(caller).into(),
		community_id,
		bounded_proposal_name,
		bounded_proposal_description,
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
		add_founder::<T>(caller.clone());
		let community_id = create_community::<T>(caller.clone());

		let proposal_name: Vec<u8> = "Jur community Language proposal".into();
		let bounded_proposal_name: BoundedVec<u8, <T as pallet::Config>::NameLimit> =
		proposal_name.try_into().unwrap();

		let proposal_description: Vec<u8> = "Description of Jur community Language proposal".into();
		let bounded_proposal_description: BoundedVec<u8, <T as pallet::Config>::DescriptionLimit> =
		proposal_description.try_into().unwrap();

	}: _(
		RawOrigin::Signed(caller),
		community_id,
		bounded_proposal_name,
		bounded_proposal_description,
		vec![
			"English".as_bytes().to_vec(),
			"Ghukliak".as_bytes().to_vec(),
			"官话".as_bytes().to_vec(),
			"Rust".as_bytes().to_vec()
		],
		false,
		5
	)
	verify {
		assert_last_event::<T>(Event::<T>::CreatedProposal(<T as pallet::Config>::Helper::proposal(1)).into());
	}

	cast_vote {
		let caller: T::AccountId = whitelisted_caller();
		let member = account("sub", 1, SEED);
		let (community_id, proposal_id, choice_id) = add_proposal::<T>(caller.clone());

		let choice: Vec<u8> = "India".into();
		let bounded_choice: BoundedVec<u8, <T as pallet::Config>::LabelLimit> =
		choice.try_into().unwrap();

	}: _(RawOrigin::Signed(member), community_id, proposal_id, bounded_choice)
	verify {
		assert_last_event::<T>(Event::<T>::VoteCasted(proposal_id).into());
	}

	impl_benchmark_test_suite!(Proposal, crate::mock::new_test_ext(), crate::mock::Test);
}
