use crate::{mock::*, EcdsaSignature, Error};
use frame_support::{assert_noop, assert_ok};
use parity_scale_codec::Encode;
use primitives::EthereumAddress;

#[test]
fn claim_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(TokenSwap::claim(
			Origin::none(),
			42,
			100,
			sig::<Test>(&alice(), &42u64.encode(), &[][..])
		));

		let eth_address = EthereumAddress([
			191, 11, 90, 64, 153, 240, 191, 108, 139, 196, 37, 46, 190, 197, 72, 186, 233, 86, 2,
			234,
		]);
		assert_eq!(TokenSwap::latest_claimed_balance(eth_address), Some(100));
	});
}

// With storage proof
// #[test]
// fn claim_works() {
// 	new_test_ext().execute_with(|| {
// 		assert_ok!(TokenSwap::claim(
// 				Origin::none(),
// 				42,
// 				["0xf85180808080808080808080a0434b850e89e7a8997071483f526310b70f1f2278a7de0f54630b583ae26f5db5a07d3c98cab3f5adc685b826f58dabf812414de3a52a6782eef2ac49e1866f0c528080808080","0xe5a033cd6489af8b2c8afd61140886b95dd3c9f21cdaccc0c4f5796077691b9d748a8382012c"],
// 				sig::<Test>(&alice(), &42u64.encode(), &[][..])
// 			));
//
// 		let eth_address = EthereumAddress([191, 11, 90, 64, 153, 240, 191, 108, 139, 196, 37, 46, 190, 197, 72, 186, 233, 86, 2, 234]);
// 		assert_eq!(TokenSwap::latest_claimed_balance(eth_address), Some(300));
// 	});
// }

#[test]
fn claim_with_invalid_ethereum_address_does_not_work() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			TokenSwap::claim(Origin::none(), 42, 1, EcdsaSignature([0; 65])),
			Error::<Test>::InvalidEthereumSignature
		);
	});
}

#[test]
fn claim_with_invalid_locked_balance_does_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(TokenSwap::claim(
			Origin::none(),
			42,
			100,
			sig::<Test>(&alice(), &42u64.encode(), &[][..])
		));

		assert_noop!(
			TokenSwap::claim(
				Origin::none(),
				42,
				1,
				sig::<Test>(&alice(), &42u64.encode(), &[][..])
			),
			Error::<Test>::NotSufficientLockedBalance
		);
	});
}
