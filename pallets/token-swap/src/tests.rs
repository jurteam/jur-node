use crate::{mock::*, EcdsaSignature, Error};
use frame_support::{assert_noop, assert_ok};
use hex_literal::hex;
use sp_core::H256;
use primitives::EthereumAddress;

#[test]
fn claim_works() {
	new_test_ext().execute_with(|| {

		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");
		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"My JUR address is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();

		assert_ok!(TokenSwap::claim(
			Origin::none(),
			42,
			100,
			EcdsaSignature(signature),
			signed_json
		));

		let eth_address = EthereumAddress([
			161, 139, 129, 135, 158, 153, 57, 77, 244, 185, 155, 120, 207, 113, 3, 120, 54, 112, 109, 178,
		]);
		assert_eq!(TokenSwap::latest_claimed_balance(eth_address), Some(100));
	});
}

#[test]
fn verify_proof_works() {
	new_test_ext().execute_with(|| {
		let proof  = hex!("e5a12013614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a6114992218281c8").to_vec();
		assert_ok!(TokenSwap::verify_proof(
			hex!("072fbb05700cf818d7d3f6de8bb4d0d18cdfed173106b2b5af87ee06fe801d39"),
			vec![proof],
			hex!("13614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a611499221").to_vec()
		));

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

// #[test]
// fn claim_with_invalid_ethereum_address_does_not_work() {
// 	new_test_ext().execute_with(|| {
// 		assert_noop!(
// 			TokenSwap::claim(Origin::none(), 42, 1, EcdsaSignature([0; 65])),
// 			Error::<Test>::InvalidEthereumSignature
// 		);
// 	});
// }
//
// #[test]
// fn claim_with_invalid_locked_balance_does_not_work() {
// 	new_test_ext().execute_with(|| {
// 		assert_ok!(TokenSwap::claim(
// 			Origin::none(),
// 			42,
// 			100,
// 			sig::<Test>(&alice(), &42u64.encode(), &[][..])
// 		));
//
// 		assert_noop!(
// 			TokenSwap::claim(
// 				Origin::none(),
// 				42,
// 				1,
// 				sig::<Test>(&alice(), &42u64.encode(), &[][..])
// 			),
// 			Error::<Test>::NotSufficientLockedBalance
// 		);
// 	});
// }
