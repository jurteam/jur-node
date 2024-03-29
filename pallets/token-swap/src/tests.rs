use crate::{mock::*, EcdsaSignature, Error};
use frame_support::{assert_noop, assert_ok};
use hex_literal::hex;
use primitives::EthereumAddress;

fn update_root() {
	let account_proof: Vec<Vec<u8>> = vec![
		hex!("f90211a08cb1ac44e82cb4da253057ed4575d7235dd22c0d965e28ff35220709ae6c16b5a00807476567c6070dbd58cee9d9b57f75f3db7079f28d40ef72e462a11deafa99a06aa64b0e91632994c7c1601752da803e9019a8475794d685df31656022842015a081adf37c65163e73ff0fcd30a23edb8d6d5e51f2c33959fccc6112a2d0faa741a092c3c0368c9b8a4bdc4bf6889a920500b3ac73b062f7964b1c31c37cb343838ba046e377b0b72bec352cc184da4036246ed1879f3a202ed1860cf53b8aef4ef0c5a0c2418dd77e392e52142244a8f840b3e8c57e34dedadfb4e797c412c8cbf3ff72a0b9917de6b92fa6bf175ef1230ae9364d7e63ff5b4bf1a6b0146dfd6a3c4fbd44a00b1f06aa845f8a4889139ed3e2cc597af62eb0f3b3fafcbe753763dfbef9bb2fa0ccfd87afdd05cd311bd6e497f9f04388c9d37723e8db897bfc7de500ac0ddddda034ec07b89e7d2b8b350061107a0f74e160b0771215409374be8cd7d649b6ef6ea02e621e8efe0375438cba6a367e564b61c822a0c0aba06e9aa06e9d5237889a26a0a768208d7fb53653c66d2ebcfc8fc4173a429ec7c0975ace75081400f32231bfa052c301a7f1bdabdd52c86bfe989bf50d3aeb21dbcf617ad7a215d2a21b88a8e0a0ba41ed78050871cec255c7de2ec39c98769f239193295a92bc0176cd1383024aa0e3d0532ed8229c7f3cd64f0759eade693763bf3ff36e0fa6b85f2869e5c7f0ee80").to_vec(),
		hex!("f90211a072f07acbe8e921d6e511d454b2951f6b2cc697dca7d9a89bd8d2496b7f49ce17a06275931f81658af53c6bc606d90a4368bbbe8c08778d6a20bae204dc799a51a4a0fbe0430a163b4972608bffb8bbfe17ededa3011e59841ab5214826871958c7eea09f9c2d0e076e4b309328fd5c9d9c44df7ca2d7b705c9ed630bfbbb2135e410d0a0b51d50c2a073dfd3a5ee14ad5107f62b509de28d66744cacd94964c68d31759ea0104458c3d026006c9f1fef01304ed803cc3a8ae5788df8d06bfce85d564450e8a036c394647f25255d805b4dab45f072fb61a5c915bd62fd9f7fe5b3eb094b5029a015cbe3e57d6c3c9d4bfaa68c2960266dc014299d6db548af92409ad59eb99afaa0138aa6f0bb6fffa13553941cdaf9350ba9698b1373a2487efbd5c5aa98b9ae74a055e1fadf168181b99f02fde7f05e77830fb194e7c44c4dbfe1af01a51091fb12a02938300d5024d0e237736b4a3807f0e8147981e6c781e6f03505448552622d1fa02e43771db196931b686900ec6be0445b21637a119e1a5d68a23699b924dae1f9a08211c17e719cc5eedd4a5165151da32564011a00bc9cc9ee4b20d48f3cebe8a4a07ea26c77ee430f2bb0ebca58903062bc8db37855089fbaebf47c7e94a6f8238ba0ed6f5576f12b53303894910b4dba88b9b21de41fde5f7ad1c986a8ed72021a9ba0036de702298e490c9250d740bee64226e331c58eb371fea4d29b8386923ffc1880").to_vec(),
		hex!("f90211a01cea4d9c9c96cbf2d8c70f7b3fa8d5752e9a19c83ede08c6e42f08061299b076a091469a894e3250fcde6e72360e988426c1de48f898ec89417e906772024b53e9a08b4280eb2f079d6e83d252189d32c2f8d3ee5711d8f01e2dfef264d1c3402360a0d62d0321153a0b991177f1a1a13ac4b12060822f2d84ca71988aed1ef7413cb6a07e3872f53621cd4e6b892c881fbcd2aee31764c676164633bf47edc75bcea47ca023ddd534d1a13e9a0dd736cec2d941854f9a5a5010d6f9eef7a956b2ba1ced25a040d8d4e90361d9ffa04a41cc204465b24c8f9a957d1c9d8b8aa07b6fffaf8595a0dfdf1cd466bf35660fb6ad5a5939fd785b138735b4c3e67bb887a007a61b43cca023a1c5dd27c6928a639a3bb8c836f977886f55159646ada842683efc42850da6a06c3c8f7d2e7ea180388e685baeadf171296d5dc4caf76de083a752047093a00ba02633f48dd9e0a57f2bfe0d7c1ab7c01c0180c769c0f858e2374babec6bb11074a0672dc2742762f4d6ca2ecc4374f8e266765f11799f11d5fd69441a063cc6504aa0e1e2786ec51e1ab41f2ac5a9edc1447ea1de548d33c1a9b10a479526c520ce4da0b05f6ce218351935899ddde34036447b5cbeeb1d5f38266bd7813404857b3eaba05937eb6e2a17857cc18345cb59e5a4c78324d127b111db5e50bca455171f0496a05f342c4abe4ead14fb9c5f9fe39a473ac8084a214094a87e3121ed6e2edde5e080").to_vec(),
		hex!("f9017180a09ffc4e7326dd2b4ae8a64890382fabfce1c6e6a079a5a1c8e914c2f9ce78eb7ca0aeeb238613088ddb97393186857d41561179f2d5366f6d39f53a99ed42c0ca7ea0d9a8980df80605e37320b2add6f4cdf43c39f07fd5d973cb17c03c40238f8cdca0683d4cb6415d88998d67edfa4c6c79fae1e1eb7cb2ca1e92e6c984e53e3dfadca061418c983a4fa07c9eb6926447d597a883100ac613882c238480dc4370462334a05c88e0a5a9f07d0c8888cace7e08ac00d6625052811fe910de4a36ead8fa4278a0a9578ef10c917b8dafdeb2e067462131bcf9f00630c72bdc1262255c6f2222e1a0db4e8d78cbdc42eea499e0600bc4b348ce58b82d9564eb6a2a1658c47b241811a05a85ccf841f1a02a9dcc066ee886fb8593677fdb09a42151fe3b47c6c70a88d0a0ddb408723f87c9f71692d8c48f919c9b73cd22284b7517107b6c5b0ad94ba4d8a0ebeb97f031c5fbcf374e5520e30392e1eac79351138dda6c432fe2b085d63bf48080808080").to_vec(),
		hex!("f87180a0424a7c77f954cd360ced27c63e459b69a2407ca9498590add188aa6a126e1a4d8080a0ca4edfdb951a6eb3064d5afd48ccf37dbb7544eefd3e6370f265f9b87ec891c08080a040ad28e3e0ff9a5c3440dc25a1ac0bd1ea103dae4342d9e50279b55d96eebdb0808080808080808080").to_vec(),
		hex!("f87d9e3f539a7fc1a6603092ab102bef7cc8c495ef7174c4668ac9b6c884cdc701b85cf85a80808094a18b81879e99394df4b99b78cf71037836706db2a06439ad2859e615114f02251c6d09c2a36e62d6de6cd55d0ad771964009ab6cc4a0072fbb05700cf818d7d3f6de8bb4d0d18cdfed173106b2b5af87ee06fe801d39").to_vec(),
	];

	assert_ok!(TokenSwap::update_state_root(
		RuntimeOrigin::root(),
		VECHAIN_ROOT_HASH,
		1,
		"".as_bytes().to_vec(),
		account_proof,
	));
}

