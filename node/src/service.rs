//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::{sync::{Arc, Mutex}, time::Duration, collections::{HashMap, BTreeMap}};
use fc_rpc_core::types::{FilterPool, PendingTransactions};
use sc_client_api::{ExecutorProvider, RemoteBackend};
use metaverse_vm_runtime::{self, opaque::Block, RuntimeApi};
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, BasePath};
use sp_inherents::InherentDataProviders;
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
//use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;
use sc_consensus_babe::{BabeBlockImport, BabeLink, BabeParams, Config as BabeConfig};
use sc_finality_grandpa::SharedVoterState;
use sc_keystore::LocalKeystore;

use crate::cli::Cli;
use crate::rpc::BabeDeps;
// Our native executor instance.
native_executor_instance!(
	pub Executor,
	metaverse_vm_runtime::api::dispatch,
	metaverse_vm_runtime::native_version,
	frame_benchmarking::benchmarking::HostFunctions,
);

type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

pub fn open_frontier_backend(config: &Configuration) -> Result<Arc<fc_db::Backend<Block>>, String> {
	let config_dir = config.base_path.as_ref()
		.map(|base_path| base_path.config_dir(config.chain_spec.id()))
		.unwrap_or_else(|| {
			BasePath::from_project("", "", "new-frontiers")
				.config_dir(config.chain_spec.id())
		});
	let database_dir = config_dir.join("frontier").join("db");

	Ok(Arc::new(fc_db::Backend::<Block>::new(&fc_db::DatabaseSettings {
		source: fc_db::DatabaseSettingsSrc::RocksDb {
			path: database_dir,
			cache_size: 0,
		}
	})?))
}

pub fn new_partial(config: &Configuration, _cli: &Cli) -> Result<sc_service::PartialComponents<
	FullClient, FullBackend, FullSelectChain,
	sp_consensus::DefaultImportQueue<Block, FullClient>,
	sc_transaction_pool::FullPool<Block, FullClient>,
	(
		//sc_consensus_aura::AuraBlockImport<
		//	Block,
		//	FullClient,
		//	sc_finality_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>,
		//	AuraPair
		//>,
		//sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
        //PendingTransactions, Option<FilterPool>, Arc<fc_db::Backend<Block>>,
		BabeBlockImport<
					Block,
					FullClient,
					sc_finality_grandpa::GrandpaBlockImport<
						FullBackend, 
						Block, 
						FullClient, 
						FullSelectChain
					>,
				>,
				sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
				BabeLink<Block>,
				PendingTransactions, Option<FilterPool>, Arc<fc_db::Backend<Block>>,
	)
>, ServiceError> {
	if config.keystore_remote.is_some() {
		return Err(ServiceError::Other(
			format!("Remote Keystores are not supported.")))
	}
	let inherent_data_providers = InherentDataProviders::new();

    let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, Executor>(
			&config,
		)?;
	let client = Arc::new(client);

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let pending_transactions: PendingTransactions
		= Some(Arc::new(Mutex::new(HashMap::new())));

	let filter_pool: Option<FilterPool>
		= Some(Arc::new(Mutex::new(BTreeMap::new())));

	let frontier_backend = open_frontier_backend(config)?;
	

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_handle(),
		client.clone(),
	);
	
	let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
	)?;

	let (babe_import, babe_link) = sc_consensus_babe::block_import(
		BabeConfig::get_or_compute(&*client)?,
		grandpa_block_import.clone(),
		client.clone(),
	)?;

	//let aura_block_import = sc_consensus_aura::AuraBlockImport::<_, _, _, AuraPair>::new(
	//	grandpa_block_import.clone(), client.clone(),
	//);

	//let import_queue = sc_consensus_aura::import_queue::<_, _, _, AuraPair, _, _>(
	//	sc_consensus_aura::slot_duration(&*client)?,
	//	aura_block_import.clone(),
	//	Some(Box::new(grandpa_block_import.clone())),
	//	client.clone(),
	//	inherent_data_providers.clone(),
	//	&task_manager.spawn_handle(),
	//	config.prometheus_registry(),
	//	sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
	//)?;
	let justification_import = grandpa_block_import.clone();
	let import_queue = sc_consensus_babe::import_queue(
		babe_link.clone(),
		babe_import.clone(),
		Some(Box::new(justification_import)),
		client.clone(),
		select_chain.clone(),
		inherent_data_providers.clone(),
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
		sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
	)?;
	//let import_setup = (babe_import.clone(), grandpa_link, babe_link.clone());
	let shared_epoch_changes = babe_link.epoch_changes().clone();
	let babe_config = babe_link.config().clone();

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		inherent_data_providers,
		other: (babe_import, grandpa_link, babe_link, pending_transactions,
				filter_pool,
				frontier_backend,
				),
	})
}

