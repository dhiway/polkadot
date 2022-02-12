// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Implementation of the Prospective Parachains subsystem - this tracks and handles
//! prospective parachain fragments and informs other backing-stage subsystems
//! of work to be done.
//!
//! This is the main coordinator of work within the node for the collation and
//! backing phases of parachain consensus.
//!
//! This is primarily an implementation of "Fragment Trees", as described in
//! [`polkadot_node_subsystem_util::inclusion_emulator::staging`].
//!
//! This also handles concerns such as the relay-chain being forkful,
//! session changes, predicting validator group assignments, and
//! the re-backing of parachain blocks as a result of these changes.
//!
//! ## Re-backing
//!
//! Since this subsystems deals in enabling the collation and extension
//! of parachains in advance of actually being recorded on the relay-chain,
//! it is possible for the validator-group that initially backed the parablock
//! to be no longer assigned at the point that the parablock is submitted
//! to the relay-chain.
//!
//! This presents an issue, because the relay-chain only accepts blocks
//! which are backed by the currently-assigned group of validators, not
//! by the group of validators previously assigned to the parachain.
//!
//! In order to avoid wasting work at group rotation boundaries, we must
//! allow validators to re-validate the work of the preceding group.
//! This process is known as re-backing.
//!
//! What happens in practice is that validators observe that they are
//! scheduled to be assigned to a specific para in the near future.
//! And as a result, they dig into the existing fragment-trees to
//! re-back what already existed.
