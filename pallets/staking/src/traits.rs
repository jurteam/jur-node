// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! traits for chain-staking

use crate::weights::WeightInfo;
use frame_support::{dispatch::PostDispatchInfo, pallet_prelude::Weight};
use sp_runtime::DispatchErrorWithPostInfo;

pub trait OnValidatorPayout<AccountId, Balance> {
	fn on_validator_payout(
		for_round: crate::RoundIndex,
		validator_id: AccountId,
		amount: Balance,
	) -> Weight;
}
impl<AccountId, Balance> OnValidatorPayout<AccountId, Balance> for () {
	fn on_validator_payout(
		_for_round: crate::RoundIndex,
		_validator_id: AccountId,
		_amount: Balance,
	) -> Weight {
		Weight::zero()
	}
}

pub trait OnNewRound {
	fn on_new_round(round_index: crate::RoundIndex) -> Weight;
}
impl OnNewRound for () {
	fn on_new_round(_round_index: crate::RoundIndex) -> Weight {
		Weight::zero()
	}
}

/// Defines the behavior to payout the validator's reward.
pub trait PayoutValidatorReward<Runtime: crate::Config> {
	fn payout_validator_reward(
		round_index: crate::RoundIndex,
		validator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight;
}

/// Defines the default behavior for paying out the validator's reward. The amount is directly
/// deposited into the validator's account.
impl<Runtime: crate::Config> PayoutValidatorReward<Runtime> for () {
	fn payout_validator_reward(
		for_round: crate::RoundIndex,
		validator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight {
		crate::Pallet::<Runtime>::mint_validator_reward(for_round, validator_id, amount)
	}
}

pub trait OnInactiveValidator<Runtime: crate::Config> {
	fn on_inactive_validator(
		validator_id: Runtime::AccountId,
		round: crate::RoundIndex,
	) -> Result<Weight, DispatchErrorWithPostInfo<PostDispatchInfo>>;
}

impl<Runtime: crate::Config> OnInactiveValidator<Runtime> for () {
	fn on_inactive_validator(
		validator_id: <Runtime>::AccountId,
		_round: crate::RoundIndex,
	) -> Result<Weight, DispatchErrorWithPostInfo<PostDispatchInfo>> {
		crate::Pallet::<Runtime>::go_offline_inner(validator_id)?;
		Ok(<Runtime as crate::Config>::WeightInfo::go_offline(crate::MAX_CANDIDATES))
	}
}
