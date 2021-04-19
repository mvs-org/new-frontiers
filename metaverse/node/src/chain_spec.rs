use sp_core::{Pair, Public, sr25519, ed25519};
use frontier_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, EVMConfig, EthereumConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;
use std::collections::BTreeMap;
use hex_literal::hex;
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
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Metaverse",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
                (
                    sr25519::Public::from_slice(&hex!("0x14cb71c93c6393a70fdee7e6b786a639d06ac55a6b47e7df9e05fbd44f195f29")).into(),
                    ed25519::Public::from_slice(&hex!("0x2fbbdd17c5aa4c48ca0ecf31b8eeebfa017b49a668aaa873a3f9ef0399cf6107")).into(),
                ),
                (
                    sr25519::Public::from_slice(&hex!("0x44ad4030c5f6c583905406c19085e00382e3d3336f430e84aab3816f704afc00")).into(),
                    ed25519::Public::from_slice(&hex!("0xef722502cd41cfc69dd143d4c5db75a35a7e3d574b12f22b5a11b7f1bd36659d")).into(),
                ),
            ],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("5EfTxKXzGSzX4QRXAL5yJz8qtSQ3iZJjykC2et8BBF1R11fd"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("5Fq4FmzHHfTEPozeNoE9nQxvUHN9kEehKp5sCc4QAz3F1f1c"),
				
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
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

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
				authority_keys_from_seed("5DtFw8yZ776C3BKrEWypLaGycL5QkQMHnHpeVEEkC6btFRCW"),
				
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("5DtFw8yZ776C3BKrEWypLaGycL5QkQMHnHpeVEEkC6btFRCW"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("5DtFw8yZ776C3BKrEWypLaGycL5QkQMHnHpeVEEkC6btFRCW"),
				
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
	const GENESIS_EVM_ACCOUNT: &'static str = "0x34249F7f5640A3c534AA4d5DBB1e999D922462E1";
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
	
		frame_system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		},
		pallet_aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		pallet_grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		pallet_sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		pallet_evm: EVMConfig {
			accounts: evm_accounts,
			
		},
		pallet_ethereum: EthereumConfig {},
		
	}
}
