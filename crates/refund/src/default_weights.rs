//! Autogenerated weights for refund
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-08, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/interbtc-standalone
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// refund
// --extrinsic
// *
// --steps
// 100
// --repeat
// 10
// --output
// crates/refund/src/default_weights.rs
// --template
// .deploy/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for refund.
pub trait WeightInfo {
	fn execute_refund() -> Weight;
}

/// Weights for refund using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Refund RefundRequests (r:1 w:1)
	// Storage: BTCRelay DisableInclusionCheck (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: BTCRelay Chains (r:1 w:0)
	// Storage: BTCRelay BlockHeaders (r:1 w:0)
	// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: BTCRelay StableParachainConfirmations (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Staking Nonce (r:1 w:0)
	// Storage: Staking TotalCurrentStake (r:1 w:0)
	// Storage: Rewards Stake (r:1 w:1)
	// Storage: Rewards TotalStake (r:1 w:1)
	// Storage: Rewards RewardTally (r:1 w:1)
	// Storage: Rewards RewardPerToken (r:1 w:0)
	fn execute_refund() -> Weight {
		(201_877_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(18 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Refund RefundRequests (r:1 w:1)
	// Storage: BTCRelay DisableInclusionCheck (r:1 w:0)
	// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	// Storage: BTCRelay Chains (r:1 w:0)
	// Storage: BTCRelay BlockHeaders (r:1 w:0)
	// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	// Storage: Security ActiveBlockCount (r:1 w:0)
	// Storage: BTCRelay StableParachainConfirmations (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:1)
	// Storage: Oracle Aggregate (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Staking Nonce (r:1 w:0)
	// Storage: Staking TotalCurrentStake (r:1 w:0)
	// Storage: Rewards Stake (r:1 w:1)
	// Storage: Rewards TotalStake (r:1 w:1)
	// Storage: Rewards RewardTally (r:1 w:1)
	// Storage: Rewards RewardPerToken (r:1 w:0)
	fn execute_refund() -> Weight {
		(201_877_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(18 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
}

