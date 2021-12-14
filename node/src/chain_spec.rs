use std::{str::FromStr, collections::BTreeMap};
use sp_core::{H160, Pair, Public, sr25519};
use metaverse_vm_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, SessionConfig, SessionKeys,
	SudoConfig, SystemConfig, EVMConfig, EthereumConfig, WASM_BINARY, Signature,
	StakerStatus, StakingConfig, ImOnlineConfig, AuthorityDiscoveryConfig,
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::ed25519::AuthorityId as ImOnlineId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_runtime::{
	traits::{Verify, IdentifyAccount},
	Perbill,
};
use sc_service::{ChainType, Properties};
use pallet_evm::GenesisAccount;
// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

pub fn nf_testnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../testnet.json")[..])
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (
	AuraId, 
	GrandpaId, 
	AccountId,
	AccountId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<ImOnlineId>(s),
		get_from_seed::<AuthorityDiscoveryId>(s),
	)
}

pub fn properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("ss58Format".into(),150.into());
	properties.insert("tokenDecimals".into(), vec![8, 8].into());
	properties.insert("tokenSymbol".into(), vec!["ETP", "DNA"].into());

	properties
}


pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(properties()),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
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
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(properties()),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AuraId, GrandpaId, AccountId, 
		AccountId, ImOnlineId, AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	const GENESIS_EVM_ACCOUNT: &'static str = "0xa6f101A982fdd1eF115A614BDbBF67DA71a6c4E3";
	let mut evm_accounts = BTreeMap::new();

	evm_accounts.insert(
		H160::from_str(GENESIS_EVM_ACCOUNT)
						.expect("internal H160 is valid; qed"),
		GenesisAccount {
			nonce: 0.into(),
			balance: 20_000_000_000_000_000_000_000_000u128.into(),
			storage: BTreeMap::new(),
			code: vec![],
		},
	);	
	// Configure endowed accounts with initial balance of 1 << 60.
	const STASH: u128 = 1 << 60;

	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, STASH)).collect(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: 60,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| 
					(x.2.clone(), x.3.clone(), STASH, StakerStatus::Validator)
				)
				.collect(),
			invulnerables: [].to_vec(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities
				.iter()
				.cloned()
				.map(|(aura, grandpa, aidStash, aid, imoId, audId)| {
					(
						aidStash.clone(),                   // account id
						aidStash.clone(),                   // validator id
						SessionKeys{aura, grandpa, im_online: imoId, authority_discovery: audId},    // session keys
					)
				})
				.collect(),
		}),
		pallet_collective_Instance1: Some(Default::default()),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(Default::default()),
		pallet_aura: Some(Default::default()),
		pallet_grandpa: Some(Default::default()),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		pallet_evm: Some(EVMConfig {
			accounts: evm_accounts,
			
		}),
		pallet_ethereum: Some(EthereumConfig {})
		
	}
}
