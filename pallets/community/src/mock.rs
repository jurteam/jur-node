use crate as pallet_community;
use crate::{Category, CommunityMetaData, CommunityType, Customs, Languages, Religions, Territories, Traditions, Values};
use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU16, ConstU32, ConstU64},
};
use frame_support_test::TestRandomness;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, Header as _, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Community: pallet_community,
		Whitelist: pallet_whitelist,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_community::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CommunityId = u32;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
	type NameLimit = ConstU32<50>;
	type DescriptionLimit = ConstU32<250>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type WeightInfo = ();
	type MyRandomness = TestRandomness<Self>;
	type TagLimit = ConstU32<50>;
	type ColorLimit = ConstU32<7>;
	type CommunityLimit = ConstU32<3>;
	type StringLimit = ConstU32<250>;
}

impl pallet_whitelist::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities = system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap()
		.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn setup_blocks(blocks: u64) {
	let mut parent_hash = System::parent_hash();

	for i in 1..(blocks + 1) {
		System::reset_events();
		System::initialize(&i, &parent_hash, &Default::default());

		let header = System::finalize();
		parent_hash = header.hash();
		System::set_block_number(*header.number());
	}
}

pub fn get_metadata() -> CommunityMetaData<ConstU32<250>> {
	let custom_one: Vec<u8> = "in public transport young people should leave the seat to elderly or pregnant women"
		.into();
	let custom_two: Vec<u8> = "name newborns with a name that starts with the letter A".into();

	let languages_1: Vec<u8> = "English".into();
	let languages_2: Vec<u8> = "German".into();

	let religions_1: Vec<u8> = "Christianity".into();
	let religions_2: Vec<u8> = "Buddhism".into();

	let territories: Vec<u8> = "Mars".into();

	let traditions_1: Vec<u8> = "Exchange gifts for Christmas".into();
	let traditions_2: Vec<u8> = "Organize one charity event every 100 blocks".into();

	let values_1: Vec<u8> = "Peace".into();
	let values_2: Vec<u8> = "No gender discrimination".into();

	let community_metadata = CommunityMetaData {
		customs: Some(vec![
			Customs(custom_one.try_into().unwrap()),
			Customs(custom_two.try_into().unwrap()),
		]),
		languages: Some(vec![
			Languages(languages_1.try_into().unwrap()),
			Languages(languages_2.try_into().unwrap())
		]),
		norms: Some(vec![]),
		religions: Some(vec![
			Religions(religions_1.try_into().unwrap()),
			Religions(religions_2.try_into().unwrap())
		]),
		territories: Some(vec![Territories(territories.try_into().unwrap())]),
		traditions: Some(vec![
			Traditions(traditions_1.try_into().unwrap()),
			Traditions(traditions_2.try_into().unwrap())
		]),
		values: Some(vec![
			Values(values_1.try_into().unwrap()),
			Values(values_2.try_into().unwrap())
		]),
	};

	community_metadata
}

pub fn add_founder() {
	Whitelist::add_founder(RuntimeOrigin::root(), 1).unwrap();
}
pub fn create_community() {
	Community::create_community(
		RuntimeOrigin::signed(1),
		// hash of IPFS path of dummy logo
		Some("bafkreifec54rzopwm6mvqm3fknmdlsw2yefpdr7xrgtsron62on2nynegq".into()),
		"Jur".into(),
		Some(
			"Jur is the core community of the Jur ecosystem, which includes all the contributors."
				.into(),
		),
		Some(vec![1, 2]),
		Some(get_metadata()),
		Category::Public,
		Some("tag".into()),
		Some("#222307".into()),
		Some("#E76080".into()),
		Some(CommunityType::Nation)
	)
	.unwrap();
}
