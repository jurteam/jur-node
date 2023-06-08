
//! Autogenerated weights for pallet_passport
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Ayushs-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/jur-node
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet-passport
// --extrinsic=*
// --steps=50
// --repeat=20
// --template=./.maintain/frame-weight-template.hbs
// --output=./pallets/passport/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_passport.
pub trait WeightInfo {
	fn mint() -> Weight;
	fn update_passport() -> Weight;
	fn add_stamps() -> Weight;
	fn update_avatar() -> Weight;
}

/// Weights for pallet_passport using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Community Communities (r:1 w:0)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Passport Passports (r:1 w:1)
	/// Proof Skipped: Passport Passports (max_values: None, max_size: None, mode: Measured)
	/// Storage: Passport NextPassportId (r:1 w:1)
	/// Proof Skipped: Passport NextPassportId (max_values: Some(1), max_size: None, mode: Measured)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `667`
		//  Estimated: `10416`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(20_000_000, 10416)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Community Communities (r:1 w:0)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Passport Passports (r:1 w:1)
	/// Proof Skipped: Passport Passports (max_values: None, max_size: None, mode: Measured)
	fn update_passport() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `791`
		//  Estimated: `8512`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(22_000_000, 8512)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:0)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Passport Passports (r:1 w:1)
	/// Proof Skipped: Passport Passports (max_values: None, max_size: None, mode: Measured)
	fn add_stamps() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `791`
		//  Estimated: `8512`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(22_000_000, 8512)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:0)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Passport Passports (r:1 w:1)
	/// Proof Skipped: Passport Passports (max_values: None, max_size: None, mode: Measured)
	fn update_avatar() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `791`
		//  Estimated: `8512`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(21_000_000, 8512)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Community Communities (r:1 w:0)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Passport Passports (r:1 w:1)
	/// Proof Skipped: Passport Passports (max_values: None, max_size: None, mode: Measured)
	/// Storage: Passport NextPassportId (r:1 w:1)
	/// Proof Skipped: Passport NextPassportId (max_values: Some(1), max_size: None, mode: Measured)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `667`
		//  Estimated: `10416`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(20_000_000, 10416)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Community Communities (r:1 w:0)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Passport Passports (r:1 w:1)
	/// Proof Skipped: Passport Passports (max_values: None, max_size: None, mode: Measured)
	fn update_passport() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `791`
		//  Estimated: `8512`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(22_000_000, 8512)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:0)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Passport Passports (r:1 w:1)
	/// Proof Skipped: Passport Passports (max_values: None, max_size: None, mode: Measured)
	fn add_stamps() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `791`
		//  Estimated: `8512`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(22_000_000, 8512)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:0)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Passport Passports (r:1 w:1)
	/// Proof Skipped: Passport Passports (max_values: None, max_size: None, mode: Measured)
	fn update_avatar() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `791`
		//  Estimated: `8512`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(21_000_000, 8512)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}