#[test]
fn claim_works() {
	new_test_ext().execute_with(|| {

		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");
		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"My JUR address is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();

		let storage_proof: Vec<Vec<u8>> =
			vec![hex!("e5a12013614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a6114992218281c8")
				.to_vec()];
		update_root();
		assert_ok!(TokenSwap::claim(
			RuntimeOrigin::none(),
			EcdsaSignature(signature),
			signed_json,
			storage_proof,
		));

		let eth_address = EthereumAddress([
			161, 139, 129, 135, 158, 153, 57, 77, 244, 185, 155, 120, 207, 113, 3, 120, 54, 112, 109, 178,
		]);
		assert_eq!(TokenSwap::latest_claimed_balance(eth_address), Some(200));
	});
}

#[test]
fn claim_with_invalid_ethereum_address_does_not_work() {
	new_test_ext().execute_with(|| {
		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"My JUR address is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();
		let storage_proof: Vec<Vec<u8>> =
			vec![hex!("e5a12013614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a6114992218281c8")
				.to_vec()];
		assert_noop!(
			TokenSwap::claim(RuntimeOrigin::none(), EcdsaSignature([0; 65]), signed_json,storage_proof),
			Error::<Test>::InvalidEthereumSignature
		);
	});
}

#[test]
fn claim_with_invalid_json_does_not_work() {
	new_test_ext().execute_with(|| {
		let signed_json = r#"{payload":{"content":"My JUR address is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();
		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");
		let storage_proof: Vec<Vec<u8>> =
			vec![hex!("e5a12013614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a6114992218281c8")
				.to_vec()];
		assert_noop!(
			TokenSwap::claim(RuntimeOrigin::none(), EcdsaSignature(signature), signed_json, storage_proof),
			Error::<Test>::InvalidJson
		);
	});
}

