use hex_literal::hex;
use jur_node_runtime::{
	AccountId, AuraConfig, BalancesConfig, Block, GrandpaConfig, RuntimeGenesisConfig, Signature,
	SudoConfig, SystemConfig, WASM_BINARY,SessionKeys,
};
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_service::Properties;
use serde::{Deserialize, Serialize};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::crypto::UncheckedInto;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::AccountId32;
use std::str::FromStr;
use sp_runtime::{Perbill, Percent};
use pallet_staking::{InflationInfo, Range};

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}
pub type Balance = u128;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

pub fn inflation_config(
	blocks_per_round: u32,
	annual_inflation_percentage: u32,
) -> InflationInfo<Balance> {
	fn to_round_inflation(annual: Range<Perbill>, blocks_per_round: u32) -> Range<Perbill> {
		use pallet_staking::inflation::{
			perbill_annual_to_perbill_round, BLOCKS_PER_YEAR,
		};
		perbill_annual_to_perbill_round(annual, BLOCKS_PER_YEAR / blocks_per_round)
	}

	let annual = Range {
		min: Perbill::from_percent(annual_inflation_percentage),
		ideal: Perbill::from_percent(annual_inflation_percentage),
		max: Perbill::from_percent(annual_inflation_percentage),
	};

	InflationInfo {
		// We have no staking expectations since inflation range is a singular value
		expect: Range { min: 0, ideal: 0, max: 0 },
		annual,
		round: to_round_inflation(annual, blocks_per_round),
	}
}

fn session_keys(
	aura: AuraId,
	grandpa: GrandpaId,
) -> SessionKeys {
	SessionKeys { aura, grandpa }
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
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, AuraId, GrandpaId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s)
	)
}

pub fn sudo_account_local() -> AccountId {
	AccountId32::from_str("5DviAKtS4ns5TBuoHyigkkwEtNbG4sN1m8mw6XAfTBW7GG7j")
		.expect("Invalid Account Id")
}

pub fn sudo_account_testnet() -> AccountId {
	AccountId32::from_str("5ESNiXWyksqs5DxjSJ9gW2PA6gmgHLvGEUWdiaFfWDfNu82P")
		.expect("Invalid Account Id")
}