fn remote_keystore(_url: &String) -> Result<Arc<LocalKeystore>, &'static str> {
	// FIXME: here would the concrete keystore be built,
	//        must return a concrete type (NOT `LocalKeystore`) that
	//        implements `CryptoStore` and `SyncCryptoStore`
	Err("Remote Keystore not supported.")
}

/// Builds a new service for a full client.
pub fn new_full(mut config: Configuration, cli: &Cli) -> Result<TaskManager, ServiceError> {

    let enable_dev_signer = cli.run.enable_dev_signer;

	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		mut keystore_container,
		select_chain,
		transaction_pool,
		inherent_data_providers,
		other: (block_import, grandpa_link, babe_link, pending_transactions, filter_pool, frontier_backend),
	} = new_partial(&config, cli)?;

	if let Some(url) = &config.keystore_remote {
		match remote_keystore(url) {
			Ok(k) => keystore_container.set_remote_keystore(k),
			Err(e) => {
				return Err(ServiceError::Other(
					format!("Error hooking up remote keystore for {}: {}", url, e)))
			}
		};
	}

	config.network.extra_sets.push(sc_finality_grandpa::grandpa_peers_set_config());

	let (network, network_status_sinks, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			on_demand: None,
			block_announce_validator_builder: None,
		})?;

    //let (command_sink, commands_stream) = futures::channel::mpsc::channel(1000);

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config, backend.clone(), task_manager.spawn_handle(), client.clone(), network.clone(),
		);
	}
	
	//let (block_import, grandpa_link, babe_link) = import_setup;
	let babe_config = babe_link.config().clone();
	
	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks: Option<()> = None;
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();
    let is_authority = role.is_authority();
	let subscription_task_executor = sc_rpc::SubscriptionTaskExecutor::new(task_manager.spawn_handle());
	let shared_epoch_changes = babe_link.epoch_changes().clone();
	let keystore = keystore_container.sync_keystore();

	let rpc_extensions_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();
		let select_chain = select_chain.clone();
        let network = network.clone();
		let pending = pending_transactions.clone();
		let filter_pool = filter_pool.clone();
		let frontier_backend = frontier_backend.clone();
		let max_past_logs = cli.run.max_past_logs;

		Box::new(move |deny_unsafe, _| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				select_chain: select_chain.clone(),
				deny_unsafe,
                is_authority,
				enable_dev_signer,
				network: network.clone(),
				babe: BabeDeps {
					babe_config: babe_config.clone(),
					shared_epoch_changes: shared_epoch_changes.clone(),
					keystore: keystore.clone(),
				},
				pending_transactions: pending.clone(),
				filter_pool: filter_pool.clone(),
				backend: frontier_backend.clone(),
				max_past_logs,
				//command_sink: Some(command_sink.clone()),
			};

			crate::rpc::create_full(deps, subscription_task_executor.clone())
		})
	};

	let (_rpc_handlers, telemetry_connection_notifier) = sc_service::spawn_tasks(
		sc_service::SpawnTasksParams {
			network: network.clone(),
			client: client.clone(),
			keystore: keystore_container.sync_keystore(),
			task_manager: &mut task_manager,
			transaction_pool: transaction_pool.clone(),
			rpc_extensions_builder,
			on_demand: None,
			remote_blockchain: None,
			backend,
			network_status_sinks,
			system_rpc_tx,
			config,
		},
	)?;
	
	if role.is_authority() {
		
		let proposer_factory = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool,
			prometheus_registry.as_ref(),
		);

		let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		//let aura = sc_consensus_aura::start_aura::<_, _, _, _, _, AuraPair, _, _, _,_>(
		//	sc_consensus_aura::slot_duration(&*client)?,
		//	client.clone(),
		//	select_chain,
		//	block_import,
		//	proposer_factory,
		//	network.clone(),
		//	inherent_data_providers.clone(),
		//	force_authoring,
		//	backoff_authoring_blocks,
		//	keystore_container.sync_keystore(),
		//	can_author_with,
		//)?;

		let babe_config = BabeParams {
			keystore: keystore_container.sync_keystore(),
			client: client.clone(),
			select_chain,
			block_import,
			env: proposer_factory,
			sync_oracle: network.clone(),
			inherent_data_providers: inherent_data_providers.clone(),
			force_authoring,
			backoff_authoring_blocks,
			babe_link,
			can_author_with,
		};
		let babe = sc_consensus_babe::start_babe(babe_config)?;
		// the AURA authoring task is considered essential, i.e. if it
		// fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking("babe", babe);
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let keystore = if role.is_authority() {
		Some(keystore_container.sync_keystore())
	} else {
		None
	};

	let grandpa_config = sc_finality_grandpa::Config {
		// FIXME #1578 make this available through chainspec
		gossip_duration: Duration::from_millis(333),
		justification_period: 512,
		name: Some(name),
		observer_enabled: false,
		keystore,
		is_authority: role.is_network_authority(),
	};

	if enable_grandpa {
		// start the full GRANDPA voter
		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
		// this point the full voter should provide better guarantees of block
		// and vote data availability than the observer. The observer has not
		// been tested extensively yet and having most nodes in a network run it
		// could lead to finality stalls.
		let grandpa_config = sc_finality_grandpa::GrandpaParams {
			config: grandpa_config,
			link: grandpa_link,
			network,
			voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry,
			shared_voter_state: SharedVoterState::empty(),
			telemetry_on_connect: telemetry_connection_notifier.map(|x| x.on_connect_stream()),
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking(
			"grandpa-voter",
			sc_finality_grandpa::run_grandpa_voter(grandpa_config)?
		);
	}

	network_starter.start_network();
	Ok(task_manager)
}

