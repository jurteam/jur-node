use super::*;
use crate as pallet_token_swap;
use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU16, ConstU64},
};
use frame_system::{self as system, EnsureRoot};
use hex_literal::hex;
use primitives::{Balance, CurrencyId, JUR};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
type Block = frame_system::mocking::MockBlock<Test>;

pub const NATIVE_CURRENCY_ID: CurrencyId = JUR;
pub const VECHAIN_ROOT_HASH: VechainHash =
	hex!("e1a1226e0df5be016d753d53ff38d22e93d935c3cdeac637d69eed653c5c482e");

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Assets: pallet_assets,
		TokenSwap: pallet_token_swap,
	}
);

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
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type MaxHolds = ();
}

parameter_types! {
	pub const AssetDeposit: u64 = 1;
	pub const ApprovalDeposit: u64 = 1;
	pub const AssetAccountDeposit: u64 = 1;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: u64 = 1;
	pub const MetadataDepositPerByte: u64 = 1;
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = u32;
	type Balance = Balance;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
	type ForceOrigin = EnsureRoot<u64>;
	type AssetDeposit = AssetDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type AssetAccountDeposit = AssetAccountDeposit;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<5>;
	type CallbackHandle = ();
}

parameter_types! {
	pub Prefix: &'static [u8] = b"My JUR address is ";
	pub const MetaBlockNumber: u64 = 1;
	pub const NativeCurrencyId: CurrencyId = NATIVE_CURRENCY_ID;
	pub const EthAddress: EthereumAddress = EthereumAddress(hex!("876e6d95b8fca0a1adeef7fd5a6b521b16bc6969"));

}

impl pallet_token_swap::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type DepositContractAddress = EthAddress;
	type Prefix = Prefix;
	type Assets = Assets;
	type Balances = Balances;
	type NativeCurrencyId = NativeCurrencyId;
	type StorageRootOrigin = EnsureRoot<u64>;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap()
		.into()
}
