use sp_core::{Pair, Public, sr25519};
use metaverse_vm_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, EVMConfig, EthereumConfig, WASM_BINARY, Signature
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;
use std::collections::BTreeMap;
use pallet_evm::GenesisAccount;
use array_bytes::fixed_hex_bytes_unchecked;
// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

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
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
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
				authority_keys_from_seed("Bob"),
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
		None,
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
		None,
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


	// const ROOT: &'static str = "0x72819fbc1b93196fa230243947c1726cbea7e33044c7eb6f736ff345561f9e4c";
	// const GENESIS_VALIDATOR: &'static str = "Alice";
	// const GENESIS_VALIDATOR_STASH: &'static str = "Alice//stash";
	// const GENESIS_VALIDATOR_BOND: Balance = COIN;
	const GENESIS_EVM_ACCOUNT: &'static str = "0xa6f101A982fdd1eF115A614BDbBF67DA71a6c4E3";
	// const GENESIS_ETHEREUM_RELAY_AUTHORITY_SIGNER: &'static str =
	// 	"0x6aA70f55E5D770898Dd45aa1b7078b8A80AAbD6C";

	// const TOKEN_REDEEM_ADDRESS: &'static str = "0x49262B932E439271d05634c32978294C7Ea15d0C";
	// const DEPOSIT_REDEEM_ADDRESS: &'static str = "0x6EF538314829EfA8386Fc43386cB13B4e0A67D1e";
	// const SET_AUTHORITIES_ADDRESS: &'static str = "0xE4A2892599Ad9527D76Ce6E26F93620FA7396D85";
	// const ETP_TOKEN_ADDRESS: &'static str = "0xb52FBE2B925ab79a821b261C82c5Ba0814AAA5e0";
	// const DNA_TOKEN_ADDRESS: &'static str = "0x1994100c58753793D52c6f457f189aa3ce9cEe94";

	// let root = AccountId::from(fixed_hex_bytes_unchecked!(ROOT, 32));
	let evm = fixed_hex_bytes_unchecked!(GENESIS_EVM_ACCOUNT, 20).into();
	// let initial_authorities = vec![get_authority_keys_from_seed(GENESIS_VALIDATOR)];
	// let endowed_accounts = vec![
	// 	(root.clone(), 1 << 56),
	// 	(
	// 		get_account_id_from_seed::<sr25519::Public>(GENESIS_VALIDATOR_STASH),
	// 		GENESIS_VALIDATOR_BOND,
	// 	),
	// ];
	let mut evm_accounts = BTreeMap::new();

	evm_accounts.insert(
		evm,
		GenesisAccount {
			nonce: 0.into(),
			balance: 20_000_000_000_000_000_000_000_000u128.into(),
			storage: BTreeMap::new(),
			code: vec![],
		},
	);	

	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		pallet_aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
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
