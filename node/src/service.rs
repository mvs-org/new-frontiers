//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::{sync::{Arc, Mutex}, time::{Duration, SystemTime, UNIX_EPOCH}, collections::{HashMap, BTreeMap}};
use fc_rpc_core::types::{FilterPool, PendingTransactions};
use sc_client_api::{ExecutorProvider, RemoteBackend};
use metaverse_vm_runtime::{self, opaque::Block, RuntimeApi};
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, BasePath};
use sp_inherents::InherentDataProviders;
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_finality_grandpa::{SharedVoterState, GrandpaBlockImport};
use sc_keystore::LocalKeystore;
use crate::cli::Cli;

use futures::prelude::*;
use parking_lot::{Mutex as PMutex};
use sp_consensus::import_queue::BasicQueue;
use sp_api::{ProvideRuntimeApi, TransactionFor};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT, UniqueSaturatedInto};
use ethash::{self, SeedHashCompute};
use ethpow::{MinimalEthashAlgorithm, EthashAlgorithm, WorkSeal};
use ethash_rpc::{self, EtheminerCmd, Work, RpcError};
use sc_consensus_pow::{PowAlgorithm, MiningWorker, MiningMetadata, MiningBuild};
use sp_core::{U256, H256};
use ethereum_types::{self, U256 as EU256, H256 as EH256};
use parity_scale_codec::{Decode, Encode};
use log::{error, info, debug, trace, warn};


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

pub fn build_inherent_data_providers() -> Result<InherentDataProviders, ServiceError> {
	let providers = InherentDataProviders::new();

	// providers
	// 	.register_provider(sp_timestamp::InherentDataProvider)
	// 	.map_err(Into::into)
	// 	.map_err(sp_consensus::error::Error::InherentData)?;

	Ok(providers)
}

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
	BasicQueue<Block, TransactionFor<FullClient, Block>>,
	sc_transaction_pool::FullPool<Block, FullClient>,
	(
		sc_consensus_pow::PowBlockImport<
			Block,
			GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>,
			FullClient,
			FullSelectChain,
			EthashAlgorithm<FullClient>,
			impl sp_consensus::CanAuthorWith<Block>,
		>,
		sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
        PendingTransactions, Option<FilterPool>, Arc<fc_db::Backend<Block>>,
		EthashAlgorithm<FullClient>,
	)
