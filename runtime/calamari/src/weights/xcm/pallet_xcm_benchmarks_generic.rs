// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_xcm_benchmarks::generic`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-90-64`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=calamari-dev
// --steps=50
// --repeat=20
// --pallet=pallet_xcm_benchmarks::generic
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/xcm-weights-output/pallet_xcm_benchmarks_generic.rs
// --template=.github/resources/xcm-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_xcm_benchmarks::generic`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	pub(crate) fn query_holding() -> Weight {
		Weight::from_ref_time(951_237_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	pub(crate) fn buy_execution() -> Weight {
		Weight::from_ref_time(4_722_000)
	}
	// Storage: PolkadotXcm Queries (r:1 w:0)
	pub(crate) fn query_response() -> Weight {
		Weight::from_ref_time(14_392_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	pub(crate) fn transact() -> Weight {
		Weight::from_ref_time(16_599_000)
	}
	pub(crate) fn refund_surplus() -> Weight {
		Weight::from_ref_time(4_705_000)
	}
	pub(crate) fn set_error_handler() -> Weight {
		Weight::from_ref_time(836_000)
	}
	pub(crate) fn set_appendix() -> Weight {
		Weight::from_ref_time(2_678_000)
	}
	pub(crate) fn clear_error() -> Weight {
		Weight::from_ref_time(2_566_000)
	}
	pub(crate) fn descend_origin() -> Weight {
		Weight::from_ref_time(7_044_000)
	}
	pub(crate) fn clear_origin() -> Weight {
		Weight::from_ref_time(1_867_000)
	}
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	pub(crate) fn report_error() -> Weight {
		Weight::from_ref_time(24_924_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PolkadotXcm AssetTraps (r:1 w:1)
	pub(crate) fn claim_asset() -> Weight {
		Weight::from_ref_time(39_192_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	pub(crate) fn trap() -> Weight {
		Weight::from_ref_time(2_596_000)
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	pub(crate) fn subscribe_version() -> Weight {
		Weight::from_ref_time(51_630_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:0 w:1)
	pub(crate) fn unsubscribe_version() -> Weight {
		Weight::from_ref_time(5_226_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	pub(crate) fn initiate_reserve_withdraw() -> Weight {
		Weight::from_ref_time(1_211_909_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}