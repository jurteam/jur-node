use jur_node_runtime::{
	AccountId, AuraConfig, BalancesConfig, CouncilConfig, CouncilMembershipConfig, DemocracyConfig,
	GenesisConfig, GrandpaConfig, Signature, SudoConfig, SystemConfig, TechnicalMembershipConfig,
	WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::AccountId32;
use std::str::FromStr;
use hex_literal::{
	hex, // for parsing string literal at compile time use hex!("...");
};
use sp_core::crypto::UncheckedInto;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn sudo_account_testnet() -> AccountId {
	AccountId32::from_str(&"5DviAKtS4ns5TBuoHyigkkwEtNbG4sN1m8mw6XAfTBW7GG7j".to_string()).expect("Invalid Account Id")
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				]
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn jur_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "JUR".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 33.into());

	Ok(ChainSpec::from_genesis(
		// Name
		"Jur Testnet",
		// ID
		"jur_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						hex!["4076403ada1e84a045cfc627efe8f7f1a734e95a50644e7030c0cb1a70dc580f"].unchecked_into(),
						hex!["e52e2901ea2bb2795601f1e130e1936c7f861e6375ea70ad7ca92ee9a121a75f"].unchecked_into()
					),
					(
						hex!["e8a6d9e3b7961f74fffcd7f7847957dc8e469e07cc49711c52beef4ecae92147"].unchecked_into(),
						hex!["e9e9d202692f8446f013c0b550e4bb1507d6de60a52cdaee0a4863cc554897f9"].unchecked_into()
					),
					(
						hex!["accec13ca659e4eb665dcf13d269a2ae529dcf7eed870453417c745e15e3ad27"].unchecked_into(),
						hex!["ee1773c391a8d3e404f2b6f1f0ec5e22b9719a753b2a24376ab50113283d49d0"].unchecked_into()
					)

				],
				// Sudo account
				sudo_account_testnet(),
				// Pre-funded accounts
				vec![
					// Endow the Sudo account
					sudo_account_testnet(),
					hex!["4076403ada1e84a045cfc627efe8f7f1a734e95a50644e7030c0cb1a70dc580f"].into(),
					hex!["e8a6d9e3b7961f74fffcd7f7847957dc8e469e07cc49711c52beef4ecae92147"].into(),
					hex!["accec13ca659e4eb665dcf13d269a2ae529dcf7eed870453417c745e15e3ad27"].into(),
					hex!["701584da64f29db0c0eb7bc5892236c9bb40c881ef931a7a63aca77d3923930d"].into(),
					hex!["dc320380454e4fd8a99b7e789138b6a39bf267a96cc2458582de086a9a3a8b69"].into(),
					hex!["54003a5867459f5db90540fa993ea2d72deeb44e98644197c524d0c23e0bc951"].into(),
				],
				true,
				vec![
					sudo_account_testnet(),
					hex!["4076403ada1e84a045cfc627efe8f7f1a734e95a50644e7030c0cb1a70dc580f"].into(),
				],
				vec![
					sudo_account_testnet(),
					hex!["4076403ada1e84a045cfc627efe8f7f1a734e95a50644e7030c0cb1a70dc580f"].into(),
				]
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("jur-testnet"),
		// Properties
		None,
		Some(properties),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
	council_members: Vec<AccountId>,
	technical_members: Vec<AccountId>
) -> GenesisConfig {

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		democracy: DemocracyConfig::default(),
		council: CouncilConfig::default(),
		council_membership: CouncilMembershipConfig {
			members: council_members,
			phantom: Default::default(),
		},
		technical_committee: Default::default(),
		technical_membership: TechnicalMembershipConfig {
			members: technical_members,
			phantom: Default::default(),
		},
		treasury: Default::default(),
		elections: Default::default(),
		assets: Default::default(),
	}
}
