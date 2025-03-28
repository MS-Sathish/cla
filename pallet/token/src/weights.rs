
//! Autogenerated weights for `pallet_token`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-11, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-102-85`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("clarus")`, DB CACHE: 1024

// Executed Command:
// ./clarus-node
// benchmark
// pallet
// --chain
// clarus
// --wasm-execution=compiled
// --pallet
// pallet_token
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./token-weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

pub trait WeightInfo {
	fn mint() -> Weight;
	fn burn() -> Weight;
	fn transfer() -> Weight;
	fn transfer_from() -> Weight;
	fn approve() -> Weight;
	fn lock() -> Weight;
	fn unlock() -> Weight;
	fn burn_tokens() -> Weight;

}

/// Weight functions for `pallet_token`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Token::Asset` (r:1 w:1)
	/// Proof: `Token::Asset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Token::BalanceOf` (r:1 w:1)
	/// Proof: `Token::BalanceOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `151`
		//  Estimated: `3616`
		// Minimum execution time: 29_623_000 picoseconds.
		Weight::from_parts(30_192_000, 0)
			.saturating_add(Weight::from_parts(0, 3616))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Token::Asset` (r:1 w:1)
	/// Proof: `Token::Asset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Token::BalanceOf` (r:1 w:1)
	/// Proof: `Token::BalanceOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `257`
		//  Estimated: `3722`
		// Minimum execution time: 34_686_000 picoseconds.
		Weight::from_parts(35_370_000, 0)
			.saturating_add(Weight::from_parts(0, 3722))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	/// Storage: `Token::Asset` (r:1 w:1)
	/// Proof: `Token::Asset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Token::BalanceOf` (r:1 w:1)
	/// Proof: `Token::BalanceOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn burn_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `257`
		//  Estimated: `3722`
		// Minimum execution time: 34_686_000 picoseconds.
		Weight::from_parts(35_370_000, 0)
			.saturating_add(Weight::from_parts(0, 3722))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	/// Storage: `Token::BalanceOf` (r:2 w:2)
	/// Proof: `Token::BalanceOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `159`
		//  Estimated: `6099`
		// Minimum execution time: 37_258_000 picoseconds.
		Weight::from_parts(37_807_000, 0)
			.saturating_add(Weight::from_parts(0, 6099))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	fn lock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `159`
		//  Estimated: `6099`
		// Minimum execution time: 37_258_000 picoseconds.
		Weight::from_parts(37_807_000, 0)
			.saturating_add(Weight::from_parts(0, 6099))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}

	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `159`
		//  Estimated: `6099`
		// Minimum execution time: 37_258_000 picoseconds.
		Weight::from_parts(37_807_000, 0)
			.saturating_add(Weight::from_parts(0, 6099))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Token::Allowance` (r:1 w:1)
	/// Proof: `Token::Allowance` (`max_values`: Some(300000), `max_size`: None, mode: `Measured`)
	/// Storage: `Token::BalanceOf` (r:2 w:2)
	/// Proof: `Token::BalanceOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn transfer_from() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `322`
		//  Estimated: `6262`
		// Minimum execution time: 70_086_000 picoseconds.
		Weight::from_parts(71_373_000, 0)
			.saturating_add(Weight::from_parts(0, 6262))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Token::BalanceOf` (r:1 w:0)
	/// Proof: `Token::BalanceOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Token::Allowance` (r:1 w:1)
	/// Proof: `Token::Allowance` (`max_values`: Some(300000), `max_size`: None, mode: `Measured`)
	fn approve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `169`
		//  Estimated: `3634`
		// Minimum execution time: 40_370_000 picoseconds.
		Weight::from_parts(40_868_000, 0)
			.saturating_add(Weight::from_parts(0, 3634))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
