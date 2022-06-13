use crate::{mock::*, EcdsaSignature, Error};
use frame_support::{assert_noop, assert_ok};
use hex_literal::hex;
use primitives::EthereumAddress;

#[test]
fn claim_works() {
	new_test_ext().execute_with(|| {

		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");
		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"My JUR address is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();

		assert_ok!(TokenSwap::claim(
			Origin::none(),
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
fn claim_with_invalid_ethereum_address_does_not_work() {
	new_test_ext().execute_with(|| {
		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"My JUR address is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();

		assert_noop!(
			TokenSwap::claim(Origin::none(), 1, EcdsaSignature([0; 65]), signed_json),
			Error::<Test>::InvalidEthereumSignature
		);
	});
}

#[test]
fn claim_with_invalid_json_does_not_work() {
	new_test_ext().execute_with(|| {
		let signed_json = r#"{payload":{"content":"My JUR address is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();
		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");

		assert_noop!(
			TokenSwap::claim(Origin::none(), 1, EcdsaSignature(signature), signed_json),
			Error::<Test>::InvalidJson
		);
	});
}

#[test]
fn claim_with_no_json_content_does_not_work() {
	new_test_ext().execute_with(|| {
		let signed_json = r#"{"domain":"localhost:3000","payload":{"type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();
		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");

		assert_noop!(
			TokenSwap::claim(Origin::none(), 1, EcdsaSignature(signature), signed_json),
			Error::<Test>::ContentNotFound
		);
	});
}

#[test]
fn claim_with_invalid_locked_balance_does_not_work() {
	new_test_ext().execute_with(|| {
		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"My JUR address is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();
		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");

		assert_ok!(
			TokenSwap::claim(
				Origin::none(),
				100,
				EcdsaSignature(signature),
				signed_json.clone()
		));

		assert_noop!(
			TokenSwap::claim(
				Origin::none(),
				1,
				EcdsaSignature(signature),
				signed_json
			),
			Error::<Test>::NotSufficientLockedBalance
		);
	});
}
