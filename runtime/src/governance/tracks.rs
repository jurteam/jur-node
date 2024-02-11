// Copyright 2023 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot. If not, see <http://www.gnu.org/licenses/>.

// Tracks configuration for governance

use super::*;

const fn percent(x: i32) -> sp_runtime::FixedI64 {
	sp_runtime::FixedI64::from_rational(x as u128, 100)
}
const fn permill(x: i32) -> sp_runtime::FixedI64 {
	sp_runtime::FixedI64::from_rational(x as u128, 1000)
}

use pallet_referenda::Curve;
use primitives::BlockNumber;

// TODO: keep adjust accordingly based on the community size
const APP_ROOT: Curve = Curve::make_reciprocal(1, 28, percent(80), percent(50), percent(100));
const SUP_ROOT: Curve = Curve::make_linear(28, 28, permill(15), percent(50));
const APP_REFERENDUM_CANCELLER: Curve =
	Curve::make_reciprocal(1, 7, percent(96), percent(50), percent(100));
const SUP_REFERENDUM_CANCELLER: Curve = Curve::make_linear(1, 7, percent(0), percent(50));
const APP_REFERENDUM_KILLER: Curve =
	Curve::make_reciprocal(1, 28, percent(96), percent(50), percent(100));
const SUP_REFERENDUM_KILLER: Curve = Curve::make_linear(1, 28, percent(0), percent(50));
const APP_TIPS: Curve = Curve::make_reciprocal(1, 7, percent(80), percent(50), percent(100));
const SUP_TIPS: Curve = Curve::make_reciprocal(7, 7, percent(10), percent(0), percent(50));
const APP_PROPOSAL: Curve = Curve::make_reciprocal(4, 28, percent(80), percent(50), percent(100));
const SUP_PROPOSAL: Curve = Curve::make_reciprocal(7, 28, percent(10), percent(0), percent(50));

const TRACKS_DATA: [(u16, pallet_referenda::TrackInfo<Balance, BlockNumber>); 5] = [
	(
		0,
		pallet_referenda::TrackInfo {
			name: "sudo",
			max_deciding: 1,
			decision_deposit: 2_512_500 * DOLLARS,
			prepare_period: 2 * HOURS,
			decision_period: 28 * DAYS,
			confirm_period: 24 * HOURS,
			min_enactment_period: 24 * MINUTES,
			min_approval: APP_ROOT,
			min_support: SUP_ROOT,
		},
	),
	(
		1,
		pallet_referenda::TrackInfo {
			name: "referendum_canceller",
			max_deciding: 1000,
			decision_deposit: 2_51_000 * DOLLARS,
			prepare_period: 2 * HOURS,
			decision_period: 7 * DAYS,
			confirm_period: 3 * HOURS,
			min_enactment_period: 1 * HOURS,
			min_approval: APP_REFERENDUM_CANCELLER,
			min_support: SUP_REFERENDUM_CANCELLER,
		},
	),
	(
		2,
		pallet_referenda::TrackInfo {
			name: "referendum_killer",
			max_deciding: 1000,
			decision_deposit: 6_28_000 * DOLLARS,
			prepare_period: 2 * HOURS,
			decision_period: 28 * DAYS,
			confirm_period: 3 * HOURS,
			min_enactment_period: 1 * HOURS,
			min_approval: APP_REFERENDUM_KILLER,
			min_support: SUP_REFERENDUM_KILLER,
		},
	),
	(
		3,
		pallet_referenda::TrackInfo {
			name: "tips",
			max_deciding: 100,
			decision_deposit: 6_300 * DOLLARS,
			prepare_period: 1 * HOURS,
			decision_period: 7 * DAYS,
			confirm_period: 1 * HOURS,
			min_enactment_period: 1 * HOURS,
			min_approval: APP_TIPS,
			min_support: SUP_TIPS,
		},
	),
	(
		4,
		pallet_referenda::TrackInfo {
			name: "proposal",
			max_deciding: 25,
			decision_deposit: 8_800 * DOLLARS,
			prepare_period: 4 * HOURS,
			decision_period: 28 * DAYS,
			confirm_period: 12 * HOURS,
			min_enactment_period: 24 * HOURS,
			min_approval: APP_PROPOSAL,
			min_support: SUP_PROPOSAL,
		},
	),
];

pub struct TracksInfo;
impl pallet_referenda::TracksInfo<Balance, BlockNumber> for TracksInfo {
	type Id = u16;
	type RuntimeOrigin = <RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;

	fn tracks() -> &'static [(Self::Id, pallet_referenda::TrackInfo<Balance, BlockNumber>)] {
		&TRACKS_DATA[..]
	}

	fn track_for(id: &Self::RuntimeOrigin) -> Result<Self::Id, ()> {
		if let Ok(system_origin) = frame_system::RawOrigin::try_from(id.clone()) {
			match system_origin {
				frame_system::RawOrigin::Root => {
					if let Some((track_id, _)) = Self::tracks()
						.into_iter()
						.find(|(_, track)| track.name == "sudo")
					{
						Ok(*track_id)
					} else {
						Err(())
					}
				},
				_ => Err(()),
			}
		} else if let Ok(custom_origin) = origins::Origin::try_from(id.clone()) {
			match custom_origin {
				origins::Origin::Sudo => Ok(0),
				origins::Origin::ReferendumCanceller => Ok(1),
				origins::Origin::ReferendumKiller => Ok(2),
				origins::Origin::Tips => Ok(3),
				origins::Origin::Proposal => Ok(4),
			}
		} else {
			Err(())
		}
	}
}