pub fn sudo_account_mainnet() -> AccountId {
	AccountId32::from_str("5H13qUDnaSjaahePMYTQoqezHGHWwQGreM5kkEbuMUHu5Vjn")
		.expect("Invalid Account Id")
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
		Default::default(),
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
						// 5Grpw9i5vNyF6pbbvw7vA8pC5Vo8GMUbG8zraLMmAn32kTNH
						hex!["d41e0bf1d76de368bdb91896b0d02d758950969ea795b1e7154343ee210de649"].into(),
						// 5DLMZF33f61KvPDbJU5c2dPNQZ3jJyptsacpvsDhwNS1wUuU
						hex!["382bd29103cf3af5f7c032bbedccfb3144fe672ca2c606147974bc2984ca2b14"].into(),
						hex!["4076403ada1e84a045cfc627efe8f7f1a734e95a50644e7030c0cb1a70dc580f"]
							.unchecked_into(),
						hex!["e52e2901ea2bb2795601f1e130e1936c7f861e6375ea70ad7ca92ee9a121a75f"]
							.unchecked_into(),
					),
					(
						// 5CFDk3yCSgQ2goiaksMfRMFRS7ZU28BZqPQDeAsgZUa6FRzt
						hex!["08050f1b6bcd4651004df427c884073652bafd54e5ca25cea69169532db2910b"].into(),
						// 5F1ks2enazaPktQa3HURLK8GywzNZaGirovPtFvvbv91TLhJ
						hex!["8275157f2a1d8373106cb00078a73a92a3303f3bf6eb72c3a67413bd943b020b"].into(),
						hex!["e8a6d9e3b7961f74fffcd7f7847957dc8e469e07cc49711c52beef4ecae92147"]
							.unchecked_into(),
						hex!["e9e9d202692f8446f013c0b550e4bb1507d6de60a52cdaee0a4863cc554897f9"]
							.unchecked_into(),
					),
					(
						// 5F6YideXfGcskpdFUczu3nZcJFmU9WKHgjjNVQjqgeVGRs66
						hex!["861c6d95051f942bb022f13fc2125b2974933d8ab1441bfdee9855e9d8051556"].into(),
						// 5F92x4qKNYaHtfp5Yy7kb9r6gHCHkN3YSvNuedERPHgrURTn
						hex!["8801f479e09a78515f1badee0169864dae45648109091e29b03a7b4ea97ec018"].into(),
						hex!["accec13ca659e4eb665dcf13d269a2ae529dcf7eed870453417c745e15e3ad27"]
							.unchecked_into(),
						hex!["ee1773c391a8d3e404f2b6f1f0ec5e22b9719a753b2a24376ab50113283d49d0"]
							.unchecked_into(),
					),
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
				true,
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
		Default::default(),
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
						// 5Grpw9i5vNyF6pbbvw7vA8pC5Vo8GMUbG8zraLMmAn32kTNH
						hex!["d41e0bf1d76de368bdb91896b0d02d758950969ea795b1e7154343ee210de649"].into(),
						// 5DLMZF33f61KvPDbJU5c2dPNQZ3jJyptsacpvsDhwNS1wUuU
						hex!["382bd29103cf3af5f7c032bbedccfb3144fe672ca2c606147974bc2984ca2b14"].into(),
						hex!["1c10840139097128f7b28315814351ac9d3b3015615acc87c821685bd3c12f75"]
							.unchecked_into(),
						hex!["b3728b98ab621a98f17a727542de3748aa2427495c82ba94f9c4d2f726efc393"]
							.unchecked_into(),
					),
					(
						// 5CFDk3yCSgQ2goiaksMfRMFRS7ZU28BZqPQDeAsgZUa6FRzt
						hex!["08050f1b6bcd4651004df427c884073652bafd54e5ca25cea69169532db2910b"].into(),
						// 5F1ks2enazaPktQa3HURLK8GywzNZaGirovPtFvvbv91TLhJ
						hex!["8275157f2a1d8373106cb00078a73a92a3303f3bf6eb72c3a67413bd943b020b"].into(),
						hex!["60ba45f02ae84bcd0c4f3dbf48af1cdc7dd3b6b555e9ff4bf49b049f46501a7d"]
							.unchecked_into(),
						hex!["403eeb3e1674713e25b61b01d6a71da99b2cad0abd21500e20c2853df9f70efd"]
							.unchecked_into(),
					),
					(
						// 5F6YideXfGcskpdFUczu3nZcJFmU9WKHgjjNVQjqgeVGRs66
						hex!["861c6d95051f942bb022f13fc2125b2974933d8ab1441bfdee9855e9d8051556"].into(),
						// 5F92x4qKNYaHtfp5Yy7kb9r6gHCHkN3YSvNuedERPHgrURTn
						hex!["8801f479e09a78515f1badee0169864dae45648109091e29b03a7b4ea97ec018"].into(),
						hex!["34a2e82b426fcba4f45b118415296b0bcb4ffc5ac569a01a4d7f4612459a6742"]
							.unchecked_into(),
						hex!["0d2358b76bc57cc34fb972b3326dec12d68b7047b5307fe6d0b9bee0b89c9835"]
							.unchecked_into(),
					),
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
				true,
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
		Default::default(),
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
						// 5Grpw9i5vNyF6pbbvw7vA8pC5Vo8GMUbG8zraLMmAn32kTNH
						hex!["d41e0bf1d76de368bdb91896b0d02d758950969ea795b1e7154343ee210de649"].into(),
						// 5DLMZF33f61KvPDbJU5c2dPNQZ3jJyptsacpvsDhwNS1wUuU
						hex!["382bd29103cf3af5f7c032bbedccfb3144fe672ca2c606147974bc2984ca2b14"].into(),
						hex!["3a7ba9a4e315a6ce061338c3605d4f2b4de436b29e8c6a44ef7a9f6c06670523"]
							.unchecked_into(),
						hex!["2ad9eddbc9a121413f0658b798bc9fedc9ebfaa03979844871780f97e32cddea"]
							.unchecked_into(),
					),
					(
						// 5CFDk3yCSgQ2goiaksMfRMFRS7ZU28BZqPQDeAsgZUa6FRzt
						hex!["08050f1b6bcd4651004df427c884073652bafd54e5ca25cea69169532db2910b"].into(),
						// 5F1ks2enazaPktQa3HURLK8GywzNZaGirovPtFvvbv91TLhJ
						hex!["8275157f2a1d8373106cb00078a73a92a3303f3bf6eb72c3a67413bd943b020b"].into(),
						hex!["28799c33bdff7174671cdf00a31de7072db82c3e0caf7b07bf308f191511db48"]
							.unchecked_into(),
						hex!["2f030305d54e650281312ed900590ad0b560fcd02b7efaedbd8ab7e3d0c05752"]
							.unchecked_into(),
					),
					(
						// 5F6YideXfGcskpdFUczu3nZcJFmU9WKHgjjNVQjqgeVGRs66
						hex!["861c6d95051f942bb022f13fc2125b2974933d8ab1441bfdee9855e9d8051556"].into(),
						// 5F92x4qKNYaHtfp5Yy7kb9r6gHCHkN3YSvNuedERPHgrURTn
						hex!["8801f479e09a78515f1badee0169864dae45648109091e29b03a7b4ea97ec018"].into(),
						hex!["2e7bffef555f987c9c63bffdb52c0fbf32c713bfe62ffd8d300f26a1531d5c69"]
							.unchecked_into(),
						hex!["a1ad48550ce00756b725d57926988771f6a960fc3ac414a0dcd4ea87a0d46618"]
							.unchecked_into(),
					),
					(
						// 5FxxpyvEnE2sVujvhr6x4A4G171uv4WKSLvrUNst9M8MfdpV
						hex!["ac8fdba5bbe008f65d0e85181daa5443c2eb492fea729a5981b2161467f8655c"].into(),
						// 5FxFAYsTNf31D5AGbXW9ETZPUZofpreHjJkdKehidcvDt5X4
						hex!["ac039bef73f76755d3747d711554f7fb0f16022da51483e0d600c9c7c8cbf821"].into(),
						hex!["a459a5db296d38e80e38ea5cb863c7f88b5ba6ea5dc2f888cae9726e8f908c36"]
							.unchecked_into(),
						hex!["f87122bdc0f31343bba1c1b8f11e1584e5ed42234ce479df89608aecde43260d"]
							.unchecked_into(),
					),
				],
				// Sudo account
				sudo_account_mainnet(),
				// Pre-funded accounts
				vec![
					// Endow the Sudo account
					sudo_account_mainnet(),
					hex!["3a7ba9a4e315a6ce061338c3605d4f2b4de436b29e8c6a44ef7a9f6c06670523"].into(),
					hex!["28799c33bdff7174671cdf00a31de7072db82c3e0caf7b07bf308f191511db48"].into(),
					hex!["2e7bffef555f987c9c63bffdb52c0fbf32c713bfe62ffd8d300f26a1531d5c69"].into(),
					hex!["a459a5db296d38e80e38ea5cb863c7f88b5ba6ea5dc2f888cae9726e8f908c36"].into(),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("jur-mainnet"),
		// Properties
		None,
		Some(properties),
		// Extensions
		Default::default(),
	))
}
const NUM_SELECTED_CANDIDATES: u32 = 6;

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> RuntimeGenesisConfig {
	let candidate_stake = jur_node_runtime::MinCandidateStk::get();

	RuntimeGenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			..Default::default()
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		},
		aura: Default::default(),
		grandpa: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		assets: Default::default(),
		treasury: Default::default(),
		staking: jur_node_runtime::StakingConfig {
			candidates: initial_authorities
				.iter()
				.cloned()
				.map(|(acc, _, _, _)| (acc, candidate_stake))
				.collect(), //todo
			delegations: vec![],
			inflation_config: inflation_config(70, 5),
			blocks_per_round: 5,
			collator_commission: Perbill::from_percent(70),
			parachain_bond_reserve_percent: Percent::from_percent(30),
			num_selected_candidates: NUM_SELECTED_CANDIDATES,
		},
		session: jur_node_runtime::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
	}
}