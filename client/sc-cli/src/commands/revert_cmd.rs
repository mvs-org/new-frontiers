// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::error;
use crate::params::{GenericNumber, PruningParams, SharedParams};
use crate::CliConfiguration;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT, NumberFor, Zero};
use sp_runtime::generic::{BlockId};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use structopt::StructOpt;
use sc_client_api::{backend::{Finalizer}, Backend, UsageProvider};
use log::info;


/// Performs a revert of `blocks` blocks.
fn revert_chain<B, BA, C>(
	client: Arc<C>,
	backend: Arc<BA>,
	blocks: NumberFor<B>,
	rfinal: bool
) -> Result<(), sc_service::Error>
where
	B: BlockT,
	C: UsageProvider<B> + Finalizer<B, BA>,
	BA: Backend<B>,
{
	let reverted = backend.revert(blocks, rfinal)?;
	let info = client.usage_info().chain;

	if reverted.0.is_zero() {
		info!("There aren't any non-finalized blocks to revert.");
	} else {
		if rfinal {
			client.finalize_block(BlockId::Number(info.finalized_number), None, false).unwrap();
		}
		info!("Reverted {} blocks. Best: #{} ({}), Finalized: #{} ({})", reverted.0, 
			info.best_number, info.best_hash, info.finalized_number, info.finalized_hash);
	}
	Ok(())
}

/// The `revert` command used revert the chain to a previous state.
#[derive(Debug, StructOpt)]
pub struct RevertCmd {
	/// Number of blocks to revert.
	#[structopt(default_value = "256")]
	pub num: GenericNumber,

	/// If revert finalized block.
	#[structopt(long = "rfinal")]
	pub rfinal: bool,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: SharedParams,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub pruning_params: PruningParams,
}

impl RevertCmd {
	/// Run the revert command
	pub async fn run<B, BA, C>(
		&self,
		client: Arc<C>,
		backend: Arc<BA>,
	) -> error::Result<()>
	where
		B: BlockT,
		BA: Backend<B>,
		C: UsageProvider<B> + Finalizer<B, BA>,
		<<<B as BlockT>::Header as HeaderT>::Number as FromStr>::Err: Debug,
	{
		let blocks = self.num.parse()?;
		revert_chain(client, backend, blocks, self.rfinal)?;

		Ok(())
	}
}

impl CliConfiguration for RevertCmd {
	fn shared_params(&self) -> &SharedParams {
		&self.shared_params
	}

	fn pruning_params(&self) -> Option<&PruningParams> {
		Some(&self.pruning_params)
	}
}
