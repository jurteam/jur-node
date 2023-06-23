
//! Autogenerated weights for pallet_community
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/jur-node
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet-community
// --extrinsic=*
// --steps=50
// --repeat=20
// --template=./.maintain/frame-weight-template.hbs
// --output=./pallets/community/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_community.
pub trait WeightInfo {
	fn create_community() -> Weight;
	fn delete_community() -> Weight;
	fn update_community() -> Weight;
	fn update_metadata() -> Weight;
	fn add_members() -> Weight;
	fn join_community() -> Weight;
}

/// Weights for pallet_community using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Community NextCommunityId (r:1 w:1)
	/// Proof Skipped: Community NextCommunityId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Community Communities (r:0 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn create_community() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1569`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(16_000_000, 1569)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Community Communities (r:1 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn delete_community() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `4098`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(16_000_000, 4098)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn update_community() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `4098`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(16_000_000, 4098)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn update_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `4098`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 4098)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn add_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `4098`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(17_000_000, 4098)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn join_community() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `4098`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(16_000_000, 4098)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Community NextCommunityId (r:1 w:1)
	/// Proof Skipped: Community NextCommunityId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Community Communities (r:0 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn create_community() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1569`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(16_000_000, 1569)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Community Communities (r:1 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn delete_community() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `4098`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(16_000_000, 4098)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn update_community() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `4098`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(16_000_000, 4098)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn update_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `4098`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 4098)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn add_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `4098`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(17_000_000, 4098)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Community Communities (r:1 w:1)
	/// Proof Skipped: Community Communities (max_values: None, max_size: None, mode: Measured)
	fn join_community() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `4098`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(16_000_000, 4098)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}