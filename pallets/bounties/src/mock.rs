use crate as pallet_bounties;
use frame_support::pallet_prelude::Hooks;
use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU16, ConstU32, ConstU64},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
// use crate::Bounties;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip,
		Community: pallet_community,
		BountyPallet: pallet_bounties,
		Passport: pallet_passport,
		Whitelist: pallet_whitelist,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
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

impl pallet_insecure_randomness_collective_flip::Config for Test {}

impl pallet_community::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CommunityId = u32;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
	type NameLimit = ConstU32<50>;
	type DescriptionLimit = ConstU32<250>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type WeightInfo = ();
	type MyRandomness = RandomnessCollectiveFlip;
	type TagLimit = ConstU32<50>;
	type ColorLimit = ConstU32<7>;
	type CommunityLimit = ConstU32<3>;
	type StringLimit = ConstU32<250>;
	type LogoLimit = ConstU32<60>;
}

impl pallet_whitelist::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

impl pallet_passport::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type PassportId = u32;
	type BadgeNameLimit = ConstU32<20>;
	type DescriptionLimit = ConstU32<250>;
	type AddressLimit = ConstU32<60>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type WeightInfo = ();
}

impl pallet_bounties::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BountyId = u32;
	type NameLimit = ConstU32<512>;
	type DescriptionLimit = ConstU32<8192>;
	type CategoryLimit = ConstU32<20>;
	type AccountLimit = ConstU32<500>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	// type WeightInfo = ();
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

fn init_block() {
	System::on_initialize(System::block_number());
	BountyPallet::on_initialize(System::block_number());
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		let b = System::block_number();

		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
			BountyPallet::on_finalize(System::block_number());
		}

		System::set_block_number(b + 1);
		init_block();
	}
}
