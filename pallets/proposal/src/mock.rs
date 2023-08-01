use crate as pallet_proposal;
use frame_support::pallet_prelude::Hooks;
use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU16, ConstU32, ConstU64},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip::{Pallet, Storage},
		Community: pallet_community::{Pallet, Call, Storage, Event<T>},
		Proposal: pallet_proposal::{Pallet, Call, Storage, Event<T>},
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
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
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
	type TagLimit = ConstU32<250>;
	type ColorLimit = ConstU32<30>;
}

impl pallet_proposal::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ProposalId = u32;
	type ChoiceId = u32;
	type NameLimit = ConstU32<60>;
	type DescriptionLimit = ConstU32<250>;
	type LabelLimit = ConstU32<10>;
	type AccountLimit = ConstU32<3>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities = system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}

fn init_block() {
	System::on_initialize(System::block_number());
	Proposal::on_initialize(System::block_number());
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		let b = System::block_number();

		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
			Proposal::on_finalize(System::block_number());
		}

		System::set_block_number(b + 1);
		init_block();
	}
}
