
//! Autogenerated weights for pallet_passport
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Pankajs-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/jur-node
// benchmark
// pallet
// --chain=dev
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
	fn add_badge() -> Weight;
	fn issue_badge() -> Weight;
	fn migrate_passport() -> Weight;
}

/// Weights for pallet_passport using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Community::Communities` (r:1 w:0)
	/// Proof: `Community::Communities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Passports` (r:1 w:1)
	/// Proof: `Passport::Passports` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::NextPassportId` (r:1 w:1)
	/// Proof: `Passport::NextPassportId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `806`
		//  Estimated: `4271`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(24_000_000, 4271)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Community::Communities` (r:1 w:0)
	/// Proof: `Community::Communities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Passports` (r:1 w:1)
	/// Proof: `Passport::Passports` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_passport() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `940`
		//  Estimated: `4405`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(24_000_000, 4405)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Community::Communities` (r:1 w:0)
	/// Proof: `Community::Communities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Badges` (r:1 w:1)
	/// Proof: `Passport::Badges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_badge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `940`
		//  Estimated: `4405`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(23_000_000, 4405)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Community::Communities` (r:1 w:0)
	/// Proof: `Community::Communities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Badges` (r:1 w:0)
	/// Proof: `Passport::Badges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Passports` (r:1 w:1)
	/// Proof: `Passport::Passports` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn issue_badge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1134`
		//  Estimated: `4599`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 4599)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Whitelist::Admins` (r:1 w:0)
	/// Proof: `Whitelist::Admins` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Community::Communities` (r:1 w:1)
	/// Proof: `Community::Communities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Badges` (r:1 w:0)
	/// Proof: `Passport::Badges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Passports` (r:1 w:1)
	/// Proof: `Passport::Passports` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn migrate_passport() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1099`
		//  Estimated: `4564`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 4564)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Community::Communities` (r:1 w:0)
	/// Proof: `Community::Communities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Passports` (r:1 w:1)
	/// Proof: `Passport::Passports` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::NextPassportId` (r:1 w:1)
	/// Proof: `Passport::NextPassportId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `806`
		//  Estimated: `4271`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(24_000_000, 4271)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Community::Communities` (r:1 w:0)
	/// Proof: `Community::Communities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Passports` (r:1 w:1)
	/// Proof: `Passport::Passports` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_passport() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `940`
		//  Estimated: `4405`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(24_000_000, 4405)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Community::Communities` (r:1 w:0)
	/// Proof: `Community::Communities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Badges` (r:1 w:1)
	/// Proof: `Passport::Badges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_badge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `940`
		//  Estimated: `4405`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(23_000_000, 4405)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Community::Communities` (r:1 w:0)
	/// Proof: `Community::Communities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Badges` (r:1 w:0)
	/// Proof: `Passport::Badges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Passports` (r:1 w:1)
	/// Proof: `Passport::Passports` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn issue_badge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1134`
		//  Estimated: `4599`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 4599)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Whitelist::Admins` (r:1 w:0)
	/// Proof: `Whitelist::Admins` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Community::Communities` (r:1 w:1)
	/// Proof: `Community::Communities` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Badges` (r:1 w:0)
	/// Proof: `Passport::Badges` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Passport::Passports` (r:1 w:1)
	/// Proof: `Passport::Passports` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn migrate_passport() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1099`
		//  Estimated: `4564`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 4564)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}