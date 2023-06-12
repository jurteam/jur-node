use jur_node_runtime::{
	AccountId, AuraConfig, BalancesConfig,
	GenesisConfig, GrandpaConfig, Signature, SudoConfig, SystemConfig,
	WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::AccountId32;
use std::str::FromStr;
use hex_literal::hex;
use sp_core::crypto::UncheckedInto;
use sc_service::Properties;

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

pub fn sudo_account_local() -> AccountId {
	AccountId32::from_str(&"5DviAKtS4ns5TBuoHyigkkwEtNbG4sN1m8mw6XAfTBW7GG7j".to_string()).expect("Invalid Account Id")
}

pub fn sudo_account_testnet() -> AccountId {
	AccountId32::from_str(&"5ESNiXWyksqs5DxjSJ9gW2PA6gmgHLvGEUWdiaFfWDfNu82P".to_string()).expect("Invalid Account Id")
}

pub fn sudo_account_mainnet() -> AccountId {
	AccountId32::from_str(&"5DUzXPgapc76wzmA5vpyjt2SiQpsKZg3xbvsZuKxf8yG6gUp".to_string()).expect("Invalid Account Id")
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

pub fn local_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let mut properties = Properties::new();
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
				sudo_account_local(),
				// Pre-funded accounts
				vec![
					// Endow the Sudo account
					sudo_account_local(),
					hex!["1c10840139097128f7b28315814351ac9d3b3015615acc87c821685bd3c12f75"].into(),
					hex!["e8a6d9e3b7961f74fffcd7f7847957dc8e469e07cc49711c52beef4ecae92147"].into(),
					hex!["accec13ca659e4eb665dcf13d269a2ae529dcf7eed870453417c745e15e3ad27"].into(),
					hex!["701584da64f29db0c0eb7bc5892236c9bb40c881ef931a7a63aca77d3923930d"].into(),
					hex!["dc320380454e4fd8a99b7e789138b6a39bf267a96cc2458582de086a9a3a8b69"].into(),
					hex!["54003a5867459f5db90540fa993ea2d72deeb44e98644197c524d0c23e0bc951"].into(),
				],
				true
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

pub fn jur_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "JUR".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 33.into());
	Ok(ChainSpec::from_genesis(
		// Name
		"Jur Testnet",
		// ID
		"jur_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						hex!["1c10840139097128f7b28315814351ac9d3b3015615acc87c821685bd3c12f75"].unchecked_into(),
						hex!["b3728b98ab621a98f17a727542de3748aa2427495c82ba94f9c4d2f726efc393"].unchecked_into()
					),
					(
						hex!["60ba45f02ae84bcd0c4f3dbf48af1cdc7dd3b6b555e9ff4bf49b049f46501a7d"].unchecked_into(),
						hex!["403eeb3e1674713e25b61b01d6a71da99b2cad0abd21500e20c2853df9f70efd"].unchecked_into()
					),
					(
						hex!["34a2e82b426fcba4f45b118415296b0bcb4ffc5ac569a01a4d7f4612459a6742"].unchecked_into(),
						hex!["0d2358b76bc57cc34fb972b3326dec12d68b7047b5307fe6d0b9bee0b89c9835"].unchecked_into()
					)
				],
				// Sudo account
				sudo_account_testnet(),
				// Pre-funded accounts
				vec![
					// Endow the Sudo account
					sudo_account_testnet(),
					hex!["1c10840139097128f7b28315814351ac9d3b3015615acc87c821685bd3c12f75"].into(),
					hex!["60ba45f02ae84bcd0c4f3dbf48af1cdc7dd3b6b555e9ff4bf49b049f46501a7d"].into(),
					hex!["34a2e82b426fcba4f45b118415296b0bcb4ffc5ac569a01a4d7f4612459a6742"].into(),
					hex!["acb6fdeba311493e0c56e8ac81355329be1d6d006b5d4bf9f3f0b05acf09522d"].into(),
					hex!["743b7ff50e3c859ad455f3194824e374e7ab553a0a35130053f26661d8099b55"].into(),
					hex!["aad0a8a8165dc2189d2a7be27647a6e6271a88dcfdda1e6b9599680ca3ac2f30"].into(),
				],
				true
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

pub fn jur_mainnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "JUR".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 33.into());
	Ok(ChainSpec::from_genesis(
		// Name
		"Jur Mainnet",
		// ID
		"jur_mainnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						hex!["f24380855d8cf4a6f2c4c4b72af1f4d3b95636e046734a84e679c14bba271443"].unchecked_into(),
						hex!["4db5ea0c4b773a739434614a1c1fd4c5c5784477f8846d41a981303a9f23584b"].unchecked_into()
					),
					(
						hex!["c461a33a4f82c99cc8bb228f6833a88ab729f5ddae32bba566664b5f7135a74c"].unchecked_into(),
						hex!["d6ceedaeacb4db68ef681811bd2b9b0f4cf55fd5fc37bf2dde4296c7f0b63d50"].unchecked_into()
					),
					(
						hex!["20ad8cebd0e0083dd308f37c77b6e037beeb725f1bec71933c465866062ac110"].unchecked_into(),
						hex!["41138203ec4b40ab6840ca5dd713c5a728577089f6fcb3175d883901f7a3aa4b"].unchecked_into()
					)
				],
				// Sudo account
				sudo_account_mainnet(),
				// Pre-funded accounts
				vec![
					// Endow the Sudo account
					sudo_account_mainnet(),
					hex!["f24380855d8cf4a6f2c4c4b72af1f4d3b95636e046734a84e679c14bba271443"].into(),
					hex!["c461a33a4f82c99cc8bb228f6833a88ab729f5ddae32bba566664b5f7135a74c"].into(),
					hex!["20ad8cebd0e0083dd308f37c77b6e037beeb725f1bec71933c465866062ac110"].into(),
					hex!["867ec61710c3a5eaf7260f37a820d76071c3d151efc7084be689f6b147fa3e66"].into(),
					hex!["18c5b57615cdfb62396bdae7b1bbc33657fbd6037c3640cefbe19ef2ab33a649"].into(),
					hex!["7e94f48c642d79a9e6b6c07bc2b17ac27f9810034bd7a50839cf3613b03e5536"].into(),
				],
				true
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("jur_mainnet"),
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
		assets: Default::default(),
		treasury: Default::default(),
	}
}
