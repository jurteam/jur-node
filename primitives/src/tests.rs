use crate::proof::verify_proof;
use frame_support::assert_ok;
use hex_literal::hex;

#[test]
fn verify_proof_works() {
    let proof =
        hex!("e5a12013614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a6114992218281c8")
            .to_vec();
    assert_ok!(verify_proof(
			hex!("072fbb05700cf818d7d3f6de8bb4d0d18cdfed173106b2b5af87ee06fe801d39"),
			vec![proof],
			hex!("13614086fa178320f9277044fb1a8a462fdd1e42c15784123ab858a611499221").to_vec()
		));
}