/// Builds a new service for a light client.
pub fn new_light(mut config: Configuration) -> Result<TaskManager, ServiceError> {
	let (client, backend, keystore_container, mut task_manager, on_demand) =
		sc_service::new_light_parts::<Block, RuntimeApi, Executor>(&config)?;

	config.network.extra_sets.push(sc_finality_grandpa::grandpa_peers_set_config());

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_light(
		config.transaction_pool.clone(),
		config.prometheus_registry(),
		task_manager.spawn_handle(),
		client.clone(),
		on_demand.clone(),
	));

	let (grandpa_block_import, _) = sc_finality_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
	)?;

	//let aura_block_import = sc_consensus_aura::AuraBlockImport::<_, _, _, AuraPair>::new(
	//	grandpa_block_import.clone(),
	//	client.clone(),
	//);
	let justification_import = grandpa_block_import.clone();
	let (babe_block_import, babe_link) = sc_consensus_babe::block_import(
		BabeConfig::get_or_compute(&*client)?,
		grandpa_block_import,
		client.clone(),
	)?;

	//let import_queue = sc_consensus_aura::import_queue::<_, _, _, AuraPair, _, _>(
	//	sc_consensus_aura::slot_duration(&*client)?,
	//	aura_block_import,
	//	Some(Box::new(grandpa_block_import)),
	//	client.clone(),
	//	InherentDataProviders::new(),
	//	&task_manager.spawn_handle(),
	//	config.prometheus_registry(),
	//	sp_consensus::NeverCanAuthor,
	//)?;
	let import_queue = sc_consensus_babe::import_queue(
		babe_link,
		babe_block_import,
		Some(Box::new(justification_import)),
		client.clone(),
		select_chain.clone(),
		InherentDataProviders::new(),
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
		sp_consensus::NeverCanAuthor,
	)?;

	let (network, network_status_sinks, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			on_demand: Some(on_demand.clone()),
			block_announce_validator_builder: None,
		})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config, backend.clone(), task_manager.spawn_handle(), client.clone(), network.clone(),
		);
	}

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		remote_blockchain: Some(backend.remote_blockchain()),
		transaction_pool,
		task_manager: &mut task_manager,
		on_demand: Some(on_demand),
		rpc_extensions_builder: Box::new(|_, _| ()),
		config,
		client,
		keystore: keystore_container.sync_keystore(),
		backend,
		network,
		network_status_sinks,
		system_rpc_tx,
	})?;

	network_starter.start_network();

	Ok(task_manager)
}
