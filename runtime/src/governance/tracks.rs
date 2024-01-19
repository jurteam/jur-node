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

use primitives::BlockNumber;
use pallet_referenda::Curve;

// ask parity about the science behind these percentages
// TODO: adjust accordingly based on the community size
// const APP_ROOT: Curve = Curve::make_reciprocal(4, 14, percent(80), percent(50), percent(100));
// const SUP_ROOT: Curve = Curve::make_linear(14, 14, permill(5), percent(25));
const APP_WHITELISTED_CALLER: Curve =
    Curve::make_reciprocal(16, 28 * 24, percent(96), percent(50), percent(100));
const SUP_WHITELISTED_CALLER: Curve =
    Curve::make_reciprocal(1, 28, percent(20), percent(5), percent(50));
const APP_REFERENDUM_CANCELLER: Curve = Curve::make_linear(17, 28, percent(50), percent(100));
const SUP_REFERENDUM_CANCELLER: Curve =
    Curve::make_reciprocal(12, 28, percent(1), percent(0), percent(50));
const APP_REFERENDUM_KILLER: Curve = Curve::make_linear(17, 28, percent(50), percent(100));
const SUP_REFERENDUM_KILLER: Curve =
    Curve::make_reciprocal(12, 28, percent(1), percent(0), percent(50));
const APP_GENERAL_ADMIN: Curve =
    Curve::make_reciprocal(4, 28, percent(80), percent(50), percent(100));
const SUP_GENERAL_ADMIN: Curve =
    Curve::make_reciprocal(7, 28, percent(10), percent(0), percent(50));

const TRACKS_DATA: [(u16, pallet_referenda::TrackInfo<Balance, BlockNumber>); 4] = [
    (
        1,
        pallet_referenda::TrackInfo {
            name: "whitelisted_caller",
            max_deciding: 100,
            decision_deposit: 1000 * DOLLARS,
            prepare_period: 5 * MINUTES,
            decision_period: 20 * MINUTES,
            confirm_period: 0 * MINUTES,
            min_enactment_period: 10 * MINUTES,
            min_approval: APP_WHITELISTED_CALLER,
            min_support: SUP_WHITELISTED_CALLER,
        },
    ),
    (
        3,
        pallet_referenda::TrackInfo {
            name: "referendum_canceller",
            max_deciding: 20,
            decision_deposit: 1000 * DOLLARS,
            prepare_period: 1 * HOURS,
            decision_period: 14 * DAYS,
            confirm_period: 3 * HOURS,
            min_enactment_period: 10 * MINUTES,
            min_approval: APP_REFERENDUM_CANCELLER,
            min_support: SUP_REFERENDUM_CANCELLER,
        },
    ),
    (
        4,
        pallet_referenda::TrackInfo {
            name: "referendum_killer",
            max_deciding: 100,
            decision_deposit: 2000 * DOLLARS,
            prepare_period: 1 * HOURS,
            decision_period: 14 * DAYS,
            confirm_period: 3 * HOURS,
            min_enactment_period: 10 * MINUTES,
            min_approval: APP_REFERENDUM_KILLER,
            min_support: SUP_REFERENDUM_KILLER,
        },
    ),
    (
        5,
        pallet_referenda::TrackInfo {
            name: "general_admin",
            max_deciding: 10,
            decision_deposit: 500 * DOLLARS,
            prepare_period: 1 * HOURS,
            decision_period: 14 * DAYS,
            confirm_period: 1 * DAYS,
            min_enactment_period: 10 * MINUTES,
            min_approval: APP_GENERAL_ADMIN,
            min_support: SUP_GENERAL_ADMIN,
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
                    if let Some((track_id, _)) =
                        Self::tracks().into_iter().find(|(_, track)| track.name == "sudo")
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
                // origins::Origin::Sudo => Ok(1),
                origins::Origin::WhitelistedCaller => Ok(2),
                origins::Origin::ReferendumCanceller => Ok(3),
                origins::Origin::ReferendumKiller => Ok(4),
                origins::Origin::GeneralAdmin => Ok(5),
            }
        } else {
            Err(())
        }
    }
}