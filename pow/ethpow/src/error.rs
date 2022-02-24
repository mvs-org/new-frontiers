// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
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

//! A manual sealing engine: the engine listens for rpc calls to seal blocks and create forks.
//! This is suitable for a testing environment.


/// Error code for rpc
mod codes {
	pub const MISMATCHED_H256_SEAL: i64 = 10_000;
	pub const INVALID_SEAL: i64 = 11_000;
	pub const EMPTY_TRANSACTION_POOL: i64 = 12_000;
	pub const BLOCK_NOT_FOUND: i64 = 13_000;
	pub const INVALID_POW: i64 = 14_000;
	pub const INHERENTS_ERROR: i64 = 15_000;
	pub const BLOCKCHAIN_ERROR: i64 = 16_000;
	pub const UNKNOWN_ERROR: i64 = 20_000;
}

/// errors encountered by background block authorship task
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
	/// Transaction pool is empty, cannot create a block
	#[display(fmt = "Transaction pool is empty, set create_empty to true,\
	if you want to create empty blocks")]
	EmptyTransactionPool,
	#[display(fmt = "Mismatched H256 Seal Element")]
	MismatchedH256SealElement,
	#[display(fmt = "Invalid WorkSeal")]
	InvalidWorkSeal,
	//#[display(fmt = "Invalid ProofOfWork: expected: {}, found: {}", _0, _1)]
	#[display(fmt = "Invalid ProofOfWork, Invalid Difficulty")]
	InvalidProofOfWork,
	/// Some other error.
	Other(String),
}

impl Error {
	fn to_code(&self) -> i64 {
		use Error::*;
		match self {
			MismatchedH256SealElement => codes::MISMATCHED_H256_SEAL,
			InvalidWorkSeal => codes::INVALID_SEAL,
			InvalidProofOfWork => codes::INVALID_POW,
			EmptyTransactionPool => codes::EMPTY_TRANSACTION_POOL,
			// ConsensusError(_) => codes::CONSENSUS_ERROR,
			// InherentError(_) => codes::INHERENTS_ERROR,
			// BlockchainError(_) => codes::BLOCKCHAIN_ERROR,
			// SendError(_) | Canceled(_) => codes::SERVER_SHUTTING_DOWN,
			_ => codes::UNKNOWN_ERROR
		}
	}
}