#[test]
fn claim_with_invalid_proof_does_not_work() {
	new_test_ext().execute_with(|| {
		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");
		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"My JUR address is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();

		let storage_proof: Vec<Vec<u8>> =
			vec![hex!("e5a12013614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a6114992218281c9")
				.to_vec()];
		update_root();
		assert_noop!(
			TokenSwap::claim(RuntimeOrigin::none(), EcdsaSignature(signature), signed_json, storage_proof),
			Error::<Test>::InvalidProof
		);
	});
}

#[test]
fn claim_with_invalid_input_does_not_work() {
	new_test_ext().execute_with(|| {
		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");
		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"My JUR address is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();

		let storage_proof: Vec<Vec<u8>> =
			vec![hex!("e5a12013614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a6114992218281c9")
				.to_vec()];

		assert_noop!(
			TokenSwap::claim(RuntimeOrigin::none(), EcdsaSignature(signature), signed_json, storage_proof),
			Error::<Test>::InvalidInput
		);
	});
}

#[test]
fn claim_with_invalid_substrate_address_does_not_work() {
	new_test_ext().execute_with(|| {
		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");
		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"My JUR address is ","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();

		let storage_proof: Vec<Vec<u8>> =
			vec![hex!("e5a12013614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a6114992218281c8")
				.to_vec()];
		update_root();
		assert_noop!(
			TokenSwap::claim(RuntimeOrigin::none(), EcdsaSignature(signature), signed_json, storage_proof),
			Error::<Test>::InvalidSubstrateAddress
		);
	});
}

#[test]
fn claim_with_no_json_content_does_not_work() {
	new_test_ext().execute_with(|| {
		let signed_json = r#"{"domain":"localhost:3000","payload":{"type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();
		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");
		let storage_proof: Vec<Vec<u8>> =
			vec![hex!("e5a12013614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a6114992218281c8")
				.to_vec()];


		assert_noop!(
			TokenSwap::claim(RuntimeOrigin::none(), EcdsaSignature(signature), signed_json, storage_proof),
			Error::<Test>::ContentNotFound
		);
	});
}

#[test]
fn claim_with_no_prefix_does_not_work() {
	new_test_ext().execute_with(|| {
		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();
		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");
		let storage_proof: Vec<Vec<u8>> =
			vec![hex!("e5a12013614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a6114992218281c8")
				.to_vec()];


		assert_noop!(
			TokenSwap::claim(RuntimeOrigin::none(), EcdsaSignature(signature), signed_json, storage_proof.clone()),
			Error::<Test>::PrefixDoesNotMatch
		);

		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"Abs    5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();

		assert_noop!(
			TokenSwap::claim(RuntimeOrigin::none(), EcdsaSignature(signature), signed_json, storage_proof),
			Error::<Test>::PrefixDoesNotMatch
		);
	});
}

#[test]
fn claim_with_invalid_locked_balance_does_not_work() {
	new_test_ext().execute_with(|| {
		let signed_json = r#"{"domain":"localhost:3000","payload":{"content":"My JUR address is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","type":"text"},"purpose":"agreement","signer":"0xa18b81879e99394df4b99b78cf71037836706db2","timestamp":1654848070}"#.as_bytes().to_vec();
		let signature = hex!("58acd0227ff9dc881e386cda6dfb316b5f8a0f1bd14069c1b39d6f6fe6e6c026145e9441d503f2b9e29a1757cb2a19f5807abd27f8c3017c808ac0468930ae7401");
		let storage_proof: Vec<Vec<u8>> =
			vec![hex!("e5a12013614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a6114992218281c8")
				.to_vec()];

		update_root();
		assert_ok!(
			TokenSwap::claim(
				RuntimeOrigin::none(),
				EcdsaSignature(signature),
				signed_json.clone(),
				storage_proof.clone(),
		));

		assert_noop!(
			TokenSwap::claim(
				RuntimeOrigin::none(),
				EcdsaSignature(signature),
				signed_json,
				storage_proof,
			),
			Error::<Test>::NotSufficientLockedBalance
		);
	});
}