>, ServiceError> {
	if config.keystore_remote.is_some() {
		return Err(ServiceError::Other(
			format!("Remote Keystores are not supported.")))
	}
	let inherent_data_providers = build_inherent_data_providers()?;

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

	let can_author_with = sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());
	let ethash_alg = EthashAlgorithm::new(client.clone());

	let pow_block_import = sc_consensus_pow::PowBlockImport::new(
		grandpa_block_import,
		client.clone(),
		ethash_alg.clone(),
		0, // check inherents starting at block 0
		select_chain.clone(),
		inherent_data_providers.clone(),
		can_author_with,
	);

	let import_queue = sc_consensus_pow::import_queue(
		Box::new(pow_block_import.clone()),
		None,
		ethash_alg.clone(),
		inherent_data_providers.clone(),
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
	)?;

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		inherent_data_providers,
		other: (pow_block_import, grandpa_link, 
				pending_transactions,
				filter_pool,
				frontier_backend,
				ethash_alg,
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
		other: (pow_block_import, grandpa_link, pending_transactions, filter_pool, frontier_backend, ethash_alg),
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

	// Channel for the rpc handler to communicate with the authorship task.
	let (command_sink, commands_stream) = futures::channel::mpsc::channel(1000);

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config, backend.clone(), task_manager.spawn_handle(), client.clone(), network.clone(),
		);
	}

	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks: Option<()> = None;
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();
    let is_authority = role.is_authority();
	let subscription_task_executor = sc_rpc::SubscriptionTaskExecutor::new(task_manager.spawn_handle());

	let rpc_extensions_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();
        let network = network.clone();
		let pending = pending_transactions.clone();
		let filter_pool = filter_pool.clone();
		let frontier_backend = frontier_backend.clone();
		let max_past_logs = cli.run.max_past_logs;

		Box::new(move |deny_unsafe, _| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				deny_unsafe,
                is_authority,
				enable_dev_signer,
				network: network.clone(),
				pending_transactions: pending.clone(),
				filter_pool: filter_pool.clone(),
				backend: frontier_backend.clone(),
				max_past_logs,
				command_sink: command_sink.clone(),
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
		let proposer = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool.clone(),
			prometheus_registry.as_ref(),
		);

		let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		let (worker, worker_task) = sc_consensus_pow::start_mining_worker(
			Box::new(pow_block_import),
			client.clone(),
			select_chain,
			ethash_alg,
			proposer,
			network.clone(),
			None,
			inherent_data_providers,
			// time to wait for a new block before starting to mine a new one
			Duration::from_secs(10),
			// how long to take to actually build the block (i.e. executing extrinsics)
			Duration::from_secs(10),
			can_author_with,
		);

		task_manager
			.spawn_essential_handle()
			.spawn_blocking("pow", worker_task);
		
		// Start Mining
		task_manager
			.spawn_essential_handle()
			.spawn_blocking("mining", run_mining_svc(worker.clone(), commands_stream));
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

	let inherent_data_providers = build_inherent_data_providers()?;
	let ethash_alg = EthashAlgorithm::new(client.clone());

	let pow_block_import = sc_consensus_pow::PowBlockImport::new(
		grandpa_block_import,
		client.clone(),
		ethash_alg.clone(),
		0, // check inherents starting at block 0
		select_chain,
		inherent_data_providers.clone(),
		// FixMe #375
		sp_consensus::AlwaysCanAuthor,
	);

	let import_queue = sc_consensus_pow::import_queue(
		Box::new(pow_block_import),
		None,
		ethash_alg.clone(),
		inherent_data_providers,
		&task_manager.spawn_handle(),
		config.prometheus_registry(),
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


pub async fn run_mining_svc<B, Algorithm, C, CS>(
	worker : Arc<PMutex<MiningWorker<B, Algorithm, C>>>,
	mut commands_stream: CS,
)
	where 
	B: BlockT<Hash = H256>,
	Algorithm: PowAlgorithm<B, Difficulty = U256>,
	C: sp_api::ProvideRuntimeApi<B>,
	CS: Stream<Item=EtheminerCmd<<B as BlockT>::Hash>> + Unpin + 'static,
{
	let seed_compute = SeedHashCompute::default();

	while let Some(command) = commands_stream.next().await {
		match command {
			EtheminerCmd::GetWork { mut sender } => {
				let metadata = worker.lock().metadata();
				if let Some(metadata) = metadata {
					let nr :u64 = UniqueSaturatedInto::<u64>::unique_saturated_into(metadata.number);
					let pow_hash:H256 = metadata.pre_hash;
					let seed_hash:H256 = seed_compute.hash_block_number(nr).into();
					let tmp:[u8; 32] = metadata.difficulty.into();
					let tmp:[u8; 32] = ethash::difficulty_to_boundary(&EU256::from(tmp)).into();
					let target:H256 = H256::from(tmp);

					let ret = Ok(Work { 
						pow_hash, 
						seed_hash,
						target, 
						number: Some(nr),
					 });

					ethash_rpc::send_result(&mut sender, ret)
					// ethash_rpc::send_result(&mut sender, future.await)
				} else {
					ethash_rpc::send_result(&mut sender, Err(RpcError::NoWork))
				}
			}
			EtheminerCmd::SubmitWork {  nonce, pow_hash, mix_digest, mut sender } => {
				let mut worker = worker.lock();
				let metadata = worker.metadata();
				if let Some(metadata) = metadata {
					let non_nr :u64 = UniqueSaturatedInto::<u64>::unique_saturated_into(nonce);
					let header_nr :u64 = UniqueSaturatedInto::<u64>::unique_saturated_into(metadata.number);
					let timestamp :u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
					let seal = WorkSeal{nonce:non_nr, pow_hash, mix_digest, difficulty:metadata.difficulty, header_nr, timestamp};
					debug!(target:"pow", "worker.submit pow_hash: {}", pow_hash);
					worker.submit(seal.encode());
					ethash_rpc::send_result(&mut sender, Ok(true))
				} else {
					ethash_rpc::send_result(&mut sender, Err(RpcError::NoMetaData))
				}

						
			}
			EtheminerCmd::SubmitHashrate { hash, mut sender } => {
				
			}
		}
	}
}

